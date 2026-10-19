# ferrum-commands / src — API Reference

This crate exports a collection of generic editing commands, along with key bindings and transactional history for Ferrum, adapted from and in 1:1 behavioral parity with [`@codemirror/commands`](https://code.haverbeke.berlin/codemirror/commands).

All commands are pure state functions with signature:
```rust
pub type StateCommand = fn(&EditorState) -> Option<TransactionSpec>;
```

---

### Keymaps

- `standard_keymap() -> Vec<KeyBinding>`: Standard cursor navigation, multi-cursor, and deletion keybindings.
- `default_keymap() -> Vec<KeyBinding>`: Extends `standard_keymap` with line movements, indentation, and line duplication.
- `history_keymap() -> Vec<KeyBinding>`: Keybindings for undo (`Mod-z`) and redo (`Mod-y` / `Shift-Mod-z`).
- `KEYMAP: LazyLock<Facet<Vec<KeyBinding>, Vec<KeyBinding>>>`: Facet registering keymaps in the editor state.
- `dispatch_key(state: &EditorState, key_name: &str, shift: bool) -> Option<TransactionSpec>`: Dispatches a key event through registered keymaps.

---

### Selection

- `simplify_selection(state) -> Option<TransactionSpec>`: Collapses multi-cursor selections to the primary cursor.
- `select_all(state) -> Option<TransactionSpec>`: Selects all text in the document (`0..doc.len_chars()`).

#### By character
- `cursor_char_left(state) -> Option<TransactionSpec>`: Move cursor one grapheme cluster backward.
- `select_char_left(state) -> Option<TransactionSpec>`: Extend selection one grapheme cluster backward.
- `cursor_char_right(state) -> Option<TransactionSpec>`: Move cursor one grapheme cluster forward.
- `select_char_right(state) -> Option<TransactionSpec>`: Extend selection one grapheme cluster forward.

#### By group / word
- `cursor_group_left(state) -> Option<TransactionSpec>`: Move cursor one word/group backward.
- `select_group_left(state) -> Option<TransactionSpec>`: Extend selection one word/group backward.
- `cursor_group_right(state) -> Option<TransactionSpec>`: Move cursor one word/group forward.
- `select_group_right(state) -> Option<TransactionSpec>`: Extend selection one word/group forward.
- `cursor_subword_backward(state) -> Option<TransactionSpec>`: Move cursor one camelCase/snake_case subword backward.
- `select_subword_backward(state) -> Option<TransactionSpec>`: Extend selection one subword backward.
- `cursor_subword_forward(state) -> Option<TransactionSpec>`: Move cursor one camelCase/snake_case subword forward.
- `select_subword_forward(state) -> Option<TransactionSpec>`: Extend selection one subword forward.

#### Vertical motion
- `cursor_line_up(state) -> Option<TransactionSpec>`: Move cursor one line up, retaining `goal_column`.
- `select_line_up(state) -> Option<TransactionSpec>`: Extend selection one line up, retaining `goal_column`.
- `cursor_line_down(state) -> Option<TransactionSpec>`: Move cursor one line down, retaining `goal_column`.
- `select_line_down(state) -> Option<TransactionSpec>`: Extend selection one line down, retaining `goal_column`.

#### By line boundary
- `cursor_line_start(state) -> Option<TransactionSpec>`: Move cursor to column 0 of current line.
- `select_line_start(state) -> Option<TransactionSpec>`: Extend selection to column 0 of current line.
- `cursor_line_end(state) -> Option<TransactionSpec>`: Move cursor to end of current line (before newline).
- `select_line_end(state) -> Option<TransactionSpec>`: Extend selection to end of current line.
- `cursor_line_boundary_backward(state) -> Option<TransactionSpec>`: Smart Home: first non-whitespace char on line, then col 0.
- `select_line_boundary_backward(state) -> Option<TransactionSpec>`: Shift+Home: extend selection to line boundary backward.
- `cursor_line_boundary_forward(state) -> Option<TransactionSpec>`: End: jump to end of line.
- `select_line_boundary_forward(state) -> Option<TransactionSpec>`: Shift+End: extend selection to end of line.
- `select_line(state) -> Option<TransactionSpec>`: Expand selection to cover entire line(s) including line break.

#### By document boundary
- `cursor_doc_start(state) -> Option<TransactionSpec>`: Move cursor to start of document (position 0).
- `select_doc_start(state) -> Option<TransactionSpec>`: Extend selection to start of document.
- `cursor_doc_end(state) -> Option<TransactionSpec>`: Move cursor to end of document (`doc.len_chars()`).
- `select_doc_end(state) -> Option<TransactionSpec>`: Extend selection to end of document.

---

### Deletion

- `delete_char_backward(state) -> Option<TransactionSpec>`: Grapheme-aware backspace (deletes entire emoji, combining marks, or CRLF).
- `delete_char_forward(state) -> Option<TransactionSpec>`: Grapheme-aware delete forward.
- `delete_group_backward(state) -> Option<TransactionSpec>`: Delete word/group backward (Ctrl+Backspace).
- `delete_group_forward(state) -> Option<TransactionSpec>`: Delete word/group forward (Ctrl+Delete).
- `delete_to_line_start(state) -> Option<TransactionSpec>`: Delete to start of line, or previous line break if at start.
- `delete_to_line_end(state) -> Option<TransactionSpec>`: Delete to end of line, or following line break if at end.
- `delete_line(state) -> Option<TransactionSpec>`: Delete whole line including its newline delimiter (Ctrl+Shift+K).
- `delete_trailing_whitespace(state) -> Option<TransactionSpec>`: Delete all trailing whitespace before line ends.

---

### Line Manipulation

- `move_line_up(state) -> Option<TransactionSpec>`: Move selected line(s) up (Alt+Up).
- `move_line_down(state) -> Option<TransactionSpec>`: Move selected line(s) down (Alt+Down).
- `copy_line_up(state) -> Option<TransactionSpec>`: Duplicate line(s) above (Shift+Alt+Up).
- `copy_line_down(state) -> Option<TransactionSpec>`: Duplicate line(s) below (Shift+Alt+Down).
- `split_line(state) -> Option<TransactionSpec>`: Split line at cursor, leaving cursor on line before break.
- `transpose_chars(state) -> Option<TransactionSpec>`: Flip characters before and after cursor.

---

### Indentation

- `indent_more(state) -> Option<TransactionSpec>`: Indent selected lines by 4 spaces (Tab / Ctrl+]).
- `indent_less(state) -> Option<TransactionSpec>`: Un-indent selected lines by up to 4 spaces (Shift+Tab / Ctrl+[).
- `insert_newline_and_indent(state) -> Option<TransactionSpec>`: Enter with auto-indentation matching previous line.
- `insert_newline_keep_indent(state) -> Option<TransactionSpec>`: Insert newline keeping indentation level.
- `insert_blank_line(state) -> Option<TransactionSpec>`: Insert blank line below without moving cursor (Ctrl+Enter).

---

### Undo History

- `history(config: HistoryConfig) -> Extension`: Enables transactional undo/redo history on the editor state.
- `history_keymap() -> Vec<KeyBinding>`: Default keybindings for history commands.
- `undo(state: &EditorState) -> Option<TransactionSpec>`: Dispatches undo transaction.
- `redo(state: &EditorState) -> Option<TransactionSpec>`: Dispatches redo transaction.
- `IsolateHistory`: Transaction annotation forcing boundary between undo events (`Before`, `After`, `Full`).
- `AddToHistory(bool)`: Annotation to exclude transactions (e.g. collaborative remote edits) while re-projecting undo stack via OT.
- `HistoryConfig`: Configuration parameters for event grouping delay (`new_group_delay`, default 500ms) and depth limits.
