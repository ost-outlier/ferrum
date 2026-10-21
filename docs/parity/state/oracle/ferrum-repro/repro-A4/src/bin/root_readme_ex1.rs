// Verbatim copy of README.md:116-151.
use ferrum::state::{EditorState, EditorStateConfig, Text, TransactionSpec};
use ferrum::commands::{
    default_keymap, history, insert_text, undo, redo, HistoryConfig, KEYMAP,
};
use ferrum::view::EditorView;

fn main() {
    // 1. Configure extensions (keymaps and undo/redo history)
    let extensions = vec![
        history(HistoryConfig::default()),
        KEYMAP.of(default_keymap()),
    ];

    // 2. Create the immutable EditorState
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("fn main() {\n    println!(\"Hello, Ferrum!\");\n}\n")),
        selection: None,
        extensions: Some(extensions),
    });

    // 3. Attach an abstract EditorView
    let mut view = EditorView::new(state);

    // 4. Dispatch transactional edits
    let insert_spec = insert_text(view.state(), "// Edited with Ferrum\n").unwrap();
    view.dispatch(insert_spec);

    println!("Document:\n{}", view.state().doc());

    // 5. Undo the modification
    let undo_spec = undo(view.state()).unwrap();
    view.dispatch(undo_spec);
    println!("After undo:\n{}", view.state().doc());
}
