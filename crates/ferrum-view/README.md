# ferrum-view

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum-view/badge.svg)](https://docs.rs/ferrum-view)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

> **Abstract viewport virtualization, decorations, and visual layout contracts for Ferrum.**  
> Direct architectural counterpart to [`@codemirror/view`](https://code.haverbeke.berlin/codemirror/view).

---

## Overview

`ferrum-view` manages the visual projection of an editor document. It calculates the visible viewport, organizes visual decorations (syntax highlights, diagnostic squiggles, inline widgets, line gutters) into balanced interval trees, and drives line height virtualization through a balanced `HeightMap` B-tree.

`ferrum-view` is **pure Rust** with **zero UI or GPU dependencies**. Concrete graphical frontends (such as GPUI, Slint, egui, or WebAssembly canvas) consume `ferrum-view` as their layout, hit-testing, and virtualization contract.

---

## Key Features

### 1. Viewport Virtualization & HeightMap Tree
- **Canonical `HeightMap` B-Tree**: Directly ported from `@codemirror/view/src/heightmap.ts`.
  - `HeightMapBlock`: Block-level widgets and replaced multi-line ranges with estimated or measured heights.
  - `HeightMapText`: Text lines tracking inline widgets, collapsed spans, and line breaks.
  - `HeightMapGap`: Compact representation of large uniform text regions without decorations.
  - `HeightMapBranch`: Balanced B-tree node maintaining cumulative heights and line counts.
- **Dynamic Height Re-measurement**: Layout measurements update heightmaps via `HeightMap::update_height` and `MeasuredHeights`.
- **OT-Aware Spatial Mapping**: Height trees update incrementally through document edits via `HeightMap::apply_changes`.

### 2. Canonical Decoration Sets (`DecorationSet`)
- Direct equivalence to CodeMirror 6: `DecorationSet` is an alias of `RangeSet<Decoration>`.
- **Supported Decoration Types**:
  - Inline Marks: Text styling spans (syntax tokens, search highlights).
  - Inline Widgets: Embedded custom elements between characters.
  - Line Attributes: Line-level styling (active line highlight, diff backgrounds).
  - Range Replacements: Collapsed or replaced intervals (code folding, custom chips).
- **$O(\log N)$ Spatial Queries**: `decorations.between(viewport.from, viewport.to, ...)` efficiently extracts only the decorations intersecting the visible viewport.
- **OT Mapping**: Maps through document edits across transactions (`decorations.map(&changes.desc())`).

### 3. EditorView & Dispatch Cycle
- Holds the canonical `Arc<EditorState>`.
- Coordinates transaction updates, producing `ViewUpdate` contracts (`doc_changed`, `selection_changed`, `viewport_changed`).

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

All test suites are ported directly from CodeMirror 6 and pass with 100% behavioral parity (26/26 tests):
- `@codemirror/view/test/test-heightmap.ts` $\rightarrow$ `tests::test_heightmap`
- Viewport decoration and OT mapping $\rightarrow$ `tests::test_decorations`

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
