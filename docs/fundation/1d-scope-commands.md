# Audit scope: `ferrum-commands` vs `@codemirror/commands`

Use with `1-parity-audit.md`. Audit order: **4 of 4** (depends on `ferrum-state`, `ferrum-view` and `ferrum-language`; start after their gates, so that `BLOCKED` statuses rest on approved facts).

`commands` is the top of the foundation: cm6 `commands` depends on `state`, `view` and `language` (see `ferrum-foundation.md` section 3). `history` and `comment` are in scope with exactly the same rigor as `commands.ts`. They are not extras.

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `CodeMirror6/commands/src/`: `commands.ts`, `history.ts`, `comment.ts` | `ferrum/crates/ferrum-commands/src/`: `commands.rs`, `history.rs`, `comment.rs`, `lib.rs` |
| Tests | `CodeMirror6/commands/test/`: `test-commands.ts`, `test-comment.ts`, `test-history.ts`, **`webtest-commands.ts`**, helper `state.ts` | `ferrum/crates/ferrum-commands/src/tests/`: `test_commands.rs`, `test_comment.rs`, `test_history.rs`, and doctests |
| Symbol list | `CodeMirror6/commands/src/README.md` | `ferrum/crates/ferrum-commands/src/README.md`, `ferrum/crates/ferrum-commands/README.md` |
| Dependencies (read-only, A5) | `package.json`: `@codemirror/state`, `@codemirror/view`, `@codemirror/language`, `@lezer/common` | `Cargo.toml`: `ferrum-state`, `ferrum-language` (no `ferrum-view`) |

Expected disposition: `CORE` for state-only commands, history and comment; `CONTRACT` for commands of cm6 type `Command` that query the view (motion by visual line, line boundary, bidi, atomic ranges) or the syntax tree; `ELSEWHERE: ferrum-view` for keymap machinery.

## Chunks (suggested)

1. Inventory and dispositions (all exports of the three source files)
2. `commands.ts`: selection motion by character, group and subword (`moveSel`, `byCharLogical`, `cursorByChar`, `cursorByGroup`, subword helpers)
3. `commands.ts`: motion by line, line boundary, page, document; syntax motion (`cursorSyntaxLeft/Right`, `selectParentSyntax`, `cursorMatchingBracket`)
4. `commands.ts`: selection commands (`selectAll`, `selectLine`, `simplifySelection`, `addCursorAbove/Below`, …)
5. `commands.ts`: deletion (`deleteBy`, `deleteByChar`, groups, line boundaries, `deleteLine`, `deleteTrailingWhitespace`)
6. `commands.ts`: line operations, newline, indentation, `transposeChars`, `insertTab`, `toggleTabFocusMode`
7. `commands.ts`: keymaps (`standardKeymap`, `defaultKeymap`, `emacsStyleKeymap`, `indentWithTab`)
8. `history.ts`
9. `comment.ts`
10. Test audit, one chunk per test file; `webtest-commands.ts` split in two

## Special attention

- **Two command types.** cm6 has `StateCommand` and `Command` (receives the view). For each `Command`-typed export, list what it asks of the view (cite `file:line`) and whether a Ferrum counterpart approximates it state-only. Record what behavior is kept or lost (visual lines, wrapped lines, bidi, folds, widgets, atomic ranges, goal column). This feeds D5.
- **"Did nothing" semantics.** cm6 commands return `false` when they would not change anything (e.g. `moveSel`), and many check `state.readOnly`. Check every command.
- **User events.** cm6 tags transactions (`"select"`, `"select.pointer"`, `"input"`, `"input.type"`, `"delete.backward"`, `"delete.forward"`, `"delete.line"`, `"move.line"`, `"undo"`, `"redo"`, …). History grouping depends on them.
- **Line separator.** cm6 inserts `state.lineBreak`, never a literal `"\n"` (D2).
- **Language-dependent tests.** `test-commands.ts` loads `javascriptLanguage` (`@lezer/javascript`) and `htmlLanguage`; `test-comment.ts` and `test-history.ts` configure `languageData` and history options. Ferrum versions using `javascript.rs` (toy language) are `DIFFERENT` until proven otherwise. Record which expected values depend on the grammar.
- **Cost.** cm6 helpers work on the current line (`findClusterBreak` over `line.text`). Record every Ferrum command that materializes the whole document.

## Calibration notes (not a work list; may contain errors)

These come from a limited external look: `commands.ts`, `commands.rs`, the two READMEs, and the cm6 symbol list. `history` and `comment` were not examined. If your audit does not independently find these, it is incomplete. If you find something different, yours prevails, with evidence.

- About 43 of the 116 symbols in the cm6 symbol list have no counterpart in `commands.rs` or the crate README (history/comment symbols were only checked against the README).
- `commands.rs` defines only `StateCommand` (L20). Example: cm6 `cursorLineUp` (L213) goes through `cursorByLine` (L204), which calls `view.moveVertically` and `view.moveToLineBoundary`; Ferrum `cursor_line_up` (L541) is state-only.
- cm6 returns `false` when a movement leaves the selection unchanged (`moveSel`, commands.ts ~L28–33). Ferrum `cursor_*` commands appear to always return `Some(..)`.
- cm6 checks `state.readOnly` in `deleteBy`, `deleteTrailingWhitespace`, `splitLine`, `transposeChars`, `moveLine`, `copyLine`, `deleteLine`, `newlineAndIndent`, `indentSelection`, `indentMore`, `indentLess` (commands.ts L534, 667, 689, 700, 732, 766, 791, 852, 897, 921, 931). `commands.rs` calls `is_read_only()` only at L1139, 1150, 1183, 1308.
- No equivalent of cm6 transaction user events found in `commands.rs`.
- `commands.rs` calls `state.doc().to_string()` at the start of the cursor/select char, group and subword commands and of `delete_char_backward`/`delete_char_forward` (L78, 104, 129, 150, 175, 197, 219, 240, 261, 283, 305, 326, 663, 686), materializing the whole document per call; this contradicts the README's O(log N) claim (README L25).
- `commands.rs` uses a literal `"\n"` (L1041, L1384, and in the `format!` of `insert_newline_keep_indent`), while cm6 uses `state.lineBreak` (commands.ts L740, 745, 770, 772, 811, 821).
- `default_keymap` binds Tab to `indent_more` and Shift-Tab to `indent_less` (L1480). cm6's `defaultKeymap` has no Tab binding; Tab lives in `indentWithTab` (commands.ts L1137).
- `insert_text`, `dispatch_key` and the `KEYMAP` facet have no cm6 counterpart in this package. In cm6 the keymap facet and key dispatch live in `view`.
- The 61 tests in `test_commands.rs` have names matching the 61 `it()` cases of `test-commands.ts`, in the same order (only names compared). Cursor and selection movement cases live in `webtest-commands.ts` (57 cases: subword, line up/down, char select, wrapped lines, folds, widgets, bidi); no Ferrum test with a matching name was found for them.
- `test_comment.rs` has 35 tests; `test-comment.ts` has 18 `it()` cases. Check whether cm6 expands them (loops, helpers) or whether the extra ones were added in Ferrum.
- `delete_char_backward` (L662) removes one grapheme cluster. cm6 `deleteCharBackward` goes through `deleteByChar(view, false, true)`, which has an indentation-unit branch (commands.ts L572–585).
- cm6's test helper `mkState` (`test/state.ts`) uses `|` for a cursor and `<…>` for a range, and enables `allowMultipleSelections`. Check Ferrum's `mk_state`/`state_str` against it (T5).
