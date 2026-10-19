use crate::history::{
    history, redo, redo_depth, undo, undo_depth, undo_selection, HistoryConfig,
};
use ferrum_state::change::ChangeSpec;
use ferrum_state::selection::EditorSelection;
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use ferrum_state::transaction::{IsolateHistory, TransactionSpec, TransactionTime};
use std::sync::Arc;

fn mk_history_state(doc: &str, config: Option<HistoryConfig>) -> Arc<EditorState> {
    let conf = config.unwrap_or_default();
    EditorState::create(EditorStateConfig {
        doc: Some(Text::from(doc)),
        extensions: Some(vec![history(conf)]),
        ..Default::default()
    })
}

#[test]
fn test_cm6_history_allows_undo_and_redo() {
    let state = mk_history_state("hello", None);
    let tr = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(5, 5, " world"))
                .with_selection(EditorSelection::cursor(11)),
        )
        .unwrap();
    let state2 = state.apply(tr);
    assert_eq!(state2.doc().to_string(), "hello world");

    let undo_spec = undo(&state2).expect("undo spec exists");
    let state3 = state2.apply(state2.update(undo_spec).unwrap());
    assert_eq!(state3.doc().to_string(), "hello");

    let redo_spec = redo(&state3).expect("redo spec exists");
    let state4 = state3.apply(state3.update(redo_spec).unwrap());
    assert_eq!(state4.doc().to_string(), "hello world");
}

#[test]
fn test_cm6_history_undoes_nearby_changes_in_one_step() {
    let state = mk_history_state("hello", None);
    let tr1 = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(5, 5, " ")))
        .unwrap();
    let state2 = state.apply(tr1);

    let tr2 = state2
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(6, 6, "world")))
        .unwrap();
    let state3 = state2.apply(tr2);
    assert_eq!(state3.doc().to_string(), "hello world");

    let undo_spec = undo(&state3).expect("undo spec");
    let state4 = state3.apply(state3.update(undo_spec).unwrap());
    assert_eq!(state4.doc().to_string(), "hello");
}

#[test]
fn test_cm6_history_tracks_multiple_levels() {
    let state = mk_history_state("0", None);
    let mut curr = state;
    for i in 1..=4 {
        let text = i.to_string();
        let tr = curr
            .update(
                TransactionSpec::new()
                    .with_changes(ChangeSpec::replace(curr.doc().len_chars(), curr.doc().len_chars(), text.as_str()))
                    .with_annotation(TransactionTime(i as u64 * 1000)),
            )
            .unwrap();
        curr = curr.apply(tr);
    }
    assert_eq!(curr.doc().to_string(), "01234");

    for expected in ["0123", "012", "01", "0"] {
        let undo_spec = undo(&curr).expect("undo step");
        curr = curr.apply(curr.update(undo_spec).unwrap());
        assert_eq!(curr.doc().to_string(), expected);
    }
}

#[test]
fn test_cm6_history_new_group_delay() {
    let state = mk_history_state("a", None);
    let tr1 = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(1, 1, "b"))
                .with_annotation(TransactionTime(1000)),
        )
        .unwrap();
    let state2 = state.apply(tr1);

    let tr2 = state2
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(2, 2, "c"))
                .with_annotation(TransactionTime(2000)), // > 500ms delay
        )
        .unwrap();
    let state3 = state2.apply(tr2);
    assert_eq!(state3.doc().to_string(), "abc");

    let undo_spec = undo(&state3).expect("undo step 1");
    let state4 = state3.apply(state3.update(undo_spec).unwrap());
    assert_eq!(state4.doc().to_string(), "ab");
}

#[test]
fn test_cm6_history_isolate_history() {
    let state = mk_history_state("a", None);
    let tr1 = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(1, 1, "b")))
        .unwrap();
    let state2 = state.apply(tr1);

    let tr2 = state2
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(2, 2, "c"))
                .with_annotation(IsolateHistory::Before),
        )
        .unwrap();
    let state3 = state2.apply(tr2);
    assert_eq!(state3.doc().to_string(), "abc");

    let undo_spec = undo(&state3).expect("undo");
    let state4 = state3.apply(state3.update(undo_spec).unwrap());
    assert_eq!(state4.doc().to_string(), "ab");
}

#[test]
fn test_cm6_history_supports_querying_for_undo_and_redo_depth() {
    let state = mk_history_state("hello", None);
    assert_eq!(undo_depth(&state), 0);
    assert_eq!(redo_depth(&state), 0);

    let tr = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(5, 5, " world")))
        .unwrap();
    let state2 = state.apply(tr);
    assert_eq!(undo_depth(&state2), 1);
    assert_eq!(redo_depth(&state2), 0);

    let undo_spec = undo(&state2).unwrap();
    let state3 = state2.apply(state2.update(undo_spec).unwrap());
    assert_eq!(undo_depth(&state3), 0);
    assert_eq!(redo_depth(&state3), 1);
}

#[test]
fn test_cm6_history_undo_selection_tracks_selection_only_transactions() {
    let state = mk_history_state("hello world", None);
    let tr = state
        .update(TransactionSpec::new().with_selection(EditorSelection::single(0, 5)))
        .unwrap();
    let state2 = state.apply(tr);
    assert_eq!(state2.selection().main().to(), 5);

    let undo_spec = undo_selection(&state2);
    assert!(undo_spec.is_some(), "undo_selection should be available");
}
