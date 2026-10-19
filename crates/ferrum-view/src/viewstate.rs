//! ViewState manages viewport virtualization, height mapping, and coordinate projections.
//! Faithful port of CodeMirror 6's `@codemirror/view/src/viewstate.ts`.

use crate::heightmap::{ChangedRange, HeightMap, HeightOracle, QueryType};
use crate::view::ViewUpdate;
use crate::viewport::Viewport;
use ferrum_state::state::EditorState;
use ferrum_state::text::Text;
use std::sync::Arc;

/// Viewport overscan margin in pixels, ported from CodeMirror 6's `VP.Margin = 1000`.
pub const VP_MARGIN: f64 = 1000.0;

/// Represents a rectangle in 2D coordinate space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PixelRect {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

impl PixelRect {
    pub fn new(left: f64, right: f64, top: f64, bottom: f64) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn height(&self) -> f64 {
        (self.bottom - self.top).max(0.0)
    }

    pub fn width(&self) -> f64 {
        (self.right - self.left).max(0.0)
    }
}

/// Strategy for scrolling a position into view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollStrategy {
    Nearest,
    Start,
    Center,
    End,
}

/// Target specification for scrolling an element or range into view.
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollTarget {
    pub head: usize,
    pub y: ScrollStrategy,
    pub x: ScrollStrategy,
    pub y_margin: f64,
    pub x_margin: f64,
}

impl ScrollTarget {
    pub fn new(head: usize) -> Self {
        Self {
            head,
            y: ScrollStrategy::Nearest,
            x: ScrollStrategy::Nearest,
            y_margin: 0.0,
            x_margin: 0.0,
        }
    }
}

/// State of the editor view's virtualization, scroll position, and viewport.
pub struct ViewState {
    pub state: Arc<EditorState>,
    pub height_map: HeightMap,
    pub height_oracle: HeightOracle,
    pub pixel_viewport: PixelRect,
    pub viewport: Viewport,
    pub scroll_top: f64,
    pub scroll_left: f64,
    pub scroll_target: Option<ScrollTarget>,
}

impl ViewState {
    /// Creates a new `ViewState` for the given editor state, height oracle, and pixel window size.
    pub fn new(state: Arc<EditorState>, mut height_oracle: HeightOracle, pixel_viewport: PixelRect) -> Self {
        let doc_len = state.doc().len_chars();
        let height_map = HeightMap::empty().apply_changes(
            &[],
            &Text::empty(),
            &mut height_oracle.set_doc(state.doc().clone()),
            &[ChangedRange::new(0, 0, 0, doc_len)],
        );

        let initial_viewport = Viewport::new(state.doc(), 0, 0);

        let mut vs = Self {
            state: state.clone(),
            height_map,
            height_oracle,
            pixel_viewport,
            viewport: initial_viewport,
            scroll_top: pixel_viewport.top,
            scroll_left: pixel_viewport.left,
            scroll_target: None,
        };

        vs.viewport = vs.get_viewport(0.0, None);
        vs
    }

    /// Computes the active visible viewport given a scroll delta bias and optional scroll target.
    /// Ported directly from CodeMirror 6's `ViewState.getViewport` (`viewstate.ts:390`).
    pub fn get_viewport(&mut self, bias: f64, scroll_target: Option<&ScrollTarget>) -> Viewport {
        let margin_top = 0.5 - (-0.5f64).max((bias / VP_MARGIN / 2.0).min(0.5));
        let visible_top = self.scroll_top;
        let view_height = self.pixel_viewport.height();
        let visible_bottom = visible_top + view_height;

        let query_top = (visible_top - margin_top * VP_MARGIN).max(0.0);
        let query_bottom = (visible_bottom + (1.0 - margin_top) * VP_MARGIN).min(self.height_map.height());

        self.height_oracle.set_doc(self.state.doc().clone());
        let top_block = self
            .height_map
            .line_at(query_top.round() as usize, QueryType::ByHeight, &mut self.height_oracle, 0.0, 0);
        let bottom_block =
            self.height_map
                .line_at(query_bottom.round() as usize, QueryType::ByHeight, &mut self.height_oracle, 0.0, 0);

        let mut vp_from = top_block.from;
        let mut vp_to = bottom_block.to();

        if let Some(target) = scroll_target {
            let head = target.head;
            if head < vp_from || head > vp_to {
                let block = self
                    .height_map
                    .line_at(head, QueryType::ByPos, &mut self.height_oracle, 0.0, 0);
                let target_top = match target.y {
                    ScrollStrategy::Center => (block.top + block.bottom()) / 2.0 - view_height / 2.0,
                    ScrollStrategy::Start => block.top,
                    ScrollStrategy::Nearest if head < vp_from => block.top,
                    _ => block.bottom() - view_height,
                };

                let extra_top = (target_top - VP_MARGIN / 2.0).max(0.0);
                let extra_bot = (target_top + view_height + VP_MARGIN / 2.0).min(self.height_map.height());

                let extra_from_block =
                    self.height_map
                        .line_at(extra_top.round() as usize, QueryType::ByHeight, &mut self.height_oracle, 0.0, 0);
                let extra_to_block =
                    self.height_map
                        .line_at(extra_bot.round() as usize, QueryType::ByHeight, &mut self.height_oracle, 0.0, 0);

                vp_from = vp_from.min(extra_from_block.from);
                vp_to = vp_to.max(extra_to_block.to());
            }
        }

        Viewport::new(self.state.doc(), vp_from, vp_to)
    }

    /// Sets the vertical scroll position in pixels, updating the active viewport.
    pub fn set_scroll_top(&mut self, scroll_top: f64) {
        let max_scroll = (self.height_map.height() - self.pixel_viewport.height()).max(0.0);
        self.scroll_top = scroll_top.clamp(0.0, max_scroll);
        let height = self.pixel_viewport.height();
        self.pixel_viewport.top = self.scroll_top;
        self.pixel_viewport.bottom = self.scroll_top + height;
        self.viewport = self.get_viewport(0.0, None);
    }

    /// Scrolls the given document position into the visible window if it is currently outside.
    pub fn scroll_into_view(&mut self, pos: usize) {
        let clamped_pos = pos.min(self.state.doc().len_chars());
        self.height_oracle.set_doc(self.state.doc().clone());
        let block = self
            .height_map
            .line_at(clamped_pos, QueryType::ByPos, &mut self.height_oracle, 0.0, 0);

        let view_height = self.pixel_viewport.height();
        if block.top < self.scroll_top {
            self.set_scroll_top(block.top);
        } else if block.bottom() > self.scroll_top + view_height {
            self.set_scroll_top(block.bottom() - view_height);
        }

        let target = ScrollTarget::new(clamped_pos);
        self.viewport = self.get_viewport(0.0, Some(&target));
    }

    /// Computes the bounding rectangle in document pixels for the given character position.
    pub fn coords_at_pos(&mut self, pos: usize) -> Option<PixelRect> {
        let clamped = pos.min(self.state.doc().len_chars());
        self.height_oracle.set_doc(self.state.doc().clone());
        let block = self
            .height_map
            .line_at(clamped, QueryType::ByPos, &mut self.height_oracle, 0.0, 0);

        let line = self.state.doc().line_at(clamped);
        let col = clamped.saturating_sub(line.from);
        let char_width = self.height_oracle.char_width;
        let left = (col as f64) * char_width;
        let right = left + char_width;

        Some(PixelRect::new(left, right, block.top, block.bottom()))
    }

    /// Converts document pixel coordinates (x, y) into a character offset within the document.
    pub fn pos_at_coords(&mut self, x: f64, y: f64) -> usize {
        let clamped_y = y.clamp(0.0, self.height_map.height());
        self.height_oracle.set_doc(self.state.doc().clone());
        let block = self
            .height_map
            .line_at(clamped_y.round() as usize, QueryType::ByHeight, &mut self.height_oracle, 0.0, 0);

        let char_width = self.height_oracle.char_width.max(1.0);
        let col = (x.max(0.0) / char_width).floor() as usize;
        let line_len = block.to().saturating_sub(block.from);
        block.from + col.min(line_len)
    }

    /// Applies an editor transaction update, maintaining the HeightMap and recomputing the viewport.
    pub fn update(&mut self, update: &ViewUpdate, scroll_target: Option<ScrollTarget>) {
        self.state = update.state.clone();
        if update.doc_changed {
            let prev_doc = update.prev_state.doc();
            let new_doc = self.state.doc();
            let changed_ranges = vec![ChangedRange::new(0, prev_doc.len_chars(), 0, new_doc.len_chars())];

            self.height_oracle.set_doc(new_doc.clone());
            self.height_map = self.height_map.apply_changes(
                &[],
                prev_doc,
                &mut self.height_oracle,
                &changed_ranges,
            );
        }

        if let Some(ref target) = scroll_target {
            self.scroll_into_view(target.head);
        } else {
            self.viewport = self.get_viewport(0.0, None);
        }
    }
}
