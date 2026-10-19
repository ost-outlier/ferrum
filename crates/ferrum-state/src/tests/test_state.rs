use crate::change::{ChangeSet, ChangeSpec};
use crate::facet::{Compartment, Facet, StateField};
use crate::selection::{EditorSelection, SelectionRange};
use crate::state::{EditorState, EditorStateConfig, RangeResult};
use crate::text::Text;
use crate::transaction::TransactionSpec;

#[test]
fn test_cm6_state_holds_doc_and_selection_properties() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello")),
        ..Default::default()
    });
    assert_eq!(state.doc().to_string(), "hello");
    assert_eq!(state.selection().main().from(), 0);
}

#[test]
fn test_cm6_state_can_apply_changes() {
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
fn test_cm6_state_maps_selection_through_changes() {
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
        ..Default::default()
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
fn test_cm6_state_stores_and_updates_fields() {
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
fn test_cm6_state_can_replace_extension_groups_compartment() {
    let comp = Compartment::new();
    let f = Facet::<usize>::define_list();
    let init = comp.of(f.of(10));

    let state = EditorState::create(EditorStateConfig {
        extensions: Some(vec![init, f.of(20)]),
        ..Default::default()
    });
    assert_eq!(state.facet(&f), &vec![10, 20]);

    // Reconfigure compartment with [f.of(1), f.of(2)]
    let reconf = comp.reconfigure(vec![f.of(1), f.of(2)].into());
    let tr = state
        .update(TransactionSpec::default().with_effect(reconf))
        .unwrap();
    let state2 = state.apply(tr);
    assert_eq!(state2.facet(&f), &vec![1, 2, 20]);

    // Reconfigure compartment with f.of(3)
    let reconf2 = comp.reconfigure(vec![f.of(3)].into());
    let tr2 = state2
        .update(TransactionSpec::default().with_effect(reconf2))
        .unwrap();
    let state3 = state2.apply(tr2);
    assert_eq!(state3.facet(&f), &vec![3, 20]);
}

#[test]
fn test_cm6_state_change_by_range() {
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
        ..Default::default()
    });

    let result = state.change_by_range(|r| {
        let count = (r.from() >> 1) + 1;
        let dashes = "-".repeat(count);
        let new_range = SelectionRange::new(r.from(), r.from() + 1 + (r.from() >> 1));
        RangeResult::range(new_range).with_changes(ChangeSpec::replace(r.from(), r.to(), dashes))
    });

    let tr = state.update(result.into_spec()).unwrap();
    let state2 = state.apply(tr);
    assert_eq!(state2.doc().to_string(), "- -- --- ----");
    let ranges_str = state2
        .selection()
        .ranges()
        .iter()
        .map(|r| format!("{}-{}", r.from(), r.to()))
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(ranges_str, "0-1 2-4 5-8 9-13");
}
