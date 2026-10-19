# ferrum-state

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum-state/badge.svg)](https://docs.rs/ferrum-state)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

> **Core functional data structures and transaction engine for Ferrum.**  
> Direct 1:1 mathematical parity with [`@codemirror/state`](https://code.haverbeke.berlin/codemirror/state).

---

## Overview

`ferrum-state` implements the core data structures that govern state management, transactional document updates, operational transformation (OT), multi-cursor selections, and the facet configuration system. It has **zero UI dependencies** and compiles to native targets as well as WebAssembly.

---

## Key Concepts & Architecture

### 1. Documents & Slicing (`Text`)
- Backed by balanced B-tree ropes ([`ropey`](https://crates.io/crates/ropey)).
- $O(\log N)$ inserts, deletions, slicing, and index translation (byte $\leftrightarrow$ UTF-16 $\leftrightarrow$ Unicode char).
- Immutable, cheap snapshots using copy-on-write (`Arc`).

### 2. Operational Transformation (`ChangeSet` & `ChangeDesc`)
- Section-based delta representation (`SectionIter`).
- **Composition**: $A \circ B$ combines two consecutive changes into a single atomic changeset.
- **Mapping (Rebasing)**: $A \circ B' = B \circ A'$ ensures confluence under concurrent transactions.
- **Inversion**: `ChangeSet::invert(&doc)` produces the exact inverse changeset needed for undo/redo stacks.

### 3. Multi-Cursor Selections (`EditorSelection` & `SelectionRange`)
- Strict canonical normalization invariant mirroring CodeMirror 6:
  - Adjacent empty cursors at the same position merge.
  - Touching non-empty ranges (e.g. `6..7` and `7..8`) **coexist** without unintended merging.
  - Overlapping ranges merge, inheriting the direction (`inverted`) of the latest overlapping range.
- First-class multi-selection support for arbitrary cursor counts.

### 4. Interval Trees & Chunk Partitioning (`RangeSet` & `RangeSetBuilder`)
- Ported directly from `@codemirror/state/src/rangeset.ts`.
- B-tree chunk partitioning (chunk size 250) for $O(\log N)$ range lookups and updates.
- Layer spilling preserves identical interval spans across overlapping attributes.
- Full `spans` iterator and `compare` algorithms with point/mark resolution.
- Maps cleanly through `ChangeDesc` deltas via `RangeSet::map`.

### 5. Facets, Compartments & State Fields (`Facet`, `Compartment`, `StateField`)
- **Facet System**: Strongly-typed dependency-injection mechanism. Extensions provide inputs to facets without mutating editor internals.
- **Compartments**: Dynamic re-configuration containers allowing parts of the configuration to be swapped at runtime via `compartment.reconfigure(...)`.
- **State Fields**: Persistent, transaction-aware data fields attached to editor states that update deterministically with every transaction.

### 6. Unicode Grapheme Awareness (`char`)
- Cursor stepping, word boundaries, subword boundaries (`camelCase` / `snake_case`), and character categorization strictly respect Unicode Extended Grapheme Clusters (`unicode-segmentation`).

---

## Quickstart

```rust
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::selection::EditorSelection;
use ferrum_state::transaction::TransactionSpec;
use ferrum_state::change::ChangeSpec;

// Initialize state
let state = EditorState::create(EditorStateConfig {
    doc: Some("Hello, world!".into()),
    selection: Some(EditorSelection::cursor(5)),
    extensions: None,
});

// Create a transactional update
let spec = TransactionSpec::new()
    .with_change(ChangeSpec::replace(5, 12, " Ferrum"))
    .with_selection(EditorSelection::cursor(14));

let tr = state.update(spec).expect("valid transaction");
let new_state = state.apply(tr);

assert_eq!(new_state.doc().to_string(), "Hello Ferrum!");
assert_eq!(new_state.selection().main().head, 14);
```

---

## Testing & Parity

All test suites are ported directly from CodeMirror 6 and pass with 100% behavioral parity (177/177 tests):
- `@codemirror/state/test/test-change.ts` $\rightarrow$ `tests::test_change`
- `@codemirror/state/test/test-selection.ts` $\rightarrow$ `tests::test_selection`
- `@codemirror/state/test/test-rangeset.ts` $\rightarrow$ `tests::test_rangeset`
- `@codemirror/state/test/test-facet.ts` $\rightarrow$ `tests::test_facet`
- `@codemirror/state/test/test-state.ts` $\rightarrow$ `tests::test_state`
- `@codemirror/state/test/test-text.ts` $\rightarrow$ `tests::test_text`

Run test suite:
```bash
cargo test -p ferrum-state --lib
```

---

## License

Dual-licensed under either of:
- MIT License ([LICENSE-MIT](https://github.com/your-org/ferrum/blob/main/LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/your-org/ferrum/blob/main/LICENSE-APACHE))

Strict Clean-Room implementation: contains zero copyleft or GPL code.
