//! `ferrum-view` - View abstractions, decorations, viewport virtualization, and surface contracts.

pub mod decoration;
pub mod view;
pub mod viewport;

pub use decoration::{Decoration, DecorationKind, DecorationSet, DECORATIONS};
pub use view::{DecoratedRange, EditorView, RenderedLine, ViewUpdate};
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
        let decos = DecorationSet::of_sorted(vec![
            Decoration::mark(0..5, "keyword"),
            Decoration::replace(7..12, Some("pill")),
        ], true).expect("Valid DecorationSet");

        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from_str("const count = 42;")),
            selection: None,
            extensions: Some(vec![DECORATIONS.of(decos)]),
        });

        let mut view = EditorView::new(state);
        let lines = view.render_lines();

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "const count = 42;");
        assert_eq!(lines[0].decorations.len(), 2);
        assert_eq!(lines[0].decorations[0].from, 0);
        assert_eq!(lines[0].decorations[0].to, 5);

        // Dispatch an edit via the view
        let update = view
            .dispatch(TransactionSpec::new().with_change(ChangeSpec::insert(0, "// note\n")))
            .expect("Dispatch update");

        assert!(update.doc_changed);
        assert_eq!(view.state().doc().len_lines(), 2);
    }

    #[test]
    fn test_decoration_set_ot_mapping() {
        let decos = DecorationSet::of_sorted(vec![
            Decoration::mark(6..11, "function_name"), // "hello" in "const hello = 1;"
        ], true).expect("Valid DecorationSet");

        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from_str("const hello = 1;")),
            selection: None,
            extensions: Some(vec![DECORATIONS.of(decos.clone())]),
        });

        let view = EditorView::new(state);

        // Insert "pub " at 0 (+4 chars)
        let tr_spec = TransactionSpec::new().with_change(ChangeSpec::insert(0, "pub "));
        let tr = view.state().update(tr_spec).unwrap();
        let mapped_decos = decos.map(tr.changes().desc());

        let mapped_ranges = mapped_decos.iter();
        assert_eq!(mapped_ranges.len(), 1);
        assert_eq!(mapped_ranges[0].from, 10);
        assert_eq!(mapped_ranges[0].to, 15);
    }
}
