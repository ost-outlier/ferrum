use std::sync::Arc;
use crate::keymap_bridge::handle_keystroke;
use crate::theme::EditorTheme;
use ferrum_commands::{delete_line, insert_text};
use ferrum_state::char::find_word_break;
use ferrum_state::selection::EditorSelection;
use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;
use ferrum_view::EditorView;
use gpui::{
    div, fill, point, px, relative, size, App, Bounds, ClipboardItem, Context, CursorStyle,
    Element, ElementId, FocusHandle, Focusable, GlobalElementId, InspectorElementId,
    InteractiveElement, IntoElement, KeyDownEvent, LayoutId, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, PaintQuad, ParentElement, Pixels, Point, Render, ShapedLine,
    SharedString, Style, Styled, TextRun, Window,
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

impl FerrumEditorView {
    /// Creates a new interactive GPUI editor view wrapping the given `EditorState`.
    pub fn new(state: Arc<EditorState>, cx: &mut Context<Self>) -> Self {
        let view = EditorView::new(state);
        let focus_handle = cx.focus_handle();
        Self {
            view,
            focus_handle,
            theme: EditorTheme::default(),
            font_size: px(14.0),
            line_height: px(22.0),
            gutter_width: px(46.0),
            last_bounds: None,
            last_layouts: Vec::new(),
            is_dragging: false,
            drag_anchor: None,
        }
    }

    /// Converts a window coordinate into a document character offset.
    pub fn pos_at_coords(&self, position: Point<Pixels>) -> Option<usize> {
        let bounds = self.last_bounds?;
        let rel_x = position.x - bounds.left() - self.gutter_width;
        let rel_y = position.y - bounds.top();
        if rel_y < px(0.0) {
            return Some(0);
        }
        let line_idx = (rel_y / self.line_height).floor().max(0.0) as usize;
        let doc = self.view.state().doc();
        if line_idx >= doc.len_lines() {
            return Some(doc.len_chars());
        }
        let line = doc.line(line_idx);
        let col = if let Some(layout) = self.last_layouts.get(line_idx) {
            let byte_offset = layout.closest_index_for_x(rel_x);
            let line_text = doc.slice_to_string(line.from..line.to);
            byte_to_char_offset(&line_text, byte_offset)
        } else {
            0
        };
        Some((line.from + col).min(line.to))
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
                // If selection is empty, copy the current line
                let main = state.selection().main();
                let line_idx = doc.char_to_line(main.head);
                let line = doc.line(line_idx);
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
                let line = doc.line(line_idx);
                let mut line_str = doc.slice_to_string(line.from..line.to);
                line_str.push('\n');
                cx.write_to_clipboard(ClipboardItem::new_string(line_str));
                if let Some(spec) = delete_line(state) {
                    self.view.dispatch(spec);
                    cx.notify();
                }
            } else {
                cx.write_to_clipboard(ClipboardItem::new_string(text_parts.join("\n")));
                if let Some(spec) = insert_text(state, "") {
                    self.view.dispatch(spec);
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
                        cx.notify();
                    }
                }
            }
            return;
        }

        // Standard keystroke dispatch (KEYMAP facet, history undo/redo, arrows, typing)
        if let Some(spec) = handle_keystroke(self.view.state(), &event.keystroke) {
            self.view.dispatch(spec);
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
                    let line = doc.line(line_idx);
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
                    let line = doc.line(line_idx);
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
        let editor = self.view_entity.read(cx);
        let state = editor.view.state();
        let doc = state.doc();
        let theme = &editor.theme;
        let font_size = editor.font_size;
        let line_height = editor.line_height;
        let gutter_width = editor.gutter_width;

        let mut shaped_lines = Vec::new();
        let mut gutter_lines = Vec::new();
        let mut line_texts = Vec::with_capacity(doc.len_lines());

        // 1. Shape each line of text and gutter numbers
        for line_idx in 0..doc.len_lines() {
            let line_info = doc.line(line_idx);
            let line_text = doc.slice_to_string(line_info.from..line_info.to);

            // Gutter line number
            let gutter_text = format!("{:3} ", line_idx + 1);
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

            // Line content runs with syntax decorations
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
            // Caret / Cursor
            let head_line_idx = doc.char_to_line(range.head);
            if let Some(layout) = shaped_lines.get(head_line_idx) {
                let head_line = doc.line(head_line_idx);
                let col = range.head.saturating_sub(head_line.from);
                let head_text = &line_texts[head_line_idx];
                let byte_col = char_to_byte_offset(head_text, col);
                let cursor_x = gutter_width + layout.x_for_index(byte_col);
                let cursor_y = line_height * (head_line_idx as f32);

                cursor_quads.push(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_x, bounds.top() + cursor_y),
                        size(px(2.0), line_height),
                    ),
                    theme.cursor_color,
                ));
            }

            // Selection highlight
            if !range.is_empty() {
                let from_line_idx = doc.char_to_line(range.from());
                let to_line_idx = doc.char_to_line(range.to());

                for l_idx in from_line_idx..=to_line_idx {
                    if let Some(layout) = shaped_lines.get(l_idx) {
                        let line = doc.line(l_idx);
                        let start_col = if l_idx == from_line_idx {
                            range.from().saturating_sub(line.from)
                        } else {
                            0
                        };
                        let end_col = if l_idx == to_line_idx {
                            range.to().saturating_sub(line.from)
                        } else {
                            line.length
                        };

                        let l_text = &line_texts[l_idx];
                        let start_byte = char_to_byte_offset(l_text, start_col);
                        let end_byte = char_to_byte_offset(l_text, end_col);

                        let start_x = gutter_width + layout.x_for_index(start_byte);
                        let end_x = gutter_width + layout.x_for_index(end_byte);
                        let y = line_height * (l_idx as f32);

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

        EditorPrepaintState {
            lines: shaped_lines,
            cursor_quads,
            selection_quads,
            gutter_lines,
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
        });

        // 4. Paint gutter numbers and line texts
        for (i, (gutter, text_line)) in prepaint
            .gutter_lines
            .drain(..)
            .zip(prepaint.lines.drain(..))
            .enumerate()
        {
            let y = bounds.top() + line_height * (i as f32);

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

        // 5. Paint cursors on top
        for cursor_quad in prepaint.cursor_quads.drain(..) {
            window.paint_quad(cursor_quad);
        }
    }
}

impl Render for FerrumEditorView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let doc = self.view.state().doc();
        let sel = self.view.state().selection().main();
        let line_idx = doc.char_to_line(sel.head);
        let col_idx = sel.head.saturating_sub(doc.line(line_idx).from);

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
                    .child(format!("Ln {}, Col {} | Length: {} chars", line_idx + 1, col_idx + 1, doc.len_chars()))
                    .child("Ferrum Engine • GPUI Native Surface • 100% Permissive (MIT / Apache-2.0)"),
            )
    }
}
