use super::*;
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;

#[test]
fn test_grapheme_aware_backspace_and_delete() {
    // 1. ZWJ emoji: 👨‍👩‍👧‍👦 (7 codepoints)
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("Start 👨‍👩‍👧‍👦 End")),
        selection: Some(EditorSelection::cursor(6 + "👨‍👩‍👧‍👦".chars().count())),
        extensions: None,
    });

    // Backspace directly behind the emoji must delete the ENTIRE 7-codepoint emoji at once!
    let spec = delete_char_backward(&state).expect("should produce backspace edit");
    let tr = state.update(spec).unwrap();
    let next_state = state.apply(tr);

    assert_eq!(next_state.doc().to_string(), "Start  End");
    assert_eq!(next_state.selection().main().head, 6);

    // 2. Combining accent: e + \u{0301}
    let state_accent = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("e\u{0301}cole")),
        selection: Some(EditorSelection::cursor(2)), // after combining mark
        extensions: None,
    });
    let spec_accent = delete_char_backward(&state_accent).unwrap();
    let tr_accent = state_accent.update(spec_accent).unwrap();
    let next_accent = state_accent.apply(tr_accent);
    assert_eq!(next_accent.doc().to_string(), "cole");
    assert_eq!(next_accent.selection().main().head, 0);

    // 3. Delete forward on CRLF sequence
    let state_crlf = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("Line1\r\nLine2")),
        selection: Some(EditorSelection::cursor(5)), // right before \r\n
        extensions: None,
    });
    let spec_crlf = delete_char_forward(&state_crlf).unwrap();
    let tr_crlf = state_crlf.update(spec_crlf).unwrap();
    let next_crlf = state_crlf.apply(tr_crlf);
    assert_eq!(next_crlf.doc().to_string(), "Line1Line2");
}

#[test]
fn test_multi_cursor_typing_and_backspace() {
    let doc = Text::from("apple\nbanana\ncherry");
    // Place cursors at the end of each word
    let sel = EditorSelection::create(
        vec![
            SelectionRange::cursor(5),  // after apple
            SelectionRange::cursor(12), // after banana
            SelectionRange::cursor(19), // after cherry
        ],
        0,
    );

    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: Some(sel),
        extensions: None,
    });

    // 1. Simultaneous insert "! " at all cursors
    let spec = insert_text(&state, "! ").unwrap();
    let tr = state.update(spec).unwrap();
    let state = state.apply(tr);

    assert_eq!(state.doc().to_string(), "apple! \nbanana! \ncherry! ");
    assert_eq!(state.selection().ranges().len(), 3);

    // 2. Simultaneous Backspace at all cursors
    let spec = delete_char_backward(&state).unwrap();
    let tr = state.update(spec).unwrap();
    let state = state.apply(tr);

    assert_eq!(state.doc().to_string(), "apple!\nbanana!\ncherry!");
}

#[test]
fn test_word_and_subword_navigation() {
    let text = "let currentEditorView = 42;";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(text)),
        selection: Some(EditorSelection::cursor(4)), // at 'c' of currentEditorView
        extensions: None,
    });

    // Subword forward navigation through camelCase:
    // "current" -> "Editor" -> "View"
    let spec1 = cursor_subword_forward(&state).unwrap();
    let state1 = state.apply(state.update(spec1).unwrap());
    assert_eq!(state1.selection().main().head, 11); // after "current", at "Editor"

    let spec2 = cursor_subword_forward(&state1).unwrap();
    let state2 = state1.apply(state1.update(spec2).unwrap());
    assert_eq!(state2.selection().main().head, 17); // after "Editor", at "View"

    let spec3 = cursor_subword_forward(&state2).unwrap();
    let state3 = state2.apply(state2.update(spec3).unwrap());
    assert_eq!(state3.selection().main().head, 21); // after "View"

    // Subword backward navigation
    let spec_back = cursor_subword_backward(&state3).unwrap();
    let state_back = state3.apply(state3.update(spec_back).unwrap());
    assert_eq!(state_back.selection().main().head, 17); // back to "View"
}

#[test]
fn test_vertical_navigation_with_goal_column() {
    let text = "short\nthis is a longer line\nshort";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(text)),
        selection: Some(EditorSelection::cursor(16)), // "this is a l|onger line" (col 10)
        extensions: None,
    });

    // Move cursor UP to "short" line (which is only 5 chars long)
    let spec_up = cursor_line_up(&state).unwrap();
    let state_up = state.apply(state.update(spec_up).unwrap());
    // Cursor is clamped to end of line (5), but retains goal_column = 10!
    assert_eq!(state_up.selection().main().head, 5);
    assert_eq!(state_up.selection().main().goal_column, Some(10));

    // Move cursor DOWN back to line 2: goal column 10 is restored!
    let spec_down = cursor_line_down(&state_up).unwrap();
    let state_down = state_up.apply(state_up.update(spec_down).unwrap());
    assert_eq!(state_down.selection().main().head, 16);
}

#[test]
fn test_line_operations_and_indentation() {
    let text = "line 1\nline 2\nline 3";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(text)),
        selection: Some(EditorSelection::cursor(8)), // in "line 2"
        extensions: None,
    });

    // Move line down (Alt+Down)
    let spec_move = move_line_down(&state).unwrap();
    let state_moved = state.apply(state.update(spec_move).unwrap());
    assert_eq!(state_moved.doc().to_string(), "line 1\nline 3\nline 2");

    // Move line up back (Alt+Up)
    let spec_up = move_line_up(&state_moved).unwrap();
    let state_restored = state_moved.apply(state_moved.update(spec_up).unwrap());
    assert_eq!(state_restored.doc().to_string(), "line 1\nline 2\nline 3");

    // Indentation: indent_more (4 spaces)
    let spec_indent = indent_more(&state_restored).unwrap();
    let state_indented = state_restored.apply(state_restored.update(spec_indent).unwrap());
    assert_eq!(state_indented.doc().to_string(), "line 1\n    line 2\nline 3");

    // Indentation: indent_less
    let spec_unindent = indent_less(&state_indented).unwrap();
    let state_unindented = state_indented.apply(state_indented.update(spec_unindent).unwrap());
    assert_eq!(state_unindented.doc().to_string(), "line 1\nline 2\nline 3");

    // Enter with automatic indent preservation
    let state_with_indent = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("    fn foo()")),
        selection: Some(EditorSelection::cursor(12)),
        extensions: None,
    });
    let spec_enter = insert_newline_and_indent(&state_with_indent).unwrap();
    let state_entered = state_with_indent.apply(state_with_indent.update(spec_enter).unwrap());
    assert_eq!(state_entered.doc().to_string(), "    fn foo()\n    ");
    assert_eq!(state_entered.selection().main().head, 17);
}

#[test]
fn test_delete_line_command() {
    let text = "Line 1\nLine 2\nLine 3\n";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(text)),
        selection: Some(EditorSelection::cursor(8)), // in "Line 2"
        extensions: None,
    });

    let spec = delete_line(&state).unwrap();
    let state = state.apply(state.update(spec).unwrap());
    assert_eq!(state.doc().to_string(), "Line 1\nLine 3\n");
}

#[test]
fn test_keymap_facet_and_dispatch() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("Hello, Ferrum")),
        selection: Some(EditorSelection::cursor(5)),
        extensions: Some(vec![KEYMAP.of(standard_keymap())]),
    });

    // Dispatch "ArrowRight" key
    let spec_right = dispatch_key(&state, "ArrowRight", false).expect("ArrowRight should match");
    let state_right = state.apply(state.update(spec_right).unwrap());
    assert_eq!(state_right.selection().main().head, 6);

    // Dispatch "Mod-a" (Select All)
    let spec_select_all = dispatch_key(&state_right, "Mod-a", false).expect("Mod-a should match");
    let state_all = state_right.apply(state_right.update(spec_select_all).unwrap());
    assert_eq!(state_all.selection().main().from(), 0);
    assert_eq!(state_all.selection().main().to(), 13);

    // Dispatch "Backspace" with selection active -> deletes selection
    let spec_backspace = dispatch_key(&state_all, "Backspace", false).expect("Backspace should match");
    let state_empty = state_all.apply(state_all.update(spec_backspace).unwrap());
    assert_eq!(state_empty.doc().to_string(), "");
}

#[test]
fn test_simplify_selection_collapses_to_primary() {
    let sel = EditorSelection::create(
        vec![
            SelectionRange::cursor(2),
            SelectionRange::cursor(5),
            SelectionRange::cursor(8),
        ],
        1, // main is index 1 (pos 5)
    );
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("0123456789")),
        selection: Some(sel),
        extensions: None,
    });

    let spec = simplify_selection(&state).unwrap();
    let state_simplified = state.apply(state.update(spec).unwrap());
    assert!(state_simplified.selection().is_single());
    assert_eq!(state_simplified.selection().main().head, 5);
}

#[test]
fn test_delete_group_and_line_boundaries() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("hello world foo bar")),
        selection: Some(EditorSelection::cursor(11)), // after "hello world"
        extensions: None,
    });

    // Delete group backward -> deletes "world"
    let spec_del_group = delete_group_backward(&state).unwrap();
    let state_after_del = state.apply(state.update(spec_del_group).unwrap());
    assert_eq!(state_after_del.doc().to_string(), "hello  foo bar");

    // Delete to line end
    let spec_del_end = delete_to_line_end(&state_after_del).unwrap();
    let state_del_end = state_after_del.apply(state_after_del.update(spec_del_end).unwrap());
    assert_eq!(state_del_end.doc().to_string(), "hello ");

    // Delete to line start
    let spec_del_start = delete_to_line_start(&state_del_end).unwrap();
    let state_del_start = state_del_end.apply(state_del_end.update(spec_del_start).unwrap());
    assert_eq!(state_del_start.doc().to_string(), "");
}

#[test]
fn test_copy_line_commands() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("alpha\nbeta")),
        selection: Some(EditorSelection::cursor(2)), // in "alpha"
        extensions: None,
    });

    // Copy line down (duplicate "alpha" below)
    let spec = copy_line_down(&state).unwrap();
    let next_state = state.apply(state.update(spec).unwrap());
    assert_eq!(next_state.doc().to_string(), "alpha\nalpha\nbeta");
}

fn mk_state(spec: &str) -> std::sync::Arc<EditorState> {
    let mut doc = String::new();
    let mut ranges = Vec::new();
    let chars: Vec<char> = spec.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '|' {
            ranges.push(SelectionRange::cursor(doc.chars().count()));
            i += 1;
        } else if chars[i] == '<' {
            let start = doc.chars().count();
            i += 1;
            while i < chars.len() && chars[i] != '>' {
                doc.push(chars[i]);
                i += 1;
            }
            if i < chars.len() && chars[i] == '>' {
                i += 1;
            }
            let end = doc.chars().count();
            ranges.push(SelectionRange::new(start, end));
        } else {
            doc.push(chars[i]);
            i += 1;
        }
    }
    EditorState::create(EditorStateConfig {
        doc: Some(Text::from(doc)),
        selection: if ranges.is_empty() {
            None
        } else {
            Some(EditorSelection::create(ranges, 0))
        },
        extensions: None,
    })
}

fn state_str(state: &EditorState) -> String {
    let mut doc = state.doc().to_string();
    let ranges = state.selection().ranges();
    for range in ranges.iter().rev() {
        if range.is_empty() {
            let idx = range.from();
            let byte_idx = doc.char_indices().nth(idx).map(|(b, _)| b).unwrap_or(doc.len());
            doc.insert(byte_idx, '|');
        } else {
            let to_idx = range.to();
            let to_byte = doc.char_indices().nth(to_idx).map(|(b, _)| b).unwrap_or(doc.len());
            doc.insert(to_byte, '>');
            let from_idx = range.from();
            let from_byte = doc.char_indices().nth(from_idx).map(|(b, _)| b).unwrap_or(doc.len());
            doc.insert(from_byte, '<');
        }
    }
    doc
}

#[test]
fn test_cm6_indent_more_command() {
    let state = mk_state("on<e\ntwo\n>three");
    let spec = indent_more(&state).unwrap();
    let next_state = state.apply(state.update(spec).unwrap());
    // Direct parity with CM6 test-commands.ts:
    // "ignores lines if a range selection ends directly at their start"
    assert_eq!(state_str(&next_state), "    on<e\n    two\n>three");
}

fn run_cmd(
    state: &std::sync::Arc<EditorState>,
    cmd: impl Fn(&EditorState) -> Option<ferrum_state::transaction::TransactionSpec>,
) -> std::sync::Arc<EditorState> {
    if let Some(spec) = cmd(state) {
        if let Some(tr) = state.update(spec) {
            return state.apply(tr);
        }
    }
    state.clone()
}

#[test]
fn test_cm6_delete_trailing_whitespace() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, delete_trailing_whitespace);
        assert_eq!(next_state.doc().to_string(), expected);
    }

    test("foo   ", "foo");
    test("one\ntwo \nthree   \n   ", "one\ntwo\nthree\n");
    test("one  \n\ntwo ", "one\n\ntwo");
}

#[test]
fn test_cm6_insert_newline_keep_indent() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, insert_newline_keep_indent);
        assert_eq!(state_str(&next_state), expected);
    }

    test("  one|", "  one\n  |");
    test("one|two", "one\n|two");
    test("if x:\n  one<two\n  three>four", "if x:\n  one\n  |four");
}

#[test]
fn test_cm6_delete_group_forward() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, delete_group_forward);
        assert_eq!(state_str(&next_state), expected);
    }

    test("one |two three", "one | three");
    test("one| two three", "one| three");
    test("one|...two", "one|two");
    test("one|  \ttwo", "one|two");
    test("one|\ntwo", "one|two");
    test("one| \n two", "one|\n two");
    test("one|\n two", "one| two");
    test("one|two", "one|");
    test("one|", "one|");
}

#[test]
fn test_cm6_delete_group_backward() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, delete_group_backward);
        assert_eq!(state_str(&next_state), expected);
    }

    test("one two| three", "one | three");
    test("one two |three", "one |three");
    test("one...|two", "one|two");
    test("one \t |two", "one|two");
    test("one\n|two", "one|two");
    test("one \n |two", "one \n|two");
    test("one \n|two", "one |two");
    test("one|two", "|two");
}

#[test]
fn test_cm6_move_line_up() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, move_line_up);
        assert_eq!(state_str(&next_state), expected);
    }

    test("one\ntwo|\nthree", "two|\none\nthree");
    test("one\nt|w|o|\n", "t|w|o|\none\n");
    test("one\ntwo\nthr<ee\nfour\nfive>\n", "one\nthr<ee\nfour\nfive>\ntwo\n");
    test("one\n<two\nth>ree\nfo|u<r\nfive>\n", "<two\nth>ree\nfo|u<r\nfive>\none\n");
    test("one\n<two\nthree\n>four", "<two\nthree\n>one\nfour");
}

#[test]
fn test_cm6_move_line_down() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, move_line_down);
        assert_eq!(state_str(&next_state), expected);
    }

    test("one\ntwo|\nthree", "one\nthree\ntwo|");
    test("one\nt|w|o|\nthree", "one\nthree\nt|w|o|");
    test("one\ntwo\nthr<ee\nfour\nfive>\nsix", "one\ntwo\nsix\nthr<ee\nfour\nfive>");
    test("one\n<two\nth>ree\nfo|u<r\nfive>\nsix\n", "one\nsix\n<two\nth>ree\nfo|u<r\nfive>\n");
    test("one\n<two\nthree\n>four\n", "one\nfour\n<two\nthree\n>");
    test("one\n<two\nthree\n>four", "one\nfour\n<two\nthree>");
}

#[test]
fn test_cm6_split_line() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, split_line);
        assert_eq!(state_str(&next_state), expected);
    }

    test("one|two", "one|\ntwo");
    test("<one>two", "|\ntwo");
}

#[test]
fn test_cm6_transpose_chars() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next_state = run_cmd(&state, transpose_chars);
        assert_eq!(state_str(&next_state), expected);
    }

    // Swaps 'b' (before cursor) and 'c' (after cursor) -> 'c' followed by 'b'
    test("ab|cd", "acb|d");
    // Swaps 'a' (before cursor) and 'b' (after cursor) -> 'b' followed by 'a'
    test("a|bc", "ba|c");
    // Multi-line: at line boundary pos == line.to but line.to < doc.length
    test("ab|\ncd", "a\nb|cd");
}

use ferrum_state::change::ChangeSpec;
use ferrum_state::transaction::TransactionSpec;
use ferrum_state::{AddToHistory, IsolateHistory, TransactionTime};
use crate::history::{
    history, redo, redo_depth, undo, undo_depth, HistoryConfig,
};

fn mk_history_state(config: Option<HistoryConfig>, doc: Option<&str>) -> std::sync::Arc<EditorState> {
    EditorState::create(EditorStateConfig {
        doc: doc.map(Text::from),
        selection: None,
        extensions: Some(vec![history(config.unwrap_or_default())]),
    })
}

fn type_text(state: &std::sync::Arc<EditorState>, text: &str) -> std::sync::Arc<EditorState> {
    type_at(state, text, state.doc().len_chars())
}

fn type_at(state: &std::sync::Arc<EditorState>, text: &str, at: usize) -> std::sync::Arc<EditorState> {
    let spec = TransactionSpec::new().with_change(ChangeSpec::insert(at, text));
    let tr = state.update(spec).expect("type transaction");
    state.apply(tr)
}

fn timed_type(state: &std::sync::Arc<EditorState>, text: &str, at_time: u64) -> std::sync::Arc<EditorState> {
    let spec = TransactionSpec::new()
        .with_change(ChangeSpec::insert(state.doc().len_chars(), text))
        .with_annotation(TransactionTime(at_time));
    let tr = state.update(spec).expect("timed type transaction");
    state.apply(tr)
}

fn receive(state: &std::sync::Arc<EditorState>, text: &str, from: usize, to: usize) -> std::sync::Arc<EditorState> {
    let spec = TransactionSpec::new()
        .with_change(ChangeSpec::replace(from, to, text))
        .with_annotation(AddToHistory(false));
    let tr = state.update(spec).expect("remote receive transaction");
    state.apply(tr)
}

fn exec_cmd(
    state: &std::sync::Arc<EditorState>,
    cmd: impl Fn(&EditorState) -> Option<TransactionSpec>,
    expect_success: bool,
) -> std::sync::Arc<EditorState> {
    if let Some(spec) = cmd(state) {
        assert!(expect_success, "Expected command to fail but it succeeded");
        if let Some(tr) = state.update(spec) {
            return state.apply(tr);
        }
    } else {
        assert!(!expect_success, "Expected command to succeed but returned None");
    }
    state.clone()
}

#[test]
fn test_cm6_history_allows_undo_and_redo() {
    let state = mk_history_state(None, None);
    let state = type_text(&state, "newtext");
    assert_eq!(state.doc().to_string(), "newtext");
    assert_eq!(undo_depth(&state), 1);
    assert_eq!(redo_depth(&state), 0);

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
    assert_eq!(undo_depth(&state), 0);
    assert_eq!(redo_depth(&state), 1);

    let state = exec_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "newtext");
    assert_eq!(undo_depth(&state), 1);
    assert_eq!(redo_depth(&state), 0);
}

#[test]
fn test_cm6_history_undoes_nearby_changes_in_one_step() {
    let state = mk_history_state(None, None);
    let state = type_text(&state, "new");
    let state = type_text(&state, "text");
    assert_eq!(state.doc().to_string(), "newtext");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");

    let state = exec_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "newtext");
}

#[test]
fn test_cm6_history_tracks_multiple_levels() {
    let state = mk_history_state(None, Some("one"));
    let state = type_text(&state, "new");
    let state = type_text(&state, "text");
    let state = type_at(&state, "some", 0);
    assert_eq!(state.doc().to_string(), "someonenewtext");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "onenewtext");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "one");

    let state = exec_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "onenewtext");

    let state = exec_cmd(&state, redo, true);
    assert_eq!(state.doc().to_string(), "someonenewtext");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "onenewtext");
}

#[test]
fn test_cm6_history_new_group_delay() {
    let config = HistoryConfig {
        new_group_delay: 1000,
        ..Default::default()
    };
    let state = mk_history_state(Some(config), None);
    let state = timed_type(&state, "a", 1000);
    let state = timed_type(&state, "b", 1600);
    assert_eq!(undo_depth(&state), 1);

    let state = timed_type(&state, "c", 2700);
    assert_eq!(undo_depth(&state), 2);

    let state = exec_cmd(&state, undo, true);
    let state = timed_type(&state, "d", 2800);
    assert_eq!(undo_depth(&state), 2);
}

#[test]
fn test_cm6_history_isolate_history() {
    let state = mk_history_state(None, None);
    let state = type_text(&state, "hello");
    let spec = TransactionSpec::new().with_annotation(IsolateHistory::Before);
    let tr = state.update(spec).unwrap();
    let state = state.apply(tr);

    let state = type_text(&state, "!");
    assert_eq!(state.doc().to_string(), "hello!");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "hello");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_history_allows_untracked_changes() {
    let state = mk_history_state(None, None);
    let state = type_text(&state, "hello");
    let state = receive(&state, "oops", 0, 0); // Not added to history
    let state = receive(&state, "!", 9, 9);
    assert_eq!(state.doc().to_string(), "oopshello!");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "oops!");
}

#[test]
fn test_cm6_history_custom_join_predicate() {
    let config = HistoryConfig {
        join_to_event: Some(std::sync::Arc::new(|tr, adj| {
            if !adj {
                return false;
            }
            let mut space = false;
            tr.changes().iter_changes(
                |_, _, _, _, text| {
                    if !text.is_empty() && text.slice_to_string(0..1) == " " {
                        space = true;
                    }
                },
                false,
            );
            !space
        })),
        ..Default::default()
    };
    let state = mk_history_state(Some(config), None);
    let mut state = state;
    for ch in "ab cd".chars() {
        state = type_text(&state, &ch.to_string());
    }
    assert_eq!(state.doc().to_string(), "ab cd");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "ab");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), "");
}

#[test]
fn test_cm6_history_undo_not_adding_redo_item() {
    let state = mk_history_state(None, Some("ab"));
    let state = type_at(&state, "cd", 1);
    let state = receive(&state, "123", 0, 4);
    let state = exec_cmd(&state, undo, false);
    let _state = exec_cmd(&state, redo, false);
}

#[test]
fn test_cm6_history_complex_editing_sequences() {
    let state = mk_history_state(None, None);
    let state = type_text(&state, "hello");
    let tr = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    let state = state.apply(tr);
    let state = type_text(&state, "!");
    let state = receive(&state, "....", 0, 0);
    let state = type_at(&state, "\n\n", 2);
    assert_eq!(state.doc().to_string(), "..\n\n..hello!");

    let state = receive(&state, "\n\n", 1, 1);
    let state = exec_cmd(&state, undo, true);
    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), ".\n\n...hello");

    let state = exec_cmd(&state, undo, true);
    assert_eq!(state.doc().to_string(), ".\n\n...");
}

#[test]
fn test_cm6_history_back_and_forth_multiple_times() {
    let state = mk_history_state(None, None);
    let state = type_text(&state, "one");
    let state = type_text(&state, " two");
    let tr = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    let state = state.apply(tr);
    let state = type_text(&state, " three");
    let state = type_at(&state, "zero ", 0);
    let tr = state.update(TransactionSpec::new().with_annotation(IsolateHistory::Before)).unwrap();
    let state = state.apply(tr);
    let state = type_at(&state, "\n\n", 0);
    let mut state = type_at(&state, "top", 0);

    for i in 0..6 {
        let is_redo = i % 2 == 1;
        for _ in 0..4 {
            state = exec_cmd(&state, if is_redo { redo } else { undo }, true);
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



