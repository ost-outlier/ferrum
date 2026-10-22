# C3 API rows

> What: chunk C3 API rows and reproductions (the reproduction ids R…/P…/S… used in all chunk files are defined here). Versions: cm6 `@codemirror/state` 6.7.6 (`CodeMirror6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`ferrum/crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `ferrum/docs/parity/README.md`.

Totals: cm6 items owned 41 (DROP 0), Ferrum items owned 34. Rows written: 41 + 34 = 75.

Scope: cm6 `CodeMirror6/state/src/selection.ts` (incl. internal `RangeFlag` :12, `SelectionRange.create` :109, `EditorSelection.normalized` :223, `checkSelection` :239), `CodeMirror6/state/src/char.ts` (+ `@marijn/find-cluster-break` 1.0.4 `src/index.js`), `CodeMirror6/state/src/charcategory.ts` (incl. internal `makeCategorizer` :28, `wordChar` :16, `hasWordChar` :18, `nonASCIISingleCaseWordChar` :13), `CodeMirror6/state/src/column.ts`. Ferrum `ferrum/crates/ferrum-state/src/selection.rs`, `char.rs`, `column.rs`, and `EditorState::char_categorizer` (`ferrum/crates/ferrum-state/src/state.rs:190`) as far as it classifies characters.

All reproductions were run (cm6: Node 22.23.3, Unicode 17.0, tsx on the harness copy of `state/src`; Ferrum: rustc 1.99.1, `unicode-segmentation` 1.13.3, `char::UNICODE_VERSION` = 17.0.0). Short repros are inline; long ones are in `## Reproductions`.Scratch paths of the producing session were replaced by archived copies under `ferrum/docs/parity/state/oracle/` (resolution rules: `ferrum/docs/parity/README.md`).

Unit notation: "UTF-16" = cm6 position unit; "char" = Ferrum position unit (Unicode scalar value / code point). Every position-taking row depends on D1.

## cm6 items

| id | cm6 item | cm6 file:line | Ferrum counterpart (file:line) or "none (searched: …)" | proposed status | behaviors (ids) | tests cm6 / Ferrum | position unit / D-deps | notes + repro |
|---|---|---|---|---|---|---|---|---|
| state.cm6.061 | `EditorSelection` | `CodeMirror6/state/src/selection.ts:115` | `EditorSelection` struct `ferrum/crates/ferrum-state/src/selection.rs:117` (derives `Debug, Clone, PartialEq, Eq`; `Default` impl :122) | PORTED, DEVIATION | state.EditorSelection.B1–B3 | test-selection#01–#04 / test_selection#001–#004 | positions inside ranges: UTF-16 vs char (D1) | Private fields + factory functions match cm6's private constructor. Deviations: zero-range state reachable in cm6 only via `fromJSON` (R1 S6B), Ferrum `create(vec![])` silently returns `cursor(0)` instead of throwing (R1 S73); value semantics (`Clone`/`PartialEq`) replace identity. Most of the cm6 API surface (`asSingle`, `addRange`, `replaceRange`, `toJSON`, `fromJSON`, `undirectionalRange`) is missing (separate rows). |
| state.cm6.062 | `EditorSelection.ranges` | `CodeMirror6/state/src/selection.ts:119` | `EditorSelection::ranges()` `ferrum/crates/ferrum-state/src/selection.rs:219` (returns `&[SelectionRange]`) | PORTED, RUST-IDIOMATIC | state.EditorSelection.ranges.B1–B2 | test-selection#01–#04 / test_selection#001–#004 | UTF-16 vs char (D1) | Readonly field → borrowing getter; same contents for every case in R1 and in the oracle (positions identical in 4999/5000 random cases, the one mismatch is mainIndex, see state.cm6.073). Range elements differ in `assoc` (state.cm6.084). repro: `create([range(0,1),range(3,2),range(4,5)],1).ranges` → cm6: `0/1,3/2,4/5` \| ferrum: `0/1,3/2,4/5`. |
| state.cm6.063 | `EditorSelection.mainIndex` | `CodeMirror6/state/src/selection.ts:122` | `EditorSelection::main_index()` `ferrum/crates/ferrum-state/src/selection.rs:213` | PORTED, DEVIATION | state.EditorSelection.mainIndex.B1–B2 | test-selection#01 / test_selection#001 (indirectly via `main()`) | — | cm6 keeps out-of-range indices; Ferrum clamps. repro: `create([cursor(1),cursor(3)],5)` → cm6: `mainIndex=5, main=undefined` \| ferrum: `main_index=1, main=3/3` (R1 S74); `create([cursor(3),cursor(1)],5)` → cm6: `mainIndex=-1` \| ferrum: `0` (R1 S75). No Ferrum test of the clamp. |
| state.cm6.064 | `EditorSelection.map` | `CodeMirror6/state/src/selection.ts:127` | `EditorSelection::map(&self, &ChangeDesc, assoc: i8)` `ferrum/crates/ferrum-state/src/selection.rs:230` | PORTED, DEVIATION | state.EditorSelection.map.B1–B5 | none / none | positions UTF-16 vs char (D1) | Untested in Ferrum (B1–B5). `assoc` has no default (cm6 `-1`). Inherits create's main-index divergence (R4) and SelectionRange.map flag loss. repro: `[c1, 3/5, c8] main 1` mapped through insert "xx" at 1, assoc 1 → cm6: `[3/3,5/7@-1,10/10] main=1` \| ferrum: `[3/3,5/7,10/10] main=1` (R1 S92; `@-1` = cm6 assoc, Ferrum assoc 0). Random oracle: 4999/5000 equal positions+main (R4). |
| state.cm6.065 | `EditorSelection.eq` | `CodeMirror6/state/src/selection.ts:136` | derived `PartialEq for EditorSelection` `ferrum/crates/ferrum-state/src/selection.rs:116` | PORTED, DEVIATION | state.EditorSelection.eq.B1–B5 | none / none | — | No `includeAssoc` parameter; derived `==` always compares `assoc` (and `goal_column`) of every range. repro: `single(1).eq(create([cursor(1,1)]))` → cm6: `true` \| ferrum: `single(1,1) == create([cursor 1 assoc 1])` → `false` (R1 S58). Untested. |
| state.cm6.066 | `EditorSelection.main` | `CodeMirror6/state/src/selection.ts:147` | `EditorSelection::main()` `ferrum/crates/ferrum-state/src/selection.rs:207` | PORTED | state.EditorSelection.main.B1 | test-selection#01 / test_selection#001 | — | `&self.ranges[self.main_index]`; Ferrum can never index out of range because `create` clamps (difference recorded on state.cm6.063/073). repro: `create([range(0,1),range(3,2),range(4,5)],1).main` → cm6: `from 2,to 3,anchor 3,head 2` \| ferrum: same (test_selection#001). |
| state.cm6.067 | `EditorSelection.asSingle` | `CodeMirror6/state/src/selection.ts:151` | none (searched all crates for `as_single`, `asSingle`, `single(`, `collapse`, `allow_multiple` uses: `EditorState::allow_multiple_selections` facet exists `ferrum/crates/ferrum-state/src/state.rs:165` but nothing reads it) | MISSING | state.EditorSelection.asSingle.B1–B2 | none / none | — | repro: `[c1, 3/5, c8] main 1 .asSingle()` → cm6: `[3/5] main=0` (R1 S81) \| ferrum: no counterpart; consequence: `EditorState::create` with two cursors and no `allowMultipleSelections` keeps 2 ranges (R6), cm6 keeps 1 (`CodeMirror6/state/src/state.ts:251`). |
| state.cm6.068 | `EditorSelection.addRange` | `CodeMirror6/state/src/selection.ts:156` | none (searched `add_range` in all crates: only `RangeSetBuilder::add_range` `ferrum/crates/ferrum-state/src/rangeset.rs:362`, unrelated) | MISSING | state.EditorSelection.addRange.B1–B3 | none / none | — | repro: `[c1,3/5,c8] main 1 .addRange(cursor(6))` → cm6: `[1/1,3/5,6/6,8/8] main=2`; `.addRange(cursor(6), false)` → `main=1` (R1 S83/S84) \| ferrum: no counterpart. |
| state.cm6.069 | `EditorSelection.replaceRange` | `CodeMirror6/state/src/selection.ts:162` | none (searched `replace_range`, `replace_main`, `with_range`: only `String::replace_range` in `ferrum/crates/ferrum-state/src/tests/test_text.rs:140`) | MISSING | state.EditorSelection.replaceRange.B1–B3 | none / none | — | repro: `.replaceRange(cursor(9))` → cm6: `[1/1,8/8,9/9] main=2` (R1 S87); out-of-range `which` → cm6 TypeError (R1 S8A) \| ferrum: no counterpart. |
| state.cm6.070 | `EditorSelection.toJSON` | `CodeMirror6/state/src/selection.ts:170` | none (searched `to_json`, `toJSON`, `Serialize`, `serde` in all crates: serde only in `ferrum/crates/ferrum-language/src/fold.rs:7`) | MISSING | state.EditorSelection.toJSON.B1 | none / none | JSON offsets are UTF-16 in cm6 (D1) | repro: `create([range(0,2),cursor(5)],1).toJSON()` → cm6: `{"ranges":[{"anchor":0,"head":2},{"anchor":5,"head":5}],"main":1}` (R1 S66) \| ferrum: no counterpart. |
| state.cm6.071 | `EditorSelection^fromJSON` | `CodeMirror6/state/src/selection.ts:175` | none (searched `from_json`, `fromJSON`, `Deserialize`) | MISSING | state.EditorSelection.fromJSON.B1–B5 | none / none | JSON offsets UTF-16 (D1) | repro: R1 S67–S6D (valid input; `main >= length` → RangeError; no `ranges` → RangeError; unsorted input NOT normalized → `[5/5,0/6] main=0`; `{ranges:[],main:-1}` accepted) \| ferrum: no counterpart. |
| state.cm6.072 | `EditorSelection^single` | `CodeMirror6/state/src/selection.ts:182` | `EditorSelection::single(anchor, head)` `ferrum/crates/ferrum-state/src/selection.rs:138` | PORTED, DEVIATION | state.EditorSelection.single.B1–B3 | none / none | UTF-16 vs char (D1) | `head` not optional (cm6 defaults to `anchor`); resulting non-empty range has `assoc 0` instead of cm6's auto assoc. repro: `single(3,1).main` → cm6: `anchor 3, head 1, assoc 1` \| ferrum: `anchor 3, head 1, assoc 0` (R1 S71). No ferrum-state test calls `single`. |
| state.cm6.073 | `EditorSelection^create` | `CodeMirror6/state/src/selection.ts:188` | `EditorSelection::create(Vec<SelectionRange>, usize)` `ferrum/crates/ferrum-state/src/selection.rs:147` | PORTED, DEVIATION | state.EditorSelection.create.B1–B10 | test-selection#01–#04 / test_selection#001–#004 | UTF-16 vs char (D1) | Deviations: (a) empty input: cm6 throws `RangeError: A selection needs at least one range` \| ferrum returns `cursor(0)` (R1 S73); (b) out-of-range main index kept (cm6) vs clamped (ferrum) (R1 S74/S75); (c) main range located by value equality instead of identity → wrong main range when duplicates exist: `create([cursor(12), range(12,18), cursor(12)], 2)` → cm6: `12/12,12/18 main=1` \| ferrum: `12/12,12/18 main=0` (R4); (d) merged ranges get `assoc 0` (cm6 `range()` gives ±1) (R1 S7A/S7B). Sorting/merging positions identical in 5000 random cases (R4). |
| state.cm6.074 | `EditorSelection^cursor` | `CodeMirror6/state/src/selection.ts:200` | `SelectionRange::cursor(pos)` `ferrum/crates/ferrum-state/src/selection.rs:20` (same return type, a range). Note: Ferrum `EditorSelection::cursor(pos)` `selection.rs:130` returns an `EditorSelection` (cm6 `EditorSelection.single(pos)`), not a range | PORTED, DEVIATION | state.EditorSelection.cursor.B1–B4 | none / none | UTF-16 vs char (D1) | No `assoc`, `bidiLevel`, `goalColumn` parameters (assoc/goal can be set afterwards through `pub` fields; bidi level does not exist). repro: `cursor(5,1,9,12)` → cm6: `assoc 1, bidiLevel 6 (clamped), goalColumn 12` (R1 S09) \| ferrum: `SelectionRange::cursor(5)` → `assoc 0, goal None`, no bidi. Untested. |
| state.cm6.075 | `EditorSelection^range` | `CodeMirror6/state/src/selection.ts:208` | `SelectionRange::new(anchor, head)` `ferrum/crates/ferrum-state/src/selection.rs:30` (+ `with_goal_column` :64) | PORTED, DEVIATION | state.EditorSelection.range.B1–B5 | test-selection#01–#04 / test_selection#001–#004 (positions only) | UTF-16 vs char (D1) | No automatic assoc, no bidi level, no assoc argument. repro: `range(3,2)` → cm6: `assoc 1`; `range(2,3)` → `assoc -1` (R1 S01/S02) \| ferrum: `new(3,2)`/`new(2,3)` → `assoc 0`. |
| state.cm6.076 | `EditorSelection^undirectionalRange` | `CodeMirror6/state/src/selection.ts:218` | none (searched `undirectional`, `undecided`, `directionless` in all crates) | MISSING | state.EditorSelection.undirectionalRange.B1–B3 | none / none | — | repro: `undirectionalRange(5,2)` → cm6: `from 5, to 2, anchor 5, head 2, undirectional true, bidiLevel 0` (no sorting of from/to; bidi flag bits 0) (R1 S11) \| ferrum: no counterpart. Used by cm6 view (`CodeMirror6/view/src/cursor.ts:35`, `CodeMirror6/view/src/input.ts:591`). |
| state.cm6.077 | `SelectionRange` | `CodeMirror6/state/src/selection.ts:24` | `SelectionRange` struct `ferrum/crates/ferrum-state/src/selection.rs:7` (derives `Debug, Clone, Copy, PartialEq, Eq`) | PORTED, DEVIATION | state.SelectionRange.B1–B4 | test-selection#01–#04 / test_selection#001–#004 | UTF-16 vs char (D1) | cm6 stores `from`, `to`, `flags` (bidi 3 bits, assoc 2 bits, inverted, undirectional), `goalColumn`, private constructor; Ferrum stores `pub anchor, head, goal_column, assoc: i8`, freely constructible and mutable, no bidi level, no undirectional flag, `assoc` can hold any `i8`. |
| state.cm6.078 | `SelectionRange.from` | `CodeMirror6/state/src/selection.ts:27` | `SelectionRange::from()` `ferrum/crates/ferrum-state/src/selection.rs:41` (`min(anchor, head)`) | PORTED, RUST-IDIOMATIC | state.SelectionRange.from.B1–B2 | test-selection#01 / test_selection#001 | UTF-16 vs char (D1) | Field → method computing `min`. Only cm6 `undirectionalRange` can produce `from > to` (B2; R1 S11), which has no Ferrum counterpart. repro: `range(3,2).from` → cm6 `2` \| ferrum `new(3,2).from()` `2`. |
| state.cm6.079 | `SelectionRange.to` | `CodeMirror6/state/src/selection.ts:29` | `SelectionRange::to()` `ferrum/crates/ferrum-state/src/selection.rs:47` (`max(anchor, head)`) | PORTED, RUST-IDIOMATIC | state.SelectionRange.to.B1–B2 | test-selection#01 / test_selection#001 | UTF-16 vs char (D1) | As `from`. repro: `range(3,2).to` → cm6 `3` \| ferrum `3`. |
| state.cm6.080 | `SelectionRange.goalColumn` | `CodeMirror6/state/src/selection.ts:35` | `pub goal_column: Option<usize>` `ferrum/crates/ferrum-state/src/selection.rs:13` | PORTED, DEVIATION | state.SelectionRange.goalColumn.B1–B4 | none / none | goal column is a pixel/column value chosen by the view, unit-independent | Mutable public field vs readonly; preservation rules match (kept by `map`, dropped by `extend` and by merge) but untested. repro: `cursor(5,0,2,7).extend(9).goalColumn` → cm6 `undefined` \| ferrum `cursor goal 7 .extend(9,9).goal_column` → `None` (R1 S46). Type: cm6 `number` (may be fractional px), Ferrum `usize`. |
| state.cm6.081 | `SelectionRange.anchor` | `CodeMirror6/state/src/selection.ts:40` | `pub anchor: usize` `ferrum/crates/ferrum-state/src/selection.rs:9` | PORTED, RUST-IDIOMATIC | state.SelectionRange.anchor.B1 | test-selection#01 / test_selection#001 | UTF-16 vs char (D1) | Derived getter (`flags & Inverted ? to : from`) → stored field; same values in every probe. Field is writable; `EditorSelection::ranges()` returns a shared slice, so ranges inside a selection cannot be mutated. |
| state.cm6.082 | `SelectionRange.head` | `CodeMirror6/state/src/selection.ts:44` | `pub head: usize` `ferrum/crates/ferrum-state/src/selection.rs:11` | PORTED, RUST-IDIOMATIC | state.SelectionRange.head.B1 | test-selection#01 / test_selection#001 | UTF-16 vs char (D1) | As `anchor`. |
| state.cm6.083 | `SelectionRange.empty` | `CodeMirror6/state/src/selection.ts:47` | `SelectionRange::is_empty()` `ferrum/crates/ferrum-state/src/selection.rs:53` | PORTED, RUST-IDIOMATIC | state.SelectionRange.empty.B1 | test-selection#03 / test_selection#003 (merge result depends on the empty branch) | — | `from == to` vs `anchor == head`: equivalent. |
| state.cm6.084 | `SelectionRange.assoc` | `CodeMirror6/state/src/selection.ts:53` | `pub assoc: i8` `ferrum/crates/ferrum-state/src/selection.rs:15` | PORTED, DEVIATION | state.SelectionRange.assoc.B1–B4 | none / none | — | cm6 assoc is always -1/0/1, set automatically for non-empty ranges by `range()` (forward → -1, inverted → 1) and by `cursor(pos, assoc)`; Ferrum constructors always give 0 and the field accepts any `i8`. repro: `range(2,3).assoc` → cm6 `-1` \| ferrum `new(2,3).assoc` → `0` (R1 S02). Doc comment calls it "Bidi association bias", cm6 calls it the side the cursor is associated with. Untested. |
| state.cm6.085 | `SelectionRange.undirectional` | `CodeMirror6/state/src/selection.ts:60` | none (searched `undirectional`, `undecided`) | MISSING | state.SelectionRange.undirectional.B1–B2 | none / none | — | repro: `undirectionalRange(2,5).undirectional` → cm6 `true`; flag survives `map` (R1 S30) \| ferrum: no counterpart. |
| state.cm6.086 | `SelectionRange.bidiLevel` | `CodeMirror6/state/src/selection.ts:66` | none (searched `bidi`, `bidi_level`, `level` in selection code; only the doc comment `selection.rs:14` mentions "Bidi") | MISSING | state.SelectionRange.bidiLevel.B1–B3 | none / none | — | repro: `cursor(5).bidiLevel` → cm6 `null`; `cursor(5,1,9).bidiLevel` → `6` (clamped); `undirectionalRange(2,5).bidiLevel` → `0` (R1 S07/S09/S12) \| ferrum: no counterpart. Consumed by cm6 view (`CodeMirror6/view/src/docview.ts:218`, `CodeMirror6/view/src/bidi.ts:438`). |
| state.cm6.087 | `SelectionRange.map` | `CodeMirror6/state/src/selection.ts:73` | `SelectionRange::map(&self, &ChangeDesc, assoc: i8)` `ferrum/crates/ferrum-state/src/selection.rs:70` | PORTED, DEVIATION | state.SelectionRange.map.B1–B6 | none / none | UTF-16 vs char (D1) | Positions identical in all probes (R1 S20–S35); `assoc` mandatory; untested. Both ignore the `assoc` argument for non-empty ranges (from maps with +1, to with -1) and both ignore the range's own `assoc` field for mapping. cm6 keeps all flags (bidi, undirectional) — Ferrum has none to keep. Out-of-range pos: cm6 RangeError, Ferrum panic with the same text (R1 S34). |
| state.cm6.088 | `SelectionRange.extend` | `CodeMirror6/state/src/selection.ts:85` | `SelectionRange::extend(&self, from, to)` `ferrum/crates/ferrum-state/src/selection.rs:98` | PORTED, DEVIATION | state.SelectionRange.extend.B1–B5 | none / none | UTF-16 vs char (D1) | No default `to`, no `assoc` argument; result assoc 0 instead of auto ±1. Positions identical (R1 S40–S48). repro: `cursor(5).extend(5,5,1).assoc` → cm6 `1` \| ferrum: argument does not exist; `extend(5,5)` → `assoc 0` (R1 S45/S49). Untested. |
| state.cm6.089 | `SelectionRange.eq` | `CodeMirror6/state/src/selection.ts:92` | derived `PartialEq for SelectionRange` `ferrum/crates/ferrum-state/src/selection.rs:6` | PORTED, DEVIATION | state.SelectionRange.eq.B1–B4 | none / none | — | cm6 default ignores assoc; with `includeAssoc` compares assoc only for empty ranges. Ferrum `==` always compares assoc. repro: `cursor(5,-1).eq(cursor(5,1))` → cm6 `true` \| ferrum `false` (R1 S50); `range(2,5,_,_,1).eq(range(2,5,_,_,-1), true)` → cm6 `true` \| ferrum (assoc fields 1 and -1) `false` (R1 S52). Untested. |
| state.cm6.090 | `SelectionRange.toJSON` | `CodeMirror6/state/src/selection.ts:98` | none (searched `to_json`, `Serialize`) | MISSING | state.SelectionRange.toJSON.B1–B2 | none / none | JSON offsets UTF-16 (D1) | repro: `cursor(4,1,2,9).toJSON()` → cm6 `{"anchor":4,"head":4}` (assoc/bidi/goal dropped) (R1 S61) \| ferrum: none. |
| state.cm6.091 | `SelectionRange^fromJSON` | `CodeMirror6/state/src/selection.ts:102` | none (searched `from_json`, `Deserialize`) | MISSING | state.SelectionRange.fromJSON.B1–B3 | none / none | JSON offsets UTF-16 (D1) | repro: `fromJSON({anchor:'3',head:1})` → cm6 RangeError; `fromJSON({anchor:-4,head:1.5})` → accepted (no integer/sign check) (R1 S63/S65) \| ferrum: none. |
| state.cm6.166 | `CharCategory` | `CodeMirror6/state/src/charcategory.ts:4` | `CharCategory` enum `ferrum/crates/ferrum-state/src/char.rs:5` | PORTED | state.CharCategory.B1 | test-charcategory#01–#03 / test_charcategory#001–#003 | — | Same three members, same order (cm6 numeric enum 0/1/2; Ferrum implicit discriminants 0/1/2, no `#[repr]`). The categorizer that produces values is a separate item (C5 `charCategorizer`, plus Ferrum-only `categorize_char`). |
| state.cm6.167 | `CharCategory.Word` | `CodeMirror6/state/src/charcategory.ts:6` | `CharCategory::Word` `ferrum/crates/ferrum-state/src/char.rs:6` | PORTED | state.CharCategory.B1 | test-charcategory#01 / test_charcategory#001 | — | — |
| state.cm6.168 | `CharCategory.Space` | `CodeMirror6/state/src/charcategory.ts:8` | `CharCategory::Space` `ferrum/crates/ferrum-state/src/char.rs:7` | PORTED | state.CharCategory.B1 | test-charcategory#02 / test_charcategory#002 | — | — |
| state.cm6.169 | `CharCategory.Other` | `CodeMirror6/state/src/charcategory.ts:10` | `CharCategory::Other` `ferrum/crates/ferrum-state/src/char.rs:8` | PORTED | state.CharCategory.B1 | test-charcategory#03 / test_charcategory#003 | — | — |
| state.cm6.212 | `findClusterBreak` | `CodeMirror6/state/src/char.ts:9` (+ `@marijn/find-cluster-break/src/index.js:41`) | `find_cluster_break(text, pos, forward)` `ferrum/crates/ferrum-state/src/char.rs:31` | PORTED, DEVIATION | state.findClusterBreak.B1–B14 | none / none | UTF-16 vs char (D1): `findClusterBreak("😀",0)` → cm6 `2` \| ferrum `1` | Different algorithm: cm6 is a hand-written approximation (Extend+most SpacingMark < U+30000, ZWJ joins anything, RI pairs); Ferrum uses full UAX #29 extended graphemes. Mismatching classes (R2): CRLF, ZWJ next to non-pictographs, Hangul L/V/T/LV/LVT, Indic conjuncts (GB9c), Prepend, tag characters & VS supplement (≥ U+E0000), control + extend. No `includeExtending` parameter. Ferrum cost O(n) per forward call and O(n²) per backward call over the whole string (R5). Untested in Ferrum. |
| state.cm6.213 | `codePointAt` | `CodeMirror6/state/src/char.ts:19` | none (searched `code_point_at`, `codepoint`, `char_code_at`, `len_utf16` in all crates) | MISSING | state.codePointAt.B1–B4 | none / none | exists only because positions are UTF-16 (D1) | repro: `codePointAt("a😀",1)` → cm6 `128512`; `("a😀",2)` → `56832` (low surrogate); `("ab",5)` → `NaN` (R3 P01–P06) \| ferrum: none. Under D1 = code points the function reduces to `str.chars().nth(pos)`; under UTF-16 it is needed as-is; under UTF-8 bytes it would need byte semantics. Proposed disposition CORE stands but depends on D1. |
| state.cm6.214 | `fromCodePoint` | `CodeMirror6/state/src/char.ts:30` | none (searched `from_code_point`, `from_u32` wrappers) | MISSING | state.fromCodePoint.B1–B3 | none / none | D1 | repro: `fromCodePoint(0x1F600)` → cm6 `"😀"`; `0xD800` → lone surrogate; `0x110000` → `"\udc00\udc00"` (no validation); `-1` → `"￿"` (R3 F01–F07) \| ferrum: none (Rust `char::from_u32` returns `Option`, cannot produce lone surrogates). |
| state.cm6.215 | `codePointSize` | `CodeMirror6/state/src/char.ts:37` | none (searched `code_point_size`, `len_utf16`) | MISSING | state.codePointSize.B1 | none / none | D1: the value it returns is the UTF-16 length; under D1 = code points it would always be 1 | repro: `codePointSize(0xFFFF)` → `1`, `(0x10000)` → `2` (R3 Z01–Z03) \| ferrum: none. |
| state.cm6.216 | `countColumn` | `CodeMirror6/state/src/column.ts:5` | `count_column(string, tab_size, to: Option<usize>)` `ferrum/crates/ferrum-state/src/column.rs:8` | PORTED, DEVIATION | state.countColumn.B1–B7 | none / none | `to` UTF-16 vs char (D1) | Tab logic identical (R3 K01–K05); cluster differences inherited from find_cluster_break (CRLF counts 1 column in Ferrum, 2 in cm6: `countColumn("a\r\nb",4)` → cm6 `4` \| ferrum `3`) (R2); `tabSize 0` → cm6 `NaN` \| ferrum panic "attempt to calculate the remainder with a divisor of zero" (R3 K08); O(n²): 40 000-char ASCII line → cm6 0.35 ms \| ferrum 63 s (R5). Untested. |
| state.cm6.217 | `findColumn` | `CodeMirror6/state/src/column.ts:24` | `find_column(string, col, tab_size, strict) -> Option<usize>` `ferrum/crates/ferrum-state/src/column.rs:28` | PORTED, DEVIATION | state.findColumn.B1–B7 | none / none | result UTF-16 vs char (D1): `findColumn("😀b",1,4)` → cm6 `2` \| ferrum `Some(1)` | `-1` → `None` (idiomatic); non-strict always `Some`. Cluster differences as countColumn (R2); `tabSize 0` → cm6 returns `1` for `findColumn("\t",1,0)` \| ferrum panic (R3 J10). Same O(n²) cost. Untested. |

## Ferrum pub items

| id | Ferrum item | file:line | cm6 counterpart (package, file:line) or "none (searched: …)" | proposed status or candidate | notes + repro |
|---|---|---|---|---|---|
| state.fe.049 | `char::CharCategory` | `ferrum/crates/ferrum-state/src/char.rs:5` | `@codemirror/state` `CharCategory` `CodeMirror6/state/src/charcategory.ts:4` | PORTED | derives `Debug, Clone, Copy, PartialEq, Eq`; see state.cm6.166. |
| state.fe.050 | `char::CharCategory::Word` | `ferrum/crates/ferrum-state/src/char.rs:6` | `CharCategory.Word` `CodeMirror6/state/src/charcategory.ts:6` | PORTED | test_charcategory#001. |
| state.fe.051 | `char::CharCategory::Space` | `ferrum/crates/ferrum-state/src/char.rs:7` | `CharCategory.Space` `CodeMirror6/state/src/charcategory.ts:8` | PORTED | test_charcategory#002. |
| state.fe.052 | `char::CharCategory::Other` | `ferrum/crates/ferrum-state/src/char.rs:8` | `CharCategory.Other` `CodeMirror6/state/src/charcategory.ts:10` | PORTED | test_charcategory#003. |
| state.fe.053 | `char::categorize_char(ch: char)` | `ferrum/crates/ferrum-state/src/char.rs:12` | no export; closest is internal `makeCategorizer("")` `CodeMirror6/state/src/charcategory.ts:28` (reached through `EditorState.charCategorizer`, `CodeMirror6/state/src/state.ts:373`, chunk C5) | candidate: FERRUM-ONLY | `is_whitespace` → Space, `is_alphanumeric \|\| '_'` → Word, else Other. Exhaustive comparison over all 1 111 998 scalar values against cm6 `charCategorizer(0)` on single-code-point strings: 2 mismatches, U+0085 (cm6 Other, ferrum Space) and U+FEFF (cm6 Space, ferrum Other) (R7). Takes one `char`; cm6 categorizer takes a string (cluster) — see behaviors `state.categorizer.*` for multi-char inputs. Ignores `wordChars` language data. Used: `ferrum/crates/ferrum-state/src/state.rs:193`, `ferrum/crates/ferrum-commands/src/commands.rs:7,746`, and by `find_word_break`/`find_subword_break`. |
| state.fe.054 | `char::find_cluster_break` | `ferrum/crates/ferrum-state/src/char.rs:31` | `findClusterBreak` `CodeMirror6/state/src/char.ts:9` | PORTED, DEVIATION | see state.cm6.212. |
| state.fe.055 | `char::find_word_break(text, pos, forward)` | `ferrum/crates/ferrum-state/src/char.rs:70` | none in `@codemirror/state`. Comparable logic in `@codemirror/view`: `byGroup` `CodeMirror6/view/src/cursor.ts:102` (used by `EditorView.moveByGroup` `CodeMirror6/view/src/editorview.ts:694`), `groupAt` `CodeMirror6/view/src/cursor.ts:15`; in `@codemirror/commands`: `toGroupStart` `CodeMirror6/commands/src/commands.ts:93` | candidate: FERRUM-ONLY or PLACEMENT DEVIATION (cm6 logic lives in view/commands) | Facts: forward from a Space char stops at the end of the whitespace run: `find_word_break("foo bar",3,true)` → `4` (R8 W01). cm6 `byGroup` (`CodeMirror6/view/src/cursor.ts:105-107`) switches category after leading whitespace (`if (cat == Space) cat = nextCat`), so a group motion from 3 would continue over `bar` (read from code; cm6 side not executed — needs an EditorView). Uses `categorize_char` (no `wordChars`), whole-text `find_cluster_break` per step (O(n²) per call). No ferrum-state test. Used: `ferrum/crates/ferrum-commands/src/commands.rs:182,204,225,246`, `ferrum/crates/ferrum-view-gpui/src/editor_view.rs:235-236`, `ferrum/crates/ferrum-view-gpui/src/tests.rs:151-152`. |
| state.fe.056 | `char::find_subword_break(text, pos, forward)` | `ferrum/crates/ferrum-state/src/char.rs:101` | none in `@codemirror/state`; comparable: `moveBySubword` `CodeMirror6/commands/src/commands.ts:115` (`@codemirror/commands`) | candidate: FERRUM-ONLY or PLACEMENT DEVIATION | Facts: per-`char` (not per cluster), no `Intl.Segmenter` CJK handling (cm6 `CodeMirror6/commands/src/commands.ts:112,146-156`), no special "word punctuation" category (cm6 :123-124). `find_subword_break("ab",5,false)` panics "index out of bounds: the len is 2 but the index is 4" (R8 S05). Outputs: `"fooBarBaz"` fwd 0 → `3`, bwd 9 → `6`, `"HTMLParser"` fwd 0 → `4` (R8). Used: `ferrum/crates/ferrum-commands/src/commands.rs:268,290,311,332`. No ferrum-state test. |
| state.fe.058 | `column::count_column` | `ferrum/crates/ferrum-state/src/column.rs:8` | `countColumn` `CodeMirror6/state/src/column.ts:5` | PORTED, DEVIATION | see state.cm6.216. Module doc claims "1:1 parity with CodeMirror 6" (`column.rs:2`): false for CRLF/cluster inputs (R2) and tabSize 0. |
| state.fe.059 | `column::find_column` | `ferrum/crates/ferrum-state/src/column.rs:28` | `findColumn` `CodeMirror6/state/src/column.ts:24` | PORTED, DEVIATION | see state.cm6.217. |
| state.fe.176 | `selection::SelectionRange` | `ferrum/crates/ferrum-state/src/selection.rs:7` | `SelectionRange` `CodeMirror6/state/src/selection.ts:24` | PORTED, DEVIATION | see state.cm6.077. |
| state.fe.177 | `SelectionRange.anchor` (pub field) | `ferrum/crates/ferrum-state/src/selection.rs:9` | `SelectionRange.anchor` getter `CodeMirror6/state/src/selection.ts:40` | PORTED, RUST-IDIOMATIC | see state.cm6.081. |
| state.fe.178 | `SelectionRange.head` (pub field) | `ferrum/crates/ferrum-state/src/selection.rs:11` | `SelectionRange.head` getter `CodeMirror6/state/src/selection.ts:44` | PORTED, RUST-IDIOMATIC | see state.cm6.082. |
| state.fe.179 | `SelectionRange.goal_column` (pub field) | `ferrum/crates/ferrum-state/src/selection.rs:13` | `SelectionRange.goalColumn` `CodeMirror6/state/src/selection.ts:35` | PORTED, DEVIATION | see state.cm6.080 (mutable, `usize`, untested). |
| state.fe.180 | `SelectionRange.assoc` (pub field) | `ferrum/crates/ferrum-state/src/selection.rs:15` | `SelectionRange.assoc` getter `CodeMirror6/state/src/selection.ts:53` | PORTED, DEVIATION | see state.cm6.084. |
| state.fe.181 | `SelectionRange::cursor(pos)` | `ferrum/crates/ferrum-state/src/selection.rs:20` | `EditorSelection^cursor` `CodeMirror6/state/src/selection.ts:200` | PORTED, DEVIATION | see state.cm6.074 (missing assoc/bidi/goal args). |
| state.fe.182 | `SelectionRange::new(anchor, head)` | `ferrum/crates/ferrum-state/src/selection.rs:30` | `EditorSelection^range` `CodeMirror6/state/src/selection.ts:208` | PORTED, DEVIATION | see state.cm6.075 (no auto assoc). |
| state.fe.183 | `SelectionRange::from()` | `ferrum/crates/ferrum-state/src/selection.rs:41` | `SelectionRange.from` `CodeMirror6/state/src/selection.ts:27` | PORTED, RUST-IDIOMATIC | see state.cm6.078. |
| state.fe.184 | `SelectionRange::to()` | `ferrum/crates/ferrum-state/src/selection.rs:47` | `SelectionRange.to` `CodeMirror6/state/src/selection.ts:29` | PORTED, RUST-IDIOMATIC | see state.cm6.079. |
| state.fe.185 | `SelectionRange::is_empty()` | `ferrum/crates/ferrum-state/src/selection.rs:53` | `SelectionRange.empty` `CodeMirror6/state/src/selection.ts:47` | PORTED, RUST-IDIOMATIC | see state.cm6.083. |
| state.fe.186 | `SelectionRange::is_inverted()` | `ferrum/crates/ferrum-state/src/selection.rs:59` | none public (searched `inverted` in `CodeMirror6/state/src/*.ts`: internal `RangeFlag.Inverted` `CodeMirror6/state/src/selection.ts:16`, observable only as `head < anchor`) | candidate: FERRUM-ONLY | `head < anchor`; equals cm6's internal Inverted flag for every range cm6 can build except `undirectionalRange(5,2)`, which stores from 5 / to 2 without the flag (R1 S11). Used: none outside selection.rs (grep `is_inverted` all crates). Untested. |
| state.fe.187 | `SelectionRange::with_goal_column(Option<usize>)` | `ferrum/crates/ferrum-state/src/selection.rs:64` | none (cm6 sets goalColumn only through `EditorSelection.cursor`/`range` arguments, `CodeMirror6/state/src/selection.ts:200,208`) | candidate: FERRUM-ONLY | Builder replacing the missing constructor argument. Used: `ferrum/crates/ferrum-commands/src/commands.rs` (4 sites). Untested in ferrum-state. |
| state.fe.188 | `SelectionRange::map` | `ferrum/crates/ferrum-state/src/selection.rs:70` | `SelectionRange.map` `CodeMirror6/state/src/selection.ts:73` | PORTED, DEVIATION | see state.cm6.087. |
| state.fe.189 | `SelectionRange::extend` | `ferrum/crates/ferrum-state/src/selection.rs:98` | `SelectionRange.extend` `CodeMirror6/state/src/selection.ts:85` | PORTED, DEVIATION | see state.cm6.088. |
| state.fe.190 | `selection::EditorSelection` | `ferrum/crates/ferrum-state/src/selection.rs:117` | `EditorSelection` `CodeMirror6/state/src/selection.ts:115` | PORTED, DEVIATION | see state.cm6.061. |
| state.fe.191 | `EditorSelection::cursor(pos)` | `ferrum/crates/ferrum-state/src/selection.rs:130` | no same-named equivalent: cm6 `EditorSelection.cursor` returns a `SelectionRange`; this function equals cm6 `EditorSelection.single(pos)` `CodeMirror6/state/src/selection.ts:182` | candidate: FERRUM-ONLY (name collides with cm6 `EditorSelection.cursor` with a different return type) | repro: ferrum `EditorSelection::cursor(5)` → `[5/5] main=0` (R1 S07b) = cm6 `EditorSelection.single(5)`. Used widely (31 sites in `ferrum/crates/ferrum-commands/src/commands.rs`, tests in ferrum-commands/ferrum-view-gpui). Untested in ferrum-state C3 tests (used in test_state#024 setup, C5). |
| state.fe.192 | `EditorSelection::single(anchor, head)` | `ferrum/crates/ferrum-state/src/selection.rs:138` | `EditorSelection^single` `CodeMirror6/state/src/selection.ts:182` | PORTED, DEVIATION | see state.cm6.072. |
| state.fe.193 | `EditorSelection::create` | `ferrum/crates/ferrum-state/src/selection.rs:147` | `EditorSelection^create` `CodeMirror6/state/src/selection.ts:188` | PORTED, DEVIATION | see state.cm6.073. Doc comment "merges overlapping ranges matching CodeMirror 6" (`selection.rs:146`): false for main-index tracking with duplicates (R4) and for empty input. |
| state.fe.194 | `EditorSelection::main()` | `ferrum/crates/ferrum-state/src/selection.rs:207` | `EditorSelection.main` `CodeMirror6/state/src/selection.ts:147` | PORTED | see state.cm6.066. |
| state.fe.195 | `EditorSelection::main_index()` | `ferrum/crates/ferrum-state/src/selection.rs:213` | `EditorSelection.mainIndex` `CodeMirror6/state/src/selection.ts:122` | PORTED, DEVIATION | see state.cm6.063. |
| state.fe.196 | `EditorSelection::ranges()` | `ferrum/crates/ferrum-state/src/selection.rs:219` | `EditorSelection.ranges` `CodeMirror6/state/src/selection.ts:119` | PORTED, RUST-IDIOMATIC | see state.cm6.062. |
| state.fe.197 | `EditorSelection::is_single()` | `ferrum/crates/ferrum-state/src/selection.rs:225` | none (cm6 callers test `ranges.length == 1`, e.g. `CodeMirror6/state/src/selection.ts:152`) | candidate: FERRUM-ONLY | `ranges.len() == 1`. Used: `ferrum/crates/ferrum-commands/src/commands.rs` (1 site). Untested. |
| state.fe.198 | `EditorSelection::map` | `ferrum/crates/ferrum-state/src/selection.rs:230` | `EditorSelection.map` `CodeMirror6/state/src/selection.ts:127` | PORTED, DEVIATION | see state.cm6.064. |
| state.fe.365 | `impl Default for EditorSelection` | `ferrum/crates/ferrum-state/src/selection.rs:122` | none as an API; cm6 `EditorState.create` defaults the selection to `EditorSelection.single(0)` (`CodeMirror6/state/src/state.ts:247`) | candidate: FERRUM-ONLY | `Default` → `cursor(0)` → `[0/0] main=0` (R1 S94), equal to cm6 `single(0)`. Used by `EditorState::create` `ferrum/crates/ferrum-state/src/state.rs:107` (`unwrap_or_default`). Untested directly (test_state#001 asserts main().from() == 0, C5). |

## Reproductions

All scripts: cm6 under `ferrum/docs/parity/state/oracle/cm6-probes/probe-C3/` (run `cd ferrum/docs/parity/state/oracle.md && tsx probe-C3/<x>.ts`), Ferrum under `ferrum/docs/parity/state/oracle/ferrum-repro/repro-C3/src/bin/` (run `cd ferrum/docs/parity/state/oracle/ferrum-repro/repro-C3 && cargo run -q [--release] --bin <x>`). Outputs captured in `ferrum/docs/parity/state/oracle/cm6-probes/probe-C3/*.out`, `ferrum/docs/parity/state/oracle/ferrum-repro/repro-C3/*.out`, `(oracle data, not archived: regenerate from the seed recorded in `ferrum/docs/parity/state/oracle.md`)`.

### R1 Selection probe (cm6 `sel.ts`, Ferrum `sel.rs`)

Each line label (S01…S99) is the same scenario on both sides; cm6 output first. Ferrum uses `SelectionRange::new` for `range`, `SelectionRange::cursor` + field writes for `cursor(pos, assoc, _, goal)`, doc length 10 for change descriptions (`ChangeSet::of_spec(ChangeSpec::insert/delete, 10)` vs cm6 `ChangeSet.of({...}, 10)`).

| label | input | cm6 | ferrum |
|---|---|---|---|
| S01 | range(3,2) / new(3,2) | from 2,to 3,anchor 3,head 2,**assoc 1**,bidi null | from 2,to 3,anchor 3,head 2,**assoc 0** |
| S02 | range(2,3) | **assoc -1** | **assoc 0** |
| S03 | range(4,4) | assoc 0 | assoc 0 |
| S04 | range(2,3,goal 7,bidi 2,assoc 1) | assoc 1,bidi 2,goal 7 | `new(2,3).with_goal_column(Some(7))` → goal Some(7), assoc 0, no bidi/assoc args |
| S05 | range(2,3,_,bidi 9) | bidi **6** (clamped by `Math.min(6, …)`) | no counterpart |
| S06 | range(4,4,_,_,assoc -1) | assoc -1 | no counterpart (field write only) |
| S07 | cursor(5) | assoc 0, bidi null, goal undefined | `SelectionRange::cursor(5)`: assoc 0, goal None |
| S07b | — | — | `EditorSelection::cursor(5)` → `[5/5] main=0` (an EditorSelection) |
| S08 | cursor(5,-1) | assoc -1 | no argument |
| S09 | cursor(5,1,9,12) | assoc 1, bidi 6, goal 12 | no arguments |
| S10 | cursor(5,3,0) | assoc 1 (any positive), bidi 0 | no arguments |
| S11 | undirectionalRange(5,2) | from **5**, to **2**, anchor 5, head 2, assoc 0, bidi **0**, undir true | no counterpart |
| S12 | undirectionalRange(2,5) | from 2,to 5, bidi 0, undir true | no counterpart |
| S20 | cursor(5).map(insert "x" at 5) | 5 | 5 (assoc -1 passed explicitly) |
| S21 | same, assoc 1 | 6 | 6 |
| S22 | cursor(5,assoc 1).map(ins 5) default assoc | 5 (range's own assoc not used for mapping), assoc kept 1 | 5, assoc kept 1 |
| S23 | range(2,5).map(ins 2) | 3–6, assoc -1 | 3–6, assoc 0 |
| S24 | range(2,5).map(ins 5) | 2–5 | 2–5 |
| S25 | range(2,5).map(ins 2, assoc 1) | 3–6 | 3–6 |
| S26 | range(5,2).map(ins 2) | anchor 6, head 3 | anchor 6, head 3 |
| S27 | range(2,5).map(del 1–6) | 1/1, assoc -1 | 1/1, assoc 0 |
| S28 | range(5,2).map(del 1–6) | 1/1, assoc 1 | 1/1, assoc 0 |
| S29 | cursor(5,1,bidi 3,goal 8).map(ins "ab" at 0) | 7, assoc 1, bidi 3, goal 8 | 7, assoc 1, goal Some(8) |
| S30 | undirectionalRange(2,5).map(ins "ab" at 0) | 4–7, undir true | no counterpart |
| S31 | identity when unchanged | `r.map(ins 8) === r` → true | n/a (value type) |
| S32 | range(2,4).map(ins "xy" at 3) | 2–6 | 2–6 |
| S33 | cursor(4).map(del 2–6) | 2 | 2 |
| S34 | cursor(12).map(ins at 0, doc len 10) | RangeError: Position 12 is out of range for changeset of length 10 | PANIC "Position 12 is out of range for changeset of length 10" |
| S35 | range(2,5).map(ins 5, assoc 1) | 2–5 (assoc arg ignored for non-empty) | 2–5 |
| S40 | range(5,8).extend(2) | anchor 5, head 2, assoc 1 | `extend(2,2)`: anchor 5, head 2, assoc 0 |
| S41 | range(5,8).extend(3,10) | 3/10 | 3/10 |
| S42 | extend(6,9) | 5/9 | 5/9 |
| S43 | extend(1,9) | 1/9 | 1/9 |
| S44 | extend(7,1) | 5/1 | 5/1 |
| S45 | cursor(5).extend(5,5,1) | assoc 1 | no assoc argument |
| S46 | cursor(5,0,bidi 2,goal 7).extend(9) | 5/9, goal undefined, bidi null | 5/9, goal None |
| S47 | range(8,5).extend(3) | 8/3 | 8/3 |
| S48 | range(5,8).extend(9,2) | 5/9 | 5/9 |
| S49 | cursor(5).extend(3,7,-1) / cursor(5,assoc 1).extend(5,5) | 3/7 assoc -1 | 5/5 assoc 0 (assoc reset) |
| S50 | cursor(5,-1) eq cursor(5,1) | true | `==` false |
| S51 | same with includeAssoc | false | no parameter |
| S52 | range(2,5,assoc 1).eq(range(2,5,assoc -1), true) | true | `==` false |
| S53 | cursor goal 3 eq cursor | false | false |
| S54 | range(2,5) eq range(5,2) | false | false |
| S55 | cursor bidi 1 eq cursor bidi 2 | true | n/a |
| S56 | undirectionalRange(2,5) eq range(2,5) | true | n/a |
| S57 | selections differing only in main | false | false |
| S58 | single(1) eq create([cursor(1,1)]) | true | `single(1,1) == create([cursor 1 assoc 1],0)` → false |
| S59 | same with includeAssoc | false | no parameter |
| S5A | different lengths | false | (derived, same) |
| S60 | range(3,1).toJSON() | {"anchor":3,"head":1} | none |
| S61 | cursor(4,1,2,9).toJSON() | {"anchor":4,"head":4} | none |
| S62 | SelectionRange.fromJSON({anchor:3,head:1}) | anchor 3, head 1, assoc 1 | none |
| S63 | fromJSON({anchor:"3",head:1}) | RangeError: Invalid JSON representation for SelectionRange | none |
| S64 | fromJSON(null) | RangeError (same) | none |
| S65 | fromJSON({anchor:-4,head:1.5}) | accepted: from -4, to 1.5 | none |
| S66 | create([range(0,2),cursor(5)],1).toJSON() | {"ranges":[{"anchor":0,"head":2},{"anchor":5,"head":5}],"main":1} | none |
| S67 | EditorSelection.fromJSON(valid) | [0/2,5/5] main=1 | none |
| S68 | main ≥ ranges.length | RangeError: Invalid JSON representation for EditorSelection | none |
| S69 | missing ranges | RangeError (same) | none |
| S6A | unsorted/overlapping ranges | [5/5,0/6] main=0 (not normalized) | none |
| S6B | {ranges:[],main:-1} | accepted: 0 ranges, main undefined | none |
| S6C | main 0.5 | accepted: main range undefined | none |
| S6D | range without head | RangeError: Invalid JSON representation for SelectionRange | none |
| S70 | single(3) | [3/3] main=0 | `single(3,3)` (head required) → [3/3] main=0 |
| S71 | single(3,1).main | anchor 3, head 1, assoc 1 | anchor 3, head 1, assoc 0 |
| S73 | create([]) | RangeError: A selection needs at least one range | `[0/0] main=0` |
| S74 | create([c1,c3],5) | [1/1,3/3] main=5, main undefined | [1/1,3/3] main=1 |
| S75 | create([c3,c1],5) | [1/1,3/3] main=-1, main undefined | [1/1,3/3] main=0 |
| S76 | create([0/2,2/4]) | [0/2,2/4] (touching non-empty kept) | same |
| S77 | create([0/2,c2]) | [0/2] (cursor at end merged) | same |
| S78 | create([c2,2/4]) | [2/2,2/4] (cursor at start kept) | same |
| S79 | create([c0,c0]) | [0/0] | same |
| S7A | create([0/3,2/5]) | [0/5] assoc -1 | [0/5] assoc 0 |
| S7B | create([0/3,5/2]) | [5/0] assoc 1 (direction of later range) | [5/0] assoc 0 |
| S7C | create([cursor(3,1,2,7), undirectionalRange(1,4)]) | [1/4] goal/bidi/undir dropped | create([cursor 3 assoc 1 goal 7, new(1,4)]) → [1/4] goal None |
| S7D | create([cursor(3,1,…), cursor(3,-1)]) | [3/3] assoc 0 | [3/3] assoc 0 |
| S7E | create([c9,0/3,2/5,c7],3) | [0/5,7/7,9/9] main=1 | same |
| S7F | same, main 2 | main=0 | main=0 |
| S7G | create([cursor(3,1,2,7)]) (sorted path) | flags kept | fields kept |
| S7H | create([1/3,c1,1/3],2) | [1/3] main=0 | same |
| S7I | create([cursor(-3)]) | [-3/-3] (no validation) | n/a (`usize`) |
| S80 | [c1,3/5,c8] main 1 .main | 3/5 | 3/5 |
| S81–S8A | asSingle/addRange/replaceRange | see rows 067–069 | none |
| S90 | sel.map(empty change) | `=== sel` true | `== sel` true |
| S91 | sel.map(del 0–6) | [0/0,2/2] main=0 | same |
| S92 | sel.map(ins "xx" at 1, assoc 1) | [3/3,5/7,10/10] main=1 | same positions |
| S93 | same, assoc -1 | [1/1,5/7,10/10] main=1 | same |
| S94 | Default | — | [0/0] main=0 |
| S95 | checkSelection(single(5), 4) | RangeError: Selection points outside of document | no counterpart |
| S96 | checkSelection(single(4), 4) | ok | — |
| S97 | EditorState.create({doc:"abc", selection: single(5)}) | RangeError: Selection points outside of document | ferrum state created, `selection().main().head` = 5 |
| S98 | state.update({selection:{anchor:9}}) on "abc" | RangeError: Selection points outside of document | ferrum (`single.rs`): `update(with_selection(cursor(9)))` applied, head = 9 |
| S99 | create with single(-1) | accepted, head -1 (checkSelection only checks `to > length`) | n/a (`usize`) |

### R2 Cluster/column table (cm6 `gen.ts`, Ferrum `charcol.rs`, compare `compare.ts`)

For each string and each code-point index i, both sides compute forward/backward cluster breaks, `countColumn(s, 4, i)`, `countColumn(s, 4)`, `findColumn(s, col, 4[, strict])` for col 0..4n+2; cm6 inputs/outputs converted between code-point index and UTF-16 offset so the two are comparable. Output in `(oracle data, not archived: regenerate from the seed recorded in `ferrum/docs/parity/state/oracle.md`)`. Table cases that **matched** completely: T01 plain, T04 LF CR, T05 e+U+0301+x, T06 two combining, T07 leading combining, T08 ZWJ family 👨‍👩‍👧‍👦, T10 trailing ZWJ, T11 double ZWJ, T12 two flags, T13 three RIs, T14 👍🏽, T15 👩🏽‍💻, T16 precomposed Hangul, T20 क + ि (SpacingMark), T21 ❤️, T22 keycap 1️⃣, T24 Thai SARA AM, T26 a+U+20DD, T27 😀😀, T28 CJK, T29 tabs, T30 a+ZWNJ+b, T32 ☺︎.

Mismatches (code-point units; `fwd:` lists `findClusterBreak(s,i,true)` for i = 0..n):

| case | string (code points) | cm6 fwd / bwd | ferrum fwd / bwd | countColumn(full) cm6 / ferrum |
|---|---|---|---|---|
| T02 CRLF | 61 0D 0A 62 | 1,2,3,4,4 / 0,0,1,2,3 | 1,3,3,4,4 / 0,0,1,1,3 | 4 / 3 |
| T03 CR CR LF | 0D 0D 0A | 1,2,3,3 / 0,0,1,2 | 1,3,3,3 / 0,0,1,1 | 3 / 2 |
| T09 ZWJ between letters | 61 200D 62 | 3,3,3,3 / 0,0,0,0 | 2,2,3,3 / 0,0,0,2 | 1 / 2 |
| T17 Hangul jamo L V T | 1100 1161 11A8 | 1,2,3,3 / 0,0,1,2 | 3,3,3,3 / 0,0,0,0 | 3 / 1 |
| T18 Hangul LV + T | AC00 11A8 | 1,2,2 / 0,0,1 | 2,2,2 / 0,0,0 | 2 / 1 |
| T19 Devanagari क्षि | 0915 094D 0937 093F | 2,2,4,4,4 / 0,0,0,2,2 | 4,4,4,4,4 / 0,0,0,0,0 | 2 / 1 |
| T23 Arabic Prepend | 0600 0661 | 1,2,2 / 0,0,1 | 2,2,2 / 0,0,0 | 2 / 1 |
| T25 England tag flag | 1F3F4 E0067 E0062 E0065 E006E E0067 E007F | 1,2,3,4,5,6,7,7 / 0,0,1,2,3,4,5,6 | 7,…,7 / 0,…,0 | 7 / 1 |
| T31 Bengali ক্ষ | 0995 09CD 09B7 | 2,2,3,3 / 0,0,0,2 | 3,3,3,3 / 0,0,0,0 | 2 / 1 |
| T33 CR + combining | 0D 0301 | 2,2,2 / 0,0,0 | 1,2,2 / 0,0,1 | 1 / 2 |
| T34 LF + ZWJ + emoji | 0A 200D 1F600 | 3,3,3,3 / 0,0,0,0 | 1,2,3,3 / 0,0,1,2 | 1 / 3 |

Cause, from the cm6 dependency source (`@marijn/find-cluster-break/src/index.js:45-66`, table built by `bin/build-extenders.js`): cm6 joins a code point to the previous one iff it is ZWJ, follows ZWJ, is in its "extending" table (GCB=Extend plus GCB=SpacingMark minus 31 listed exceptions, restricted to code points < U+30000, so tags U+E0020–E007F and VS supplement U+E0100–E01EF are not extending), or completes a regional-indicator pair. It has no CR/LF/Control rules (GB3–GB5), no Hangul rules (GB6–GB8), no Prepend (GB9b), no conjunct rule (GB9c), and ZWJ joins regardless of Extended_Pictographic (GB11). `unicode-segmentation` 1.13.3 `grapheme_indices(true)` implements UAX #29 extended clusters.

### R3 Char/column extras (cm6 `char.ts`, Ferrum `char.rs`)

| label | input | cm6 | ferrum |
|---|---|---|---|
| C01 | fcb("éx",0,true,**false**) | 1 | no includeExtending param |
| C02 | fcb("éx",0) | 2 | 2 |
| C03 | fcb("éx",2,false,false) | 1 | no param |
| C04 | fcb("éx",2,false) | 0 | 0 |
| C05 | fcb("a‍b",0,true,false) | 3 (ZWJ joins even without includeExtending) | no param |
| C06 | fcb("😀x",1) (inside surrogate pair) | 2 (moves to pair start first) | unrepresentable in char units |
| C07 | fcb("😀x",1,false) | 0 | unrepresentable |
| C08 | fcb("😀",0) | 2 (UTF-16) | 1 (char) |
| C09 | fcb("a\ud83db",1) lone high surrogate | 2 | `&str` cannot hold lone surrogates |
| C10 | fcb("a\ude00b",1) lone low surrogate | 2 | same |
| C11 | fcb("\ude00",0) | 1 | same |
| C12 | fcb("ab",5) | 7 (pos > length: NaN code point, advances by 2) | 5 (returns pos) |
| C13 | fcb("ab",5,false) | 2 | 1 |
| C14–C17 | at end / at start / empty string | 2 / 0 / 0 / 0 | 2 / 0 / 0 / 0 |
| C18 | fcb("ab",-1) | 1 | n/a (`usize`) |
| C19 | fcb("❤️",0,true,false) | 1 | no param |
| C20 | fcb("👍🏽",0,true,false) | 2 (modifier not joined) | no param |
| C21 | flags "🇫🇷🇩🇪" fwd from 0 | 4 (UTF-16) | 2 (char) |
| C22 | bwd from end | 4 (UTF-16) | 2 (char) |
| C23/C24 | from inside the first flag (UTF-16 2 / char 1) | 4 | fwd 2; bwd from char 3 → 2 |
| P01 | codePointAt("a😀",1) | 128512 | none |
| P02 | codePointAt("a😀",2) | 56832 | none |
| P03 | codePointAt("\ud83d",0) | 55357 | none |
| P04 | codePointAt("\ud83dx",0) | 55357 | none |
| P05 | codePointAt("ab",5) | NaN (JSON prints null) | none |
| P06 | codePointAt("ab",0) | 97 | none |
| F01–F07 | fromCodePoint 0x61 / 0x1F600 / 0xD800 / 0x110000 / -1 / 0xFFFF / 0x10000 | "61" / "d83d de00" / "d800" / "dc00 dc00" / "ffff" / "ffff" / "d800 dc00" | none |
| Z01–Z03 | codePointSize 0xFFFF / 0x10000 / 0x61 | 1 / 2 / 1 | none |
| K01 | countColumn("\tab",4) | 6 | 6 |
| K02 | countColumn("ab\tc",4) | 5 | 5 |
| K03 | countColumn("a\t\t",2) | 4 | 4 |
| K04 | countColumn("abcd",4,2) | 2 | 2 |
| K05 | countColumn("abcd",4,10) | 4 | 4 |
| K06 | countColumn("😀b",4,1) (to inside pair) | 1 | unrepresentable |
| K07 | countColumn("😀b",4) | 2 | 2 |
| K08 | countColumn("\t",0) | NaN | PANIC attempt to calculate the remainder with a divisor of zero |
| K09 | countColumn("ab",4,-1) | 0 | n/a |
| K10 | countColumn("中x",4) | 2 (no East Asian width) | 2 |
| J01 | findColumn("\tab",5,4) | 2 | Some(2) |
| J02 | findColumn("\tab",2,4) (inside tab) | 1 | Some(1) |
| J03 | findColumn("ab",5,4) | 2 | Some(2) |
| J04 | findColumn("ab",5,4,true) | -1 | None |
| J05 | findColumn("ab",2,4,true) | 2 | Some(2) |
| J06 | findColumn("😀b",1,4) | 2 (UTF-16) | Some(1) (char) |
| J07 | findColumn("éb",1,4) | 2 | Some(2) |
| J08 | findColumn("ab",0,4,true) | 0 | Some(0) |
| J09 | findColumn("ab",-3,4) | 0 | n/a |
| J10 | findColumn("\t",1,0) | 1 (n becomes NaN; `NaN >= 1` false, loop ends at length) | PANIC divisor of zero |
| J11 | findColumn("",0,4,true) | 0 | Some(0) |
| J12 | findColumn("",1,4,true) | -1 | None |

### R4 Selection oracle (`selgen.ts` / `selorc.rs`)

Seed `0x5E1C`, PRNG mulberry32, 5000 cases: 1–6 ranges with anchor/head in 0..19 (40 % empty), random main index, one replacement change on a 20-char doc (from 0..20, length 0–4, insert 0–3 chars), assoc ±1. Compared: `create` result (anchor/head list + main index), `map` result, assoc of every range. Result: create positions+main 4999/5000 equal, map 4999/5000, assoc 620/5000 equal (all differences are cm6 ±1 vs Ferrum 0). Mismatch minimised by hand:

```
cm6:    EditorSelection.create([cursor(12), range(12,18), cursor(12)], 2) → "12/12,12/18 main=1"
ferrum: EditorSelection::create(vec![cursor(12), new(12,18), cursor(12)], 2) → "12/12,12/18 main=0"
```
(`ferrum/docs/parity/state/oracle/cm6-probes/probe-C3/min.ts`, `ferrum/docs/parity/state/oracle/ferrum-repro/repro-C3/src/bin/min.rs`.) Cause: cm6 `ranges.indexOf(main)` (`CodeMirror6/state/src/selection.ts:226`) uses object identity; Ferrum `ranges.iter().position(|r| *r == main)` (`selection.rs:174`) finds the first equal value. The main range ends up being the cursor instead of the merged range that absorbed the original main cursor.

### R5 Complexity (`perf.rs`, `perf2.rs` release build; `perf.ts`, `perf2.ts`)

| n (ASCII 'a' × n) | ferrum `count_column(s,4,None)` | cm6 `countColumn(s,4)` | ferrum 1000 × `find_cluster_break(s, n/2, true)` | cm6 1000 × | ferrum 1 × `find_cluster_break(s, n, false)` | cm6 1 × |
|---|---|---|---|---|---|---|
| 10 000 | 3.66 s | 4.17 ms (JIT warm-up) | 365 ms | 0.24 ms | 3.5 ms | 0.22 ms |
| 20 000 | 16.6 s | 1.37 ms | 746 ms | 0.20 ms | 13.0 ms | 0.012 ms |
| 40 000 | 63.0 s | 0.35 ms | 1.46 s | 0.14 ms | 45.0 ms | 0.009 ms |

Ferrum `find_cluster_break` counts all chars, converts char→byte offset, collects every grapheme of the whole string into a `Vec`, and (backward) re-counts the chars of each prefix (`char.rs:32,38-44,63`): O(n) forward, O(n²) backward per call. `count_column`/`find_column` call it once per cluster: O(n²). cm6 looks at most a few code units around `pos` (O(1) amortised forward; backward calls `nextClusterBreak(pos-2)` a few times).

### R6 Single selection / bounds (`single.rs`)

`EditorState::create({doc:"123", selection: create([cursor 0, cursor 1])})` without `allowMultipleSelections` → ferrum keeps 2 ranges; cm6 `CodeMirror6/state/src/state.ts:251` keeps 1 (`asSingle`). `update(with_selection(cursor(9)))` on a 3-char doc → ferrum applies it (head 9); cm6 throws (R1 S98).

### R7 Categorizer, exhaustive (`catall.ts` / `catall.rs`)

Every scalar value U+0000–U+10FFFF except surrogates, as a one-code-point string: cm6 `EditorState.create({}).charCategorizer(0)` vs ferrum `state.char_categorizer(0)` (asserted identical to `categorize_char`). Result in `(oracle data, not archived: regenerate from the seed recorded in `ferrum/docs/parity/state/oracle.md`)`: 2 mismatches — U+0085 cm6 Other / ferrum Space; U+FEFF cm6 Space / ferrum Other. Multi-code-point inputs (`char.ts` / `char.rs` G-lines):

| input | cm6 | ferrum |
|---|---|---|
| "" | Space (`!/\S/.test("")`) | Other |
| " ́" | Other | Space (first char only) |
| " a" | Word (any word char in the string) | Space |
| "/a" | Word | Other |
| "a " | Word | Word |
| "é" | Word | Word |
| "́ " | Other | Other |
| NEL U+0085 | Other | Space |
| BOM U+FEFF | Space | Other |
| `$` with `wordChars: "$"` language data | Word | no wordChars support (Other) |

All other single characters in the G list agree (space, tab, NBSP, U+3000, U+2028, ZWSP U+200B Other, U+180E Other, digits of other scripts Word, Roman numeral/superscript/fraction Word, combining mark alone Other, virama Other, vowel sign i Word, CJK/Hiragana/Hangul Word, emoji/ZWJ sequence/flags Other, `$`/`-` Other, ß/ª/Ⓐ/𝐀 Word).

### R8 Ferrum-only word helpers (`word.rs`)

`find_word_break("foo bar",3,true)` → 4; `("foo bar",0,true)` → 3; `("foo bar",4,false)` → 3; `("foo  ..bar",3,true)` → 5; `("ab",5,false)` → 0; `("ée",0,true)` → 3. `find_subword_break("fooBarBaz",0,true)` → 3; `("fooBarBaz",9,false)` → 6; `("HTMLParser",0,true)` → 4; `("foo_bar",0,true)` → 3; `("  foo",0,true)` → 2; `("ab",5,false)` → PANIC index out of bounds: the len is 2 but the index is 4.

## Unfinished

none
