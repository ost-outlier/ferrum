//! `ferrum-view` - View abstractions, decorations, viewport virtualization, and surface contracts.

pub mod decoration;
pub mod view;
pub mod viewport;

pub use decoration::{Decoration, DecorationKind, DecorationSet, DECORATIONS};
pub use view::{EditorView, RenderedLine, ViewUpdate};
pub use viewport::Viewport;

#[cfg(test)]
mod tests {
    use super::*;
    use ferrum_state::change::ChangeSpec;
    use ferrum_state::state::{EditorState, EditorStateConfig};
    use ferrum_state::text::Text;
    use ferrum_state::transaction::TransactionSpec;

    #[test]
    fn test_editor_view_and_decorations() {
        let decos = DecorationSet::from_vec(vec![
            Decoration::mark(0..5, "keyword"),
            Decoration::replace(7..12, Some("pill")),
        ]);

        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("const count = 42;")),
            selection: None,
            extensions: Some(vec![DECORATIONS.of(decos)]),
        });

        let mut view = EditorView::new(state);
        let lines = view.render_lines();

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "const count = 42;");
        assert_eq!(lines[0].decorations.len(), 2);

        // Dispatch an edit via the view
        let update = view
            .dispatch(TransactionSpec::new().with_change(ChangeSpec::insert(0, "// note\n")))
            .expect("Dispatch update");

        assert!(update.doc_changed);
        assert_eq!(view.state().doc().len_lines(), 2);
    }
}
