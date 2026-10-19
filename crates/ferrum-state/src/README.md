# ferrum-state / src — API Reference

This crate exports the core data structures that govern state management, transactional document updates, operational transformation, multi-cursor selections, and the facet configuration system, in 1:1 mathematical parity with [`@codemirror/state`](https://code.haverbeke.berlin/codemirror/state).

---

### Editor State & Configuration

- `EditorState`: Immutable, functional representation of the entire editor state. Reusable across thread boundaries (`Arc`).
  - `state.doc() -> &Text`: Accesses the underlying rope document.
  - `state.selection() -> &EditorSelection`: Accesses the normalized multi-cursor selection.
  - `state.facet(&facet) -> &Output`: Queries the combined output value of a facet.
  - `state.field(&field) -> &Value`: Queries the value of an attached `StateField`.
  - `state.update(spec: TransactionSpec) -> Option<Transaction>`: Prepares a transactional update.
  - `state.apply(tr: Transaction) -> Arc<EditorState>`: Computes the new state resulting from a transaction.
- `EditorStateConfig`: Configuration struct used when creating states via `EditorState::create`.

---

### Selections & Multi-Cursor

- `SelectionRange`: A single selection range from `anchor` to `head`.
  - `SelectionRange::cursor(pos: usize)`: Creates a point selection (caret).
  - `SelectionRange::new(anchor: usize, head: usize)`: Creates a selection range.
  - `range.from() -> usize`: Minimum of `anchor` and `head`.
  - `range.to() -> usize`: Maximum of `anchor` and `head`.
  - `range.is_empty() -> bool`: True if `anchor == head`.
  - `range.is_inverted() -> bool`: True if `head < anchor`.
  - `range.goal_column: Option<usize>`: Retained column for vertical navigation across lines.
- `EditorSelection`: A normalized collection of one or more `SelectionRange`s with a designated `main_index`.
  - Normalization invariants match CodeMirror 6:
    - Merges adjacent empty cursors at the same position.
    - Preserves touching non-empty ranges (e.g. `6..7` and `7..8`).
    - Merges overlapping ranges, inheriting the direction (`inverted`) of the latest range.

---

### Document & Text Rope

- `Text`: Immutable B-tree rope document backed by [`ropey`](https://crates.io/crates/ropey).
  - `Text::from_str(s: &str) -> Self`: Constructs text from a string slice.
  - `text.len_chars() -> usize`: Length of document in Unicode characters (codepoints).
  - `text.len_lines() -> usize`: Total number of lines in document.
  - `text.line(idx: usize) -> Line`: Queries line by 0-based index.
  - `text.line_at(char_pos: usize) -> Line`: Queries line containing `char_pos`.
  - `text.char_to_line(char_pos: usize) -> usize`: 0-based line index containing `char_pos`.
  - `text.slice_to_string(range) -> String`: Extracts slice as a standard string.
- `Line`: Metadata for a single line (`from`, `to`, `length`, `number`).

---

### Changes and Transactions

- `ChangeSpec`: High-level specification of a text edit (`insert`, `delete`, `replace`).
- `ChangeDesc`: Low-level delta representation recording section lengths (`SectionIter`).
  - `desc.map_pos(pos: usize, assoc: i8) -> usize`: Projects a position through edits.
  - `desc.compose(&other) -> ChangeDesc`: Combines two consecutive deltas.
  - `desc.map(&other, before: bool) -> ChangeDesc`: Maps across concurrent changes (OT).
- `ChangeSet`: Associates a `ChangeDesc` with inserted replacement text.
  - `changeset.invert(&doc) -> ChangeSet`: Computes exact mathematical inverse changeset.
  - `changeset.apply(&doc) -> Text`: Applies changes to the document rope in a single linear pass.
- `TransactionSpec`: Builder describing changes, selections, effects, and annotations for a transaction.
- `Transaction`: Validated, immutable transaction containing state, changes, selection, and doc.

---

### Extending Editor State (Facets & Fields)

- `Facet<Input, Output>`: Strongly-typed extension point with deterministic combination function.
  - `facet.of(value: Input) -> Extension`: Supplies an input value to a facet.
- `Compartment`: Dynamic re-configuration container allowing parts of the extension tree to be replaced at runtime via `compartment.reconfigure(new_extension)`.
- `StateField<Value>`: Transaction-aware persistent state field that evolves deterministically with state updates.
- `Prec`: Precedence modifier for ordering extensions (`Highest`, `High`, `Default`, `Low`, `Lowest`).

---

### Range Sets (Interval Trees)

- `RangeSet<T: RangeValue>`: Balanced B-tree chunk interval tree partitioned into chunks (size 250) with layer spilling.
  - `rangeset.between(from: usize, to: usize, callback: impl FnMut(usize, usize, &T))`: $O(\log N)$ spatial query for intersecting intervals.
  - `rangeset.map(&change_desc) -> RangeSet<T>`: Projects intervals through document edits.
- `RangeSetBuilder<T: RangeValue>`: Efficient monotonic builder for assembling `RangeSet` instances in sorted order.
- `RangeValue`: Trait defining mapping behavior and equality for range payloads.
