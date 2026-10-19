use crate::comment::{
    toggle_block_comment, toggle_block_comment_by_line, toggle_line_comment, BlockToken,
    CommentTokens, COMMENT_TOKENS,
};
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use ferrum_state::transaction::TransactionSpec;
use std::sync::Arc;

fn s(doc_with_markers: &str, config: CommentTokens) -> Arc<EditorState> {
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

    EditorState::create(EditorStateConfig {
        doc: Some(Text::from(clean.as_str())),
        selection: Some(sel),
        extensions: Some(vec![
            EditorState::allow_multiple_selections().of(true),
            COMMENT_TOKENS.of(config),
        ]),
    })
}

fn same(actual: &EditorState, expected: &EditorState) {
    assert_eq!(actual.doc().to_string(), expected.doc().to_string());
    assert_eq!(actual.selection().main().anchor, expected.selection().main().anchor);
    assert_eq!(actual.selection().main().head, expected.selection().main().head);
    assert_eq!(actual.selection().ranges().len(), expected.selection().ranges().len());
    for (r_act, r_exp) in actual.selection().ranges().iter().zip(expected.selection().ranges().iter()) {
        assert_eq!(r_act.anchor, r_exp.anchor);
        assert_eq!(r_act.head, r_exp.head);
    }
}

fn check_toggle_chain(
    cmd: impl Fn(&EditorState) -> Option<TransactionSpec>,
    config: CommentTokens,
    docs: &[&str],
) {
    let mut state = s(docs[0], config.clone());
    for i in 1..=docs.len() {
        if let Some(spec) = cmd(&state) {
            if let Some(tr) = state.update(spec) {
                state = state.apply(tr);
            }
        }
        let target_idx = if i == docs.len() {
            docs.len().saturating_sub(2)
        } else {
            i
        };
        let expected = s(docs[target_idx], config.clone());
        same(&state, &expected);
    }
}

// -------------------------------------------------------------------------
// Line comments ('//') (9 tests)
// -------------------------------------------------------------------------

fn line_cfg(tok: &str) -> CommentTokens {
    CommentTokens {
        line: Some(tok.to_string()),
        block: None,
    }
}

#[test]
fn test_cm6_comment_line_slash_slash_toggles_in_empty_single_selection() {
    let cfg = line_cfg("//");
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n  // // // //line| 2\nline 3\n",
        "\nline 1\n  // // //line| 2\nline 3\n",
        "\nline 1\n  // //line| 2\nline 3\n",
        "\nline 1\n  //line| 2\nline 3\n",
        "\nline 1\n  line| 2\nline 3\n",
        "\nline 1\n  // line| 2\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n  //line 2|\nline 3\n",
        "\nline 1\n  line 2|\nline 3\n",
        "\nline 1\n  // line 2|\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n|  //line 2\nline 3\n",
        "\nline 1\n|  line 2\nline 3\n",
        "\nline 1\n|  // line 2\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n|//\nline 3\n",
        "\nline 1\n|\nline 3\n",
        "\nline 1\n// |\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg, &[
        "\nline 1\n line 2\nline 3\n|//",
        "\nline 1\n line 2\nline 3\n|",
        "\nline 1\n line 2\nline 3\n// |",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_toggles_comments_in_single_line_when_cursor_at_beginning() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "line 1\n  |line 2\nline 3\n",
        "line 1\n  // |line 2\nline 3\n",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_toggles_comments_in_single_line_selection() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "line 1\n  //li|ne |2\nline 3\n",
        "line 1\n  li|ne |2\nline 3\n",
        "line 1\n  // li|ne |2\nline 3\n",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_toggles_comments_in_multi_line_selection() {
    let cfg = line_cfg("//");
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  //lin|e 1\n  //  line 2\n  // line |3\n",
        "\n  lin|e 1\n   line 2\n  line |3\n",
        "\n  // lin|e 1\n  //  line 2\n  // line |3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  //lin|e 1\n  //  line 2\n   line 3\n  // li|ne 4\n",
        "\n  // //lin|e 1\n  // //  line 2\n  //  line 3\n  // // li|ne 4\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  // lin|e 1\n\n  // line |3\n",
        "\n  lin|e 1\n\n  line |3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  // lin|e 1\n     \n  // line |3\n",
        "\n  lin|e 1\n     \n  line |3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n|\n  // line 2\n    | \n",
        "\n|\n  line 2\n    | \n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg, &[
        "\n|\n\n    | \n",
        "\n|\n\n    | \n",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_toggles_comments_in_multi_line_multi_range_selection() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "\n  lin|e 1\n  line |2\n  line 3\n  l|ine 4\n  line| 5\n",
        "\n  // lin|e 1\n  // line |2\n  line 3\n  // l|ine 4\n  // line| 5\n",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_can_handle_multiple_selections_on_one_line() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "|line| |with| |ranges|",
        "// |line| |with| |ranges|",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_doesnt_include_lines_in_which_a_selection_range_ends() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "line| 1\nline 2\n|line 3",
        "// line| 1\n// line 2\n|line 3",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_leaves_empty_lines_alone() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "line| 1\n\nline 3|",
        "// line| 1\n\n// line 3|",
    ]);
}

#[test]
fn test_cm6_comment_line_slash_slash_does_comment_empty_lines_with_a_cursor() {
    check_toggle_chain(toggle_line_comment, line_cfg("//"), &[
        "|\nline 2",
        "// |\nline 2",
    ]);
}

// -------------------------------------------------------------------------
// Line comments ('#') (9 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_comment_line_hash_toggles_in_empty_single_selection() {
    let cfg = line_cfg("#");
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n  # # # #line| 2\nline 3\n",
        "\nline 1\n  # # #line| 2\nline 3\n",
        "\nline 1\n  # #line| 2\nline 3\n",
        "\nline 1\n  #line| 2\nline 3\n",
        "\nline 1\n  line| 2\nline 3\n",
        "\nline 1\n  # line| 2\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n  #line 2|\nline 3\n",
        "\nline 1\n  line 2|\nline 3\n",
        "\nline 1\n  # line 2|\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n|  #line 2\nline 3\n",
        "\nline 1\n|  line 2\nline 3\n",
        "\nline 1\n|  # line 2\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\nline 1\n|#\nline 3\n",
        "\nline 1\n|\nline 3\n",
        "\nline 1\n# |\nline 3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg, &[
        "\nline 1\n line 2\nline 3\n|#",
        "\nline 1\n line 2\nline 3\n|",
        "\nline 1\n line 2\nline 3\n# |",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_toggles_comments_in_single_line_when_cursor_at_beginning() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "line 1\n  |line 2\nline 3\n",
        "line 1\n  # |line 2\nline 3\n",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_toggles_comments_in_single_line_selection() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "line 1\n  #li|ne |2\nline 3\n",
        "line 1\n  li|ne |2\nline 3\n",
        "line 1\n  # li|ne |2\nline 3\n",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_toggles_comments_in_multi_line_selection() {
    let cfg = line_cfg("#");
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  #lin|e 1\n  #  line 2\n  # line |3\n",
        "\n  lin|e 1\n   line 2\n  line |3\n",
        "\n  # lin|e 1\n  #  line 2\n  # line |3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  #lin|e 1\n  #  line 2\n   line 3\n  # li|ne 4\n",
        "\n  # #lin|e 1\n  # #  line 2\n  #  line 3\n  # # li|ne 4\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  # lin|e 1\n\n  # line |3\n",
        "\n  lin|e 1\n\n  line |3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n  # lin|e 1\n     \n  # line |3\n",
        "\n  lin|e 1\n     \n  line |3\n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg.clone(), &[
        "\n|\n  # line 2\n    | \n",
        "\n|\n  line 2\n    | \n",
    ]);
    check_toggle_chain(toggle_line_comment, cfg, &[
        "\n|\n\n    | \n",
        "\n|\n\n    | \n",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_toggles_comments_in_multi_line_multi_range_selection() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "\n  lin|e 1\n  line |2\n  line 3\n  l|ine 4\n  line| 5\n",
        "\n  # lin|e 1\n  # line |2\n  line 3\n  # l|ine 4\n  # line| 5\n",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_can_handle_multiple_selections_on_one_line() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "|line| |with| |ranges|",
        "# |line| |with| |ranges|",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_doesnt_include_lines_in_which_a_selection_range_ends() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "line| 1\nline 2\n|line 3",
        "# line| 1\n# line 2\n|line 3",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_leaves_empty_lines_alone() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "line| 1\n\nline 3|",
        "# line| 1\n\n# line 3|",
    ]);
}

#[test]
fn test_cm6_comment_line_hash_does_comment_empty_lines_with_a_cursor() {
    check_toggle_chain(toggle_line_comment, line_cfg("#"), &[
        "|\nline 2",
        "# |\nline 2",
    ]);
}

// -------------------------------------------------------------------------
// Block comments ('/* */') (8 tests)
// -------------------------------------------------------------------------

fn block_cfg(open: &str, close: &str) -> CommentTokens {
    CommentTokens {
        line: None,
        block: Some(BlockToken {
            open: open.to_string(),
            close: close.to_string(),
        }),
    }
}

#[test]
fn test_cm6_comment_block_cstyle_toggles_block_comment_in_multi_line_selection() {
    check_toggle_chain(toggle_block_comment, block_cfg("/*", "*/"), &[
        "\n  lin|e 1\n  line 2\n  line 3\n  line |4\n  line 5\n",
        "\n  lin/* |e 1\n  line 2\n  line 3\n  line | */4\n  line 5\n",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_toggles_block_comment_in_multi_line_multi_range_selection() {
    check_toggle_chain(toggle_block_comment, block_cfg("/*", "*/"), &[
        "\n  lin|e 1\n  line |2\n  l|ine 3\n  line 4\n  line |5\n",
        "\n  lin/* |e 1\n  line | */2\n  l/* |ine 3\n  line 4\n  line | */5\n",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_can_toggle_comments_inside_the_selection() {
    check_toggle_chain(toggle_block_comment, block_cfg("/*", "*/"), &[
        "|/* one\ntwo */| three",
        "|one\ntwo| three",
        "/* |one\ntwo| */ three",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_comments_the_entire_line() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("/*", "*/"), &[
        "one|\ntwo",
        "/* one| */\ntwo",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_comments_multiple_lines() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("/*", "*/"), &[
        "on|e\nt|wo",
        "/* on|e\nt|wo */",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_joins_selected_blocks_of_lines() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("/*", "*/"), &[
        "on|e\nt|w|o\nth|ree",
        "/* on|e\nt|w|o\nth|ree */",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_doesnt_include_lines_that_the_selection_stops_at_the_start_of() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("/*", "*/"), &[
        "|one\n|two",
        "/* |one */\n|two",
    ]);
}

#[test]
fn test_cm6_comment_block_cstyle_does_include_lines_with_cursor_selection_at_the_start() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("/*", "*/"), &[
        "|one\ntwo",
        "|/* one */\ntwo",
    ]);
}

// -------------------------------------------------------------------------
// Block comments ('<!-- -->') (8 tests)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_comment_block_html_toggles_block_comment_in_multi_line_selection() {
    check_toggle_chain(toggle_block_comment, block_cfg("<!--", "-->"), &[
        "\n  lin|e 1\n  line 2\n  line 3\n  line |4\n  line 5\n",
        "\n  lin<!-- |e 1\n  line 2\n  line 3\n  line | -->4\n  line 5\n",
    ]);
}

#[test]
fn test_cm6_comment_block_html_toggles_block_comment_in_multi_line_multi_range_selection() {
    check_toggle_chain(toggle_block_comment, block_cfg("<!--", "-->"), &[
        "\n  lin|e 1\n  line |2\n  l|ine 3\n  line 4\n  line |5\n",
        "\n  lin<!-- |e 1\n  line | -->2\n  l<!-- |ine 3\n  line 4\n  line | -->5\n",
    ]);
}

#[test]
fn test_cm6_comment_block_html_can_toggle_comments_inside_the_selection() {
    check_toggle_chain(toggle_block_comment, block_cfg("<!--", "-->"), &[
        "|<!-- one\ntwo -->| three",
        "|one\ntwo| three",
        "<!-- |one\ntwo| --> three",
    ]);
}

#[test]
fn test_cm6_comment_block_html_comments_the_entire_line() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("<!--", "-->"), &[
        "one|\ntwo",
        "<!-- one| -->\ntwo",
    ]);
}

#[test]
fn test_cm6_comment_block_html_comments_multiple_lines() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("<!--", "-->"), &[
        "on|e\nt|wo",
        "<!-- on|e\nt|wo -->",
    ]);
}

#[test]
fn test_cm6_comment_block_html_joins_selected_blocks_of_lines() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("<!--", "-->"), &[
        "on|e\nt|w|o\nth|ree",
        "<!-- on|e\nt|w|o\nth|ree -->",
    ]);
}

#[test]
fn test_cm6_comment_block_html_doesnt_include_lines_that_the_selection_stops_at_the_start_of() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("<!--", "-->"), &[
        "|one\n|two",
        "<!-- |one -->\n|two",
    ]);
}

#[test]
fn test_cm6_comment_block_html_does_include_lines_with_cursor_selection_at_the_start() {
    check_toggle_chain(toggle_block_comment_by_line, block_cfg("<!--", "-->"), &[
        "|one\ntwo",
        "|<!-- one -->\ntwo",
    ]);
}

// -------------------------------------------------------------------------
// Multi-language doc (1 test)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_comment_toggles_line_comment_in_multilanguage_doc() {
    let before = "<script>\n  // This is a |line comment\n  console.log(\"Hello\");\n</script>\n<!-- HTML only provides block comments -->";
    let after = "<script>\n  This is a |line comment\n  console.log(\"Hello\");\n</script>\n<!-- HTML only provides block comments -->";
    let cfg = line_cfg("//");
    let state = s(before, cfg.clone());
    let spec = toggle_line_comment(&state);
    let next = if let Some(sp) = spec {
        if let Some(tr) = state.update(sp) { state.apply(tr) } else { state }
    } else {
        state
    };
    same(&next, &s(after, cfg));
}
