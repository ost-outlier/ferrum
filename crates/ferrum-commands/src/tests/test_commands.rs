use crate::commands::{
    delete_group_backward, delete_group_forward, delete_trailing_whitespace, indent_less,
    indent_more, indent_selection, insert_newline_and_indent, insert_newline_keep_indent,
    move_line_down, move_line_up,
};
use ferrum_language::indent::INDENT_UNIT;
use ferrum_language::javascript_language;
use ferrum_state::facet::Extension;
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use std::sync::Arc;

fn mk_state(s: &str) -> Arc<EditorState> {
    mk_state_with_ext(s, vec![])
}

fn mk_state_with_ext(s: &str, exts: Vec<Extension>) -> Arc<EditorState> {
    let mut clean = String::new();
    let mut ranges = Vec::new();
    let mut in_range: Option<usize> = None;

    for ch in s.chars() {
        if ch == '|' {
            let pos = clean.chars().count();
            ranges.push(SelectionRange::cursor(pos));
        } else if ch == '<' {
            in_range = Some(clean.chars().count());
        } else if ch == '>' {
            if let Some(anchor) = in_range.take() {
                let head = clean.chars().count();
                ranges.push(SelectionRange::new(anchor, head));
            }
        } else {
            clean.push(ch);
        }
    }

    let sel = if ranges.is_empty() {
        EditorSelection::cursor(0)
    } else {
        EditorSelection::create(ranges, 0)
    };

    EditorState::create(EditorStateConfig {
        doc: Some(Text::from(clean.as_str())),
        selection: Some(sel),
        extensions: if exts.is_empty() { None } else { Some(exts) },
    })
}

fn state_str(state: &EditorState) -> String {
    let doc = state.doc().to_string();
    let mut points: Vec<(usize, char)> = Vec::new();

    for r in state.selection().ranges() {
        if r.is_empty() {
            points.push((r.head, '|'));
        } else {
            let from = r.from();
            let to = r.to();
            points.push((from, '<'));
            points.push((to, '>'));
        }
    }

    points.sort_by_key(|&(pos, _)| pos);

    let mut result = String::new();
    let mut last_idx = 0;
    for (pos, marker) in points {
        let ch_slice: String = doc.chars().skip(last_idx).take(pos - last_idx).collect();
        result.push_str(&ch_slice);
        result.push(marker);
        last_idx = pos;
    }
    let tail: String = doc.chars().skip(last_idx).collect();
    result.push_str(&tail);
    result
}

fn run_cmd(
    state: &Arc<EditorState>,
    cmd: impl Fn(&EditorState) -> Option<ferrum_state::transaction::TransactionSpec>,
) -> Arc<EditorState> {
    if let Some(spec) = cmd(state) {
        if let Some(tr) = state.update(spec) {
            return state.apply(tr);
        }
    }
    state.clone()
}

fn test_cmd(
    from: &str,
    expected: &str,
    cmd: impl Fn(&EditorState) -> Option<ferrum_state::transaction::TransactionSpec>,
) {
    test_cmd_with_ext(from, expected, vec![], cmd);
}

fn test_cmd_with_ext(
    from: &str,
    expected: &str,
    exts: Vec<Extension>,
    cmd: impl Fn(&EditorState) -> Option<ferrum_state::transaction::TransactionSpec>,
) {
    let st = mk_state_with_ext(from, exts);
    let next = run_cmd(&st, cmd);
    assert_eq!(state_str(&next), expected);
}

// -------------------------------------------------------------------------
// indentMore (4 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_indent_more_adds_indentation() {
    test_cmd("one\ntwo|\nthree", "one\n  two|\nthree", indent_more);
}

#[test]
fn test_cm6_cmd_indent_more_indents_all_lines_in_a_range() {
    test_cmd("one\n<two\nthree>", "one\n  <two\n  three>", indent_more);
}

#[test]
fn test_cm6_cmd_indent_more_doesnt_double_indent_a_given_line() {
    test_cmd("on|e|\n<two\nth><ree\nfour>", "  on|e|\n  <two\n  th><ree\n  four>", indent_more);
}

#[test]
fn test_cm6_cmd_indent_more_ignores_lines_if_a_range_selection_ends_directly_at_their_start() {
    test_cmd("on<e\ntwo\n>three", "  on<e\n  two\n>three", indent_more);
}

// -------------------------------------------------------------------------
// indentLess (5 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_indent_less_removes_indentation() {
    test_cmd("one\n  two|\nthree", "one\ntwo|\nthree", indent_less);
}

#[test]
fn test_cm6_cmd_indent_less_removes_one_unit_of_indentation() {
    test_cmd("one\n    two|\n     three|", "one\n  two|\n   three|", indent_less);
}

#[test]
fn test_cm6_cmd_indent_less_dedents_all_lines_in_a_range() {
    test_cmd("one\n  <two\n  three>", "one\n<two\nthree>", indent_less);
}

#[test]
fn test_cm6_cmd_indent_less_takes_tabs_into_account() {
    test_cmd("   \tone|\n  \ttwo|", "  one|\n  two|", indent_less);
}

#[test]
fn test_cm6_cmd_indent_less_can_split_tabs() {
    test_cmd("\tone|", "  one|", indent_less);
}

// -------------------------------------------------------------------------
// indentSelection (4 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_indent_selection_auto_indents_the_current_line() {
    test_cmd_with_ext("if (0)\nfoo()|", "if (0)\n  foo()|", vec![javascript_language()], indent_selection);
}

#[test]
fn test_cm6_cmd_indent_selection_moves_the_cursor_ahead_of_the_indentation() {
    test_cmd_with_ext("if (0)\n | foo()", "if (0)\n  |foo()", vec![javascript_language()], indent_selection);
}

#[test]
fn test_cm6_cmd_indent_selection_indents_blocks_of_lines() {
    test_cmd_with_ext("if (0) {\n<one\ntwo\nthree>\n}", "if (0) {\n  <one\n  two\n  three>\n}", vec![javascript_language()], indent_selection);
}

#[test]
fn test_cm6_cmd_indent_selection_includes_previous_indentation_changes_in_relative_indentation() {
    test_cmd_with_ext("<{\n{\n{\n{}\n}\n}\n}>", "<{\n  {\n    {\n      {}\n    }\n  }\n}>", vec![javascript_language()], indent_selection);
}

// -------------------------------------------------------------------------
// insertNewlineKeepIndent (3 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_insert_newline_keep_indent_keeps_indentation() {
    test_cmd("  one|", "  one\n  |", insert_newline_keep_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_keep_indent_keeps_zero_indentation() {
    test_cmd("one|two", "one\n|two", insert_newline_keep_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_keep_indent_deletes_the_selection() {
    test_cmd("if x:\n  one<two\n  three>four", "if x:\n  one\n  |four", insert_newline_keep_indent);
}

// -------------------------------------------------------------------------
// insertNewlineAndIndent (14 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_insert_newline_and_indent_indents_the_new_line() {
    test_cmd_with_ext("{|", "{\n  |", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_can_handle_multiple_selections() {
    test_cmd_with_ext("{|\n  foo()|", "{\n  |\n  foo()\n  |", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_isnt_confused_by_text_after_the_cursor() {
    test_cmd_with_ext("{|two", "{\n  |two", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_clears_empty_lines_before_the_cursor() {
    test_cmd_with_ext("    |", "\n|", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_deletes_selected_text() {
    test_cmd_with_ext("{<one>two", "{\n  |two", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_can_explode_brackets() {
    test_cmd_with_ext("let x = [|]", "let x = [\n  |\n]", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_can_explode_in_indented_positions() {
    test_cmd_with_ext("{\n  foo(|)", "{\n  foo(\n    |\n  )", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_can_explode_brackets_with_whitespace() {
    test_cmd_with_ext("foo( | )", "foo(\n  |\n)", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_doesnt_try_to_explode_already_exploded_brackets() {
    test_cmd_with_ext("foo(\n  |\n)", "foo(\n\n  |\n)", vec![javascript_language()], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_doesnt_indent_when_previous_line_lacks_indentation() {
    test_cmd("foo|", "foo\n|", insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_indents_when_previous_line_uses_two_space_indentation() {
    test_cmd("  foo|", "  foo\n  |", insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_indents_when_previous_line_uses_four_space_indentation() {
    test_cmd("    foo|", "    foo\n    |", insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_indents_when_previous_line_uses_tab_indentation() {
    test_cmd_with_ext("\tfoo|", "\tfoo\n\t|", vec![INDENT_UNIT.of("\t".to_string())], insert_newline_and_indent);
}

#[test]
fn test_cm6_cmd_insert_newline_and_indent_indents_when_previous_line_uses_tab_indentation_and_short_alignment() {
    test_cmd_with_ext("\t  foo|", "\t  foo\n\t  |", vec![INDENT_UNIT.of("\t".to_string())], insert_newline_and_indent);
}

// -------------------------------------------------------------------------
// deleteTrailingWhitespace (3 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_delete_trailing_whitespace_deletes_trailing_whitespace() {
    let st = mk_state("foo   ");
    let next = run_cmd(&st, delete_trailing_whitespace);
    assert_eq!(next.doc().to_string(), "foo");
}

#[test]
fn test_cm6_cmd_delete_trailing_whitespace_checks_multiple_lines() {
    let st = mk_state("one\ntwo \nthree   \n   ");
    let next = run_cmd(&st, delete_trailing_whitespace);
    assert_eq!(next.doc().to_string(), "one\ntwo\nthree\n");
}

#[test]
fn test_cm6_cmd_delete_trailing_whitespace_can_handle_empty_lines() {
    let st = mk_state("one  \n\ntwo ");
    let next = run_cmd(&st, delete_trailing_whitespace);
    assert_eq!(next.doc().to_string(), "one\n\ntwo");
}

// -------------------------------------------------------------------------
// deleteGroupForward (9 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_delete_group_forward_deletes_a_word() {
    test_cmd("one |two three", "one | three", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_deletes_a_word_with_leading_space() {
    test_cmd("one| two three", "one| three", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_deletes_a_group_of_punctuation() {
    test_cmd("one|...two", "one|two", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_deletes_a_group_of_space() {
    test_cmd("one|  \ttwo", "one|two", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_deletes_a_newline() {
    test_cmd("one|\ntwo", "one|two", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_stops_deleting_at_a_newline() {
    test_cmd("one| \n two", "one|\n two", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_stops_deleting_after_a_newline() {
    test_cmd("one|\n two", "one| two", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_deletes_up_to_the_end_of_the_doc() {
    test_cmd("one|two", "one|", delete_group_forward);
}

#[test]
fn test_cm6_cmd_delete_group_forward_does_nothing_at_the_end_of_the_doc() {
    test_cmd("one|", "one|", delete_group_forward);
}

// -------------------------------------------------------------------------
// deleteGroupBackward (8 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_delete_group_backward_deletes_a_word() {
    test_cmd("one two| three", "one | three", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_deletes_a_word_with_trailing_space() {
    test_cmd("one two |three", "one |three", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_deletes_a_group_of_punctuation() {
    test_cmd("one...|two", "one|two", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_deletes_a_group_of_space() {
    test_cmd("one \t |two", "one|two", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_deletes_a_newline() {
    test_cmd("one\n|two", "one|two", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_stops_deleting_at_a_newline() {
    test_cmd("one \n |two", "one \n|two", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_stops_deleting_after_a_newline() {
    test_cmd("one \n|two", "one |two", delete_group_backward);
}

#[test]
fn test_cm6_cmd_delete_group_backward_deletes_up_to_the_start_of_the_doc() {
    test_cmd("one|two", "|two", delete_group_backward);
}

// -------------------------------------------------------------------------
// moveLineUp (5 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_move_line_up_can_move_a_line_up() {
    test_cmd("one\ntwo|\nthree", "two|\none\nthree", move_line_up);
}

#[test]
fn test_cm6_cmd_move_line_up_preserves_multiple_cursors_on_a_single_line() {
    test_cmd("one\nt|w|o|\n", "t|w|o|\none\n", move_line_up);
}

#[test]
fn test_cm6_cmd_move_line_up_moves_selected_blocks_as_one() {
    test_cmd("one\ntwo\nthr<ee\nfour\nfive>\n", "one\nthr<ee\nfour\nfive>\ntwo\n", move_line_up);
}

#[test]
fn test_cm6_cmd_move_line_up_moves_blocks_made_of_multiple_ranges_as_one() {
    test_cmd("one\n<two\nth>ree\nfo|u<r\nfive>\n", "<two\nth>ree\nfo|u<r\nfive>\none\n", move_line_up);
}

#[test]
fn test_cm6_cmd_move_line_up_does_not_include_a_trailing_line_after_a_range() {
    test_cmd("one\n<two\nthree\n>four", "<two\nthree\n>one\nfour", move_line_up);
}

// -------------------------------------------------------------------------
// moveLineDown (6 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_cmd_move_line_down_can_move_a_line_down() {
    test_cmd("one\ntwo|\nthree", "one\nthree\ntwo|", move_line_down);
}

#[test]
fn test_cm6_cmd_move_line_down_preserves_multiple_cursors_on_a_single_line() {
    test_cmd("one\nt|w|o|\nthree", "one\nthree\nt|w|o|", move_line_down);
}

#[test]
fn test_cm6_cmd_move_line_down_moves_selected_blocks_as_one() {
    test_cmd("one\ntwo\nthr<ee\nfour\nfive>\nsix", "one\ntwo\nsix\nthr<ee\nfour\nfive>", move_line_down);
}

#[test]
fn test_cm6_cmd_move_line_down_moves_blocks_made_of_multiple_ranges_as_one() {
    test_cmd("one\n<two\nth>ree\nfo|u<r\nfive>\nsix\n", "one\nsix\n<two\nth>ree\nfo|u<r\nfive>\n", move_line_down);
}

#[test]
fn test_cm6_cmd_move_line_down_does_not_include_a_trailing_line_after_a_range() {
    test_cmd("one\n<two\nthree\n>four\n", "one\nfour\n<two\nthree\n>", move_line_down);
}

#[test]
fn test_cm6_cmd_move_line_down_clips_the_selection_when_moving_to_the_end_of_the_doc() {
    test_cmd("one\n<two\nthree\n>four", "one\nfour\n<two\nthree>", move_line_down);
}
