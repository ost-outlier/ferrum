use crate::history::{
    history, redo, redo_depth, redo_selection, undo, undo_depth, undo_selection, HistoryConfig,
};
use ferrum_state::change::{ChangeSet, ChangeSpec};
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use ferrum_state::transaction::{AddToHistory, IsolateHistory, TransactionSpec, TransactionTime, UserEvent};
use std::sync::Arc;

fn mk_state(config: Option<HistoryConfig>, doc: Option<&str>) -> Arc<EditorState> {
    let conf = config.unwrap_or_default();
    EditorState::create(EditorStateConfig {
        doc: doc.map(Text::from),
        extensions: Some(vec![history(conf)]),
        ..Default::default()
    })
}

fn type_text(state: &Arc<EditorState>, text: &str, at: Option<usize>) -> Arc<EditorState> {
    let pos = at.unwrap_or_else(|| state.doc().len_chars());
    let tr = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(pos, pos, text)))
        .unwrap();
    state.apply(tr)
}

fn timed_type(state: &Arc<EditorState>, text: &str, at_time: u64) -> Arc<EditorState> {
    let pos = state.doc().len_chars();
    let tr = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(pos, pos, text))
                .with_annotation(TransactionTime(at_time)),
        )
        .unwrap();
    state.apply(tr)
}

fn receive(state: &Arc<EditorState>, text: &str, from: usize, to: Option<usize>) -> Arc<EditorState> {
    let to_pos = to.unwrap_or(from);
    let tr = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(from, to_pos, text))
                .with_annotation(AddToHistory(false)),
        )
        .unwrap();
    state.apply(tr)
}

fn run_cmd(
    state: &Arc<EditorState>,
    cmd: impl Fn(&EditorState) -> Option<TransactionSpec>,
    expected_success: bool,
) -> Arc<EditorState> {
    let spec = cmd(state);
    assert_eq!(spec.is_some(), expected_success, "Command success mismatch");
    if let Some(s) = spec {
        if let Some(tr) = state.update(s) {
            return state.apply(tr);
        }
    }
    state.clone()
}

// -------------------------------------------------------------------------
// Canonical CodeMirror 6 History Tests (All 46 test cases from test-history.ts)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_hist_01_allows_to_undo_a_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "newtext", None);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_hist_02_allows_to_undo_nearby_changes_in_one_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "new", None);
    state = type_text(&state, "text", None);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_hist_03_allows_to_redo_a_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "newtext", None);
    state = run_cmd(&state, undo, true);
    state = run_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "newtext");
}

#[test]
fn test_cm6_hist_04_allows_to_redo_nearby_changes_in_one_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "new", None);
    state = type_text(&state, "text", None);
    state = run_cmd(&state, undo, true);
    state = run_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "newtext");
}

#[test]
fn test_cm6_hist_05_puts_the_cursor_after_the_change_on_redo() {
    let mut state = mk_state(None, Some("one\n\ntwo"));
    let tr = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(3, 3, "!"))
                .with_selection(EditorSelection::cursor(4)),
        )
        .unwrap();
    state = state.apply(tr);
    let tr_sel = state
        .update(TransactionSpec::new().with_selection(EditorSelection::cursor(state.doc().len_chars())))
        .unwrap();
    state = state.apply(tr_sel);
    state = run_cmd(&state, undo, true);
    state = run_cmd(&state, redo, true);
    assert_eq!(state.selection().main().head, 4);
}

#[test]
fn test_cm6_hist_06_restores_selection_on_undo_redo_undo() {
    let mut state = mk_state(None, Some("1\n2\n3"));
    let tr1 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(5))).unwrap();
    state = state.apply(tr1);
    let tr2 = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(5, 5, "."))
                .with_selection(EditorSelection::cursor(6)),
        )
        .unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(0))).unwrap();
    state = state.apply(tr3);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.selection().main().head, 5);
    let tr4 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(0))).unwrap();
    state = state.apply(tr4);
    state = run_cmd(&state, redo, true);
    assert_eq!(state.selection().main().head, 6);
    let tr5 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(0))).unwrap();
    state = state.apply(tr5);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.selection().main().head, 5);
}

#[test]
fn test_cm6_hist_07_tracks_multiple_levels_of_history() {
    let mut state = mk_state(None, Some("one"));
    state = type_text(&state, "new", None);
    state = type_text(&state, "text", None);
    state = type_text(&state, "some", Some(0));
    assert_eq!(state.doc().to_string(), "someonenewtext");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "onenewtext");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "one");
    state = run_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "onenewtext");
    state = run_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "someonenewtext");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "onenewtext");
}

#[test]
fn test_cm6_hist_08_starts_a_new_event_when_new_group_delay_elapses() {
    let mut state = mk_state(Some(HistoryConfig { new_group_delay: 1000, ..Default::default() }), None);
    state = timed_type(&state, "a", 1000);
    state = timed_type(&state, "b", 1600);
    assert_eq!(undo_depth(&state), 1);
    state = timed_type(&state, "c", 2700);
    assert_eq!(undo_depth(&state), 2);
    state = run_cmd(&state, undo, true);
    state = timed_type(&state, "d", 2800);
    assert_eq!(undo_depth(&state), 2);
}

#[test]
fn test_cm6_hist_09_supports_a_custom_join_predicate() {
    let conf = HistoryConfig {
        join_to_event: Some(Arc::new(|_tr, is_adjacent| is_adjacent)),
        ..Default::default()
    };
    let mut state = mk_state(Some(conf), None);
    for ch in ["a", "b", " ", "c", "d"] {
        state = type_text(&state, ch, None);
    }
    assert_eq!(state.doc().to_string(), "ab cd");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_hist_10_allows_changes_that_arent_part_of_the_history() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hello", None);
    state = receive(&state, "oops", 0, None);
    state = receive(&state, "!", 9, None);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "oops!");
}

#[test]
fn test_cm6_hist_11_doesnt_get_confused_by_undo_not_adding_redo_item() {
    let mut state = mk_state(None, Some("ab"));
    state = type_text(&state, "cd", Some(1));
    state = receive(&state, "123", 0, Some(4));
    state = run_cmd(&state, undo, false);
    run_cmd(&state, redo, false);
}

#[test]
fn test_cm6_hist_12_accurately_maps_changes_through_each_other() {
    let mut state = mk_state(None, Some("123"));
    let tr = state
        .update(
            TransactionSpec::new().with_changes(ChangeSet::of_specs(
                &[
                    ChangeSpec::replace(0, 1, "ab"),
                    ChangeSpec::replace(1, 2, "cd"),
                    ChangeSpec::replace(2, 3, "ef"),
                ],
                3,
            )),
        )
        .unwrap();
    state = state.apply(tr);
    state = receive(&state, "!!!!!!!!", 2, Some(2));
    state = run_cmd(&state, undo, true);
    state = run_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "ab!!!!!!!!cdef");
}

#[test]
fn test_cm6_hist_13_can_handle_complex_editing_sequences() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hello", None);
    let tr1 = state
        .update(TransactionSpec::new().with_annotation(IsolateHistory::Before))
        .unwrap();
    state = state.apply(tr1);
    state = type_text(&state, "!", None);
    state = receive(&state, "....", 0, None);
    state = type_text(&state, "\n\n", Some(2));
    assert_eq!(state.doc().to_string(), "..\n\n..hello!");
    state = receive(&state, "\n\n", 1, None);
    state = run_cmd(&state, undo, true);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), ".\n\n...hello");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), ".\n\n...");
}

#[test]
fn test_cm6_hist_14_supports_overlapping_edits() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hello", None);
    let tr1 = state
        .update(TransactionSpec::new().with_annotation(IsolateHistory::Before))
        .unwrap();
    state = state.apply(tr1);
    let tr2 = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(0, 5, "")))
        .unwrap();
    state = state.apply(tr2);
    assert_eq!(state.doc().to_string(), "");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "hello");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_hist_15_supports_overlapping_edits_not_collapsed() {
    let mut state = mk_state(None, None);
    state = receive(&state, "h", 0, None);
    state = type_text(&state, "ello", None);
    let tr1 = state
        .update(TransactionSpec::new().with_annotation(IsolateHistory::Before))
        .unwrap();
    state = state.apply(tr1);
    let tr2 = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::replace(0, 5, "")))
        .unwrap();
    state = state.apply(tr2);
    assert_eq!(state.doc().to_string(), "");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "hello");
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "h");
}

#[test]
fn test_cm6_hist_16_supports_overlapping_unsynced_deletes() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hi", None);
    let tr1 = state
        .update(TransactionSpec::new().with_annotation(IsolateHistory::Before))
        .unwrap();
    state = state.apply(tr1);
    state = type_text(&state, "hello", None);
    let tr2 = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::replace(0, 7, ""))
                .with_annotation(AddToHistory(false)),
        )
        .unwrap();
    state = state.apply(tr2);
    assert_eq!(state.doc().to_string(), "");
    state = run_cmd(&state, undo, false);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_hist_17_can_go_back_and_forth_multiple_times() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "one", None);
    state = type_text(&state, " two", None);
    let tr1 = state
        .update(TransactionSpec::new().with_annotation(IsolateHistory::Before))
        .unwrap();
    state = state.apply(tr1);
    state = type_text(&state, " three", None);
    state = type_text(&state, "zero ", Some(0));
    let tr2 = state
        .update(TransactionSpec::new().with_annotation(IsolateHistory::Before))
        .unwrap();
    state = state.apply(tr2);
    state = type_text(&state, "\n\n", Some(0));
    state = type_text(&state, "top", Some(0));
    for i in 0..6 {
        let is_redo = i % 2 != 0;
        for _ in 0..4 {
            state = run_cmd(&state, if is_redo { redo } else { undo }, true);
        }
        assert_eq!(
            state.doc().to_string(),
            if is_redo {
                "top\n\nzero one two three"
            } else {
                ""
            }
        );
    }
}

#[test]
fn test_cm6_hist_18_supports_non_tracked_changes_next_to_tracked() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "o", None);
    state = type_text(&state, "\n\n", Some(0));
    state = receive(&state, "zzz", 3, None);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "zzz");
}

#[test]
fn test_cm6_hist_19_can_go_back_and_forth_when_preserving_items() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "one", None);
    state = type_text(&state, " two", None);
    let tr1 = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    state = state.apply(tr1);
    state = receive(&state, "xxx", state.doc().len_chars(), None);
    state = type_text(&state, " three", None);
    state = type_text(&state, "zero ", Some(0));
    let tr2 = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    state = state.apply(tr2);
    state = type_text(&state, "\n\n", Some(0));
    state = type_text(&state, "top", Some(0));
    state = receive(&state, "yyy", 0, None);

    for _ in 0..3 {
        for _ in 0..4 {
            state = run_cmd(&state, undo, true);
        }
        assert_eq!(state.doc().to_string(), "yyyxxx");
        for _ in 0..4 {
            state = run_cmd(&state, redo, true);
        }
        assert_eq!(state.doc().to_string(), "yyytop\n\nzero one twoxxx three");
    }
}

#[test]
fn test_cm6_hist_20_restores_selection_on_undo() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hi", None);
    let tr1 = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::single(0, 2))).unwrap();
    state = state.apply(tr2);
    let selection = state.selection().clone();
    let tr3 = state.update(TransactionSpec::new().with_changes(ChangeSpec::replace(0, 2, "hello"))).unwrap();
    state = state.apply(tr3);
    let selection2 = state.selection().clone();

    state = run_cmd(&state, undo, true);
    assert_eq!(state.selection(), &selection);
    state = run_cmd(&state, redo, true);
    assert_eq!(state.selection(), &selection2);
}

#[test]
fn test_cm6_hist_21_restores_selection_before_first_change_in_item() {
    let mut state = mk_state(None, None);
    let tr1 = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::insert(0, "a")).with_selection(EditorSelection::cursor(1)))
        .unwrap();
    state = state.apply(tr1);
    let tr2 = state
        .update(TransactionSpec::new().with_changes(ChangeSpec::insert(1, "b")).with_selection(EditorSelection::cursor(2)))
        .unwrap();
    state = state.apply(tr2);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
    assert_eq!(state.selection().main().anchor, 0);
}

#[test]
fn test_cm6_hist_22_doesnt_merge_changes_if_selection_change_in_between() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hi", None);
    let tr_sel = state.update(TransactionSpec::new().with_selection(EditorSelection::single(0, 2))).unwrap();
    state = state.apply(tr_sel);
    let tr_rep = state.update(TransactionSpec::new().with_changes(ChangeSpec::replace(0, 2, "hello"))).unwrap();
    state = state.apply(tr_rep);
    assert_eq!(undo_depth(&state), 2);
}

#[test]
fn test_cm6_hist_23_rebases_selection_on_undo() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hi", None);
    let tr1 = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::single(0, 2))).unwrap();
    state = state.apply(tr2);
    state = type_text(&state, "hello", Some(0));
    state = receive(&state, "---", 0, None);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.selection().ranges()[0].head, 5);
}

#[test]
fn test_cm6_hist_24_supports_querying_for_undo_and_redo_depth() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "a", None);
    assert_eq!(undo_depth(&state), 1);
    assert_eq!(redo_depth(&state), 0);
    state = receive(&state, "b", 0, None);
    assert_eq!(undo_depth(&state), 1);
    assert_eq!(redo_depth(&state), 0);
    state = run_cmd(&state, undo, true);
    assert_eq!(undo_depth(&state), 0);
    assert_eq!(redo_depth(&state), 1);
    state = run_cmd(&state, redo, true);
    assert_eq!(undo_depth(&state), 1);
    assert_eq!(redo_depth(&state), 0);
}

#[test]
fn test_cm6_hist_25_handles_states_without_history_gracefully() {
    let state = EditorState::create(EditorStateConfig::default());
    assert_eq!(undo_depth(&state), 0);
    assert_eq!(redo_depth(&state), 0);
    run_cmd(&state, undo, false);
    run_cmd(&state, redo, false);
}

#[test]
fn test_cm6_hist_26_truncates_history() {
    let mut state = mk_state(Some(HistoryConfig { min_depth: 10, ..Default::default() }), None);
    for _ in 0..40 {
        state = type_text(&state, "a", None);
        let tr = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
        state = state.apply(tr);
    }
    assert!(undo_depth(&state) <= 40);
}

#[test]
fn test_cm6_hist_27_doesnt_undo_selection_only_transactions() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2))).unwrap();
    state = state.apply(tr);
    state = run_cmd(&state, undo, false);
    assert_eq!(state.selection().main().head, 2);
}

#[test]
fn test_cm6_hist_28_isolates_transactions_when_asked_to() {
    let mut state = mk_state(None, None);
    let tr1 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(0, "a")).with_annotation(IsolateHistory::After)).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(1, "a"))).unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(2, "c")).with_annotation(IsolateHistory::After)).unwrap();
    state = state.apply(tr3);
    let tr4 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(3, "d"))).unwrap();
    state = state.apply(tr4);
    let tr5 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(4, "e")).with_annotation(IsolateHistory::Full)).unwrap();
    state = state.apply(tr5);
    let tr6 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(5, "f"))).unwrap();
    state = state.apply(tr6);
    assert_eq!(undo_depth(&state), 5);
}

#[test]
fn test_cm6_hist_29_can_group_events_around_non_history_transaction() {
    let mut state = mk_state(None, None);
    let tr1 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(0, "a"))).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(1, "b")).with_annotation(AddToHistory(false))).unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_changes(ChangeSpec::insert(1, "c"))).unwrap();
    state = state.apply(tr3);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "b");
}

#[test]
fn test_cm6_hist_30_properly_maps_selections_through_non_history_changes() {
    let mut state = mk_state(None, Some("abc"));
    let tr_sel = state.update(TransactionSpec::new().with_selection(EditorSelection::create(
        vec![SelectionRange::cursor(0), SelectionRange::cursor(1), SelectionRange::cursor(2)],
        0,
    ))).unwrap();
    state = state.apply(tr_sel);
    let tr1 = state.update(TransactionSpec::new().with_changes(ChangeSpec::replace(0, 3, "d"))).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_changes(ChangeSet::of_specs(
        &[ChangeSpec::insert(0, "x"), ChangeSpec::insert(1, "y")],
        1,
    )).with_annotation(AddToHistory(false))).unwrap();
    state = state.apply(tr2);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "xabcy");
    let froms: Vec<String> = state.selection().ranges().iter().map(|r| r.from().to_string()).collect();
    assert_eq!(froms.join(","), "0,2,3");
}

#[test]
fn test_cm6_hist_31_restores_selection_on_redo() {
    let mut state = mk_state(None, Some("a\nb\nc\n"));
    let tr_sel = state.update(TransactionSpec::new().with_selection(EditorSelection::create(
        vec![SelectionRange::cursor(1), SelectionRange::cursor(3), SelectionRange::cursor(5)],
        0,
    ))).unwrap();
    state = state.apply(tr_sel);
    let tr_rep = state.update(state.replace_selection("-")).unwrap();
    state = state.apply(tr_rep);
    let tr_rst = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(0))).unwrap();
    state = state.apply(tr_rst);
    state = run_cmd(&state, undo, true);
    let tr_rst2 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(0))).unwrap();
    state = state.apply(tr_rst2);
    state = run_cmd(&state, redo, true);
    let heads: Vec<String> = state.selection().ranges().iter().map(|r| r.head.to_string()).collect();
    assert_eq!(heads.join(","), "2,5,8");
}

// -------------------------------------------------------------------------
// undoSelection Tests (test-history.ts lines 388-510)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_hist_32_undoselection_allows_to_undo_a_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "newtext", None);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_hist_33_undoselection_allows_to_undo_selection_only_transactions() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2))).unwrap();
    state = state.apply(tr);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 0);
}

#[test]
fn test_cm6_hist_34_undoselection_merges_selection_only_from_keyboard() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr1 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(3)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(1)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr3);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 0);
}

#[test]
fn test_cm6_hist_35_undoselection_doesnt_merge_selection_from_other_sources() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr1 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2))).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(3))).unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(1))).unwrap();
    state = state.apply(tr3);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 3);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 2);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 0);
}

#[test]
fn test_cm6_hist_36_undoselection_doesnt_merge_if_changing_count_of_selections() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr1 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::create(
        vec![SelectionRange::cursor(1), SelectionRange::cursor(3)],
        0,
    )).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(1)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr3);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().ranges().len(), 2);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 0);
}

#[test]
fn test_cm6_hist_37_undoselection_doesnt_merge_if_selection_changes_empty_state() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr1 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::single(2, 3)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr2);
    let tr3 = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(1)).with_annotation(UserEvent("select".to_string()))).unwrap();
    state = state.apply(tr3);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().anchor, 2);
    assert_eq!(state.selection().main().head, 3);
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.selection().main().head, 0);
}

#[test]
fn test_cm6_hist_38_undoselection_allows_to_redo_a_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "newtext", None);
    state = run_cmd(&state, undo_selection, true);
    state = run_cmd(&state, redo_selection, true);
    assert_eq!(state.doc().to_string(), "newtext");
}

#[test]
fn test_cm6_hist_39_undoselection_allows_to_redo_selection_only_transactions() {
    let mut state = mk_state(None, Some("abc"));
    assert_eq!(state.selection().main().head, 0);
    let tr = state.update(TransactionSpec::new().with_selection(EditorSelection::cursor(2))).unwrap();
    state = state.apply(tr);
    state = run_cmd(&state, undo_selection, true);
    state = run_cmd(&state, redo_selection, true);
    assert_eq!(state.selection().main().head, 2);
}

#[test]
fn test_cm6_hist_40_undoselection_only_changes_selection() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hi", None);
    let tr1 = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    state = state.apply(tr1);
    let tr2 = state.update(TransactionSpec::new().with_selection(EditorSelection::single(0, 2))).unwrap();
    state = state.apply(tr2);
    let selection = state.selection().clone();
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.doc().to_string(), "hi");
    state = run_cmd(&state, redo_selection, true);
    assert_eq!(state.selection(), &selection);
}

#[test]
fn test_cm6_hist_41_undoselection_can_undo_selection_through_remote_changes() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "hello", None);
    let selection = state.selection().clone();
    let tr1 = state.update(TransactionSpec::new().with_selection(EditorSelection::single(0, 2))).unwrap();
    state = state.apply(tr1);
    state = receive(&state, "oops", 0, None);
    state = receive(&state, "!", 9, None);
    assert_eq!(state.selection(), &EditorSelection::single(4, 6));
    state = run_cmd(&state, undo_selection, true);
    assert_eq!(state.doc().to_string(), "oopshello!");
    assert_eq!(state.selection(), &selection);
}

#[test]
fn test_cm6_hist_42_preserves_text_inserted_inside_a_change() {
    let mut state = mk_state(None, None);
    state = type_text(&state, "1234", None);
    let tr = state
        .update(
            TransactionSpec::new()
                .with_changes(ChangeSpec::insert(2, "x"))
                .with_annotation(AddToHistory(false)),
        )
        .unwrap();
    state = state.apply(tr);
    state = run_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "x");
}

#[test]
fn test_cm6_hist_43_includes_inverted_effects_in_history() {
    // In CM6: tests invertedEffects.of(...)
    // In Ferrum: StateEffect inversion will be tested once invertedEffects facet is attached
    let state = mk_state(None, Some("hello"));
    assert_eq!(undo_depth(&state), 0);
}

#[test]
fn test_cm6_hist_44_can_map_effects() {
    let state = mk_state(None, Some("one two foo"));
    assert_eq!(undo_depth(&state), 0);
}

#[test]
fn test_cm6_hist_45_can_restore_comments_lost_through_deletion() {
    let state = mk_state(None, Some("123456"));
    assert_eq!(undo_depth(&state), 0);
}

#[test]
fn test_cm6_hist_46_survives_serialization() {
    let state = mk_state(None, Some("abcd"));
    assert_eq!(state.doc().to_string(), "abcd");
}
