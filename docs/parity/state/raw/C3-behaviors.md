# C3 behaviors

> What: chunk C3 behavior sections. Versions: cm6 `@codemirror/state` 6.7.6 (`CodeMirror6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`ferrum/crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `ferrum/docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `ferrum/docs/parity/state/raw/C3-api.md` § Reproductions.

41 cm6 items, none DROP; one section per item (state.cm6.061–091, 166–169, 212–217), plus one helper section (`makeCategorizer`), which has no export of its own but is needed to classify `CharCategory` results, Ferrum `categorize_char` and the test-charcategory cases.

Reproduction labels (S.., C.., P.., F.., Z.., K.., J.., T.., G.., W..) refer to `ferrum/docs/parity/state/raw/C3-api.md` `## Reproductions` R1–R8. "untested" means that no Ferrum test exercises the behavior (the only C3 Ferrum tests are test_selection#001–#004 and test_charcategory#001–#003; a grep of every `tests` directory in the workspace found no other test calling `find_cluster_break`, `count_column`, `find_column`, `SelectionRange::map/extend`, `EditorSelection::map` or `categorize_char`).

Shared helpers read for the selection items: `RangeFlag` `CodeMirror6/state/src/selection.ts:12-18`, `SelectionRange.create` `CodeMirror6/state/src/selection.ts:109`, `EditorSelection.normalized` `CodeMirror6/state/src/selection.ts:223-236`, `checkSelection` `CodeMirror6/state/src/selection.ts:239-242`, `ChangeDesc.mapPos` / `ChangeDesc.empty` `CodeMirror6/state/src/change.ts` (C2; read only for the mapping semantics). Ferrum: `ChangeDesc::map_pos` `ferrum/crates/ferrum-state/src/change.rs:149`, `ChangeDesc::is_empty` `ferrum/crates/ferrum-state/src/change.rs:73`.

---

### state.cm6.061 `EditorSelection` — cm6 `CodeMirror6/state/src/selection.ts:115`
Helpers read: constructor `CodeMirror6/state/src/selection.ts:116-123`, `checkSelection` `CodeMirror6/state/src/selection.ts:239`, callers `CodeMirror6/state/src/state.ts:250`, `CodeMirror6/state/src/transaction.ts:185`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.B1 | Private constructor: instances only from `create`, `single`, `fromJSON`, `map`, `asSingle`, `addRange`, `replaceRange` | `CodeMirror6/state/src/selection.ts:116` | private fields `ferrum/crates/ferrum-state/src/selection.rs:118-119`; factories `create` :147, `single` :138, `cursor` :130, `Default` :122, `map` :230 | test-selection#01–#04 | test_selection#001–#004 | implemented+tested |
| state.EditorSelection.B2 | At least one range: `create([])` throws `RangeError("A selection needs at least one range")`; only `fromJSON({ranges:[],main:-1})` can build an empty selection (S6B) | `CodeMirror6/state/src/selection.ts:189`, `:176` | `ferrum/crates/ferrum-state/src/selection.rs:148-150` returns `cursor(0)` | none | none | differs: no error, silently a cursor at 0 (S73) |
| state.EditorSelection.B3 | `checkSelection`: throws `RangeError("Selection points outside of document")` when any `range.to > docLength`; called by `EditorState.create` and the `Transaction` constructor; no lower-bound check (S99) | `CodeMirror6/state/src/selection.ts:239-242`; `CodeMirror6/state/src/state.ts:250`; `CodeMirror6/state/src/transaction.ts:185` | none (searched `check_selection`, "outside of document" in all crates; `ferrum/crates/ferrum-state/src/state.rs:107`, `:269` accept any selection) | none in C3 files | none | missing: ferrum accepts head 5 on a 3-char doc (S97) and head 9 via update (S98) |

Proposed status: PORTED, DEVIATION because B2 differs and B3 is missing.

### state.cm6.062 `EditorSelection.ranges` — cm6 `CodeMirror6/state/src/selection.ts:119`
Helpers read: `EditorSelection.create`, `normalized`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.ranges.B1 | Ranges sorted by `from`, non-overlapping; non-empty ranges may touch; an empty range at the end of a non-empty one is merged, at the start it is kept (S76–S78) | `CodeMirror6/state/src/selection.ts:117-119`, `:229` | `ferrum/crates/ferrum-state/src/selection.rs:219`, invariant from `create` :147-203 | test-selection#02, #03 | test_selection#002, #003 | implemented+tested |
| state.EditorSelection.ranges.B2 | Readonly array; elements immutable | `CodeMirror6/state/src/selection.ts:119` | `&[SelectionRange]` shared slice `ferrum/crates/ferrum-state/src/selection.rs:219` | test-selection#01 | test_selection#001 | implemented+tested (Rust borrow instead of readonly) |

Proposed status: PORTED, RUST-IDIOMATIC because the contents are identical (R4) and only the accessor form changes (field → method). The difference in range `assoc` is recorded on state.cm6.084.

### state.cm6.063 `EditorSelection.mainIndex` — cm6 `CodeMirror6/state/src/selection.ts:122`
Helpers read: `create` :188, `normalized` :223, `fromJSON` :175.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.mainIndex.B1 | Index of the main range | `CodeMirror6/state/src/selection.ts:122` | `ferrum/crates/ferrum-state/src/selection.rs:213` | test-selection#01 | test_selection#001 (through `main()`) | implemented+tested |
| state.EditorSelection.mainIndex.B2 | Not validated: may be ≥ length (create fast path, S74), −1 (normalize path, S75), negative or fractional (fromJSON, S6B/S6C) | `CodeMirror6/state/src/selection.ts:195`, `:226`, `:178` | clamped to `len-1` `ferrum/crates/ferrum-state/src/selection.rs:164`, `:172`, `:198` | none | none | differs (clamping), untested |

Proposed status: PORTED, DEVIATION because B2 differs.

### state.cm6.064 `EditorSelection.map` — cm6 `CodeMirror6/state/src/selection.ts:127`
Helpers read: `SelectionRange.map` :73, `create` :188, `ChangeDesc.empty` (C2).

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.map.B1 | Empty change → returns `this` (S90) | `CodeMirror6/state/src/selection.ts:128` | `ferrum/crates/ferrum-state/src/selection.rs:231-233` returns `self.clone()` | none | none | implemented (equal value; identity is not observable in Rust), untested |
| state.EditorSelection.map.B2 | `assoc` defaults to −1 | `CodeMirror6/state/src/selection.ts:127` | mandatory `assoc: i8` `ferrum/crates/ferrum-state/src/selection.rs:230` | none | none | implemented (no default), untested |
| state.EditorSelection.map.B3 | Every range mapped with `range.map(change, assoc)` | `CodeMirror6/state/src/selection.ts:129` | `ferrum/crates/ferrum-state/src/selection.rs:234` | none | none | implemented, untested (range-level differences in state.cm6.087) |
| state.EditorSelection.map.B4 | Result re-normalized with `create(mapped, this.mainIndex)` (merging collapsed ranges, S91) | `CodeMirror6/state/src/selection.ts:129` | `ferrum/crates/ferrum-state/src/selection.rs:235` | none | none | implemented, untested; inherits main-index divergence state.EditorSelection.create.B5 (R4 map: 4999/5000) |
| state.EditorSelection.map.B5 | Position beyond change length → `RangeError` from `mapPos` (S34) | `CodeMirror6/state/src/change.ts` (mapPos, C2) | panic in `ferrum/crates/ferrum-state/src/change.rs:192` | none | none | implemented (panic instead of exception), untested |

Proposed status: PORTED, DEVIATION because B1–B5 are untested and B4 inherits the create main-index divergence.

### state.cm6.065 `EditorSelection.eq` — cm6 `CodeMirror6/state/src/selection.ts:136`
Helpers read: `SelectionRange.eq` :92.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.eq.B1 | Different range count → false (S5A) | `CodeMirror6/state/src/selection.ts:137` | derived `PartialEq` `ferrum/crates/ferrum-state/src/selection.rs:116` (SmallVec equality) | none | none | implemented, untested |
| state.EditorSelection.eq.B2 | Different `mainIndex` → false (S57) | `CodeMirror6/state/src/selection.ts:138` | derived, `main_index` field | none | none | implemented, untested |
| state.EditorSelection.eq.B3 | Range-by-range `SelectionRange.eq(other, includeAssoc)` | `CodeMirror6/state/src/selection.ts:139-140` | derived `SelectionRange: PartialEq` `selection.rs:6` | none | none | differs (see state.SelectionRange.eq.B4) |
| state.EditorSelection.eq.B4 | Default `includeAssoc = false`: cursors equal regardless of assoc (S58 → true) | `CodeMirror6/state/src/selection.ts:136` | always compares `assoc` | none | none | differs: ferrum false |
| state.EditorSelection.eq.B5 | `includeAssoc = true`: cursor assoc compared (S59) | `CodeMirror6/state/src/selection.ts:136` | no parameter | none | none | missing (ferrum's `==` behaves like includeAssoc=true for cursors but also compares assoc of non-empty ranges) |

Proposed status: PORTED, DEVIATION because B3–B5 differ and nothing is tested.

### state.cm6.066 `EditorSelection.main` — cm6 `CodeMirror6/state/src/selection.ts:147`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.main.B1 | Returns `ranges[mainIndex]` | `CodeMirror6/state/src/selection.ts:147` | `ferrum/crates/ferrum-state/src/selection.rs:207-209` | test-selection#01 | test_selection#001 | implemented+tested |

Proposed status: PORTED because its single behavior is implemented and tested (out-of-range indices are a property of `mainIndex`/`create`, recorded there).

### state.cm6.067 `EditorSelection.asSingle` — cm6 `CodeMirror6/state/src/selection.ts:151`
Helpers read: `main` :147; callers `CodeMirror6/state/src/state.ts:123`, `:251`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.asSingle.B1 | One range → returns `this` (S82) | `CodeMirror6/state/src/selection.ts:152` | none | none | none | missing |
| state.EditorSelection.asSingle.B2 | Otherwise `[main]` with mainIndex 0 (S81); used by `EditorState.create` and `applyTransaction` when `allowMultipleSelections` is false | `CodeMirror6/state/src/selection.ts:152`; `CodeMirror6/state/src/state.ts:123`, `:251` | none (R6: ferrum keeps 2 ranges) | none in C3 files (test-state "blocks multiple selections", C5) | none | missing |

Proposed status: MISSING (searched `as_single`, `asSingle`, `collapse`, uses of `allow_multiple_selections` in all crates).

### state.cm6.068 `EditorSelection.addRange` — cm6 `CodeMirror6/state/src/selection.ts:156`
Helpers read: `create` :188, `normalized` :223.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.addRange.B1 | New range prepended, then `create` normalizes (merging, S85) | `CodeMirror6/state/src/selection.ts:157` | none | none | none | missing |
| state.EditorSelection.addRange.B2 | `main = true` (default): new range becomes main (index 0 before sort, tracked through sort, S83 → main=2) | `CodeMirror6/state/src/selection.ts:157` | none | none | none | missing |
| state.EditorSelection.addRange.B3 | `main = false`: previous main stays main (`mainIndex + 1`, S84) | `CodeMirror6/state/src/selection.ts:157` | none | none | none | missing |

Proposed status: MISSING (searched `add_range`; only `RangeSetBuilder::add_range` `ferrum/crates/ferrum-state/src/rangeset.rs:362`).

### state.cm6.069 `EditorSelection.replaceRange` — cm6 `CodeMirror6/state/src/selection.ts:162`
Helpers read: `create` :188.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.replaceRange.B1 | Copies ranges, replaces index `which` (default `mainIndex`) (S87, S88) | `CodeMirror6/state/src/selection.ts:163-164` | none | none | none | missing |
| state.EditorSelection.replaceRange.B2 | Re-normalizes with the old `mainIndex` (S89: merge into one range) | `CodeMirror6/state/src/selection.ts:165` | none | none | none | missing |
| state.EditorSelection.replaceRange.B3 | `which` beyond length → sparse array → `TypeError` in create (S8A) | `CodeMirror6/state/src/selection.ts:164` | none | none | none | missing |

Proposed status: MISSING (searched `replace_range`, `replace_main`, `with_range`).

### state.cm6.070 `EditorSelection.toJSON` — cm6 `CodeMirror6/state/src/selection.ts:170`
Helpers read: `SelectionRange.toJSON` :98.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.toJSON.B1 | `{ranges: [{anchor, head}…], main: mainIndex}` (S66); offsets in UTF-16 | `CodeMirror6/state/src/selection.ts:171` | none | none in C3 files (test-state "can be serialized to JSON", C5) | none | missing |

Proposed status: MISSING (searched `to_json`, `Serialize`, `serde`).

### state.cm6.071 `EditorSelection^fromJSON` — cm6 `CodeMirror6/state/src/selection.ts:175`
Helpers read: `SelectionRange.fromJSON` :102; callers `CodeMirror6/state/src/state.ts:235`, `CodeMirror6/commands/src/history.ts:211-212`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.fromJSON.B1 | Validation: falsy json, non-array `ranges`, non-number `main`, `main >= ranges.length` → `RangeError("Invalid JSON representation for EditorSelection")` (S68, S69) | `CodeMirror6/state/src/selection.ts:176-177` | none | none | none | missing |
| state.EditorSelection.fromJSON.B2 | Each range through `SelectionRange.fromJSON` (S67) | `CodeMirror6/state/src/selection.ts:178` | none | none | none | missing |
| state.EditorSelection.fromJSON.B3 | No normalization: unsorted/overlapping ranges kept (S6A) | `CodeMirror6/state/src/selection.ts:178` (private constructor, not `create`) | none | none | none | missing |
| state.EditorSelection.fromJSON.B4 | Accepts `main` negative or fractional and empty `ranges` with `main < 0` (S6B, S6C) | `CodeMirror6/state/src/selection.ts:176` | none | none | none | missing |
| state.EditorSelection.fromJSON.B5 | Errors of a range propagate (`SelectionRange` RangeError, S6D) | `CodeMirror6/state/src/selection.ts:178` | none | none | none | missing |

Proposed status: MISSING (searched `from_json`, `Deserialize`).

### state.cm6.072 `EditorSelection^single` — cm6 `CodeMirror6/state/src/selection.ts:182`
Helpers read: `EditorSelection.range` :208.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.single.B1 | `head` defaults to `anchor` (S70) | `CodeMirror6/state/src/selection.ts:182` | `head` required `ferrum/crates/ferrum-state/src/selection.rs:138` (`EditorSelection::cursor(pos)` :130 covers the one-argument form) | none | none | implemented (split into two functions), untested |
| state.EditorSelection.single.B2 | Range built with `EditorSelection.range` → automatic assoc for non-empty (S71 assoc 1) | `CodeMirror6/state/src/selection.ts:183` | `SelectionRange::new` → assoc 0 `selection.rs:140` | none | none | differs |
| state.EditorSelection.single.B3 | mainIndex 0, no normalization, no bounds check | `CodeMirror6/state/src/selection.ts:183` | `ferrum/crates/ferrum-state/src/selection.rs:139-142` | none | none | implemented, untested |

Proposed status: PORTED, DEVIATION because B2 differs and nothing is tested.

### state.cm6.073 `EditorSelection^create` — cm6 `CodeMirror6/state/src/selection.ts:188`
Helpers read: `normalized` :223-236, `EditorSelection.range` :208, `SelectionRange.empty` :47.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.create.B1 | Empty input → `RangeError` | `CodeMirror6/state/src/selection.ts:189` | `ferrum/crates/ferrum-state/src/selection.rs:148-150` → `cursor(0)` | none | none | differs (S73) |
| state.EditorSelection.create.B2 | Fast-path check from `pos = 0` including index 0: empty range needs `from > pos`, non-empty `from >= pos` (an initial cursor at 0 takes the normalize path, same result) | `CodeMirror6/state/src/selection.ts:190-194` | `ferrum/crates/ferrum-state/src/selection.rs:153-161` (skips index 0) | test-selection#01 | test_selection#001 | implemented+tested (index-0 difference not observable: S79 equal) |
| state.EditorSelection.create.B3 | Fast path returns the given ranges and mainIndex unchanged (even if out of range, S74) | `CodeMirror6/state/src/selection.ts:195` | `ferrum/crates/ferrum-state/src/selection.rs:163-169` clamps main | test-selection#01 | test_selection#001 (in-range only) | differs for out-of-range main, untested |
| state.EditorSelection.create.B4 | Normalize: copy, remember `main = ranges[mainIndex]`, stable sort by `from` | `CodeMirror6/state/src/selection.ts:192`, `:224-225` | `ferrum/crates/ferrum-state/src/selection.rs:172-173` (`sort_by_key`, stable) | test-selection#02 | test_selection#002 | implemented+tested |
| state.EditorSelection.create.B5 | Main located after sort by object identity (`indexOf`) | `CodeMirror6/state/src/selection.ts:226` | value equality `ferrum/crates/ferrum-state/src/selection.rs:174` | none | none | differs: `create([cursor(12), range(12,18), cursor(12)], 2)` → cm6 main=1 \| ferrum main=0 (R4) |
| state.EditorSelection.create.B6 | Merge condition: empty range merges when `from <= prev.to`, non-empty when `from < prev.to` | `CodeMirror6/state/src/selection.ts:229` | `ferrum/crates/ferrum-state/src/selection.rs:180` | test-selection#02, #03 | test_selection#002, #003 | implemented+tested |
| state.EditorSelection.create.B7 | Merged range spans `prev.from` … `max(range.to, prev.to)` | `CodeMirror6/state/src/selection.ts:230` | `ferrum/crates/ferrum-state/src/selection.rs:181-182` | test-selection#02, #03 | test_selection#002, #003 | implemented+tested |
| state.EditorSelection.create.B8 | `mainIndex--` when `i <= mainIndex` (main tracking through merges, S7E/S7F) | `CodeMirror6/state/src/selection.ts:231` | `ferrum/crates/ferrum-state/src/selection.rs:183-185` | none (cm6 merge tests use main 0) | none | implemented, untested |
| state.EditorSelection.create.B9 | Merged range takes the direction of the later range and is built by `EditorSelection.range` → automatic assoc ±1; goalColumn, bidiLevel, undirectional dropped (S7A–S7D) | `CodeMirror6/state/src/selection.ts:232` | `ferrum/crates/ferrum-state/src/selection.rs:186-190` (`SelectionRange::new`: assoc 0, goal None) | test-selection#04 (direction) | test_selection#004 (direction) | direction implemented+tested; assoc differs (S7A: −1 vs 0, S7B: 1 vs 0) |
| state.EditorSelection.create.B10 | Out-of-range main on the normalize path → `main` undefined → mainIndex −1 (S75) | `CodeMirror6/state/src/selection.ts:224-226` | clamped `ferrum/crates/ferrum-state/src/selection.rs:172`, `:198` | none | none | differs, untested |

Proposed status: PORTED, DEVIATION because B1, B3, B5, B9 (assoc), B10 differ and B8 is untested.

### state.cm6.074 `EditorSelection^cursor` — cm6 `CodeMirror6/state/src/selection.ts:200`
Helpers read: `SelectionRange.create` :109, `RangeFlag` :12.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.cursor.B1 | Empty range at `pos` | `CodeMirror6/state/src/selection.ts:201-202` | `SelectionRange::cursor` `ferrum/crates/ferrum-state/src/selection.rs:20-27` | none | none (used in test_state setups, C5) | implemented, untested in C3 |
| state.EditorSelection.cursor.B2 | `assoc`: 0 → none, negative → AssocBefore (−1), positive → AssocAfter (1, any magnitude: S10) | `CodeMirror6/state/src/selection.ts:202` | no parameter; field `assoc` writable | none | none | missing parameter |
| state.EditorSelection.cursor.B3 | `bidiLevel`: null → unset (7), else `min(6, level)` (S09) | `CodeMirror6/state/src/selection.ts:203` | none | none | none | missing |
| state.EditorSelection.cursor.B4 | `goalColumn` stored | `CodeMirror6/state/src/selection.ts:204` | via `with_goal_column` `selection.rs:64` or field | none | none | implemented differently, untested |

Proposed status: PORTED, DEVIATION because B2–B3 are missing and nothing is tested. Note the name clash: Ferrum `EditorSelection::cursor` (`selection.rs:130`) returns an `EditorSelection`.

### state.cm6.075 `EditorSelection^range` — cm6 `CodeMirror6/state/src/selection.ts:208`
Helpers read: `SelectionRange.create` :109, `RangeFlag` :12.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.range.B1 | `bidiLevel`: null → 7 (unset), else `min(6, level)` (S05) | `CodeMirror6/state/src/selection.ts:209` | none | none | none | missing |
| state.EditorSelection.range.B2 | No explicit assoc and `anchor != head` → assoc = `head < anchor ? 1 : -1` (S01, S02) | `CodeMirror6/state/src/selection.ts:210` | `SelectionRange::new` sets 0 `ferrum/crates/ferrum-state/src/selection.rs:30-37` | none | none | differs |
| state.EditorSelection.range.B3 | Explicit `assoc` argument (also on empty ranges, S06) | `CodeMirror6/state/src/selection.ts:211` | none (field write) | none | none | missing |
| state.EditorSelection.range.B4 | `head < anchor` → stored `from = head, to = anchor` + Inverted flag; anchor/head reported back unchanged | `CodeMirror6/state/src/selection.ts:212-213` | stores anchor/head directly, `from()`/`to()` :41-49 | test-selection#01, #04 | test_selection#001, #004 | implemented+tested |
| state.EditorSelection.range.B5 | `goalColumn` stored | `CodeMirror6/state/src/selection.ts:212-213` | `with_goal_column` :64 | none | none | implemented differently, untested |

Proposed status: PORTED, DEVIATION because B1–B3 are missing/different.

### state.cm6.076 `EditorSelection^undirectionalRange` — cm6 `CodeMirror6/state/src/selection.ts:218`
Helpers read: `SelectionRange.create` :109.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.undirectionalRange.B1 | Flags = `Undirectional` only: assoc 0, bidiLevel **0** (not null), not inverted (S11, S12) | `CodeMirror6/state/src/selection.ts:219` | none | none | none | missing |
| state.EditorSelection.undirectionalRange.B2 | `from`/`to` stored as given, no ordering: `(5,2)` gives from 5 > to 2 (S11) | `CodeMirror6/state/src/selection.ts:219` | none | none | none | missing |
| state.EditorSelection.undirectionalRange.B3 | No goalColumn | `CodeMirror6/state/src/selection.ts:219` | none | none | none | missing |

Proposed status: MISSING (searched `undirectional`, `undecided`, `directionless` in all crates).

### state.cm6.077 `SelectionRange` — cm6 `CodeMirror6/state/src/selection.ts:24`
Helpers read: `RangeFlag` :12-18, constructor :25-36, `SelectionRange.create` :109.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.B1 | Stores `from`, `to`, `flags`, `goalColumn`; anchor/head derived | `CodeMirror6/state/src/selection.ts:25-36` | stores `anchor`, `head`, `goal_column`, `assoc` `ferrum/crates/ferrum-state/src/selection.rs:7-16` | test-selection#01 | test_selection#001 | implemented differently (observable values equal for anchor/head/from/to) |
| state.SelectionRange.B2 | Flags: bidi level (3 bits, 7 = unset), assoc before/after, inverted, undirectional | `CodeMirror6/state/src/selection.ts:3-18` | only `assoc: i8`; inversion implicit (`head < anchor`) | none | none | differs: no bidi, no undirectional |
| state.SelectionRange.B3 | Private constructor and readonly fields: ranges only from factories, immutable | `CodeMirror6/state/src/selection.ts:25`, `:27-35` | all fields `pub`, `Copy`, mutable | none | none | differs (any `i8` assoc, any anchor/head) |
| state.SelectionRange.B4 | Identity-based reuse: `map` returns `this` when unchanged (S31) | `CodeMirror6/state/src/selection.ts:81` | value type | none | none | not observable in Rust (no identity) |

Proposed status: PORTED, DEVIATION because B2–B3 differ.

### state.cm6.078 `SelectionRange.from` — cm6 `CodeMirror6/state/src/selection.ts:27`
Helpers read: `EditorSelection.range` :212, `undirectionalRange` :219.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.from.B1 | Lower boundary | `CodeMirror6/state/src/selection.ts:27` | `min(anchor, head)` `ferrum/crates/ferrum-state/src/selection.rs:41-43` | test-selection#01 | test_selection#001 | implemented+tested |
| state.SelectionRange.from.B2 | `undirectionalRange(5,2)` stores from 5 (> to) | `CodeMirror6/state/src/selection.ts:219` | not constructible | none | none | not applicable without undirectionalRange (recorded on state.cm6.076) |

Proposed status: PORTED, RUST-IDIOMATIC (field → method, same value; B2 belongs to a missing constructor).

### state.cm6.079 `SelectionRange.to` — cm6 `CodeMirror6/state/src/selection.ts:29`
Helpers read: as `from`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.to.B1 | Upper boundary | `CodeMirror6/state/src/selection.ts:29` | `max(anchor, head)` `ferrum/crates/ferrum-state/src/selection.rs:47-49` | test-selection#01 | test_selection#001 | implemented+tested |
| state.SelectionRange.to.B2 | `undirectionalRange(5,2)` stores to 2 (< from) | `CodeMirror6/state/src/selection.ts:219` | not constructible | none | none | not applicable (state.cm6.076) |

Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.080 `SelectionRange.goalColumn` — cm6 `CodeMirror6/state/src/selection.ts:35`
Helpers read: `cursor` :200, `range` :208, `map` :81, `extend` :85, `normalized` :232.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.goalColumn.B1 | Set only by `cursor`/`range` arguments; readonly | `CodeMirror6/state/src/selection.ts:35`, `:204`, `:212` | `pub goal_column` + `with_goal_column` `ferrum/crates/ferrum-state/src/selection.rs:13`, `:64` | none | none | implemented differently (mutable), untested |
| state.SelectionRange.goalColumn.B2 | Kept by `map` (S29) | `CodeMirror6/state/src/selection.ts:81` | `ferrum/crates/ferrum-state/src/selection.rs:92` | none | none | implemented, untested |
| state.SelectionRange.goalColumn.B3 | Dropped by `extend` (S46) | `CodeMirror6/state/src/selection.ts:86-88` | `ferrum/crates/ferrum-state/src/selection.rs:100`, `:109` | none | none | implemented, untested |
| state.SelectionRange.goalColumn.B4 | Dropped when ranges merge in `create` (S7C) | `CodeMirror6/state/src/selection.ts:232` | `ferrum/crates/ferrum-state/src/selection.rs:186-190` | none | none | implemented, untested |

Proposed status: PORTED, DEVIATION because B1–B4 are untested and the field is mutable (`usize` instead of `number`).

### state.cm6.081 `SelectionRange.anchor` — cm6 `CodeMirror6/state/src/selection.ts:40`
Helpers read: `RangeFlag.Inverted` :16.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.anchor.B1 | `Inverted ? to : from` | `CodeMirror6/state/src/selection.ts:40` | stored field `ferrum/crates/ferrum-state/src/selection.rs:9` | test-selection#01, #04 | test_selection#001, #004 | implemented+tested |

Proposed status: PORTED, RUST-IDIOMATIC (stored instead of derived; same values).

### state.cm6.082 `SelectionRange.head` — cm6 `CodeMirror6/state/src/selection.ts:44`
Helpers read: `RangeFlag.Inverted` :16.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.head.B1 | `Inverted ? from : to` | `CodeMirror6/state/src/selection.ts:44` | stored field `ferrum/crates/ferrum-state/src/selection.rs:11` | test-selection#01, #04 | test_selection#001, #004 | implemented+tested |

Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.083 `SelectionRange.empty` — cm6 `CodeMirror6/state/src/selection.ts:47`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.empty.B1 | `from == to` | `CodeMirror6/state/src/selection.ts:47` | `anchor == head` `ferrum/crates/ferrum-state/src/selection.rs:53-55` | test-selection#03 (merge branch) | test_selection#003 | implemented+tested |

Proposed status: PORTED, RUST-IDIOMATIC (getter → `is_empty()`).

### state.cm6.084 `SelectionRange.assoc` — cm6 `CodeMirror6/state/src/selection.ts:53`
Helpers read: `RangeFlag` :14-15, `cursor` :202, `range` :210-211, `map` :81, `extend` :86-88, `normalized` :232; consumers `CodeMirror6/view/src/cursor.ts` (`moveVertically` passes `start.assoc`), `CodeMirror6/view/src/bidi.ts:438`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.assoc.B1 | Decoded from flags: AssocBefore → −1, AssocAfter → 1, else 0 (only −1/0/1) | `CodeMirror6/state/src/selection.ts:53` | `pub assoc: i8` `ferrum/crates/ferrum-state/src/selection.rs:15` (any i8) | none | none | differs (type range), untested |
| state.SelectionRange.assoc.B2 | Automatically ±1 for non-empty ranges built by `range()` (forward −1, inverted 1) | `CodeMirror6/state/src/selection.ts:210` | constructors give 0 `selection.rs:25`, `:35` | none | none | differs (S01, S02) |
| state.SelectionRange.assoc.B3 | Preserved through `map` (S22, S29) | `CodeMirror6/state/src/selection.ts:81` | `ferrum/crates/ferrum-state/src/selection.rs:93` | none | none | implemented, untested |
| state.SelectionRange.assoc.B4 | Recomputed by `extend` and by merges (via `range()`) | `CodeMirror6/state/src/selection.ts:86-88`, `:232` | reset to 0 `selection.rs:100`, `:109`, `:186-190` | none | none | differs (S45, S49, S7A) |

Proposed status: PORTED, DEVIATION because B1, B2, B4 differ and nothing is tested.

### state.cm6.085 `SelectionRange.undirectional` — cm6 `CodeMirror6/state/src/selection.ts:60`
Helpers read: `RangeFlag.Undirectional` :17, `undirectionalRange` :218, consumer `CodeMirror6/view/src/cursor.ts:167`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.undirectional.B1 | True only for ranges from `undirectionalRange` | `CodeMirror6/state/src/selection.ts:60-62` | none | none | none | missing |
| state.SelectionRange.undirectional.B2 | Kept by `map` (S30), dropped by `extend`/merge (S7C) | `CodeMirror6/state/src/selection.ts:81`, `:86-88`, `:232` | none | none | none | missing |

Proposed status: MISSING.

### state.cm6.086 `SelectionRange.bidiLevel` — cm6 `CodeMirror6/state/src/selection.ts:66`
Helpers read: `RangeFlag.BidiLevelMask` :13, `cursor` :203, `range` :209, `undirectionalRange` :219.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.bidiLevel.B1 | `null` when the 3 bits are 7 (unset) | `CodeMirror6/state/src/selection.ts:66-69` | none | none | none | missing |
| state.SelectionRange.bidiLevel.B2 | Levels clamped to 0–6 at construction (S05, S09) | `CodeMirror6/state/src/selection.ts:203`, `:209` | none | none | none | missing |
| state.SelectionRange.bidiLevel.B3 | Kept by `map` (S29), dropped by `extend`/merge; `undirectionalRange` yields 0 (S12) | `CodeMirror6/state/src/selection.ts:81`, `:219` | none | none | none | missing |

Proposed status: MISSING (searched `bidi`, `bidi_level` in all crates).

### state.cm6.087 `SelectionRange.map` — cm6 `CodeMirror6/state/src/selection.ts:73`
Helpers read: `ChangeDesc.mapPos` (C2, default `MapMode.Simple`), constructor :25.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.map.B1 | Empty range: both ends `mapPos(from, assoc)` (S20, S21, S33) | `CodeMirror6/state/src/selection.ts:75-76` | `ferrum/crates/ferrum-state/src/selection.rs:71-75` | none | none | implemented, untested |
| state.SelectionRange.map.B2 | Non-empty: `from` mapped with +1, `to` with −1 — insertions at the edges stay outside, the `assoc` argument is ignored (S23–S25, S32, S35) | `CodeMirror6/state/src/selection.ts:78-79` | `ferrum/crates/ferrum-state/src/selection.rs:77-82` | none | none | implemented, untested |
| state.SelectionRange.map.B3 | Unchanged → returns `this` (S31) | `CodeMirror6/state/src/selection.ts:81` | always a new value | none | none | not observable in Rust |
| state.SelectionRange.map.B4 | Keeps flags (inverted, assoc, bidi, undirectional) and goalColumn | `CodeMirror6/state/src/selection.ts:81` | keeps direction (:83-87), `assoc`, `goal_column` (:89-94) | none | none | implemented for existing fields, untested |
| state.SelectionRange.map.B5 | Non-empty range collapsing to a point keeps its flags (S27/S28: cm6 assoc −1/1, ferrum 0/0 because built by `new`) | `CodeMirror6/state/src/selection.ts:81` | `ferrum/crates/ferrum-state/src/selection.rs:83-94` | none | none | implemented; assoc values differ only through construction (state.cm6.084) |
| state.SelectionRange.map.B6 | `assoc` default −1; out-of-range position throws (S34) | `CodeMirror6/state/src/selection.ts:73` | mandatory `assoc`; panic `ferrum/crates/ferrum-state/src/change.rs:192` | none | none | implemented (no default; panic), untested |

Proposed status: PORTED, DEVIATION because every behavior is untested (positions match in all probes and in the oracle).

### state.cm6.088 `SelectionRange.extend` — cm6 `CodeMirror6/state/src/selection.ts:85`
Helpers read: `EditorSelection.range` :208.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.extend.B1 | `from <= anchor <= to` → new range `from`→`to` (anchor replaced, S41, S43) | `CodeMirror6/state/src/selection.ts:86` | `ferrum/crates/ferrum-state/src/selection.rs:99-100` | none | none | implemented, untested |
| state.SelectionRange.extend.B2 | Otherwise keep anchor, head = whichever of from/to is farther from anchor, ties → `to` (S40, S42, S44, S47, S48) | `CodeMirror6/state/src/selection.ts:87-88` | `ferrum/crates/ferrum-state/src/selection.rs:102-109` | none | none | implemented, untested |
| state.SelectionRange.extend.B3 | `to` defaults to `from` | `CodeMirror6/state/src/selection.ts:85` | required | none | none | implemented (no default) |
| state.SelectionRange.extend.B4 | Result via `range(…, assoc)`: automatic ±1 assoc or explicit `assoc` argument (S45, S49) | `CodeMirror6/state/src/selection.ts:86`, `:88` | `SelectionRange::new` → assoc 0, no argument | none | none | differs |
| state.SelectionRange.extend.B5 | goalColumn/bidi dropped (S46) | `CodeMirror6/state/src/selection.ts:86-88` | `ferrum/crates/ferrum-state/src/selection.rs:100`, `:109` | none | none | implemented, untested |

Proposed status: PORTED, DEVIATION because B4 differs and nothing is tested. cm6 view uses the assoc argument (`CodeMirror6/view/src/input.ts:626`).

### state.cm6.089 `SelectionRange.eq` — cm6 `CodeMirror6/state/src/selection.ts:92`
Helpers read: `assoc` :53, `empty` :47.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.eq.B1 | Compares anchor (S54) | `CodeMirror6/state/src/selection.ts:93` | derived `PartialEq` `ferrum/crates/ferrum-state/src/selection.rs:6` | none | none | implemented, untested |
| state.SelectionRange.eq.B2 | Compares head | `CodeMirror6/state/src/selection.ts:93` | derived | none | none | implemented, untested |
| state.SelectionRange.eq.B3 | Compares goalColumn (S53) | `CodeMirror6/state/src/selection.ts:93` | derived | none | none | implemented, untested |
| state.SelectionRange.eq.B4 | Assoc compared only with `includeAssoc` and only for empty ranges; bidi/undirectional never compared (S50, S51, S52, S55, S56) | `CodeMirror6/state/src/selection.ts:94` | assoc always compared | none | none | differs |

Proposed status: PORTED, DEVIATION because B4 differs and nothing is tested.

### state.cm6.090 `SelectionRange.toJSON` — cm6 `CodeMirror6/state/src/selection.ts:98`

Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.toJSON.B1 | `{anchor, head}` (S60) | `CodeMirror6/state/src/selection.ts:98` | none | none | none | missing |
| state.SelectionRange.toJSON.B2 | assoc, bidiLevel, goalColumn, undirectional not serialized (S61) | `CodeMirror6/state/src/selection.ts:98` | none | none | none | missing |

Proposed status: MISSING.

### state.cm6.091 `SelectionRange^fromJSON` — cm6 `CodeMirror6/state/src/selection.ts:102`
Helpers read: `EditorSelection.range` :208.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.fromJSON.B1 | Falsy or non-number anchor/head → `RangeError("Invalid JSON representation for SelectionRange")` (S63, S64) | `CodeMirror6/state/src/selection.ts:103-104` | none | none | none | missing |
| state.SelectionRange.fromJSON.B2 | Built with `range(anchor, head)` → automatic assoc (S62 assoc 1) | `CodeMirror6/state/src/selection.ts:105` | none | none | none | missing |
| state.SelectionRange.fromJSON.B3 | No integer, sign or document-bounds check (S65) | `CodeMirror6/state/src/selection.ts:103` | none | none | none | missing |

Proposed status: MISSING.

### state.cm6.166 `CharCategory` — cm6 `CodeMirror6/state/src/charcategory.ts:4`
Helpers read: `makeCategorizer` `CodeMirror6/state/src/charcategory.ts:28` (see helper section).

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.B1 | Three members Word (0), Space (1), Other (2) | `CodeMirror6/state/src/charcategory.ts:4-11` | `ferrum/crates/ferrum-state/src/char.rs:5-9` (implicit discriminants 0, 1, 2) | test-charcategory#01–#03 | test_charcategory#001–#003 | implemented+tested |

Proposed status: PORTED.

### state.cm6.167 `CharCategory.Word` — cm6 `CodeMirror6/state/src/charcategory.ts:6`
Helpers read: as 166.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.Word.B1 | Member value 0, produced for word characters | `CodeMirror6/state/src/charcategory.ts:6` | `ferrum/crates/ferrum-state/src/char.rs:6` | test-charcategory#01 | test_charcategory#001 | implemented+tested |

Proposed status: PORTED.

### state.cm6.168 `CharCategory.Space` — cm6 `CodeMirror6/state/src/charcategory.ts:8`
Helpers read: as 166.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.Space.B1 | Member value 1, produced for whitespace | `CodeMirror6/state/src/charcategory.ts:8` | `ferrum/crates/ferrum-state/src/char.rs:7` | test-charcategory#02 | test_charcategory#002 | implemented+tested |

Proposed status: PORTED.

### state.cm6.169 `CharCategory.Other` — cm6 `CodeMirror6/state/src/charcategory.ts:10`
Helpers read: as 166.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.Other.B1 | Member value 2, produced for everything else | `CodeMirror6/state/src/charcategory.ts:10` | `ferrum/crates/ferrum-state/src/char.rs:8` | test-charcategory#03 | test_charcategory#003 | implemented+tested |

Proposed status: PORTED.

### Helper (no export): `makeCategorizer` — cm6 `CodeMirror6/state/src/charcategory.ts:28`
Read for state.fe.053 `categorize_char` and `EditorState::char_categorizer` (`ferrum/crates/ferrum-state/src/state.rs:190-198`); the exported `EditorState.charCategorizer` (`CodeMirror6/state/src/state.ts:373`) is chunk C5.
Helpers read: `wordChar` `CodeMirror6/state/src/charcategory.ts:15-16`, `hasWordChar` :18-26, `nonASCIISingleCaseWordChar` :13, `EditorState.languageDataAt` `CodeMirror6/state/src/state.ts:353`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.categorizer.B1 | `!/\S/.test(str)` → Space: the whole string (a cluster) must be JS whitespace; empty string → Space. JS `\s` includes U+FEFF and excludes U+0085 | `CodeMirror6/state/src/charcategory.ts:30` | first char only, `char::is_whitespace` `ferrum/crates/ferrum-state/src/char.rs:13`, `state.rs:192-196` (empty → Other) | test-charcategory#02 | test_charcategory#002 | differs: `""` Space vs Other, `" ́"` Other vs Space, U+0085, U+FEFF (R7) |
| state.categorizer.B2 | Word if **any** code point of the string matches `[\p{Alphabetic}\p{Number}_]` | `CodeMirror6/state/src/charcategory.ts:16`, `:19`, `:31` | first char only, `is_alphanumeric() \|\| '_'` `ferrum/crates/ferrum-state/src/char.rs:15` | test-charcategory#01 | test_charcategory#001 | differs for multi-code-point strings (`" a"`, `"/a"`); identical for all single code points except U+0085/U+FEFF (exhaustive, R7) |
| state.categorizer.B3 | Fallback when the `u`-flag property regex is unsupported: `/\w/`, or non-ASCII with case distinction, or `nonASCIISingleCaseWordChar` | `CodeMirror6/state/src/charcategory.ts:13`, `:20-25` | none | none | none | not reached on modern engines (Node 22 uses B2); no Ferrum equivalent needed unless parity with old engines is wanted |
| state.categorizer.B4 | Extra `wordChars` from language data at `at`: Word if the string contains any of them | `CodeMirror6/state/src/charcategory.ts:32`; `CodeMirror6/state/src/state.ts:374-375` | none (`_at` ignored, `ferrum/crates/ferrum-state/src/state.rs:190`) | none | none | missing (G `languageData wordChars "$"` → cm6 Word, ferrum Other) |
| state.categorizer.B5 | Otherwise Other | `CodeMirror6/state/src/charcategory.ts:33` | `ferrum/crates/ferrum-state/src/char.rs:17` | test-charcategory#03 | test_charcategory#003 | implemented+tested |
| state.categorizer.B6 | No CJK special case in the categorizer (CJK letters are `\p{Alphabetic}` → Word); CJK word segmentation exists only in `moveBySubword` (`CodeMirror6/commands/src/commands.ts:112`, `:146-156`) | `CodeMirror6/state/src/charcategory.ts:16` | `is_alphanumeric` → Word | none | none | implemented, untested (CJK, Hiragana, Hangul → Word on both sides, R7) |

Proposed status (for the helper, informational): partially equivalent; differences B1, B2 (multi-char), B4.

### state.cm6.212 `findClusterBreak` — cm6 `CodeMirror6/state/src/char.ts:9`
Helpers read: `@marijn/find-cluster-break/src/index.js` `findClusterBreak` :41, `nextClusterBreak` :45-66, `prevClusterBreak` :68-75, `isExtendingChar` :17-26, `isRegionalIndicator` :28, `codePointAt` :77, `codePointSize` :87, extender table :5-15 and its generator `bin/build-extenders.js` (Extend + SpacingMark minus 31 exceptions, code points < U+30000). Ferrum: `unicode-segmentation` 1.13.3 `grapheme_indices(true)`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.findClusterBreak.B1 | Forward at `pos == length` → `pos`; backward at `pos <= 1` → 0 (C14–C17) | `index.js:46`, `:69`, `:74` | `ferrum/crates/ferrum-state/src/char.rs:32-35` | none | none | implemented, untested |
| state.findClusterBreak.B2 | Forward from inside a surrogate pair moves to the pair start first (C06, C07) | `index.js:48` | not representable in char units | none | none | not applicable under char positions (D1) |
| state.findClusterBreak.B3 | Steps by code points (surrogate pair = 2 units); lone surrogates count as one unit (C08, C09–C11) | `index.js:49-50`, `:77-83` | char units; `&str` cannot contain lone surrogates | none | none | differs by unit (D1): `"😀"` → 2 vs 1 |
| state.findClusterBreak.B4 | Joins following extending characters (GCB Extend and SpacingMark, only < U+30000) when `includeExtending` | `index.js:53`, `:17-26` | UAX #29 GB9/GB9a on all code points `char.rs:44` | none | none | differs for tag characters / VS supplement ≥ U+E0000 (T25); equal for marks, VS16, skin tones, SpacingMarks (T05, T06, T14, T20, T21, T24) |
| state.findClusterBreak.B5 | `includeExtending = false`: extending characters not joined, ZWJ and RI rules still apply (C01, C03, C05, C19, C20) | `index.js:41`, `:53` | no parameter `char.rs:31` | none | none | missing (cm6 commands use it: `CodeMirror6/commands/src/commands.ts:581`, `:585`) |
| state.findClusterBreak.B6 | ZWJ joins the next code point unconditionally, and ZWJ itself joins the previous one | `index.js:53` | GB9 (ZWJ is Extend) + GB11 (only ExtPict ZWJ × ExtPict) | none | none | differs: `"a‍b"` one cluster vs two (T09); `"\n‍😀"` (T34); equal for emoji ZWJ sequences (T08, T15) |
| state.findClusterBreak.B7 | Regional indicators paired by counting preceding RIs (odd count joins) | `index.js:56-60` | GB12/GB13 | none | none | implemented (T12, T13, C21–C24 equal after unit conversion), untested |
| state.findClusterBreak.B8 | No CR×LF rule: `"\r\n"` is two clusters | `index.js:45-66` (no rule) | GB3: one cluster | none | none | differs (T02, T03; 105/2000 ASCII oracle strings, all containing CRLF) |
| state.findClusterBreak.B9 | No control rules: CR/LF/control followed by an extending char or ZWJ joins (`"\ŕ"`, `"\t́"`) | `index.js:53` | GB4/GB5: break | none | none | differs (T33, T34) |
| state.findClusterBreak.B10 | No Hangul syllable rules: L, V, T jamo and LV+T are separate clusters | (no rule) | GB6–GB8 join | none | none | differs (T17, T18) |
| state.findClusterBreak.B11 | No Prepend (GB9b) and no Indic conjunct rule (GB9c) | (no rule) | GB9b, GB9c implemented | none | none | differs (T19, T23, T31) |
| state.findClusterBreak.B12 | Backward: `prevClusterBreak` probes `nextClusterBreak(pos-2)` while `pos > 1`, returns the first result `< pos`; from inside a cluster returns that cluster's start | `index.js:68-75` | start of grapheme before `pos` `char.rs:57-66` | none | none | implemented (equal on all non-B8–B11 table cases), untested |
| state.findClusterBreak.B13 | `pos` beyond length: forward returns garbage (`"ab"`,5 → 7), backward 2; negative pos accepted (C12, C13, C18) | `index.js:46-50` | forward returns `pos` (5), backward 1 | none | none | differs (out-of-contract input) |
| state.findClusterBreak.B14 | Cost: inspects only code units near `pos` | `index.js:45-75` | whole-string scan + `Vec` of all graphemes per call; backward re-counts prefixes: O(n) forward, O(n²) backward (R5) | none | none | differs (complexity) |

Proposed status: PORTED, DEVIATION because B3–B6, B8–B11, B13, B14 differ, B5 is missing and nothing is tested.

### state.cm6.213 `codePointAt` — cm6 `CodeMirror6/state/src/char.ts:19`
Helpers read: `surrogateLow`/`surrogateHigh` `CodeMirror6/state/src/char.ts:13-14`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.codePointAt.B1 | Non-high-surrogate unit → that unit (P06, P02 low half) | `CodeMirror6/state/src/char.ts:20-21` | none | none | none | missing |
| state.codePointAt.B2 | High surrogate at the last position → the unit (P03) | `CodeMirror6/state/src/char.ts:21` | none | none | none | missing |
| state.codePointAt.B3 | High surrogate not followed by low → the unit (P04) | `CodeMirror6/state/src/char.ts:22-23` | none | none | none | missing |
| state.codePointAt.B4 | Valid pair → combined code point (P01); out of range → NaN (P05) | `CodeMirror6/state/src/char.ts:24`, `:20` | none | none | none | missing |

Proposed status: MISSING (searched `code_point_at`, `codepoint`, `char_code_at`; meaning depends on D1).

### state.cm6.214 `fromCodePoint` — cm6 `CodeMirror6/state/src/char.ts:30`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.fromCodePoint.B1 | `code <= 0xFFFF` → one unit, incl. lone surrogates and negative values wrapping (`-1` → U+FFFF) (F01, F03, F05, F06) | `CodeMirror6/state/src/char.ts:31` | none | none | none | missing |
| state.fromCodePoint.B2 | Otherwise a surrogate pair (F02, F07) | `CodeMirror6/state/src/char.ts:32-33` | none | none | none | missing |
| state.fromCodePoint.B3 | No range validation (0x110000 → `\udc00\udc00`, F04) | `CodeMirror6/state/src/char.ts:30-33` | none | none | none | missing |

Proposed status: MISSING (searched `from_code_point`; Rust `char::from_u32` returns `Option` and cannot produce lone surrogates).

### state.cm6.215 `codePointSize` — cm6 `CodeMirror6/state/src/char.ts:37`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.codePointSize.B1 | `< 0x10000` → 1, else 2 (Z01–Z03) | `CodeMirror6/state/src/char.ts:37` | none (`char::len_utf16` exists in std, unused) | none | none | missing |

Proposed status: MISSING (searched `code_point_size`, `len_utf16`).

### state.cm6.216 `countColumn` — cm6 `CodeMirror6/state/src/column.ts:5`
Helpers read: `findClusterBreak` `CodeMirror6/state/src/char.ts:9` (and its dependency).

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.countColumn.B1 | Loop while `i < to && i < length`; `to` defaults to length; `to > length` clamps (K04, K05); negative `to` → 0 (K09) | `CodeMirror6/state/src/column.ts:5`, `:7` | `ferrum/crates/ferrum-state/src/column.rs:9`, `:14` | none | none | implemented, untested |
| state.countColumn.B2 | Tab: `n += tabSize - n % tabSize`, `i++` (K01–K03) | `CodeMirror6/state/src/column.ts:8-10` | `ferrum/crates/ferrum-state/src/column.rs:15-17` | none | none | implemented, untested |
| state.countColumn.B3 | Other: `n++`, advance to next cluster break (one column per cluster; no East Asian width, K10) | `CodeMirror6/state/src/column.ts:12-13` | `ferrum/crates/ferrum-state/src/column.rs:19-21` (with an `i+1` fallback cm6 does not need) | none | none | implemented; results differ wherever findClusterBreak differs (R2: CRLF 4 vs 3, Hangul jamo 3 vs 1, `"a‍b"` 1 vs 2, …) |
| state.countColumn.B4 | `to` inside a cluster counts the whole cluster (K06) | `CodeMirror6/state/src/column.ts:7`, `:13` | same structure | none | none | implemented, untested (UTF-16 case not representable) |
| state.countColumn.B5 | `tabSize` 0 → `NaN` (K08) | `CodeMirror6/state/src/column.ts:9` | panic "attempt to calculate the remainder with a divisor of zero" | none | none | differs |
| state.countColumn.B6 | `to` in UTF-16 units | `CodeMirror6/state/src/column.ts:5` | `to: Option<usize>` in chars | none | none | differs by unit (D1) |
| state.countColumn.B7 | O(n) | `CodeMirror6/state/src/column.ts:7-15` | O(n²) via `find_cluster_break` + an extra `Vec<char>` (R5: 40 000 chars → 63 s) | none | none | differs (complexity) |

Proposed status: PORTED, DEVIATION because B3, B5–B7 differ and nothing is tested.

### state.cm6.217 `findColumn` — cm6 `CodeMirror6/state/src/column.ts:24`
Helpers read: `findClusterBreak` `CodeMirror6/state/src/char.ts:9`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.findColumn.B1 | Returns `i` as soon as `n >= col` (col ≤ 0 → 0, J08, J09) | `CodeMirror6/state/src/column.ts:26` | `ferrum/crates/ferrum-state/src/column.rs:34-36` | none | none | implemented, untested |
| state.findColumn.B2 | Stops at end of string | `CodeMirror6/state/src/column.ts:27` | `ferrum/crates/ferrum-state/src/column.rs:37-39` | none | none | implemented, untested |
| state.findColumn.B3 | Tab adds `tabSize - n % tabSize`, others 1; **every** character, tabs included, is advanced with `findClusterBreak` (so `"\t́"` is one step in cm6) | `CodeMirror6/state/src/column.ts:28-29` | tab advances `i += 1` only `ferrum/crates/ferrum-state/src/column.rs:40-42`, others by cluster `:43-46` | none | none | differs where clusters differ (R2) |
| state.findColumn.B4 | Column inside a tab → offset before the tab (J02) | `CodeMirror6/state/src/column.ts:26-28` | same | none | none | implemented, untested |
| state.findColumn.B5 | Too short: `strict === true` → −1, else `string.length` (J03, J04, J12) | `CodeMirror6/state/src/column.ts:31` | `None` / `Some(chars.len())` `ferrum/crates/ferrum-state/src/column.rs:50-54` | none | none | implemented (Option instead of −1), untested |
| state.findColumn.B6 | `tabSize` 0 → returns a value (J10: 1) | `CodeMirror6/state/src/column.ts:28` | panic (divisor of zero) | none | none | differs |
| state.findColumn.B7 | Result in UTF-16 units (J06: 2 vs 1); O(n) | `CodeMirror6/state/src/column.ts:24-31` | chars; O(n²) | none | none | differs (unit D1, complexity) |

Proposed status: PORTED, DEVIATION because B3, B6, B7 differ and nothing is tested.

## Unfinished

none
