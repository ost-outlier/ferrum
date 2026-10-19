# ferrum-view / src — API Reference

This crate exports the abstract viewport virtualization, interval-tree decoration sets, and visual update contracts for Ferrum, adapted from [`@codemirror/view`](https://code.haverbeke.berlin/codemirror/view).

---

### Viewport Virtualization

- `Viewport`: Represents the currently visible range of the document.
  - `from: usize`: Start character offset of visible window.
  - `to: usize`: End character offset of visible window.
  - `viewport.contains(pos: usize) -> bool`: Checks if position is within viewport bounds.
  - `viewport.overlaps(from: usize, to: usize) -> bool`: Checks if range intersects viewport.

---

### Decorations & Interval Sets

- `Decoration`: A visual styling, replacement, or widget associated with an interval.
  - `Decoration::new(from: usize, to: usize, kind: DecorationKind) -> Range<Decoration>`: Constructs a decorated range.
- `DecorationKind`: The visual form of the decoration:
  - `Mark { class: &'static str }`: Inline text styling (spans, syntax tokens, search matches).
  - `Widget { id: &'static str }`: Embedded custom UI element between characters.
  - `Line { class: &'static str }`: Line-level attribute (active line indicator, diff highlight).
  - `Replace { id: &'static str }`: Replaces document interval (code folding).
- `DecorationSet`: Alias for `RangeSet<Decoration>`.
  - `DecorationSet::empty() -> Self`: Creates empty decoration set.
  - `DecorationSet::of_sorted(ranges: Vec<Range<Decoration>>) -> Self`: Builds decoration set from sorted ranges.
  - `decorations.between(from, to, callback)`: $O(\log N)$ query extracting visible decorations.
  - `decorations.map(&change_desc) -> Self`: Maps decorations through document edits.

---

### EditorView Lifecycle

- `EditorView`: Pure Rust, UI-agnostic view controller coordinating document updates and visual projections.
  - `EditorView::new(state: Arc<EditorState>) -> Self`: Creates a new view.
  - `view.state() -> &Arc<EditorState>`: Accesses current editor state.
  - `view.dispatch(spec: TransactionSpec) -> Option<ViewUpdate>`: Executes an update transaction.
  - `view.set_decorations(decorations: DecorationSet)`: Updates active decoration set.
  - `view.visible_decorations(viewport: &Viewport) -> Vec<DecoratedRange>`: Queries decorations intersecting viewport.
- `ViewUpdate`: Describes changes resulting from a transaction (`doc_changed`, `selection_changed`, `viewport_changed`).
