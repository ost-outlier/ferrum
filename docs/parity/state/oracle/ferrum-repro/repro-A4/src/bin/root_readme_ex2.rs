// README.md:155-175 verbatim, preceded by the minimal setup of example 1 (state + view) so `view` exists.
use ferrum::state::{EditorState, EditorStateConfig, Text};
use ferrum::state::{EditorSelection, SelectionRange, TransactionSpec};
use ferrum::commands::insert_text;
use ferrum::view::EditorView;

fn main() {
let state = EditorState::create(EditorStateConfig {
    doc: Some(Text::from_str("fn main() {\n    println!(\"Hello, Ferrum!\");\n}\n")),
    selection: None,
    extensions: None,
});
let mut view = EditorView::new(state);
// ---- verbatim from here ----
// Create multiple simultaneous selection carets
let multi_selection = EditorSelection::create(
    vec![
        SelectionRange::cursor(0),
        SelectionRange::cursor(24),
    ],
    0, // Primary cursor index
);

// Dispatch selection update
view.dispatch(TransactionSpec::new().with_selection(multi_selection));

// Type concurrently across all cursors with automatic OT mapping
let typing_spec = insert_text(view.state(), "    // [TAG] ").unwrap();
view.dispatch(typing_spec);
// ---- end verbatim ----
println!("{:?}", view.state().doc().to_string());
println!("{:?}", view.state().selection().ranges().iter().map(|r| (r.anchor, r.head)).collect::<Vec<_>>());
}
