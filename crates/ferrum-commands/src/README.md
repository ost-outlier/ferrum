# ferrum-commands / src — API Reference

This crate exports a complete collection of pure editing commands, keybindings, transactional undo/redo history, and line/block commenting for the Ferrum editor engine. Adapted from and with 1:1 behavioral parity to [`@codemirror/commands`](https://code.haverbeke.berlin/codemirror/commands).

All state commands are pure functions with the signature:
```rust
pub type StateCommand = fn(&EditorState) -> Option<TransactionSpec>;
```

---

## 1. Keymaps & Key Dispatch

- `standard_keymap() -> Vec<KeyBinding>`: Core navigation, multi-cursor, and character deletion keybindings.
- `default_keymap() -> Vec<KeyBinding>`: Extends `standard_keymap` with line movement, indentation, line duplication, and history bindings.
- `history_keymap() -> Vec<KeyBinding>`: Keybindings for undo (`Mod-z`) and redo (`Mod-y` / `Shift-Mod-z`).
- `KEYMAP: LazyLock<Facet<Vec<KeyBinding>, Vec<KeyBinding>>>`: Facet for registering keymaps in the editor state.
- `dispatch_key(state: &EditorState, key_name: &str, shift: bool) -> Option<TransactionSpec>`: Dispatches a key event through active keymap facets.

---

## 2. Selection & Cursor Navigation

### Selection Utilities
- `simplify_selection(state: &EditorState) -> Option<TransactionSpec>`: Collapses multi-cursor selections into the primary cursor.
- `select_all(state: &EditorState) -> Option<TransactionSpec>`: Expands selection to cover the entire document (`0..doc.len_chars()`).

### By Character (Unicode Grapheme Cluster Aware)
- `cursor_char_left(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one grapheme cluster backward.
- `select_char_left(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one grapheme cluster backward.
- `cursor_char_right(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one grapheme cluster forward.
- `select_char_right(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one grapheme cluster forward.

### By Group & Subword
- `cursor_group_left(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one word/group backward.
- `select_group_left(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one word/group backward.
- `cursor_group_right(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one word/group forward.
- `select_group_right(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one word/group forward.
- `cursor_subword_backward(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one camelCase or snake_case subword backward.
- `select_subword_backward(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one subword backward.
- `cursor_subword_forward(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one subword forward.
- `select_subword_forward(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one subword forward.

### Vertical Motion
- `cursor_line_up(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one line up, preserving `goal_column`.
- `select_line_up(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one line up, preserving `goal_column`.
- `cursor_line_down(state: &EditorState) -> Option<TransactionSpec>`: Move cursor one line down, preserving `goal_column`.
- `select_line_down(state: &EditorState) -> Option<TransactionSpec>`: Extend selection one line down, preserving `goal_column`.

### By Line Boundary
- `cursor_line_start(state: &EditorState) -> Option<TransactionSpec>`: Move cursor to column 0 of the line.
- `select_line_start(state: &EditorState) -> Option<TransactionSpec>`: Extend selection to column 0 of the line.
- `cursor_line_end(state: &EditorState) -> Option<TransactionSpec>`: Move cursor to the end of the line (before newline).
- `select_line_end(state: &EditorState) -> Option<TransactionSpec>`: Extend selection to the end of the line.
- `cursor_line_boundary_backward(state: &EditorState) -> Option<TransactionSpec>`: Smart Home: toggles between first non-whitespace character and column 0.
- `select_line_boundary_backward(state: &EditorState) -> Option<TransactionSpec>`: Extend selection to smart line boundary backward.
- `cursor_line_boundary_forward(state: &EditorState) -> Option<TransactionSpec>`: Move cursor to end of line boundary.
- `select_line_boundary_forward(state: &EditorState) -> Option<TransactionSpec>`: Extend selection to end of line boundary.
- `select_line(state: &EditorState) -> Option<TransactionSpec>`: Expands selections to entire line(s) including trailing newline delimiter.

### By Document Boundary
- `cursor_doc_start(state: &EditorState) -> Option<TransactionSpec>`: Move cursor to the beginning of the document (offset 0).
- `select_doc_start(state: &EditorState) -> Option<TransactionSpec>`: Extend selection to the beginning of the document.
- `cursor_doc_end(state: &EditorState) -> Option<TransactionSpec>`: Move cursor to the end of the document.
- `select_doc_end(state: &EditorState) -> Option<TransactionSpec>`: Extend selection to the end of the document.

---

## 3. Deletion (Grapheme-Cluster-Aware)

- `delete_char_backward(state: &EditorState) -> Option<TransactionSpec>`: Backspace: deletes selection or preceding grapheme cluster (emojis, combining marks, CRLF pairs).
- `delete_char_forward(state: &EditorState) -> Option<TransactionSpec>`: Delete: deletes selection or following grapheme cluster.
- `delete_group_backward(state: &EditorState) -> Option<TransactionSpec>`: Delete preceding word/group (`Ctrl+Backspace`).
- `delete_group_forward(state: &EditorState) -> Option<TransactionSpec>`: Delete following word/group (`Ctrl+Delete`).
- `delete_to_line_start(state: &EditorState) -> Option<TransactionSpec>`: Deletes from cursor to start of line; deletes previous newline if already at start.
- `delete_to_line_end(state: &EditorState) -> Option<TransactionSpec>`: Deletes from cursor to end of line; deletes following newline if already at end.
- `delete_line(state: &EditorState) -> Option<TransactionSpec>`: Deletes the entire line containing the cursor, including its newline (`Ctrl+Shift+K`).
- `delete_trailing_whitespace(state: &EditorState) -> Option<TransactionSpec>`: Strips all trailing spaces and tabs from document lines.

---

## 4. Line Manipulation

- `move_line_up(state: &EditorState) -> Option<TransactionSpec>`: Moves selected line(s) upward (`Alt+Up`).
- `move_line_down(state: &EditorState) -> Option<TransactionSpec>`: Moves selected line(s) downward (`Alt+Down`).
- `copy_line_up(state: &EditorState) -> Option<TransactionSpec>`: Duplicates selected line(s) upward (`Shift+Alt+Up`).
- `copy_line_down(state: &EditorState) -> Option<TransactionSpec>`: Duplicates selected line(s) downward (`Shift+Alt+Down`).
- `split_line(state: &EditorState) -> Option<TransactionSpec>`: Breaks line at cursor without advancing cursor position.
- `transpose_chars(state: &EditorState) -> Option<TransactionSpec>`: Swaps characters surrounding the cursor.

---

## 5. Indentation & Newlines

- `indent_selection(state: &EditorState) -> Option<TransactionSpec>`: Context-sensitive auto-indentation using `ferrum-language` syntax tree analysis.
- `indent_more(state: &EditorState) -> Option<TransactionSpec>`: Indents selected lines by one unit (`INDENT_UNIT` facet, spaces or tabs).
- `indent_less(state: &EditorState) -> Option<TransactionSpec>`: Dedents selected lines by one unit, handling mixed space and tab columns.
- `insert_tab(state: &EditorState) -> Option<TransactionSpec>`: Inserts a physical tab or soft spaces, or indents if multi-line selection.
- `insert_newline_and_indent(state: &EditorState) -> Option<TransactionSpec>`: Inserts newline with auto-computed indentation, with bracket explosion when between `{}` / `[]` / `()`.
- `insert_newline_keep_indent(state: &EditorState) -> Option<TransactionSpec>`: Inserts newline copying exact indentation of current line.
- `insert_blank_line(state: &EditorState) -> Option<TransactionSpec>`: Inserts an empty line below without moving the cursor (`Ctrl+Enter`).
- `insert_text(state: &EditorState, text: &str) -> Option<TransactionSpec>`: Inserts text across all cursors with multi-cursor OT mapping.

---

## 6. Transactional History (Undo / Redo)

- `history(config: HistoryConfig) -> Extension`: Installs history state field and grouping configuration into the editor.
- `history_keymap() -> Vec<KeyBinding>`: Default keybindings for undo/redo.
- `undo(state: &EditorState) -> Option<TransactionSpec>`: Undoes the most recent edit event.
- `redo(state: &EditorState) -> Option<TransactionSpec>`: Redoes the previously undone edit event.
- `undo_selection(state: &EditorState) -> Option<TransactionSpec>`: Undoes cursor movement / selection changes independently.
- `redo_selection(state: &EditorState) -> Option<TransactionSpec>`: Redoes cursor movement / selection changes.
- `undo_depth(state: &EditorState) -> usize`: Returns the number of undo events available on the stack.
- `redo_depth(state: &EditorState) -> usize`: Returns the number of redo events available on the stack.
- `isolate_history`: Transaction annotation forcing an undo event boundary (`Before`, `After`, `Full`).
- `add_to_history`: Transaction annotation (`bool`) to exclude edits from history while re-inverting and mapping concurrent edits via OT.
- `HISTORY_FIELD`: Canonical `StateField` maintaining transactional history branches.

---

## 7. Commenting & Uncommenting

- `COMMENT_TOKENS: LazyLock<Facet<CommentTokens, Option<CommentTokens>>>`: Facet registering line and block comment syntax for languages.
- `CommentTokens`: Configuration defining `line: Option<String>` (e.g. `"//"`, `"#"`), and `block: Option<BlockToken>` (e.g. `("/*", "*/")`, `("<!--", "-->")`).
- `toggle_comment(state: &EditorState) -> Option<TransactionSpec>`: Toggles line comment if configured, otherwise toggles block comment.
- `toggle_line_comment(state: &EditorState) -> Option<TransactionSpec>`: Toggles line comments across lines intersecting selection.
- `line_comment(state: &EditorState) -> Option<TransactionSpec>`: Adds line comment prefixes to selected lines.
- `line_uncomment(state: &EditorState) -> Option<TransactionSpec>`: Strips line comment prefixes from selected lines.
- `toggle_block_comment(state: &EditorState) -> Option<TransactionSpec>`: Toggles enclosing block comment tokens around selection.
- `block_comment(state: &EditorState) -> Option<TransactionSpec>`: Wraps selection in block comment tokens.
- `block_uncomment(state: &EditorState) -> Option<TransactionSpec>`: Removes enclosing block comment tokens from selection.
- `toggle_block_comment_by_line(state: &EditorState) -> Option<TransactionSpec>`: Toggles block comment on each line individually.
