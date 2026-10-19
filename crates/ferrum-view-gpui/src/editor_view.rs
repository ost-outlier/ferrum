use std::sync::Arc;
use crate::keymap_bridge::handle_keystroke;
use crate::theme::EditorTheme;
use ferrum_commands::{delete_line, insert_text};
use ferrum_state::char::find_word_break;
use ferrum_state::selection::EditorSelection;
use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;
use ferrum_view::viewstate::PixelRect;
use ferrum_view::{EditorView, HeightOracle};
use gpui::{
    div, fill, point, px, relative, size, App, Bounds, ClipboardItem, Context, CursorStyle,
    Element, ElementId, FocusHandle, Focusable, GlobalElementId, InspectorElementId,
    InteractiveElement, IntoElement, KeyDownEvent, LayoutId, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, PaintQuad, ParentElement, Pixels, Point, Render, ScrollDelta,
    ScrollWheelEvent, ShapedLine, SharedString, Style, Styled, TextRun, Window,
};

/// An interactive GPUI view that hosts the Ferrum EditorView.
pub struct FerrumEditorView {
    pub view: EditorView,
    pub focus_handle: FocusHandle,
    pub theme: EditorTheme,
    pub font_size: Pixels,
    pub line_height: Pixels,
    pub gutter_width: Pixels,
    pub last_bounds: Option<Bounds<Pixels>>,
    pub last_layouts: Vec<ShapedLine>,
    pub last_start_line: usize,
    pub is_dragging: bool,
    pub drag_anchor: Option<usize>,
}

/// Converts a character (codepoint) offset within `text` to its corresponding UTF-8 byte offset.
#[inline]
pub fn char_to_byte_offset(text: &str, char_offset: usize) -> usize {
    text.char_indices()
        .nth(char_offset)
        .map(|(byte_offset, _)| byte_offset)
        .unwrap_or(text.len())
}

/// Converts a UTF-8 byte offset within `text` to its corresponding character (codepoint) offset.
#[inline]
pub fn byte_to_char_offset(text: &str, byte_offset: usize) -> usize {
    let clamped = byte_offset.min(text.len());
    let safe_byte = if text.is_char_boundary(clamped) {
        clamped
    } else {
        (0..=clamped)
            .rev()
            .find(|&b| text.is_char_boundary(b))
            .unwrap_or(0)
    };
    text[..safe_byte].chars().count()
}

/// Helper to convert GPUI Pixels to f64
#[inline]
pub fn px_to_f64(p: Pixels) -> f64 {
    (p / px(1.0)) as f64
}

impl FerrumEditorView {
    /// Creates a new interactive GPUI editor view wrapping the given `EditorState`.
    pub fn new(state: Arc<EditorState>, cx: &mut Context<Self>) -> Self {
        let font_size = px(14.0);
        let line_height = px(22.0);
        let gutter_width = px(46.0);

        let mut oracle = HeightOracle::new(false);
        oracle.line_height = px_to_f64(line_height);
        oracle.char_width = px_to_f64(font_size) * 0.6;
        let pixel_viewport = PixelRect::new(0.0, 1000.0, 0.0, 800.0);
        let view = EditorView::with_oracle(state, oracle, pixel_viewport);
        let focus_handle = cx.focus_handle();

        Self {
            view,
            focus_handle,
            theme: EditorTheme::default(),
            font_size,
            line_height,
            gutter_width,
            last_bounds: None,
            last_layouts: Vec::new(),
            last_start_line: 0,
            is_dragging: false,
            drag_anchor: None,
        }
    }

    /// Converts a window coordinate into a document character offset.
    pub fn pos_at_coords(&self, position: Point<Pixels>) -> Option<usize> {
        let bounds = self.last_bounds?;
        let rel_x = position.x - bounds.left() - self.gutter_width;
        let scroll_top = self.view.view_state().scroll_top;
        let rel_y = px_to_f64(position.y - bounds.top()) + scroll_top;
        if rel_y < 0.0 {
            return Some(0);
        }
        let line_idx = (rel_y / px_to_f64(self.line_height)).floor().max(0.0) as usize;
        let doc = self.view.state().doc();
        if line_idx >= doc.len_lines() {
            return Some(doc.len_chars());
        }
        let line = doc.line_by_index(line_idx);
        let col = if line_idx >= self.last_start_line
            && (line_idx - self.last_start_line) < self.last_layouts.len()
        {
            let layout = &self.last_layouts[line_idx - self.last_start_line];
            let byte_offset = layout.closest_index_for_x(rel_x);
            let line_text = doc.slice_to_string(line.from..line.to);
            byte_to_char_offset(&line_text, byte_offset)
        } else {
            let char_w = (px_to_f64(self.font_size) * 0.6).max(1.0);
            ((px_to_f64(rel_x).max(0.0) / char_w) as usize).min(line.length)
        };
        Some((line.from + col).min(line.to))
    }

    /// Handles mouse wheel scrolling with virtualized heightmap offsets.
    pub fn on_scroll_wheel(&mut self, event: &ScrollWheelEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let delta_y = match event.delta {
            ScrollDelta::Lines(p) => (p.y as f64) * px_to_f64(self.line_height),
            ScrollDelta::Pixels(p) => px_to_f64(p.y),
        };
        let current_scroll = self.view.view_state().scroll_top;
        self.view.view_state_mut().set_scroll_top(current_scroll - delta_y);
        cx.notify();
    }

    /// Handles keyboard events, executing commands, clipboard operations, or inserting text.
    pub fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let is_mod = event.keystroke.modifiers.platform || event.keystroke.modifiers.control;

        // Clipboard Copy (Ctrl+C / Cmd+C)
        if is_mod && event.keystroke.key.eq_ignore_ascii_case("c") {
            let state = self.view.state();
            let doc = state.doc();
            let mut text_parts = Vec::new();
            for r in state.selection().ranges() {
                if !r.is_empty() {
                    text_parts.push(doc.slice_to_string(r.from()..r.to()));
                }
            }
            let copy_text = if text_parts.is_empty() {
                let main = state.selection().main();
                let line_idx = doc.char_to_line(main.head);
                let line = doc.line_by_index(line_idx);
                let mut line_str = doc.slice_to_string(line.from..line.to);
                line_str.push('\n');
                line_str
            } else {
                text_parts.join("\n")
            };
            cx.write_to_clipboard(ClipboardItem::new_string(copy_text));
            return;
        }

        // Clipboard Cut (Ctrl+X / Cmd+X)
        if is_mod && event.keystroke.key.eq_ignore_ascii_case("x") {
            let state = self.view.state();
            let doc = state.doc();
            let mut text_parts = Vec::new();
            for r in state.selection().ranges() {
                if !r.is_empty() {
                    text_parts.push(doc.slice_to_string(r.from()..r.to()));
                }
            }
            if text_parts.is_empty() {
                let main = state.selection().main();
                let line_idx = doc.char_to_line(main.head);
                let line = doc.line_by_index(line_idx);
                let mut line_str = doc.slice_to_string(line.from..line.to);
                line_str.push('\n');
                cx.write_to_clipboard(ClipboardItem::new_string(line_str));
                if let Some(spec) = delete_line(state) {
                    self.view.dispatch(spec);
                    let head = self.view.state().selection().main().head;
                    self.view.view_state_mut().scroll_into_view(head);
                    cx.notify();
                }
            } else {
                cx.write_to_clipboard(ClipboardItem::new_string(text_parts.join("\n")));
                if let Some(spec) = insert_text(state, "") {
                    self.view.dispatch(spec);
                    let head = self.view.state().selection().main().head;
                    self.view.view_state_mut().scroll_into_view(head);
                    cx.notify();
                }
            }
            return;
        }

        // Clipboard Paste (Ctrl+V / Cmd+V)
        if is_mod && event.keystroke.key.eq_ignore_ascii_case("v") {
            if let Some(item) = cx.read_from_clipboard() {
                if let Some(text) = item.text() {
                    if let Some(spec) = insert_text(self.view.state(), &text) {
                        self.view.dispatch(spec);
                        let head = self.view.state().selection().main().head;
                        self.view.view_state_mut().scroll_into_view(head);
                        cx.notify();
                    }
                }
            }
            return;
        }

        // Standard keystroke dispatch (KEYMAP facet, history undo/redo, arrows, typing)
        if let Some(spec) = handle_keystroke(self.view.state(), &event.keystroke) {
            self.view.dispatch(spec);
            let head = self.view.state().selection().main().head;
            self.view.view_state_mut().scroll_into_view(head);
            cx.notify();
        }
    }

    /// Handles mouse clicks (single, double, triple, shift-click, right-click).
    pub fn on_mouse_down(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(pos) = self.pos_at_coords(event.position) else {
            return;
        };
        let doc = self.view.state().doc();

        match event.button {
            MouseButton::Left => {
                if event.click_count == 2 {
                    // Double-click: select word
                    let line_idx = doc.char_to_line(pos);
                    let line = doc.line_by_index(line_idx);
                    let line_text = doc.slice_to_string(line.from..line.to);
                    let rel_pos = pos.saturating_sub(line.from);
                    let start_rel = find_word_break(&line_text, rel_pos, false);
                    let end_rel = find_word_break(&line_text, rel_pos, true);
                    let word_from = line.from + start_rel;
                    let word_to = line.from + end_rel;

                    self.drag_anchor = Some(word_from);
                    self.is_dragging = false;
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::single(word_from, word_to)));
                    cx.notify();
                } else if event.click_count >= 3 {
                    // Triple-click: select line
                    let line_idx = doc.char_to_line(pos);
                    let line = doc.line_by_index(line_idx);
                    let end = if line_idx + 1 < doc.len_lines() {
                        line.to + 1
                    } else {
                        line.to
                    };
                    self.drag_anchor = Some(line.from);
                    self.is_dragging = false;
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::single(line.from, end)));
                    cx.notify();
                } else if event.modifiers.shift {
                    // Shift-click: extend current selection
                    let main = self.view.state().selection().main();
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::single(main.anchor, pos)));
                    cx.notify();
                } else {
                    // Single-click: set cursor and prepare for drag selection
                    self.drag_anchor = Some(pos);
                    self.is_dragging = true;
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::cursor(pos)));
                    cx.notify();
                }
            }
            MouseButton::Right => {
                // Right-click: if clicked outside selection, move cursor to clicked position
                let is_inside = self.view.state().selection().ranges().iter().any(|r| pos >= r.from() && pos <= r.to());
                if !is_inside {
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::cursor(pos)));
                    cx.notify();
                }
            }
            _ => {}
        }
    }

    /// Handles mouse drag to extend selection.
    pub fn on_mouse_move(&mut self, event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.is_dragging {
            if let Some(anchor) = self.drag_anchor {
                if let Some(pos) = self.pos_at_coords(event.position) {
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::single(anchor, pos)));
                    self.view.view_state_mut().scroll_into_view(pos);
                    cx.notify();
                }
            }
        }
    }

    /// Handles mouse button release to end drag selection.
    pub fn on_mouse_up(&mut self, _event: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
    }
}

impl Focusable for FerrumEditorView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub struct EditorCanvasElement {
    view_entity: gpui::Entity<FerrumEditorView>,
}

pub struct EditorPrepaintState {
    lines: Vec<ShapedLine>,
    cursor_quads: Vec<PaintQuad>,
    selection_quads: Vec<PaintQuad>,
    gutter_lines: Vec<ShapedLine>,
    start_line: usize,
    scroll_top: f64,
}

impl IntoElement for EditorCanvasElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditorCanvasElement {
    type RequestLayoutState = ();
    type PrepaintState = EditorPrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let view_height = px_to_f64(bounds.size.height);
        let view_width = px_to_f64(bounds.size.width);

        // Synchronize measured pixel viewport with ViewState
        self.view_entity.update(cx, |editor, _cx| {
            let vs = editor.view.view_state_mut();
            if (vs.pixel_viewport.height() - view_height).abs() > 1.0 {
                vs.pixel_viewport.bottom = vs.scroll_top + view_height;
                vs.pixel_viewport.right = view_width;
                vs.viewport = vs.get_viewport(0.0, None);
            }
        });

        let editor = self.view_entity.read(cx);
        let state = editor.view.state();
        let doc = state.doc();
        let theme = &editor.theme;
        let font_size = editor.font_size;
        let line_height = editor.line_height;
        let gutter_width = editor.gutter_width;
        let scroll_top = editor.view.view_state().scroll_top;

        let viewport = editor.view.viewport();
        let start_line = doc.char_to_line(viewport.from);
        let end_line = doc.char_to_line(viewport.to).min(doc.len_lines().saturating_sub(1));

        let num_lines = (end_line - start_line + 1).max(1);
        let mut shaped_lines = Vec::with_capacity(num_lines);
        let mut gutter_lines = Vec::with_capacity(num_lines);
        let mut line_texts = Vec::with_capacity(num_lines);

        // 1. Shape ONLY lines inside the virtualized viewport
        for line_idx in start_line..=end_line {
            let line_info = doc.line_by_index(line_idx);
            let line_text = doc.slice_to_string(line_info.from..line_info.to);

            // Gutter line number
            let gutter_text = format!("{:4} ", line_idx + 1);
            let gutter_run = TextRun {
                len: gutter_text.len(),
                font: window.text_style().font(),
                color: theme.gutter_fg.into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let gutter_shaped = window
                .text_system()
                .shape_line(SharedString::from(gutter_text), font_size, &[gutter_run], None);
            gutter_lines.push(gutter_shaped);

            // Line content
            let default_run = TextRun {
                len: line_text.len(),
                font: window.text_style().font(),
                color: theme.text_color.into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };

            let shaped = window.text_system().shape_line(
                SharedString::from(line_text.clone()),
                font_size,
                &[default_run],
                None,
            );
            shaped_lines.push(shaped);
            line_texts.push(line_text);
        }

        // 2. Prepare selection and cursor quads
        let mut cursor_quads = Vec::new();
        let mut selection_quads = Vec::new();

        for range in state.selection().ranges() {
            // Only process selections intersecting active viewport
            if range.to() >= viewport.from && range.from() <= viewport.to {
                // Caret / Cursor
                let head_line_idx = doc.char_to_line(range.head);
                if head_line_idx >= start_line && head_line_idx <= end_line {
                    let layout_idx = head_line_idx - start_line;
                    if let Some(layout) = shaped_lines.get(layout_idx) {
                        let head_line = doc.line_by_index(head_line_idx);
                        let col = range.head.saturating_sub(head_line.from);
                        let head_text = &line_texts[layout_idx];
                        let byte_col = char_to_byte_offset(head_text, col);
                        let cursor_x = gutter_width + layout.x_for_index(byte_col);
                        let doc_y = (head_line_idx as f64) * px_to_f64(line_height);
                        let cursor_y = px((doc_y - scroll_top) as f32);

                        cursor_quads.push(fill(
                            Bounds::new(
                                point(bounds.left() + cursor_x, bounds.top() + cursor_y),
                                size(px(2.0), line_height),
                            ),
                            theme.cursor_color,
                        ));
                    }
                }

                // Selection highlight
                if !range.is_empty() {
                    let from_line_idx = doc.char_to_line(range.from()).max(start_line);
                    let to_line_idx = doc.char_to_line(range.to()).min(end_line);

                    for l_idx in from_line_idx..=to_line_idx {
                        let layout_idx = l_idx - start_line;
                        if let Some(layout) = shaped_lines.get(layout_idx) {
                            let line = doc.line_by_index(l_idx);
                            let start_col = if l_idx == doc.char_to_line(range.from()) {
                                range.from().saturating_sub(line.from)
                            } else {
                                0
                            };
                            let end_col = if l_idx == doc.char_to_line(range.to()) {
                                range.to().saturating_sub(line.from)
                            } else {
                                line.length
                            };

                            let l_text = &line_texts[layout_idx];
                            let start_byte = char_to_byte_offset(l_text, start_col);
                            let end_byte = char_to_byte_offset(l_text, end_col);

                            let start_x = gutter_width + layout.x_for_index(start_byte);
                            let end_x = gutter_width + layout.x_for_index(end_byte);
                            let doc_y = (l_idx as f64) * px_to_f64(line_height);
                            let y = px((doc_y - scroll_top) as f32);

                            if end_x > start_x {
                                selection_quads.push(fill(
                                    Bounds::from_corners(
                                        point(bounds.left() + start_x, bounds.top() + y),
                                        point(bounds.left() + end_x, bounds.top() + y + line_height),
                                    ),
                                    theme.selection_color,
                                ));
                            }
                        }
                    }
                }
            }
        }

        EditorPrepaintState {
            lines: shaped_lines,
            cursor_quads,
            selection_quads,
            gutter_lines,
            start_line,
            scroll_top,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let theme = self.view_entity.read(cx).theme.clone();
        let gutter_width = self.view_entity.read(cx).gutter_width;
        let line_height = self.view_entity.read(cx).line_height;
        let start_line = prepaint.start_line;
        let scroll_top = prepaint.scroll_top;

        // 1. Paint background
        window.paint_quad(fill(bounds, theme.background));

        // 2. Paint gutter background strip
        let gutter_bounds = Bounds::new(
            bounds.origin,
            size(gutter_width, bounds.size.height),
        );
        window.paint_quad(fill(gutter_bounds, theme.gutter_bg));

        // 3. Paint selections behind text
        for sel_quad in prepaint.selection_quads.drain(..) {
            window.paint_quad(sel_quad);
        }

        // Store bounds and layout copies for click hit-testing
        let layouts = prepaint.lines.clone();
        self.view_entity.update(cx, |editor, _cx| {
            editor.last_bounds = Some(bounds);
            editor.last_layouts = layouts;
            editor.last_start_line = start_line;
        });

        // 4. Paint virtualized gutter numbers and line texts
        for (idx, (gutter, text_line)) in prepaint
            .gutter_lines
            .drain(..)
            .zip(prepaint.lines.drain(..))
            .enumerate()
        {
            let line_idx = start_line + idx;
            let doc_y = (line_idx as f64) * px_to_f64(line_height);
            let y = bounds.top() + px((doc_y - scroll_top) as f32);

            // Only paint if intersecting visible bounds
            if y + line_height >= bounds.top() && y <= bounds.bottom() {
                // Gutter number
                gutter
                    .paint(point(bounds.left() + px(4.0), y), line_height, window, cx)
                    .ok();

                // Line text
                text_line
                    .paint(
                        point(bounds.left() + gutter_width, y),
                        line_height,
                        window,
                        cx,
                    )
                    .ok();
            }
        }

        // 5. Paint cursors on top
        for cursor_quad in prepaint.cursor_quads.drain(..) {
            window.paint_quad(cursor_quad);
        }

        // 6. Paint scrollbar if document exceeds viewport height
        let doc_lines = self.view_entity.read(cx).view.state().doc().len_lines();
        let total_doc_height = (doc_lines as f64) * px_to_f64(line_height);
        let view_h = px_to_f64(bounds.size.height);
        if total_doc_height > view_h && view_h > 0.0 {
            let scrollbar_width = px(6.0);
            let scrollbar_x = bounds.right() - scrollbar_width - px(2.0);
            let thumb_height = px(((view_h / total_doc_height) * view_h).max(24.0) as f32);
            let max_scroll = (total_doc_height - view_h).max(1.0);
            let scroll_ratio = (scroll_top / max_scroll).clamp(0.0, 1.0);
            let thumb_top = bounds.top() + px((scroll_ratio * (view_h - px_to_f64(thumb_height))) as f32);

            let thumb_bounds = Bounds::new(
                point(scrollbar_x, thumb_top),
                size(scrollbar_width, thumb_height),
            );
            window.paint_quad(fill(thumb_bounds, theme.gutter_fg));
        }
    }
}

impl Render for FerrumEditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let doc = self.view.state().doc();
        let sel = self.view.state().selection().main();
        let line_idx = doc.char_to_line(sel.head);
        let col_idx = sel.head.saturating_sub(doc.line_by_index(line_idx).from);

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(self.theme.background)
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_key_down(cx.listener(Self::on_key_down))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_down(MouseButton::Right, cx.listener(Self::on_mouse_down))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_scroll_wheel(cx.listener(Self::on_scroll_wheel))
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(EditorCanvasElement {
                        view_entity: cx.entity(),
                    }),
            )
            .child(
                // Bottom Status Bar
                div()
                    .h(px(24.0))
                    .w_full()
                    .bg(self.theme.gutter_bg)
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(8.0))
                    .text_size(px(12.0))
                    .text_color(self.theme.gutter_fg)
                    .child(format!("Ln {}, Col {} | Length: {} chars | {} lines", line_idx + 1, col_idx + 1, doc.len_chars(), doc.len_lines()))
                    .child("Ferrum Engine • HeightMap Virtualized • 100% Permissive (MIT / Apache-2.0)"),
            )
    }
}
