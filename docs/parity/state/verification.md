# ferrum-state verification pass

Verifier: fresh-context subagent (did not write any audited row). Date: 2026-10-21. Sample: `scratchpad/prompts/sample.txt` (seed 20261021): 66 random non-`PORTED` API rows (10% of 655), all 38 `PORTED` API rows, 20 random test-case rows. Extension sample: 66 further non-`PORTED` API rows drawn from the 551 not yet sampled (Python `random.seed(20261022)`, `random.sample`).

Scratch root below: `SCR = /tmp/claude-1000/-workspace/8fb87903-f955-477c-a67c-0747517f569e/scratchpad`. Verifier scripts: `SCR/cm6/probe-V/*.ts` (run `cd SCR/cm6 && npx tsx probe-V/<x>.ts`), `SCR/repro-V/src/bin/*.rs` (run `cd SCR/repro-V && cargo run -q --offline --bin <x>`). Re-runs of the chunk reproductions used the chunk programs unchanged (`SCR/cm6/probe-C<n>/…`, `SCR/repro-C<n>`), with a separate `CARGO_TARGET_DIR` under `SCR/verif/`.

## Summary

- **Rows checked:** 104 API rows (sample) + 66 API rows (extension) + 20 test rows = 190.
- **Mismatches:** 11 rows/entries in 5 findings.
  - medium (status or verdict wrong): 2 findings: `api-matrix.md` state.fe.363 (`PORTED` without a test); `claims.md` state.claim.132 and state.claim.483 (verdict `true` contradicted by the C3 R4 main-index case).
  - low (justification text not backed by evidence): 1 finding on 5 sampled rows: state.fe.079, state.fe.102, state.fe.105, state.fe.135, state.fe.143 (boilerplate "the row's repro shows invariants can be broken from outside" without a repro; the same sentence appears in 55 rows).
  - minor (citation detail): 2 findings: state.fe.019 (incomplete usage list); `_cross-crate.md` XC-032 (wrong repro id), XC-017 (cm6 line range off by one).
- **Totals recount:** all totals confirmed (A1 264, A2 391, T1 172, T2 177, `cargo test -p ferrum-state` 177 passed; matrix 264 + 391 rows, no duplicate ids, `## Counts` matches the rows; Table 1 172 rows, every Ferrum test id mapped; Table 4 and `progress.md` consistent).
- **Targeted checks:** 4a explained (both sides reproduced; A4's generator never builds the triggering shape); 4b re-measured (40 000 chars: Ferrum release 56.4 s, debug 524 s, cm6 0.35 ms); 4c confirmed; 4d all three confirmed; 4e verified from the `ist` 1.1.7 source (`==` default); 4f 7 D1 rows re-run, all confirmed; 4g 10 entries checked, 2 minor citation errors.
- **Extension samples:** one (seed 20261022, 66 rows): clean.
- **Clean?** The first sample is **not** clean (fe.363 status, boilerplate justification on 5 rows, 2 minor citations). The extension sample is **clean**. Per brief section 9, the mismatches above still have to be fixed in the deliverables by the main session; this pass did not edit them.

## Mismatches

| # | deliverable + row id | deliverable says | verifier found | evidence / repro | severity | suggested correction |
|---|---|---|---|---|---|---|
| 1 | `api-matrix.md` state.fe.363 `impl Display for ChangeSet` (`change.rs:541`) | `PORTED`; "Prints sections only (no inserted text), same as cm6." | Behavior equals cm6, but **no Ferrum test exercises `ChangeSet`'s `Display`**: every test prints `set.desc().to_string()` (the `ChangeDesc` impl, state.fe.358) or a `Text`. Searched `crates/*` for `changes().to_string`, `set.to_string()`, `cs.to_string()`, `format!("{}", set|cs|changes)`: no hit. The same row family uses `PORTED, DEVIATION` for "untested directly" (state.fe.037 `ChangeSet::is_empty`). | R10 re-run: cm6 `String(set)` → `"1 1:0 2 0:2 2"` \| Ferrum `Display` → `"1 1:0 2 0:2 2"` (equal). Test search above. | medium | `PORTED, DEVIATION` (untested), or keep `PORTED` only if a test is cited. Ferrum-row counts would become `PORTED` 22, `PORTED, DEVIATION` 155. |
| 2 | `api-matrix.md` state.fe.079 `AnyFacet::default_erased`, state.fe.102 `StateField as AnyStateField::init`, state.fe.105 `Compartment.id`, state.fe.135 `Chunk.max_point`, state.fe.143 `RangeSet.next_layer` (main-session decision column) | "HUMAN DECISION: exposes a cm6 `@internal`/non-exported mechanism as public API with no verifiable reason, and **the row's repro shows invariants can be broken from outside**." | None of the five rows has a repro (notes: fe.079 "Never called…", fe.102 "Calls `init_fn` directly…", fe.105 "Tests use it…", fe.135 "`isize`, −1…", fe.143 "`Option<Arc<…>>`…"; the raw C4/C6 rows have none either). For fe.079 and fe.102 the claim is false: both are read-only. For fe.105 it is true but unproven by the row. The status `HUMAN DECISION` itself is fine. The sentence appears verbatim in 55 matrix rows; others without a supporting repro include fe.076, fe.081, fe.082, fe.088, fe.093, fe.101, fe.119, fe.269 (not individually verified). | `SCR/repro-V/src/bin/hd.rs`: `default_erased(&num)` → `Some([])` (just the default); `AnyStateField::init(&f, &st)` → `Some(3)`, `st.field(&f)` unchanged `Some(3)`; fe.105: `b.id = a.id` → `a == b` true, and `b.reconfigure(num.of(9))` replaces the content of `a.of(num.of(1))` → `[9]`. | low | Per row, either cite a repro (fe.105: the one above) or drop the "row's repro shows…" clause (fe.079, fe.102, and every other row without one). |
| 3 | `claims.md` (and `raw/A4-claims.md`) state.claim.132 ("Canonical selection normalization guarantees identical, deterministic behavior…", `ferrum/README.md:29`) and state.claim.483 (2f15006 "enforce 1:1 parity with CodeMirror 6 selection normalization") | `true`, evidence "R5: 0/5000 mismatches" | Normalization is not identical: the main index differs when the main range has an equal twin earlier in sort order (C3 R4), and merged ranges get assoc 0 where cm6 gives ±1 (C3 R1 S7A/S7B). A4's generator never produced the triggering shape (see 4a). An exhaustive run finds 20 main-index mismatches in 46,875 three-range cases. | 4a below: `create([cursor(12), range(12,18), cursor(12)], 2)` → cm6 `12/12,12/18 main=1` \| Ferrum `main=0`; exhaustive 46,875 cases → 20 mismatches, all main-index only. | medium | Verdict `false` for .132 and .483. For .019, .077 and .287 (which concern the sorted/non-overlapping invariant, still true) add the C3 R4 exception to the evidence. Close `progress.md` Q-merge-1 with the 4a explanation. |
| 4 | `api-matrix.md` state.fe.019 `ChangeDesc::from_string` | "Used only by `tests/test_change.rs:5`." | Also called at `tests/test_change.rs:603` (test_change#042). Still test-only, so the conclusion holds. | `grep -rn from_string crates/` → `change.rs:198`, `test_change.rs:5`, `test_change.rs:603`. | minor | "Used only by tests (`test_change.rs:5`, `:603`)." |
| 5 | `_cross-crate.md` XC-032; XC-017 | XC-032: "calling it inside a field update overflows the stack, C5 R4". XC-017: cm6 evidence `view/src/decoration.ts:297-300,314-317`. | XC-032: C5 R4 is the two-accesses identity repro (`(false, 2, 1)`). The stack overflow is C5 R33c (`repro-C5/src/bin/reentrant.rs`). XC-017: the `range()` overrides are at `decoration.ts:298-300` and `:317-319`. | `cargo run --bin reentrant` → `thread 'main' has overflowed its stack`; `sed -n 297,320p view/src/decoration.ts`. | minor | XC-032 cite "C5 R33c"; XC-017 cite `:298-300, :317-319`. |

## Rows checked and found correct

For each row: the cm6 counterpart and `file:line`, the Ferrum counterpart and `file:line`, the status under brief section 4, and the row's reproduction were re-derived. "R…" ids refer to the row's raw file (`raw/C<n>-api.md ## Reproductions`). "both" means the same output on both sides. For `MISSING` rows the workspace search was redone over `/workspace/ferrum/crates/*` with the patterns listed.

### Sample, C1 (text)
- state.cm6.224 `TextIterator` MISSING: grep `TextIterator|RawTextCursor|LineCursor|iter_range|iterRange|iter_lines|fn iter\b` → only `rangeset.rs:191`. R23 cm6 `[["ab",false],["\n",true],["\n",true],["c",false]]` (re-run).
- state.cm6.236 `Text.slice` PORTED, DEVIATION: `text.rs:170` has no default `to`. R16b both `0`, R16d cm6 `["c","de"]`, R16e cm6 `[4,2,["bc","d"]]` \| Ferrum `(4, 2, "bc\nd")`.
- state.cm6.240 `Text.iterRange` MISSING: same grep, no hit. R25 cm6 `[["b",false],["\n",true],["cd",false],["\n",true],["e",false]]`, R25b reversed (re-run).
- state.fe.245 `Text::byte_to_char` BLOCKED: D1: `text.rs:80` clamps. R20d Ferrum `(5, 6, 1, 2, 3, 6)`. `ferrum-view-gpui` helpers are at `editor_view.rs:36,45`, re-exported at `lib.rs:11` (citation accepted).
- state.fe.354 `Text as Display` PORTED, DEVIATION: `text.rs:200` writes the rope raw. R19 Ferrum `"ab\r\n\r\nc"` vs cm6 toString `"ab\n\nc"`.

### Sample, C2 (change)
- state.cm6.147 `ChangeSet^fromJSON` MISSING: grep `from_json|fromjson|to_json|tojson|serde|Serialize` → only test names and `ferrum-language/src/fold.rs` (fold serde, unrelated). R6b cm6 `["1 1:0 2 0:3 2 2:2 2","023a\nb45xy89",…]`; R2j cm6 `RangeError: Invalid JSON representation of ChangeSet`.
- state.fe.019 `ChangeDesc::from_string` HUMAN DECISION: status fine (see mismatch 4 for the citation). R2q Ferrum panic `Invalid length in spec: ParseIntError { kind: InvalidDigit }`; R2r `"-5"`.
- state.fe.022 `ChangeSpec.to` PORTED, DEVIATION: `change.rs:248` not optional; `insert()` sets `to = at` (`:261-267`).
- state.fe.026 `ChangeSpec::replace` FERRUM-ONLY: `change.rs:269`, plain constructor.
- state.fe.031 `Changes::to_changeset` PORTED, DEVIATION: `change.rs:291`; callers `state.rs:215,224,260` (grep). R1d2 Ferrum `("5", 5)` accepted vs cm6 `RangeError: Mismatched change set length (got 5, expected 10)`.
- state.fe.037 `ChangeSet::is_empty` PORTED, DEVIATION: `change.rs:353`; no direct test (grep `is_empty()` in tests: only rangeset).
- state.cm6.149 `ChangeDesc.length` PORTED: formula `change.ts:34-38` = `change.rs:46-54`; indirect proof by the `apply` length guard (`change.rs:380`) in test_change#031–#041, accepted. R3f both `[10,11]`/`(10, 11)`.
- state.cm6.150 `ChangeDesc.newLength` PORTED: `change.ts:41-48` = `change.rs:57-70`; used as doc length in test_change#038–#040 (`of_specs(…, a.new_length())` then compose/apply). R3f both.
- state.cm6.152 `ChangeDesc.iterGaps` PORTED: `change.ts:57-66` = `change.rs:79-95`; test_change#036 asserts `[(0,0,4),(4,6,2),(8,8,2)]` = cm6 test-change#34 values. R4f both `[[0,0,2],[6,4,4]]`. behaviors B1–B2 complete.
- state.cm6.161 `MapMode` PORTED: `change.ts:6-16` = `change.rs:6-15`, same order. Modes asserted with `None` in test_change#023/#027 (same tuples as cm6 test-change#21/#25 read side by side).
- state.cm6.162 `MapMode.Simple` PORTED: `change.rs:167` guard; test_change#022–#029.
- state.cm6.163 `MapMode.TrackDel` PORTED: `"4 4:0 4"` pos 5 → `None`, 4 and 8 → `Some(4)` (test_change#023 lines 177, 181, 186).
- state.cm6.164 `MapMode.TrackBefore` PORTED: pos 8 → `None`, 4 → `Some(4)` (test_change#023).
- state.cm6.165 `MapMode.TrackAfter` PORTED: pos 4 → `None`, 8 → `Some(4)` (test_change#023).
- state.fe.002–state.fe.006 `MapMode` and variants PORTED: same as cm6.161–165.
- state.fe.010 `ChangeDesc::length`, state.fe.011 `ChangeDesc::new_length`, state.fe.035 `ChangeSet::length`, state.fe.036 `ChangeSet::new_length` PORTED: delegation `change.rs:343-350`; indirect test as above.
- state.fe.013 `ChangeDesc::iter_gaps`, state.fe.041 `ChangeSet::iter_gaps` PORTED: `change.rs:374-376` delegates; test_change#036.
- state.fe.021 `ChangeSpec.from` PORTED: `change.rs:247`; range guard `change.rs:513-518`; R2c (`from:-1`) unrepresentable.
- state.fe.029 `Changes::Single` PORTED: R1c both `"3 2:1 5"`.
- state.fe.358 `impl Display for ChangeDesc` PORTED: same format as `change.ts:157-164`; asserted by every `comp`/`over`/`under` helper (`test_change.rs:13,18,23`). R10 both `"1 1:0 2 0:2 2"`.

### Sample, C3 (selection, char)
- state.cm6.065 `EditorSelection.eq` PORTED, DEVIATION: derived `PartialEq` `selection.rs:116`; cm6 `selection.ts:136-142` (`includeAssoc`). S58 cm6 `true` \| Ferrum `false` (re-run).
- state.cm6.078 `SelectionRange.from` PORTED, RUST-IDIOMATIC: `selection.rs:41` `min`; S11 (undirectional `from 5,to 2`) has no Ferrum counterpart, recorded. test_selection#001 asserts `from() == 2`.
- state.cm6.090 `SelectionRange.toJSON` MISSING: grep as cm6.147. S61 cm6 `{"anchor":4,"head":4}`.
- state.fe.055 `find_word_break` PLACEMENT DEVIATION: `char.rs:70`; cm6 `view/src/cursor.ts:102-110` (`if (cat == CharCategory.Space) cat = nextCat` at :107), `editorview.ts:694`, `commands.ts:93`; usages `commands.rs:182,204,225,246`, `editor_view.rs:235-236`, `tests.rs:151-152` (grep). W01 Ferrum `4`.
- state.fe.178 `SelectionRange.head`, state.fe.184 `to()`, state.fe.185 `is_empty()` PORTED, RUST-IDIOMATIC: `selection.rs:11,47,53`; same values as cm6 getters (`selection.ts:44,29,47`).
- state.fe.188 `SelectionRange::map` PORTED, DEVIATION: `selection.rs:70-95` vs `selection.ts:73-81`: same position logic, keeps inversion; mandatory `assoc`; untested.
- state.cm6.066 `EditorSelection.main` PORTED: `selection.ts:147` = `selection.rs:207-209`; test_selection#001 asserts from/to/anchor/head of main (2, 3, 3, 2), same as test-selection#01.
- state.cm6.166–169 `CharCategory` and members PORTED: `charcategory.ts:4-11` = `char.rs:5-9`; test_charcategory#001–#003 = test-charcategory#01–#03 (same inputs and expected values).
- state.fe.049–052 `CharCategory` (Ferrum rows) PORTED: as above.
- state.fe.194 `EditorSelection::main()` PORTED: as cm6.066.

### Sample, C4 (facet, config)
- state.cm6.053 `Prec.high` PORTED, RUST-IDIOMATIC: `facet.rs:25`; test_facet#005 asserts `d,c,f,a,b,g,e` (= test-facet#05).
- state.cm6.057 `Compartment` PORTED, DEVIATION: `facet.rs:218-240`. R14 cm6 `0,20 | 2 | 10,2` \| Ferrum `0,20 | 2 | 0,2` (re-run).
- state.cm6.058 `Compartment.of` PORTED, DEVIATION: R13 cm6 `RangeError: Duplicate use of compartment in extensions` \| Ferrum `1,1`; bin `nested` → `thread 'main' has overflowed its stack` (re-run).
- state.cm6.254 `StateField.define config.update` PORTED, DEVIATION: R60 cm6 `2` \| Ferrum bin `trstate` stack overflow (re-run); `transaction.rs:176-178` re-applies. R18 cm6 `create, update 0 => 1` \| Ferrum `… => Some(1)`.
- state.fe.084 `facet::Facet` PORTED, DEVIATION: R52 Ferrum `3,4`.
- state.fe.090 `Facet::compute` PORTED, DEVIATION: `facet.rs:103-112`; R5 cm6 `2` \| Ferrum panic `index out of bounds: the len is 0 but the index is 1` (re-run).
- state.fe.079, state.fe.102, state.fe.105: status `HUMAN DECISION` correct; justification text wrong (mismatch 2).

### Sample, C5 (state, transaction)
- state.cm6.001 `EditorStateConfig` PORTED, DEVIATION: `state.rs:83-90`. R21 cm6 `[3,7]` \| Ferrum `(4, 8)`.
- state.cm6.013 `EditorState.toText` MISSING: grep `to_text` → none. R19 cm6 `[4,7,"a\nb\nc\nd"]`.
- state.cm6.015 `EditorState.facet` PORTED, DEVIATION: R33d cm6 `[42]` \| Ferrum panic `index out of bounds: the len is 0 but the index is 1`.
- state.cm6.017 `EditorState^fromJSON` MISSING: grep as cm6.147. R22b cm6 `RangeError: Invalid JSON representation for EditorState`.
- state.cm6.020 `EditorState^tabSize` PORTED, DEVIATION: `state.rs:17-19` combine `first or 4` = `state.ts:267-269`. R26 both `2`.
- state.cm6.032 `EditorState^changeFilter` MISSING: grep `change_filter|ChangeFilter` → none. R11 cm6 `["one two",7,"one 2"]`.
- state.cm6.095 `Transaction.selection` PORTED, DEVIATION: `state.rs:269-275` stores the mapped selection. R5 cm6 `undefined` \| Ferrum `Some("2:2 main=0")`.
- state.cm6.100 `Transaction.state` PORTED, DEVIATION: `transaction.rs:176-178`. R4 cm6 `{"same":true,"updates":1}` \| Ferrum `(false, 2, 1)`; reentrant bin stack overflow.
- state.cm6.106 `Transaction^userEvent` PORTED, DEVIATION: `transaction.rs:155`. R9c Ferrum `(Some("select.pointer"), Some(false))`.
- state.cm6.108 `Transaction^remote` MISSING: grep `\bremote\b|Remote` → none. R41 cm6 `[true,false]`.
- state.cm6.111 `TransactionSpec.selection` PORTED, DEVIATION: no check in `update` (`state.rs:257-305`). R32 accepted in Ferrum (R1 re-run: cm6 throws `Selection points outside of document`).
- state.cm6.114 `TransactionSpec.userEvent` MISSING: `TransactionSpec` has 4 fields (`transaction.rs:67-76`); `ferrum-commands/src/history.rs` reads a `UserEvent` annotation, no spec field. R10b cm6 `[1,"input"]`.
- state.cm6.120 `Annotation.value` PORTED, RUST-IDIOMATIC: `transaction.rs:132-138`. R37 cm6 `[true,"v"]`.
- state.cm6.124 `StateEffect` PORTED, DEVIATION: `transaction.rs:11-46`, TypeId identity.
- state.cm6.126 `StateEffect.map` MISSING: grep `fn map` in `transaction.rs`, `map_effects` → none. R36 cm6 `{"a":6,"b":"undefined",…}`.
- state.fe.205 `RangeResult::with_changes` FERRUM-ONLY: `state.rs:48-51` replaces earlier changes.
- state.fe.266 `StateEffect::downcast_ref` PORTED, RUST-IDIOMATIC: `transaction.rs:39-45`.
- state.cm6.006 `EditorState.doc` PORTED: `state.rs:138-140`; test_state#001/#002 assert the doc. behaviors B1 complete.
- state.fe.208 `ChangeByRangeResult.changes` PORTED: `state.rs:63`; test_state#018/#019 assert the doc produced by these changes (`"qi"`, `"- -- --- ----"`), accepted as proof.
- state.fe.218 `EditorState::doc` PORTED: as cm6.006.

### Sample, C6 (rangeset)
- state.cm6.171 `RangeValue.eq` PORTED, DEVIATION: `rangeset.rs:24` structural default. R20 cm6 `range(0,5,[p],[p])` \| Ferrum `(no calls)`.
- state.cm6.175 `RangeValue.point` PORTED, DEVIATION: `rangeset.rs:20`.
- state.cm6.190 `RangeSet^spans` PORTED, DEVIATION: R11 cm6 `span(5,6,[A,B],0)…` \| Ferrum `span(5,6,[A,B],1)…`; R12, R12b, R18, R19 (X lost) all re-run with the documented outputs.
- state.cm6.199 `RangeCursor.to` MISSING: Ferrum cursors are private (`rangeset.rs:429,542,635`).
- state.cm6.263 `RangeSet.update spec.filterFrom` PORTED, DEVIATION: `rangeset.rs:223` `r.to < filter_from || r.from > filter_to` = `rangeset.ts:290`. R29 both `b,c -> 0-2:a 6-8:d`.
- state.fe.135, state.fe.143: status `HUMAN DECISION` correct; justification text (mismatch 2).
- state.fe.160 `RangeSetUpdate.filter`, state.fe.162 `filter_to` PORTED, DEVIATION: `rangeset.rs:277,279`.
- state.fe.164 `RangeSetBuilder::new` PORTED, RUST-IDIOMATIC: `rangeset.rs:319-335`.
- state.fe.369 `RangeSetUpdate as Default` PORTED, RUST-IDIOMATIC: `rangeset.rs:282-291`; `usize::MAX` vs `this.length` is equivalent for existing ranges.
- state.cm6.182 `RangeSet.size` PORTED: `rangeset.ts:252-257` = `rangeset.rs:122-131`; test_rangeset#001 (`5002`), #003, #004, #005–#007, #009 assert sizes. R25 both `2`. behaviors B1–B2 complete.
- state.fe.147 `RangeSet::size` PORTED: as cm6.182.

### Sample, main-session rows
- state.fe.175 `ferrum_state::selection` FERRUM-ONLY: module `lib.rs:10`; HD-API-PATHS exists (`human-decisions.md:31`).
- state.fe.312, .314, .332, .343, .344, .350 (crate-root re-exports, `lib.rs:19,19,26,31,31,32`), state.fe.375, .377, .387 (facade, `ferrum/src/lib.rs:12`): every target row exists and has the inherited status. Checked mechanically for all 68 re-export rows: 0 status mismatches.

### Extension sample (seed 20261022), all correct
state.cm6.004 (test_state#003 passes, `"QabcdQefghQ"`); .008 (R23 `None`); .045 (DROP, phantom `tag`, `facet.ts:125-128`); .050 (R2c cm6 `7,8,1` \| Ferrum `7,8,1,7,8`; R45 panic `Facet input type mismatch`); .055 (test_facet#005); .060 (`config.rs:26` `pub(crate)`, `state.rs:98` private; R15 cm6 `true undefined`); .071, .091 (grep JSON: none); .072 (no test calls `EditorSelection::single`, grep); .073 (4a confirms (c); 46,875 exhaustive cases, 0 range mismatches); .074; .076 (grep `undirectional|undecided|directionless`: none); .080 (goal kept by `map`, dropped by `extend`/merge: `selection.rs:92,100,109,186-189`); .087; .110; .118; .119 (`type_id` `pub(crate)` `transaction.rs:118`); .127 (no test calls `is::<`); .133; .134 (grep `combine_config`: none; `history.rs:100-126` hand-rolled merge, cm6 `history.ts:36-40` `combineConfig`; R31 cm6 `Config merge conflict for field a`); .141 (R2h panic text; R2p both `"1 0:1 4"`. The C2 `repro.ts` line labelled R2p is a different call, `mapDesc`, which throws because `ChangeSet.mapDesc` has no empty shortcut, `change.ts:213`. The row's description matches `repro2.ts` R2p'); .142 (R4a/R4b both; guard `change.rs:624`); .157 (R2f same text, R7j `0`); .172 (R31 table re-run, all 10 values as documented); .184 (R3, R4a/b/c, R5, R6/R6b re-run as documented); .191 (R9 cm6 throws \| Ferrum `0-0:b 0-0:a`; R22h); .192 (grep `fn join`: only `heightmap.rs:157`; cm6 `docview.ts:505`, `viewstate.ts:686`); .194 (private cursors `rangeset.rs:429,542,635`); .218, .223 (R22 `(99, 2)`); .241, .243 (grep: none); .248 (R12b cm6 `…afterEmptyTr=2` \| Ferrum `…=3`); .258. state.fe.009 (R3b2 `"5"`); .016; .023 (R1f/R1g/R1p re-run); .032; .040; .048; .098; .115 (HUMAN DECISION; R13 supports it); .126; .155 (R13–R17, R23 re-run as documented); .173 (R11/R12); .180; .197 (`is_single` used once, `commands.rs:1421`); .199; .202; .212; .221; .225 (R24 `(true, false, true)`); .253; .263; .275; .289; .295 (reentrant overflow); .296 (no test calls `start_state()`); .316, .321, .328, .349, .380, .385 (re-exports, statuses match targets); .355 (R19c `Text("ab\nc", chars=4, lines=2)`); .361 (`test_state.rs:64`).

### Test rows (20)
- test-change#17 → test_change#019 EQUIVALENT: both `over("2:2 2 2:2","2 2:0 2","2:2 2:2")` (`test-change.ts:113-115`, `test_change.rs:137-139`).
- test-change#24 → test_change#026 EQUIVALENT: same 11 tuples (assoc given as third element in cm6; `-1` default) line by line.
- test-change#27 → test_change#029 EQUIVALENT: `[2,2,-1] [2,2,1]`.
- test-change#30 → test_change#032 EQUIVALENT: same two compose+apply cases, same strings.
- test-change#36 → test_change#038 WEAKER: cm6 100 iterations, `count = floor(i/10)+1` (1–10), `size = r(20)`; Ferrum 50 iterations, count 1–3, size 1–20; same assertion. T4 none (no changed expected value).
- test-facet#05 → test_facet#005 EQUIVALENT: same inputs, `with_prec`, same order.
- test-facet#20 → test_facet#020 DIFFERENT: Ferrum `num.compute(|_| 1)` asserts `[1]`, no cycle. T4 "throw → plain value; none documented" correct.
- test-rangeset#08 → test_rangeset#008 WEAKER: exact 202 calls and sequence → `200 < count < 400`.
- test-rangeset#10 → test_rangeset#010 WEAKER: `from` only. Doc length 50 vs cm6 helper 5100 (`test-rangeset.ts:43`), documented in the row.
- test-rangeset#17 → test_rangeset#017 WEAKER: `from` only.
- test-rangeset#18 → test_rangeset#018 WEAKER: identity → `is_empty()`. The second map uses doc length 1000 (cm6 5100), noted in the row.
- test-rangeset#31 → test_rangeset#031 DIFFERENT: Ferrum old set empty, new `{(3,50)}`, no assertion; cm6 old `{(3,50)}` + add `(40,80)`, expects `[50,80]`.
- test-rangeset#47 → test_rangeset#047 EQUIVALENT: cm6 `mk(0,0)` = point, sides 1/1; Ferrum `TestVal::point` sides 1/1; `[(0,0)]`.
- test-selection#01 → test_selection#001 EQUIVALENT: five assertions, same values.
- test-selection#04 → test_selection#004 EQUIVALENT: `"10/0"`.
- test-text#02 → test_text#002 EQUIVALENT.
- test-text#12 → test_text#012 WEAKER: 200 → 50 iterations, LCG seed 42.
- test-text#15 → test_text#015 EQUIVALENT: `replace(100..201,"abc")` vs `[LINE+"abc"] + 198 × LINE`.
- test-text#18 → test_text#018 DIFFERENT: Ferrum reverses `to_string()` and `text0` and compares them, which cannot fail. The verdict is correct.
- test-text#22 → test_text#022 DIFFERENT: `slice_string` instead of `iterRange().next()`.

## Totals recount

| total | declared | recount | method |
|---|---|---|---|
| A1 cm6 rows | 264 | 264 | Own enumerator `SCR/verif/a1/enum.cjs` (TypeScript checker: `getExportsOfModule(index.ts)`, own declared non-private non-`@internal` properties of classes/interfaces, statics, enum members, ctor parameter properties) → 243 names. The difference from A1 is by design and fully explained: +17 config-object fields (FacetConfig 5, StateFieldSpec 6, StateEffectSpec 1, RangeSetUpdate 5; field lists re-read from `facet.ts:6`, `:260`, `transaction.ts:38`, `rangeset.ts:202`), +5 `Prec` members (`facet.ts:393-404`), +1 `RangeSetBuilder.constructor`; `Text.[Symbol.iterator]` = checker's `__@iterator`; my 2 extra `FacetReader.id`/`.default` are `@internal` (`facet.ts:121-124`), A1 rightly excludes them. 39 top-level exports = `index.ts`. |
| A2 Ferrum rows | 391 | 391 | Every `pub` item line in `crates/ferrum-state/src/*.rs` (non-test) has an A2 row at that `file:line` (only multi-line `pub use`/`pub mod` lines in `lib.rs` lack a location match, and their items are present); 48 crate-root re-exports counted in `lib.rs:16-33`; 19 std trait impls (`grep -nE "^impl.*(Display|Debug|Default|From<).* for "` → 19); 20 facade rows (`ferrum/src/lib.rs:3,12-16`). |
| T1 cm6 cases | 172 | 172 | `grep -cE "(^|[^a-zA-Z_.])it\("` over `state/test/*.ts`: 22+3+29+40+27+47+4. (A plain `grep -c "it("` reports 28 for `test-state.ts` because of `init(`.) |
| T2 Ferrum tests | 177 | 177 | `#[test]` count 3+32+4+42+27+22+47; `CARGO_TARGET_DIR=SCR/verif/target cargo test -p ferrum-state` → `177 passed; 0 failed; 0 ignored` (+ 0 doctests). |
| `api-matrix.md` rows | 264 + 391 | 264 + 391 = 655 | ids `state.cm6.001–264`, `state.fe.001–391`, each exactly once, no duplicates, no gaps. |
| `## Counts` | as printed | identical | cm6: DEVIATION 139, MISSING 78, RUST-IDIOMATIC 22, PORTED 15, HUMAN DECISION 4, BLOCKED: D1 3, DROP 2, PLACEMENT 1. Ferrum own: DEVIATION 154, HUMAN DECISION 59, FERRUM-ONLY 40, RUST-IDIOMATIC 31, PORTED 23, BLOCKED: D1 7, PLACEMENT 6, BLOCKED: D2 3. Re-export: 49/9/4/3/2/1. (These counts change if mismatch 1 is accepted.) |
| Table 1 | 172 | 172 rows, 172 unique ids | Every one of the 177 `test_<file>#NNN` ids of `raw/t2-ferrum-tests.md` occurs in Table 1. |
| Table 4 | per file | identical | Verdicts per file from Table 1: text 11/7/11, change 34/5/1, selection 4/0/0, charcategory 3/0/0, facet 6/5/11 (test-facet#13 has the verdict "DIFFERENT (input facet config differs; …)"), state 3/8/16, rangeset 7/19/21; total 68/44/60. |
| `progress.md` chunk table | 264 / 391 / 172 / 177 | sums match | cm6 rows 30+31+41+38+77+47; Ferrum rows 33+53+34+58+76+59+78; tests 29+40+7+22+27+47 and 32+42+7+22+27+47. |
| other | behaviors 948 rows, claims 488 | 948, 488 | row-id counts in `behaviors.md`, `claims.md`. |

Merge consistency (the session log in `progress.md` asks for it): every `api-matrix.md` row with a `raw/C<n>-api.md` evidence column was compared with the raw row's status. 107 `candidate:` rows were decided by the main session. 10 raw statuses carry a "(candidate)" suffix and are identical otherwise. 3 rows (state.cm6.213–215) were changed from raw `MISSING` to `BLOCKED: D1`, with a justification in the decision column. There are no unexplained differences.

## Targeted checks (4a–4g)

**4a. `EditorSelection.create` main index.** Re-run (`SCR/cm6/probe-V/sel4a.ts`, `SCR/repro-V/src/bin/sel4a.rs`): `create([cursor(12), range(12,18), cursor(12)], 2)` → cm6 `12/12,12/18 main=1` \| Ferrum `12/12,12/18 main=0`. A4's own oracle re-run (`probe-A4/selnorm.ts` + `repro-A4 --bin selnorm`) → `5000 cases, 0 mismatches`. Both claims are true; they measure different things.
- **Cause:** cm6 finds the main range after sorting by identity (`ranges.indexOf(main)`, `selection.ts:226`). Ferrum uses value equality (`position(|r| *r == main)`, `selection.rs:174`), so it picks the first equal twin. The final main index differs only when the twins are separated in sort order by a non-empty range with the same `from`, so that the merge loop's `if (i <= mainIndex) mainIndex--` runs a different number of times.
- **Why A4 saw nothing:** in A4's generator (1–5 ranges, anchors/heads 0..10, xorshift seed `0xA4A4A4A4`), 73 of the 5000 cases have a main range with an equal twin, but none in the triggering order.
- **Exhaustive check:** 3 ranges with anchor/head in 0..4, every main index (46,875 cases; `SCR/verif/sel-exh-cm6.txt`) → 20 mismatches, all main-index only, 0 in ranges. First case: `[(0,0),(0,1),(0,0)] main 2` → cm6 main 1 \| Ferrum main 0.
- **Consequence:** `claims.md` .132/.483 (mismatch 3). `progress.md` Q-merge-1 can be closed with this explanation.

**4b. `count_column` cost (XC-038), `"a" × n`, tab size 4.** `SCR/repro-V/src/bin/col.rs`, `SCR/cm6/probe-V/col.ts`.

| n | Ferrum release | Ferrum debug | cm6 (3 runs; first is JIT warm-up) |
|---|---|---|---|
| 2 500 | — | 2.04 s | — |
| 5 000 | — | 8.26 s | — |
| 10 000 | 3.52 s | 33.2 s | 3.87 / 0.092 / 0.125 ms |
| 20 000 | 14.2 s | — | 1.37 / 0.184 / 0.177 ms |
| 40 000 | 56.4 s | 524.0 s | 0.425 / 0.351 / 0.355 ms |

The release figure confirms XC-038's order of magnitude (XC-038 says 63 s; this machine measured 56.4 s). Both builds grow quadratically (×4 per doubling). XC-038 does not name a build profile; the 63 s figure is release (C3 R5: "release build").

**4c. XC-010 side truncation.** `rangeset.rs:247,250` (`start_side() as i8`, `end_side() as i8` for points) and `:259,260` (ranges) confirmed by reading. C6 R3a re-run: widget-like point with side −1e8 at 5, insertion at 5 → cm6 `5-5` (kept before) \| Ferrum `6-6` (moved). R3b–R3d and the R31 truncation table were also re-run, with identical values to `raw/C6-api.md` (e.g. side 128 → Ferrum `5-5` vs cm6 `6-6`; −100000000 → Ferrum `6-6` vs cm6 `5-5`).

**4d. C4/C5 failure claims.** All three confirmed.
- **Computed facet reading a computed facet:** C4 R5 → cm6 `2` \| Ferrum `PANIC index out of bounds: the len is 0 but the index is 1`. C5 R33d → cm6 `[42]` \| Ferrum the same panic.
- **Field update calling `tr.state()`:** `repro-C5 --bin reentrant` and `repro-C4 --bin trstate` → `thread 'main' has overflowed its stack` / `fatal runtime error: stack overflow, aborting`. cm6 R60 → `2`.
- **`readOnly` combine `[false, true]`:** C5 R24 → cm6 `[false,false,true]` \| Ferrum `(true, false, true)`. Cause: Ferrum `READ_ONLY` combines with `any` (`state.rs:21-23`); cm6 takes `values[0]` (`extension.ts:29-31`).

**4e. `ist` default comparator.** `npm view ist` → latest 1.1.7 (versions 1.0.0–1.1.7). I downloaded it with `npm pack ist@1.1.7` into `SCR/verif/ist-dl/x/` and read `ist.js`:
- `ist(a, b)` with no third argument uses `ops["=="]`, i.e. `a == b` (`ist.js:29-31`).
- Checked by running it: `ist([1],[1])` fails with `1 != 1`; `ist(a, a)` passes; `ist("1", 1)` passes (loose equality).
- So for objects and arrays the default is reference identity, as `raw/C4-tests.md` states.
- Not verified: which `ist` version cm6's test runner resolves. Neither `state/package.json` nor `buildhelper/package.json` in this checkout lists `ist`.

**4f. D1 table (`decisions-evidence.md`).** Seven rows were re-run on both sides (`SCR/cm6/probe-V/d1.ts`, `SCR/repro-V/src/bin/d1.rs`). All match the table.

| row | cm6 | Ferrum |
|---|---|---|
| `Text.of(["a😀b"]).length` | 4 | 3 (bytes 6) |
| insert `"😀"` at 1 | `1 0:2 3`, newLength 6 | `1 0:1 2`, new_length 4 |
| cm6 `delete(1,3)` replayed | valid | panic `Mismatched document length: changeset expected 4, doc is 3` |
| `findClusterBreak` of `"😀"` / `"🇫🇷🇩🇪"` | 2 / 4 | 1 / 2 |
| `findColumn("😀b",1,4)` | 2 | `Some(1)` |
| `codePointSize(0x1F600)` | 2 | (absent) |
| `replaceSelection("😀")` at 1 in `"ab"` | cursor 3 | cursor 2 |
| `sliceString(1,2)` | `"\ud83d"` | (not expressible) |

**4g. Ten `_cross-crate.md` entries** (`random.seed(20261021)`, excluding XC-010 and XC-038, which were checked above): XC-001, 008, 017, 022, 026, 027, 032, 033, 036, 037. Every cm6 evidence line was printed and says what is claimed: `extension.ts:10`, `history.ts:1`, `view/extension.ts:447,494-495`, `fold.ts:104`, `language.ts:422`, `tooltip.ts:31`, `comment.ts:64`, `indent.ts:373,376`, `cursor.ts:16,35,103,167`, `commands.ts:94,116,581,585,604`, `editorview.ts:261,308`, `input.ts:591,626`, `domchange.ts:60`, `docview.ts:218`, `bidi.ts:438`. The exception is XC-017, whose line range is off by one. Every Ferrum "what exists" was checked by grep/sed and is accurate:
- `commands.rs:20` `StateCommand` signature;
- `change.rs:99` untested in ferrum-state (used by `history.rs:419,421`), `tree.rs:176` `_changes`;
- `decoration.rs:197-225` `Range::new`;
- no `FacetReader` anywhere;
- `state.rs:269-275`, `view.rs:96,98`;
- `comment.rs:59`, `state.rs:190-198`, `commands.rs:746`;
- `test_fold.rs:41`, `test_stream_parser.rs:91,126,143`;
- `TRANSACTION_FILTER` used by `examples/headless_poc.rs`, `poc_editor.rs`;
- no bidi/undirectional in `selection.rs`;
- `char.rs:31` without `includeExtending`, `commands.rs:669,693`.

XC-032 cites the wrong repro id (mismatch 5).

## Unfinished

- Oracle-based figures quoted in rows (e.g. state.cm6.184 "2 472 of 4 500", state.cm6.050 "343 of 2000", state.cm6.157 "2,615,800 `mapPos` values") were not re-run; only the rows' point reproductions were. The C3 selection oracle was re-checked through 4a's exhaustive run.
- The 55 rows sharing the boilerplate HUMAN DECISION sentence (mismatch 2) were checked only for the 6 that fell in the samples (5 wrong, fe.115 supported by R13).
- Which `ist` version cm6's own test setup installs could not be determined from this checkout (4e).

## Corrections applied by the main session (after this pass)

| finding | correction |
|---|---|
| 1 state.fe.363 | `api-matrix.md`: status changed `PORTED` → `PORTED, DEVIATION` (untested); Ferrum counts now `PORTED` 22, `PORTED, DEVIATION` 155. |
| 2 claims | `claims.md`: state.claim.019, .077, .132, .483 changed `true` → `false` with the main-index counterexample; .287 kept `true` with a note; totals now 397 true / 80 false / 11 unverifiable (`raw/A4-claims.md` left as produced). |
| 3 boilerplate on 55 HUMAN DECISION rows | Sentence rewritten in `api-matrix.md` and `ferrum-only.md`: names the repros that exist (C4 R45, R46, R46b, R50, R50b; C2 R2s, R11c; C5 R45; this file for `Compartment.id`) and states that read-only accessors cannot break invariants by themselves. `human-decisions.md` HD-INTERNALS updated likewise. Statuses unchanged. |
| 4 state.fe.019 | Usage note now cites `crates/ferrum-state/src/tests/test_change.rs:603`. |
| 5 `_cross-crate.md` | XC-032 now cites C5 R4 (re-run) and C5 R33c (stack overflow); XC-017 cm6 lines corrected to `view/src/decoration.ts:298-300,317-319`; XC-038 adds the re-measured figures (release 56.4 s, debug 524 s). |

The extension sample (seed 20261022) came back clean, so per brief section 9 no further extension is required.
