# C6 behaviors (rangeset)

Owned cm6 items: 47 (none DROP); one section each, in A1 id order. cm6 paths are relative to `/workspace/CodeMirror6/`, Ferrum paths to `/workspace/ferrum/`. Test ids: cm6 `test-rangeset#NN` (T1), Ferrum `test_rangeset#NNN` (T2). "R<n>" = reproduction in `C6-api.md` `## Reproductions`. "Replay" = R33 (cm6 cases re-run against Ferrum with cm6 inputs). That is evidence collected during this audit, not a Ferrum test. Where the Ferrum test column says "none", no test in `crates/ferrum-state/src/tests/test_rangeset.rs` asserts the behavior.

Shared cm6 helpers, read for all sections below: `cmpVal` `state/src/rangeset.ts:38`, `cmpRange` `:58`, `C` `:95`, `Chunk` `:102-179` (`findIndex` `:116`, `between` `:128`, `map` `:133`), `last` `:470`, `lazySort` `:472`, `RangeSetBuilder` `:487-568`, `findSharedChunks` `:570`, `LayerCursor` `:584-668`, `HeapCursor` `:670-723`, `heapBubble` `:725`, `SpanCursor` `:741-866`, `compare` `:868`, `sameValues` `:898`, `remove` `:904`, `insert` `:909`, `findMinIndex` `:914`, and from `state/src/change.ts`: `ChangeDesc.mapPos` `:122`, `touchesRange` `:147`, `iterGaps` `:56`, `empty`/`length`.
Ferrum counterparts read: `crates/ferrum-state/src/rangeset.rs:1-962` in full, `crates/ferrum-state/src/change.rs:26-195` (`ChangeDesc::create`, `empty`, `length`, `new_length`, `is_empty`, `iter_gaps`, `map_pos`), `:318-376` (`ChangeSet::create_set`, `desc`).

### state.cm6.170 `RangeValue` — cm6 `state/src/rangeset.ts:5`
Helpers read: prototype defaults `state/src/rangeset.ts:34-36`, `cmpVal` `:38`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.B1 | Abstract base class; values subclass it and may override `eq`, `startSide`, `endSide`, `mapMode`, `point`, `range` | `state/src/rangeset.ts:5-32` | trait with provided methods `crates/ferrum-state/src/rangeset.rs:14-25` (no `range`) | test-rangeset#01–#47 (`Value`) | test_rangeset#001–#047 (`TestVal`) | differs: there is no `range` hook (see state.RangeValue.range) |
| state.RangeValue.B2 | Defaults on the prototype: `startSide = endSide = 0`, `point = false`, `mapMode = TrackDel` | `state/src/rangeset.ts:34-36` | `crates/ferrum-state/src/rangeset.rs:16-22` | none (the test `Value` overrides every default) | none (`TestVal` overrides sides and point) | implemented, untested |
| state.RangeValue.B3 | Values of different classes can live in one set (`RangeSet<RangeValue>`); `cmpVal` treats different constructors as unequal before calling `eq` | `state/src/rangeset.ts:39` | a set is generic over one `T`; there is no constructor check (`rangeset.rs:840`, `:870`) | none | none | differs: the generic `T` replaces the constructor check (an enum `T` must handle it inside `eq_val`) |
| state.RangeValue.B4 | Any `number` is allowed for sides (the view uses values up to ±6e8) | `state/src/rangeset.ts:15-17`; `view/src/decoration.ts:171-185` | `i32` sides; truncated to `i8` in `map` (`rangeset.rs:247`, `:250`, `:259`, `:260`) | none | none | differs: R3, R31 |
Proposed status: PORTED, DEVIATION because the default `eq` differs (B3/eq row), sides are truncated during mapping (B4), and there is no `range` hook.

### state.cm6.171 `RangeValue.eq` — cm6 `state/src/rangeset.ts:11`
Helpers read: `cmpVal` `state/src/rangeset.ts:38`, `sameValues` `:898`, `compare` `:868`, `RangeSet.eq` `:389`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.eq.B1 | Default implementation compares by identity (`this == other`) | `state/src/rangeset.ts:11` | default `self == other` (`PartialEq`) `crates/ferrum-state/src/rangeset.rs:24` | none | none | differs: structural, not identity (R20) |
| state.RangeValue.eq.B2 | `cmpVal`: identical objects are equal without calling `eq`; otherwise the constructors must match, then `eq` decides | `state/src/rangeset.ts:39` | `eq_val` called directly `crates/ferrum-state/src/rangeset.rs:840`, `:870` | test-rangeset#27 (identical names), #30 | none (test_rangeset#027, #030 assert nothing) | differs: no identity shortcut and no constructor check; untested |
| state.RangeValue.eq.B3 | `sameValues` compares element-wise in order (`a[i] != b[i] && !cmpVal`) | `state/src/rangeset.ts:898-902` | `same_values` `crates/ferrum-state/src/rangeset.rs:835-845` | test-rangeset#25–#34 | none | implemented, untested |
| state.RangeValue.eq.B4 | Used by `compare` (ranges and points) and `RangeSet.eq` | `state/src/rangeset.ts:882-888`, `:401-403` | `compare_cursors` `rangeset.rs:868-876`; no `RangeSet::eq` | test-rangeset#25–#34 | none | implemented for compare, untested; missing for `eq` |
Proposed status: PORTED, DEVIATION because the default semantics differ (B1, R20) and nothing is tested.

### state.cm6.172 `RangeValue.startSide` — cm6 `state/src/rangeset.ts:15`
Helpers read: `cmpRange` `:58`, `RangeSetBuilder.addInner` `:524`, `Chunk.findIndex` `:116`, `Chunk.map` `:133`, `LayerCursor.compare` `:664`, `RangeSet.spans` `:428-430`, `ChangeDesc.mapPos` `change.ts:122`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.startSide.B1 | Default 0 | `state/src/rangeset.ts:34` | `rangeset.rs:16` | none | none | implemented, untested |
| state.RangeValue.startSide.B2 | Orders ranges starting at the same position (`cmpRange`, builder strict check, `of` sort) | `state/src/rangeset.ts:58-60`, `:526` | `rangeset.rs:155-158`, `:372-375` | test-rangeset#02 (second assertion) | none (test_rangeset#002 checks only the from-order case) | implemented, untested in Ferrum (Replay #02 passes) |
| state.RangeValue.startSide.B3 | Used as `assoc` when mapping the start (empty ranges: with `mapMode`; non-empty: `Simple`); full `number` precision | `state/src/rangeset.ts:139`, `:147` | `rangeset.rs:247`, `:259` (`as i8`) | test-rangeset#11, #12, #19, #22–#24 | test_rangeset#011, #012, #019 | differs: truncated to `i8` (R3, R31); small sides tested |
| state.RangeValue.startSide.B4 | Heap/iteration order: `from`, then `startSide` | `state/src/rangeset.ts:665` | `rangeset.rs:533-538` (cursor), `:194-197` (`iter`) | test-rangeset#19, #22 | test_rangeset#019 | implemented+tested |
| state.RangeValue.startSide.B5 | In spans, a point with `startSide < 0` reports `openCount = active.length` | `state/src/rangeset.ts:429` | `rangeset.rs:943-944` | none | none | implemented, untested |
Proposed status: PORTED, DEVIATION because of B3 (i8 truncation).

### state.cm6.173 `RangeValue.endSide` — cm6 `state/src/rangeset.ts:17`
Helpers read: as state.cm6.172 plus `SpanCursor.next` `:803`, `findMinIndex` `:914`, `activeForPoint` `:850`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.endSide.B1 | Default 0 | `state/src/rangeset.ts:34` | `rangeset.rs:18` | none | none | implemented, untested |
| state.RangeValue.endSide.B2 | Builder: a range overlaps the previous one in its layer when `from < lastTo` or `from == lastTo && startSide < last.endSide`; it then goes to the next layer | `state/src/rangeset.ts:525`, `:528` | `rangeset.rs:371`, `:378-380` | test-rangeset#01, #20 | test_rangeset#001 | implemented+tested |
| state.RangeValue.endSide.B3 | Used as `assoc` when mapping the end (always `Simple` mode) | `state/src/rangeset.ts:143`, `:148` | `rangeset.rs:250` (uses `map_mode`), `:260` (`as i8`) | test-rangeset#11, #13, #16 | test_rangeset#011, #013, #016 | differs: R3 (truncation), R32 (mode) |
| state.RangeValue.endSide.B4 | A non-empty range that collapses with `startSide > 0 && endSide <= 0` is dropped | `state/src/rangeset.ts:149` | `rangeset.rs:262` | test-rangeset#14 | test_rangeset#014 | implemented+tested |
| state.RangeValue.endSide.B5 | A mapped range is dropped when `(newTo - newFrom \|\| endSide - startSide) < 0` (an empty result with `endSide < startSide`) | `state/src/rangeset.ts:151` | none (`rangeset.rs:254` keeps `new_to >= new_pos`) | none | none | missing: R5 |
| state.RangeValue.endSide.B6 | Cursor/active end ordering: `findIndex(end)`, `forward`, `findMinIndex`, and activeForPoint's `endSide >= point.endSide` | `state/src/rangeset.ts:121`, `:623`, `:917`, `:855` | `rangeset.rs:67-68`, `:494`, `:709`, `:816` | test-rangeset#40 | test_rangeset#040 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B3 and B5.

### state.cm6.174 `RangeValue.mapMode` — cm6 `state/src/rangeset.ts:22`
Helpers read: `Chunk.map` `:133-151`, `ChangeDesc.mapPos` `change.ts:122-142`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.mapMode.B1 | Default `MapMode.TrackDel` | `state/src/rangeset.ts:36` | `rangeset.rs:22` | test-rangeset#15 (point at 3 inside deletion 0..4 dropped) | test_rangeset#015 | implemented+tested |
| state.RangeValue.mapMode.B2 | Applied only to the start mapping of empty ranges (`curFrom == curTo`); `null` drops the range | `state/src/rangeset.ts:139-140` | `rangeset.rs:247` | test-rangeset#15 | test_rangeset#015 | implemented+tested |
| state.RangeValue.mapMode.B3 | NOT applied to the end-side mapping of an empty range with `startSide != endSide` (`Simple`) | `state/src/rangeset.ts:143` | differs `rangeset.rs:250` (passes `map_mode()`; `None` → keeps `new_to = new_pos`) | none | none | differs: R32 |
| state.RangeValue.mapMode.B4 | NOT applied to non-empty ranges (both ends `Simple`) | `state/src/rangeset.ts:147-148` | `rangeset.rs:259-260` (`MapMode::Simple`) | test-rangeset#16 | test_rangeset#016 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B3.

### state.cm6.175 `RangeValue.point` — cm6 `state/src/rangeset.ts:28`
Helpers read: `addInner` `:537`, `Chunk.map` `:153`, `LayerCursor.next` `:640`, `HeapCursor.from` `:686`, `SpanCursor.next` `:826-841`, `RangeSet.compare` `:376-377`, `findSharedChunks` `:573`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.point.B1 | Default false | `state/src/rangeset.ts:35` | `rangeset.rs:20` | none | none | implemented, untested |
| state.RangeValue.point.B2 | Points contribute `to - from` to the chunk/set `maxPoint` (−1 = no points) | `state/src/rangeset.ts:537`, `:153`, `:505` | `rangeset.rs:397-399`, `:348`; map recomputes through the builder | none directly | none | implemented, untested |
| state.RangeValue.point.B3 | In spans/compare, a point shadows the ranges and points it covers; a non-empty point ending exactly at the previous point's end is skipped | `state/src/rangeset.ts:826-841` | `rangeset.rs:778-792` | test-rangeset#39, #40, #42 | test_rangeset#039, #040, #042 | implemented+tested |
| state.RangeValue.point.B4 | `minPointSize`: layers/chunks whose `maxPoint` is below it are skipped, and only points of at least that size are yielded | `state/src/rangeset.ts:611`, `:640`, `:686` | `rangeset.rs:475`, `:526`, `:556` | test-rangeset#41 | test_rangeset#041 | implemented+tested (spans only; compare has no minPointSize, see state.cm6.188) |
Proposed status: PORTED, DEVIATION because B1/B2 are untested and point filtering is missing from `compare`.

### state.cm6.176 `RangeValue.range` — cm6 `state/src/rangeset.ts:31`
Helpers read: `Range.create` `state/src/rangeset.ts:53`; overrides `view/src/decoration.ts:297-300`, `:314-317`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.range.B1 | `value.range(from, to)` creates `Range(from, to, this)` | `state/src/rangeset.ts:31` | `Range::new(from, to, value)` `rangeset.rs:37` | all (`mk`) | all (`mk`) | implemented+tested (different API shape) |
| state.RangeValue.range.B2 | `to` defaults to `from` | `state/src/rangeset.ts:31` | none (`to` is required) | test-rangeset#10, #12, #17 (`mk(1)`) | none | missing (R22g) |
| state.RangeValue.range.B3 | Overridable per value type, e.g. to validate (the view's mark/line decorations throw) | `view/src/decoration.ts:297-300` | none (no method on the trait) | none | none | missing |
Proposed status: PORTED, DEVIATION because B2 and B3 are missing (`Range::new` is the only constructor).

### state.cm6.177 `Range` — cm6 `state/src/rangeset.ts:43`
Helpers read: `Range.create` `:53`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.B1 | Private constructor: ranges are obtained only via `RangeValue.range` (or @internal `Range.create`) | `state/src/rangeset.ts:44`, `:53` | `pub fn new` `rangeset.rs:37` and `pub` fields | all | all | differs: freely constructible |
| state.Range.B2 | Immutable (`readonly` fields) | `state/src/rangeset.ts:46-50` | `pub` mutable fields `rangeset.rs:30-32` | none | none | differs (Rust ownership: only the owner's copy changes) |
Proposed status: PORTED, DEVIATION because B1 and B2 differ (encapsulation).

### state.cm6.178 `Range.from` — cm6 `state/src/rangeset.ts:46`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.from.B1 | Start position, read-only, in the caller's document unit | `state/src/rangeset.ts:46` | `rangeset.rs:30` (`pub usize`) | test-rangeset#19, #22, #44 | test_rangeset#010, #012, #017, #019 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (public field on an owned value; observable behavior is the same).

### state.cm6.179 `Range.to` — cm6 `state/src/rangeset.ts:48`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.to.B1 | End position, read-only | `state/src/rangeset.ts:48` | `rangeset.rs:31` | test-rangeset#10–#24 | test_rangeset#011, #013, #016 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.180 `Range.value` — cm6 `state/src/rangeset.ts:50`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.value.B1 | The associated value (shared object reference) | `state/src/rangeset.ts:50` | `rangeset.rs:32` (owned clone; `iter` clones, `:208`) | test-rangeset#19, #22 | test_rangeset#019 | implemented+tested (clone instead of reference; identity is not observable in Ferrum) |
Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.181 `RangeSet` — cm6 `state/src/rangeset.ts:226`
Helpers read: constructor/`create` `:227-243`, `chunkEnd` `:260`, `isEmpty` `:356`, `empty` `:467`, `:482`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.B1 | Ranges are stored in chunks of at most 250 (`C.ChunkSize`) with chunk-relative offsets | `state/src/rangeset.ts:97`, `:529-531` | `rangeset.rs:10`, `:382-392` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSet.B2 | Overlapping ranges spill into `nextLayer` sets | `state/src/rangeset.ts:519-520` | `rangeset.rs:363-366` | test-rangeset#01, #20 | test_rangeset#001 (`next_layer.is_some()`) | implemented+tested |
| state.RangeSet.B3 | Immutable; operations return new sets and share untouched chunks (by identity) | `state/src/rangeset.ts:274`, `:287`, `:315-318` | `update`/`map` rebuild every chunk (`rangeset.rs:218-270`) | test-rangeset#03 | none | differs: R6 |
| state.RangeSet.B4 | Internals are `@internal`; construction only through `of`/builder/`empty` | `state/src/rangeset.ts:227-243` | `pub` fields `rangeset.rs:93-96`, struct literal possible | test-rangeset#01, #03, #05, #07, #18 read `chunk`/`nextLayer` | test_rangeset#001, #007, #018 read `chunk`/`next_layer` | differs (encapsulation) |
| state.RangeSet.B5 | The empty set is a unique object whose `nextLayer` is itself | `state/src/rangeset.ts:467`, `:482` | `next_layer: None` `rangeset.rs:111` | test-rangeset#09, #18 | test_rangeset#009, #018 (`is_empty`) | differs: there is no singleton |
Proposed status: PORTED, DEVIATION because of B3–B5.

### state.cm6.182 `RangeSet.size` — cm6 `state/src/rangeset.ts:252`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.size.B1 | 0 for the empty set | `state/src/rangeset.ts:253` | `rangeset.rs:122-131` (no chunks, no layer) | test-rangeset#09 (indirect) | test_rangeset#009, #014, #015 | implemented+tested |
| state.RangeSet.size.B2 | The sum of chunk value counts plus `nextLayer.size` | `state/src/rangeset.ts:254-256` | `rangeset.rs:124-129` | test-rangeset#01, #03–#07 | test_rangeset#001, #003–#007 | implemented+tested |
Proposed status: PORTED (every behavior is implemented and tested; the oracle shows 0/9 003 mismatches).

### state.cm6.183 `RangeSet.update` — cm6 `state/src/rangeset.ts:271`
Helpers read: `LayerCursor` `:584-668` (incl. `goto`, `next`, `nextChunk`), `RangeSetBuilder.addInner` `:524`, `addChunk` `:542`, `finishInner` `:560`, `RangeSet.of` `:448`, `cmpRange` `:58`, `chunkEnd` `:260`, `length` `:246`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.update.B1 | With no `add` and no `filter`, returns `this` unchanged (identity) | `state/src/rangeset.ts:274` | none (always rebuilds, `rangeset.rs:219-235`) | none | none | differs: R7a |
| state.RangeSet.update.B2 | `sort: true` sorts a copy of `add` by `cmpRange`; otherwise `add` must already be sorted | `state/src/rangeset.ts:275` | `sort` ignored; always sorts everything `rangeset.rs:235` | none | none | differs: R8 |
| state.RangeSet.update.B3 | On an empty set: `RangeSet.of(add)` (strict, throws on unsorted `add`) or `this` | `state/src/rangeset.ts:276` | no special case | none | none | differs: R8d |
| state.RangeSet.update.B4 | Merges existing ranges and `add` in `(from, startSide)` order; on ties the added range goes first | `state/src/rangeset.ts:280-283` | full re-sort, stable: existing ranges first on ties (`rangeset.rs:219`, `:232`, `:235`) | test-rangeset#03, #04 | test_rangeset#003, #004 | differs in layer placement and order on ties (R1 class; the oracle shows 1 236 order-only differences, 0 content) |
| state.RangeSet.update.B5 | An added range that does not fit in the current layer spills into `nextLayer.update({add: spill, …})` | `state/src/rangeset.ts:283`, `:298-299` | `RangeSetBuilder::add_range` spill `rangeset.rs:363-366` | test-rangeset#04 | test_rangeset#004 | implemented+tested (different structure, same content) |
| state.RangeSet.update.B6 | Whole chunks before the next added range and outside the filter range are reused (`addChunk`, structural sharing) | `state/src/rangeset.ts:284-288`, `:542-553` | none | test-rangeset#03 | none | missing: R6b |
| state.RangeSet.update.B7 | `filter` is called only for ranges touching `[filterFrom, filterTo]` (`filterFrom > to \|\| filterTo < from` skips); a range is kept when it returns true | `state/src/rangeset.ts:290` | `rangeset.rs:221-229` | test-rangeset#05–#09 | test_rangeset#005–#009 (#008 weak) | implemented+tested (exact call list only in Replay #08 and R29) |
| state.RangeSet.update.B8 | Defaults: `filterFrom = 0`, `filterTo = this.length` | `state/src/rangeset.ts:272` | `rangeset.rs:288-289` (`usize::MAX`) | test-rangeset#05 | test_rangeset#005 | implemented+tested (equivalent) |
| state.RangeSet.update.B9 | The filter is applied to every layer with the same `filterFrom`/`filterTo` | `state/src/rangeset.ts:299` | `iter()` collects all layers `rangeset.rs:201-215` | test-rangeset#05 (set0 is single-layer) | none (multi-layer) | implemented, untested on multiple layers (oracle: 0 content mismatches) |
| state.RangeSet.update.B10 | Returns `RangeSet.empty` when everything is filtered out | `state/src/rangeset.ts:298`, `:562` | returns an empty set (not a singleton) | test-rangeset#09 | test_rangeset#009 (size 0) | differs: no identity (R7b) |
| state.RangeSet.update.B11 | On a non-empty set, added ranges go through non-strict `addInner`: an unsorted `add` does not throw | `state/src/rangeset.ts:283` | `rangeset.rs:235` (sorted anyway) | none | none | implemented (R8a, R8b match) |
| state.RangeSet.update.B12 | Cost: proportional to the touched chunks plus `add` (chunks reused) | `state/src/rangeset.ts:284-288` | O(n log n) and n value clones on every call (`iter` + sort + rebuild) | none | none | differs (complexity) |
Proposed status: PORTED, DEVIATION because B1–B4, B6, B10 and B12 differ. Content parity is confirmed by the oracle (4 500 cases).

### state.cm6.184 `RangeSet.map` — cm6 `state/src/rangeset.ts:303`
Helpers read: `Chunk.map` `:133-178`, `RangeSetBuilder.addRange` `:518`, `finishInner` `:560`, `ChangeDesc.touchesRange` `change.ts:147`, `mapPos` `change.ts:122`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.map.B1 | Empty changes or an empty set → returns `this` | `state/src/rangeset.ts:304` | `rangeset.rs:240-242` (`self.clone()`, Arc-shared chunks) | none | none | implemented (R28), untested |
| state.RangeSet.map.B2 | A chunk not touched by the changes is kept by identity at `mapPos(start)` | `state/src/rangeset.ts:315-318` | none (each range is re-mapped and rebuilt) | none | none | differs: R6a (content is the same) |
| state.RangeSet.map.B3 | A chunk entirely covered by one change (`touchesRange == "cover"`) is dropped with all its ranges | `state/src/rangeset.ts:314-328` | none | none | none | missing: R4 |
| state.RangeSet.map.B4 | Empty range: start mapped with `(startSide, mapMode)`; `null` → dropped | `state/src/rangeset.ts:138-141` | `rangeset.rs:246-247` | test-rangeset#15, #17 | test_rangeset#015, #017 | implemented+tested |
| state.RangeSet.map.B5 | Empty range with `startSide != endSide`: end mapped with `(endSide, Simple)`; dropped if `newTo < newFrom` | `state/src/rangeset.ts:142-145` | `rangeset.rs:249-256` (uses `map_mode`) | none | none | differs: R32 |
| state.RangeSet.map.B6 | Non-empty range: `from`/`to` mapped `Simple` with the sides; dropped if `newFrom > newTo`, or if it collapsed with `startSide > 0 && endSide <= 0` | `state/src/rangeset.ts:146-150` | `rangeset.rs:258-265` | test-rangeset#11, #13, #14, #16 | test_rangeset#011, #013, #014, #016 | implemented+tested |
| state.RangeSet.map.B7 | Any result with `(newTo - newFrom \|\| endSide - startSide) < 0` is dropped | `state/src/rangeset.ts:151` | none | none | none | missing: R5 |
| state.RangeSet.map.B8 | Ranges that move out of order: points are reinserted at the right place in the chunk, others spill into a new layer | `state/src/rangeset.ts:155-175` | global re-sort + builder (`rangeset.rs:269`) | test-rangeset#19–#24 | test_rangeset#019 (names), #020–#024 (size only) | implemented+tested (different structure; Replay #19–#24 pass) |
| state.RangeSet.map.B9 | Spilled ranges are built on top of the mapped `nextLayer` | `state/src/rangeset.ts:330-331` | single rebuild | test-rangeset#20 | test_rangeset#020 (weak) | implemented (different structure) |
| state.RangeSet.map.B10 | `maxPoint` recomputed from mapped points | `state/src/rangeset.ts:153`, `:316`, `:324` | through the builder `rangeset.rs:397-399` | none | none | implemented, untested |
| state.RangeSet.map.B11 | Result is `next` (possibly `RangeSet.empty`) when no chunk survives | `state/src/rangeset.ts:332` | returns an empty set | test-rangeset#18 | test_rangeset#018 (`is_empty`) | implemented+tested (no singleton identity) |
| state.RangeSet.map.B12 | Sides are used as `assoc` at full precision | `state/src/rangeset.ts:139`, `:143`, `:147-148` | `as i8` `rangeset.rs:247`, `:250`, `:259`, `:260` | none | none | differs: R3, R31 |
| state.RangeSet.map.B13 | Never fails (mapping cannot produce misordered layers) | `state/src/rangeset.ts:303-333` | `unwrap_or_else(|_| Self::empty())` `rangeset.rs:269`, unreachable after sorting | none | none | implemented (the silent fallback would hide a bug) |
| state.RangeSet.map.B14 | Cost: only touched chunks are re-mapped | `state/src/rangeset.ts:314-318` | O(n log n), clones every value | none | none | differs (complexity) |
Proposed status: PORTED, DEVIATION because B2, B3, B5, B7, B12 and B14 differ.

### state.cm6.185 `RangeSet.between` — cm6 `state/src/rangeset.ts:339`
Helpers read: `Chunk.between` `:128`, `Chunk.findIndex` `:116`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.between.B1 | Empty set → no calls | `state/src/rangeset.ts:340` | loop over zero chunks `rangeset.rs:173` | none | none | implemented, untested |
| state.RangeSet.between.B2 | Only chunks with `to >= start && from <= start + length` are visited | `state/src/rangeset.ts:343` | `rangeset.rs:176` | test-rangeset#46 | test_rangeset#046 | implemented+tested |
| state.RangeSet.between.B3 | Within a chunk, reports ranges with `to >= from` and `from <= to` (inclusive touch), using sentinel sides `±C.Far` | `state/src/rangeset.ts:129` | `rangeset.rs:79-80` with `±1_000_000` and `saturating_sub` | test-rangeset#46, #47 | test_rangeset#046 (weak), #047 | differs: R2 (sides beyond ±1e6) |
| state.RangeSet.between.B4 | A callback returning `false` stops the whole iteration, including later layers | `state/src/rangeset.ts:130`, `:344` | `rangeset.rs:82-84`, `:177-178`, `:183-184` | none | none | implemented (R27), untested |
| state.RangeSet.between.B5 | No order guarantee; actual order is layer by layer, chunk order | `state/src/rangeset.ts:335-338` | same traversal | none | none | implemented (R27b: both `A,C,B`) |
| state.RangeSet.between.B6 | Zero-length set/range at 0 is reported for `between(0,0)` | `state/src/rangeset.ts:129` | same | test-rangeset#47 | test_rangeset#047 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B3 (R2).

### state.cm6.186 `RangeSet.iter` — cm6 `state/src/rangeset.ts:351`
Helpers read: `HeapCursor.from` `:678`, `goto` `:695`, `next` `:708`, `heapBubble` `:725`, `LayerCursor.goto` `:600`, `compare` `:664`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.iter.B1 | Returns a lazy `RangeCursor` (no materialization) | `state/src/rangeset.ts:351-353` | returns a `Vec<Range<T>>` with cloned values `rangeset.rs:191-199` | test-rangeset#44 | test_rangeset#044 | differs (API shape, allocation O(n)) |
| state.RangeSet.iter.B2 | `iter(from)` starts at the first range ending at or after `from` (default 0) | `state/src/rangeset.ts:351`, `:600-602`, `:616` | none (no parameter) | test-rangeset#45 | test_rangeset#045 (emulated by filtering) | missing: R21 |
| state.RangeSet.iter.B3 | Order: `from`, `startSide`, `rank`, `to`, `endSide` | `state/src/rangeset.ts:664-667` | sort by `from`, `startSide` only (stable by layer) `rangeset.rs:194-197` | test-rangeset#19, #22, #44 | test_rangeset#019, #022 (size only), #044 (count only) | differs: R1 |
| state.RangeSet.iter.B4 | Ties with all keys equal follow the heap's swap-on-equal behavior | `state/src/rangeset.ts:730`, `:734` | n/a for `iter` (sort) | none | none | differs: R18 |
| state.RangeSet.iter.B5 | Iterates every layer | `state/src/rangeset.ts:685` | `collect_ranges` recursion `rangeset.rs:212-214` | test-rangeset#44 | test_rangeset#044 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B1–B4.

### state.cm6.187 `RangeSet^iter` — cm6 `state/src/rangeset.ts:360`
Helpers read: `HeapCursor.from` `:678-691`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^iter.B1 | Iterates several sets at once in heap order, starting at `from` | `state/src/rangeset.ts:360-362` | none (private `HeapCursor` `rangeset.rs:542`, not exposed) | none | none | missing |
| state.RangeSet^iter.B2 | `rank` = index of the set in `sets` | `state/src/rangeset.ts:687` | none (private) | none | none | missing |
| state.RangeSet^iter.B3 | All layers of one set share its rank | `state/src/rangeset.ts:685-687` | none | none | none | missing |
| state.RangeSet^iter.B4 | Returns the `LayerCursor` directly when only one layer exists | `state/src/rangeset.ts:690` | none | none | none | missing |
Proposed status: MISSING (searched: see `C6-api.md` state.cm6.187). R22b.

### state.cm6.188 `RangeSet^compare` — cm6 `state/src/rangeset.ts:366`
Helpers read: `findSharedChunks` `:570`, `SpanCursor` `:741-866`, `compare` `:868-896`, `sameValues` `:898`, `cmpVal` `:38`, `ChangeDesc.iterGaps` `change.ts:56`, `touchesRange` `change.ts:147`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^compare.B1 | `textDiff` is a `ChangeDesc` | `state/src/rangeset.ts:370` | takes `&ChangeSet`, uses `.desc()` `rangeset.rs:899`, `:908` | all compare tests | test_rangeset#025–#034 | differs (callers holding only a `ChangeDesc` cannot call it) |
| state.RangeSet^compare.B2 | Sets are filtered: keep `maxPoint > 0 \|\| !isEmpty && maxPoint >= minPointSize` | `state/src/rangeset.ts:376-377` | none (all sets used; empty layers skipped in `HeapCursor::from_sets` `rangeset.rs:556`) | test-rangeset#30 | none | differs only with minPointSize (B9) |
| state.RangeSet^compare.B3 | Chunks without non-empty points that are identical (same object) at the mapped position and untouched by the changes are skipped on both sides | `state/src/rangeset.ts:378`, `:570-582`, `:609`, `:648-650` | none | test-rangeset#29 | none | missing: R13 (changes the reported active arrays and split points) |
| state.RangeSet^compare.B4 | Only unchanged gaps (`iterGaps`) are compared; changed regions are never reported | `state/src/rangeset.ts:383` | gaps with `length > 0` `rangeset.rs:908-914`, plus an extra tail comparison `:915-922` | test-rangeset#28, #33 | none (test_rangeset#028, #033 assert nothing) | differs: R15 (tail region compared) |
| state.RangeSet^compare.B5 | Empty document (`textDiff.empty && length == 0`): compares at position 0 | `state/src/rangeset.ts:384` | none | none | none | missing: R14 |
| state.RangeSet^compare.B6 | Each gap starts with `goto(start)` side `-C.Far` | `state/src/rangeset.ts:872-873`, `:766` | `goto(start, -1_000_000)` `rangeset.rs:855-856` | none | none | differs: R16 |
| state.RangeSet^compare.B7 | `compareRange(pos, clipEnd, a.active, b.active)` when the active sets differ (`sameValues`, order-sensitive) and the piece is non-empty | `state/src/rangeset.ts:888` | `rangeset.rs:876-878` | test-rangeset#25–#27, #33 | none | implemented, untested in Ferrum (Replay #25–#27, #33 pass) |
| state.RangeSet^compare.B8 | `comparePoint` when the points differ, or when the active ranges covering the point (`activeForPoint`) differ | `state/src/rangeset.ts:880-885` | `rangeset.rs:867-874` | test-rangeset#30–#32, #34 | none | implemented, untested in Ferrum (Replay passes) |
| state.RangeSet^compare.B9 | `minPointSize` (default −1): ignore non-point ranges and points smaller than it | `state/src/rangeset.ts:374`, `:380-381` | none (`min_point = -1` hard-coded `rangeset.rs:902`) | none | none | missing: R23 |
| state.RangeSet^compare.B10 | `boundChange(pos)` when the boundary between ranges changes (`dEnd` or a different `openEnd`), not for points, only if the comparator defines it | `state/src/rangeset.ts:876-889` | none | none | none | missing: R17 |
| state.RangeSet^compare.B11 | Reported positions are in new-document coordinates, clipped to the gap end | `state/src/rangeset.ts:874-879` | `rangeset.rs:857-865` | test-rangeset#28 | none | implemented, untested in Ferrum (Replay #28 passes) |
| state.RangeSet^compare.B12 | Steps both cursors in lockstep (`diff <= 0` → a.next, `diff >= 0` → b.next), ending after the gap end | `state/src/rangeset.ts:891-894` | `rangeset.rs:881-890` | test-rangeset#33 | none | implemented, untested |
Proposed status: PORTED, DEVIATION because of B1, B3–B6, B9 and B10; Ferrum tests assert nothing. On identical new sets the oracle gives 994/4 500 raw call-list mismatches (`compareN`).

### state.cm6.189 `RangeSet^eq` — cm6 `state/src/rangeset.ts:389`
Helpers read: `findSharedChunks` `:570`, `SpanCursor` `:741`, `sameValues` `:898`, `cmpVal` `:38`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^eq.B1 | `to` defaults to `C.Far - 1`, `from` to 0 | `state/src/rangeset.ts:391-393` | none | none | none | missing |
| state.RangeSet^eq.B2 | Sets present (by identity) in both lists are ignored; empty sets ignored | `state/src/rangeset.ts:394-395` | none | none | none | missing |
| state.RangeSet^eq.B3 | A different number of remaining sets → false; none remaining → true | `state/src/rangeset.ts:396-397` | none | none | none | missing |
| state.RangeSet^eq.B4 | Walks both sides with `SpanCursor(…, minPoint 0)`: only point ranges are considered | `state/src/rangeset.ts:399` | none | none | none | missing (R30: marks differing only in value compare equal) |
| state.RangeSet^eq.B5 | Unequal when span ends, active sets or points differ | `state/src/rangeset.ts:401-404` | none | none | none | missing |
| state.RangeSet^eq.B6 | Returns true once past `to` | `state/src/rangeset.ts:405` | none | none | none | missing |
| state.RangeSet^eq.B7 | Shared chunks are skipped | `state/src/rangeset.ts:398` | none | none | none | missing |
Proposed status: MISSING (searched: see `C6-api.md` state.cm6.189).

### state.cm6.190 `RangeSet^spans` — cm6 `state/src/rangeset.ts:415`
Helpers read: `SpanCursor` `:741-866` (`goto`, `forward`, `next`, `addActive`, `removeActive`, `activeForPoint`, `openEnd`), `HeapCursor` `:670-723`, `LayerCursor` `:584-668`, `insert`/`remove`/`findMinIndex` `:904-921`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^spans.B1 | The whole `[from, to]` is covered by `span`/`point` calls in order; empty non-point pieces are skipped | `state/src/rangeset.ts:424-440` | `rangeset.rs:937-960` | test-rangeset#35, #36 | test_rangeset#035, #036 | implemented+tested |
| state.RangeSet^spans.B2 | `active` holds covering non-point ranges ordered by rank, then by `to` (bigger outside) | `state/src/rangeset.ts:790-799` | `rangeset.rs:719-740` | test-rangeset#37, #38, #43 | test_rangeset#037, #038, #043 | implemented+tested |
| state.RangeSet^spans.B3 | Ties in the heap (all keys equal) are resolved by swap-on-equal | `state/src/rangeset.ts:730`, `:734` | `rangeset.rs:623`, `:626` (no swap on equal) | none | none | differs: R18 (active order `a,b` vs `b,a`) |
| state.RangeSet^spans.B4 | `openStart` of a span = the number of trailing `active` entries that started before the span start | `state/src/rangeset.ts:844-847`, `:423`, `:435` | `rangeset.rs:796-803` counts every entry started before | none | none | differs: R11 |
| state.RangeSet^spans.B5 | `openEnd(to)` = the number of trailing `active` entries ending after `to` | `state/src/rangeset.ts:861-865` | `rangeset.rs:824-832` counts every entry | none | none | differs: R12 |
| state.RangeSet^spans.B6 | Point `openCount`: `pointFrom < from` → `active.length + 1`; `startSide < 0` → `active.length`; else `min(active.length, openRanges)` | `state/src/rangeset.ts:428-430` | `rangeset.rs:941-947` | none | none | implemented (R26 matches), untested |
| state.RangeSet^spans.B7 | Point `active` = ranges with `rank >= pointRank` that cover the point end (`activeForPoint`) | `state/src/rangeset.ts:850-859` | `rangeset.rs:806-822` | test-rangeset#40 | test_rangeset#040 | implemented+tested (oracle case 277 differs through B5/B11 effects) |
| state.RangeSet^spans.B8 | Point `index` = rank of the point's set | `state/src/rangeset.ts:431` | `rangeset.rs:948` | none | none | implemented, untested |
| state.RangeSet^spans.B9 | A non-empty point ending at the previous point's end is omitted; zero-length points at the same place are all reported | `state/src/rangeset.ts:829-831` | `rangeset.rs:781-782` | test-rangeset#39, #42 | test_rangeset#039, #042 | implemented+tested |
| state.RangeSet^spans.B10 | `minPointSize >= 0` ignores non-point ranges and smaller points | `state/src/rangeset.ts:420`, `:640` | `rangeset.rs:526`, `:931` | test-rangeset#41 | test_rangeset#041 | implemented+tested (R24 matches) |
| state.RangeSet^spans.B11 | After a point, cursors are forwarded past it (`forward`), which may jump a layer to a later chunk | `state/src/rangeset.ts:839`, `:622-625`, `:606-620` | `rangeset.rs:790`, `:493-498`, `:470-491` (`saturating_sub`) | test-rangeset#40 | test_rangeset#040 | differs: R19 (a point at a later chunk start is lost) |
| state.RangeSet^spans.B12 | Return value: open count at the end, +1 if a point extends past `to` | `state/src/rangeset.ts:437` | `rangeset.rs:954-957` | none | none | implemented (values differ through B5: R12b), untested |
Proposed status: PORTED, DEVIATION because of B3, B4, B5 and B11; open counts are untested on both sides.

### state.cm6.191 `RangeSet^of` — cm6 `state/src/rangeset.ts:448`
Helpers read: `lazySort` `:472`, `cmpRange` `:58`, `RangeSetBuilder.add` `:515`, `finish` `:557`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^of.B1 | Accepts a single `Range` or an array | `state/src/rangeset.ts:450` | `Vec` only `rangeset.rs:148` | none | none | differs: R22h |
| state.RangeSet^of.B2 | `sort = true` → `lazySort` (sorts a copy only if out of order; stable) | `state/src/rangeset.ts:450`, `:472-479` | `of_sorted(…, true)` always sorts (stable) `rangeset.rs:154-159` | test-rangeset#01, #44 (pre-sorted with `cmp`) | test_rangeset#001, #015, #016, #035–#044 | implemented+tested (same result) |
| state.RangeSet^of.B3 | Strict add: misordered input throws "Ranges must be added sorted by `from` position and `startSide`" | `state/src/rangeset.ts:451`, `:526-527` | `add_strict` → `Err(…)` `rangeset.rs:162`, `:374-376` | test-rangeset#02 | test_rangeset#002 (first assertion only) | differs: `Result` instead of a throw; misses R9 |
| state.RangeSet^of.B4 | Overlapping ranges are layered | `state/src/rangeset.ts:519-520` | `rangeset.rs:363-366` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSet^of.B5 | Empty input → `RangeSet.empty` | `state/src/rangeset.ts:562` | empty set (not a singleton) | none | none | implemented (no identity), untested |
Proposed status: PORTED, DEVIATION because of B1, B3 and B5.

### state.cm6.192 `RangeSet^join` — cm6 `state/src/rangeset.ts:456`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^join.B1 | Empty array → `RangeSet.empty` | `state/src/rangeset.ts:457` | none | none | none | missing |
| state.RangeSet^join.B2 | Layers of `sets[0..n-2]` are stacked on top of the last set, in order; `maxPoint` is the max | `state/src/rangeset.ts:458-462` | none | none | none | missing |
| state.RangeSet^join.B3 | No chunk is copied (layers reused by reference) | `state/src/rangeset.ts:461` | none | none | none | missing |
Proposed status: MISSING (R22a).

### state.cm6.193 `RangeSet^empty` — cm6 `state/src/rangeset.ts:467`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^empty.B1 | A singleton with no chunks, `maxPoint = -1`, `nextLayer = itself` | `state/src/rangeset.ts:467`, `:482` | `RangeSet::empty()` `rangeset.rs:107-114` (fresh value, `next_layer: None`) | test-rangeset#09, #18 | test_rangeset#009, #018 | differs: no singleton |
| state.RangeSet^empty.B2 | Operations return this exact object when the result is empty, so callers test `set == RangeSet.empty` | `state/src/rangeset.ts:298`, `:332`, `:562` | `is_empty()` `rangeset.rs:117` | test-rangeset#09, #18 | test_rangeset#018 | differs (identity replaced by `is_empty`; R7b) |
Proposed status: PORTED, DEVIATION.

### state.cm6.194 `RangeCursor` — cm6 `state/src/rangeset.ts:185`
Helpers read: `HeapCursor` `:670`, `LayerCursor` `:584`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.B1 | Starts pointing at the first range (read before calling `next`) | `state/src/rangeset.ts:181-184` | none (`Vec`) | test-rangeset#19, #44 | none | missing |
| state.RangeCursor.B2 | At the end `value` is `null` and `from = to = C.Far` | `state/src/rangeset.ts:630-631`, `:710-712` | none | test-rangeset#19 | none | missing |
Proposed status: MISSING.

### state.cm6.195 `RangeCursor.next` — cm6 `state/src/rangeset.ts:187`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.next.B1 | Advances lazily to the next range in heap order | `state/src/rangeset.ts:708-722`, `:627-643` | none (public); private `HeapCursor::next` `rangeset.rs:600` | test-rangeset#19, #44, #45 | none | missing |
Proposed status: MISSING.

### state.cm6.196 `RangeCursor.goto` — cm6 `state/src/rangeset.ts:189`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.goto.B1 | Re-positions at the first range ending at or after `pos` (side `-C.Far`) | `state/src/rangeset.ts:695-700`, `:600-604` | none (public); private `rangeset.rs:575` | none | none | missing (R22c) |
| state.RangeCursor.goto.B2 | Can move backwards (resets chunk/range index) | `state/src/rangeset.ts:601` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.197 `RangeCursor.value` — cm6 `state/src/rangeset.ts:192`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.value.B1 | The current value, `null` at the end | `state/src/rangeset.ts:192`, `:631`, `:711` | none (each `Range.value` in the `Vec`) | test-rangeset#19, #22 | none | missing |
Proposed status: MISSING.

### state.cm6.198 `RangeCursor.from` — cm6 `state/src/rangeset.ts:194`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.from.B1 | The current start; `C.Far` at the end | `state/src/rangeset.ts:194`, `:630` | none | test-rangeset#19, #22, #44, #45 | none | missing |
Proposed status: MISSING.

### state.cm6.199 `RangeCursor.to` — cm6 `state/src/rangeset.ts:196`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.to.B1 | The current end; `C.Far` at the end | `state/src/rangeset.ts:196`, `:630` | none | test-rangeset#22, #45 | none | missing |
Proposed status: MISSING.

### state.cm6.200 `RangeCursor.rank` — cm6 `state/src/rangeset.ts:199`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.rank.B1 | Index of the origin set; −1 at the end of a HeapCursor | `state/src/rangeset.ts:199`, `:712`, `:718` | none (private field `rangeset.rs:547`, `usize::MAX` at the end `:605`) | none | none | missing |
Proposed status: MISSING.

### state.cm6.201 `RangeSetBuilder` — cm6 `state/src/rangeset.ts:487`
Helpers read: `finishChunk` `:501`, `addRange` `:518`, `addInner` `:524`, `addChunk` `:542`, `finishInner` `:560`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.B1 | Builds a set without an intermediate `Range` array | `state/src/rangeset.ts:484-486` | `rangeset.rs:295-415` | indirect (all) | indirect (all via `of`) | implemented+tested |
| state.RangeSetBuilder.B2 | Chunks closed at 250 ranges | `state/src/rangeset.ts:529` | `rangeset.rs:382-384` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.B3 | Layering through a nested builder | `state/src/rangeset.ts:519-520`, `:564` | `rangeset.rs:363-366`, `:407` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.B4 | `addChunk`/`finishInner` let `update`/`map` reuse chunks and stack on an existing next layer | `state/src/rangeset.ts:542-567` | none | test-rangeset#03 | none | missing (internal; it is what makes R6 possible) |
Proposed status: PORTED, DEVIATION because of B4 and `add` (state.cm6.203).

### state.cm6.202 `RangeSetBuilder.constructor` — cm6 `state/src/rangeset.ts:511`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.constructor.B1 | Creates an empty builder (`lastFrom = lastTo = -C.Far`, `maxPoint = -1`) | `state/src/rangeset.ts:488-511` | `rangeset.rs:319-335` | indirect | indirect (`of_sorted` `rangeset.rs:160`) | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (`new()` + `Default`).

### state.cm6.203 `RangeSetBuilder.add` — cm6 `state/src/rangeset.ts:515`
Helpers read: `addRange` `:518`, `addInner` `:524-539`, `finishChunk` `:501`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.add.B1 | Strict: throws when the range is before the previous one by `(from, startSide)` and overlaps it (`diff <= 0`) | `state/src/rangeset.ts:515`, `:526-527` | `add` is non-strict `rangeset.rs:353-355`; the strict variant is `add_strict` `:358` | test-rangeset#02 | test_rangeset#002 (via `of`) | differs: R10 |
| state.RangeSetBuilder.add.B2 | The strict check also fires when `from == lastTo && startSide == last.endSide` and `(from, startSide)` precede the previous range's | `state/src/rangeset.ts:525-526` | `diff` never 0 (`rangeset.rs:371`) | none | none | differs: R9 |
| state.RangeSetBuilder.add.B3 | A range overlapping the previous one (`diff < 0`) goes to the next-layer builder | `state/src/rangeset.ts:528`, `:519-520` | `rangeset.rs:378-380`, `:363-366` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.add.B4 | Positions stored relative to the chunk start | `state/src/rangeset.ts:530-532` | `rangeset.rs:386-392` | test-rangeset#01 (`checkSet`) | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.add.B5 | Points update `maxPoint` | `state/src/rangeset.ts:537` | `rangeset.rs:397-399` | none | none | implemented, untested |
| state.RangeSetBuilder.add.B6 | `add` after `finish` throws (`from = null`) | `state/src/rangeset.ts:565` | `finish(self)` consumes the builder (compile time) | none | none | implemented (RUST-IDIOMATIC) |
Proposed status: PORTED, DEVIATION because of B1 and B2.

### state.cm6.204 `RangeSetBuilder.finish` — cm6 `state/src/rangeset.ts:557`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.finish.B1 | Closes the pending chunk and returns the set; with no ranges returns `RangeSet.empty` | `state/src/rangeset.ts:557-562` | `rangeset.rs:405-414` (an empty set) | indirect | indirect | implemented+tested (no singleton) |
| state.RangeSetBuilder.finish.B2 | Next-layer builders are finished recursively | `state/src/rangeset.ts:563-564` | `rangeset.rs:407` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.finish.B3 | The builder is unusable afterwards | `state/src/rangeset.ts:565` | consumes `self` | none | none | implemented (compile time) |
Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.205 `RangeComparator` — cm6 `state/src/rangeset.ts:63`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.B1 | Interface with `compareRange`, `comparePoint` and an optional `boundChange` | `state/src/rangeset.ts:63-75` | trait with two required methods `rangeset.rs:418-421` | test-rangeset#25–#34 | test_rangeset#025–#034 | differs: no `boundChange` |
| state.RangeComparator.B2 | Notified with new-document positions | `state/src/rangeset.ts:64-66` | same | test-rangeset#28 | none | implemented, untested |
Proposed status: PORTED, DEVIATION.

### state.cm6.206 `RangeComparator.compareRange` — cm6 `state/src/rangeset.ts:67`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.compareRange.B1 | Called for each non-empty piece where the active sets differ | `state/src/rangeset.ts:888` | `rangeset.rs:876-878` | test-rangeset#25–#27, #33 | none (no assertions) | implemented, untested |
| state.RangeComparator.compareRange.B2 | Ranges inside shared chunks do not appear in `activeA`/`activeB` | `state/src/rangeset.ts:378` | none | test-rangeset#29 | none | differs: R13 |
| state.RangeComparator.compareRange.B3 | Never called inside changed regions | `state/src/rangeset.ts:383` | tail region compared `rangeset.rs:915-922` | test-rangeset#28 | none | differs: R15 |
Proposed status: PORTED, DEVIATION.

### state.cm6.207 `RangeComparator.comparePoint` — cm6 `state/src/rangeset.ts:69`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.comparePoint.B1 | Called for changed, inserted or deleted points (`null` on the absent side) | `state/src/rangeset.ts:880-885` | `rangeset.rs:867-874` (`Option<&T>`) | test-rangeset#31, #32 | none | implemented, untested |
| state.RangeComparator.comparePoint.B2 | Called when the same point has a different covering range set ("different cover") | `state/src/rangeset.ts:883` | `rangeset.rs:871` | test-rangeset#34 | none | implemented, untested (Replay #34 passes) |
| state.RangeComparator.comparePoint.B3 | Points at a gap start with sides below `-1e6`, and points in an empty document, are reported | `state/src/rangeset.ts:384`, `:872` | none | none | none | differs: R14, R16 |
Proposed status: PORTED, DEVIATION.

### state.cm6.208 `RangeComparator.boundChange` — cm6 `state/src/rangeset.ts:74`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.boundChange.B1 | Optional; if absent, no bound tracking happens | `state/src/rangeset.ts:876` | none | none | none | missing |
| state.RangeComparator.boundChange.B2 | Called at the position where ranges used to be split (or are now split) although the covered values are equal | `state/src/rangeset.ts:887`, `:889` | none | none | none | missing (R17) |
| state.RangeComparator.boundChange.B3 | Not reported for point segments, nor at the gap end | `state/src/rangeset.ts:885`, `:889` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.209 `SpanIterator` — cm6 `state/src/rangeset.ts:80`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SpanIterator.B1 | The iterated range is fully covered by `span`/`point` calls | `state/src/rangeset.ts:77-79` | `rangeset.rs:424-427` | test-rangeset#35, #41 | test_rangeset#035, #041 | implemented+tested |
Proposed status: PORTED, DEVIATION through its members (rows 210, 211).

### state.cm6.210 `SpanIterator.span` — cm6 `state/src/rangeset.ts:85`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SpanIterator.span.B1 | Receives `(from, to, active)` for every non-point piece, `active` possibly empty | `state/src/rangeset.ts:434` | `rangeset.rs:951` | test-rangeset#35–#38, #43 | test_rangeset#035–#038, #043 | implemented+tested |
| state.SpanIterator.span.B2 | `openStart` = the number of active ranges continued from before the span | `state/src/rangeset.ts:423`, `:435` | `rangeset.rs:936`, `:952` | none | none | differs: R11, R12 |
| state.SpanIterator.span.B3 | Not called for empty pieces | `state/src/rangeset.ts:433` | `rangeset.rs:950` | test-rangeset#39 | test_rangeset#039 | implemented+tested |
Proposed status: PORTED, DEVIATION.

### state.cm6.211 `SpanIterator.point` — cm6 `state/src/rangeset.ts:92`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SpanIterator.point.B1 | Receives `(from, to, value)` clipped to the iterated range (from = current position) | `state/src/rangeset.ts:431` | `rangeset.rs:948` | test-rangeset#39, #40 | test_rangeset#039, #040 | implemented+tested |
| state.SpanIterator.point.B2 | `active` = higher-or-equal-precedence ranges covering the point | `state/src/rangeset.ts:427`, `:850-859` | `rangeset.rs:940`, `:806-822` | none (names not checked for points) | none | implemented, untested |
| state.SpanIterator.point.B3 | `openStart` rule (`active.length + 1` when the point started before `from`) | `state/src/rangeset.ts:428-430` | `rangeset.rs:941-947` | none | none | implemented (R26), untested |
| state.SpanIterator.point.B4 | Every point not shadowed is reported, including one at the start of a later chunk after a forward jump | `state/src/rangeset.ts:839`, `:616` | `rangeset.rs:790`, `:484` | none | none | differs: R19 |
Proposed status: PORTED, DEVIATION.

### state.cm6.260 `RangeSet.update spec.add` — cm6 `state/src/rangeset.ts:207`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.add.B1 | Default `[]` | `state/src/rangeset.ts:272` | `rangeset.rs:285` | test-rangeset#05 | test_rangeset#005 | implemented+tested |
| state.RangeSetUpdate.add.B2 | Must be sorted by `(from, startSide)` unless `sort` is true | `state/src/rangeset.ts:203-207` | always sorted with the existing ranges | none | none | differs: R8d |
| state.RangeSetUpdate.add.B3 | Added ranges become part of the set | `state/src/rangeset.ts:280-283` | `rangeset.rs:231-233` | test-rangeset#03, #04 | test_rangeset#003, #004 | implemented+tested |
Proposed status: PORTED, DEVIATION.

### state.cm6.261 `RangeSet.update spec.sort` — cm6 `state/src/rangeset.ts:210`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.sort.B1 | Default false | `state/src/rangeset.ts:272` | `rangeset.rs:286` | none | none | implemented (field), untested |
| state.RangeSetUpdate.sort.B2 | true → `add` is sorted with `cmpRange` before merging | `state/src/rangeset.ts:275` | field never read in `update` (`rangeset.rs:218-236`) | none | none | differs: ignored (R8) |
Proposed status: PORTED, DEVIATION.

### state.cm6.262 `RangeSet.update spec.filter` — cm6 `state/src/rangeset.ts:213`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.filter.B1 | Keeps the ranges for which it returns true | `state/src/rangeset.ts:290` | `rangeset.rs:222-228` | test-rangeset#05–#07, #09 | test_rangeset#005–#007, #009 | implemented+tested |
| state.RangeSetUpdate.filter.B2 | Called exactly once per range touching the filter range, in position order | `state/src/rangeset.ts:290` | `rangeset.rs:222` (sorted `Vec`) | test-rangeset#08 | test_rangeset#008 (count between 200 and 400) | implemented, untested exactly (R29, Replay #08 match) |
| state.RangeSetUpdate.filter.B3 | Not called for ranges inside reused chunks outside the filter range | `state/src/rangeset.ts:286` | the equivalent condition per range `rangeset.rs:223` | test-rangeset#08 | test_rangeset#008 (weak) | implemented |
Proposed status: PORTED, DEVIATION because B2 is untested exactly in Ferrum.

### state.cm6.263 `RangeSet.update spec.filterFrom` — cm6 `state/src/rangeset.ts:217`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.filterFrom.B1 | Default 0 | `state/src/rangeset.ts:272` | `rangeset.rs:288` | test-rangeset#05 | test_rangeset#005 | implemented+tested |
| state.RangeSetUpdate.filterFrom.B2 | Ranges with `to < filterFrom` are kept without calling the filter | `state/src/rangeset.ts:290` | `rangeset.rs:223` | test-rangeset#08 | test_rangeset#008 (weak) | implemented, untested exactly |
Proposed status: PORTED, DEVIATION.

### state.cm6.264 `RangeSet.update spec.filterTo` — cm6 `state/src/rangeset.ts:219`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.filterTo.B1 | Default `this.length` | `state/src/rangeset.ts:272` | `usize::MAX` `rangeset.rs:289` | test-rangeset#05 | test_rangeset#005 | implemented+tested (equivalent) |
| state.RangeSetUpdate.filterTo.B2 | Ranges with `from > filterTo` are kept without calling the filter | `state/src/rangeset.ts:290` | `rangeset.rs:223` | test-rangeset#08 | test_rangeset#008 (weak) | implemented, untested exactly |
Proposed status: PORTED, DEVIATION.

## Unfinished

none
