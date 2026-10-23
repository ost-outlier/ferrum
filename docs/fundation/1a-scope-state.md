# Audit scope: `ferrum-state` vs `@codemirror/state`

> **Status: historical.** This sheet drove the state audit of 2026-10-21 (`docs/parity/state/`), under the previous version of `1-parity-audit.md` (`git show f63225f:docs/fundation/1-parity-audit.md`). Its cm6 content (scope, chunks, special attention) remains valid and is read once by `2a-implementation-state.md` §1 (Phase 0) to produce the specification. Its Ferrum column and calibration notes describe the old crate (commit `ed70610`), which is not kept: do not use them as a work list. For the rebuild of `ferrum-state`, start from `2a-implementation-state.md`. <!-- refs:ignore: git path syntax -->

Use with `1-parity-audit.md`. Audit order: **1 of 4** (no Ferrum dependencies).

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `mirrors/cm6/state/src/`: `text.ts`, `change.ts`, `selection.ts`, `state.ts`, `transaction.ts`, `facet.ts`, `extension.ts`, `config.ts`, `rangeset.ts`, `char.ts`, `charcategory.ts`, `column.ts`, `index.ts` | `crates/ferrum-state/src/`: `text.rs`, `change.rs`, `selection.rs`, `state.rs`, `transaction.rs`, `facet.rs`, `config.rs`, `rangeset.rs`, `char.rs`, `column.rs`, `lib.rs` |
| Tests | `mirrors/cm6/state/test/`: `test-text.ts`, `test-change.ts`, `test-selection.ts`, `test-state.ts`, `test-facet.ts`, `test-rangeset.ts`, `test-charcategory.ts` | `crates/ferrum-state/src/tests/*.rs` and doctests |
| Symbol list | `mirrors/cm6/state/src/README.md` | `crates/ferrum-state/src/README.md`, `crates/ferrum-state/README.md` |
| Dependencies | `@marijn/find-cluster-break` | `ropey`, `smallvec`, `parking_lot`, `thiserror`, `unicode-segmentation` |

Expected disposition: almost everything `CORE`. Dispositions other than `CORE` need a strong reason.

## Chunks (suggested)

1. `text.ts` (Text, Line, TextIterator, line splitting)
2. `change.ts` (ChangeDesc, ChangeSet, MapMode, mapping, composition, inversion, `iterChanges`, `iterGaps`, JSON)
3. `selection.ts`
4. `facet.ts` + `config.ts` + `extension.ts` (Facet, StateField, Compartment, Prec, extension resolution, dynamic slots, `compare`, `enables`, `provide`, `computeN`, `from`)
5. `state.ts` + `transaction.ts` (EditorState API, transactions, annotations, effects, filters, extenders, `changeByRange`, `languageDataAt`, `phrase`, `wordAt`, JSON)
6. `rangeset.ts` (RangeSet, RangeValue, builder, `iter`, `spans`, `compare`, `eq`, `join`, `update`, `map`)
7. `char.ts`, `charcategory.ts`, `column.ts`

## Special attention

- **Position unit (D1).** cm6 counts UTF-16 code units; Ferrum counts code points. For every API that takes or returns positions, record the unit and every place where the difference changes results (surrogate pairs, emoji, `findClusterBreak`, `countColumn`). Feed `decisions-evidence.md`.
- **Line breaks and `Text` model (D2).** cm6 splits on `DefaultSplit` (`/\r\n?|\n/`) or on the `lineSeparator` facet, stores lines without separators, and counts a line break as one position. Ferrum wraps `ropey` with default features (`unicode_lines`), which also breaks lines on U+2028, U+2029, U+0085, VT, FF and lone CR, and stores separators raw (`text.rs` `line_by_index` strips them per call). Audit `Text.of`, `line`, `lineAt`, `lines`, `length`, `sliceString` with a line separator argument, `iter`/`iterLines`/`iterRange`, and `state.lineBreak`/`sliceDoc`/`toText`.
- **`Text` tree semantics.** cm6 tests assert tree depth and node boundaries (`children`, "creates a balanced tree", "can insert on node boundaries"). Ferrum's port replaced some of these assertions (for example `len_lines` instead of depth). Classify each as `WEAKER`/`DIFFERENT` with the reason, and decide whether the behavior is observable through the public API.
- **Facet system.** Dynamic facet recomputation and `compare`, static vs dynamic, `enables`, precedence ordering, compartments and `reconfigure`/`appendConfig`, field `provide`, cyclic dependency detection, `StateField.init` overrides. These are subtle and drive every crate above.
- **Transaction pipeline.** Order of `changeFilter`, `transactionFilter`, `transactionExtender`, `filter: false`, `sequential`, `scrollIntoView`, `userEvent`/`isUserEvent`, `addToHistory`, `remote`, `time`.
- **`RangeSet`.** Ordering by `from`/`startSide`, `point` ranges, `mapMode`, `iter`/`spans`/`compare` with `minPointSize`, open-start/end semantics. `ferrum-view` and `ferrum-language` depend on it.
- **JSON.** `toJSON`/`fromJSON` for `ChangeSet`, `EditorSelection`, `EditorState` (with fields).

## Calibration notes (not a work list; may contain errors)

From a quick look; the audit must find these independently or refute them with evidence.

- A grep found no Ferrum counterpart for `changeFilter`, `transactionExtender`, `languageData`/`languageDataAt`, `phrases`/`phrase`, `toJSON`/`fromJSON`, `wordAt`, `sliceDoc`, `lineBreak`, `isUserEvent`, `appendConfig`, `StateEffectType`, `computeN`.
- `ropey = "1.6"` is used with default features (`unicode_lines`, `simd`) in the workspace `Cargo.toml`.
- `Text::line_by_index` (`crates/ferrum-state/src/text.rs:105`) converts the whole line to a `String` to strip the separator on each call.
- `EditorState::create` returns `Arc<EditorState>`; check where `Arc` changes observable behavior (identity comparisons that cm6 relies on, such as `state.field(f) === prevState.field(f)` for change detection).
