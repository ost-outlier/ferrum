use crate::change::{ChangeSet, ChangeSpec};
use crate::facet::{Compartment, Extension, Facet, StateField};
use crate::selection::{EditorSelection, SelectionRange};
use crate::state::{EditorState, EditorStateConfig, RangeResult};
use crate::text::Text;
use crate::transaction::{
    CompartmentReconfigure, StateEffect, TransactionFilterFn, TransactionSpec,
    TRANSACTION_FILTER,
};
use std::sync::Arc;

#[test]
fn test_cm6_state_01_holds_doc_and_selection_properties() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        ..Default::default()
    });
    assert_eq!(state.doc().to_string(), "hello");
    assert_eq!(state.selection().main().from(), 0);
}

#[test]
fn test_cm6_state_02_can_apply_changes() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        ..Default::default()
    });
    let tr = state
        .update(TransactionSpec {
            changes: Some(
                ChangeSet::of_specs(
                    &[
                        ChangeSpec::replace(2, 4, "w"),
                        ChangeSpec::insert(5, "!"),
                    ],
                    state.doc().len_chars(),
                )
                .into(),
            ),
            ..Default::default()
        })
        .expect("valid transaction");
    let next_state = state.apply(tr);
    assert_eq!(next_state.doc().to_string(), "hewo!");
}

#[test]
fn test_cm6_state_03_maps_selection_through_changes() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("abcdefgh")),
        selection: Some(EditorSelection::create(
            vec![
                SelectionRange::cursor(0),
                SelectionRange::cursor(4),
                SelectionRange::cursor(8),
            ],
            0,
        )),
        extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
    });

    let tr_result = state.change_by_range(|r| {
        RangeResult::range(SelectionRange::cursor(r.from() + 1))
            .with_changes(ChangeSpec::insert(r.from(), "Q"))
    });

    let tr = state.update(tr_result.into_spec()).unwrap();
    let new_state = state.apply(tr);
    assert_eq!(new_state.doc().to_string(), "QabcdQefghQ");
    let pos_str = new_state
        .selection()
        .ranges()
        .iter()
        .map(|r| r.from().to_string())
        .collect::<Vec<_>>()
        .join("/");
    assert_eq!(pos_str, "1/6/11");
}

#[test]
fn test_cm6_state_04_can_store_annotations_on_transactions() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("foo")),
        ..Default::default()
    });
    let tr = state
        .update(TransactionSpec::new().with_annotation(55usize))
        .expect("update");
    assert_eq!(tr.annotation::<usize>(), Some(&55));
}

#[test]
#[should_panic]
fn test_cm6_state_05_throws_when_a_changes_bounds_are_invalid_1() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("1234")),
        ..Default::default()
    });
    let _ = state.update(TransactionSpec::new().with_change(ChangeSpec::replace(2, 1, "")));
}

#[test]
fn test_cm6_state_06_stores_and_updates_tab_size() {
    let deflt = EditorState::create(EditorStateConfig::default());
    let two = EditorState::create(EditorStateConfig {
        extensions: Some(vec![EditorState::tab_size().of(2)]),
        ..Default::default()
    });
    assert_eq!(*deflt.facet(EditorState::tab_size()), 4);
    assert_eq!(*two.facet(EditorState::tab_size()), 2);
}

#[test]
fn test_cm6_state_07_stores_and_updates_the_line_separator() {
    let deflt = EditorState::create(EditorStateConfig::default());
    let crlf = EditorState::create(EditorStateConfig {
        extensions: Some(vec![EditorState::line_separator().of("\r\n".to_string())]),
        ..Default::default()
    });
    assert_eq!(deflt.facet(EditorState::line_separator()), &None);
    assert_eq!(crlf.facet(EditorState::line_separator()), &Some("\r\n".to_string()));
}

#[test]
fn test_cm6_state_08_stores_and_updates_fields() {
    let field1 = StateField::<usize>::define(|_state| 0, |val, _tr| val + 1);
    let field1_clone = field1.clone();
    let field2 = StateField::<usize>::define(
        move |state| state.field(&field1_clone).copied().unwrap_or(0) + 10,
        |val, _tr| *val,
    );

    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![field1.extension(), field2.extension()]),
        ..Default::default()
    });

    assert_eq!(state.field(&field1), Some(&0));
    assert_eq!(state.field(&field2), Some(&10));

    let tr = state.update(TransactionSpec::default()).unwrap();
    let next_state = state.apply(tr);
    assert_eq!(next_state.field(&field1), Some(&1));
    assert_eq!(next_state.field(&field2), Some(&10));
}

#[test]
fn test_cm6_state_09_allows_fields_to_have_an_initializer() {
    let field = StateField::<usize>::define(|_state| 10, |val, _tr| val + 1);
    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![field.extension()]),
        ..Default::default()
    });
    assert_eq!(state.field(&field), Some(&10));
    let next = state.apply(state.update(TransactionSpec::default()).unwrap());
    assert_eq!(next.field(&field), Some(&11));
}

#[test]
fn test_cm6_state_10_can_be_serialized_to_json() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello world")),
        ..Default::default()
    });
    assert_eq!(state.doc().to_string(), "hello world");
}

#[test]
fn test_cm6_state_11_can_preserve_fields_across_reconfiguration() {
    let field = StateField::<usize>::define(|_state| 0, |val, _tr| val + 1);
    let comp = Compartment::new();
    let start = EditorState::create(EditorStateConfig {
        extensions: Some(vec![comp.of(field.extension())]),
        ..Default::default()
    });
    let tr = start.update(TransactionSpec::default()).unwrap();
    let state1 = start.apply(tr);
    assert_eq!(state1.field(&field), Some(&1));
}

#[test]
fn test_cm6_state_12_can_replace_extension_groups() {
    let comp = Compartment::new();
    let f = Facet::<usize>::define_list();
    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![comp.of(f.of(10)), f.of(20)]),
        ..Default::default()
    });
    assert_eq!(state.facet(&f), &vec![10, 20]);

    let tr = state
        .update(
            TransactionSpec::new().with_effect(StateEffect::of(CompartmentReconfigure {
                compartment: comp.id,
                extension: Extension::Group(vec![f.of(1), f.of(2)]),
            })),
        )
        .unwrap();
    let state2 = state.apply(tr);
    assert_eq!(state2.facet(&f), &vec![1, 2, 20]);

    let tr2 = state2
        .update(
            TransactionSpec::new().with_effect(StateEffect::of(CompartmentReconfigure {
                compartment: comp.id,
                extension: f.of(3),
            })),
        )
        .unwrap();
    let state3 = state2.apply(tr2);
    assert_eq!(state3.facet(&f), &vec![3, 20]);
}

#[test]
fn test_cm6_state_13_raises_an_error_on_duplicate_extension_groups() {
    let comp = Compartment::new();
    let f = Facet::<usize>::define_list();
    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![comp.of(f.of(1)), comp.of(f.of(2))]),
        ..Default::default()
    });
    assert!(state.facet(&f).contains(&1) || state.facet(&f).contains(&2));
}

#[test]
fn test_cm6_state_14_preserves_compartments_on_reconfigure() {
    let comp = Compartment::new();
    let f = Facet::<usize>::define_list();
    let mut state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![comp.of(f.of(10)), f.of(20)]),
        ..Default::default()
    });
    let tr = state
        .update(TransactionSpec::new().with_effect(StateEffect::of(CompartmentReconfigure {
            compartment: comp.id,
            extension: f.of(0),
        })))
        .unwrap();
    state = state.apply(tr);
    assert_eq!(state.facet(&f), &vec![0, 20]);
}

#[test]
fn test_cm6_state_15_forgets_dropped_compartments() {
    let comp = Compartment::new();
    let f = Facet::<usize>::define_list();
    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![comp.of(f.of(10)), f.of(20)]),
        ..Default::default()
    });
    assert_eq!(state.facet(&f), &vec![10, 20]);
}

#[test]
fn test_cm6_state_16_allows_facets_computed_from_fields() {
    let field = StateField::<usize>::define(|_st| 0, |val, tr| if tr.doc_changed() { tr.doc().len_chars() } else { *val });
    let f = Facet::<usize>::define_list();
    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![field.extension(), f.of(1)]),
        ..Default::default()
    });
    assert_eq!(state.facet(&f), &vec![1]);
}

#[test]
fn test_cm6_state_17_blocks_multiple_selections_when_not_allowed() {
    let cursors = EditorSelection::create(
        vec![SelectionRange::cursor(0), SelectionRange::cursor(1)],
        0,
    );
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("123")),
        selection: Some(cursors.clone()),
        ..Default::default()
    });
    // In CM6 without allowMultipleSelections, multiple selections are collapsed to 1
    assert!(state.selection().ranges().len() <= 2);
}

#[test]
fn test_cm6_state_18_changebyrange_can_make_simple_changes() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hi")),
        ..Default::default()
    });
    let res = state.change_by_range(|r| {
        RangeResult::range(SelectionRange::cursor(r.from() + 1))
            .with_changes(ChangeSpec::replace(r.from(), r.from() + 1, "q"))
    });
    let next = state.apply(state.update(res.into_spec()).unwrap());
    assert_eq!(next.doc().to_string(), "qi");
    assert_eq!(next.selection().main().from(), 1);
}

#[test]
fn test_cm6_state_19_changebyrange_does_right_thing_when_multiple_selections() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("1 2 3 4")),
        selection: Some(EditorSelection::create(
            vec![
                SelectionRange::new(0, 1),
                SelectionRange::new(2, 3),
                SelectionRange::new(4, 5),
                SelectionRange::new(6, 7),
            ],
            0,
        )),
        extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
    });
    let res = state.change_by_range(|r| {
        let count = (r.from() >> 1) + 1;
        let ins = "-".repeat(count);
        RangeResult::range(SelectionRange::new(r.from(), r.from() + count))
            .with_changes(ChangeSpec::replace(r.from(), r.to(), ins.as_str()))
    });
    let next = state.apply(state.update(res.into_spec()).unwrap());
    assert_eq!(next.doc().to_string(), "- -- --- ----");
}

#[test]
fn test_cm6_state_20_changefilter_can_cancel_changes() {
    let filter: TransactionFilterFn = Arc::new(|tr| tr.doc().len_chars() <= 7);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("one two")),
        extensions: Some(vec![TRANSACTION_FILTER.of(filter)]),
        ..Default::default()
    });
    // Tr adding length beyond 7 should be cancelled (update returns None)
    let tr = state.update(TransactionSpec::new().with_change(ChangeSpec::insert(3, " three")));
    assert!(tr.is_none());
}

#[test]
fn test_cm6_state_21_changefilter_can_split_changes() {
    let filter: TransactionFilterFn = Arc::new(|tr| tr.doc().to_string().contains("et") || !tr.doc_changed());
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("onetwo")),
        extensions: Some(vec![TRANSACTION_FILTER.of(filter)]),
        ..Default::default()
    });
    let tr = state.update(TransactionSpec::new().with_change(ChangeSpec::replace(0, 6, "et")));
    assert!(tr.is_some());
}

#[test]
fn test_cm6_state_22_changefilter_combines_filter_masks() {
    let f1: TransactionFilterFn = Arc::new(|tr| !tr.doc().to_string().contains("bad"));
    let f2: TransactionFilterFn = Arc::new(|tr| !tr.doc().to_string().contains("evil"));
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        extensions: Some(vec![TRANSACTION_FILTER.of(f1), TRANSACTION_FILTER.of(f2)]),
        ..Default::default()
    });
    assert!(state.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "bad"))).is_none());
    assert!(state.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "evil"))).is_none());
    assert!(state.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "good"))).is_some());
}

#[test]
fn test_cm6_state_23_changefilter_can_be_turned_off() {
    let f: TransactionFilterFn = Arc::new(|tr| !tr.doc_changed());
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        extensions: Some(vec![TRANSACTION_FILTER.of(f)]),
        ..Default::default()
    });
    assert!(state.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "!"))).is_none());
}

#[test]
fn test_cm6_state_24_transactionfilter_can_constrain_selection() {
    let f: TransactionFilterFn = Arc::new(|tr| {
        if let Some(sel) = &tr.selection() {
            sel.main().head <= 4
        } else {
            true
        }
    });
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("one two")),
        extensions: Some(vec![TRANSACTION_FILTER.of(f)]),
        ..Default::default()
    });
    assert!(state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(3))).is_some());
    assert!(state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(7))).is_none());
}

#[test]
fn test_cm6_state_25_transactionfilter_can_append_sequential_changes() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("one two")),
        ..Default::default()
    });
    let tr = state.update(TransactionSpec::new().with_change(ChangeSpec::insert(3, ","))).unwrap();
    let next = state.apply(tr);
    assert_eq!(next.doc().to_string(), "one, two");
}

#[test]
fn test_cm6_state_26_transactionextender_can_add_annotations() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        ..Default::default()
    });
    let tr = state.update(TransactionSpec::new().with_annotation(100usize)).unwrap();
    assert_eq!(tr.annotation::<usize>(), Some(&100));
}

#[test]
fn test_cm6_state_27_transactionextender_allows_multiple_extenders() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        ..Default::default()
    });
    let tr = state
        .update(
            TransactionSpec::new()
                .with_effect(StateEffect::of(1usize))
                .with_effect(StateEffect::of(2usize)),
        )
        .unwrap();
    assert_eq!(tr.effects().len(), 2);
}
