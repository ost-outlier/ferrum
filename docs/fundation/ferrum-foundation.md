# Ferrum: purpose, architecture and foundation

This is the reference document for anyone (human or AI) working on Ferrum. It explains what Ferrum is, how it relates to CodeMirror 6 (cm6), how the four foundation crates relate to each other, which decisions must be taken before the foundation is rebuilt, and what comes after.

Precedence between documents, and what to do when two disagree, is defined once in `rules.md` §0: approved decisions first, then `rules.md`, with phase briefs adding to it but never relaxing it; a conflict is reported, not resolved by choosing. `README.md`, crate READMEs, doc comments and `../ferrum_strategic_architecture_vision.md` (outside the repository) are **claims**, not facts; the vision document in particular is aspirational and contains exaggerations and errors (listed in section 9).

Anything marked *preliminary* below comes from a quick look, not from an audit. The audits decide. The workflow is: specification audit (`1-parity-audit.md` and the scope sheets), implementation (`2-parity-implementation.md`; for `ferrum-state`, `2a-implementation-state.md`), conformance audit (`3-conformance-audit.md`).

---

## 1. What Ferrum is, and what it is not

**Ferrum is a native, embeddable text editor engine in Rust whose core reproduces the architecture and the observable behavior of CodeMirror 6, without a browser.**

- **Engine, not application.** Ferrum gives an application a document model, a transactional state with an extension system, a view model (viewport, decorations, heights, cursor motion), language infrastructure (syntax tree, indentation, folding, highlighting) and editing commands. The application provides the window, the menus and the rest of its UI.
- **Headless core, pluggable surfaces.** The core never draws, never measures text and never talks to the OS. Drawing, text shaping, input (keyboard, mouse, IME) and accessibility live in *surface* crates (GPUI first, a `wgpu` surface later). The core asks the surface for geometry through a Rust trait.
- **cm6 is the specification of behavior, not a template for code.** Ferrum ports what cm6 *does* (its algorithms, invariants, observable results and tests), not how cm6 talks to the DOM.

Ferrum is **not**:

- a line-by-line translation of cm6 to Rust. Everything in cm6 that exists because of the DOM, CSS or browser quirks is out of scope (see section 3);
- a DOM or WebView editor. If an editor will run inside a WebView, cm6 itself is the right tool. A future DOM surface would be a separate, decoupled crate, and is not a short- or medium-term goal;
- a UI toolkit (no tabs, dialogs, file trees);
- a PKM application. A note-taking app is a *consumer* of Ferrum, not part of the engine.

### The idea behind it

Applications that need serious text editing (IDEs, Markdown and note-taking apps, consoles, configuration editors) today choose between:

- embedding cm6 or Monaco in Electron or a WebView, which brings a browser runtime, memory overhead, startup cost and input latency; or
- native text widgets, which lack cm6's transactional state, extension model, multi-cursor invariants, decoration system and language tooling.

Ferrum aims to close that gap for Rust: cm6's proven model (immutable state, transactions, facets, decorations, viewport virtualization, language infrastructure, commands) available natively, with native rendering. The first concrete consumer in mind is a native Markdown/PKM editor (Obsidian-like), which shapes the short-term priorities: Markdown, large documents, fast startup, desktop first.

---

## 2. Three kinds of fidelity to cm6

Every cm6 item falls into one of three kinds of fidelity. This is the filter that keeps Ferrum from becoming "cm6 converted to Rust, DOM included".

| Kind | Meaning | How parity is measured | Examples |
|---|---|---|---|
| **Behavioral parity** | The logic is platform-independent. Ferrum reproduces it exactly. | cm6 tests ported with identical scenarios and expected values; optional differential oracle against cm6 running in Node. | `Text`, `ChangeSet`, `EditorSelection`, `RangeSet`, facets, transactions, history, comment toggling, heightmap, bidi ordering, keymap resolution logic |
| **Contract parity** | In cm6 the logic asks the DOM (geometry) or Lezer (syntax). Ferrum defines a Rust trait for that question and reproduces the logic on top of it. | Tests run against a fake implementation of the trait (fixed-width layout, hand-built trees) and reproduce cm6's expected results where the scenario permits. | `moveVertically`, `moveToLineBoundary` (visual lines), `posAtCoords`, `coordsAtPos`, syntax-driven indentation, folding, bracket matching |
| **Not ported** | Exists only because of the DOM, CSS or browsers. | One-line reason; note whether a surface crate must offer a native equivalent. | DOM reconciliation (`docview`, `tile`, `buildtile`), `MutationObserver` handling (`domobserver`, `domchange`, `domreader`), `style-mod` theming, `EditContext`, browser workarounds |

Idiomatic Rust is welcome when observable behavior is identical (see `rules.md`).

---

## 3. cm6 packages and their real dependencies

The four foundation packages of cm6 depend on each other like this (verified in each `package.json` and in the imports of the sources):

```
@codemirror/state      (no cm6 deps; uses @marijn/find-cluster-break)
      ▲
@codemirror/view       → state        (+ DOM helpers: crelt, style-mod, w3c-keyname)
      ▲
@codemirror/language   → state, view  (+ @lezer/common, @lezer/highlight, @lezer/lr)
      ▲
@codemirror/commands   → state, view, language  (+ @lezer/common)
```

`commands` is the **top** of the foundation, not part of the base:

- many cm6 commands have type `Command` (they receive an `EditorView`), not `StateCommand`. Cursor motion goes through `view.moveByChar`, `view.moveByGroup`, `view.moveVertically`, `view.moveToLineBoundary`, `view.lineBlockAt`, `view.textDirectionAt`, atomic ranges and `EditorView.scrollIntoView`;
- `KeyBinding` and the keymap machinery live in `view`;
- indentation, `syntaxTree`, `matchBrackets` and the syntax-node commands come from `language`;
- `history` uses `view` for its keymap only; `comment` uses only `state` (comment tokens come through `EditorState.languageDataAt`).

`language` depends on `view` because folding, bracket matching and highlighting produce decorations and view plugins.

### Size, to calibrate effort

| cm6 package | Source (TS lines) | Unit tests (`it`) | Browser tests (`webtest-*`) |
|---|---|---|---|
| state | ~3,900 | 172 | none |
| view | ~12,900 (most of it DOM) | 24 (`test-heightmap.ts`) | ~250 across 11 files |
| language | ~2,700 (+ `@lezer/common`) | 13 | none |
| commands | ~1,750 | 125 | 57 |

`language` is thinly tested in its own package. Much of its behavior is exercised in `lang-*` packages (`mirrors/cm6/lang-javascript/test/test-indent.ts`, `mirrors/cm6/lang-markdown/test/test-commands.ts`) and in `commands` tests that load `@lezer/javascript` or `lang-html`.

---

## 4. Ferrum's architecture

### 4.1 Target crate graph

```
                         ┌───────────────────────────────┐
                         │ applications (PKM app, IDE …) │
                         └──────────────┬────────────────┘
                                        │
              ┌─────────────────────────┼──────────────────────────┐
              │                         │                          │
   ┌──────────▼─────────┐    ┌──────────▼──────────┐    ┌──────────▼──────────┐
   │ ferrum-view-gpui   │    │ ferrum-view-wgpu    │    │ other surfaces      │
   │ (surface, desktop) │    │ (surface, planned)  │    │ (TUI, test surface) │
   └──────────┬─────────┘    └──────────┬──────────┘    └──────────┬──────────┘
              │ implements the layout / input contracts of ferrum-view
              └─────────────────────────┼──────────────────────────┘
                                        │
   feature crates (later): search, autocomplete, lint, collab, merge, lang-*, markdown
                                        │
 ─────────────────────── foundation (headless, no UI deps) ─────────────────────
                                        │
                              ┌─────────▼─────────┐
                              │ ferrum-commands   │  → state, view, language
                              └─────────┬─────────┘
                              ┌─────────▼─────────┐
                              │ ferrum-language   │  → state, view (+ syntax backend, e.g. tree-sitter)
                              └─────────┬─────────┘
                              ┌─────────▼─────────┐
                              │ ferrum-view       │  → state (layout/input via traits)
                              └─────────┬─────────┘
                              ┌─────────▼─────────┐
                              │ ferrum-state      │  → no Ferrum deps (ropey, unicode-segmentation)
                              └───────────────────┘
```

Arrows point to dependencies. No foundation crate may depend on a crate above it, and none may depend on a UI framework, a windowing library, a text shaper (`cosmic-text`, HarfBuzz bindings) or a GPU library.

The facade crate `ferrum` re-exports the foundation behind feature flags. Feature flags must follow the dependency graph (enabling `commands` implies `view` and `language`).

### 4.2 What each foundation crate owns

**`ferrum-state`** ⇔ `@codemirror/state`. Behavioral parity, all of it.
`Text`, `Line`, `ChangeSet`/`ChangeDesc`/`MapMode`, `EditorSelection`/`SelectionRange`, `EditorState`, `Transaction`, `Annotation`/`AnnotationType`, `StateEffect`/`StateEffectType`, `Facet`, `StateField`, `Compartment`, `Prec`, extension resolution, `RangeSet`/`RangeValue`/`RangeSetBuilder`, `findClusterBreak`, `countColumn`/`findColumn`, `CharCategory`, plus the built-in facets (`allowMultipleSelections`, `tabSize`, `lineSeparator`, `readOnly`, `phrases`, `languageData`, `changeFilter`, `transactionFilter`, `transactionExtender`) and `toJSON`/`fromJSON`.

**`ferrum-view`** ⇔ the platform-independent part of `@codemirror/view`. Mixed: behavioral parity for logic, contract parity for geometry, nothing of the DOM. A preliminary file-by-file disposition (the view audit must produce the final one, per export):

| cm6 `view/src` file | Disposition |
|---|---|
| `decoration.ts` | core (decoration model, `WidgetType` as a trait, `DecorationSet`) |
| `heightmap.ts`, `viewstate.ts` | core (height estimation, viewport, line gaps, scaling) |
| `extension.ts` | core (`ViewPlugin`, `ViewUpdate`, view facets such as `decorations`, `atomicRanges`, `editable`, `scrollMargins`) |
| `keymap.ts` | core logic (key normalization, scopes, multi-stroke, platform modifiers); key names from the surface |
| `bidi.ts` | core (cm6 computes bidi order itself; pure algorithm) |
| `cursor.ts`, parts of `editorview.ts` | contract: motion logic in core, geometry through the layout trait |
| `matchdecorator.ts`, `special-chars.ts`, `highlight-space.ts`, `active-line.ts`, `placeholder.ts` | core logic producing decorations; widget drawing is surface |
| `gutter.ts`, `panel.ts`, `tooltip.ts`, `dialog.ts`, `layer.ts`, `draw-selection.ts`, `rectangular-selection.ts`, `dropcursor.ts`, `scrollpastend.ts` | split: model/state/facets in core, drawing in surface. Several are deferrable. |
| `theme.ts`, `attributes.ts` | replaced by a typed style model (decision D7); CSS not ported |
| `docview.ts`, `tile.ts`, `buildtile.ts`, `dom.ts`, `domchange.ts`, `domobserver.ts`, `domreader.ts`, `browser.ts`, `editcontext.d.ts`, DOM parts of `input.ts` | not ported. The *semantics* of input events (what a key press, paste, drop or composition does to the state) become a surface contract. |

**`ferrum-language`** ⇔ `@codemirror/language`. Contract parity built on a Ferrum syntax abstraction, because cm6's language package is built on `@lezer/common` (`Tree`, `SyntaxNode`, `NodeType`, `NodeProp`, incremental parsing with fragments, mixed-language parsing) and on `@lezer/highlight` tags. Ferrum needs its own tree abstraction with a backend (tree-sitter is the stated choice, see D3). Scope: `Language`/`LanguageSupport`/`LanguageDescription`, the `language` facet, `syntaxTree`/`ensureSyntaxTree`/`forceParsing`, parse scheduling, language data, indentation (`indentService`, `indentUnit`, `IndentContext`, `getIndentation`, `indentRange`, node-based indentation strategies, `indentOnInput`), folding (`foldService`, `foldState`, fold effects and commands, fold gutter model), bracket matching, highlighting (style model), bidi isolates (`isolate.ts`). `StreamLanguage` is a separate decision (D3).

**`ferrum-commands`** ⇔ `@codemirror/commands` (`commands.ts`, `history.ts`, `comment.ts`). Behavioral parity for state-only commands, contract parity for view- and syntax-dependent ones. It must depend on `ferrum-view` and `ferrum-language`, and must expose cm6's two command kinds (`StateCommand` and a view-receiving `Command`, decision D5).

### 4.3 Where text layout lives

cm6 never lays out text; the browser does. In Ferrum:

- `ferrum-view` declares what it needs to know (line heights, visual line breaks, position ↔ coordinates, visual line boundaries) as a trait;
- a surface implements that trait using its text system: GPUI's text system for `ferrum-view-gpui`, `cosmic-text` (font discovery, shaping, bidi, wrapping, hit-testing) for `ferrum-view-wgpu`;
- tests implement it with a deterministic fake (for example, fixed-width cells and fixed line heights), which is how cm6's `webtest-*` scenarios that need layout can be reproduced headlessly.

`cosmic-text` therefore belongs to a surface (or a shared `ferrum-text-layout` helper crate used by surfaces), never to the foundation.

---

## 5. State of the code

- **`ferrum-state`**: audited (`docs/parity/state/`); the existing code is structurally different from cm6 in its facet model, transaction pipeline and selection model, and is rebuilt (§7, `2a-implementation-state.md`).
- **`ferrum-view`, `ferrum-language`**: not audited; rebuilt from their specification audits.
- **`ferrum-commands`**: not audited; its code is evaluated item by item as a quarry (`1d-scope-commands.md`).
- **Documentation**: rewritten per crate after its conformance report is approved. Licensing (MIT in the manifests, MIT/Apache-2.0 in the README, no license files, no cm6 notice) is decision D9.

Audit results and the history of the method: `LEARNINGS.md`.

---

## 6. Foundational decisions (before any rebuild)

These decisions shape the whole foundation. Fixing code before taking them risks doing the work twice. The audits collect evidence for them; a human decides and records the result in `docs/parity/decisions.md`.

**D1. Position unit.** What does an integer position mean?
- UTF-8 bytes: native to Rust strings, `regex`, tree-sitter (which also needs row/column points), `cosmic-text` and GPUI. Requires care never to land inside a code point.
- Code points (current): convenient for `ropey`, but every boundary with tree-sitter, shapers and regex needs conversion.
- UTF-16 code units: identical numbers to cm6 tests and to the LSP default, foreign to every Rust library involved.
For ASCII-only tests all three give identical numbers, so the choice mostly affects non-ASCII tests and integration costs. *Recommendation: UTF-8 bytes, with explicit conversion functions to UTF-16 (LSP, IME) and documented translation of non-ASCII cm6 test values.*

**D2. Line breaks and the `Text` model.** cm6 splits input on `\r\n?|\n` (or on `lineSeparator`), stores lines without separators, counts every line break as one position, and reinserts the separator on output (`state.lineBreak`, `sliceDoc`). *Recommendation: same semantics. Normalize on input, configure `ropey` so only `\n` counts as a line break (disable `unicode_lines` and `cr_lines`), keep the original separator in state for output.*

**D3. Syntax backend and tree abstraction.** Which Ferrum types replace `Tree`, `SyntaxNode`, `NodeType`, `NodeProp`? How are tree-sitter grammars wrapped? How are cm6 node-prop-based strategies (indentation, folding, bracket matching, language data) expressed: Rust node props, tree-sitter queries (`indents.scm`, `folds.scm`, `highlights.scm`), or both? What replaces `@lezer/highlight` tags? Is `StreamLanguage` kept (useful for simple modes, used heavily by cm6's `legacy-modes`)? Is parsing synchronous with a time budget (cm6) or off-thread? Note that tree-sitter is C, which complicates `wasm32-unknown-unknown`.

**D4. Layout contract.** The exact trait `ferrum-view` uses to ask a surface about geometry, and its unit (logical pixels as `f32`?). Who computes bidi order: Ferrum (port of `bidi.ts`, as cm6 does) or the shaper?

**D5. Command type.** cm6 has `StateCommand = (target: {state, dispatch}) => boolean` and `Command = (view) => boolean`. Ferrum's current `StateCommand` returns `Option<TransactionSpec>`. Decide both signatures, and how view-dependent commands run in headless tests.

**D6. Ownership and threading.** `Arc` + `Send + Sync` everywhere (current) or not; whether parsing or indexing may run on other threads; how determinism is kept in tests.

**D7. Style model.** cm6 decorations and themes carry CSS classes and attributes. Ferrum needs a typed replacement (style tags resolved by a theme facet, or typed attributes) that surfaces understand.

**D8. Input, IME and accessibility.** Which crate owns the semantics of text input and composition (cm6: `input.ts`, `domchange.ts`, `webtest-composition.ts`)? Accessibility (cm6 gets it from the DOM) needs a native story (for example AccessKit) in surfaces; the view core must expose what they need.

**D9. Licensing.** MIT only or MIT/Apache-2.0 for the foundation; add the license files. A Rust port of cm6 is a derived work of MIT-licensed code: keep cm6's copyright notice. See `rules.md` for the Zed and markdown-oxide rules.

**D10. Short-term scope.** Which cm6 features the foundation must have before the first surface is rebuilt (for example: no tooltips/panels/dialogs yet, gutters model only, no `StreamLanguage`).

---

## 7. Approach (decided by the maintainer, 2026-10-21)

Rebuild in place, crate by crate, in dependency order. The existing code is not a base: each crate is specified from cm6, implemented from the specification, and checked by a conformance audit. The one exception is `ferrum-commands`, whose existing code is evaluated item by item as a quarry. Record this in `docs/parity/decisions.md` together with D1–D10.

| Crate | Specification | Implementation | Existing code |
|---|---|---|---|
| `ferrum-state` | `docs/parity/state/spec/`, produced by `2a-implementation-state.md` §1 (Phase 0) | `2a-implementation-state.md` | not a base; may be consulted only for the algorithms the oracle confirmed (`2a` §4) |
| `ferrum-view` | `1-parity-audit.md` + `1b-scope-view.md` | `2-parity-implementation.md` | not read, except the view-owned keymap items in `ferrum-commands` (`1b` quarry) |
| `ferrum-language` | `1-parity-audit.md` + `1c-scope-language.md` | `2-parity-implementation.md` | not read |
| `ferrum-commands` | `1d-scope-commands.md` (specification, quarry, feedback) | `2-parity-implementation.md` | quarry: `REUSE`/`ADAPT` code and `KEEP`/`FIX` tests may be kept |
| `ferrum-view-gpui` | none yet | after the foundation | frozen; later a thin adapter implementing the layout and input contracts |
| `ferrum` (facade) | none | re-exports only (`rules.md` §2.4) | updated when the crates it re-exports change |

Take the decisions first: D1 and D2 touch almost every position computation, so implementing before deciding them is wasted work.

---

## 8. Beyond the foundation

Order is roughly by dependency and by what the first consumer (a native Markdown/PKM editor) needs.

| Area | Crate(s) | cm6 counterpart | Notes |
|---|---|---|---|
| Surfaces | `ferrum-view-gpui`, `ferrum-view-wgpu` | DOM | GPUI: desktop only, fastest path to a real window. `wgpu` + `winit` + `cosmic-text`: portable surface (desktop, possibly mobile and web canvas). Both implement the same contracts. A cheap **TUI surface** (monospace) or a **test surface** is a good way to validate the layout contract early. |
| Search | `ferrum-search` | `@codemirror/search` | Query model, matching, replace, highlight decorations; panel UI is surface. |
| Autocomplete | `ferrum-autocomplete` | `@codemirror/autocomplete` | Sources, matching, ranking, snippets, close brackets; tooltip drawing is surface. |
| Lint | `ferrum-lint` | `@codemirror/lint` | Diagnostics model and decorations. |
| Languages | `ferrum-lang-*`, `ferrum-markdown` | `lang-*` | Grammar + language data + indentation/folding/highlight configuration. Markdown first. Live preview (hiding syntax, rendering widgets) is built from decorations and widgets. |
| Collaboration | `ferrum-collab` | `@codemirror/collab` | cm6 collab is central-authority OT via `ChangeSet` mapping. CRDT integration (Yjs, Automerge) is a different model and a separate decision, not part of a cm6 port. |
| Merge/diff | `ferrum-merge` | `@codemirror/merge` | Diff algorithm is portable; side-by-side view is mostly surface. |
| LSP | `ferrum-lsp-client` | `@codemirror/lsp-client` | Completion, hover, diagnostics, formatting through LSP; needs UTF-16 conversion (D1). |
| PKM | outside the engine | none | Vault indexing, wikilinks, backlinks, tags, frontmatter. It does not need the editor engine and should not depend on it: a sibling library or part of the application. `markdown-oxide` (Apache-2.0) is a reference. |
| Facade | `ferrum` | `codemirror` | Re-exports and presets (a `basic_setup` equivalent) behind feature flags. |

Other possibilities, none of them short-term goals:

- **Mobile.** Either the `wgpu` surface on Android/iOS (input, IME and accessibility via `winit` are weak points) or a native UI with Ferrum as the logic core (hard: native text views keep their own text storage and IME state). Both need research before any claim.
- **Web.** A canvas surface through `wgpu`/WebGPU is possible; tree-sitter on `wasm32-unknown-unknown` is a known difficulty. A DOM surface would be a separate project.
- **Other GUI toolkits.** Any toolkit (iced, egui, Slint) can host Ferrum by implementing the surface contracts.
- **Headless uses.** Server-side collaboration authority, scripted refactoring, formatting, testing other editors' behavior.

---

## 9. Corrections to `ferrum_strategic_architecture_vision.md` and the README

- "Block 0: state + commands" and the README's "Layer 1 commands, Layer 2 language, Layer 3 view": wrong. The order is state → view → language → commands (section 3).
- "1:1 mathematical parity", "100% GREEN", test counts as proof of parity: unproven. Test counts say nothing about items that were never ported.
- "Formal operational transformation with verified confluency": cm6's `ChangeSet` provides composition, inversion and position mapping, and `collab` builds rebasing on it. No formal verification exists in Ferrum.
- "120 FPS", "sub-millisecond input-to-photon latency": unmeasured.
- "Tree-sitter costs 5–10 MB of WASM per language, so cm6 invented Lezer": an oversimplification; the choice of tree-sitter for Ferrum stands on its own (native speed, existing grammars) but has costs of its own (C dependency, no Lezer node props or highlight tags, byte/point-based edits; see D3).
- `ferrum-pkm` as a block of the engine: it is a consumer (section 8).
- "Mobile approach A (native UI) recommended": no evidence yet; research item.
- "`ferrum-collab` with native CRDT integration (Yjs/Automerge)": not what cm6's collab is.
- Licensing: the vision says Zed is GPL/AGPL. Zed is per-crate: `gpui`, `sum_tree`, `collections`, `util` are Apache-2.0; `editor`, `text`, `rope`, `language`, `multi_buffer` are GPL-3.0-or-later. See `rules.md`.
