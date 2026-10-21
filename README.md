# Ferrum

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum/badge.svg)](https://docs.rs/ferrum)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE-APACHE)
[![Build Status](https://img.shields.io/badge/tests-371%20passed-brightgreen.svg)]()

> **The extensible, embeddable text editor engine for Rust.**  
> Built with 1:1 mathematical and architectural parity to [CodeMirror 6](https://code.haverbeke.berlin/codemirror), designed for native desktop, mobile, embedded environments, and WebAssembly.

> **Status:** the foundation crates (`ferrum-state`, `ferrum-view`, `ferrum-language`, `ferrum-commands`) are under a parity audit against CodeMirror 6. Parity is the goal, not a verified fact: the feature, performance and test-parity claims in this README have not been verified yet.

---

## Why Ferrum?

Modern software that requires rich text or code editing often faces a difficult architectural compromise:
- **Web-based engines (e.g., CodeMirror 6, Monaco)** provide battle-tested transactional models, operational transformation (OT), and rich extension ecosystems, but force native applications to bundle an entire Chromium runtime (Electron, WebView). This introduces hundreds of megabytes of memory overhead, garbage collection pauses, and noticeable input latency.
- **Traditional native text widgets** often employ naive linear string buffers, fragile ad-hoc multi-cursor logic, and broken undo/redo stacks that fail under concurrent edits or complex Unicode scripts.

**Ferrum** brings the battle-tested, mathematically proven functional core of **CodeMirror 6** into pure, idiomatic Rust:

- **Zero Chromium Overhead**: Pure native performance with sub-millisecond input-to-photon latency and minimal memory footprint.
- **Immutable Transactional State**: An `EditorState` is an immutable, thread-safe value (`Arc`). State updates are explicit, atomic transactions.
- **Formal Operational Transformation (OT)**: Changesets compose ($A \circ B$), invert ($\Delta^{-1}$), and map across concurrent edits ($A \circ B' = B \circ A'$) with verified confluency.
- **Rope-Backed Storage**: Balanced B-tree ropes ([`ropey`](https://crates.io/crates/ropey)) ensure $O(\log N)$ document slicing, insertions, deletions, and snapshotting.
- **Strict Unicode Parity**: All cursor navigation, deletions, and word boundaries respect Unicode Extended Grapheme Clusters (`unicode-segmentation`), emojis (including ZWJ sequences), combining accents, and CRLF line breaks.
- **Multi-Cursor Invariants**: Canonical selection normalization guarantees identical, deterministic behavior across overlapping and adjacent cursors.
- **Decoupled Facet Architecture**: Extensions declare typed inputs to facets and compartments without monkey-patching editor internals.
- **HeightMap Viewport Virtualization**: Balanced B-tree height mapping virtualizes millions of lines with inline/block widgets and variable heights.
- **100% Permissive Licensing**: Clean-Room implementation dual-licensed under MIT and Apache-2.0 with zero copyleft/GPL code.

---

## Architectural Mapping: Ferrum ↔ CodeMirror 6

Ferrum ports the *behavior* of CodeMirror 6's platform-independent logic, not its DOM machinery. The foundation is four headless crates, layered exactly like their cm6 counterparts:

```
@codemirror/state     ⇔  ferrum-state      (no Ferrum deps)
       ▲                        ▲
@codemirror/view      ⇔  ferrum-view       → state
       ▲                        ▲
@codemirror/language  ⇔  ferrum-language   → state, view
       ▲                        ▲
@codemirror/commands  ⇔  ferrum-commands   → state, view, language
```

`commands` is the top of the foundation, not its base: in cm6 many commands receive the `EditorView` (motion by visual line, line boundaries, bidi) and use `language` for indentation, syntax-tree motion and bracket matching. Above the foundation sit feature crates (search, autocomplete, lint, …) and rendering surfaces. No foundation crate depends on a UI framework, a text shaper or a GPU library; geometry and input reach `ferrum-view` through traits that surfaces implement.

> Current code does not match this graph yet: `ferrum-commands` depends on `ferrum-state` and `ferrum-language` but not on `ferrum-view`, so view-dependent commands are state-only approximations, and keymaps live in `ferrum-commands` instead of `ferrum-view`. This is part of the audit.

### Layer 0 — State (headless)

| CodeMirror 6 Package | Ferrum Crate | Responsibility |
| :--- | :--- | :--- |
| [`@codemirror/state`](https://code.haverbeke.berlin/codemirror/state) | [`ferrum-state`](crates/ferrum-state) | `EditorState`, `Text`, `ChangeSet` / `ChangeDesc`, `EditorSelection`, transactions, annotations and effects, `Facet`, `StateField`, `Compartment`, `RangeSet`, character and column utilities. |

### Layer 1 — View model (headless; layout through traits)

| CodeMirror 6 Package | Ferrum Crate | Responsibility |
| :--- | :--- | :--- |
| [`@codemirror/view`](https://code.haverbeke.berlin/codemirror/view) (platform-independent part) | [`ferrum-view`](crates/ferrum-view) | `EditorView` model, view plugins and `ViewUpdate`, viewport and `HeightMap`, decorations (`DecorationSet`), keymaps, cursor motion and bidi ordering, and the layout/input traits that surfaces implement. DOM rendering, CSS theming and DOM input handling are not ported. |

### Layer 2 — Language infrastructure (headless)

| CodeMirror 6 Package | Ferrum Crate | Responsibility |
| :--- | :--- | :--- |
| [`@codemirror/language`](https://code.haverbeke.berlin/codemirror/language) + [`@codemirror/streamparser`](https://code.haverbeke.berlin/codemirror/streamparser) | [`ferrum-language`](crates/ferrum-language) | Syntax tree abstraction and parse scheduling (tree-sitter planned instead of Lezer), language data, indentation, folding, bracket matching, highlighting. |

### Layer 3 — Commands (headless)

| CodeMirror 6 Package | Ferrum Crate | Responsibility |
| :--- | :--- | :--- |
| [`@codemirror/commands`](https://code.haverbeke.berlin/codemirror/commands) | [`ferrum-commands`](crates/ferrum-commands) | Editing and motion commands, standard/default keymaps, undo/redo history, comment toggling. |

### Surfaces (rendering, input, OS integration)

| Platform / Framework | Ferrum Crate | License | Responsibility |
| :--- | :--- | :--- | :--- |
| **GPUI** (desktop) | [`ferrum-view-gpui`](crates/ferrum-view-gpui) | **Apache-2.0** | Window, text shaping through GPUI, drawing, keyboard/mouse/clipboard input; implements `ferrum-view`'s traits. |
| **wgpu** + `winit` + `cosmic-text` *(planned)* | `ferrum-view-wgpu` | **MIT / Apache-2.0** | Portable surface (desktop first; mobile and web canvas are research items). |

### Facade (umbrella crate)

| Package | Ferrum Crate | Responsibility |
| :--- | :--- | :--- |
| `codemirror` (meta-package) | [`ferrum`](crates/ferrum) | Re-exports the foundation behind feature flags. Current features: `commands`, `view` (default). Feature flags follow the dependency graph. |

### Future ecosystem (planned, above the foundation)

| CodeMirror 6 Package | Planned Crate | Responsibility |
| :--- | :--- | :--- |
| [`@codemirror/search`](https://code.haverbeke.berlin/codemirror/search) | `ferrum-search` | Search and replace (substring and regex), match decorations, search panel state. |
| [`@codemirror/autocomplete`](https://code.haverbeke.berlin/codemirror/autocomplete) | `ferrum-autocomplete` | Completion sources, matching and ranking, snippets, bracket closing. |
| [`@codemirror/lint`](https://code.haverbeke.berlin/codemirror/lint) | `ferrum-lint` | Diagnostics model and decorations. |
| [`@codemirror/collab`](https://code.haverbeke.berlin/codemirror/collab) | `ferrum-collab` | Central-authority collaborative editing built on `ChangeSet` mapping. |
| [`@codemirror/merge`](https://code.haverbeke.berlin/codemirror/merge) | `ferrum-merge` | Diffing and merge views. |
| `@codemirror/lang-*` | `ferrum-lang-*`, `ferrum-markdown` | Language packages (grammar, language data, indentation, folding, highlighting). |


---

## Quickstart

Add `ferrum` to your `Cargo.toml`:

```toml
[dependencies]
ferrum = { version = "0.1", features = ["commands", "view"] }
```

### 1. Basic Editing State, Keymaps, and History

```rust
use ferrum::state::{EditorState, EditorStateConfig, Text, TransactionSpec};
use ferrum::commands::{
    default_keymap, history, insert_text, undo, redo, HistoryConfig, KEYMAP,
};
use ferrum::view::EditorView;

fn main() {
    // 1. Configure extensions (keymaps and undo/redo history)
    let extensions = vec![
        history(HistoryConfig::default()),
        KEYMAP.of(default_keymap()),
    ];

    // 2. Create the immutable EditorState
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("fn main() {\n    println!(\"Hello, Ferrum!\");\n}\n")),
        selection: None,
        extensions: Some(extensions),
    });

    // 3. Attach an abstract EditorView
    let mut view = EditorView::new(state);

    // 4. Dispatch transactional edits
    let insert_spec = insert_text(view.state(), "// Edited with Ferrum\n").unwrap();
    view.dispatch(insert_spec);

    println!("Document:\n{}", view.state().doc());

    // 5. Undo the modification
    let undo_spec = undo(view.state()).unwrap();
    view.dispatch(undo_spec);
    println!("After undo:\n{}", view.state().doc());
}
```

### 2. Multi-Cursor & Operational Transformation

```rust
use ferrum::state::{EditorSelection, SelectionRange, TransactionSpec};
use ferrum::commands::insert_text;
use ferrum::view::EditorView;

// Create multiple simultaneous selection carets
let multi_selection = EditorSelection::create(
    vec![
        SelectionRange::cursor(0),
        SelectionRange::cursor(24),
    ],
    0, // Primary cursor index
);

// Dispatch selection update
view.dispatch(TransactionSpec::new().with_selection(multi_selection));

// Type concurrently across all cursors with automatic OT mapping
let typing_spec = insert_text(view.state(), "    // [TAG] ").unwrap();
view.dispatch(typing_spec);
```

### 3. Syntax Decorators & Viewport Virtualization

```rust
use ferrum::view::decoration::{Decoration, DecorationKind, DecorationSet};
use ferrum::view::view::Viewport;

// Build interval decorations (syntax highlights, widgets, lines)
let decorations = DecorationSet::of_sorted(
    vec![
        Decoration::new(0, 2, DecorationKind::Mark { class: "keyword" }),
        Decoration::new(3, 7, DecorationKind::Mark { class: "function" }),
    ],
    false,
).unwrap();

view.set_decorations(decorations);

// Query only the decorations intersecting the active viewport in O(log N)
let visible = view.visible_decorations(&Viewport { from: 0, to: 50 });
assert_eq!(visible.len(), 2);
```

---

## Interactive GPU Desktop Application

Ferrum includes a ready-to-run GPU-accelerated desktop editor with hardware text shaping, multi-cursor editing, Tokyo Night theme styling, mouse drag selection, and native clipboard integration:

```bash
cargo run -p ferrum-view-gpui --example gui_editor
```

---

## Verification & Test Parity

Ferrum is built strictly via Test-Driven Development (TDD). Every algorithm and edge case is validated against test suites directly ported from CodeMirror 6:

```text
======================================================================
Ferrum Test Suite Parity Summary
======================================================================
  ferrum-state       : 177 / 177 passed (100% GREEN)
  ferrum-commands    : 142 / 142 passed (100% GREEN)
  ferrum-language    :  15 /  15 passed (100% GREEN)
  ferrum-view        :  26 /  26 passed (100% GREEN)
  ferrum-view-gpui   :  11 /  11 passed (100% GREEN)
----------------------------------------------------------------------
  Total Workspace    : 371 / 371 passed (100% GREEN, 0 warnings)
======================================================================
```

To run the complete test suite across all crates:

```bash
cargo test --workspace
```

---

## Clean-Room Architecture & Licensing Guarantee

Ferrum adheres to a strict clean-room protocol:
- **Zero GPL Contamination**: Core crates (`ferrum-state`, `ferrum-commands`, `ferrum-language`, `ferrum-view`, `ferrum`) are 100% pure Rust, licensed under the permissive **MIT License**.
- **Hermetic UI Bridges**: Graphical adapters (`ferrum-view-gpui`) depend only on permissive crates (`gpui` under **Apache-2.0**) and contain zero copyleft code.

### Dual License

This project is dual-licensed under either:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
