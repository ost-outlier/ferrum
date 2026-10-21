# ferrum-state differential oracle (T8)

Pure-state items were run on the same generated inputs in cm6 (Node 22.23.3 + `tsx`, `@codemirror/state` 6.7.6 sources) and in Ferrum (scratch crates depending on `crates/ferrum-state` by path). PRNG: mulberry32 everywhere, seeded; the Rust side replays inputs recorded by the cm6 generator, so both sides see identical inputs. Main runs use ASCII-only input (positions coincide in UTF-16 and code points); separate non-ASCII samples document D1/D2.

## Harness

Sources are archived under `oracle/` (no generated data; data is reproducible from the seeds):

- `oracle/cm6-probes/probe-C<n>/`: cm6 generators, cm6 runs and comparators (`oracle.ts`, `gen.ts`, `compare.ts`, …).
- `oracle/ferrum-repro/repro-C<n>/`: Ferrum side (`src/bin/oracle.rs` etc.); standalone crates with an empty `[workspace]` table, `ferrum-state` by path, `serde_json` in C2/C6 only.
- `oracle/tools/`: inventory scripts (`cm6-exports.cjs`, `cm6-tests.cjs`, `ferrum-pub.py`) and C6 diff tools.

Setup used for the audit (not part of the repository):

```sh
mkdir cm6 && cd cm6 && npm init -y && npm install @marijn/find-cluster-break@1.0.4 typescript@5.6.3
cp -r /workspace/CodeMirror6/state/src ./src           # cm6 sources, imported as "../src/index.ts"
cp -r <docs>/state/oracle/cm6-probes/probe-C2 .         # one probe dir per chunk
tsx probe-C2/oracle.ts gen 3266 5000 ascii cases.json   # example; arguments per script header
cd <docs>/state/oracle/ferrum-repro/repro-C2 && cargo build --release --bin oracle
```

Exact command lines, file names and arguments are in each chunk's `raw/C<n>-notes.md` `## Oracle`.

## Results

| chunk | items compared | seed | cases | result | minimal mismatching case |
|---|---|---|---|---|---|
| C1 text | `Text.of`, `replace`, `length`, `lines`, `toString`, line texts, `lineAt`, `line`, `sliceString`, `slice` (8 random replaces per case, 6 probes each) | 20261021 | 300 ASCII (81,600 checks) | **0 mismatches** | — |
| C1 text | same, BMP non-ASCII (é, 中) | 11 | 40 (10,880 checks) | 0 mismatches | — |
| C1 text | same, emoji (U+1F600) | 11 | 40 | every category mismatches (D1) | `Text.of(["a😀b"]).length` cm6 4, Ferrum 3 (C1 R9) |
| C1 text | same, U+2028 / `\r` in line content | 11 | 40 each | strings and lengths agree; `lines`, `line`, `lineAt`, `slice.lines`, line texts mismatch (D2) | U+2028: lines cm6 2, Ferrum 3 (C1 R5); `\r`: `Text.of(["a\r","b"]).line(1)` cm6 `{0,2,"a\r"}`, Ferrum `(0,1,"a")` (C1 P2/R6b) |
| C2 change | `ChangeSet.of` (flat lists), `apply`, `compose`, `composeDesc`, `map`/`mapDesc` (before false/true), convergence, `invert`, `invertedDesc`, `iterChanges` (both `individual`), `iterGaps`, `iterChangedRanges`, `length`, `newLength`, `empty`, `mapPos` (every position, assoc ±1, 4 modes) | 3266 / 777 | 5,000 + 20,000 ASCII (~2.6M `mapPos`) | **0 mismatches** | — |
| C2 change | same, non-ASCII inserts (é, 😀, `\r\n`, `\r`, U+2028) | 4242 | 500 | 383 mismatch: every case whose inserts contain 😀 or `\r`; é / U+2028 only: 0 | doc `"dacg"`, insert `"😀bc"` at 1: cm6 `1 0:4 3`, Ferrum `1 0:3 3` |
| C3 char | `findClusterBreak`, `countColumn`, `findColumn` | 0xC3C3 | 34 table + 2,000 ASCII + 500 non-ASCII | ASCII: 105 mismatches, all containing CRLF; non-ASCII: 122, all in the classes CRLF, ZWJ next to letters, Hangul jamo, Indic conjuncts, Prepend, tag flags, control + mark (grapheme policy) | `findClusterBreak("\r\n", 0)` cm6 1, Ferrum 2 (C3 R2) |
| C3 selection | `EditorSelection.create` normalization, `map`, per-range assoc | 0x5E1C | 5,000 | anchor/head/main: 1 mismatch; `map`: 1 (same case); assoc: 4,380 mismatch (cm6 ±1 on non-empty ranges, Ferrum 0) | `create([cursor(12), range(12,18), cursor(12)], 2)`: cm6 main 1, Ferrum main 0 (C3 R4) |
| C3 categorizer | `charCategorizer(0)` vs `categorize_char` on every scalar value (Unicode 17.0 on both sides) | exhaustive | 1,111,998 | 2 mismatches | U+0085 (cm6 Other, Ferrum Space), U+FEFF (cm6 Space, Ferrum Other) |
| C4 facet | extension flattening: precedence buckets, nesting, overrides | 196 | 2,000 (no shared sub-extensions) | **0 mismatches** | — |
| C4 facet | same with shared sub-extensions (same object reused) | 197 | 2,000 | 343 mismatch (no identity dedup) | `(g (d 0 v0) (r 0))`: cm6 `0`, Ferrum `0,0` |
| C5 state | `replaceSelection`, `changeByRange` (4 callback shapes), multi-range selections | 3285 | 3,000 ASCII | **0 mismatches** | — |
| C5 state | `replaceSelection`, `changeByRange` (unit-independent callbacks), positions converted to code points | 777 | 500 non-ASCII | 0 mismatches | — |
| C6 rangeset | `size` | 51966 / 1234567 | 5,960 + 3,043 sets | 0 mismatches | — |
| C6 rangeset | `iter` order | same | same | 1,078 + 281, order only (ties on from/startSide) | C6 R1 |
| C6 rangeset | `between` | same | 3,000 + 1,500 | 0 with small sides; 515 with view-like sides (±1e6 sentinels) | C6 R2 |
| C6 rangeset | `update` | same | 3,000 + 1,500 | 977 + 259, order only; content identical | C6 R1 |
| C6 rangeset | `map` | same | 3,000 + 1,500 | 1,620 + 852 (of which 477 + 81 order only) | C6 R3 (i8 truncation), R4, R5, R32 |
| C6 rangeset | `spans` (calls, open counts, return value) | same | 3,000 + 1,500 | 194 + 115 | C6 R11/R12 (open counts), R18 (tie order), R19 (lost point) |
| C6 rangeset | `compare` (isolated from `map` differences: `compareN`) | same | 3,000 + 1,500 | 501 + 493 | C6 R13 (shared chunks), R14, R15 (change at doc end), R16 |

Additionally: the cm6 test cases of `test-rangeset.ts` were replayed against Ferrum with cm6's exact inputs and expectations (`repro-C6/src/bin/cm6tests.rs`): 46 of 47 pass; #03 (chunk reuse by `update`) fails (C6 R33).

## Not covered by any oracle

Error paths (covered by targeted repros in each `raw/C<n>-api.md`), JSON (no Ferrum counterpart), iterators, `touchesRange`, `children`, `ChangeSpec` forms Ferrum cannot express (nested arrays, `ChangeSet` inside lists, `lineSep`), dynamic facet recomputation and value identity (C4 R5–R12), transaction filters/extenders (absent in Ferrum), effect mapping (C5 R17), out-of-range selections (C5 R1).

## Note on conflicting sample results

`raw/A4-claims.md` reports 0 mismatches in 5,000 random `EditorSelection.create` cases, while the C3 selection oracle above finds 1 in 5,000. The C3 mismatch needs two equal ranges in the input (main index found by value equality instead of identity); the A4 generator is the one in `oracle/ferrum-repro/repro-A4/src/bin/selnorm.rs`. Resolved in `verification.md` (check 4a): both results are correct. The defect needs the main range to have an equal twin earlier in sort order, separated by a non-empty range with the same `from`; the A4 generator never builds that shape. An exhaustive run over three-range inputs finds 20 mismatches in 46,875 cases, all in the main index only. `claims.md` state.claim.019, .077, .132 and .483 were corrected to `false` accordingly.
