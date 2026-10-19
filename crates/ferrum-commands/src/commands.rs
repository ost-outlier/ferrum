use ferrum_language::indent::{
    get_indent_unit, get_indentation_with_context, indent_string, IndentContext,
    IndentContextOptions, INDENT_UNIT,
};
use ferrum_state::change::ChangeSpec;
use ferrum_state::char::{
    categorize_char, find_cluster_break, find_subword_break, find_word_break, CharCategory,
};
use ferrum_state::column::count_column;
use ferrum_state::facet::Facet;
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, RangeResult};
use ferrum_state::transaction::TransactionSpec;
use std::cmp::min;
use std::sync::{Arc, LazyLock};

/// Pure command function operating on an `EditorState`.
/// Returns `Some(TransactionSpec)` if the command produced edits or cursor changes,
/// or `None` if the command did not apply.
pub type StateCommand = fn(&EditorState) -> Option<TransactionSpec>;

/// Representation of a key binding that maps a key combination to commands.
#[derive(Clone)]
pub struct KeyBinding {
    /// Key combination identifier (e.g. `"ArrowLeft"`, `"Mod-ArrowLeft"`, `"Enter"`, `"Backspace"`, `"Mod-a"`).
    pub key: &'static str,
    /// Primary command to run.
    pub run: StateCommand,
    /// Optional command to run when Shift is held.
    pub shift: Option<StateCommand>,
}

impl KeyBinding {
    /// Creates a key binding with a primary command.
    pub fn new(key: &'static str, run: StateCommand) -> Self {
        Self {
            key,
            run,
            shift: None,
        }
    }

    /// Attaches a Shift variant command.
    pub fn with_shift(mut self, shift: StateCommand) -> Self {
        self.shift = Some(shift);
        self
    }
}

/// Facet that registers keymaps in the editor state.
pub static KEYMAP: LazyLock<Facet<Vec<KeyBinding>, Vec<KeyBinding>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.concat()));

/// Dispatches a key event through the registered keymaps in the state's `KEYMAP` facet.
pub fn dispatch_key(state: &EditorState, key_name: &str, shift: bool) -> Option<TransactionSpec> {
    let keymaps = state.facet(&KEYMAP);
    for binding in keymaps {
        if binding.key == key_name {
            if shift && binding.shift.is_some() {
                if let Some(spec) = (binding.shift.unwrap())(state) {
                    return Some(spec);
                }
            } else if let Some(spec) = (binding.run)(state) {
                return Some(spec);
            }
        }
    }
    None
}

// =========================================================================
// Character-level Cursor Movement (Grapheme Cluster Aware)
// =========================================================================

/// Move cursor one character/cluster to the left.
/// If selection is non-empty, collapses cursor to the lower bound (`from`).
pub fn cursor_char_left(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            if !r.is_empty() {
                SelectionRange::cursor(r.from())
            } else {
                let new_pos = find_cluster_break(&doc_str, r.head, false);
                SelectionRange::cursor(new_pos)
            }
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Move cursor one character/cluster to the right.
/// If selection is non-empty, collapses cursor to the upper bound (`to`).
pub fn cursor_char_right(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            if !r.is_empty() {
                SelectionRange::cursor(r.to())
            } else {
                let new_pos = find_cluster_break(&doc_str, r.head, true);
                SelectionRange::cursor(new_pos)
            }
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head one character/cluster to the left.
pub fn select_char_left(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let new_head = find_cluster_break(&doc_str, r.head, false);
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head one character/cluster to the right.
pub fn select_char_right(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let new_head = find_cluster_break(&doc_str, r.head, true);
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

// =========================================================================
// Word and Group Movement
// =========================================================================

/// Move cursor one word/group to the left.
pub fn cursor_group_left(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let pos = if !r.is_empty() { r.from() } else { r.head };
            let new_pos = find_word_break(&doc_str, pos, false);
            SelectionRange::cursor(new_pos)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Move cursor one word/group to the right.
pub fn cursor_group_right(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let pos = if !r.is_empty() { r.to() } else { r.head };
            let new_pos = find_word_break(&doc_str, pos, true);
            SelectionRange::cursor(new_pos)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head one word/group to the left.
pub fn select_group_left(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let new_head = find_word_break(&doc_str, r.head, false);
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head one word/group to the right.
pub fn select_group_right(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let new_head = find_word_break(&doc_str, r.head, true);
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Move cursor one camelCase or snake_case subword backward.
pub fn cursor_subword_backward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let pos = if !r.is_empty() { r.from() } else { r.head };
            let new_pos = find_subword_break(&doc_str, pos, false);
            SelectionRange::cursor(new_pos)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Move cursor one camelCase or snake_case subword forward.
pub fn cursor_subword_forward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let pos = if !r.is_empty() { r.to() } else { r.head };
            let new_pos = find_subword_break(&doc_str, pos, true);
            SelectionRange::cursor(new_pos)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head one subword backward.
pub fn select_subword_backward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let new_head = find_subword_break(&doc_str, r.head, false);
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head one subword forward.
pub fn select_subword_forward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let new_head = find_subword_break(&doc_str, r.head, true);
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

// =========================================================================
// Line and Document Boundary Navigation
// =========================================================================

/// Move cursor to the start of the current line.
pub fn cursor_line_start(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let line = state.doc().line_at(r.head);
            SelectionRange::cursor(line.from)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Move cursor to the end of the current line (before newline).
pub fn cursor_line_end(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let line = state.doc().line_at(r.head);
            SelectionRange::cursor(line.to)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head to the start of the current line.
pub fn select_line_start(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let line = state.doc().line_at(r.head);
            SelectionRange::new(r.anchor, line.from)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head to the end of the current line.
pub fn select_line_end(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let line = state.doc().line_at(r.head);
            SelectionRange::new(r.anchor, line.to)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Smart Home: jump to first non-whitespace character on line; if already there, jump to col 0.
pub fn cursor_line_boundary_backward(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let line = state.doc().line_at(r.head);
            let line_text = state.doc().slice_to_string(line.from..line.to);
            let first_non_space = line_text
                .char_indices()
                .find(|(_, c)| !c.is_whitespace())
                .map(|(idx, _)| line.from + line_text[..idx].chars().count())
                .unwrap_or(line.from);

            let new_pos = if r.head == first_non_space && first_non_space != line.from {
                line.from
            } else {
                first_non_space
            };
            SelectionRange::cursor(new_pos)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// End: jump to the end of the line.
pub fn cursor_line_boundary_forward(state: &EditorState) -> Option<TransactionSpec> {
    cursor_line_end(state)
}

/// Extend selection head to line boundary backward (Shift+Home).
pub fn select_line_boundary_backward(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let line = state.doc().line_at(r.head);
            let line_text = state.doc().slice_to_string(line.from..line.to);
            let first_non_space = line_text
                .char_indices()
                .find(|(_, c)| !c.is_whitespace())
                .map(|(idx, _)| line.from + line_text[..idx].chars().count())
                .unwrap_or(line.from);

            let new_head = if r.head == first_non_space && first_non_space != line.from {
                line.from
            } else {
                first_non_space
            };
            SelectionRange::new(r.anchor, new_head)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection head to line boundary forward (Shift+End).
pub fn select_line_boundary_forward(state: &EditorState) -> Option<TransactionSpec> {
    select_line_end(state)
}

/// Move cursor to the beginning of the document (Ctrl+Home).
pub fn cursor_doc_start(_state: &EditorState) -> Option<TransactionSpec> {
    Some(
        TransactionSpec::new().with_selection(EditorSelection::cursor(0)),
    )
}

/// Move cursor to the end of the document (Ctrl+End).
pub fn cursor_doc_end(state: &EditorState) -> Option<TransactionSpec> {
    Some(
        TransactionSpec::new().with_selection(EditorSelection::cursor(state.doc().len_chars())),
    )
}

/// Extend selection head to document start.
pub fn select_doc_start(state: &EditorState) -> Option<TransactionSpec> {
    let main = state.selection().main();
    Some(
        TransactionSpec::new().with_selection(EditorSelection::single(main.anchor, 0)),
    )
}

/// Extend selection head to document end.
pub fn select_doc_end(state: &EditorState) -> Option<TransactionSpec> {
    let main = state.selection().main();
    Some(
        TransactionSpec::new().with_selection(EditorSelection::single(
            main.anchor,
            state.doc().len_chars(),
        )),
    )
}

// =========================================================================
// Vertical Navigation (Line Up / Down with goal_column retention)
// =========================================================================

/// Move cursor one line up, preserving goal column.
pub fn cursor_line_up(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let curr_line_idx = state.doc().char_to_line(r.head);
            if curr_line_idx == 0 {
                return SelectionRange::cursor(0);
            }
            let curr_line = state.doc().line_by_index(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let prev_line = state.doc().line_by_index(curr_line_idx - 1);
            let target_char = prev_line.from + min(goal_col, prev_line.length);
            SelectionRange::cursor(target_char).with_goal_column(Some(goal_col))
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Move cursor one line down, preserving goal column.
pub fn cursor_line_down(state: &EditorState) -> Option<TransactionSpec> {
    let max_lines = state.doc().len_lines();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let curr_line_idx = state.doc().char_to_line(r.head);
            if curr_line_idx + 1 >= max_lines {
                let curr_line = state.doc().line_by_index(curr_line_idx);
                return SelectionRange::cursor(curr_line.to);
            }
            let curr_line = state.doc().line_by_index(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let next_line = state.doc().line_by_index(curr_line_idx + 1);
            let target_char = next_line.from + min(goal_col, next_line.length);
            SelectionRange::cursor(target_char).with_goal_column(Some(goal_col))
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection one line up.
pub fn select_line_up(state: &EditorState) -> Option<TransactionSpec> {
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let curr_line_idx = state.doc().char_to_line(r.head);
            if curr_line_idx == 0 {
                return SelectionRange::new(r.anchor, 0);
            }
            let curr_line = state.doc().line_by_index(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let prev_line = state.doc().line_by_index(curr_line_idx - 1);
            let target_char = prev_line.from + min(goal_col, prev_line.length);
            SelectionRange::new(r.anchor, target_char).with_goal_column(Some(goal_col))
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Extend selection one line down.
pub fn select_line_down(state: &EditorState) -> Option<TransactionSpec> {
    let max_lines = state.doc().len_lines();
    let ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            let curr_line_idx = state.doc().char_to_line(r.head);
            if curr_line_idx + 1 >= max_lines {
                let curr_line = state.doc().line_by_index(curr_line_idx);
                return SelectionRange::new(r.anchor, curr_line.to);
            }
            let curr_line = state.doc().line_by_index(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let next_line = state.doc().line_by_index(curr_line_idx + 1);
            let target_char = next_line.from + min(goal_col, next_line.length);
            SelectionRange::new(r.anchor, target_char).with_goal_column(Some(goal_col))
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

// =========================================================================
// Deletion Commands (Backspace / Delete / Group / Line)
// =========================================================================

/// Delete character backward (Backspace). Grapheme-cluster aware.
/// Deletes entire emoji, combining marks, or CRLF sequence if at cluster boundary.
pub fn delete_char_backward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let res = state.change_by_range(|r| {
        if !r.is_empty() {
            RangeResult::range(SelectionRange::cursor(r.from()))
                .with_changes(ChangeSpec::delete(r.from(), r.to()))
        } else if r.head > 0 {
            let prev = find_cluster_break(&doc_str, r.head, false);
            RangeResult::range(SelectionRange::cursor(prev))
                .with_changes(ChangeSpec::delete(prev, r.head))
        } else {
            RangeResult::range(*r)
        }
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

/// Delete character forward (Delete). Grapheme-cluster aware.
pub fn delete_char_forward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let doc_len = state.doc().len_chars();
    let res = state.change_by_range(|r| {
        if !r.is_empty() {
            RangeResult::range(SelectionRange::cursor(r.from()))
                .with_changes(ChangeSpec::delete(r.from(), r.to()))
        } else if r.head < doc_len {
            let next = find_cluster_break(&doc_str, r.head, true);
            RangeResult::range(SelectionRange::cursor(r.head))
                .with_changes(ChangeSpec::delete(r.head, next))
        } else {
            RangeResult::range(*r)
        }
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

fn delete_by_group(state: &EditorState, forward: bool) -> Option<TransactionSpec> {
    let doc = state.doc();
    let num_lines = doc.len_lines();
    let res = state.change_by_range(|range| {
        if !range.is_empty() {
            return RangeResult::range(SelectionRange::cursor(range.from()))
                .with_changes(ChangeSpec::delete(range.from(), range.to()));
        }
        let head = range.head;
        let line = doc.line_at(head);
        let line_text = doc.slice_to_string(line.from..line.to);
        let mut pos = head;
        let mut cat: Option<CharCategory> = None;

        loop {
            let line_limit = if forward { line.to } else { line.from };
            if pos == line_limit {
                if pos == head {
                    let is_last_line = line.number == num_lines;
                    let is_first_line = line.number == 1;
                    if forward && !is_last_line {
                        pos += 1;
                    } else if !forward && !is_first_line {
                        pos -= 1;
                    }
                }
                break;
            }

            let next_rel = find_cluster_break(&line_text, pos - line.from, forward);
            let next = next_rel + line.from;
            let (start_rel, end_rel) = if pos < next {
                (pos - line.from, next - line.from)
            } else {
                (next - line.from, pos - line.from)
            };
            let cluster: String = line_text.chars().skip(start_rel).take(end_rel - start_rel).collect();
            let first_char = cluster.chars().next().unwrap_or(' ');
            let next_cat = categorize_char(first_char);

            if let Some(c) = cat {
                if next_cat != c {
                    break;
                }
            }
            if cluster != " " || pos != head {
                cat = Some(next_cat);
            }
            pos = next;
        }

        if pos == head {
            RangeResult::range(*range)
        } else {
            let from = std::cmp::min(head, pos);
            let to = std::cmp::max(head, pos);
            RangeResult::range(SelectionRange::cursor(from))
                .with_changes(ChangeSpec::delete(from, to))
        }
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

/// Delete word/group backward (Ctrl+Backspace).
pub fn delete_group_backward(state: &EditorState) -> Option<TransactionSpec> {
    delete_by_group(state, false)
}

/// Delete word/group forward (Ctrl+Delete).
pub fn delete_group_forward(state: &EditorState) -> Option<TransactionSpec> {
    delete_by_group(state, true)
}

fn delete_by(
    state: &EditorState,
    by: impl Fn(&EditorState, &SelectionRange) -> usize,
) -> Option<TransactionSpec> {
    let res = state.change_by_range(|range| {
        let mut from = range.from();
        let mut to = range.to();
        if from == to {
            let towards = by(state, range);
            from = std::cmp::min(from, towards);
            to = std::cmp::max(to, towards);
        }
        if from == to {
            RangeResult::range(*range)
        } else {
            RangeResult::range(SelectionRange::cursor(from))
                .with_changes(ChangeSpec::delete(from, to))
        }
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

/// Delete from cursor to the start of the current line.
/// If cursor is at line start, deletes preceding line break.
pub fn delete_to_line_start(state: &EditorState) -> Option<TransactionSpec> {
    delete_by(state, |s, range| {
        let line = s.doc().line_at(range.head);
        if range.head > line.from {
            line.from
        } else {
            range.head.saturating_sub(1)
        }
    })
}

/// Delete from cursor to the end of the current line.
/// If cursor is at line end, deletes following line break.
pub fn delete_to_line_end(state: &EditorState) -> Option<TransactionSpec> {
    delete_by(state, |s, range| {
        let line = s.doc().line_at(range.head);
        if range.head < line.to {
            line.to
        } else {
            std::cmp::min(s.doc().len_chars(), range.head + 1)
        }
    })
}

/// Delete whole line including its newline delimiter (Ctrl+Shift+K).
pub fn delete_line(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let mut changes = Vec::new();
    let mut new_ranges = Vec::new();

    for range in state.selection().ranges() {
        let start_line = doc.char_to_line(range.from());
        let end_line = doc.char_to_line(range.to());

        let from = doc.line_by_index(start_line).from;
        let to = if end_line + 1 < doc.len_lines() {
            doc.line_by_index(end_line + 1).from
        } else {
            doc.len_chars()
        };

        changes.push(ChangeSpec::delete(from, to));
        new_ranges.push(SelectionRange::cursor(from));
    }

    Some(
        TransactionSpec::new()
            .with_changes(changes)
            .with_selection(EditorSelection::create(
                new_ranges,
                state.selection().main_index(),
            )),
    )
}

// =========================================================================
// Line Manipulation (Move Line Up/Down, Duplicate Line)
// =========================================================================

struct SelectedBlock {
    from: usize,
    to: usize,
    ranges: Vec<SelectionRange>,
}

fn selected_line_blocks(state: &EditorState) -> Vec<SelectedBlock> {
    let mut blocks: Vec<SelectedBlock> = Vec::new();
    let mut upto: isize = -1;
    let doc = state.doc();

    for range in state.selection().ranges() {
        let start_line = doc.line_at(range.from());
        let mut end_line = doc.line_at(range.to());
        if !range.is_empty() && range.to() == end_line.from && range.to() > 0 {
            end_line = doc.line_at(range.to() - 1);
        }
        let start_num = start_line.number as isize;
        let end_num = end_line.number as isize;

        if upto >= start_num && !blocks.is_empty() {
            let prev = blocks.last_mut().unwrap();
            prev.to = end_line.to;
            prev.ranges.push(*range);
        } else {
            blocks.push(SelectedBlock {
                from: start_line.from,
                to: end_line.to,
                ranges: vec![*range],
            });
        }
        upto = end_num + 1;
    }
    blocks
}

fn move_line(state: &EditorState, forward: bool) -> Option<TransactionSpec> {
    let doc = state.doc();
    let doc_len = doc.len_chars();
    let mut changes: Vec<ChangeSpec> = Vec::new();
    let mut ranges: Vec<SelectionRange> = Vec::new();

    for block in selected_line_blocks(state) {
        if forward {
            if block.to >= doc_len {
                continue;
            }
            let next_line = doc.line_at(block.to + 1);
            let size = next_line.length + 1;
            let next_line_text = doc.slice_to_string(next_line.from..next_line.to);

            changes.push(ChangeSpec::delete(block.to, next_line.to));
            changes.push(ChangeSpec::insert(block.from, format!("{}\n", next_line_text)));

            for r in block.ranges {
                let new_anchor = std::cmp::min(doc_len, r.anchor + size);
                let new_head = std::cmp::min(doc_len, r.head + size);
                ranges.push(SelectionRange::new(new_anchor, new_head));
            }
        } else {
            if block.from == 0 {
                continue;
            }
            let next_line = doc.line_at(block.from - 1);
            let size = next_line.length + 1;
            let next_line_text = doc.slice_to_string(next_line.from..next_line.to);

            changes.push(ChangeSpec::delete(next_line.from, block.from));
            changes.push(ChangeSpec::insert(block.to, format!("\n{}", next_line_text)));

            for r in block.ranges {
                let new_anchor = r.anchor.saturating_sub(size);
                let new_head = r.head.saturating_sub(size);
                ranges.push(SelectionRange::new(new_anchor, new_head));
            }
        }
    }

    if changes.is_empty() {
        None
    } else {
        changes.sort_by_key(|c| c.from);
        Some(
            TransactionSpec::new()
                .with_changes(changes)
                .with_selection(EditorSelection::create(
                    ranges,
                    state.selection().main_index(),
                )),
        )
    }
}

/// Move selected lines up one line (Alt+Up).
pub fn move_line_up(state: &EditorState) -> Option<TransactionSpec> {
    move_line(state, false)
}

/// Move selected lines down one line (Alt+Down).
pub fn move_line_down(state: &EditorState) -> Option<TransactionSpec> {
    move_line(state, true)
}

fn copy_line(state: &EditorState, forward: bool) -> Option<TransactionSpec> {
    let doc = state.doc();
    let mut changes: Vec<ChangeSpec> = Vec::new();

    for block in selected_line_blocks(state) {
        let block_text = doc.slice_to_string(block.from..block.to);
        if forward {
            changes.push(ChangeSpec::insert(block.from, format!("{}\n", block_text)));
        } else {
            changes.push(ChangeSpec::insert(block.to, format!("\n{}", block_text)));
        }
    }
    if changes.is_empty() {
        return None;
    }

    changes.sort_by_key(|c| c.from);
    let cs = ferrum_state::change::ChangeSet::of_specs(&changes, doc.len_chars());
    let assoc = if forward { 1 } else { -1 };
    let new_selection = state.selection().map(cs.desc(), assoc);

    Some(
        TransactionSpec::new()
            .with_changes(changes)
            .with_selection(new_selection),
    )
}

/// Duplicate current line above (Shift+Alt+Up).
pub fn copy_line_up(state: &EditorState) -> Option<TransactionSpec> {
    copy_line(state, false)
}

/// Duplicate current line below (Shift+Alt+Down).
pub fn copy_line_down(state: &EditorState) -> Option<TransactionSpec> {
    copy_line(state, true)
}

/// Delete all whitespace directly before a line end from the document.
pub fn delete_trailing_whitespace(state: &EditorState) -> Option<TransactionSpec> {
    let mut changes: Vec<ChangeSpec> = Vec::new();
    let num_lines = state.doc().len_lines();
    for line_idx in 0..num_lines {
        let line = state.doc().line_by_index(line_idx);
        let line_text = state.doc().slice_to_string(line.from..line.to);
        let trimmed = line_text.trim_end_matches(|c: char| c == ' ' || c == '\t');
        let trailing_len = line_text.chars().count() - trimmed.chars().count();
        if trailing_len > 0 {
            let from = line.to - trailing_len;
            changes.push(ChangeSpec::delete(from, line.to));
        }
    }
    if changes.is_empty() {
        None
    } else {
        Some(TransactionSpec::new().with_changes(changes))
    }
}

/// Replace each selection range with a line break, leaving the cursor
/// on the line before the break.
pub fn split_line(state: &EditorState) -> Option<TransactionSpec> {
    let res = state.change_by_range(|range| {
        RangeResult::range(SelectionRange::cursor(range.from()))
            .with_changes(ChangeSpec::replace(range.from(), range.to(), "\n"))
    });
    Some(res.into_spec())
}

/// Replace the selection with a newline and the same amount of
/// indentation as the line above.
pub fn insert_newline_keep_indent(state: &EditorState) -> Option<TransactionSpec> {
    let res = state.change_by_range(|range| {
        let line = state.doc().line_at(range.from());
        let line_text = state.doc().slice_to_string(line.from..line.to);
        let indent: String = line_text
            .chars()
            .take_while(|&c| c == ' ' || c == '\t')
            .collect();
        let insert = format!("\n{}", indent);
        let new_pos = range.from() + 1 + indent.chars().count();
        RangeResult::range(SelectionRange::cursor(new_pos))
            .with_changes(ChangeSpec::replace(range.from(), range.to(), insert))
    });
    Some(res.into_spec())
}

/// Flip the characters before and after the cursor(s).
pub fn transpose_chars(state: &EditorState) -> Option<TransactionSpec> {
    let doc_len = state.doc().len_chars();
    let res = state.change_by_range(|range| {
        if !range.is_empty() || range.from() == 0 || range.from() == doc_len {
            return RangeResult::range(*range);
        }
        let pos = range.from();
        let line = state.doc().line_at(pos);
        let line_text = state.doc().slice_to_string(line.from..line.to);

        let from = if pos == line.from {
            pos - 1
        } else {
            find_cluster_break(&line_text, pos - line.from, false) + line.from
        };
        let to = if pos == line.to {
            pos + 1
        } else {
            find_cluster_break(&line_text, pos - line.from, true) + line.from
        };

        let insert = format!(
            "{}{}",
            state.doc().slice_to_string(pos..to),
            state.doc().slice_to_string(from..pos)
        );
        RangeResult::range(SelectionRange::cursor(to))
            .with_changes(ChangeSpec::replace(from, to, insert))
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

// =========================================================================
// Indentation and Formatting
// =========================================================================

fn change_by_selected_line<F>(state: &EditorState, mut f: F) -> Option<TransactionSpec>
where
    F: FnMut(&ferrum_state::text::Line, &mut Vec<ChangeSpec>, &SelectionRange),
{
    let mut at_line = -1isize;
    let res = state.change_by_range(|range| {
        let mut changes = Vec::new();
        let mut pos = range.from();
        while pos <= range.to() {
            let line = state.doc().line_at(pos);
            let line_num = line.number as isize;
            if line_num > at_line && (range.is_empty() || range.to() > line.from) {
                f(&line, &mut changes, range);
                at_line = line_num;
            }
            pos = line.to + 1;
        }
        let cs = ferrum_state::change::ChangeSet::of_specs(&changes, state.doc().len_chars());
        let anchor = cs.map_pos(range.anchor, 1);
        let head = cs.map_pos(range.head, 1);
        let new_range = SelectionRange::new(anchor, head);
        RangeResult::range(new_range).with_changes(changes)
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

/// Add a unit of indentation to all selected lines.
pub fn indent_more(state: &EditorState) -> Option<TransactionSpec> {
    if state.is_read_only() {
        return None;
    }
    let unit = state.facet(&INDENT_UNIT).clone();
    change_by_selected_line(state, |line, changes, _| {
        changes.push(ChangeSpec::insert(line.from, unit.clone()));
    })
}

/// Remove a unit of indentation from all selected lines.
pub fn indent_less(state: &EditorState) -> Option<TransactionSpec> {
    if state.is_read_only() {
        return None;
    }
    let tab_size = *state.facet(EditorState::tab_size());
    let unit_len = get_indent_unit(state);

    change_by_selected_line(state, |line, changes, _| {
        let text = &line.text;
        let space_count = text.chars().take_while(|c| c.is_whitespace()).count();
        if space_count == 0 {
            return;
        }
        let space: String = text.chars().take(space_count).collect();
        let col = count_column(&space, tab_size, None);
        let target = col.saturating_sub(unit_len);
        let norm = indent_string(state, target);

        let space_chars: Vec<char> = space.chars().collect();
        let norm_chars: Vec<char> = norm.chars().collect();
        let mut keep = 0;
        while keep < space_chars.len() && keep < norm_chars.len() && space_chars[keep] == norm_chars[keep] {
            keep += 1;
        }

        let replace_from = line.from + keep;
        let replace_to = line.from + space_chars.len();
        let insert_slice: String = norm_chars.into_iter().skip(keep).collect();
        changes.push(ChangeSpec::replace(replace_from, replace_to, insert_slice));
    })
}

/// Auto-indent the selected lines based on syntax indentation services.
pub fn indent_selection(state: &EditorState) -> Option<TransactionSpec> {
    if state.is_read_only() {
        return None;
    }
    let updated = Arc::new(std::sync::Mutex::new(std::collections::HashMap::<usize, usize>::new()));
    let updated_clone = updated.clone();
    let override_fn: Arc<dyn Fn(usize) -> Option<isize> + Send + Sync> = Arc::new(move |pos: usize| {
        let map = updated_clone.lock().unwrap();
        map.get(&pos).map(|&v| v as isize)
    });

    let context = IndentContext::with_options(
        state,
        IndentContextOptions {
            override_indentation: Some(override_fn),
            simulate_break: None,
            simulate_double_break: false,
        },
    );

    let updated_for_edit = updated.clone();
    change_by_selected_line(state, |line, changes, range| {
        let indent = match get_indentation_with_context(&context, line.from) {
            Some(i) => {
                if !line.text.chars().any(|c| !c.is_whitespace()) {
                    0
                } else {
                    i
                }
            }
            None => return,
        };

        let cur_space_count = line.text.chars().take_while(|c| c.is_whitespace()).count();
        let cur_space: String = line.text.chars().take(cur_space_count).collect();
        let norm = indent_string(state, indent);

        if cur_space != norm || range.from() < line.from + cur_space_count {
            updated_for_edit.lock().unwrap().insert(line.from, indent);
            changes.push(ChangeSpec::replace(
                line.from,
                line.from + cur_space_count,
                norm,
            ));
        }
    })
}

/// Insert a tab character at the cursor or, if text is selected, indent the selection.
pub fn insert_tab(state: &EditorState) -> Option<TransactionSpec> {
    if state.selection().ranges().iter().any(|r| !r.is_empty()) {
        indent_more(state)
    } else {
        Some(state.replace_selection("\t"))
    }
}

fn is_between_brackets(state: &EditorState, pos: usize) -> Option<(usize, usize)> {
    if pos == 0 || pos >= state.doc().len_chars() {
        return None;
    }
    let doc = state.doc();
    let line = doc.line_at(pos);
    let rel_pos = pos.saturating_sub(line.from);
    let line_chars: Vec<char> = line.text.chars().collect();

    // Check adjacent brackets first: (), [], {}
    if rel_pos > 0 && rel_pos < line_chars.len() {
        let prev = line_chars[rel_pos - 1];
        let next = line_chars[rel_pos];
        if (prev == '(' && next == ')')
            || (prev == '[' && next == ']')
            || (prev == '{' && next == '}')
        {
            return Some((pos, pos));
        }
    }

    // Check bracket pair with only whitespace between them on the same line
    let mut open_idx = None;
    let mut i = rel_pos;
    while i > 0 {
        i -= 1;
        let c = line_chars[i];
        if c == '(' || c == '[' || c == '{' {
            open_idx = Some((i, c));
            break;
        } else if !c.is_whitespace() {
            break;
        }
    }

    let mut close_idx = None;
    let mut j = rel_pos;
    while j < line_chars.len() {
        let c = line_chars[j];
        if c == ')' || c == ']' || c == '}' {
            close_idx = Some((j, c));
            break;
        } else if !c.is_whitespace() {
            break;
        }
        j += 1;
    }

    if let (Some((open_i, open_c)), Some((close_j, close_c))) = (open_idx, close_idx) {
        let matches = match (open_c, close_c) {
            ('(', ')') => true,
            ('[', ']') => true,
            ('{', '}') => true,
            _ => false,
        };
        if matches {
            let between = &line_chars[open_i + 1..close_j];
            if between.iter().all(|c| c.is_whitespace()) {
                return Some((line.from + open_i + 1, line.from + close_j));
            }
        }
    }

    None
}

/// Replace the selection with a newline and indent the newly created line(s).
/// If the cursor is between matching brackets, an additional newline is inserted.
pub fn insert_newline_and_indent(state: &EditorState) -> Option<TransactionSpec> {
    if state.is_read_only() {
        return None;
    }
    let tab_size = *state.facet(EditorState::tab_size());
    let res = state.change_by_range(|range| {
        let mut from = range.from();
        let mut to = range.to();
        let line = state.doc().line_at(from);
        let explode = if range.is_empty() {
            is_between_brackets(state, from)
        } else {
            None
        };

        let cx = IndentContext::with_options(
            state,
            IndentContextOptions {
                override_indentation: None,
                simulate_break: Some(from),
                simulate_double_break: explode.is_some(),
            },
        );

        let indent = get_indentation_with_context(&cx, from).unwrap_or_else(|| {
            let leading_space: String = line.text.chars().take_while(|c| c.is_whitespace()).collect();
            count_column(&leading_space, tab_size, None)
        });

        // Skip whitespace after cursor up to line end
        let rel_to = to.saturating_sub(line.from);
        let line_chars: Vec<char> = line.text.chars().collect();
        let mut skipped_to = to;
        let mut idx = rel_to;
        while idx < line_chars.len() && line_chars[idx].is_whitespace() {
            skipped_to += 1;
            idx += 1;
        }
        to = skipped_to;

        if let Some((ex_from, ex_to)) = explode {
            from = ex_from;
            to = ex_to;
        } else if from > line.from && from < line.from + 100 {
            let rel_from = from - line.from;
            if line_chars.iter().take(rel_from).all(|c| c.is_whitespace()) {
                from = line.from;
            }
        }

        let mid_indent = indent_string(state, indent);
        let (insert_str, cursor_pos) = if explode.is_some() {
            let base_indent = indent_string(state, cx.line_indent(line.from, -1));
            let s = format!("\n{}\n{}", mid_indent, base_indent);
            let pos = from + 1 + mid_indent.chars().count();
            (s, pos)
        } else {
            let s = format!("\n{}", mid_indent);
            let pos = from + 1 + mid_indent.chars().count();
            (s, pos)
        };

        RangeResult::range(SelectionRange::cursor(cursor_pos))
            .with_changes(ChangeSpec::replace(from, to, insert_str))
    });

    Some(res.into_spec())
}

/// Insert a blank line below without moving cursor (Ctrl+Enter).
pub fn insert_blank_line(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let main_range = state.selection().main();
    let line = doc.line_at(main_range.head);

    Some(
        TransactionSpec::new()
            .with_change(ChangeSpec::insert(line.to, "\n"))
            .with_selection(state.selection().clone()),
    )
}

// =========================================================================
// General Selection & Editing Helpers
// =========================================================================

/// Select all text in the document (Ctrl+A).
pub fn select_all(state: &EditorState) -> Option<TransactionSpec> {
    Some(
        TransactionSpec::new().with_selection(EditorSelection::single(0, state.doc().len_chars())),
    )
}

/// Expand the selection to cover entire lines.
pub fn select_line(state: &EditorState) -> Option<TransactionSpec> {
    let doc_len = state.doc().len_chars();
    let ranges: Vec<SelectionRange> = selected_line_blocks(state)
        .into_iter()
        .map(|block| {
            let to = std::cmp::min(block.to + 1, doc_len);
            SelectionRange::new(block.from, to)
        })
        .collect();

    Some(
        TransactionSpec::new().with_selection(EditorSelection::create(
            ranges,
            state.selection().main_index(),
        )),
    )
}

/// Collapse multi-cursor selection into just the primary cursor (Escape).
pub fn simplify_selection(state: &EditorState) -> Option<TransactionSpec> {
    if state.selection().is_single() && state.selection().main().is_empty() {
        return None;
    }
    let main = state.selection().main();
    Some(
        TransactionSpec::new().with_selection(EditorSelection::cursor(main.head)),
    )
}

/// Insert text at each selection range, replacing selected text.
pub fn insert_text(state: &EditorState, text: &str) -> Option<TransactionSpec> {
    let text_len = text.chars().count();
    let res = state.change_by_range(|r| {
        RangeResult::range(SelectionRange::cursor(r.from() + text_len))
            .with_changes(ChangeSpec::replace(r.from(), r.to(), text))
    });

    Some(res.into_spec())
}

// =========================================================================
// Standard Keymaps
// =========================================================================

/// Returns the standard CodeMirror 6 keymap.
pub fn standard_keymap() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("ArrowLeft", cursor_char_left).with_shift(select_char_left),
        KeyBinding::new("Mod-ArrowLeft", cursor_group_left).with_shift(select_group_left),
        KeyBinding::new("ArrowRight", cursor_char_right).with_shift(select_char_right),
        KeyBinding::new("Mod-ArrowRight", cursor_group_right).with_shift(select_group_right),
        KeyBinding::new("ArrowUp", cursor_line_up).with_shift(select_line_up),
        KeyBinding::new("ArrowDown", cursor_line_down).with_shift(select_line_down),
        KeyBinding::new("Home", cursor_line_boundary_backward).with_shift(select_line_boundary_backward),
        KeyBinding::new("End", cursor_line_boundary_forward).with_shift(select_line_boundary_forward),
        KeyBinding::new("Mod-Home", cursor_doc_start).with_shift(select_doc_start),
        KeyBinding::new("Mod-End", cursor_doc_end).with_shift(select_doc_end),
        KeyBinding::new("Enter", insert_newline_and_indent),
        KeyBinding::new("Mod-a", select_all),
        KeyBinding::new("Backspace", delete_char_backward),
        KeyBinding::new("Delete", delete_char_forward),
        KeyBinding::new("Mod-Backspace", delete_group_backward),
        KeyBinding::new("Mod-Delete", delete_group_forward),
    ]
}

/// Returns the default keymap including indentation and line commands.
pub fn default_keymap() -> Vec<KeyBinding> {
    let mut map = vec![
        KeyBinding::new("Alt-ArrowUp", move_line_up),
        KeyBinding::new("Shift-Alt-ArrowUp", copy_line_up),
        KeyBinding::new("Alt-ArrowDown", move_line_down),
        KeyBinding::new("Shift-Alt-ArrowDown", copy_line_down),
        KeyBinding::new("Escape", simplify_selection),
        KeyBinding::new("Mod-Enter", insert_blank_line),
        KeyBinding::new("Alt-l", select_line),
        KeyBinding::new("Mod-[", indent_less),
        KeyBinding::new("Mod-]", indent_more),
        KeyBinding::new("Shift-Mod-k", delete_line),
        KeyBinding::new("Tab", indent_more).with_shift(indent_less),
    ];
    map.extend(standard_keymap());
    map
}
