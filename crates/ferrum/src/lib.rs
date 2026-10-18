//! `ferrum` - Embeddable, high-performance, modular text editor engine in Rust.

pub use ferrum_state as state;

#[cfg(feature = "commands")]
pub use ferrum_commands as commands;

#[cfg(feature = "view")]
pub use ferrum_view as view;

// Convenient top-level re-exports:
pub use ferrum_state::{
    ChangeSet, ChangeSpec, Compartment, EditorSelection, EditorState, EditorStateConfig,
    Extension, Facet, Prec, SelectionRange, StateEffect, StateField, Text, Transaction,
    TransactionSpec,
};

#[cfg(feature = "view")]
pub use ferrum_view::{Decoration, DecorationKind, DecorationSet, EditorView, ViewUpdate, Viewport};

#[cfg(feature = "commands")]
pub use ferrum_commands::{
    cursor_char_left, cursor_char_right, default_keymap, delete_char_backward, delete_char_forward,
    dispatch_key, history, insert_text, redo, select_all, standard_keymap, undo, HistoryConfig,
    KeyBinding, StateCommand, KEYMAP,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ferrum_facade_integration() {
        // Define a custom facet in consumer code
        let theme_facet: Facet<&'static str, &'static str> =
            Facet::define(|inputs| inputs.last().copied().unwrap_or("dark"));

        // Create an EditorState with extensions
        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("fn main() {\n    println!(\"Ferrum!\");\n}")),
            selection: None,
            extensions: Some(vec![theme_facet.of("solarized")]),
        });

        assert_eq!(*state.facet(&theme_facet), "solarized");

        // Instantiate the view
        let mut view = EditorView::new(state);
        assert_eq!(view.state().doc().len_lines(), 3);

        // Execute a high-level command through the facade
        let spec = insert_text(view.state(), "// Header\n").unwrap();
        let update = view.dispatch(spec).unwrap();
        assert!(update.doc_changed);
        assert_eq!(view.state().doc().len_lines(), 4);
    }
}
