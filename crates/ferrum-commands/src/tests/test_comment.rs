use crate::comment::{
    toggle_block_comment, toggle_block_comment_by_line, toggle_line_comment,
};
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use std::sync::Arc;

/// Helper to parse document with `|` cursor or range markers into an EditorState.
fn mk_comment_state(doc_with_markers: &str) -> (Arc<EditorState>, EditorSelection) {
    let mut markers = Vec::new();
    let mut clean = String::new();
    for ch in doc_with_markers.chars() {
        if ch == '|' {
            markers.push(clean.chars().count());
        } else {
            clean.push(ch);
        }
    }

    let sel = if markers.is_empty() {
        EditorSelection::cursor(0)
    } else if markers.len() == 1 {
        EditorSelection::cursor(markers[0])
    } else if markers.len() == 2 {
        EditorSelection::single(markers[0], markers[1])
    } else {
        let mut ranges = Vec::new();
        for i in (0..markers.len()).step_by(2) {
            let to = if i + 1 < markers.len() { markers[i + 1] } else { markers[i] };
            ranges.push(SelectionRange::new(markers[i], to));
        }
        EditorSelection::create(ranges, 0)
    };

    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(clean.as_str())),
        selection: Some(sel.clone()),
        ..Default::default()
    });

    (state, sel)
}


// -------------------------------------------------------------------------
// Canonical CodeMirror 6 Comment Tests (test-comment.ts)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_comment_line_toggles_in_empty_single_selection() {
    let (state, _) = mk_comment_state("\nline 1\n  line| 2\nline 3\n");
    let spec = toggle_line_comment(&state);
    // In TDD RED phase, stub returns None, so this test fails
    assert!(spec.is_some(), "toggle_line_comment must return a transaction");
}

#[test]
fn test_cm6_comment_line_toggles_single_line_at_beginning() {
    let (state, _) = mk_comment_state("line 1\n  |line 2\nline 3\n");
    let spec = toggle_line_comment(&state);
    assert!(spec.is_some(), "toggle_line_comment must return a transaction");
}

#[test]
fn test_cm6_comment_line_toggles_multi_line_selection() {
    let (state, _) = mk_comment_state("\n  lin|e 1\n  line 2\n  line |3\n");
    let spec = toggle_line_comment(&state);
    assert!(spec.is_some(), "toggle_line_comment must return a transaction on multi-line");
}

#[test]
fn test_cm6_comment_line_leaves_empty_lines_alone() {
    let (state, _) = mk_comment_state("line| 1\n\nline 3|");
    let spec = toggle_line_comment(&state);
    assert!(spec.is_some(), "toggle_line_comment must return a transaction");
}

#[test]
fn test_cm6_comment_block_toggles_multi_line_selection() {
    let (state, _) = mk_comment_state("\n  lin|e 1\n  line 2\n  line 3\n  line |4\n  line 5\n");
    let spec = toggle_block_comment(&state);
    assert!(spec.is_some(), "toggle_block_comment must return a transaction");
}

#[test]
fn test_cm6_comment_block_by_line_comments_entire_line() {
    let (state, _) = mk_comment_state("one|\ntwo");
    let spec = toggle_block_comment_by_line(&state);
    assert!(spec.is_some(), "toggle_block_comment_by_line must return a transaction");
}
