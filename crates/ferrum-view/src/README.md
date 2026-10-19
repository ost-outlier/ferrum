# ferrum-view / src — API Reference

This crate exports the abstract viewport virtualization, interval-tree decoration sets, heightmap line virtualization, and visual update contracts for Ferrum, adapted from [`@codemirror/view`](https://code.haverbeke.berlin/codemirror/view).

---

## 1. Viewport Virtualization

- `Viewport`: Represents the currently visible range of the document.
  - `from: usize`: Start character offset of visible window.
  - `to: usize`: End character offset of visible window.
  - `viewport.contains(pos: usize) -> bool`: Checks if position is within viewport bounds.
  - `viewport.overlaps(from: usize, to: usize) -> bool`: Checks if range intersects viewport.

---

## 2. HeightMap Line Virtualization Tree

Ported directly from `@codemirror/view/src/heightmap.ts`:

- `HeightMap`: Balanced B-tree node representing layout heights across the document.
  - `HeightMap::empty() -> HeightMap`: Initializes an empty heightmap.
  - `HeightMap::of(decorations, oracle, from, to) -> HeightMap`: Builds height tree from decoration sets and oracle metrics.
  - `map.height() -> f64`: Total pixel height of the document.
  - `map.length() -> usize`: Total character length covered by this node.
  - `map.lines() -> usize`: Total line count covered by this node.
  - `map.line_at(pos, query_type, oracle, top, offset) -> BlockInfo`: Computes geometric bounds of the line or block at a given position or vertical coordinate.
  - `map.block_at(pos, query_type, oracle, top, offset) -> BlockInfo`: Computes geometric bounds of the specific sub-block at a given position.
  - `map.update_height(oracle, offset, force, measured) -> HeightMap`: Refreshes heights with measured layout values.
  - `map.apply_changes(decorations, doc, oracle, changes) -> HeightMap`: Updates tree across transactional OT edits.
- `HeightMapBlock`: Block widget or replaced multi-line region.
- `HeightMapText`: Text line node tracking collapsed ranges and widget heights.
- `HeightMapGap`: Compact representation of uniform text lines without decorations.
- `HeightMapBranch`: B-tree branch node maintaining cumulative heights and sub-nodes.
- `HeightOracle`: Stores document reference and metrics (`line_height`, `char_width`).
- `BlockInfo`: Calculated geometry for a block or line (`from`, `to`, `top`, `bottom`, `height`, `block_type`).
- `BlockType`: Enum categorization (`Text`, `WidgetBefore`, `WidgetAfter`, `WidgetRange`).
- `MeasuredHeights`: Stream of layout measurements passed from the native text shaper.
- `NodeBuilder`: Adapter implementing `SpanIterator<Decoration>` to assemble heightmap nodes from `RangeSet::spans`.

---

## 3. Decorations & Interval Sets

- `Decoration`: A visual styling, replacement, or widget associated with an interval.
  - `Decoration::mark(range, spec)`: Inline text styling (spans, syntax tokens, search matches).
  - `Decoration::widget(pos, spec)`: Embedded custom UI element between characters.
  - `Decoration::line(pos, spec)`: Line-level attribute (active line indicator, diff highlight).
  - `Decoration::replace(range, spec)`: Replaces document interval (code folding).
- `DecorationSet`: Alias for `RangeSet<Decoration>`.
  - `DecorationSet::empty() -> Self`: Creates empty decoration set.
  - `DecorationSet::of_sorted(ranges: Vec<Range<Decoration>>, is_points: bool) -> Result<Self, ()>`: Builds decoration set from sorted ranges.
  - `decorations.between(from, to, callback)`: $O(\log N)$ query extracting visible decorations.
  - `decorations.map(&change_desc) -> Self`: Maps decorations through document edits.

---

## 4. EditorView Lifecycle

- `EditorView`: Pure Rust, UI-agnostic view controller coordinating document updates and visual projections.
  - `EditorView::new(state: Arc<EditorState>) -> Self`: Creates a new view.
  - `view.state() -> &Arc<EditorState>`: Accesses current editor state.
  - `view.dispatch(spec: TransactionSpec) -> Option<ViewUpdate>`: Executes an update transaction.
  - `view.set_decorations(decorations: DecorationSet)`: Updates active decoration set.
  - `view.visible_decorations(viewport: &Viewport) -> Vec<DecoratedRange>`: Queries decorations intersecting viewport.
- `ViewUpdate`: Describes changes resulting from a transaction (`doc_changed`, `selection_changed`, `viewport_changed`).
