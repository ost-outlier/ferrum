# C3 tests

> What: chunk C3 test mapping. Versions: cm6 `@codemirror/state` 6.7.6 (`CodeMirror6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`ferrum/crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `ferrum/docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `ferrum/docs/parity/state/raw/C3-api.md` § Reproductions.

Totals: cm6 cases owned 7, Ferrum tests owned 7.

cm6 cases: `CodeMirror6/state/test/test-selection.ts` 4 (test-selection#01–#04), `CodeMirror6/state/test/test-charcategory.ts` 3 (test-charcategory#01–#03). Ferrum tests: `ferrum/crates/ferrum-state/src/tests/test_selection.rs` 4 (test_selection#001–#004), `ferrum/crates/ferrum-state/src/tests/test_charcategory.rs` 3 (test_charcategory#001–#003). All 7 Ferrum tests pass (`cargo test -p ferrum-state -- test_selection test_charcategory`, run 2026-10-21 with a scratch target dir: 7 passed). Both test bodies were compared line by line.

## Table 1: cm6 cases (one row per cm6 case)

| cm6 id | file:line | scenario (purpose; input doc/selection/config/extensions; action; expected) | Ferrum id(s) | Ferrum file:line | Ferrum scenario | verdict | behaviors covered | T4 changed expectations (value cm6 → value Ferrum, documented cause or "none documented") |
|---|---|---|---|---|---|---|---|---|
| test-selection#01 | `CodeMirror6/state/test/test-selection.ts:5` | "stores ranges with a primary range": `EditorSelection.create([range(0,1), range(3,2), range(4,5)], 1)` (no doc, no state); expects `main.from 2`, `main.to 3`, `main.anchor 3`, `main.head 2`, ranges `"0/1,3/2,4/5"` | test_selection#001 | `ferrum/crates/ferrum-state/src/tests/test_selection.rs:4` | `EditorSelection::create(vec![new(0,1), new(3,2), new(4,5)], 1)`; same five assertions (`from()`, `to()`, `anchor`, `head`, joined `anchor/head` string) with the same values | EQUIVALENT | state.EditorSelection.B1, state.EditorSelection.ranges.B2, state.EditorSelection.mainIndex.B1, state.EditorSelection.main.B1, state.EditorSelection.create.B2, state.EditorSelection.create.B3 (in-range main only), state.EditorSelection.range.B4, state.SelectionRange.B1, state.SelectionRange.from.B1, state.SelectionRange.to.B1, state.SelectionRange.anchor.B1, state.SelectionRange.head.B1 | none (identical values). Constructor differs (`EditorSelection.range` sets assoc ±1, `SelectionRange::new` sets 0), but neither test asserts assoc. |
| test-selection#02 | `CodeMirror6/state/test/test-selection.ts:14` | "merges and sorts ranges when normalizing": `create` of 8 ranges `10/12, 6/7, 4/5, 3/4, 0/6, 7/8, 9/13, 13/14`, default main; expects `"0/6,6/7,7/8,9/13,13/14"` | test_selection#002 | `ferrum/crates/ferrum-state/src/tests/test_selection.rs:22` | same 8 ranges via `SelectionRange::new`, explicit main index 0 (= cm6 default); same expected string | EQUIVALENT | state.EditorSelection.ranges.B1, state.EditorSelection.create.B4, state.EditorSelection.create.B6, state.EditorSelection.create.B7 | none |
| test-selection#03 | `CodeMirror6/state/test/test-selection.ts:28` | "merges adjacent point ranges when normalizing": `create([10/12, 12/12, 12/12, 10/10, 8/10])`; expects `"8/10,10/12"` | test_selection#003 | `ferrum/crates/ferrum-state/src/tests/test_selection.rs:41` | same 5 ranges, main 0; same expected string | EQUIVALENT | state.EditorSelection.ranges.B1, state.EditorSelection.create.B6, state.EditorSelection.create.B7, state.SelectionRange.empty.B1 | none |
| test-selection#04 | `CodeMirror6/state/test/test-selection.ts:39` | "preserves the direction of the last range when merging ranges": `create([range(0,2), range(10,1)])`; expects `"10/0"` | test_selection#004 | `ferrum/crates/ferrum-state/src/tests/test_selection.rs:57` | same 2 ranges via `new`, main 0; same expected string | EQUIVALENT | state.EditorSelection.create.B9 (direction only), state.EditorSelection.range.B4, state.SelectionRange.anchor.B1, state.SelectionRange.head.B1 | none. cm6 merged range has assoc 1, Ferrum 0 (R1 S7B); not asserted by either test. |
| test-charcategory#01 | `CodeMirror6/state/test/test-charcategory.ts:9` | "categorises into alphanumeric": `EditorState.create({extensions: []})`, `charCategorizer(0)("1")` and `("a")` → `CharCategory.Word` | test_charcategory#001 | `ferrum/crates/ferrum-state/src/tests/test_charcategory.rs:5` | `EditorState::create(EditorStateConfig::default())`, `char_categorizer(0)("1")`, `("a")` → `CharCategory::Word` | EQUIVALENT | state.CharCategory.B1, state.CharCategory.Word.B1, state.categorizer.B2 (single ASCII chars) | none |
| test-charcategory#02 | `CodeMirror6/state/test/test-charcategory.ts:15` | "categorises into whitespace": `charCategorizer(0)(" ")` → `Space` | test_charcategory#002 | `ferrum/crates/ferrum-state/src/tests/test_charcategory.rs:13` | `char_categorizer(0)(" ")` → `Space` | EQUIVALENT | state.CharCategory.B1, state.CharCategory.Space.B1, state.categorizer.B1 (single ASCII space) | none |
| test-charcategory#03 | `CodeMirror6/state/test/test-charcategory.ts:20` | "categorises into other": `charCategorizer(0)("/")`, `("<")` → `Other` | test_charcategory#003 | `ferrum/crates/ferrum-state/src/tests/test_charcategory.rs:20` | `char_categorizer(0)("/")`, `("<")` → `Other` | EQUIVALENT | state.CharCategory.B1, state.CharCategory.Other.B1, state.categorizer.B5 | none |

Coverage note (T7): the cm6 tests themselves only cover ASCII positions/characters. No cm6 or Ferrum test in these files covers `SelectionRange.map/extend/eq`, `EditorSelection.map/eq/asSingle/addRange/replaceRange/toJSON/fromJSON/single/cursor/undirectionalRange`, `assoc`, `bidiLevel`, `goalColumn`, `undirectional`, main-index tracking through merges (create.B8), `findClusterBreak`, `codePointAt`, `fromCodePoint`, `codePointSize`, `countColumn`, `findColumn`, multi-code-point or non-ASCII categorizer input, or `wordChars`. (`@marijn/find-cluster-break` has its own test file `test/test-cluster.js`, outside `@codemirror/state`.)

## Table 2: Ferrum tests with no cm6 counterpart (one row per test)

| Ferrum id | file:line | scenario | candidate verdict (FERRUM-ONLY / maps to cm6 id …) | behaviors covered |
|---|---|---|---|---|
| — | — | none: all 7 C3 Ferrum tests are mapped in Table 1 | — | — |

## Table 3: helpers (T5)

| helper | cm6 | Ferrum | parse/assert equivalence verdict with evidence |
|---|---|---|---|
| assertion `ist(a, b)` | `ist` package, imported `CodeMirror6/state/test/test-selection.ts:1`, `CodeMirror6/state/test/test-charcategory.ts:2`; two-argument form compares with `==` per the ist documentation (package not installed in `/workspace/CodeMirror6`, so not verified locally) | `assert_eq!` (strict `PartialEq`) in both Ferrum files | EQUIVALENT for these tests: all compared values are numbers, strings or enum members of the same type on both sides, where `==` and `PartialEq` agree. |
| state constructor `mk(...extensions)` | `CodeMirror6/state/test/test-charcategory.ts:4-6`: `EditorState.create({extensions})`, called with no extensions (empty doc, default selection) | inline `EditorState::create(EditorStateConfig::default())` `ferrum/crates/ferrum-state/src/tests/test_charcategory.rs:6`, `:14`, `:21` (doc `None`, selection `None`, extensions `None`) | EQUIVALENT: both build a default state with no extensions; the categorizer under test reads no facet in Ferrum (`state.rs:190`), and in cm6 `languageDataAt("wordChars", 0)` returns nothing without extensions. |
| range formatting `ranges.map(r => r.anchor + "/" + r.head).join(",")` | inline in each test-selection case (`CodeMirror6/state/test/test-selection.ts:11`, `:25`, `:36`, `:41`) | inline `ranges().iter().map(\|r\| format!("{}/{}", r.anchor, r.head)).collect::<Vec<_>>().join(",")` (`test_selection.rs:17`, `:36`, `:52`, `:65`) | EQUIVALENT: same fields, same separator and order; anchor/head values are identical integers (R1). Neither prints assoc, so the assoc difference (state.SelectionRange.assoc.B2) is invisible to both. |
| range constructor in test input | `EditorSelection.range(a, h)` (sets assoc ±1, bidi unset) | `SelectionRange::new(a, h)` (assoc 0) | EQUIVALENT for the asserted values (positions only); WEAKER as a constructor mapping in general (no assoc). |

## Table 4: counts

| | count |
|---|---|
| cm6 cases owned | 7 |
| Ferrum tests owned | 7 |
| EQUIVALENT | 7 |
| WEAKER | 0 |
| DIFFERENT | 0 |
| MISSING | 0 |
| NOT APPLICABLE | 0 |
| FERRUM-ONLY | 0 |
| Ferrum tests mapped in Table 1 | 7 of 7 |

T6: the Ferrum README/doc claims about test counts are audited in the main session (A4); C3 counts: 7 cm6 cases ↔ 7 Ferrum tests, one-to-one.

## Unfinished

none
