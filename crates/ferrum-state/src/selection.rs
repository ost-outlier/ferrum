use smallvec::{smallvec, SmallVec};
use std::cmp::{max, min};

/// A single range of selection within the editor.
/// If `anchor == head`, the range represents a simple cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange {
    /// The fixed end of the selection when dragging or extending.
    pub anchor: usize,
    /// The active end of the selection that moves with cursor motions.
    pub head: usize,
    /// Optional goal column for vertical navigation (retained across lines).
    pub goal_column: Option<usize>,
    /// Bidi association bias: -1 for left/preceding, 1 for right/following.
    pub assoc: i8,
}

impl SelectionRange {
    /// Creates a new cursor range at the given position.
    pub fn cursor(pos: usize) -> Self {
        Self {
            anchor: pos,
            head: pos,
            goal_column: None,
            assoc: 0,
        }
    }

    /// Creates a selection range from an anchor to a head.
    pub fn new(anchor: usize, head: usize) -> Self {
        Self {
            anchor,
            head,
            goal_column: None,
            assoc: 0,
        }
    }

    /// The lower bound position of the range.
    #[inline]
    pub fn from(&self) -> usize {
        min(self.anchor, self.head)
    }

    /// The upper bound position of the range.
    #[inline]
    pub fn to(&self) -> usize {
        max(self.anchor, self.head)
    }

    /// True if this range represents an empty cursor (`anchor == head`).
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// True if the cursor is inverted (`head < anchor`), selecting backwards.
    #[inline]
    pub fn is_inverted(&self) -> bool {
        self.head < self.anchor
    }

    /// Returns a new range with `goal_column` set.
    pub fn with_goal_column(mut self, col: Option<usize>) -> Self {
        self.goal_column = col;
        self
    }
}

/// An immutable collection of selection ranges, with one designated as the primary selection.
/// Follows CodeMirror 6's invariant: ranges are always sorted by position and non-overlapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSelection {
    ranges: SmallVec<[SelectionRange; 1]>,
    main_index: usize,
}

impl Default for EditorSelection {
    fn default() -> Self {
        Self::cursor(0)
    }
}

impl EditorSelection {
    /// Creates a single cursor selection at `pos`.
    pub fn cursor(pos: usize) -> Self {
        Self {
            ranges: smallvec![SelectionRange::cursor(pos)],
            main_index: 0,
        }
    }

    /// Creates a single selection range from `anchor` to `head`.
    pub fn single(anchor: usize, head: usize) -> Self {
        Self {
            ranges: smallvec![SelectionRange::new(anchor, head)],
            main_index: 0,
        }
    }

    /// Creates an `EditorSelection` from an arbitrary list of ranges and a primary index.
    /// Automatically sorts ranges by `from` and merges overlapping/touching ranges.
    pub fn create(ranges: Vec<SelectionRange>, main_index: usize) -> Self {
        if ranges.is_empty() {
            return Self::cursor(0);
        }

        // Tag each range with its original index so we can track the main selection
        let mut tagged: Vec<(usize, SelectionRange)> = ranges
            .into_iter()
            .enumerate()
            .collect();

        tagged.sort_by_key(|(_, r)| (r.from(), r.to()));

        let mut merged: SmallVec<[SelectionRange; 1]> = SmallVec::new();
        let mut new_main = 0;

        for (orig_idx, range) in tagged {
            if let Some(last) = merged.last_mut() {
                // If overlapping or touching (and at least one is non-empty)
                if range.from() <= last.to() {
                    let from = min(last.from(), range.from());
                    let to = max(last.to(), range.to());

                    // Preserve direction of the active/main range
                    let (anchor, head) = if orig_idx == main_index {
                        if range.is_inverted() { (to, from) } else { (from, to) }
                    } else if last.is_inverted() {
                        (to, from)
                    } else {
                        (from, to)
                    };

                    *last = SelectionRange {
                        anchor,
                        head,
                        goal_column: if orig_idx == main_index { range.goal_column } else { last.goal_column },
                        assoc: if orig_idx == main_index { range.assoc } else { last.assoc },
                    };

                    if orig_idx == main_index {
                        new_main = merged.len() - 1;
                    }
                    continue;
                }
            }

            if orig_idx == main_index {
                new_main = merged.len();
            }
            merged.push(range);
        }

        Self {
            ranges: merged,
            main_index: new_main,
        }
    }

    /// Returns the primary selection range.
    #[inline]
    pub fn main(&self) -> &SelectionRange {
        &self.ranges[self.main_index]
    }

    /// Returns the main index.
    #[inline]
    pub fn main_index(&self) -> usize {
        self.main_index
    }

    /// Returns all selection ranges.
    #[inline]
    pub fn ranges(&self) -> &[SelectionRange] {
        &self.ranges
    }

    /// Returns true if this selection contains only a single range.
    #[inline]
    pub fn is_single(&self) -> bool {
        self.ranges.len() == 1
    }
}
