use crate::commands::{
    delete_group_backward, delete_group_forward, delete_to_line_end, delete_to_line_start,
    delete_trailing_whitespace, indent_less, indent_more, insert_newline_keep_indent,
    move_line_down, move_line_up, select_line, split_line, transpose_chars,
};
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use std::sync::Arc;

fn mk_state(s: &str) -> Arc<EditorState> {
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
        ..Default::default()
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

// -------------------------------------------------------------------------
// indentMore
// -------------------------------------------------------------------------

#[test]
fn test_cm6_indent_more() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, indent_more);
        assert_eq!(state_str(&next), expected);
    }

    test("on<e\ntwo\n>three", "    on<e\n    two\n>three");
    test("one\ntwo|\nthree", "one\n    two|\nthree");
    test("one\n<two\nthree>", "one\n    <two\n    three>");
}

// -------------------------------------------------------------------------
// indentLess
// -------------------------------------------------------------------------

#[test]
fn test_cm6_indent_less() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, indent_less);
        assert_eq!(state_str(&next), expected);
    }

    test("one\n    two|\nthree", "one\ntwo|\nthree");
    test("one\n  <two\n  three>", "one\n<two\nthree>");
}

// -------------------------------------------------------------------------
// insertNewlineKeepIndent
// -------------------------------------------------------------------------

#[test]
fn test_cm6_insert_newline_keep_indent() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, insert_newline_keep_indent);
        assert_eq!(state_str(&next), expected);
    }

    test("  one|", "  one\n  |");
    test("one|two", "one\n|two");
    test("if x:\n  one<two\n  three>four", "if x:\n  one\n  |four");
}

// -------------------------------------------------------------------------
// deleteTrailingWhitespace
// -------------------------------------------------------------------------

#[test]
fn test_cm6_delete_trailing_whitespace() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, delete_trailing_whitespace);
        assert_eq!(next.doc().to_string(), expected);
    }

    test("foo   ", "foo");
    test("one\ntwo \nthree   \n   ", "one\ntwo\nthree\n");
    test("one  \n\ntwo ", "one\n\ntwo");
}

// -------------------------------------------------------------------------
// deleteGroupForward / Backward
// -------------------------------------------------------------------------

#[test]
fn test_cm6_delete_group_forward() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, delete_group_forward);
        assert_eq!(state_str(&next), expected);
    }

    test("one |two three", "one | three");
    test("one| two three", "one| three");
    test("one|...two", "one|two");
    test("one|  \ttwo", "one|two");
    test("one|\ntwo", "one|two");
    test("one|two", "one|");
    test("one|", "one|");
}

#[test]
fn test_cm6_delete_group_backward() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, delete_group_backward);
        assert_eq!(state_str(&next), expected);
    }

    test("one two| three", "one | three");
    test("one two |three", "one |three");
    test("one...|two", "one|two");
    test("one \t |two", "one|two");
    test("one\n|two", "one|two");
    test("one|two", "|two");
}

// -------------------------------------------------------------------------
// moveLineUp / moveLineDown
// -------------------------------------------------------------------------

#[test]
fn test_cm6_move_line_up() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, move_line_up);
        assert_eq!(state_str(&next), expected);
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
        let next = run_cmd(&state, move_line_down);
        assert_eq!(state_str(&next), expected);
    }

    test("one\ntwo|\nthree", "one\nthree\ntwo|");
    test("one\nt|w|o|\nthree", "one\nthree\nt|w|o|");
    test("one\ntwo\nthr<ee\nfour\nfive>\nsix", "one\ntwo\nsix\nthr<ee\nfour\nfive>");
    test("one\n<two\nth>ree\nfo|u<r\nfive>\nsix\n", "one\nsix\n<two\nth>ree\nfo|u<r\nfive>\n");
    test("one\n<two\nthree\n>four\n", "one\nfour\n<two\nthree\n>");
    test("one\n<two\nthree\n>four", "one\nfour\n<two\nthree>");
}

// -------------------------------------------------------------------------
// Line manipulation: splitLine, transposeChars, selectLine, deleteToLineStart/End
// -------------------------------------------------------------------------

#[test]
fn test_cm6_split_line() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, split_line);
        assert_eq!(state_str(&next), expected);
    }

    test("one|two", "one|\ntwo");
    test("<one>two", "|\ntwo");
}

#[test]
fn test_cm6_transpose_chars() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, transpose_chars);
        assert_eq!(state_str(&next), expected);
    }

    test("ab|cd", "acb|d");
    test("a|bc", "ba|c");
}

#[test]
fn test_cm6_select_line() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, select_line);
        assert_eq!(state_str(&next), expected);
    }

    test("hello|\nworld", "<hello\n>world");
    test("hello\nwor|ld", "hello\n<world>");
    test("one|\ntwo|\nthree", "<one\ntwo\n>three");
    test("one|\ntwo\nthree|", "<one\n>two\n<three>");
    test("on<e\ntw>o\nthree", "<one\ntwo\n>three");
}

#[test]
fn test_cm6_delete_to_line_start() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, delete_to_line_start);
        assert_eq!(state_str(&next), expected);
    }

    test("hello |world", "|world");
    test("line1\n|line2", "line1|line2");
    test("hello <wor>ld", "hello |ld");
}

#[test]
fn test_cm6_delete_to_line_end() {
    fn test(from: &str, expected: &str) {
        let state = mk_state(from);
        let next = run_cmd(&state, delete_to_line_end);
        assert_eq!(state_str(&next), expected);
    }

    test("hello |world\nline2", "hello |\nline2");
    test("line1|\nline2", "line1|line2");
    test("hello <wor>ld", "hello |ld");
}
