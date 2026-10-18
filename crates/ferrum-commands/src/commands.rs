use ferrum_state::change::ChangeSpec;
use ferrum_state::char::{
    find_cluster_break, find_subword_break, find_word_break,
};
use ferrum_state::facet::Facet;
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::{EditorState, RangeResult};
use ferrum_state::transaction::TransactionSpec;
use std::cmp::min;
use std::sync::LazyLock;

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
            let curr_line = state.doc().line(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let prev_line = state.doc().line(curr_line_idx - 1);
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
                let curr_line = state.doc().line(curr_line_idx);
                return SelectionRange::cursor(curr_line.to);
            }
            let curr_line = state.doc().line(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let next_line = state.doc().line(curr_line_idx + 1);
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
            let curr_line = state.doc().line(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let prev_line = state.doc().line(curr_line_idx - 1);
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
                let curr_line = state.doc().line(curr_line_idx);
                return SelectionRange::new(r.anchor, curr_line.to);
            }
            let curr_line = state.doc().line(curr_line_idx);
            let goal_col = r.goal_column.unwrap_or(r.head - curr_line.from);

            let next_line = state.doc().line(curr_line_idx + 1);
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

/// Delete word/group backward (Ctrl+Backspace).
pub fn delete_group_backward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let res = state.change_by_range(|r| {
        if !r.is_empty() {
            RangeResult::range(SelectionRange::cursor(r.from()))
                .with_changes(ChangeSpec::delete(r.from(), r.to()))
        } else if r.head > 0 {
            let prev = find_word_break(&doc_str, r.head, false);
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

/// Delete word/group forward (Ctrl+Delete).
pub fn delete_group_forward(state: &EditorState) -> Option<TransactionSpec> {
    let doc_str = state.doc().to_string();
    let doc_len = state.doc().len_chars();
    let res = state.change_by_range(|r| {
        if !r.is_empty() {
            RangeResult::range(SelectionRange::cursor(r.from()))
                .with_changes(ChangeSpec::delete(r.from(), r.to()))
        } else if r.head < doc_len {
            let next = find_word_break(&doc_str, r.head, true);
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

/// Delete from cursor to the start of the current line.
pub fn delete_to_line_start(state: &EditorState) -> Option<TransactionSpec> {
    let res = state.change_by_range(|r| {
        let line = state.doc().line_at(r.head);
        if r.head > line.from {
            RangeResult::range(SelectionRange::cursor(line.from))
                .with_changes(ChangeSpec::delete(line.from, r.head))
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

/// Delete from cursor to the end of the current line.
pub fn delete_to_line_end(state: &EditorState) -> Option<TransactionSpec> {
    let res = state.change_by_range(|r| {
        let line = state.doc().line_at(r.head);
        if r.head < line.to {
            RangeResult::range(SelectionRange::cursor(r.head))
                .with_changes(ChangeSpec::delete(r.head, line.to))
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

/// Delete whole line including its newline delimiter (Ctrl+Shift+K).
pub fn delete_line(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let mut changes = Vec::new();
    let mut new_ranges = Vec::new();

    for range in state.selection().ranges() {
        let start_line = doc.char_to_line(range.from());
        let end_line = doc.char_to_line(range.to());

        let from = doc.line(start_line).from;
        let to = if end_line + 1 < doc.len_lines() {
            doc.line(end_line + 1).from
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

/// Move selected lines up one line (Alt+Up).
pub fn move_line_up(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let main_range = state.selection().main();
    let start_line_idx = doc.char_to_line(main_range.from());
    if start_line_idx == 0 {
        return None; // Already at the top line
    }
    let end_line_idx = doc.char_to_line(main_range.to());

    let prev_line = doc.line(start_line_idx - 1);
    let curr_end_line = doc.line(end_line_idx);

    // Text of previous line
    let prev_text = doc.slice_to_string(prev_line.from..prev_line.to);
    // Text of current line range
    let curr_text = doc.slice_to_string(doc.line(start_line_idx).from..curr_end_line.to);

    let replacement = format!("{}\n{}", curr_text, prev_text);
    let from = prev_line.from;
    let to = curr_end_line.to;

    let shift = (prev_line.length + 1) as isize;
    let new_anchor = (main_range.anchor as isize - shift).max(0) as usize;
    let new_head = (main_range.head as isize - shift).max(0) as usize;

    Some(
        TransactionSpec::new()
            .with_change(ChangeSpec::replace(from, to, replacement))
            .with_selection(EditorSelection::single(new_anchor, new_head)),
    )
}

/// Move selected lines down one line (Alt+Down).
pub fn move_line_down(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let main_range = state.selection().main();
    let start_line_idx = doc.char_to_line(main_range.from());
    let end_line_idx = doc.char_to_line(main_range.to());
    if end_line_idx + 1 >= doc.len_lines() {
        return None; // Already at the bottom line
    }

    let next_line = doc.line(end_line_idx + 1);
    let curr_start_line = doc.line(start_line_idx);
    let curr_end_line = doc.line(end_line_idx);

    let curr_text = doc.slice_to_string(curr_start_line.from..curr_end_line.to);
    let next_text = doc.slice_to_string(next_line.from..next_line.to);

    let replacement = format!("{}\n{}", next_text, curr_text);
    let from = curr_start_line.from;
    let to = next_line.to;

    let shift = (next_line.length + 1) as usize;
    let new_anchor = main_range.anchor + shift;
    let new_head = main_range.head + shift;

    Some(
        TransactionSpec::new()
            .with_change(ChangeSpec::replace(from, to, replacement))
            .with_selection(EditorSelection::single(new_anchor, new_head)),
    )
}

/// Duplicate current line above (Shift+Alt+Up).
pub fn copy_line_up(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let main_range = state.selection().main();
    let start_line = doc.line_at(main_range.from());
    let end_line = doc.line_at(main_range.to());

    let text = doc.slice_to_string(start_line.from..end_line.to);
    let insert_text = format!("{}\n", text);

    Some(
        TransactionSpec::new()
            .with_change(ChangeSpec::insert(start_line.from, insert_text))
            .with_selection(state.selection().clone()),
    )
}

/// Duplicate current line below (Shift+Alt+Down).
pub fn copy_line_down(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let main_range = state.selection().main();
    let start_line = doc.line_at(main_range.from());
    let end_line = doc.line_at(main_range.to());

    let text = doc.slice_to_string(start_line.from..end_line.to);
    let insert_text = format!("\n{}", text);

    let shift = text.chars().count() + 1;
    let new_anchor = main_range.anchor + shift;
    let new_head = main_range.head + shift;

    Some(
        TransactionSpec::new()
            .with_change(ChangeSpec::insert(end_line.to, insert_text))
            .with_selection(EditorSelection::single(new_anchor, new_head)),
    )
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
        let new_range = if range.is_inverted() {
            SelectionRange::new(anchor.max(head), anchor.min(head))
        } else {
            SelectionRange::new(anchor.min(head), anchor.max(head))
        };
        RangeResult::range(new_range).with_changes(changes)
    });

    if res.changes.is_empty() {
        None
    } else {
        Some(res.into_spec())
    }
}

/// Indent selected lines by 4 spaces (Tab / Ctrl+]).
pub fn indent_more(state: &EditorState) -> Option<TransactionSpec> {
    change_by_selected_line(state, |line, changes, _| {
        changes.push(ChangeSpec::insert(line.from, "    "));
    })
}

/// Un-indent selected lines by up to 4 spaces (Shift+Tab / Ctrl+[).
pub fn indent_less(state: &EditorState) -> Option<TransactionSpec> {
    change_by_selected_line(state, |line, changes, _| {
        let text = state.doc().slice_to_string(line.from..line.to);
        let space_count = text.chars().take(4).take_while(|c| *c == ' ').count();
        if space_count > 0 {
            changes.push(ChangeSpec::delete(line.from, line.from + space_count));
        } else if text.starts_with('\t') {
            changes.push(ChangeSpec::delete(line.from, line.from + 1));
        }
    })
}

/// Insert a newline preserving current line indentation (Enter).
pub fn insert_newline_and_indent(state: &EditorState) -> Option<TransactionSpec> {
    let doc = state.doc();
    let res = state.change_by_range(|r| {
        let line = doc.line_at(r.head);
        let line_text = doc.slice_to_string(line.from..line.to);
        let indent: String = line_text.chars().take_while(|c| c.is_whitespace()).collect();
        let insert_str = format!("\n{}", indent);
        let insert_len = insert_str.chars().count();

        RangeResult::range(SelectionRange::cursor(r.from() + insert_len))
            .with_changes(ChangeSpec::replace(r.from(), r.to(), insert_str))
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

/// Select the entire line under the cursor (Alt+L).
pub fn select_line(state: &EditorState) -> Option<TransactionSpec> {
    let main = state.selection().main();
    let line = state.doc().line_at(main.head);
    Some(
        TransactionSpec::new().with_selection(EditorSelection::single(line.from, line.to)),
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
