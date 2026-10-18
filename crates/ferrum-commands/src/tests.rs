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

