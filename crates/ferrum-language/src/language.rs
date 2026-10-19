//! Core Language abstraction, syntax tree accessors, and language facets.
//! Faithful port of CodeMirror 6's `@codemirror/language/src/language.ts`.

use crate::tree::{Parser, SyntaxTree};
use ferrum_state::facet::{Extension, Facet};
use ferrum_state::state::EditorState;
use std::sync::{Arc, LazyLock};

/// A language definition providing parsing capabilities and configuration.
#[derive(Clone)]
pub struct Language {
    pub name: String,
    pub parser: Arc<dyn Parser>,
}

impl Language {
    pub fn new(name: impl Into<String>, parser: Arc<dyn Parser>) -> Self {
        Self {
            name: name.into(),
            parser,
        }
    }
}

/// Facet that provides the active language definition for an editor state.
pub static LANGUAGE: LazyLock<Facet<Language, Option<Language>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.last().cloned()));

/// Helper bundling a language with its companion extensions.
pub struct LanguageSupport {
    pub language: Language,
    pub support: Extension,
}

impl LanguageSupport {
    pub fn new(language: Language, support: Extension) -> Self {
        Self { language, support }
    }
}

/// Returns the current syntax tree for the document in the state.
pub fn syntax_tree(state: &EditorState) -> SyntaxTree {
    if let Some(lang) = state.facet(&LANGUAGE) {
        lang.parser.parse(&state.doc().to_string(), &[], &[(0, state.doc().len_chars())])
    } else {
        SyntaxTree::empty()
    }
}

/// Ensures the syntax tree is parsed up to the given position, if available.
pub fn ensure_syntax_tree(state: &EditorState, upto: usize) -> Option<SyntaxTree> {
    let tree = syntax_tree(state);
    if tree.length >= upto {
        Some(tree)
    } else {
        None
    }
}
