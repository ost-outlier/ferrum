use super::*;
use std::sync::Arc;

#[test]
fn test_text_rope_operations() {
    let doc = Text::from("Hello, World!\nSecond Line\r\nThird Line");
    assert_eq!(doc.len_lines(), 3);
    assert_eq!(doc.line(0).number, 1);
    assert_eq!(doc.line(0).length, 13);
    assert_eq!(doc.slice_to_string(0..5), "Hello");

    let edited = doc.replace(7..12, "Ferrum");
    assert_eq!(edited.slice_to_string(0..14), "Hello, Ferrum!");
}

#[test]
fn test_selection_normalization() {
    let sel = EditorSelection::create(
        vec![
            SelectionRange::new(10, 15),
            SelectionRange::new(5, 8),
            SelectionRange::new(12, 20), // Overlaps with 10..15!
        ],
        0,
    );

    // Overlapping ranges 10..15 and 12..20 must merge into 10..20!
    assert_eq!(sel.ranges().len(), 2);
    assert_eq!(sel.ranges()[0].from(), 5);
    assert_eq!(sel.ranges()[0].to(), 8);
    assert_eq!(sel.ranges()[1].from(), 10);
    assert_eq!(sel.ranges()[1].to(), 20);
}

#[test]
fn test_static_facets_and_defaults() {
    let tab_size: Facet<usize, usize> = Facet::define(|inputs| inputs.last().copied().unwrap_or(4));
    let unregistered_facet: Facet<bool, bool> = Facet::define(|inputs| inputs.iter().any(|&v| v));

    // 1. Unregistered facet returns default value directly
    let state = EditorState::create(EditorStateConfig::default());
    assert_eq!(*state.facet(&tab_size), 4);
    assert_eq!(*state.facet(&unregistered_facet), false);

    // 2. Multiple extensions contributing to the same facet coexisting without conflict
    let state2 = EditorState::create(EditorStateConfig {
        doc: None,
        selection: None,
        extensions: Some(vec![
            tab_size.of(2),
            tab_size.of(8), // Last registered wins according to combine logic
        ]),
    });
    assert_eq!(*state2.facet(&tab_size), 8);
}

#[test]
fn test_facet_precedence() {
    let greeting: Facet<&'static str, Vec<&'static str>> = Facet::define_list();

    let state = EditorState::create(EditorStateConfig {
        doc: None,
        selection: None,
        extensions: Some(vec![
            greeting.of("middle").with_prec(Prec::Default),
            greeting.of("first").with_prec(Prec::Highest),
            greeting.of("last").with_prec(Prec::Lowest),
        ]),
    });

    let result = state.facet(&greeting);
    assert_eq!(result, &vec!["first", "middle", "last"]);
}

#[test]
fn test_dynamic_facets_and_state_fields() {
    // A StateField tracking the total number of applied transactions
    let tx_counter = StateField::define(|_state| 0usize, |count, _tr| count + 1);

    // A dynamic facet computing the character count from EditorState
    let char_count: Facet<usize, usize> = Facet::define(|inputs| inputs.iter().sum());

    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("Ferrum")),
        selection: None,
        extensions: Some(vec![
            tx_counter.extension(),
            char_count.compute(|state| state.doc().len_chars()),
        ]),
    });

    assert_eq!(state.field(&tx_counter), Some(&0));
    assert_eq!(*state.facet(&char_count), 6);

    // Apply an insertion transaction
    let spec = TransactionSpec::new().with_change(ChangeSpec::insert(6, " Editor"));
    let tr = state.update(spec).expect("Transaction should succeed");
    let state = state.apply(tr);

    assert_eq!(state.doc().to_string(), "Ferrum Editor");
    assert_eq!(state.field(&tx_counter), Some(&1));
    assert_eq!(*state.facet(&char_count), 13);
}

#[test]
fn test_transaction_filter() {
    // Filter that forbids typing the character 'X'
    let block_x: TransactionFilterFn = Arc::new(|tr| {
        for change in tr.changes().changes() {
            if change.insert.contains('X') {
                return false;
            }
        }
        true
    });

    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("Hello")),
        selection: None,
        extensions: Some(vec![TRANSACTION_FILTER.of(block_x)]),
    });

    // Valid edit: allowed
    let valid_spec = TransactionSpec::new().with_change(ChangeSpec::insert(5, "!"));
    assert!(state.update(valid_spec).is_some());

    // Invalid edit with 'X': rejected by transactionFilter
    let invalid_spec = TransactionSpec::new().with_change(ChangeSpec::insert(5, "X"));
    assert!(state.update(invalid_spec).is_none());
}

#[test]
fn test_compartment_reconfiguration() {
    let mode_facet: Facet<&'static str, &'static str> =
        Facet::define(|inputs| inputs.last().copied().unwrap_or("normal"));

    let mode_compartment = Compartment::new();

    let state = EditorState::create(EditorStateConfig {
        doc: None,
        selection: None,
        extensions: Some(vec![mode_compartment.of(mode_facet.of("insert"))]),
    });

    assert_eq!(*state.facet(&mode_facet), "insert");

    // Hot-reconfigure the compartment inside a transaction
    let reconfig_effect = mode_compartment.reconfigure(mode_facet.of("visual"));
    let spec = TransactionSpec::new().with_effect(reconfig_effect);
    let tr = state.update(spec).expect("Reconfiguration transaction");
    let state = state.apply(tr);

    assert_eq!(*state.facet(&mode_facet), "visual");
}
