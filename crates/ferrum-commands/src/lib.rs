//! `ferrum-commands` - Text editing commands for the Ferrum editor engine.

pub mod commands;

pub use commands::{
    delete_backward, delete_forward, insert_text, move_cursor_left, move_cursor_right, select_all,
};

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_state::state::{EditorState, EditorStateConfig};
    use ferrum_state::text::Text;

    #[test]
    fn test_insert_and_delete() {
        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("Hello")),
            selection: None,
            extensions: None,
        });

        // Insert at cursor 0
        let spec = insert_text(&state, "World ").unwrap();
        let tr = state.update(spec).unwrap();
        let state = state.apply(tr);
        assert_eq!(state.doc().to_string(), "World Hello");
        assert_eq!(state.selection().main().head, 6);

        // Delete backward (Backspace)
        let spec = delete_backward(&state).unwrap();
        let tr = state.update(spec).unwrap();
        let state = state.apply(tr);
        assert_eq!(state.doc().to_string(), "WorldHello");
        assert_eq!(state.selection().main().head, 5);
    }

    #[test]
    fn test_navigation_and_select_all() {
        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("Ferrum")),
            selection: None,
            extensions: None,
        });

        // Move right with selection
        let spec = move_cursor_right(&state, true).unwrap();
        let tr = state.update(spec).unwrap();
        let state = state.apply(tr);
        assert_eq!(state.selection().main().from(), 0);
        assert_eq!(state.selection().main().to(), 1);

        // Select all
        let spec = select_all(&state).unwrap();
        let tr = state.update(spec).unwrap();
        let state = state.apply(tr);
        assert_eq!(state.selection().main().from(), 0);
        assert_eq!(state.selection().main().to(), 6);
    }
}
