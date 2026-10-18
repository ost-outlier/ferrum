use std::sync::Arc;
use crate::keymap_bridge::handle_keystroke;
use crate::theme::EditorTheme;
use ferrum_state::selection::EditorSelection;
use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;
use ferrum_view::EditorView;
use gpui::{
    div, fill, point, px, relative, size, App, Bounds, Context, CursorStyle, Element, ElementId,
    FocusHandle, Focusable, GlobalElementId, InspectorElementId, InteractiveElement, IntoElement,
    KeyDownEvent, LayoutId, MouseButton, MouseDownEvent, PaintQuad, ParentElement, Pixels, Render,
    ShapedLine, SharedString, Style, Styled, TextRun, Window,
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
        }
    }

    /// Handles keyboard events, executing commands or inserting text.
    pub fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(spec) = handle_keystroke(self.view.state(), &event.keystroke) {
            self.view.dispatch(spec);
            cx.notify();
        }
    }

    /// Handles mouse clicks to set the cursor position.
    pub fn on_mouse_down(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(bounds) = self.last_bounds {
            let rel_x = event.position.x - bounds.left() - self.gutter_width;
            let rel_y = event.position.y - bounds.top();
            if rel_y >= px(0.0) {
                let line_idx = (rel_y / self.line_height).floor().max(0.0) as usize;
                let doc = self.view.state().doc();
                if line_idx < doc.len_lines() {
                    let line = doc.line(line_idx);
                    let col = if let Some(layout) = self.last_layouts.get(line_idx) {
                        layout.closest_index_for_x(rel_x)
                    } else {
                        0
                    };
                    let pos = (line.from + col).min(line.to);
                    self.view.dispatch(TransactionSpec::new().with_selection(EditorSelection::cursor(pos)));
                    cx.notify();
                }
            }
        }
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
                SharedString::from(line_text),
                font_size,
                &[default_run],
                None,
            );
            shaped_lines.push(shaped);
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
                let cursor_x = gutter_width + layout.x_for_index(col);
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

                        let start_x = gutter_width + layout.x_for_index(start_col);
                        let end_x = gutter_width + layout.x_for_index(end_col);
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
