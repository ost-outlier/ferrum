//! `ferrum-commands` - Text editing commands and keymaps for the Ferrum editor engine.

pub mod commands;
pub mod history;

pub use commands::{
    copy_line_down, copy_line_up, cursor_char_left, cursor_char_right, cursor_doc_end,
    cursor_doc_start, cursor_group_left, cursor_group_right, cursor_line_boundary_backward,
    cursor_line_boundary_forward, cursor_line_down, cursor_line_end, cursor_line_start,
    cursor_line_up, cursor_subword_backward, cursor_subword_forward, default_keymap,
    delete_char_backward, delete_char_forward, delete_group_backward, delete_group_forward,
    delete_line, delete_to_line_end, delete_to_line_start, delete_trailing_whitespace, dispatch_key,
    indent_less, indent_more, insert_blank_line, insert_newline_and_indent,
    insert_newline_keep_indent, insert_text, move_line_down, move_line_up, select_all,
    select_char_left, select_char_right, select_doc_end, select_doc_start, select_group_left,
    select_group_right, select_line, select_line_boundary_backward, select_line_boundary_forward,
    select_line_down, select_line_end, select_line_start, select_line_up, select_subword_backward,
    select_subword_forward, simplify_selection, split_line, standard_keymap, transpose_chars,
    KeyBinding, StateCommand, KEYMAP,
};

pub use history::{
    history, redo, redo_depth, redo_selection, undo, undo_depth, undo_selection, BranchName,
    HistEvent, HistoryConfig, HistoryState, HISTORY_CONFIG, HISTORY_FIELD,
};

#[cfg(test)]
mod tests;
