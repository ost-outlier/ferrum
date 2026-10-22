# Implementation of `ferrum-state` (rebuild)

`ferrum-state` is rebuilt from cm6 `@codemirror/state` 6.7.6. This sheet adds what that crate needs on top of `2-parity-implementation.md`, which applies in full unless stated otherwise here. Why a separate sheet: the state audit (`ferrum/docs/parity/state/`, 2026-10-21) followed the previous audit brief. Its cm6 side is a valid specification but lacks parts the new briefs require (golden fixtures, cost classes, hazards, implementation sketch); its Ferrum side describes the old code, which is not kept.

Paths are relative to `/workspace/`; instruction documents named alone live in `ferrum/docs/fundation/`; id schemes are resolved in `ferrum/docs/parity/README.md`. The old crate is `ferrum/crates/ferrum-state/` at commit `ed70610`.

---

## 1. What the state audit provides

| Use as specification | File |
|---|---|
| inventory: 264 cm6 rows (`state.cm6.NNN`) | `ferrum/docs/parity/state/raw/a1-cm6-exports.md` |
| dispositions (262 `CORE`, 2 `DROP`; to be approved) | `ferrum/docs/parity/state/dispositions.md` |
| behaviors `state.<Item>.B<n>`: the columns *behavior*, *cm6 file:line* and *cm6 test(s)* (948 rows) | `ferrum/docs/parity/state/behaviors.md` |
| cm6 test catalog: 172 cases, the cm6 columns of Table 1 (scenario, `file:line`) | `ferrum/docs/parity/state/tests-side-by-side.md`, `ferrum/docs/parity/state/raw/t1-cm6-tests.md` |
| cm6-side generators and probes | `ferrum/docs/parity/state/oracle/cm6-probes/`, harness in `ferrum/docs/parity/state/oracle.md` |
| decision evidence (D1, D2, D5, D6, D9, D10, grapheme policy) | `ferrum/docs/parity/decisions-evidence.md` |
| capabilities that view, language and commands need from state: XC-004 … XC-039 (all state-owned, all open) | `ferrum/docs/parity/_cross-crate.md` |
| open questions, grouped `HD-…` | `ferrum/docs/parity/state/human-decisions.md` |
| known defects of the old code, as a list of traps (§4) | `ferrum/docs/parity/state/raw/C<n>-api.md` reproductions, `ferrum/docs/parity/state/verification.md` |

| Do not use as specification | Why |
|---|---|
| Ferrum columns of `api-matrix.md`, `behaviors.md`, `tests-side-by-side.md`; `raw/a2-ferrum-pub.md`, `raw/t2-ferrum-tests.md` | they describe the old code |
| `ferrum-only.md`, `placement.md` | old public items; their placement facts are carried into §3 |
| `claims.md` | about old documentation; keep only as a list of claims not to repeat (`2-parity-implementation.md` §6) |

---

## 2. Phase 0: complete the specification (audit work, no code)

Run these steps of `1-parity-audit.md` on the existing material, in the main session with subagents per chunk (C1–C6 as in `ferrum/docs/parity/state/progress.md`), writing to `ferrum/docs/parity/state/` (new files next to the old ones):

1. **`raw/behaviors.jsonl`** (§11 schema) from the cm6 columns of `behaviors.md`, keeping the ids `state.<Item>.B<n>` unchanged. Fill `deps`, `hazards`, `consumer_values` and `cost` per item (§6 step 4, §6.3, §9).
2. **Fixtures** (§6.2) for every behavior, from the cm6 harness. Extend the existing generators to the shape checklist (§8); the state audit showed the gaps that matter: equal and duplicate ranges (`EditorSelection.create` main index), the same extension object reused (identity deduplication), decoration side values ±1e8…±6e8 (`RangeSet.map`/`between`), astral characters, CRLF and U+2028.
3. **`hazards.md`** (§9). Seed list from the audit: value identity of facets and fields (`compare`, `compareInput`), `Transaction.state` computed once and re-entrant, facet slot evaluation in ascending id order (`CodeMirror6/state/src/facet.ts:489`), module-level id counter, `Date.now()` in `Transaction.time`, JS numbers as sides and `assoc`, `===` in `EditorSelection.create` (main index), exceptions and their messages, UTF-16 string operations (`findClusterBreak`, `countColumn`, `Text.sliceString` inside a surrogate pair).
4. **Test catalog** (`raw/tests.jsonl`, §10): all 172 cases are `LOGIC-ONLY` (`@codemirror/state` has no `webtest-*`); add the values that change under D1/D2 and the derived-test list (behaviors whose *cm6 test(s)* column is empty).
5. **`implementation-plan.md`** (§17): module map, Rust shapes, obvious decisions, native opportunities. Several `HD-…` groups become sketch choices or obvious decisions here: `HD-INTERNALS`, `HD-NAMES`, `HD-ROPE`, `HD-API-PATHS` were about old public items, and for the new crate they reduce to "cm6 `@internal` stays `pub(crate)`" (obvious) plus the module/re-export layout.
6. **Verification** (§12) of the completed parts, then **Gate 2**.

---

## 3. Preconditions for code

In addition to `2-parity-implementation.md` §0:

- `decisions.md` records **D1** and **D2** (they change almost every type of the crate), **D5** (where `StateCommand` lives and its shape), **D6** (ownership, threading, id allocation), **D9** (license fields, license files, cm6 MIT notice in the repository), **D10** (first-version scope: for example JSON, `phrases`, `Text.children`).
- The remaining `HD-…` groups are answered: `HD-TYPED-IDS`, `HD-ERRORS`, `HD-GRAPHEME`, `HD-SELECTION-MODEL`, `HD-CHANGESPEC`, `HD-ASSOC-TYPE`, `HD-TX-PIPELINE`, `HD-FACET-MODEL`, `HD-RANGESET`, `HD-TEXT`, `HD-TESTS`.
- **Workspace while rebuilding** (human decision): the other crates compile against the old `ferrum-state` API and will break. Options: (a) remove `ferrum-view`, `ferrum-language`, `ferrum-commands`, `ferrum-view-gpui` and the facade `ferrum` from `[workspace] members` in `ferrum/Cargo.toml` until they are rebuilt (they stay in the tree; the commands quarry runs them from a worktree at `ed70610`, `1d-scope-commands.md` part B); (b) keep them compiling with adapters (work thrown away later); (c) delete them (git history keeps them). Recommended: (a), so that `cargo test --workspace` and `clippy` stay meaningful (`rules.md` §3.5, §6.6).

Placement facts that the new crate must respect (from `ferrum/docs/parity/state/placement.md` and the register):

- `StateCommand` lives in `ferrum-state`, unless D5 decides otherwise (XC-001).
- `isolateHistory` is a `ferrum-commands` item; do not define it in state (XC-002).
- group and subword motion (`find_word_break`, `find_subword_break` in the old crate) are not state items (XC-003); character categories come from `EditorState.charCategorizer(at)` with language `wordChars` (XC-027).

---

## 4. Implementation notes

Order: `text` (D1/D2 decide its types; everything else depends on it), `change`, `selection` with `char`/`charcategory`/`column`, `rangeset`, `facet`/`config`/`extension`, `state`/`transaction` (the transaction pipeline uses facets). Inside each module, cm6 source order.

**The old code is not the base.** It may be *consulted* as a reference for an algorithm only where the audit's differential oracle found 0 mismatches on ASCII input (`ferrum/docs/parity/state/oracle.md`): `ChangeSet`/`ChangeDesc` (`of` for flat lists, `apply`, `compose`, `map`, `invert`, `mapPos`, `iterChanges`, `iterGaps`, `iterChangedRanges`), `changeByRange`/`replaceSelection`, and the precedence ordering of extension flattening. Even there the old code has known defects (see the traps below); write the item against the new types, with its tests ported first.

**Traps found by the audit** (each must have a test that would have caught it; ids resolve in `ferrum/docs/parity/README.md`):

| Area | Trap | Evidence |
|---|---|---|
| text | separators stored raw, ropey `unicode_lines` breaking on U+2028/VT/FF/NEL; positions inside CRLF; `line()` copying the line into a `String` | `ferrum/docs/parity/state/raw/C1-api.md` R5, R6c, P1–P4, P7 |
| text | no iterators (`iter`, `iterRange`, `iterLines`), no `sliceString(…, lineSep)` | XC-004, XC-005 |
| change | string inserts not split on `\r\n?\|\n`/`lineSep`; `assoc` as `i8`; no `touchesRange`, no JSON | `ferrum/docs/parity/state/raw/C2-api.md` R9, XC-007, XC-009, XC-010 |
| selection | main range found by value instead of identity; `assoc` always 0; equality comparing `assoc`; no `checkSelection`; no bidi level, no undirectional ranges | `ferrum/docs/parity/state/raw/C3-api.md` R4, XC-035, XC-036 |
| char / column | `findClusterBreak` O(n) per call, `countColumn` O(n²) (56 s on a 40,000-char line vs 0.35 ms) | XC-038, `ferrum/docs/parity/state/verification.md` check 4b |
| rangeset | sides truncated to `i8`; ±1e6 sentinels instead of ±1e9; `spans` open counts and a lost point; no structural sharing; no static `iter`, `eq`, `join`; `compare` without `minPointSize`/`boundChange` | `ferrum/docs/parity/state/raw/C6-api.md` R2, R3, R11, R12, R13, R19, R31; XC-010–XC-018 |
| facet / config | computed facet reading a computed facet panics; no dependencies, no `compare`, no identity; no deduplication by identity; no cycle detection; compartments (duplicate use, nesting, stale content); missing `enables`, `provide`, `Facet.from`, `StateField.init`, `Compartment.get`, `combineConfig` | `ferrum/docs/parity/state/raw/C4-api.md` R5–R14, R17, R19; XC-019–XC-025 |
| state / transaction | `Transaction::state()` recomputed per call and overflowing the stack when re-entered; no `changeFilter`, `transactionExtender`, `filter: false`, `sequential`, multi-spec merge; filter as a boolean veto; `Transaction.selection` holding the mapped selection; `readOnly` combining with `any`; `allowMultipleSelections` and `lineSeparator` never read; effect and annotation types identified by `TypeId` | `ferrum/docs/parity/state/raw/C5-api.md` R1–R5, R9, R16–R24, R33c; XC-026–XC-035 |

**Tests.** Port all 172 cm6 cases (`tests.jsonl`) and the derived tests. cm6 test-text#13 contains a bug (`CodeMirror6/state/test/test-text.ts:96` overwrites its random range): follow the `HD-TESTS` answer. Replay every fixture and run every oracle driver (C1–C6) at the audit's sizes before hand-off.

---

## 5. Hand-off and afterwards

Hand-off as in `2-parity-implementation.md` §7, then `3-conformance-audit.md` with the specification of §1–§2. After the conformance report is approved:

- rewrite `ferrum/crates/ferrum-state/README.md` and `ferrum/crates/ferrum-state/src/README.md` (`2-parity-implementation.md` §6);
- close the register entries XC-004 … XC-039 that the crate now provides, citing item and test; the view, language and commands audits check them against the new API;
- the state audit folder stays as history; mark its Ferrum-side files as superseded in `ferrum/docs/parity/state/progress.md`.
