# ferrum-state audit: progress

Crate 1 of 4 (`1-parity-audit.md`). cm6 reference: `@codemirror/state` 6.7.6 (`CodeMirror6/state/package.json`). Ferrum: `crates/ferrum-state` at commit `fb99ca1`.

`docs/parity/decisions.md` does not exist yet: no foundational decision has been taken.

## Totals (declared before the audit)

| Inventory | Total | Source |
|---|---|---|
| A1 cm6 public API rows | 264 (39 top-level exports + members + config-object fields) | `raw/a1-cm6-exports.md` |
| cm6 source files | 13 | `CodeMirror6/state/src/*.ts` |
| A2 Ferrum public items | 391 (352 rustdoc items incl. 48 crate-root re-exports and 10 modules, + 19 std trait impls, + 20 facade re-exports) | `raw/a2-ferrum-pub.md` |
| T1 cm6 test cases | 172 (none loop-generated; no `webtest-*` files in this package) | `raw/t1-cm6-tests.md` |
| T2 Ferrum tests | 177 unit tests + 0 doctests | `raw/t2-ferrum-tests.md` |

Baseline: `cargo test -p ferrum-state` → 177 passed, 0 failed, 0 ignored (2026-10-21).

## Chunks

| Chunk | cm6 sources | Ferrum sources | cm6 rows | Ferrum rows | cm6 tests | Ferrum tests | State |
|---|---|---|---|---|---|---|---|
| C0 inventories | all | all | 264 | 391 | 172 | 177 | done |
| C1 text | `text.ts` | `text.rs` | 30 | 33 | 29 (`test-text`) | 32 (`test_text`) | done (raw/C1-*) |
| C2 change | `change.ts` | `change.rs` | 31 | 53 | 40 (`test-change`) | 42 (`test_change`) | done (raw/C2-*) |
| C3 selection + char | `selection.ts`, `char.ts`, `charcategory.ts`, `column.ts` | `selection.rs`, `char.rs`, `column.rs` | 41 | 34 | 7 (`test-selection`, `test-charcategory`) | 7 | done (raw/C3-*) |
| C4 facet/config | `facet.ts`, `extension.ts`, `config.ts` | `facet.rs`, `config.rs` | 38 | 58 | 22 (`test-facet`) | 22 (`test_facet`) | done (raw/C4-*) |
| C5 state/transaction | `state.ts`, `transaction.ts` | `state.rs`, `transaction.rs` | 77 | 76 | 27 (`test-state`) | 27 (`test_state`) | done (raw/C5-*) |
| C6 rangeset | `rangeset.ts` | `rangeset.rs` | 47 | 59 | 47 (`test-rangeset`) | 47 (`test_rangeset`) | done (raw/C6-*) |
| C7 main-session rows | `index.ts` | `lib.rs`, facade | 0 | 78 | 0 | 0 | done (in `api-matrix.md`) |
| A4 claims | — | READMEs, doc comments, facade, vision doc | — | — | — | — | done (raw/A4-claims.md) |
| A5 cross-crate, decision evidence, merge | — | — | — | — | — | — | done (`../_cross-crate.md`, `../decisions-evidence.md`, deliverables) |
| Verification (section 9) | — | — | — | — | — | — | done (`verification.md`; corrections applied) |

Done: 11 of 11 chunks. Crate is at the human gate.

Session log: 2026-10-21, chunks C1–C6 and A4 were interrupted mid-way by an account usage limit and then resumed from their saved state. Partial files present at that point: `raw/C1-api.md`, `raw/C1-behaviors.md`, `raw/C2-api.md`, `raw/C3-api.md`, `raw/C4-api.md`, `raw/C5-behaviors.md`. The verification pass must check that the files written before and after the resume are consistent with each other.

`dispositions.md` written (proposal): 262 `CORE`, 2 `DROP`.

## Open questions

(filled during merge)
- Q-merge-1 (resolved by verification check 4a; see `oracle.md`): `raw/A4-claims.md` reports 0 mismatches in 5000 random `EditorSelection.create` normalization cases, while `raw/C3-*` reports a main-index mismatch found by its oracle (`create([cursor(12), range(12,18), cursor(12)], 2)`: cm6 main 1, Ferrum main 0). The generators differ (A4 likely never generates duplicate equal ranges). Both results are correct; the A4 generator never builds the triggering input.

## Deliverables (merge, 2026-10-21)

`dispositions.md`, `api-matrix.md` (264 + 391 rows), `behaviors.md` (263 sections, 948 behavior rows), `tests-side-by-side.md` (172 cm6 cases; 177 Ferrum tests all mapped), `claims.md` (488 claims), `ferrum-only.md`, `placement.md`, `human-decisions.md`, `oracle.md` (harness sources in `oracle/`), `../_cross-crate.md` (XC-001…XC-039), `../decisions-evidence.md`. Main-session decisions on the 107 `candidate:` rows are in the `main-session decision` column of `api-matrix.md`.

Unaudited items: none (every A1/A2 row has a status; every T1 case has a verdict).
