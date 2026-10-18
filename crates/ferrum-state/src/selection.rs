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

    /// Maps this selection range through a `ChangeDesc`.
    pub fn map(&self, change: &crate::change::ChangeDesc, assoc: i8) -> Self {
        let (anchor, head) = if self.is_empty() {
            let pos = change
                .map_pos(self.from(), assoc, crate::change::MapMode::Simple)
                .unwrap_or(self.from());
            (pos, pos)
        } else {
            let from = change
                .map_pos(self.from(), 1, crate::change::MapMode::Simple)
                .unwrap_or(self.from());
            let to = change
                .map_pos(self.to(), -1, crate::change::MapMode::Simple)
                .unwrap_or(self.to());
            if self.is_inverted() {
                (to, from)
            } else {
                (from, to)
            }
        };
        Self {
            anchor,
            head,
            goal_column: self.goal_column,
            assoc: self.assoc,
        }
    }

    /// Extends this range to cover at least `from` to `to`.
    pub fn extend(&self, from: usize, to: usize) -> Self {
        if from <= self.anchor && to >= self.anchor {
            Self::new(from, to)
        } else {
            let head = if (from as isize - self.anchor as isize).abs()
                > (to as isize - self.anchor as isize).abs()
            {
                from
            } else {
                to
            };
            Self::new(self.anchor, head)
        }
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
    /// Automatically sorts ranges by `from` and merges overlapping ranges matching CodeMirror 6.
    pub fn create(mut ranges: Vec<SelectionRange>, mut main_index: usize) -> Self {
        if ranges.is_empty() {
            return Self::cursor(0);
        }

        // Fast-path: check if ranges are already strictly sorted and non-overlapping
        let mut sorted = true;
        let mut pos = 0;
        for (i, r) in ranges.iter().enumerate() {
            if i > 0 && (if r.is_empty() { r.from() <= pos } else { r.from() < pos }) {
                sorted = false;
                break;
            }
            pos = r.to();
        }

        if sorted {
            let clamped_main = main_index.min(ranges.len().saturating_sub(1));
            return Self {
                ranges: ranges.into(),
                main_index: clamped_main,
            };
        }

        // Normalization path (CM6 algorithm):
        let main = ranges[main_index.min(ranges.len() - 1)];
        ranges.sort_by_key(|r| r.from());
        main_index = ranges.iter().position(|r| *r == main).unwrap_or(0);

        let mut i = 1;
        while i < ranges.len() {
            let range = ranges[i];
            let prev = ranges[i - 1];
            if if range.is_empty() { range.from() <= prev.to() } else { range.from() < prev.to() } {
                let from = prev.from();
                let to = max(range.to(), prev.to());
                if i <= main_index {
                    main_index = main_index.saturating_sub(1);
                }
                let merged_range = if range.anchor > range.head {
                    SelectionRange::new(to, from)
                } else {
                    SelectionRange::new(from, to)
                };
                ranges[i - 1] = merged_range;
                ranges.remove(i);
            } else {
                i += 1;
            }
        }

        let clamped_main = main_index.min(ranges.len().saturating_sub(1));
        Self {
            ranges: ranges.into(),
            main_index: clamped_main,
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

    /// Maps all ranges through a `ChangeDesc`, re-sorting and merging if necessary.
    pub fn map(&self, change: &crate::change::ChangeDesc, assoc: i8) -> Self {
        if change.is_empty() {
            return self.clone();
        }
        let mapped: Vec<SelectionRange> = self.ranges.iter().map(|r| r.map(change, assoc)).collect();
        Self::create(mapped, self.main_index)
    }
}

