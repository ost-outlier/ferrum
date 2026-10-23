# C6 notes (rangeset)

> What: chunk C6 decision evidence, cross-crate facts, oracle, open questions. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `docs/parity/state/raw/C6-api.md` § Reproductions.

## Decision evidence

**D1 (position unit).** Items affected: every positional parameter/return in C6. That covers `Range.from/to` (state.cm6.178/179), `between`, `iter`, `update` (`filterFrom`/`filterTo`), `map`, `compare`, `spans`, `RangeSetBuilder.add`, and the comparator/iterator callbacks (state.cm6.183–190, 203, 206–211).
- cm6: `number`, which callers fill with UTF-16 offsets (`mirrors/cm6/state/src/rangeset.ts:46-48`). Ferrum: `usize`, which callers fill with code-point offsets (`crates/ferrum-state/src/rangeset.rs:30-31`).
- `rangeset.ts`/`rangeset.rs` never read text. Under any D1 option the module's results are the same for the same integer inputs; the difference lies in which integers callers produce. Example: a mark over "😀b" in doc "a😀b" is (1,4) in cm6 (UTF-16) and (1,3) in Ferrum (code points). Under UTF-8 bytes it would be (1,6). Mapping it through an insertion at 0 shifts both ends by the inserted length in the same unit. No C6 test uses non-ASCII text (positions only). The oracle is therefore unit-free and needed no non-ASCII sample.
- Implication per option: nothing changes inside RangeSet for any option. With UTF-16 or UTF-8 bytes, ranges must never be created inside a surrogate pair or a multi-byte sequence; with code points every integer is a valid boundary. RangeSet does not validate either way.

**Not a D-decision but foundational for the view: side values.** cm6 sides are `number`; the view uses ±1e8…±6e8 (`mirrors/cm6/view/src/decoration.ts:171-185`). Ferrum stores `i32` (enough), but `map` casts sides to `i8` (`crates/ferrum-state/src/rangeset.rs:247`, `:250`, `:259`, `:260`) because `ChangeDesc::map_pos` takes `assoc: i8` (`crates/ferrum-state/src/change.rs:149`). The R31 table shows the sign flips (−1e8 → 0 → maps like `assoc > 0`; 5e8−1 → −1). Two sentinels use ±1e6 instead of ±1e9 (`rangeset.rs:79-80`, `:855-856`). This is relevant to whoever owns `ChangeDesc::map_pos`'s signature (chunk C2).

## Cross-crate facts

1. **`boundChange` missing** (state.cm6.208). cm6 evidence: `mirrors/cm6/view/src/docview.ts:637`, `:650` (DecorationComparator/WrapperComparator define it), used through `RangeSet.compare` `mirrors/cm6/view/src/docview.ts:642`, `:655`. Searched `bound_change`, `boundChange`, `bound` in `crates/*`: absent. `ferrum-view` will need it.
2. **`compare` with `minPointSize`** (state.cm6.188 B9). cm6: `mirrors/cm6/view/src/heightmap.ts:722` (`RangeSet.compare(a, b, diff, comp, 0)`). Ferrum `compare` has no such parameter (`crates/ferrum-state/src/rangeset.rs:896-923`). `ferrum-view` has no call to `RangeSet::compare` (grep `RangeSet::compare` in `crates/ferrum-view/src`: none).
3. **`compare` takes `&ChangeSet`** (state.cm6.188 B1). cm6 callers pass a `ChangeSet` (`mirrors/cm6/view/src/docview.ts:640`, `mirrors/cm6/view/src/heightmap.ts:720`), so they are compatible. Callers with only a `ChangeDesc` cannot use it.
4. **Static `RangeSet.iter` with `rank`/`goto`** (state.cm6.187, 196, 200). cm6: `mirrors/cm6/view/src/gutter.ts:194`, `:286`; `mirrors/cm6/view/src/buildtile.ts:384`, `:193` (`goto`), `:199` (`cur.rank`). Ferrum: private `HeapCursor` only (`crates/ferrum-state/src/rangeset.rs:542`); searched `RangeCursor`, `iter_sets`, `pub fn goto`: absent.
5. **`RangeSet.eq`** (state.cm6.189). cm6: `mirrors/cm6/view/src/gutter.ts:227`, `:372`. Ferrum: absent (searched `fn eq(`, `sets_eq`).
6. **`RangeSet.join`** (state.cm6.192). cm6: `mirrors/cm6/view/src/docview.ts:505`, `mirrors/cm6/view/src/viewstate.ts:686`. Ferrum: absent (searched `fn join`: only the unrelated `crates/ferrum-view/src/heightmap.rs:157`).
7. **`spans` `openStart`/return value** (state.cm6.190 B4, B5, B12). cm6 relies on them: `mirrors/cm6/view/src/buildtile.ts:516` (uses the return value `openEnd`). Ferrum's counts differ (R11, R12). Ferrum consumer: `crates/ferrum-view/src/heightmap.rs:1436` (`RangeSet::spans(…, 0)`, `NodeBuilder` impl at `:1441`).
8. **Decoration sides in `ferrum-view` hit the i8 truncation and ±1e6 sentinels.** `crates/ferrum-view/src/decoration.rs:22-34` defines `SIDE_*` = ±1e8…±6e8, used by `impl RangeValue for Decoration` (`:110-150`). Consequences, from the state-level repros R2/R3 with the same numbers:
   - `RangeSet::map` maps widgets with side ≤ 0 (−1e8 → `assoc` 0) after an insertion at their position.
   - Mark ends (−6e8 → 0) become inclusive at insertions.
   - `between` (used by `crates/ferrum-view/src/view.rs:117`) misses widgets/points that end at `from` with `endSide < -1e6` or start at `to` with `startSide > 1e6`.
   - `crates/ferrum-language/src/fold.rs:52` maps fold ranges (`Decoration::replace_spec`) through `folded.map(tr.changes().desc())`. Their end side −6e8 truncates to 0, so a fold extends over text inserted at its end (cm6 `mirrors/cm6/language/src/fold.ts` keeps it exclusive). (Ferrum-view also uses `SIDE_NON_INC_START`/`END` without cm6's −1/+1 adjustment, `mirrors/cm6/view/src/decoration.ts:254-255`; that is a C-view audit matter, recorded here only as an observation.)
9. **`RangeValue.range` hook** (state.cm6.176). cm6 view overrides it for validation (`mirrors/cm6/view/src/decoration.ts:297-300`, `:314-317`). Ferrum has no hook; the ferrum-view constructors `Decoration::mark`/`widget` (`crates/ferrum-view/src/decoration.rs:197-225`) build `Range::new` directly.
10. **`RangeSet.update` sort/identity** used by language folding. `crates/ferrum-language/src/fold.rs:63-81` passes `sort: true` and ignores errors (`if let Ok`). cm6 `mirrors/cm6/language/src/fold.ts` relies on `update`. Ferrum always rebuilds (O(n log n) per fold effect).

## Oracle

- Files (archived under `docs/parity/state/oracle/`):
  - generator + cm6 run: `cm6/probe-C6/oracle.ts`;
  - Ferrum run: `repro-C6/src/bin/oracle.rs` (release build);
  - diff tools: `tools/c6_oracle_diff.py`, `tools/c6_map_classify.py`, `tools/c6_compare_merge.py`;
  - data: `oracle-C6/cases-{small,big}.txt`, `oracle-C6/cm6-{small,big}.txt`, `oracle-C6/ferrum-{small,big}.txt`.
- PRNG: mulberry32. Seeds: 51966 (`small`, 3000 cases, sides ∈ {−2…2}) and 1234567 (`big`, 1500 cases, view-like sides {−6e8+1, −6e8, −4e8, −2e8, −1e8, −1, 0, 1, 1e8+1, 3e8, 5e8−1, 5e8}).
- Inputs per case:
  - doc length 10–49 and 1–3 sets of up to 30/10 random ranges, sorted with cm6 `cmpRange`;
  - 25% empty ranges, 85% of those points; 25% of non-empty ranges points; random sides and mapMode;
  - a random `ChangeSet` (≤ 3 specs; its sections passed to Ferrum as `ChangeDesc::create`/`ChangeSet::create_set`);
  - a random sorted `add` list, a `filter` (value number mod k ≠ 0) with random `filterFrom`/`filterTo`, a random `between` window, and a `spans` window with `minPointSize` ∈ {−1, −1, 0, 2}.
- Ops compared:
  - `iter` and `size` of each set;
  - `between` (as a sorted multiset);
  - `map` of set 0;
  - `update` of set 0;
  - `spans` over all sets (exact calls incl. open counts and return value);
  - `compare(old, new)`, where `new` = mapped sets with set 0 updated then mapped (cm6 with minPointSize −1);
  - `compareN`: the same, but Ferrum builds the new sets from cm6's own mapped/updated ranges, which isolates `compare` from `map` differences.
- No cm6 `RangeSet.of` threw on the generated inputs (0 skipped).

| op | small: cases / mismatches (order-only) | big: cases / mismatches (order-only) | cause (minimal case) |
|---|---|---|---|
| size | 5960 / 0 | 3043 / 0 | — |
| iter | 5960 / 1078 (1078) | 3043 / 281 (281) | R1 (order on from/startSide ties) |
| between | 3000 / 0 | 1500 / 515 | R2 (±1e6 sentinels) |
| update | 3000 / 977 (977) | 1500 / 259 (259) | order only (R1); content identical |
| map | 3000 / 1620 (477) | 1500 / 852 (81) | small: R5 (1 928 surviving tokens), R4 (oracle case 688, reduced to a single range (8,9) s−2/e1 under `[7,-1,5,1,1,-1]`), R32 (case 190); big: additionally R3 (i8 truncation) |
| spans | 3000 / 194 | 1500 / 115 | small: 181 differ only in open counts (R11/R12); 13 others: heap-tie order of `active`/same-place points (R18, e.g. case 628 `n13179`/`n13176` swapped), activeForPoint/forward effects (case 277, R19 class) |
| compare | 3000 / 1183 | 1500 / 838 | mixes the map differences with the compare differences |
| compareN | 3000 / 501 (163 still differ after merging adjacent ranges like the view's `addRange`) | 1500 / 493 (229 after merging) | R15 (141 small mismatches have a change at the doc end), R13 (shared chunks), R14, R16, and order-sensitive `same_values` with a different layer structure (e.g. case 25: `r(12,13,[n497,n500],[n500,n497])` reported by Ferrum only) |

Minimal mismatching case per op: iter R1, between R2, map R3/R4/R5/R32, spans R11/R12/R18/R19, compare R13–R16. The cm6 test replay (`repro-C6/src/bin/cm6tests.rs`) passes 46/47 (R33).

## Open questions

1. `RangeSet.eq` in cm6 6.7.6 compares only point ranges (`SpanCursor(…, 0)`, `mirrors/cm6/state/src/rangeset.ts:399`; R30). A port must decide whether to reproduce this exactly (it is current cm6 behavior, used by the gutter with marker sets) — HUMAN DECISION when implementing.
2. Should Ferrum expose `Chunk`, `CHUNK_SIZE`, the `RangeSet` internals, `is_empty`, `length` and `add_range` publicly (cm6 keeps them internal)? Candidates are listed as FERRUM-ONLY in `docs/parity/state/raw/C6-api.md`; no verifiable reason was found for making them public other than test access (`set.chunk.len()`).
3. Structural sharing (cm6 `update`/`map` reuse chunks; `compare` skips shared chunks). Without it the results of `compare` differ (R13), not only performance. Is sharing a required behavior for Ferrum (it is observable through `compare`'s reported `active` arrays and split points)?
4. `RangeValue::eq_val` default: cm6 identity vs Ferrum `PartialEq`. In Rust there is no object identity for owned clones; options: keep `PartialEq`, require an explicit `eq_val`, or wrap values in `Arc` and compare pointers. HUMAN DECISION.
5. `RangeSetBuilder::add` is non-strict in Ferrum and silently layers misordered input (R10). cm6 throws. Options: panic, return `Result`, or keep lenient. No verifiable reason found for the current choice.

## Unfinished

none
