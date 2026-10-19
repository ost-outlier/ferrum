//! Commenting and uncommenting commands for the Ferrum editor engine.
//! Faithful port of CodeMirror 6's `@codemirror/commands/src/comment.ts`.

use ferrum_state::change::{ChangeSet, ChangeSpec, Changes};
use ferrum_state::facet::Facet;
use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;
use std::sync::LazyLock;

/// Configuration for block comment tokens.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BlockToken {
    pub open: String,
    pub close: String,
}

impl BlockToken {
    pub fn new(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            open: open.into(),
            close: close.into(),
        }
    }
}

/// An object of this type configures comment syntax for a language.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommentTokens {
    /// The block comment syntax, if any.
    pub block: Option<BlockToken>,
    /// The line comment syntax, if any.
    pub line: Option<String>,
}

impl CommentTokens {
    pub fn line(token: impl Into<String>) -> Self {
        Self {
            block: None,
            line: Some(token.into()),
        }
    }

    pub fn block(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            block: Some(BlockToken::new(open, close)),
            line: None,
        }
    }

    pub fn both(line: impl Into<String>, open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            block: Some(BlockToken::new(open, close)),
            line: Some(line.into()),
        }
    }
}

/// Facet to configure comment tokens for a document or language.
pub static COMMENT_TOKENS: LazyLock<Facet<CommentTokens, CommentTokens>> = LazyLock::new(|| {
    Facet::define(|inputs: &[CommentTokens]| {
        inputs.last().cloned().unwrap_or_default()
    })
});

#[derive(Clone, Copy, PartialEq, Eq)]
enum CommentOption {
    Toggle,
    Comment,
    Uncomment,
}

fn get_config(state: &EditorState, _pos: usize) -> CommentTokens {
    state.facet(&COMMENT_TOKENS).clone()
}

/// Comment or uncomment the current selection. Will use line comments if available,
/// otherwise falling back to block comments.
pub fn toggle_comment(state: &EditorState) -> Option<TransactionSpec> {
    let line = state.doc().line_at(state.selection().main().from());
    let config = get_config(state, line.from);
    if config.line.is_some() {
        toggle_line_comment(state)
    } else if config.block.is_some() {
        toggle_block_comment_by_line(state)
    } else {
        None
    }
}

/// Comment or uncomment the current selection using line comments.
pub fn toggle_line_comment(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<(usize, usize)> = state.selection().ranges().iter().map(|r| (r.from(), r.to())).collect();
    change_line_comment(CommentOption::Toggle, state, &ranges)
}

/// Comment the current selection using line comments.
pub fn line_comment(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<(usize, usize)> = state.selection().ranges().iter().map(|r| (r.from(), r.to())).collect();
    change_line_comment(CommentOption::Comment, state, &ranges)
}

/// Uncomment the current selection using line comments.
pub fn line_uncomment(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<(usize, usize)> = state.selection().ranges().iter().map(|r| (r.from(), r.to())).collect();
    change_line_comment(CommentOption::Uncomment, state, &ranges)
}

/// Comment or uncomment the current selection using block comments.
pub fn toggle_block_comment(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<(usize, usize)> = state.selection().ranges().iter().map(|r| (r.from(), r.to())).collect();
    change_block_comment(CommentOption::Toggle, state, &ranges)
}

/// Comment the current selection using block comments.
pub fn block_comment(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<(usize, usize)> = state.selection().ranges().iter().map(|r| (r.from(), r.to())).collect();
    change_block_comment(CommentOption::Comment, state, &ranges)
}

/// Uncomment the current selection using block comments.
pub fn block_uncomment(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<(usize, usize)> = state.selection().ranges().iter().map(|r| (r.from(), r.to())).collect();
    change_block_comment(CommentOption::Uncomment, state, &ranges)
}

/// Comment or uncomment the lines around the current selection using block comments.
pub fn toggle_block_comment_by_line(state: &EditorState) -> Option<TransactionSpec> {
    let ranges = selected_line_ranges(state);
    change_block_comment(CommentOption::Toggle, state, &ranges)
}

struct LineInfo {
    line_from: usize,
    line_text: String,
    comment: isize,
    token: String,
    indent: usize,
    empty: bool,
    single: bool,
}

fn change_line_comment(
    option: CommentOption,
    state: &EditorState,
    ranges: &[(usize, usize)],
) -> Option<TransactionSpec> {
    let mut lines: Vec<LineInfo> = Vec::new();
    let mut prev_line: isize = -1;

    for &(from, to) in ranges {
        let start_i = lines.len();
        let mut min_indent = usize::MAX;
        let mut token: Option<String> = None;

        let mut pos = from;
        while pos <= to {
            let line = state.doc().line_at(pos);
            if token.is_none() {
                token = get_config(state, line.from).line;
                if token.is_none() {
                    break;
                }
            }
            let tok = token.as_ref().unwrap();

            if (line.from as isize) > prev_line && (from == to || to > line.from) {
                prev_line = line.from as isize;
                let text = &line.text;
                let indent = text.len() - text.trim_start().len();
                let empty = indent == text.len();
                let comment = if text[indent..].starts_with(tok.as_str()) {
                    indent as isize
                } else {
                    -1
                };
                if indent < text.len() && indent < min_indent {
                    min_indent = indent;
                }
                lines.push(LineInfo {
                    line_from: line.from,
                    line_text: text.clone(),
                    comment,
                    token: tok.clone(),
                    indent,
                    empty,
                    single: false,
                });
            }
            pos = line.to + 1;
        }

        if min_indent < usize::MAX {
            for i in start_i..lines.len() {
                if lines[i].indent < lines[i].line_text.len() {
                    lines[i].indent = min_indent;
                }
            }
        }
        if lines.len() == start_i + 1 {
            lines[start_i].single = true;
        }
    }

    if option != CommentOption::Uncomment && lines.iter().any(|l| l.comment < 0 && (!l.empty || l.single)) {
        let mut specs = Vec::new();
        for l in &lines {
            if l.single || !l.empty {
                specs.push(ChangeSpec::insert(l.line_from + l.indent, format!("{} ", l.token)));
            }
        }
        let cs = ChangeSet::of_specs(&specs, state.doc().len_chars());
        let new_sel = state.selection().map(cs.desc(), 1);
        Some(
            TransactionSpec::new()
                .with_changes(Changes::Set(cs))
                .with_selection(new_sel),
        )
    } else if option != CommentOption::Comment && lines.iter().any(|l| l.comment >= 0) {
        let mut specs = Vec::new();
        for l in &lines {
            if l.comment >= 0 {
                let from = l.line_from + (l.comment as usize);
                let mut to = from + l.token.len();
                if l.line_text.as_bytes().get(to - l.line_from) == Some(&b' ') {
                    to += 1;
                }
                specs.push(ChangeSpec::delete(from, to));
            }
        }
        let cs = ChangeSet::of_specs(&specs, state.doc().len_chars());
        let new_sel = state.selection().map(cs.desc(), 1);
        Some(
            TransactionSpec::new()
                .with_changes(Changes::Set(cs))
                .with_selection(new_sel),
        )
    } else {
        None
    }
}

fn selected_line_ranges(state: &EditorState) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for r in state.selection().ranges() {
        let from_line = state.doc().line_at(r.from());
        let mut to_line = if r.to() <= from_line.to {
            from_line.clone()
        } else {
            state.doc().line_at(r.to())
        };

        if to_line.from > from_line.from && to_line.from == r.to() {
            to_line = if r.to() == from_line.to + 1 {
                from_line.clone()
            } else {
                state.doc().line_at(r.to().saturating_sub(1))
            };
        }

        let leading = from_line.text.len() - from_line.text.trim_start().len();
        if let Some(last) = ranges.last_mut() {
            if last.1 > from_line.from {
                last.1 = to_line.to;
                continue;
            }
        }
        ranges.push((from_line.from + leading, to_line.to));
    }
    ranges
}

const SEARCH_MARGIN: usize = 50;

#[derive(Clone, Copy)]
struct PosMargin {
    pos: usize,
    margin: usize,
}

#[derive(Clone, Copy)]
struct BlockCommentInfo {
    open: PosMargin,
    close: PosMargin,
}

fn find_block_comment(
    state: &EditorState,
    token: &BlockToken,
    from: usize,
    to: usize,
) -> Option<BlockCommentInfo> {
    let doc_str = state.doc().to_string();
    let text_before = &doc_str[from.saturating_sub(SEARCH_MARGIN)..from];
    let after_end = (to + SEARCH_MARGIN).min(doc_str.len());
    let text_after = &doc_str[to..after_end];

    let space_before = text_before.len() - text_before.trim_end().len();
    let space_after = text_after.len() - text_after.trim_start().len();
    let before_off = text_before.len() - space_before;

    if before_off >= token.open.len()
        && &text_before[before_off - token.open.len()..before_off] == token.open.as_str()
        && text_after.len() >= space_after + token.close.len()
        && &text_after[space_after..space_after + token.close.len()] == token.close.as_str()
    {
        return Some(BlockCommentInfo {
            open: PosMargin {
                pos: from - space_before,
                margin: if space_before > 0 { 1 } else { 0 },
            },
            close: PosMargin {
                pos: to + space_after,
                margin: if space_after > 0 { 1 } else { 0 },
            },
        });
    }

    let (start_text, end_text) = if to - from <= 2 * SEARCH_MARGIN {
        let s = &doc_str[from..to];
        (s, s)
    } else {
        (&doc_str[from..from + SEARCH_MARGIN], &doc_str[to - SEARCH_MARGIN..to])
    };

    let start_space = start_text.len() - start_text.trim_start().len();
    let end_space = end_text.len() - end_text.trim_end().len();

    if end_text.len() >= end_space + token.close.len() {
        let end_off = end_text.len() - end_space - token.close.len();
        if start_text.len() >= start_space + token.open.len()
            && &start_text[start_space..start_space + token.open.len()] == token.open.as_str()
            && &end_text[end_off..end_off + token.close.len()] == token.close.as_str()
        {
            let open_margin = if start_text.len() > start_space + token.open.len()
                && start_text[start_space + token.open.len()..]
                    .chars()
                    .next()
                    .map_or(false, |c| c.is_whitespace())
            {
                1
            } else {
                0
            };
            let close_margin = if end_off > 0
                && end_text[..end_off]
                    .chars()
                    .last()
                    .map_or(false, |c| c.is_whitespace())
            {
                1
            } else {
                0
            };

            return Some(BlockCommentInfo {
                open: PosMargin {
                    pos: from + start_space + token.open.len(),
                    margin: open_margin,
                },
                close: PosMargin {
                    pos: to - end_space - token.close.len(),
                    margin: close_margin,
                },
            });
        }
    }

    None
}

fn change_block_comment(
    option: CommentOption,
    state: &EditorState,
    ranges: &[(usize, usize)],
) -> Option<TransactionSpec> {
    let tokens: Vec<Option<BlockToken>> = ranges
        .iter()
        .map(|&(from, _)| get_config(state, from).block)
        .collect();
    if !tokens.iter().all(|t| t.is_some()) {
        return None;
    }
    let unwrapped_tokens: Vec<BlockToken> = tokens.into_iter().map(|t| t.unwrap()).collect();
    let comments: Vec<Option<BlockCommentInfo>> = ranges
        .iter()
        .zip(&unwrapped_tokens)
        .map(|(&(from, to), tok)| find_block_comment(state, tok, from, to))
        .collect();

    if option != CommentOption::Uncomment && !comments.iter().all(|c| c.is_some()) {
        let mut specs = Vec::new();
        for (i, &(from, to)) in ranges.iter().enumerate() {
            if comments[i].is_none() {
                specs.push(ChangeSpec::insert(from, format!("{} ", unwrapped_tokens[i].open)));
                specs.push(ChangeSpec::insert(to, format!(" {}", unwrapped_tokens[i].close)));
            }
        }
        let cs = ChangeSet::of_specs(&specs, state.doc().len_chars());
        Some(
            TransactionSpec::new()
                .with_changes(Changes::Set(cs)),
        )
    } else if option != CommentOption::Comment && comments.iter().any(|c| c.is_some()) {
        let mut specs = Vec::new();
        for (i, comment) in comments.iter().enumerate() {
            if let Some(c) = comment {
                let token = &unwrapped_tokens[i];
                let open_from = c.open.pos - token.open.len();
                let open_to = c.open.pos + c.open.margin;
                let close_from = c.close.pos - c.close.margin;
                let close_to = c.close.pos + token.close.len();
                specs.push(ChangeSpec::delete(open_from, open_to));
                specs.push(ChangeSpec::delete(close_from, close_to));
            }
        }
        let cs = ChangeSet::of_specs(&specs, state.doc().len_chars());
        Some(
            TransactionSpec::new()
                .with_changes(Changes::Set(cs)),
        )
    } else {
        None
    }
}
