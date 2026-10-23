# C5 notes (state.ts + transaction.ts)

> What: chunk C5 decision evidence, cross-crate facts, oracle, open questions. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `docs/parity/state/raw/C5-api.md` § Reproductions.

## Decision evidence

### D1 (position unit)
Every C5 API that takes or returns positions uses UTF-16 code units in cm6 and code points (`char` indices) in Ferrum: `EditorStateConfig.selection`, `TransactionSpec.selection`/`changes`, `changes()`, `changeByRange` callback ranges, `replaceSelection` cursor, `sliceDoc(from, to)`, `wordAt(pos)`, `languageDataAt(name, pos)`, `charCategorizer(at)`, `toJSON`/`fromJSON` selection offsets, `checkSelection` (doc length), `StateEffect.map` (positions inside effect values).

| item | cm6 file:line | Ferrum file:line | example (cm6 → Ferrum) | UTF-8 bytes | code points (current) | UTF-16 |
|---|---|---|---|---|---|---|
| `replaceSelection` cursor = `from + text.length` | `mirrors/cm6/state/src/state.ts:132` | `crates/ferrum-state/src/state.rs:247-249` | "ab", cursor 1, insert "😀" → cm6 cursor 3, Ferrum 2 (R16d) | cursor 5 (`len()` of "😀" = 4) | 2 | 3 (matches cm6 tests verbatim) |
| `wordAt` cluster stepping | `mirrors/cm6/state/src/state.ts:385-396` | none | "a𝐀b c", wordAt(1) → cm6 0-4 (R30c) | 0-6 | 0-3 | 0-4 |
| `changeByRange` callback arithmetic | `mirrors/cm6/state/src/state.ts:144-171` | `crates/ferrum-state/src/state.rs:208-242` | cm6 callbacks doing `r.from - 1` next to an astral char land inside a surrogate pair (observed in the first, flawed non-ASCII oracle run: cm6 produced lone surrogates shown as U+FFFD); with code points this cannot happen, with bytes it panics on a non-boundary slice unless clamped | must guard char boundaries | cannot split a code point (can split a grapheme) | can split a surrogate pair (cm6 behavior) |
| `toJSON`/`fromJSON` selection offsets | `mirrors/cm6/state/src/state.ts:208,235` | none | JSON produced by cm6 for a doc with astral chars is only interchangeable if Ferrum converts offsets | conversion needed | conversion needed | identical to cm6 JSON |
| `checkSelection` bound = `doc.length` | `mirrors/cm6/state/src/state.ts:250`, `mirrors/cm6/state/src/transaction.ts:185` | none | bound must be expressed in the chosen unit | `len_bytes` | `len_chars` | UTF-16 length |
| `StateEffect.map` / `mapEffects` | `mirrors/cm6/state/src/transaction.ts:79-106` | none | effect positions mapped through `ChangeDesc` in the same unit | — | — | — |

Oracle evidence: with positions converted to code points, `replaceSelection` and `changeByRange` (`rs`, `cbr3`) agree on 500 non-ASCII cases (0 mismatches), i.e. the algorithm is unit-independent; only the numbers differ.

### D2 (line breaks / Text model)
| item | cm6 file:line | Ferrum file:line | example | note |
|---|---|---|---|---|
| `create` string doc split | `mirrors/cm6/state/src/state.ts:246` | `crates/ferrum-state/src/state.rs:85,106` (Text only) | "a\r\nb\rc\u2028d" → cm6 3 lines / length 7; Ferrum `Text::from` 4 lines / length 8 (R21) | ropey `unicode_lines` breaks on U+2028 and stores `\r\n` as 2 chars |
| `create` with `lineSeparator` "\r\n" | `mirrors/cm6/state/src/state.ts:246` | none | "a\nb\r\nc" → cm6 2 lines / length 5; Ferrum 3 lines / length 6 (R21c) | facet ignored |
| `toText` | `mirrors/cm6/state/src/state.ts:185` | none | "a\r\nb\rc\nd" → cm6 4 lines / length 7; with sep "\r\n" 2 lines (R19, R19b) | — |
| `sliceDoc` / `lineBreak` | `mirrors/cm6/state/src/state.ts:189-191,286` | none | doc "a\r\nb\nc" → cm6 "a\nb\nc" (len 5); with sep "\r\n" → "a\r\nb\nc"; Ferrum `slice_string` → "a\r\nb\nc" (len 6) (R20, R20b, R20c) | Ferrum output keeps whatever separators the input had |
| `replaceSelection` | `mirrors/cm6/state/src/state.ts:130` | `crates/ferrum-state/src/state.rs:245-253` | "a\r\nb" into "xy" → cm6 "a\nbxy" len 5 cursor 3; Ferrum "a\r\nbxy" len 6 cursor 4 (R16); with sep "\r\n" and "a\r\nb\nc" → cm6 len 7/2 lines/cursor 5, Ferrum 8/3/6 (R16b) | — |
| `changes()` / `TransactionSpec.changes` string inserts | `mirrors/cm6/state/src/state.ts:178`, `mirrors/cm6/state/src/transaction.ts:335`, `mirrors/cm6/state/src/change.ts:335` | `crates/ferrum-state/src/state.rs:201-203`, `crates/ferrum-state/src/change.rs:244-277` | insert "a\r\nb" → cm6 inserted length 3; Ferrum 4 (R18) | — |
| `lineSeparator` facet | `mirrors/cm6/state/src/extension.ts:17` | `crates/ferrum-state/src/state.rs:25-27,185` | facet value stored, never read | Under the D2 recommendation (normalize on input, keep separator for output) every row above would need the facet consulted |

### D6 (ownership / Arc)
| item | cm6 file:line | Ferrum file:line | evidence |
|---|---|---|---|
| `Transaction.state` caching | `mirrors/cm6/state/src/transaction.ts:219-222` | `crates/ferrum-state/src/transaction.rs:176-178` | Each call builds a new `Arc<EditorState>` and re-runs all field updates (R4: `ptr_eq` false, 2 updates). A cached `OnceLock<Arc<EditorState>>` (or equivalent) would be needed to keep cm6 identity semantics. |
| reentrancy (`tr._state` set before slots) | `mirrors/cm6/state/src/state.ts:58` | `crates/ferrum-state/src/state.rs:308-365` | Field update reading `tr.state()` → stack overflow abort (R33c / `src/bin/reentrant.rs`). cm6 test-state#16 depends on it. |
| value identity across states | `mirrors/cm6/state/src/state.ts:121` + facet `compare` (C4) | `crates/ferrum-state/src/state.rs:357-361` | Dynamic facets recomputed for every new state; `std::ptr::eq` on outputs false where cm6 `===` is true (R34). Consumers comparing `startState.facet(x) != state.facet(x)` (e.g. `mirrors/cm6/view/src/draw-selection.ts:69`, `mirrors/cm6/view/src/gutter.ts:225,502`) would see a change on every transaction unless they compare by value. |
| `EditorState::create` returns `Arc` | `mirrors/cm6/state/src/state.ts:243` | `crates/ferrum-state/src/state.rs:105` | `update` requires `self: &Arc<Self>`; `Arc::ptr_eq` is the analogue of `===` and holds for `start_state` (R40). |

## Cross-crate facts
| id | capability | cm6 evidence | searched in Ferrum | what exists |
|---|---|---|---|---|
| CC-C5-1 | `Transaction.selection` = explicit selection only (drives `ViewUpdate.selectionSet`) | `mirrors/cm6/view/src/extension.ts:494-496` (`transactions.some(tr => tr.selection)`) | `selection_changed` in ferrum-view | `crates/ferrum-view/src/view.rs:96` sets `selection_changed = tr.selection().is_some()`, which is true for every doc-changing transaction because Ferrum stores the mapped selection (R5) |
| CC-C5-2 | `languageDataAt` (comment tokens, indentOnInput, wordChars) | `mirrors/cm6/commands/src/comment.ts:64`, `mirrors/cm6/language/src/indent.ts:376`, `mirrors/cm6/view/src/cursor.ts:16,103`, `mirrors/cm6/commands/src/commands.ts:94,116,604` | `language_data`, `languageData`, `word_chars`, `indent_on_input` in all crates | absent; ferrum-commands uses its own position-independent `COMMENT_TOKENS` facet `crates/ferrum-commands/src/comment.rs:59`, `get_config` ignores `pos` (`:72-74`) |
| CC-C5-3 | `isUserEvent` prefix matching | `mirrors/cm6/language/src/fold.ts:103`, `mirrors/cm6/language/src/indent.ts:375`, `mirrors/cm6/view/src/input.ts:418`, `mirrors/cm6/view/src/domobserver.ts:689` | `is_user_event`, `user_event` | absent; `crates/ferrum-commands/src/history.rs:489` reads the raw `UserEvent` string |
| CC-C5-4 | `sliceDoc` / `lineBreak` / `toText` | `mirrors/cm6/commands/src/comment.ts:82-97`, `mirrors/cm6/commands/src/commands.ts:740,745,770,772,811,821`, `mirrors/cm6/view/src/input.ts:490,499` | `slice_doc`, `line_break`, `to_text` | absent in all crates |
| CC-C5-5 | `phrase` | `mirrors/cm6/view/src/special-chars.ts:157`, `mirrors/cm6/view/src/dialog.ts:127` | `phrase` | absent |
| CC-C5-6 | `StateEffect.appendConfig` | `mirrors/cm6/view/src/dialog.ts:53`, `mirrors/cm6/language/src/fold.ts:177` | `append_config` | absent |
| CC-C5-7 | `Transaction.scrollIntoView` | `mirrors/cm6/view/src/editorview.ts:307` | `scroll_into_view` | only imperative `ViewState::scroll_into_view(pos)` `crates/ferrum-view/src/viewstate.rs:177`, called directly by `crates/ferrum-view-gpui/src/editor_view.rs:181,189,203,215,288` |
| CC-C5-8 | `StateEffect.map` / `mapEffects` | `mirrors/cm6/commands/src/history.ts:318,346`, `mirrors/cm6/view/src/extension.ts:77` (scrollIntoView effect map) | `map_effects`, `fn map` on effects | absent; `crates/ferrum-commands/src/tests/test_history.rs:738` `test_cm6_hist_44_can_map_effects` only asserts `undo_depth == 0` |
| CC-C5-9 | `isolateHistory` annotation lives in `@codemirror/commands` | `mirrors/cm6/commands/src/history.ts:14` | `IsolateHistory` | defined in `crates/ferrum-state/src/transaction.rs:143`, consumed by `crates/ferrum-commands/src/history.rs:471,502` (candidate PLACEMENT DEVIATION) |
| CC-C5-10 | `Transaction.state` cached and identity-stable | `mirrors/cm6/view/src/editorview.ts:261,308` (`tr.state`) | `tr.state()` callers | `crates/ferrum-language/src/tests/test_fold.rs:41`, `crates/ferrum-language/src/tests/test_stream_parser.rs:91,126,143` call `tr.state()`; `crates/ferrum-view/src/view.rs:98` uses `apply` |
| CC-C5-11 | transaction filters/extenders used by view/language (e.g. `mirrors/cm6/language/src/indent.ts:375` `indentOnInput` is a `transactionFilter` returning `[tr, {changes, sequential: true}]`) | `mirrors/cm6/language/src/indent.ts:373-394` | `TRANSACTION_FILTER` users | only `crates/ferrum/examples/headless_poc.rs`, `crates/ferrum/examples/poc_editor.rs`; the boolean contract cannot express `indentOnInput` |
| CC-C5-12 | `readOnly` combine (first value) | `mirrors/cm6/state/src/extension.ts:29-31`; consumers commands | `is_read_only` | `crates/ferrum-commands/src/commands.rs` uses `is_read_only()` (Ferrum combine = any, R24) |
| CC-C5-13 | `allowMultipleSelections` enforcement | `mirrors/cm6/state/src/state.ts:123,251` | `allow_multiple_selections` | only read by `crates/ferrum-commands/src/tests/test_comment.rs` (as extension); never enforced |

## Oracle
- Files: cm6 side `docs/parity/state/oracle/cm6-probes/probe-C5/oracle.ts` (generator + cm6 run + comparator); Ferrum side `docs/parity/state/oracle/ferrum-repro/repro-C5/src/bin/oracle.rs`; data in `(oracle data, not archived: regenerate from the seed recorded in `docs/parity/state/oracle.md`)` (`ascii.tsv`, `.cm6`, `.ferrum`; `nonascii.tsv`, …).
- PRNG: mulberry32. ASCII run: seed 3285, 3000 cases. Non-ASCII run: seed 777, 500 cases.
- Inputs: doc of 0–19 chars from `"ab \n"` (non-ASCII: `"ab \né😀"`, picked per code point), 1–4 random ranges (anchor/head anywhere), random main index, `allowMultipleSelections` on. Ops: `rs` = `replaceSelection(text)` with text of 0–3 chars from `"xy\n"`; `cbr1` = replace range by `"Z"×((from+2·to) mod 3)`, range from..from+n; `cbr2` = replace `from−1..to+1` (clamped) by "W", cursor; `cbr3` = no change, range reversed; `cbr4` = two inserts "<" at from and ">" at to, range from+1..to+1.
- Compared: resulting doc, selection (anchor:head list), main index, change description string (`ChangeDesc.toString` vs Ferrum `Display`, same format), normalized input selection.
- Result ASCII: 3000 cases, 0 mismatches.
- Result non-ASCII: restricted to `rs` and `cbr3` (the other ops compute positions arithmetically, so the same callback means different things in different units); cm6 positions converted to code points, change description omitted: 500 cases, 0 mismatches.
- Not covered by the oracle (known mismatches, minimal cases in docs/parity/state/raw/C5-api.md): `\r` in inserted text (R16), effect mapping (R17), cursor `assoc` (R42), selection collapse without `allowMultipleSelections` (R2/R3), out-of-range selections (R1).

## Open questions
1. Annotation/effect typing (state.cm6.121, 122, 128, 132): typed `AnnotationType<T>`/`StateEffectType<T>` objects with ids, or `TypeId` keyed by value type with a newtype convention? The current design makes two cm6 types with the same value type indistinguishable (R9, R9b) and has no place for `StateEffect.define({map})`.
2. Transaction filter contract (state.cm6.033, state.fe.303/304): cm6 filters rewrite transactions (return specs); Ferrum filters veto (`bool`, `update → None`). Keep a veto at all? cm6 has no rejection — a filter returns an empty spec instead (R12c).
3. `EditorState::update` returning `Option<Transaction>` vs always a `Transaction` (state.EditorState.update.B23); and single spec vs `...specs`.
4. `Transaction::state()` caching and reentrancy (state.Transaction.state.B1–B3): needed for cm6 semantics, interacts with D6 (`Arc`, `Send + Sync`).
5. `Transaction::selection()` returning the mapped selection when none was set (state.Transaction.selection.B1): changes `ViewUpdate` semantics in ferrum-view (CC-C5-1).
6. `EditorState::apply` is public and accepts a transaction built on a different state (R45): keep public, restrict, or make it an internal of `Transaction::state()`?
7. `readOnly` combine `any` vs cm6 "first value" (R24): no verifiable reason found for the change.
8. `CompartmentReconfigure` and builders on `TransactionSpec`/`RangeResult` (FERRUM-ONLY candidates): keep as public API?
9. `IsolateHistory` placement in ferrum-state instead of ferrum-commands (CC-C5-9).
10. cm6 quirk state.EditorState.update.B22 (extenders applied twice when a transaction filter returns specs): reproduce in Ferrum or not?

## Unfinished
none
