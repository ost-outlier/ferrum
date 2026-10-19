//! Syntax highlighting tags and styles.
//! Faithful port of CodeMirror 6's `@lezer/highlight` and `@codemirror/language/src/highlight.ts`.

use std::sync::atomic::{AtomicUsize, Ordering};

static TAG_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// A syntax highlighting tag representing a lexical role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag {
    pub id: usize,
}

impl Tag {
    /// Defines a new unique tag.
    pub fn define() -> Self {
        Self {
            id: TAG_COUNTER.fetch_add(1, Ordering::SeqCst),
        }
    }
}
