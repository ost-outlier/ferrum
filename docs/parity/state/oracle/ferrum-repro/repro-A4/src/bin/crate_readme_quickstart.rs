// Verbatim copy of crates/ferrum-state/README.md:57-80, wrapped in fn main.
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::selection::EditorSelection;
use ferrum_state::transaction::TransactionSpec;
use ferrum_state::change::ChangeSpec;

fn main() {
// Initialize state
let state = EditorState::create(EditorStateConfig {
    doc: Some("Hello, world!".into()),
    selection: Some(EditorSelection::cursor(5)),
    extensions: None,
});

// Create a transactional update
let spec = TransactionSpec::new()
    .with_change(ChangeSpec::replace(5, 12, " Ferrum"))
    .with_selection(EditorSelection::cursor(14));

let tr = state.update(spec).expect("valid transaction");
let new_state = state.apply(tr);

assert_eq!(new_state.doc().to_string(), "Hello Ferrum!");
assert_eq!(new_state.selection().main().head, 14);
println!("quickstart OK; doc len = {}, main head = {}", new_state.doc().len_chars(), new_state.selection().main().head);
}
