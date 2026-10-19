//! Text decorations and `DecorationSet` based directly on the canonical CodeMirror 6 `RangeSet` model.

use ferrum_state::facet::Facet;
use ferrum_state::rangeset::{Range, RangeSet, RangeValue};
use std::ops::Range as StdRange;
use std::sync::LazyLock;

use std::sync::Arc;

/// Trait implemented by widgets rendered inside the document or between lines.
pub trait WidgetType: std::fmt::Debug + Send + Sync {
    /// Returns the estimated height of this widget. Defaults to -1.0 (unspecified).
    fn estimated_height(&self) -> f64 {
        -1.0
    }
}

/// Specification for a widget decoration.
#[derive(Debug, Clone, Default)]
pub struct WidgetSpec {
    pub widget: Option<Arc<dyn WidgetType>>,
    pub side: i32,
    pub block: bool,
}

/// Specification for a replacement decoration.
#[derive(Debug, Clone, Default)]
pub struct ReplaceSpec {
    pub widget: Option<Arc<dyn WidgetType>>,
    pub block: bool,
}

/// The type and attributes of a decoration applied to text.
#[derive(Debug, Clone)]
pub enum DecorationKind {
    /// Applies inline styling or a CSS/theme tag to a range of text.
    Mark {
        tag: &'static str,
    },
    /// Replaces a range of text with alternative rendering or hides it (essential for Live Preview).
    Replace {
        widget_id: Option<&'static str>,
        block: bool,
        widget: Option<Arc<dyn WidgetType>>,
    },
    /// Injects an inline or block widget at a specific point without replacing characters.
    Widget {
        widget_id: Option<&'static str>,
        side: i32, // Negative before position, positive after position
        block: bool,
        widget: Option<Arc<dyn WidgetType>>,
    },
}

impl PartialEq for DecorationKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DecorationKind::Mark { tag: a }, DecorationKind::Mark { tag: b }) => a == b,
            (
                DecorationKind::Replace { widget_id: a, block: ba, widget: wa },
                DecorationKind::Replace { widget_id: b, block: bb, widget: wb },
            ) => {
                a == b && ba == bb && match (wa, wb) {
                    (Some(x), Some(y)) => Arc::ptr_eq(x, y),
                    (None, None) => true,
                    _ => false,
                }
            }
            (
                DecorationKind::Widget { widget_id: a, side: sa, block: ba, widget: wa },
                DecorationKind::Widget { widget_id: b, side: sb, block: bb, widget: wb },
            ) => {
                a == b && sa == sb && ba == bb && match (wa, wb) {
                    (Some(x), Some(y)) => Arc::ptr_eq(x, y),
                    (None, None) => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl Eq for DecorationKind {}

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
            kind: DecorationKind::Replace { widget_id, block: false, widget: None },
        })
    }

    /// Replaces the range of text using a `ReplaceSpec`.
    pub fn replace_spec(range: StdRange<usize>, spec: ReplaceSpec) -> Range<Self> {
        Range::new(range.start, range.end, Self {
            kind: DecorationKind::Replace { widget_id: None, block: spec.block, widget: spec.widget },
        })
    }

    /// Injects a widget at a specific point.
    pub fn widget(pos: usize, widget_id: &'static str, side: i32) -> Range<Self> {
        Range::new(pos, pos, Self {
            kind: DecorationKind::Widget { widget_id: Some(widget_id), side, block: false, widget: None },
        })
    }

    /// Injects a widget using a `WidgetSpec`.
    pub fn widget_spec(pos: usize, spec: WidgetSpec) -> Range<Self> {
        Range::new(pos, pos, Self {
            kind: DecorationKind::Widget { widget_id: None, side: spec.side, block: spec.block, widget: spec.widget },
        })
    }
}

/// In CodeMirror 6, `DecorationSet` is an alias / specialization of `RangeSet<Decoration>`.
pub type DecorationSet = RangeSet<Decoration>;

/// Facet that collects decoration sets from all contributing extensions.
pub static DECORATIONS: LazyLock<Facet<DecorationSet, Vec<DecorationSet>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.to_vec()));
