# Audit scope: `ferrum-commands` vs `@codemirror/commands`

Audit order: **4 of 4**, after `ferrum-state`, `ferrum-view` and `ferrum-language` have been rebuilt and their conformance audits (`3-conformance-audit.md`) approved.

This is a **hybrid audit**. It has three parts:

- **Part A, specification.** The cm6 side exactly as in `1-parity-audit.md` (inventory, dispositions, behaviors with golden fixtures, test catalog, hazards, contracts). That brief applies in full, except its rule 2.6: here the existing Ferrum code *is* read, in part B.
- **Part B, quarry.** The existing `ferrum-commands` code and tests are evaluated against the part A specification and against the **new** APIs of state, view and language. Unlike the other foundation crates, this one contains a good deal of work worth keeping (142 tests, most of them named after cm6 cases; history and comment logic); the goal is to salvage it item by item, not to describe its defects.
- **Part C, feedback.** `commands` is the top consumer of the foundation. Every capability a cm6 command needs from the lower crates is checked against their approved APIs. Gaps go back to the owning crate before the command is implemented, and the command tests that exercise lower-crate behavior become extra conformance fixtures for those crates.

No documentation or claims audit (READMEs, doc comments, commit messages): the implementer rewrites the crate docs after the conformance audit is approved.

## Preconditions

- State, view and language: conformance reports approved (`ferrum/docs/parity/<crate>/conformance.md`).
- `decisions.md` records D1, D2, D3 (syntax backend and grammars), D4 (layout contract) and **D5 (command types)**. D5 decides how `Command` (receives the view) and `StateCommand` look and where `StateCommand` lives; with it undecided, part B cannot classify any command.
- Placement decisions already recorded by earlier audits apply: keymap machinery (`KeyBinding`, the keymap facet, key dispatch) belongs to `ferrum-view`; `StateCommand` to `ferrum-state` (unless D5 says otherwise); `isolateHistory` to `ferrum-commands`.

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `CodeMirror6/commands/src/`: `commands.ts`, `history.ts`, `comment.ts`, `index.ts` | quarry (part B): `ferrum/crates/ferrum-commands/src/`: `commands.rs` (1,484 lines), `history.rs` (571), `comment.rs` (424), `lib.rs` |
| Tests | `CodeMirror6/commands/test/`: `test-commands.ts` (61 cases), `test-comment.ts` (18), `test-history.ts` (46), **`webtest-commands.ts` (57)**, helper `state.ts` | quarry: `src/tests/test_commands.rs` (61), `test_comment.rs` (35), `test_history.rs` (46) |
| Symbol list | `CodeMirror6/commands/src/README.md` | not audited |
| Lower crates (read-only, part C) | `@codemirror/state`, `@codemirror/view`, `@codemirror/language`, `@lezer/common` | public APIs and conformance reports of the new `ferrum-state`, `ferrum-view`, `ferrum-language` |

Test counts above were taken at commit `ed70610`. `history` and `comment` are in scope with the same rigor as `commands.ts`.

## Part A: specification

Follow `1-parity-audit.md` sections 5–12. Expected dispositions: `CORE` for state-only commands, history and comment; `CONTRACT` for commands of cm6 type `Command` whose answer comes from the view (visual lines, line boundaries, bidi, atomic ranges, page size) or from the syntax tree, with the question resolved against the now-existing contract traits of `ferrum-view`/`ferrum-language` instead of left open; `ELSEWHERE: ferrum-view` for keymap machinery.

Specific to this package:

- **Two command types.** For each `Command`-typed export, list each view query it makes (`view.moveByChar`, `moveByGroup`, `moveVertically`, `moveToLineBoundary`, `lineBlockAt`, `textDirectionAt`, `bidiSpans`, atomic ranges, `scrollIntoView`, `EditorView.scrollIntoView` effect) with `file:line`, and the Ferrum view API that answers it (part C).
- **"Did nothing" semantics.** cm6 commands return `false` when they would change nothing (`CodeMirror6/commands/src/commands.ts:28-33`, `moveSel`) and many check `state.readOnly`: `deleteBy`, `deleteTrailingWhitespace`, `splitLine`, `transposeChars`, `moveLine`, `copyLine`, `deleteLine`, `newlineAndIndent`, `indentSelection`, `indentMore`, `indentLess` (`CodeMirror6/commands/src/commands.ts`: `:534` `deleteBy`, `:667` `deleteTrailingWhitespace`, `:689` `splitLine`, `:700` `transposeChars`, `:732` `moveLine`, `:766` `copyLine`, `:791` `deleteLine`, `:852` `newlineAndIndent`, `:897` `indentSelection`, `:921` `indentMore`, `:931` `indentLess`). One behavior per check.
- **User events.** `"select"`, `"select.pointer"`, `"input"`, `"input.type"`, `"delete.backward"`, `"delete.forward"`, `"delete.line"`, `"move.line"`, `"undo"`, `"redo"`, …; history grouping depends on them and on `isUserEvent` prefix matching.
- **Line separator.** cm6 inserts `state.lineBreak`, never a literal `"\n"` (`CodeMirror6/commands/src/commands.ts`: `:740`, `:745` `moveLine`; `:770`, `:772` `copyLine`; `:811` `insertNewline`; `:821` `insertNewlineKeepIndent`).
- **Grammars.** `test-commands.ts` loads `javascriptLanguage` (`@lezer/javascript`) and `htmlLanguage`; `test-comment.ts` and `test-history.ts` configure `languageData` and history options. Install the Lezer grammars in the harness so that fixtures come from cm6 with the real grammar, and record which expected values depend on it (`NEEDS GRAMMAR`). They are compared against the grammar chosen in D3 during conformance.
- **Cost.** cm6 helpers work on the current line (`findClusterBreak` over `line.text`, `countColumn` on one line). State the cost class of every command.
- **History.** `history.ts` is a state machine (event grouping by time and user event, `newGroupDelay`, `joinToEvent`, selection-only events, `isolateHistory`, `invertedEffects`, effect mapping through `StateEffect.mapEffects`, `minDepth`, JSON serialization). Specify every transition as a behavior, with fixtures that drive sequences of transactions with explicit `Transaction.time` values (never wall-clock time).
- **Comment.** Line vs block comments, `languageDataAt("commentTokens", pos)` per range, mixed selections, blank lines, indentation of the inserted tokens.
- **Hazards to expect** (section 9 of the brief): identity of history events and `===` checks on state fields, `Transaction.time`, `Date.now()`, `Intl.Segmenter` in subword motion (`CodeMirror6/commands/src/commands.ts:112` `segmenter`, `:146-156` `moveBySubword`), surrogate handling in `deleteByChar` (`includeExtending`, `CodeMirror6/commands/src/commands.ts:572-585` `deleteByChar`).

## Part B: quarry evaluation

The existing crate was written against the old state/language APIs (and without `ferrum-view`); after the rebuild of the lower crates it will not compile. Evaluate it from its source and, where a behavior must be observed, by running it at the last commit where the workspace compiles (`ed70610` or later; record the commit) in a scratch git worktree, comparing against the part A fixtures.

Per Ferrum public item, one row with a verdict:

| Verdict | Meaning |
|---|---|
| `REUSE` | Implements every behavior of its cm6 item; only mechanical API renames needed. List the renames. |
| `ADAPT` | The algorithm is right, but it must be rewired to the new APIs (for example a state-only approximation that must call the view's `moveVertically`, or the whole-document string replaced by line access). List the behaviors already right and the exact API changes. |
| `REWRITE` | One or more behaviors differ or are missing at the algorithm level. List the behavior ids. |
| `MOVE: <crate>` | Belongs elsewhere. Keymap machinery (`KeyBinding`, `KEYMAP`, `dispatch_key`, `insert_text`) → `ferrum-view`: already evaluated by the view audit (`ferrum/docs/parity/view/quarry.md`, from `1b-scope-view.md` "Quarry from `ferrum-commands`"); record only the `MOVE` verdict here. |
| `REMOVE` | No cm6 counterpart and no justification (for example a helper with a cm6 name that behaves differently). |

Per Ferrum test, one row: `KEEP` (scenario, inputs, assertions and expected values identical to the cm6 case; adapt only the setup), `FIX` (same cm6 case but weaker or different: say exactly what), `REPLACE` (does not test the cm6 case it is named after), or `EXTRA` (no cm6 counterpart; keep only as a derived test of a listed behavior). Compare test bodies by behavior, not by name. Test helpers (`mk_state`, `state_str`) are compared with `CodeMirror6/commands/test/state.ts` (`|` cursor, `<…>` range, `allowMultipleSelections` on).

Deliverable: `ferrum/docs/parity/commands/quarry.md` (rendered from `raw/quarry.jsonl`: `id, item, file, line, cm6_item, verdict, behaviors_ok[], behaviors_bad[], api_changes, evidence`), with counts per verdict. The implementation (`2-parity-implementation.md`) copies code only for `REUSE`/`ADAPT` rows and tests only for `KEEP`/`FIX` rows, citing the row.

## Part C: feedback to the lower crates

1. For every cm6 command, list each capability it uses from state, view and language (cite `file:line`) and locate it in the new Ferrum API (crate, item, `file:line`). Absent, or present with different behavior: open an entry in `_cross-crate.md` owned by that crate, with the cm6 evidence and a fixture showing the expected behavior. The command is `BLOCKED: <crate>` until the entry is resolved and the owning crate's conformance audit has been re-run for that item.
2. For every cm6 test case of this package that exercises lower-crate behavior (most of `webtest-commands.ts`: wrapped lines, folds, widgets, bidi, line up/down; and the grammar-dependent cases), record which lower-crate behavior ids it pins down. Add them to the owning crate's conformance fixtures as integration cases (`ferrum/docs/parity/<crate>/fixtures/from-commands.jsonl`).
3. Report both lists in the gate summary.

Capabilities known to be needed (seed list, not a work list): from view `moveByChar`, `moveByGroup`, `moveToLineBoundary`, `moveVertically`, `lineBlockAt`, `textDirectionAt`, `bidiSpans`, `atomicRanges`, `scrollIntoView`, the keymap facet and `KeyBinding`; from language `getIndentation`, `IndentContext`, `indentString`, `getIndentUnit`, `indentUnit`, `syntaxTree`, `matchBrackets`, `foldable`/fold effects; from state `changeByRange`, `languageDataAt`, `charCategorizer`, `toText`, `lineBreak`, `sliceDoc`, `isUserEvent`, `readOnly`, `allowMultipleSelections`, `EditorSelection.range/cursor` with `assoc`, goal column and bidi level, `StateEffect.mapEffects`, `Transaction.time`/`userEvent`/`addToHistory`/`remote`, JSON for `ChangeSet`/`ChangeDesc`/`EditorSelection` (history serialization).

## Chunks (suggested)

1. Inventory and dispositions (all exports of the three source files)
2. `commands.ts`: motion by character, group and subword (`moveSel`, `byCharLogical`, `cursorByChar`, `cursorByGroup`, subword helpers)
3. `commands.ts`: motion by line, line boundary, page, document; syntax motion (`cursorSyntaxLeft/Right`, `selectParentSyntax`, `cursorMatchingBracket`)
4. `commands.ts`: selection commands (`selectAll`, `selectLine`, `simplifySelection`, `addCursorAbove/Below`, …)
5. `commands.ts`: deletion (`deleteBy`, `deleteByChar`, groups, line boundaries, `deleteLine`, `deleteTrailingWhitespace`)
6. `commands.ts`: line operations, newline, indentation, `transposeChars`, `insertTab`, `toggleTabFocusMode`
7. `commands.ts`: keymaps (`standardKeymap`, `defaultKeymap`, `emacsStyleKeymap`, `indentWithTab`)
8. `history.ts`
9. `comment.ts`
10. Tests: one chunk per test file; `webtest-commands.ts` split in two
11. Quarry: one chunk per Ferrum source file plus its tests (run after the matching part A chunks)
12. Feedback (part C): main session

## Calibration notes (not a work list; may contain errors)

From earlier looks at the existing crate (line numbers at commit `ed70610`, unchanged since `fb99ca1`) and from the `ferrum-state` audit. If part B does not find these independently, it is incomplete; if it finds something different, it prevails, with evidence.

- About 43 of the 116 symbols in the cm6 symbol list had no counterpart in `commands.rs` (history/comment symbols were only checked against the README).
- Only `StateCommand` exists, `ferrum/crates/ferrum-commands/src/commands.rs:20`, as `fn(&EditorState) -> Option<TransactionSpec>`; cm6 `cursorLineUp` (`CodeMirror6/commands/src/commands.ts:213`) goes through `cursorByLine` (`CodeMirror6/commands/src/commands.ts:204`) → `view.moveVertically`/`moveToLineBoundary`, while `cursor_line_up` (`ferrum/crates/ferrum-commands/src/commands.rs:541`) is state-only. Movement commands appear to always return `Some(..)` where cm6 returns `false` for an unchanged selection.
- `is_read_only()` is called only at `ferrum/crates/ferrum-commands/src/commands.rs` `:1139` `indent_more`, `:1150` `indent_less`, `:1183` `indent_selection`, `:1308` `insert_newline_and_indent`, against 11 cm6 read-only checks. No cm6 user events are set. History reads the raw `UserEvent` string (`ferrum/crates/ferrum-commands/src/history.rs:489`) instead of prefix matching.
- 14 `to_string()` calls in `ferrum/crates/ferrum-commands/src/commands.rs` materialize the whole document (cursor/select by char, group and subword; `delete_char_backward`/`forward`), and `find_cluster_break`/`find_word_break`/`find_subword_break` (18 call sites) run over that string; in the old state crate `find_cluster_break` was O(n) per call.
- Literal `"\n"` at `ferrum/crates/ferrum-commands/src/commands.rs:1041` (`split_line`), `:1384` (`insert_blank_line`) and in `insert_newline_keep_indent`; cm6 uses `state.lineBreak`.
- `default_keymap` binds Tab/Shift-Tab (`ferrum/crates/ferrum-commands/src/commands.rs:1480`); cm6 keeps Tab in `indentWithTab` (`CodeMirror6/commands/src/commands.ts:1137`). `insert_text`, `dispatch_key` and the `KEYMAP` facet have no counterpart in `@codemirror/commands` (cm6 keymaps live in view).
- `delete_char_backward` (`ferrum/crates/ferrum-commands/src/commands.rs:662`) removes one grapheme cluster; cm6 `deleteByChar` has an indentation-unit branch and `includeExtending` handling (`CodeMirror6/commands/src/commands.ts:572-585`).
- Comment tokens come from a crate-local `COMMENT_TOKENS` facet (`ferrum/crates/ferrum-commands/src/comment.rs:59`) whose `get_config` ignores the position (`ferrum/crates/ferrum-commands/src/comment.rs:72-74`); cm6 uses `languageDataAt("commentTokens", pos)`.
- History configuration is merged by hand in `HISTORY_CONFIG` (`ferrum/crates/ferrum-commands/src/history.rs:104`) instead of `combineConfig`; history uses `IsolateHistory` from the old state crate (`ferrum/crates/ferrum-commands/src/history.rs:471,502`), `inverted_desc` (`ferrum/crates/ferrum-commands/src/history.rs:278`) and `iter_changed_ranges` (`ferrum/crates/ferrum-commands/src/history.rs:419-421`).
- `test_cm6_hist_44_can_map_effects` (`ferrum/crates/ferrum-commands/src/tests/test_history.rs:738`) asserts only the undo depth, and `test_cm6_hist_46_survives_serialization` (`ferrum/crates/ferrum-commands/src/tests/test_history.rs:750`) serializes nothing.
- `ferrum/crates/ferrum-commands/src/tests/test_commands.rs` has 61 tests whose names match the 61 `it()` cases of `test-commands.ts` in order (names only compared). No Ferrum test matches the 57 `webtest-commands.ts` cases. `ferrum/crates/ferrum-commands/src/tests/test_comment.rs` has 35 tests for 18 cm6 cases: check for loops or helpers in cm6 versus additions in Ferrum.

## Gate deliverable

As in `1-parity-audit.md` section 14, plus: quarry counts per verdict (items and tests), the list of code and tests the implementation may copy, part C entries opened per lower crate, and the integration fixtures handed to each lower crate.
