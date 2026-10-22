# Audit scope: `ferrum-language` vs `@codemirror/language`

Use with `1-parity-audit.md` (specification audit for a new crate). Audit order: **3 of 4**; it depends on `ferrum-state` and `ferrum-view` and may overlap with the view audit once the view's Gate 1 (dispositions) is approved. The language crate is rebuilt from cm6: the existing `ferrum/crates/ferrum-language/` is not read, tested or described.

Versions: cm6 `@codemirror/language` 6.13.1 (`CodeMirror6/language/package.json`), `@codemirror/streamparser` as in `CodeMirror6/streamparser/package.json`; Lezer packages at the versions the harness installs (record them in `ferrum/docs/parity/language/progress.md`).

This audit is different from the others. cm6's `language` is built on Lezer (`@lezer/common` trees, `NodeType`, `NodeProp`, incremental parsing with `TreeFragment`s, mixed parsing; `@lezer/highlight` tags). Ferrum intends to use tree-sitter behind its own abstraction (decision D3). So the audit has two goals:

1. **Behavior specification, independent of the parser.** What `language` does for an editor (which tree is current, when parsing happens, how indentation, folding, bracket matching, highlighting and language data are computed and exposed), as behaviors with fixtures.
2. **Parser-coupling map.** For each behavior, which Lezer capability it relies on (node props, `resolveInner`, `enterUnfinishedNodesBefore`, `TreeCursor`, `NodeIterator`, fragments, `Parser.startParse`, `parseMixed`, tags), with `file:line`. This is the evidence for D3 and the input to the syntax contract (`contracts.md`); it is not a design. The implementation sketch (`1-parity-audit.md` §16) may propose how a tree-sitter backend answers each capability, marked as a sketch.

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `CodeMirror6/language/src/`: `language.ts`, `indent.ts`, `fold.ts`, `highlight.ts`, `matchbrackets.ts`, `stream-parser.ts`, `isolate.ts`, `index.ts`; `CodeMirror6/streamparser/src/` (`stringstream.ts`, `index.ts`) | none (new crate) |
| Tests | `CodeMirror6/language/test/`: `test-syntax.ts`, `test-fold.ts`, `test-stream-parser.ts` | none |
| Behavior tests elsewhere (for T7) | `CodeMirror6/lang-javascript/test/test-indent.ts`, `CodeMirror6/lang-markdown/test/test-commands.ts`, and the cases of `CodeMirror6/commands/test/test-commands.ts` that use `javascriptLanguage`, `htmlLanguage` or `indentUnit` | n/a |
| Symbol list | `CodeMirror6/language/src/README.md` | not audited |
| Dependencies | `@codemirror/state`, `@codemirror/view`, `@codemirror/streamparser`, `@lezer/common`, `@lezer/highlight`, `@lezer/lr`, `style-mod` | lower crates: `ferrum-state`, `ferrum-view` (public APIs and conformance reports) |

The `@lezer/*` packages are not in the local mirror: install them in the harness with npm (`@lezer/common`, `@lezer/lr`, `@lezer/highlight`, and the grammars the tests use, such as `@lezer/javascript`, `@lezer/html`, `@lezer/markdown`), read their sources there, and produce fixtures with the real grammars. Cite Lezer code as `<package> <version> <file>:<line>` (`ferrum/docs/parity/README.md`, "External sources").

## Chunks (suggested)

1. Inventory and dispositions (all exports; `StreamLanguage`/`StringStream` get a disposition proposal for D3)
2. `language.ts`: `Language`, `LRLanguage`, `defineLanguageFacet`, `language` facet, `syntaxTree`, `ensureSyntaxTree`, `syntaxTreeAvailable`, `syntaxParserRunning`, `forceParsing`, `ParseContext` (work budget, viewport-aware parsing, `skipUntilInView`, fragments, `getSkippingParser`), `LanguageSupport`, `LanguageDescription`, `languageDataProp`, `sublanguageProp`, `DocInput`
3. `indent.ts`
4. `fold.ts`
5. `matchbrackets.ts`
6. `highlight.ts` (style model: D7; tags: D3)
7. `stream-parser.ts` + `CodeMirror6/streamparser/src/`
8. `isolate.ts`
9. Tests (including the behavior tests elsewhere)

## Special attention

- **Parse scheduling.** cm6 parses incrementally with a time budget, viewport first, continuing in the background (`ParseWorker`). Record every timing, budget and viewport behavior: they matter for large documents regardless of parser. Time-budget behavior is a hazard (`Date.now()`, `1-parity-audit.md` §9) and a native opportunity (parsing off the main thread changes observable behavior: list it as such in the sketch, §17.3).
- **Indentation.** `getIndentation` resolution order (`indentService` first, then syntax), `IndentContext` options (`overrideIndentation`, `simulateBreak`, `simulateDoubleBreak`), `TreeIndentContext` (`baseIndent`, `baseIndentFor`, `continue`, `textAfter`), `delimitedIndent`/`continuedIndent`/`flatIndent`, `indentOnInput` (a `transactionFilter` returning `[tr, {changes, sequential: true}]`, which needs the state's full transaction pipeline), `indentUnit` validation (cm6 throws on invalid configuration: an error-contract question).
- **Folding.** `foldService`, `foldNodeProp`, `foldInside`, `foldable`, `foldState` mapping through changes (fold decorations use non-inclusive sides: consumer values for `RangeSet.map`), `foldEffect`/`unfoldEffect`, fold commands and keymap, `codeFolding` config, placeholder widget, `foldGutter` (model in core, drawing in surface).
- **Bracket matching.** `matchBrackets` with and without a tree, `bracketMatching` config, `bracketMatchingHandle`; it iterates the document with `iterRange`.
- **Language data.** `languageDataProp`, `Language.data`, and how `EditorState.languageDataAt` reaches them: commands' comment toggling and the categorizer depend on it.
- **Tests outside the package.** Behaviors without a test in `CodeMirror6/language/test/` are common. Link them to `lang-*` or `commands` tests where those exist; otherwise they get derived tests from fixtures.
- **Grammar-dependent values.** Every fixture and test value that depends on a Lezer grammar is marked `NEEDS GRAMMAR` with the grammar and version; the conformance audit compares them against the grammar chosen in D3 and lists every changed value.

## Capabilities to check in the lower crates

`@codemirror/language` uses these state capabilities. For each, check the rebuilt `ferrum-state` public API (§10 of the brief) and record the result in `raw/xc.jsonl`; a missing or different one becomes a register entry against `ferrum-state`. (Register ids, where one already exists, in parentheses.)

- `Text` iteration: `iter`, `iterRange`, `iterLines` (XC-004); `ChangeDesc.iterChangedRanges` (XC-008).
- `RangeSet.map` with the sides of fold decorations passed through without truncation (XC-010); `RangeSet.update` with `filter`/`filterFrom`/`filterTo`, `sort` and ordering errors (XC-018).
- `Facet.from`, field `provide`, `StateField.init`, `StateEffect.appendConfig`, computed facets with declared dependencies (XC-020, XC-023, XC-024).
- `EditorState.languageDataAt` (XC-027); `Transaction.isUserEvent` prefix matching (XC-028).
- `transactionFilter` returning replacement specs with `sequential: true` (needed by `indentOnInput`) (XC-033).

From `ferrum-view`: the decoration, view plugin and widget APIs that folding, bracket matching and highlighting use, as its conformance report lists them.


## Calibration notes (not a work list; may contain errors)

- `CodeMirror6/language/test/` has only 13 cases in its own package (`ferrum-foundation.md` §3); most of the behavior is tested in `lang-*` and `commands`.
- `StreamLanguage` is used heavily by `CodeMirror6/legacy-modes/`; whether Ferrum keeps it is part of D3.
