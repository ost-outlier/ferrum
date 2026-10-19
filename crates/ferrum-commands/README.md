# ferrum-commands

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum-commands/badge.svg)](https://docs.rs/ferrum-commands)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

> **Pure editing commands, keymaps, and transactional history for Ferrum.**  
> Direct 1:1 behavioral parity with [`@codemirror/commands`](https://github.com/codemirror/commands).

---

## Overview

`ferrum-commands` provides the complete suite of text editing operations, cursor navigation algorithms, line manipulation utilities, and the time-bucketed transactional history engine (`undo` / `redo`). All commands are pure functions operating on `EditorState`, producing `Option<TransactionSpec>`.

## Key Features

### 1. Pure State Commands (`StateCommand`)
Every editing command follows the signature:
```rust
pub type StateCommand = fn(&EditorState) -> Option<TransactionSpec>;
```
Commands do not mutate state directly; they inspect the state and return a transaction specification if applicable, enabling seamless composition, keymap dispatch, and undo tracking.

### 2. Grapheme-Cluster-Aware Navigation & Deletion
- Navigation and deletions never slice mid-codepoint or split Unicode extended grapheme clusters.
- Emojis with zero-width joiners (e.g., 👨‍👩‍👧‍👦), accented characters (combining marks), and CRLF (`\r\n`) line endings are handled atomically.
- Subword navigation respects camelCase and snake_case transitions.
- Vertical navigation (`cursor_line_up` / `cursor_line_down`) accurately retains the cursor's `goal_column` across shorter intervening lines.

### 3. Line Manipulation & Indentation
- **Move Lines**: `move_line_up` / `move_line_down` moves selected lines and blocks of lines, merging multi-selections cleanly.
- **Copy Lines**: `copy_line_up` / `copy_line_down` duplicates line selections above or below.
- **Line Selection**: `select_line` expands selections to entire lines including line breaks.
- **Line Deletions**: `delete_to_line_start` and `delete_to_line_end` delete to boundary or merge across line breaks.
- **Indentation**: `indent_more` / `indent_less` (4 spaces or tabs), `insert_newline_and_indent` (auto-indent), and `insert_newline_keep_indent`.
- **Formatting**: `delete_trailing_whitespace`, `split_line`, and `transpose_chars`.

### 4. Transactional History Engine (`history`, `undo`, `redo`)
Ported directly from `@codemirror/commands/src/history.ts`:
- **StateField-Driven**: Managed completely through `history()` extension and `EditorState` transactions.
- **Event Grouping**: Automatically groups adjacent keystrokes into single undo items based on time intervals (`new_group_delay`, default 500ms).
- **History Isolation**: `isolate_history` annotation forces boundaries between undo events.
- **Re-inversion & OT Mapping**: Untracked remote changes (e.g. collaborative edits) are rebased through the history stack using `ChangeSet::invert` and `map`.

### 5. Keymap System
- Configurable `KEYMAP` facet for keyboard bindings.
- Built-in `standard_keymap()` and `default_keymap()` covering all standard editor shortcuts.

---

## Quickstart

```rust
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_commands::commands::{
    default_keymap, dispatch_key, insert_newline_and_indent, move_line_down, KEYMAP,
};
use ferrum_commands::history::{history, undo, redo};

// Configure editor with default keymap and history tracking
let state = EditorState::create(EditorStateConfig {
    doc: Some("    fn example() {\n    }\n".into()),
    selection: None,
    extensions: Some(vec![
        KEYMAP.of(default_keymap()),
        history(),
    ]),
});

// Dispatch key press through the keymap facet
if let Some(spec) = dispatch_key(&state, "ArrowDown", false) {
    let next_state = state.apply(state.update(spec).unwrap());
    // ...
}
```

---

## Testing & Parity

All test suites are ported directly from CodeMirror 6:
- `@codemirror/commands/test/test-commands.ts` $\rightarrow$ `tests::test_cm6_*`
- `@codemirror/commands/test/test-history.ts` $\rightarrow$ `tests::test_cm6_history_*`

Run test suite:
```bash
cargo test -p ferrum-commands --lib
```

---

## License

Dual-licensed under either of:
- MIT License ([LICENSE-MIT](https://github.com/your-org/ferrum/blob/main/LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/your-org/ferrum/blob/main/LICENSE-APACHE))

Strict Clean-Room implementation: contains zero copyleft or GPL code.
