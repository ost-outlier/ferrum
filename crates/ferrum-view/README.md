# ferrum-view

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum-view/badge.svg)](https://docs.rs/ferrum-view)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

> **Abstract viewport virtualization, decorations, and visual layout contracts for Ferrum.**  
> Direct architectural counterpart to [`@codemirror/view`](https://github.com/codemirror/view).

---

## Overview

`ferrum-view` manages the visual projection of an editor document. It calculates the visible viewport, organizes visual decorations (syntax highlights, diagnostic squiggles, inline widgets, line gutters) into balanced interval trees, and translates between buffer offsets and 2D coordinates.

`ferrum-view` is **pure Rust** with **zero UI or GPU dependencies**. Concrete UI frontends (such as GPUI, Slint, egui, or WebAssembly canvas) consume `ferrum-view` as their layout and virtualization contract.

## Key Features

### 1. Viewport Virtualization
- `Viewport` tracks visible line ranges (`from` to `to`) with overscan margins.
- Instead of allocating glyphs or rendering the entire document, views query only the visible slice in $O(\log N)$ time via `ropey`.

### 2. Canonical Decoration Sets (`DecorationSet`)
- Direct equivalence to CodeMirror 6: `DecorationSet` is an alias of `RangeSet<Decoration>`.
- **Interval Partitioning**: Backed by balanced B-tree chunks (`Chunk<Decoration>`) in `ferrum-state`.
- **Supported Decoration Kinds**:
  - `DecorationKind::Mark`: Inline text styling (spans, syntax tokens, search highlights).
  - `DecorationKind::Widget`: Inline custom elements embedded between characters.
  - `DecorationKind::Line`: Line-level attributes (current line highlighting, diff backgrounds).
  - `DecorationKind::Replace`: Range replacement (code folding, hidden markers).
- **$O(\log N)$ Spatial Queries**: `decorations.between(viewport.from, viewport.to, ...)` efficiently extracts only the decorations intersecting the visible viewport.
- **OT Mapping**: Maps through document edits across transactions (`decorations.map(&changes.desc())`).

### 3. EditorView & Dispatch Cycle
- Holds the canonical `Arc<EditorState>`.
- Coordinates transaction updates, notifying listeners and maintaining layout coherence.

---

## Quickstart

```rust
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_view::decoration::{Decoration, DecorationKind, DecorationSet};
use ferrum_view::view::{EditorView, Viewport};

// Initialize editor state
let state = EditorState::create(EditorStateConfig {
    doc: Some("fn main() {\n    println!(\"Hello!\");\n}\n".into()),
    selection: None,
    extensions: None,
});

// Construct an abstract EditorView
let mut view = EditorView::new(state);

// Build decorations (e.g. syntax highlight marks)
let decorations = DecorationSet::of_sorted(vec![
    Decoration::new(0, 2, DecorationKind::Mark { class: "keyword" }),
    Decoration::new(3, 7, DecorationKind::Mark { class: "function" }),
]);
view.set_decorations(decorations);

// Query decorations within the active viewport
let viewport = Viewport { from: 0, to: 15 };
let visible = view.visible_decorations(&viewport);
assert_eq!(visible.len(), 2);
```

---

## Testing & Parity

Test suite includes:
- Viewport decoration queries and clipping
- Operational transformation mapping of `DecorationSet` through document edits

Run test suite:
```bash
cargo test -p ferrum-view --lib
```

---

## License

Dual-licensed under either of:
- MIT License ([LICENSE-MIT](https://github.com/your-org/ferrum/blob/main/LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/your-org/ferrum/blob/main/LICENSE-APACHE))

Strict Clean-Room implementation: contains zero copyleft or GPL code.
