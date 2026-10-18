use crate::text::Text;

/// A specification for a single atomic change to the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSpec {
    /// Start character position of the range to replace.
    pub from: usize,
    /// End character position of the range to replace.
    pub to: usize,
    /// Text to insert at the range.
    pub insert: String,
}

impl ChangeSpec {
    /// Creates a deletion change.
    pub fn delete(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            insert: String::new(),
        }
    }

    /// Creates an insertion change.
    pub fn insert(at: usize, text: impl Into<String>) -> Self {
        Self {
            from: at,
            to: at,
            insert: text.into(),
        }
    }

    /// Creates a replacement change.
    pub fn replace(from: usize, to: usize, text: impl Into<String>) -> Self {
        Self {
            from,
            to,
            insert: text.into(),
        }
    }
}

/// A validated, ordered collection of atomic changes applied to a document.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangeSet {
    changes: Vec<ChangeSpec>,
}

impl ChangeSet {
    /// Creates an empty change set.
    pub fn empty() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    /// Creates a change set containing a single change.
    pub fn single(change: ChangeSpec) -> Self {
        Self {
            changes: vec![change],
        }
    }

    /// Creates a change set from multiple specs, ensuring they are sorted and non-overlapping.
    pub fn from_specs(mut specs: Vec<ChangeSpec>) -> Self {
        specs.sort_by_key(|c| c.from);
        Self { changes: specs }
    }

    /// Returns true if there are no changes.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Returns the slice of individual changes.
    pub fn changes(&self) -> &[ChangeSpec] {
        &self.changes
    }

    /// Applies this change set to a `Text` buffer, returning the new `Text`.
    pub fn apply(&self, doc: &Text) -> Text {
        if self.changes.is_empty() {
            return doc.clone();
        }

        // Apply changes in reverse order so character offsets remain valid
        let mut result = doc.clone();
        for change in self.changes.iter().rev() {
            result = result.replace(change.from..change.to, &change.insert);
        }
        result
    }

    /// Maps a character position through these changes.
    /// `assoc`: bias when the position lands inside an edited region (-1 = stay at start, 1 = move to end).
    pub fn map_pos(&self, mut pos: usize, assoc: i8) -> usize {
        for change in &self.changes {
            let delete_len = change.to - change.from;
            let insert_len = change.insert.chars().count();

            if pos < change.from {
                // Position is before the change, untouched
                break;
            } else if pos > change.to {
                // Position is after the change, shift by the delta
                if insert_len >= delete_len {
                    pos += insert_len - delete_len;
                } else {
                    pos -= delete_len - insert_len;
                }
            } else {
                // Position is inside the replaced range
                if assoc <= 0 {
                    pos = change.from;
                } else {
                    pos = change.from + insert_len;
                }
            }
        }
        pos
    }
}
