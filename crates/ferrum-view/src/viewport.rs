use ferrum_state::text::Text;

/// Represents the visible slice of document lines currently rendered in the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    /// Starting character offset of the visible region.
    pub from: usize,
    /// Ending character offset of the visible region.
    pub to: usize,
    /// 1-indexed starting visible line number.
    pub top_line: usize,
    /// 1-indexed ending visible line number.
    pub bottom_line: usize,
}

impl Viewport {
    /// Creates a new viewport covering the specified character bounds.
    pub fn new(doc: &Text, from: usize, to: usize) -> Self {
        let from = from.min(doc.len_chars());
        let to = to.min(doc.len_chars()).max(from);

        let top_line = doc.line_at(from).number;
        let bottom_line = doc.line_at(to).number;

        Self {
            from,
            to,
            top_line,
            bottom_line,
        }
    }

    /// Returns true if the given character position lies inside the visible viewport.
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.from && pos <= self.to
    }

    /// Returns true if the range `[from, to]` intersects this viewport.
    pub fn intersects(&self, from: usize, to: usize) -> bool {
        !(to < self.from || from > self.to)
    }
}
