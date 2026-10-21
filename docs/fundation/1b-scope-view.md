# Audit scope: `ferrum-view` vs `@codemirror/view` (platform-independent part)

Use with `1-parity-audit.md`. Audit order: **2 of 4** (depends on `ferrum-state`; start the behavior audit after the state gate).

This is the audit where the purpose filter matters most. cm6's view is ~12,900 lines, most of it DOM. The goal is **not** to list what Ferrum lacks of the DOM, but to separate what a native, headless view model must own (`CORE`), what it must ask a surface for (`CONTRACT`), what surfaces own (`SURFACE`) and what is irrelevant (`DROP`). Read `ferrum-foundation.md` section 4 before starting: it has a preliminary file-by-file disposition, which you must verify per export and may overturn with evidence.

## Scope

|  | cm6 | Ferrum |
|---|---|---|
| Source | `CodeMirror6/view/src/` (all 35 `.ts` files) | `ferrum/crates/ferrum-view/src/`: `view.rs`, `viewstate.rs`, `heightmap.rs`, `decoration.rs`, `viewport.rs`, `lib.rs` |
| Tests | `CodeMirror6/view/test/`: `test-heightmap.ts`, every `webtest-*.ts` (11 files), helper `tempview.ts` | `ferrum/crates/ferrum-view/src/tests/*.rs` and doctests |
| Symbol list | `CodeMirror6/view/src/README.md` | `ferrum/crates/ferrum-view/src/README.md`, `ferrum/crates/ferrum-view/README.md` |
| Dependencies | `@codemirror/state`, `crelt`, `style-mod`, `w3c-keyname` | `ferrum-state`, `smallvec`, `thiserror` |

Also in scope as **consumers** (read-only, for A5 and dispositions): how `ferrum-view-gpui` uses `ferrum-view`, and which view capabilities `@codemirror/language` and `@codemirror/commands` use (grep their imports from `@codemirror/view`).

## Phase 1 is the main deliverable

1. **Dispositions for every export and every source file**, with reasons. For `CONTRACT` items, name the question asked of the surface. For `SURFACE` items, name the model or facet the core still owns (for example: gutters are drawn by the surface, but `gutter`/`GutterMarker`/`lineNumbers` configuration and marker sets are core state).
2. **Draft list of contract questions**, grouped: geometry (`coordsAtPos`, `posAtCoords`, `lineBlockAt`, `lineBlockAtHeight`, visual line boundaries, `moveVertically`), measurement cycle (`requestMeasure`, `measure`/`write` phases, `defaultLineHeight`, `defaultCharacterWidth`, `contentDOM` size), text direction and bidi (`textDirectionAt`, `bidiSpans`), input (key events, text input, composition, paste, drop, mouse selection), scrolling (`scrollIntoView` effect and its targets), focus. This feeds decision D4 and D8; do not design the traits.

The human approves dispositions before phase 2. Phase 2 (behavior and test audit) covers only non-`DROP` items.

## Chunks (suggested)

1. Inventory and dispositions (all files)
2. `decoration.ts` (+ `attributes.ts` for the style question, D7)
3. `heightmap.ts`
4. `viewstate.ts`
5. `extension.ts` (ViewPlugin, ViewUpdate, view facets)
6. `editorview.ts`: public API only; for each member, disposition and dependencies
7. `cursor.ts` + `bidi.ts` (motion and bidi ordering)
8. `keymap.ts` (+ key-related parts of `input.ts`)
9. decoration producers: `matchdecorator.ts`, `special-chars.ts`, `highlight-space.ts`, `active-line.ts`, `placeholder.ts`
10. models with surface drawing: `gutter.ts`, `panel.ts`, `tooltip.ts`, `dialog.ts`, `layer.ts`, `draw-selection.ts`, `rectangular-selection.ts`, `dropcursor.ts`, `scrollpastend.ts`
11. Test audit: `test-heightmap.ts`, then each `webtest-*.ts` file as its own chunk

## Special attention

- **`webtest-*` files (~250 cases).** Each case gets `LOGIC-ONLY`, `NEEDS LAYOUT` or `DOM-ONLY` with a reason. `webtest-draw-decoration.ts` (~97 cases) mostly asserts DOM output, but the decoration *semantics* behind each assertion (ordering, nesting, `inclusive`, block widgets, replaced ranges, line decorations) are core; record which semantic each case pins down.
- **Heightmap and viewport.** `heightmap.rs` and `viewstate.rs` claim to be faithful ports. Verify against `heightmap.ts`/`viewstate.ts` behavior by behavior, including `HeightOracle` estimation, line gaps, viewport margins, `scaler` for huge documents, and the update path after changes.
- **Decorations.** `Decoration.mark/widget/replace/line` specs (`inclusive`, `inclusiveStart`, `inclusiveEnd`, `side`, `block`, `attributes`, `class`, `tagName`, `bidiIsolate`), `WidgetType` (`eq`, `updateDOM` as a concept, `estimatedHeight`, `lineBreaks`, `ignoreEvent`, `coordsAt`), point vs mark ordering, `DecorationSet` mapping. Ferrum's `Decoration::mark` takes a `&'static str` tag: record it against D7.
- **`ViewPlugin`/`ViewUpdate`.** cm6's plugin lifecycle (`create`, `update`, `destroy`, `docViewUpdate`), `ViewUpdate` flags (`docChanged`, `selectionSet`, `viewportChanged`, `heightChanged`, `geometryChanged`, `focusChanged`), plugin-provided decorations and their precedence.
- **Keymap.** cm6's `keymap` facet, `KeyBinding` (`key`, `mac`/`win`/`linux`, `run`, `shift`, `any`, `scope`, `preventDefault`, `stopPropagation`), multi-stroke sequences, `Mod-` resolution. Ferrum currently has `KEYMAP` and `dispatch_key` in `ferrum-commands`: that is a placement question for this audit and the commands audit.
- **Missing view API used by commands.** `moveByChar`, `moveByGroup`, `moveToLineBoundary`, `moveVertically`, `lineBlockAt`, `textDirectionAt`, `bidiSpans`, `atomicRanges`, `scrollIntoView`. Open register entries if `ferrum-commands` needs them.

## Calibration notes (not a work list; may contain errors)

- `EditorView` (`view.rs`) exposes `new`, `with_oracle`, `state`, `viewport`, `set_viewport`, `view_state`, `view_state_mut`, `dispatch`, `visible_decorations`, `render_lines`. No plugin system, motion API, keymap or layout contract found.
- `ViewUpdate` (`view.rs` L9) has `doc_changed` and `selection_changed` only.
- `RenderedLine` (`view.rs`) materializes each line's text as a `String`.
- The view tests count 31 in Ferrum vs 24 cases in `test-heightmap.ts`; check what the extra ones are.
