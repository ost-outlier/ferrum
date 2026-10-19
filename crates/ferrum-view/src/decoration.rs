//! Text decorations and `DecorationSet` based directly on the canonical CodeMirror 6 `RangeSet` model.

use ferrum_state::facet::Facet;
use ferrum_state::rangeset::{Range, RangeSet, RangeValue};
use std::ops::Range as StdRange;
use std::sync::LazyLock;

/// The type and attributes of a decoration applied to text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecorationKind {
    /// Applies inline styling or a CSS/theme tag to a range of text.
    Mark {
        tag: &'static str,
    },
    /// Replaces a range of text with alternative rendering or hides it (essential for Live Preview).
    Replace {
        widget_id: Option<&'static str>,
    },
    /// Injects an inline widget at a specific point without replacing characters.
    Widget {
        widget_id: &'static str,
        side: i32, // Negative before position, positive after position
    },
}

/// A decoration value attached to a character range in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoration {
    pub kind: DecorationKind,
}

impl RangeValue for Decoration {
    fn start_side(&self) -> i32 {
        match &self.kind {
            DecorationKind::Widget { side, .. } => *side,
            _ => 0,
        }
    }

    fn end_side(&self) -> i32 {
        match &self.kind {
            DecorationKind::Widget { side, .. } => *side,
            _ => 0,
        }
    }

    fn point(&self) -> bool {
        match &self.kind {
            DecorationKind::Widget { .. } => true,
            DecorationKind::Replace { .. } => true,
            DecorationKind::Mark { .. } => false,
        }
    }
}

impl Decoration {
    /// Creates an inline styling mark across the range.
    pub fn mark(range: StdRange<usize>, tag: &'static str) -> Range<Self> {
        Range::new(range.start, range.end, Self {
            kind: DecorationKind::Mark { tag },
        })
    }

    /// Replaces the range of text with an optional widget or hides it.
    pub fn replace(range: StdRange<usize>, widget_id: Option<&'static str>) -> Range<Self> {
        Range::new(range.start, range.end, Self {
            kind: DecorationKind::Replace { widget_id },
        })
    }

    /// Injects a widget at a specific point.
    pub fn widget(pos: usize, widget_id: &'static str, side: i32) -> Range<Self> {
        Range::new(pos, pos, Self {
            kind: DecorationKind::Widget { widget_id, side },
        })
    }
}

/// In CodeMirror 6, `DecorationSet` is an alias / specialization of `RangeSet<Decoration>`.
pub type DecorationSet = RangeSet<Decoration>;

/// Facet that collects decoration sets from all contributing extensions.
pub static DECORATIONS: LazyLock<Facet<DecorationSet, Vec<DecorationSet>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.to_vec()));
