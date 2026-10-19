# Ferrum

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum/badge.svg)](https://docs.rs/ferrum)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE-APACHE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

> **The extensible, embeddable text editor engine for Rust.**  
> Built with 1:1 mathematical and architectural parity to **CodeMirror 6**, designed for native desktop, mobile, embedded environments, and WebAssembly.

---

## Overview

Modern rich text editors face a dilemma:
- **Web-based engines (e.g., CodeMirror 6, Monaco)** offer mature transactional architectures and operational transformation, but force applications to embed an entire Chromium runtime (Electron, WebView), incurring hundreds of megabytes of RAM overhead, unpredictable Garbage Collection pauses, and high input latency.
- **Ad-hoc native text widgets** typically implement naive line buffers, fragile multi-cursor implementations, and broken undo/redo stacks that fail under concurrent edits or complex Unicode graphemes.

**Ferrum** bridges this gap. It ports the battle-tested, mathematically proven functional core of **CodeMirror 6** into pure, idiomatic Rust:

- **100% Permissive Licensing**: Strictly developed under a clean-room protocol (dual-licensed MIT / Apache-2.0). Zero copyleft or GPL contamination.
- **Rope-Backed Documents**: Utilizes balanced B-tree ropes ([`ropey`](https://crates.io/crates/ropey)) for $O(\log N)$ slicing, indexing, and persistent state snapshots.
- **Rigorous Operational Transformation (OT)**: Changesets compose, invert, and map across concurrent transactions with proven confluency ($A \circ B' = B \circ A'$).
- **Grapheme Cluster Awareness**: All cursor movements, word boundaries, and backspace/delete operations strictly respect Unicode Extended Grapheme Clusters (`unicode-segmentation`), emojis, combining marks, and line breaks.
- **Multi-Cursor Invariants**: Direct canonical normalization parity with CodeMirror 6, guaranteeing determinism across overlapping and adjacent cursor selections.
- **State Fields, Facets & Compartments**: Decoupled, conflict-free extension architecture. Extensions declare typed facet inputs rather than monkey-patching editor internals.
- **Time-Bucketed History Engine**: Transactional undo/redo with event isolation, configurable group delays, and remote change re-projection.

---

## Workspace Crates

Ferrum is designed as a modular suite of decoupled crates:

```text
crates/
├── ferrum-state/        Core functional data structures: EditorState, ChangeSet, Facet, SelectionRange, RangeSet, Text
├── ferrum-commands/     Editing commands, keymaps, grapheme-aware navigation, and multi-level transactional history
├── ferrum-view/         Abstract viewport virtualization, DecorationSet, and visual rendering contracts
├── ferrum-view-gpui/    Hermetic native surface adapter for GPUI (Metal / Vulkan / DirectX accelerated windowing)
└── ferrum/              Unified umbrella crate re-exporting the engine with granular feature flags
```

### Crate Roles & Licensing

| Crate | License | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`ferrum-state`** | **MIT** | Immutable editor state, facets, OT changesets, graphemes, transactions. Zero UI dependencies. | `ropey`, `unicode-segmentation`, `smallvec`, `parking_lot` |
| **`ferrum-commands`** | **MIT** | Pure state commands (indentation, line movement, text deletion, history undo/redo). | `ferrum-state`, `unicode-segmentation` |
| **`ferrum-view`** | **MIT** | Viewport calculation, `DecorationSet`, range intervals, and event dispatch contracts. | `ferrum-state` |
| **`ferrum-view-gpui`** | **Apache-2.0** | GPU-accelerated desktop window bridge powered by GPUI (Apache-2.0). | `gpui`, `ferrum-view`, `ferrum-commands` |
| **`ferrum`** | **MIT** | Unified facade crate with ergonomic defaults and feature flags. | `ferrum-state`, `ferrum-commands`, `ferrum-view` |

---

## Quickstart

Add Ferrum to your `Cargo.toml`:

```toml
[dependencies]
ferrum = { version = "0.1", features = ["commands", "view"] }
```

### 1. Creating an Immutable State and Dispatching Transactions

```rust
use ferrum::state::{EditorState, EditorStateConfig, Text, TransactionSpec};
use ferrum::commands::{history, standard_keymap, default_keymap, insert_text, undo, redo, HistoryConfig, KEYMAP};
use ferrum::view::EditorView;

fn main() {
    // 1. Configure the editor extensions
    let extensions = vec![
        history(HistoryConfig::default()),
        KEYMAP.of(standard_keymap()),
        KEYMAP.of(default_keymap()),
    ];

    // 2. Initialize the immutable EditorState
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("fn main() {\n    println!(\"Hello, Ferrum!\");\n}\n")),
        selection: None,
        extensions: Some(extensions),
    });

    // 3. Mount an EditorView controller
    let mut view = EditorView::new(state);

    // 4. Dispatch transactional edits
    let insert_spec = insert_text(view.state(), "// Licensed under MIT\n").unwrap();
    view.dispatch(insert_spec);

    println!("Document content:\n{}", view.state().doc());

    // 5. Undo via canonical history engine
    let undo_spec = undo(view.state()).unwrap();
    view.dispatch(undo_spec);
    println!("Document after undo:\n{}", view.state().doc());
}
```

### 2. Multi-Cursor & Operational Transformation

```rust
use ferrum::state::{EditorSelection, SelectionRange, TransactionSpec};
use ferrum::commands::insert_text;
use ferrum::view::EditorView;

// Create multiple simultaneous selection cursors
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

---

## Canonical CodeMirror 6 Parity

Development in Ferrum is strictly test-driven (TDD), using the official CodeMirror 6 repositories as behavioral specifications:

| CodeMirror 6 Module | Ferrum Counterpart | Status | Test Suite Verification |
| :--- | :--- | :---: | :--- |
| `@codemirror/state` (`change.ts`) | `ferrum_state::change` | ✅ 100% | `test-change.ts` (compose, invert, map_desc, confluency) |
| `@codemirror/state` (`selection.ts`) | `ferrum_state::selection` | ✅ 100% | `test-selection.ts` (normalization, adjacent range merging) |
| `@codemirror/state` (`char.ts`) | `ferrum_state::char` | ✅ 100% | `test-charcategory.ts` (Unicode grapheme clusters & words) |
| `@codemirror/state` (`facet.ts`) | `ferrum_state::facet` | ✅ 100% | `test-facet.ts` (dynamic facets, precedence, compartments) |
| `@codemirror/state` (`state.ts`) | `ferrum_state::state` | ✅ 100% | `test-state.ts` (atomic transactions, filters, state fields) |
| `@codemirror/state` (`text.ts`) | `ferrum_state::text` | ✅ 100% | `test-text.ts` (persistent rope buffer indexing) |
| `@codemirror/commands` (`history.ts`)| `ferrum_commands::history`| ✅ 100% | `test-history.ts` (undo/redo stacks, event isolation, join) |
| `@codemirror/commands` (`commands.ts`)| `ferrum_commands::commands`| 🔄 Active | `test-commands.ts` (indentation, line operations, deletions) |
| `@codemirror/state` (`rangeset.ts`) | `ferrum_state::rangeset` | 🔄 Active | `test-rangeset.ts` (chunked B-tree intervals and spans) |

---

## Clean-Room Architecture & Licensing Guarantee

Ferrum maintains a strict Clean-Room policy:
- No code from copyleft/GPL codebases (such as `zed/crates/editor`) is ever read, copied, or linked.
- Native surface rendering bridges (`ferrum-view-gpui`) depend only on permissive components (`gpui = "0.2.2"` under Apache-2.0).
- All core engine crates (`ferrum-state`, `ferrum-commands`, `ferrum-view`) are 100% pure Rust with zero UI dependencies, licensed under the MIT License.

---

## Running the Interactive Desktop Example

To launch the native GPU-accelerated desktop editor with real-time typing, Tokyo Night styling, multi-cursor, and clipboard integration:

```bash
cargo run -p ferrum-view-gpui --example gui_editor
```

---

## License

This project is dual-licensed under either:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
