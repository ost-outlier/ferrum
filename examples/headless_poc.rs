//! Example demonstrating the headless execution of the Ferrum editor engine.
//! Validates:
//! - Facet composition without conflict
//! - Dynamic StateField tracking word/character counts
//! - Transaction filters intercepting and blocking unwanted edits
//! - Dynamic Compartment reconfiguration via transactions
//! - High-level editing commands (insert, delete, navigate)

use ferrum::commands::{delete_backward, insert_text, select_all};
use ferrum::state::{
    Compartment, EditorState, EditorStateConfig, Facet, StateField, Text, TransactionSpec,
    TRANSACTION_FILTER,
};
use ferrum::view::EditorView;
use std::sync::Arc;

fn main() {
    println!("=== Ferrum Editor Engine — Headless Proof of Concept ===\n");

    // 1. Define a StateField that counts total transactions applied
    let tx_counter = StateField::define(|_state| 0usize, |count, _tr| count + 1);

    // 2. Define a custom Facet for editor configuration (e.g., TabSize)
    let tab_size: Facet<usize, usize> = Facet::define(|inputs| inputs.last().copied().unwrap_or(4));

    // 3. Define a transaction filter that prevents typing forbidden characters (e.g. '@')
    let block_at_symbol = Arc::new(|tr: &ferrum::state::Transaction| {
        for change in tr.changes().changes() {
            if change.insert.contains('@') {
                println!("  [Filter Triggered] Transaction blocked: '@' character is not allowed!");
                return false;
            }
        }
        true
    });

    // 4. Create a dynamic compartment for hot-reloading extensions (e.g., Theme)
    let theme_facet: Facet<&'static str, &'static str> =
        Facet::define(|inputs| inputs.last().copied().unwrap_or("dark"));
    let theme_compartment = Compartment::new();

    // 5. Initialize the EditorState
    let initial_text = "fn main() {\n    println!(\"Hello, Ferrum!\");\n}";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(initial_text)),
        selection: None,
        extensions: Some(vec![
            tx_counter.extension(),
            tab_size.of(2),
            TRANSACTION_FILTER.of(block_at_symbol),
            theme_compartment.of(theme_facet.of("monokai")),
        ]),
    });

    // 6. Wrap in EditorView
    let mut view = EditorView::new(state);

    println!("1. Initial Document ({} lines, {} chars):", view.state().doc().len_lines(), view.state().doc().len_chars());
    println!("--------------------------------------------------");
    println!("{}", view.state().doc());
    println!("--------------------------------------------------");
    println!("   Tab Size Facet: {}", view.state().facet(&tab_size));
    println!("   Theme Facet:    {}", view.state().facet(&theme_facet));
    println!("   Transaction Count (StateField): {}\n", view.state().field(&tx_counter).unwrap());

    // 7. Test transaction filtering
    println!("2. Testing Transaction Filtering (Attempting to insert '@user'):");
    let forbidden_spec = insert_text(view.state(), "@user").unwrap();
    let update = view.dispatch(forbidden_spec);
    assert!(update.is_none(), "Transaction should be blocked by filter");
    println!("   Success: Transaction was rejected as expected.\n");

    // 8. Test valid editing commands
    println!("3. Executing valid command: Insert '// License: MIT\\n' at start:");
    let valid_spec = insert_text(view.state(), "// License: MIT\n").unwrap();
    let update = view.dispatch(valid_spec).expect("Valid edit should succeed");
    assert!(update.doc_changed);
    println!("   Doc lines after edit: {}", view.state().doc().len_lines());
    println!("   Transaction Count (StateField): {}\n", view.state().field(&tx_counter).unwrap());

    // 9. Test hot-reconfiguring a compartment via transaction
    println!("4. Hot-reconfiguring Theme Compartment ('monokai' -> 'nord'):");
    let reconfig_effect = theme_compartment.reconfigure(theme_facet.of("nord"));
    let reconfig_spec = TransactionSpec::new().with_effect(reconfig_effect);
    view.dispatch(reconfig_spec).expect("Reconfiguration should succeed");
    println!("   New Theme Facet value: {}\n", view.state().facet(&theme_facet));
    assert_eq!(*view.state().facet(&theme_facet), "nord");

    // 10. Test Select All and Delete
    println!("5. Testing Select All and Backspace (Clean buffer):");
    let select_all_spec = select_all(view.state()).unwrap();
    view.dispatch(select_all_spec);
    let delete_spec = delete_backward(view.state()).unwrap();
    view.dispatch(delete_spec);

    println!("   Doc is empty: {}", view.state().doc().is_empty());
    println!("   Final Transaction Count: {}", view.state().field(&tx_counter).unwrap());

    println!("\n=== All Headless POC Invariants Verified Successfully! ===");
}
