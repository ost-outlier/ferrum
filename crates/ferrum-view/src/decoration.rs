use ferrum_state::facet::Facet;
use std::ops::Range;
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
        side: i8, // -1 before position, 1 after position
    },
}

/// A decoration attached to a character range in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoration {
    pub from: usize,
    pub to: usize,
    pub kind: DecorationKind,
}

impl Decoration {
    /// Creates an inline styling mark across the range.
    pub fn mark(range: Range<usize>, tag: &'static str) -> Self {
        Self {
            from: range.start,
            to: range.end,
            kind: DecorationKind::Mark { tag },
        }
    }

    /// Replaces the range of text with an optional widget or hides it.
    pub fn replace(range: Range<usize>, widget_id: Option<&'static str>) -> Self {
        Self {
            from: range.start,
            to: range.end,
            kind: DecorationKind::Replace { widget_id },
        }
    }

    /// Injects a widget at a specific point.
    pub fn widget(pos: usize, widget_id: &'static str, side: i8) -> Self {
        Self {
            from: pos,
            to: pos,
            kind: DecorationKind::Widget { widget_id, side },
        }
    }
}

/// An ordered set of decorations covering a document.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DecorationSet {
    decorations: Vec<Decoration>,
}

impl DecorationSet {
    /// Creates an empty decoration set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a decoration set from a vector of decorations, ensuring they are sorted.
    pub fn from_vec(mut decos: Vec<Decoration>) -> Self {
        decos.sort_by_key(|d| (d.from, d.to));
        Self { decorations: decos }
    }

    /// Returns an iterator over all decorations.
    pub fn iter(&self) -> impl Iterator<Item = &Decoration> {
        self.decorations.iter()
    }

    /// Returns true if there are no decorations in this set.
    pub fn is_empty(&self) -> bool {
        self.decorations.is_empty()
    }

    /// Returns the number of decorations in this set.
    pub fn len(&self) -> usize {
        self.decorations.len()
    }
}

/// Facet that collects decoration sets from all contributing extensions.
pub static DECORATIONS: LazyLock<Facet<DecorationSet, Vec<DecorationSet>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.to_vec()));
