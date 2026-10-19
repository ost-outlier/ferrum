use crate::decoration::{Decoration, DECORATIONS};
use crate::viewport::Viewport;
use ferrum_state::state::EditorState;
use ferrum_state::text::Line;
use ferrum_state::transaction::{Transaction, TransactionSpec};
use std::sync::Arc;

/// Represents an update delivered to view plugins and listeners.
pub struct ViewUpdate {
    /// The new editor state after the update.
    pub state: Arc<EditorState>,
    /// The previous state before this update.
    pub prev_state: Arc<EditorState>,
    /// All transactions applied during this update.
    pub transactions: Vec<Transaction>,
    /// True if the document text was altered.
    pub doc_changed: bool,
    /// True if the selection changed.
    pub selection_changed: bool,
}

/// A decoration applied across character boundaries `[from, to]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecoratedRange {
    pub from: usize,
    pub to: usize,
    pub decoration: Decoration,
}

/// Abstract representation of a rendered line within the viewport.
#[derive(Debug, Clone)]
pub struct RenderedLine {
    /// The text line information from the document.
    pub line: Line,
    /// Text slice of this line.
    pub text: String,
    /// All decorations intersecting this line.
    pub decorations: Vec<DecoratedRange>,
}

/// The core view controller of the editor.
/// Bridges the immutable `EditorState` with rendering surfaces and user interactions.
pub struct EditorView {
    state: Arc<EditorState>,
    viewport: Viewport,
}

impl EditorView {
    /// Creates a new `EditorView` with the given initial state.
    pub fn new(state: Arc<EditorState>) -> Self {
        let len = state.doc().len_chars();
        let viewport = Viewport::new(state.doc(), 0, len);
        Self { state, viewport }
    }

    /// Returns the current `EditorState`.
    pub fn state(&self) -> &Arc<EditorState> {
        &self.state
    }

    /// Returns the current visible viewport.
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    /// Updates the visible viewport bounds.
    pub fn set_viewport(&mut self, from: usize, to: usize) {
        self.viewport = Viewport::new(self.state.doc(), from, to);
    }

    /// Dispatches a transaction specification, updating the view state.
    /// Returns the resulting `ViewUpdate` if the transaction was accepted.
    pub fn dispatch(&mut self, spec: TransactionSpec) -> Option<ViewUpdate> {
        let prev_state = self.state.clone();
        let tr = self.state.update(spec)?;
        let doc_changed = tr.doc_changed();
        let selection_changed = tr.selection().is_some();

        let new_state = self.state.apply(tr);
        self.state = new_state.clone();

        // Adjust viewport if document changed
        if doc_changed {
            self.set_viewport(self.viewport.from, self.viewport.to);
        }

        Some(ViewUpdate {
            state: new_state,
            prev_state,
            transactions: Vec::new(),
            doc_changed,
            selection_changed,
        })
    }

    /// Collects and aggregates all decorations that intersect the current viewport via canonical `RangeSet.between`.
    pub fn visible_decorations(&self) -> Vec<DecoratedRange> {
        let sets = self.state.facet(&DECORATIONS);
        let mut visible = Vec::new();

        for set in sets {
            set.between(self.viewport.from, self.viewport.to, |from, to, deco| {
                visible.push(DecoratedRange {
                    from,
                    to,
                    decoration: deco.clone(),
                });
                true
            });
        }

        visible.sort_by_key(|d| (d.from, d.to));
        visible
    }

    /// Builds the list of lines to be rendered for the current viewport.
    pub fn render_lines(&self) -> Vec<RenderedLine> {
        let doc = self.state.doc();
        let all_decos = self.visible_decorations();
        let mut lines = Vec::new();

        for line_idx in (self.viewport.top_line.saturating_sub(1))..self.viewport.bottom_line {
            if line_idx >= doc.len_lines() {
                break;
            }

            let line_info = doc.line_by_index(line_idx);
            let line_text = doc.slice_to_string(line_info.from..line_info.to);

            // Filter decorations intersecting this line
            let line_decos: Vec<DecoratedRange> = all_decos
                .iter()
                .filter(|d| !(d.to < line_info.from || d.from > line_info.to))
                .cloned()
                .collect();

            lines.push(RenderedLine {
                line: line_info,
                text: line_text,
                decorations: line_decos,
            });
        }

        lines
    }
}
