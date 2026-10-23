# Audit scope: `ferrum-view` vs `@codemirror/view` (platform-independent part)

Use with `1-parity-audit.md` (specification audit for a new crate). Audit order: **2 of 4**. The view is rebuilt from cm6: the existing `ferrum/crates/ferrum-view/` is not read, tested or described. One exception, section "Quarry from `ferrum-commands`" below: a few items that the old code placed in `ferrum-commands` but cm6 places in `@codemirror/view`.

Versions: cm6 `@codemirror/view` 6.43.14 (`CodeMirror6/view/package.json`); Ferrum line numbers at commit `ed70610`.

This is the audit where the purpose filter matters most. cm6's view is ~12,900 lines, most of it DOM. The goal is to separate what a native, headless view model must own (`CORE`), what it must ask a surface for (`CONTRACT`), what surfaces own (`SURFACE`) and what is irrelevant (`DROP`). Read `ferrum-foundation.md` §4 first: it has a preliminary file-by-file disposition, which you must verify per export and may overturn with evidence.

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `CodeMirror6/view/src/` (all 35 `.ts` files) | none (new crate); quarry: see below |
| Tests | `CodeMirror6/view/test/`: `test-heightmap.ts`, every `webtest-*.ts` (11 files), helper `tempview.ts` | none |
| Symbol list | `CodeMirror6/view/src/README.md` | not audited |
| Dependencies | `@codemirror/state`, `crelt`, `style-mod`, `w3c-keyname` | lower crate: `ferrum-state` (public API and conformance report, `ferrum/docs/parity/state/`) |

Also in scope as **consumers** (read-only, for dispositions, consumer values and A5): which view capabilities `@codemirror/language` and `@codemirror/commands` use (grep their imports from `@codemirror/view` in `CodeMirror6/language/src/` and `CodeMirror6/commands/src/`), and how the surface crate `ferrum/crates/ferrum-view-gpui/` feeds keys and text into the engine (`ferrum/crates/ferrum-view-gpui/src/keymap_bridge.rs`), as evidence for D8 only.

## Gate 1 is the main early deliverable

1. **Dispositions for every export and every source file**, with reasons. For `CONTRACT` items, name the question asked of the surface. For `SURFACE` items, name the model or facet the core still owns (for example: gutters are drawn by the surface, but `gutter`/`GutterMarker`/`lineNumbers` configuration and marker sets are core state).
2. **Draft list of contract questions**, grouped: geometry (`coordsAtPos`, `posAtCoords`, `lineBlockAt`, `lineBlockAtHeight`, visual line boundaries, `moveVertically`), measurement cycle (`requestMeasure`, `measure`/`write` phases, `defaultLineHeight`, `defaultCharacterWidth`, content size), text direction and bidi (`textDirectionAt`, `bidiSpans`), input (key events, text input, composition, paste, drop, mouse selection), scrolling (`scrollIntoView` effect and its targets), focus. This feeds D4 and D8; it states questions, it does not design traits (the implementation sketch, `1-parity-audit.md` §16, may propose shapes marked as sketches).
3. **First-version scope**: the `DEFER` list (D10), for example tooltips, panels, dialogs, drop cursor, rectangular selection.

The human approves these before phase 2. Phase 2 covers only in-scope, non-`DROP` items.

## Chunks (suggested)

1. Inventory and dispositions (all files)
2. `decoration.ts` (+ `attributes.ts` for the style question, D7)
3. `heightmap.ts`
4. `viewstate.ts`
5. `extension.ts` (`ViewPlugin`, `ViewUpdate`, view facets)
6. `editorview.ts`: public API only; for each member, disposition and dependencies
7. `cursor.ts` + `bidi.ts` (motion and bidi ordering)
8. `keymap.ts` (+ key-related parts of `input.ts`) and the quarry evaluation below
9. decoration producers: `matchdecorator.ts`, `special-chars.ts`, `highlight-space.ts`, `active-line.ts`, `placeholder.ts`
10. models with surface drawing: `gutter.ts`, `panel.ts`, `tooltip.ts`, `dialog.ts`, `layer.ts`, `draw-selection.ts`, `rectangular-selection.ts`, `dropcursor.ts`, `scrollpastend.ts`
11. Tests: `test-heightmap.ts`, then each `webtest-*.ts` file as its own chunk

## Special attention

- **`webtest-*` files (~250 cases).** Each case gets `LOGIC-ONLY`, `NEEDS LAYOUT` or `DOM-ONLY` with a reason. `webtest-draw-decoration.ts` (~97 cases) mostly asserts DOM output, but the decoration *semantics* behind each assertion (ordering, nesting, `inclusive`, block widgets, replaced ranges, line decorations) are core: record which semantic each case pins down. These cases need a browser to produce golden values (`1-parity-audit.md` §6.1).
- **Heightmap and viewport.** `HeightOracle` estimation, line gaps, viewport margins, the `scaler` for huge documents, and the update path after changes, behavior by behavior. `test-heightmap.ts` runs headless in Node: every case yields fixtures.
- **Decorations.** `Decoration.mark/widget/replace/line` specs (`inclusive`, `inclusiveStart`, `inclusiveEnd`, `side`, `block`, `attributes`, `class`, `tagName`, `bidiIsolate`), `WidgetType` (`eq`, `updateDOM` as a concept, `estimatedHeight`, `lineBreaks`, `ignoreEvent`, `coordsAt`), point vs mark ordering, `DecorationSet` mapping. Record the side constants (`CodeMirror6/view/src/decoration.ts` `Side`, ±1e8…±6e8, and the −1/+1 adjustments for non-inclusive ends) as consumer values; they are also the values to check `RangeSet` with (next section).
- **`ViewPlugin`/`ViewUpdate`.** Plugin lifecycle (`create`, `update`, `destroy`, `docViewUpdate`), `ViewUpdate` flags (`docChanged`, `selectionSet`, `viewportChanged`, `heightChanged`, `geometryChanged`, `focusChanged`), plugin-provided decorations and their precedence. cm6 detects change by identity in many places (`update.state.facet(x) != update.startState.facet(x)`): list every such site as a hazard (`1-parity-audit.md` §9).
- **Keymap.** The `keymap` facet, `KeyBinding` (`key`, `mac`/`win`/`linux`, `run`, `shift`, `any`, `scope`, `preventDefault`, `stopPropagation`), key normalization (`w3c-keyname` names, modifier order), `Mod-` resolution per platform, multi-stroke sequences and their timeout, `runScopeHandlers`. Which part needs the surface (raw key events, platform) is a contract question (D8).
- **View API used by commands.** `moveByChar`, `moveByGroup`, `moveToLineBoundary`, `moveVertically`, `lineBlockAt`, `textDirectionAt`, `bidiSpans`, `atomicRanges`, `scrollIntoView`. List each with its `CodeMirror6/commands/src/commands.ts` call sites: the commands audit (`1d-scope-commands.md` part C) checks them against this crate.

## Capabilities to check in `ferrum-state`

`@codemirror/view` uses these state capabilities. For each, check the rebuilt `ferrum-state` public API (`1-parity-audit.md` §10) and record the result in `raw/xc.jsonl`; a missing or different one becomes a register entry against `ferrum-state`, and the view items that need it wait for it. (Register ids, where one already exists, in parentheses.)

- `ChangeDesc.touchesRange` (XC-007) and `iterChangedRanges` (XC-008).
- `RangeSet.map` and `between` with the view's side values (±1e8…±6e8, and the ±1 adjustments for non-inclusive ends) passed through without truncation; sentinels beyond those values (XC-010, XC-011).
- `RangeSet.compare` with `minPointSize`, `RangeComparator.boundChange`, shared-chunk skipping (XC-012); static `RangeSet.iter(sets, from)` returning a cursor with `goto` and `rank` (XC-013); `RangeSet.eq` (XC-014); `RangeSet.join` (XC-015); `RangeSet.spans` open-start/open-end counts and return value (XC-016); the `RangeValue.range` hook (XC-017).
- Facet machinery: `Facet.from`, field `provide`, `enables`, `FacetReader`/`Facet.reader`, `StateField.init`, `StateEffect.appendConfig`, computed facets with declared dependencies, and identity-stable facet and field values (`compare`, `compareInput`) (XC-020 … XC-025).
- `Transaction.selection` holding only an explicitly set selection (drives `ViewUpdate.selectionSet`) (XC-026); `EditorState.phrase` (XC-029); `Transaction.scrollIntoView` (XC-030).
- `SelectionRange` with `bidiLevel`, `undirectional`, automatic `assoc`; `EditorSelection.addRange`/`replaceRange`; `extend` with `assoc` (XC-036).


## Quarry from `ferrum-commands`

The old code put view-owned keymap machinery into `ferrum-commands`. These are the only existing Ferrum items this audit evaluates. Use the verdicts of `1d-scope-commands.md` part B (`REUSE`, `ADAPT`, `REWRITE`, `REMOVE`) against the behaviors specified for the corresponding cm6 view items. Write the rows to `ferrum/docs/parity/view/quarry.md`; the commands audit then marks them `MOVE: ferrum-view` and does not evaluate them again.

| Ferrum item (`ed70610`) | cm6 counterpart | First look (to be confirmed) |
|---|---|---|
| `KeyBinding` (`ferrum/crates/ferrum-commands/src/commands.rs:24`), with `new`/`with_shift` | `KeyBinding` (`CodeMirror6/view/src/keymap.ts`) | Only `key: &'static str`, `run`, `shift`; no `mac`/`win`/`linux`, `any`, `scope`, `preventDefault`, `stopPropagation`. `run` is the old `StateCommand` (`fn(&EditorState) -> Option<TransactionSpec>`), so it cannot receive the view (D5). |
| `KEYMAP` facet (`ferrum/crates/ferrum-commands/src/commands.rs:51`) | `keymap` facet (`CodeMirror6/view/src/keymap.ts`) | Concatenates inputs; no precedence handling beyond facet order, no compiled per-scope map. |
| `dispatch_key` (`ferrum/crates/ferrum-commands/src/commands.rs:55`) | `runScopeHandlers` and the keydown handling in `CodeMirror6/view/src/keymap.ts` | Exact string comparison of `binding.key`; no key normalization, no `Mod-` resolution per platform, no multi-stroke, no fallthrough rules beyond "first command returning `Some`". |
| `insert_text` (`ferrum/crates/ferrum-commands/src/commands.rs:1431`) | default text input in `CodeMirror6/view/src/input.ts` (`applyDOMChange`/`inputHandler` path, user event `input.type`) | `change_by_range` replacing each range; no user event, no `inputHandler` facet, cursor offset in `char`s. |
| key-name bridge (`ferrum/crates/ferrum-view-gpui/src/keymap_bridge.rs`, surface crate, read-only) | `w3c-keyname` key names used by `CodeMirror6/view/src/keymap.ts` | Evidence for D8 (who normalizes key names: surface or core); not a quarry item. |

The keymap *contents* (`standard_keymap`, `default_keymap`, `history_keymap`) are cm6 `@codemirror/commands` items and stay in the commands audit.

## Calibration notes (not a work list; may contain errors)

- `CodeMirror6/view/src/editorview.ts` is the largest file; split it by export ranges declared in `ferrum/docs/parity/view/progress.md`.
- `CodeMirror6/view/test/test-heightmap.ts` has 24 cases and is the only Node-runnable test file; all other cases are `webtest-*`.
- cm6 keymap handling depends on `w3c-keyname` (an npm dependency, not in the mirror); install it in the harness to read its name table.
