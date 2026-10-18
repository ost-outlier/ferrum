use super::*;
use std::sync::Arc;

fn mk(spec: &str) -> ChangeDesc {
    ChangeDesc::from_string(spec)
}

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
fn test_change_desc_composition() {
    fn comp(specs: &[&str], expected: &str) {
        let mut curr = mk(specs[0]);
        for &s in &specs[1..] {
            curr = curr.compose_desc(&mk(s));
        }
        assert_eq!(curr.to_string(), expected);
    }

    // Ported directly from CodeMirror 6 test-change.ts
    comp(&["5 0:2", "1 2:0 4"], "1 2:0 2 0:2");
    comp(&["2 0:2 2", "2 2:0 2"], "4");
    comp(&["2 0:2 2", "4 0:3 2"], "2 0:5 2");
    comp(&["2 5:0", "1 1:0"], "1 6:0");
    comp(&["2 2:0 0:3", "5:0"], "4:0");
    comp(&["", "0:8", "8:0", "", ""], "");
    comp(&["2 2:2 2:2 2", "1 2:2 2:2 2:2 1"], "1 6:6 1");
    comp(&["1 2:2 2:2 2:2 1", "2 2:2 2:2 2"], "1 6:6 1");
    comp(&["1 2:3 3:2 1", "2 3:1 2"], "1 5:3 1");
}

#[test]
fn test_change_desc_mapping() {
    fn over(a: &str, b: &str, expected: &str) {
        let res = mk(a).map_desc(&mk(b), false);
        assert_eq!(res.to_string(), expected);
    }
    fn under(a: &str, b: &str, expected: &str) {
        let res = mk(a).map_desc(&mk(b), true);
        assert_eq!(res.to_string(), expected);
    }

    // Ported directly from CodeMirror 6 test-change.ts
    over("4 0:1", "0:3 4", "7 0:1");
    over("4 0:1", "2:0 2", "2 0:1");

    over("2 0:1 2", "2 0:1 2", "3 0:1 2");
    under("2 0:1 2", "2 0:1 2", "2 0:1 3");

    over("2 2:0", "2 1:2 1", "4 1:0");
    under("2 2:0", "2 1:2 1", "4 1:0");

    over("0:1 2:0 8", "6 1:0 0:5 3", "0:1 2:0 12");
    over("5:0 2 3:0 2", "4 4:0 4", "6:0 2");

    under("2 0:1 2", "4:0", "0:1");
    over("4 0:1 4", "2 4:0 2", "2 0:1 2");

    over("2 2:2 2", "0:2 6", "4 2:2 2");
    over("2 2:2 2", "3:0 3", "1:2 2");
    over("1 4:4 1", "3 0:2 3", "1 2:4 2 2:0 1");
    over("1 4:4 1", "2 2:0 2", "1 2:4 1");
    over("2 2:2 2", "3 2:0 1", "2 1:2 1");

    over("2:2 2 2:2", "2 2:0 2", "2:2 2:2");
    under("2 2:0 2", "2 2:0 2", "4");
    over("2 2:0 2", "2 2:0 2", "4");

    over("1 1:2 1", "1 1:1 1", "2 0:2 1");
    under("1 1:2 1", "1 1:1 1", "1 0:2 2");
    over("1 1:2 2", "1 2:1 1", "1 0:2 2");
    over("2 1:2 1", "1 2:1 1", "2 0:2 1");
    over("2:1 1", "1 2:2", "1:1 2");
    over("1 2:1", "2:2 1", "2 1:1");
}

#[test]
fn test_change_desc_map_pos() {
    let set = mk("4 0:2 4");
    assert_eq!(set.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(set.map_pos(4, -1, MapMode::Simple), Some(4));
    assert_eq!(set.map_pos(4, 1, MapMode::Simple), Some(6));
    assert_eq!(set.map_pos(5, -1, MapMode::Simple), Some(7));
    assert_eq!(set.map_pos(8, -1, MapMode::Simple), Some(10));

    let del_set = mk("4 4:0 4");
    assert_eq!(del_set.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(del_set.map_pos(4, -1, MapMode::Simple), Some(4));
    assert_eq!(del_set.map_pos(4, -1, MapMode::TrackDel), Some(4));
    assert_eq!(del_set.map_pos(4, -1, MapMode::TrackBefore), Some(4));
    assert_eq!(del_set.map_pos(4, -1, MapMode::TrackAfter), None);

    assert_eq!(del_set.map_pos(5, -1, MapMode::Simple), Some(4));
    assert_eq!(del_set.map_pos(5, -1, MapMode::TrackDel), None);
    assert_eq!(del_set.map_pos(5, -1, MapMode::TrackBefore), None);
    assert_eq!(del_set.map_pos(5, -1, MapMode::TrackAfter), None);

    assert_eq!(del_set.map_pos(8, -1, MapMode::TrackDel), Some(4));
    assert_eq!(del_set.map_pos(8, -1, MapMode::TrackBefore), None);
    assert_eq!(del_set.map_pos(8, -1, MapMode::TrackAfter), Some(4));
}

#[test]
fn test_changeset_creation_and_apply() {
    let doc10 = Text::from("0123456789");

    // CM6 test cases:
    let set1 = ChangeSet::of_spec(ChangeSpec::insert(5, "hi"), 10);
    assert_eq!(set1.desc().to_string(), "5 0:2 5");

    let set2 = ChangeSet::of_spec(ChangeSpec::delete(5, 7), 10);
    assert_eq!(set2.desc().to_string(), "5 2:0 3");

    let set3 = ChangeSet::of_specs(
        &[
            ChangeSpec::insert(5, "hi"),
            ChangeSpec::insert(5, "ok"),
            ChangeSpec::delete(0, 3),
            ChangeSpec::delete(4, 6),
            ChangeSpec::insert(8, "boo"),
        ],
        10,
    );
    assert_eq!(set3.desc().to_string(), "3:0 1 1:0 0:4 1:0 2 0:3 2");

    // Apply
    let applied1 = ChangeSet::of_spec(ChangeSpec::insert(2, "ok"), 10).apply(&doc10);
    assert_eq!(applied1.to_string(), "01ok23456789");

    let applied2 = ChangeSet::of_spec(ChangeSpec::delete(1, 9), 10).apply(&doc10);
    assert_eq!(applied2.to_string(), "09");

    let applied3 = ChangeSet::of_specs(
        &[ChangeSpec::delete(2, 8), ChangeSpec::insert(1, "hi")],
        10,
    )
    .apply(&doc10);
    assert_eq!(applied3.to_string(), "0hi189");
}

#[test]
fn test_changeset_compose_and_invert() {
    let doc10 = Text::from("0123456789");

    // Compose
    let set_a = ChangeSet::of_spec(ChangeSpec::insert(8, "ABCD"), 10);
    let set_b = ChangeSet::of_spec(ChangeSpec::delete(8, 11), 14);
    let composed = set_a.compose(&set_b);
    assert_eq!(composed.apply(&doc10).to_string(), "01234567D89");

    // Invert
    let set0 = ChangeSet::of_specs(
        &[ChangeSpec::insert(5, "hi"), ChangeSpec::delete(8, 10)],
        10,
    );
    let modified = set0.apply(&doc10);
    let inverted = set0.invert(&doc10);
    let restored = inverted.apply(&modified);
    assert_eq!(restored.to_string(), doc10.to_string());
}

#[test]
fn test_changeset_ot_confluence_property() {
    // Operational Transformation property TP1:
    // A.compose(B.map(A, false)) == B.compose(A.map(B, true))
    let doc = Text::from("The quick brown fox jumps over the lazy dog");
    let doc_len = doc.len_chars();

    // User A edits: replaces "brown fox" (pos 10..19) with "red panda"
    let edit_a = ChangeSet::of_spec(ChangeSpec::replace(10, 19, "red panda"), doc_len);

    // User B edits: replaces "lazy dog" (pos 35..43) with "sleepy cat" and inserts "really " at 4
    let edit_b = ChangeSet::of_specs(
        &[
            ChangeSpec::insert(4, "really "),
            ChangeSpec::replace(35, 43, "sleepy cat"),
        ],
        doc_len,
    );

    let b_over_a = edit_b.map(edit_a.desc(), false);
    let a_over_b = edit_a.map(edit_b.desc(), true);

    let ab = edit_a.compose(&b_over_a);
    let ba = edit_b.compose(&a_over_b);

    let res_ab = ab.apply(&doc);
    let res_ba = ba.apply(&doc);

    assert_eq!(res_ab.to_string(), res_ba.to_string());
    assert_eq!(
        res_ab.to_string(),
        "The really quick red panda jumps over the sleepy cat"
    );
}

#[test]
fn test_unicode_grapheme_clusters() {
    // Test 1: CRLF cluster
    let crlf_text = "abc\r\ndef";
    // Pos 3 is '\r'. Forward break from 3 should skip '\r\n' together to 5 ('d')
    assert_eq!(find_cluster_break(crlf_text, 3, true), 5);
    // Backward break from 5 should jump back to 3
    assert_eq!(find_cluster_break(crlf_text, 5, false), 3);

    // Test 2: Combining accent marks (e + \u{0301} = é)
    let accent_text = "e\u{0301}cole";
    // 'e' is pos 0, combining acute is pos 1. Forward break from 0 should skip both to 2 ('c')
    assert_eq!(find_cluster_break(accent_text, 0, true), 2);
    // Backward break from 2 should return to 0
    assert_eq!(find_cluster_break(accent_text, 2, false), 0);

    // Test 3: Complex ZWJ emoji family (👨‍👩‍👧‍👦 = man + ZWJ + woman + ZWJ + girl + ZWJ + boy)
    let family_emoji = "A👨‍👩‍👧‍👦B";
    let emoji_chars = "👨‍👩‍👧‍👦".chars().count(); // 7 codepoints
    let end_of_emoji = 1 + emoji_chars;
    // Moving forward from 1 should jump over the ENTIRE family emoji
    assert_eq!(find_cluster_break(family_emoji, 1, true), end_of_emoji);
    // Moving backward from end_of_emoji should jump all the way back to 1
    assert_eq!(find_cluster_break(family_emoji, end_of_emoji, false), 1);
}

#[test]
fn test_multi_cursor_change_by_range() {
    // Document with 3 identical words: "item item item"
    let doc = Text::from("item item item");
    // Selection with 3 cursors at the start of each "item" (0, 5, 10)
    let sel = EditorSelection::create(
        vec![
            SelectionRange::cursor(0),
            SelectionRange::cursor(5),
            SelectionRange::cursor(10),
        ],
        0,
    );

    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: Some(sel),
        extensions: None,
    });

    // Simultaneously insert "new_" at all 3 cursor positions via change_by_range
    let result = state.change_by_range(|range| {
        RangeResult::range(SelectionRange::cursor(range.head + 4))
            .with_changes(ChangeSpec::insert(range.head, "new_"))
    });

    let tr = state.update(result.into_spec()).unwrap();
    let new_state = state.apply(tr);

    assert_eq!(new_state.doc().to_string(), "new_item new_item new_item");
    assert_eq!(new_state.selection().ranges().len(), 3);
    assert_eq!(new_state.selection().ranges()[0].head, 4);
    assert_eq!(new_state.selection().ranges()[1].head, 13);
    assert_eq!(new_state.selection().ranges()[2].head, 22);
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
        let mut allowed = true;
        tr.changes().iter_changes(
            |_, _, _, _, inserted| {
                if inserted.to_string().contains('X') {
                    allowed = false;
                }
            },
            false,
        );
        allowed
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

#[test]
fn test_inverted_selection_mapping() {
    let _doc = Text::from("0123456789");
    // Selection from 8 back to 2 (inverted: anchor = 8, head = 2)
    let sel = EditorSelection::single(8, 2);
    assert!(sel.main().is_inverted());
    assert_eq!(sel.main().from(), 2);
    assert_eq!(sel.main().to(), 8);

    // Change at 4: insert "ABCD"
    let cs = ChangeSet::of_spec(ChangeSpec::insert(4, "ABCD"), 10);
    let mapped = sel.map(cs.desc(), -1);
    assert!(mapped.main().is_inverted());
    assert_eq!(mapped.main().head, 2);
    assert_eq!(mapped.main().anchor, 12);
}

#[test]
fn test_empty_transaction_preserves_state() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("Immutable")),
        selection: Some(EditorSelection::single(2, 5)),
        extensions: None,
    });

    let tr = state.update(TransactionSpec::new()).unwrap();
    assert!(!tr.doc_changed());
    let next_state = state.apply(tr);
    assert_eq!(next_state.doc().to_string(), "Immutable");
    assert_eq!(next_state.selection().main().anchor, 2);
    assert_eq!(next_state.selection().main().head, 5);
}

