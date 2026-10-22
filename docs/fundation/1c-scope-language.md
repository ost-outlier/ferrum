# Audit scope: `ferrum-language` vs `@codemirror/language`

Use with `1-parity-audit.md`. Audit order: **3 of 4** (depends on `ferrum-state` and `ferrum-view`; may overlap with the view audit once the view inventory is approved).

This audit is different from the others. cm6's `language` is built on Lezer (`@lezer/common` trees, `NodeType`, `NodeProp`, incremental parsing with `TreeFragment`s, mixed parsing; `@lezer/highlight` tags). Ferrum intends to use tree-sitter behind its own abstraction (decision D3). So the audit has two goals:

1. **Behavior inventory, independent of the parser.** What `language` does for an editor (which tree is current, when parsing happens, how indentation, folding, bracket matching, highlighting and language data are computed and exposed), as numbered behaviors.
2. **Parser-coupling map.** For each behavior, which Lezer capability it relies on (node props, `resolveInner`, `enterUnfinishedNodesBefore`, `TreeCursor`, `NodeIterator`, fragments, `Parser.startParse`, `parseMixed`, tags). This is the evidence for D3, not a design.

Ferrum's current crate is close to a stub; expect most items to be `MISSING` or `BLOCKED: D3`. Do not spend effort describing stub internals beyond what the claims audit needs.

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `CodeMirror6/language/src/`: `language.ts`, `indent.ts`, `fold.ts`, `highlight.ts`, `matchbrackets.ts`, `stream-parser.ts`, `isolate.ts`, `index.ts`; and `CodeMirror6/streamparser/src/` (`stringstream.ts`, `index.ts`) | `ferrum/crates/ferrum-language/src/`: `language.rs`, `tree.rs`, `indent.rs`, `fold.rs`, `highlight.rs`, `stream.rs`, `javascript.rs`, `lib.rs` |
| Tests | `CodeMirror6/language/test/`: `test-syntax.ts`, `test-fold.ts`, `test-stream-parser.ts` | `ferrum/crates/ferrum-language/src/tests/*.rs` |
| Behavior tests elsewhere (read-only, for T7) | `CodeMirror6/lang-javascript/test/test-indent.ts`, `CodeMirror6/lang-markdown/test/test-commands.ts`, and `CodeMirror6/commands/test/test-commands.ts` cases that use `javascriptLanguage`, `htmlLanguage` or `indentUnit` | n/a |
| Symbol list | `CodeMirror6/language/src/README.md` | `ferrum/crates/ferrum-language/src/README.md`, `ferrum/crates/ferrum-language/README.md` |
| Dependencies | `@codemirror/state`, `@codemirror/view`, `@codemirror/streamparser`, `@lezer/common`, `@lezer/highlight`, `@lezer/lr`, `style-mod` | `ferrum-state`, `ferrum-view`, `regex`, `serde`, `serde_json`, `smallvec`, `unicode-segmentation` |

`@lezer/*` packages are not in the local mirror. Where a behavior depends on Lezer internals, cite the cm6 call site and describe the Lezer contract from its documentation; mark the item `HUMAN DECISION` if the contract cannot be established.

## Chunks (suggested)

1. Inventory and dispositions (all exports; `StreamLanguage`/`StringStream` get a disposition proposal for D3)
2. `language.ts`: `Language`, `LRLanguage`, `defineLanguageFacet`, `language` facet, `syntaxTree`, `ensureSyntaxTree`, `syntaxTreeAvailable`, `syntaxParserRunning`, `forceParsing`, `ParseContext` (work budget, viewport-aware parsing, `skipUntilInView`, fragments, `getSkippingParser`), `LanguageSupport`, `LanguageDescription`, `languageDataProp`, `sublanguageProp`, `DocInput`
3. `indent.ts`
4. `fold.ts`
5. `matchbrackets.ts`
6. `highlight.ts` (style model: D7; tags: D3)
7. `stream-parser.ts` + `streamparser/src`
8. `isolate.ts`
9. Test audit

## Special attention

- **Parse scheduling.** cm6 parses incrementally with a time budget, in the viewport first, and continues in the background (`ParseWorker`). Record every timing, budget and viewport behavior: they matter for large documents regardless of parser.
- **Indentation.** `getIndentation` resolution order (`indentService` first, then syntax), `IndentContext` options (`overrideIndentation`, `simulateBreak`, `simulateDoubleBreak`), `TreeIndentContext` (`baseIndent`, `baseIndentFor`, `continue`, `textAfter`), `delimitedIndent`/`continuedIndent`/`flatIndent`, `indentOnInput`, `indentUnit` validation. Ferrum's tests for indent-unit errors use `should_panic`: check against cm6 (which throws on invalid config) and against `rules.md` (no panics in production paths).
- **Folding.** `foldService`, `foldNodeProp`, `foldInside`, `foldable`, `foldState` mapping through changes, `foldEffect`/`unfoldEffect`, fold commands and keymap, `codeFolding` config, placeholder widget, `foldGutter` (model in core, drawing in surface).
- **Bracket matching.** `matchBrackets` with and without a tree, `bracketMatching` config, `bracketMatchingHandle`. The Ferrum crate README lists `match_brackets`; check whether it exists.
- **Tests outside the package.** Behaviors with no test in `CodeMirror6/language/test/` are common. Link them to `lang-*` or `commands` tests where those exist; otherwise record "no cm6 test".

## Calibration notes (not a work list; may contain errors)

- `ParseContext::work` and `ParseContext::changes` in `ferrum/crates/ferrum-language/src/tree.rs:171-186` (`work`, `changes`) ignore their arguments.
- `SyntaxNode::resolve` ignores `assoc` (`ferrum/crates/ferrum-language/src/tree.rs:54`).
- No tree-sitter dependency in `Cargo.toml`.
- `javascript.rs` defines a toy language used by tests; cm6 tests that use `@lezer/javascript` cannot be equivalent with it.
- `ferrum/crates/ferrum-language/src/README.md:37` lists `match_brackets`; no function with that name was found.
