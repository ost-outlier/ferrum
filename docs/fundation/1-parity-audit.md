# Specification audit for a new Ferrum crate

This brief applies to every Ferrum crate that is **built from scratch** against a cm6 package: `ferrum-view`, `ferrum-language`, and any later crate (search, autocomplete, lint, collab, merge, `lang-*`). It does not apply to `ferrum-commands`, which has its own sheet (`1d-scope-commands.md`) because its existing code is evaluated as well, or to checking an implemented crate, which is `3-conformance-audit.md`.

The audit reads **cm6 only**, plus the public API of the Ferrum crates below the audited one (to check that the capabilities it needs exist). It does not read, test or describe any existing Ferrum code of the crate being audited: that code is not a base and its defects are irrelevant. The output is the **implementation specification**: what the crate must do, item by item and behavior by behavior, with golden values produced by running cm6, and the list of cm6 tests to port. `2-parity-implementation.md` implements it; `3-conformance-audit.md` checks the result against it.

This phase is **audit only**: no Ferrum code, no Ferrum tests.

Per crate, use it with the crate's scope sheet (`1b-scope-view.md`, `1c-scope-language.md`, or a new one). The view and language sheets were written for the previous version of this brief: ignore their Ferrum columns, their calibration notes about existing Ferrum code and every claims-audit instruction; everything they say about cm6 still applies.

---

## 0. What the `ferrum-state` audit taught (why this brief looks like this)

The first audit (`docs/parity/state/`) compared cm6 with existing Ferrum code. Measured results that shaped this version:

| Observation | Consequence here |
|---|---|
| The cm6 side (inventory, dispositions, behavior lists, test inventory, oracle generators) is what the rebuild needs. The Ferrum side (391-row public-item matrix, Ferrum test mapping, 107 `FERRUM-ONLY` candidates, reproductions against code that will be discarded) only described defects of code that is not kept. | No Ferrum-side work for a new crate. |
| The claims audit (README, doc comments, commit messages, vision document) cost about 13% of all subagent tokens (≈300k of ≈2.35M). Of its 488 claims, 397 were true (after verification), most of them doc comments describing Ferrum's own code. Its one lasting finding (licensing, D9) is now in `decisions-evidence.md`. | No documentation or claims audit. The implementer rewrites the crate docs after the conformance audit is approved. |
| The most valuable evidence was produced by **running cm6**: differential oracles found defects no reading found (identity-based deduplication, the main-index bug, side values truncated to `i8`). | Every behavior gets golden examples produced by cm6 (section 6), stored as fixtures that the conformance audit replays. |
| One random oracle reported 0 mismatches in 5,000 cases on an input class where another oracle found a bug: its generator never produced the triggering shape (two equal ranges). | Generators must follow a shape checklist (section 8). |
| A `RangeSet` defect only appeared with the side values that `@codemirror/view` uses (±1e8…±6e8), never with the values in cm6's own state tests. | Behaviors are specified with the values that consumer packages use (section 6.3). |
| Many defects came from translating JavaScript semantics: object identity (`===`), lazily cached getters, global id counters, integer key order of objects, JS numbers in narrow Rust integers, exceptions. | A JS→Rust hazard list per item (section 9). |
| `count_column` was quadratic: 56 s on a 40,000-char line in release, against 0.35 ms in cm6. No test caught it. | The spec states the cost class of each item, and the conformance audit measures it. |
| 948 behavior rows: 22% were implemented and tested, 14% implemented but untested, 25% different, 33% missing. Many behaviors have no cm6 test at all. | Behaviors without a cm6 test get a derived-test specification with golden values. |
| Subagents produced Markdown tables with varying columns and `\|` inside code cells. Merging needed a custom parser; one boilerplate justification was copied onto 55 rows and was wrong for some of them. | Raw rows are JSON Lines with a fixed schema; Markdown is rendered by script; no shared boilerplate (section 11). |
| All seven parallel subagents hit an account usage limit at the same time. Work survived only because it was in files. | Subagents append each finished item to their raw file; at most 4 run in parallel (section 11). |
| Every subagent read ~25k tokens of instructions. | One condensed subagent card per crate (section 11). |
| The 10% verification sample found 5 real errors and the extension sample came back clean. | Verification is kept, and extended with mechanical checks (section 12). |
| References were written for the session that produced them, not for the AI that reads them later: paths relative to three different, unstated roots (`state/src/…` meant `CodeMirror6/state/src/…`, `crates/…` meant `ferrum/crates/…`, `rules.md` existed in two places); evidence ids such as `C1 R9` or `P7` that only resolve if one knows which raw file and section; scratch paths under `/tmp` that disappear after the session; line numbers without the symbol or the commit they refer to. | One path root, qualified evidence ids, a resolution index, no scratch paths, `file:line` plus symbol plus version; checked mechanically (section 11.1). |

---

## 1. Preconditions and inputs

- **Decisions.** D1 (position unit) and D2 (line breaks) must be recorded in `ferrum/docs/parity/decisions.md` before phase 2: almost every behavior depends on them. Crate-specific decisions (D3 for language, D4/D7/D8 for view) may still be open: the audit collects their evidence and states each dependent behavior in a decision-neutral way.
- **Lower crates.** For each Ferrum crate the audited one depends on: its conformance report (`docs/parity/<crate>/conformance.md`) if it is implemented, otherwise its approved specification (`inventory`, `dispositions`, `behaviors`). These define which capabilities exist.
- **Read before starting:** `ferrum-foundation.md`, `rules.md`, this brief, the scope sheet, `decisions.md`, `_cross-crate.md`, the crate's `progress.md` if it exists. Subagents read only the subagent card (section 11).

Paths: cm6 under `/workspace/CodeMirror6/`, Ferrum under `/workspace/ferrum/`, outputs in `/workspace/ferrum/docs/parity/<crate>/`.

---

## 2. Ground rules

1. **cm6's code is the specification**; its README and docs are secondary. When they disagree, the code wins.
2. **Purpose filter.** Ferrum is a native, headless engine (`ferrum-foundation.md` §1–2). Do not make something `CORE` only because cm6 has it, and do not `DROP` it only because cm6 implements it with the DOM: ask what it does for a native editor.
3. **Missing DOM is not an excuse.** Every `DROP` needs an item-specific reason. If the *behavior* matters natively (input semantics, composition, selection drawing, scrolling, measuring), it is `CONTRACT` or `SURFACE`.
4. **Specify, do not design.** For `CONTRACT` items, state the question asked and what the logic does with the answer; do not write traits.
5. **Do not decide foundational questions**, approve your own dispositions, or invent justifications. Unresolvable points go to `human-decisions.md` with options.
6. **Do not read the existing Ferrum crate being audited**, its tests or its docs. Read lower Ferrum crates only through their public API and conformance report.

---

## 3. Deliverables

Per crate, in `docs/parity/<crate>/` (English):

| File | Content |
|---|---|
| `progress.md` | totals, chunks with state and token cost, n of N done, open questions |
| `raw/*.jsonl` | source of truth for every row (schemas in section 11); everything below is rendered from it |
| `inventory.md` | A1: one row per cm6 export and public member |
| `dispositions.md` | one row per export and per source file; disposition and reason |
| `behaviors.md` | per non-`DROP`/non-`DEFER` item: behaviors `B1…Bn`, cost class, hazards, golden fixture ids, cm6 tests |
| `contracts.md` | per `CONTRACT` item: the question, the logic around it, the minimal fake needed to test it |
| `tests.md` | T1: every cm6 test case with classification and port plan; helper specifications; derived-test list |
| `fixtures/<chunk>.jsonl` + `oracle/` | golden values produced by cm6, and the cm6-side generators with seeds |
| `hazards.md` | JS→Rust hazards per item (section 9) |
| `human-decisions.md` | everything left to a human, with options |
| `verification.md` | section 12 |

Workspace-wide: `docs/parity/_cross-crate.md` (A5 register) and `docs/parity/decisions-evidence.md`.

---

## 4. Phases and gates

| Phase | Work | Output |
|---|---|---|
| P0 Setup | build the cm6 harness (section 7), run the mechanical inventory, declare totals | `progress.md`, `raw/inventory.jsonl` |
| P1 Dispositions | disposition per export and per file; proposed first-version scope (`DEFER` list, D10) | `dispositions.md` |
| **Gate 1** | human approves dispositions and the `DEFER` list | |
| P2 Behaviors | behaviors, cost class, consumer values, golden fixtures; contract questions | `behaviors.md`, `contracts.md`, `fixtures/` |
| P3 Tests | T1 catalog, classification, port plan, helper specs, derived-test list | `tests.md` |
| P4 Cross-cutting | hazards, cross-crate capability check, decision evidence | `hazards.md`, `_cross-crate.md`, `decisions-evidence.md` |
| P5 Verification | fresh-context check (section 12) | `verification.md` |
| **Gate 2** | human approves the specification; implementation may start | |

P2 and P3 cover only items whose approved disposition is `CORE`, `CONTRACT`, `SURFACE` (the core-owned part) or `ELSEWHERE` pointing to this crate, and that are not `DEFER`. `DEFER` and `DROP` items stop after P1.

---

## 5. Inventory and dispositions (P0–P1)

**A1.** Derive the public API from every `export` reachable from the package's `src/index.ts`, not from the README. One row per: top-level export (class, interface, type, enum, function, const), public instance and static member (methods, getters, fields, constructor parameter properties), enum member, and field of every config object type accepted by an exported function (for example `FacetConfig`, `StateFieldSpec`, `RangeSetUpdate`; these fields carry behavior and were a main source of `MISSING` items in the state audit). Exclude `private` and `@internal` members, but read them as helpers. Report every discrepancy with the package README symbol list.

Generate the list mechanically with the TypeScript compiler API (`docs/parity/state/oracle/tools/cm6-exports.cjs`; adapt its list of config object types per package) and count test cases the same way (`cm6-tests.cjs`). Declare totals before any other work.

**Dispositions** (one per row and per source file; proposals until Gate 1):

| Disposition | Meaning |
|---|---|
| `CORE` | Platform-independent logic; belongs in this crate with behavioral parity. |
| `CONTRACT` | Logic belongs here but needs geometry, input or syntax that cm6 gets from the DOM or Lezer. Name the question. |
| `SURFACE` | Drawing, measuring or OS integration belongs to a surface crate. Name the model or facet this crate still owns. |
| `ELSEWHERE: <crate>` | Belongs to another Ferrum crate. |
| `DEFER` | Needed by a native editor but not in the first version (D10). Reason and evidence of who needs it. |
| `DROP` | Exists only because of the DOM, CSS, browsers or the TypeScript type system. Item-specific reason. |

---

## 6. Behavior specification (P2, the core)

For every item in scope:

1. Read the whole cm6 function and, transitively, every helper it uses (list them with `file:line`).
2. List its observable behaviors as `B1…Bn`, ids `<crate>.<item>.B<n>`, stable once published. One behavior per branch, guard, early return, edge case and side effect. Each behavior states: input class, observable result, cm6 `file:line`, decision dependencies (`D1`, `D2`, …), cm6 tests that cover it (or `none`).
3. Produce **golden examples** (6.2) for every behavior that can run headless.
4. State the item's **cost class** as cm6 implements it: O(1), O(log n), local to a line/range/chunk, linear in the document, and what cm6 avoids (for example materializing the document as a string).
5. For `CONTRACT` items, write the question in cm6 terms in `contracts.md` (for example: "`moveVertically` asks for the screen coordinates of the head and for the position at (x, y ± line height)"), what the logic does with each answer, and the smallest fake implementation that reproduces the cm6 tests (fixed-width cells, fixed line height, hand-built syntax tree).

Dimensions to consider (add whatever the code shows): return value including "did nothing"; resulting selection (anchor/head, `assoc`, goal column, bidi level, main index, ordering, merging); generated changes (ranges, order, merging); transaction metadata (user events, `scrollIntoView`, effects, annotations, filters, time); configuration consulted (read-only, tab size, indent unit, line separator, language data, facets, precedence); boundaries (empty document, document start/end, empty line, whitespace-only line, last line without break); text semantics (position unit, separators, grapheme clusters, ZWJ, combining marks, surrogate pairs, CRLF, RTL, tabs, word categories); layout dependencies (wrapping, folds, widgets, atomic ranges, viewport, direction); syntax dependencies; **identity** (does any caller compare results with `===`?); interaction with other packages.

### 6.1 Items without a runnable cm6 entry point

Some behaviors need a DOM (view) or a grammar (language). Describe the inputs in terms of the fake from `contracts.md`, and take expected values from cm6 test assertions where they exist. If a headless browser runner is available (for example Playwright with Chromium installed in the harness), `webtest-*` scenarios may be run to produce golden values; record the browser version. Otherwise write `golden: none (needs DOM)` with the reason.

### 6.2 Golden fixtures

A fixture is one cm6 run: `{id, item, behaviors, input, output}` in `fixtures/<chunk>.jsonl`. Rules:

- Produced by running the cm6 sources in the harness, never written by hand.
- Positions and lengths as cm6 produces them (UTF-16 code units). Store the text as well, so the conformance audit can map positions to the unit chosen in D1.
- For every behavior that depends on D1 or D2, at least one ASCII fixture and one fixture with astral characters, CRLF and U+2028 as relevant.
- Strings must be representable in Rust: never emit lone surrogates in JSON. Encode such strings as arrays of UTF-16 code units and mark the fixture.
- Exceptions are outputs: record the error class and message.

### 6.3 Consumer values

For every item another cm6 package uses, grep those packages for the constants and value ranges they pass (for example decoration sides ±1e8…±6e8 in `view/src/decoration.ts`, `Prec` levels, large positions, `minPointSize`) and include them in the behaviors and fixtures. Record them as `consumer values` in the item's section.

---

## 7. Harness

- cm6 side: a scratch directory with the package sources copied in and dependencies installed with npm (`@marijn/find-cluster-break`, `@lezer/*`, grammars such as `@lezer/javascript`), run with `tsx`. The state audit's setup is described in `docs/parity/state/oracle.md`; reuse its scripts.
- Keep generators and probe scripts in `docs/parity/<crate>/oracle/` (sources only; data is reproducible from seeds; large outputs stay out of the repository).
- No Ferrum side in this phase. The conformance audit writes the Ferrum replayers.

---

## 8. Oracle preparation (shape checklist)

For every pure item (state-only, or reachable through the fake), write a seeded generator (mulberry32 or equivalent; record seed and case count) whose outputs become fixtures. A generator must cover, and its file header must say how:

- empty input, single element, many elements;
- **equal and duplicate elements** (same range twice, same object reused twice in an extension tree);
- adjacent, overlapping and nested elements; unsorted input;
- boundaries: 0, document end, end of line, positions just inside and outside a range;
- consumer values (6.3) and extreme values of every numeric parameter;
- ASCII, BMP non-ASCII, astral characters, CRLF, lone CR, U+2028 (where text is involved);
- sizes large enough to expose cost (at least one case 100× the typical size, used by the conformance audit's scaling check).

Record per generator which checklist entries are not applicable and why.

---

## 9. JS→Rust hazards

For each item, list which of these its cm6 implementation relies on, with `file:line`. The conformance audit tests every listed hazard.

- object identity (`===`, `indexOf`, `Map`/`WeakMap` keys) used for equality, deduplication or change detection;
- lazily computed and cached getters (for example `Transaction.state`), re-entrancy during computation;
- mutation of shared objects, aliasing;
- exceptions (`RangeError`, `Error`) and their messages; `undefined`/`null`/`NaN` results;
- JS numbers: values that do not fit a narrow Rust integer, negative values, fractional values, `Infinity`, `-1` sentinels;
- integer-keyed object iteration order (ascending), `Array.sort` stability, `Map` insertion order;
- UTF-16 string operations (`length`, `charCodeAt`, `slice` inside a surrogate pair), regex `\p{…}` and the `u` flag, `Intl.Segmenter`;
- module-level counters and global state (ids), `Date.now()`;
- prototype dispatch, `instanceof`, duck typing, union-typed parameters (`ChangeSpec`), optional fields with defaults;
- closures capturing mutable state.

---

## 10. Tests, cross-crate capabilities, decision evidence (P3–P4)

**T1 catalog.** Every `it()` in every file of the package's `test/` directory, **including `webtest-*.ts`**, with a unique id (`<file>#<nn>`). Expand loop-generated cases. Per case: `file:line`, scenario (input document, selection, configuration and extensions, languages loaded), action, expected result, behaviors covered, classification:

- `LOGIC-ONLY`: depends only on state; ports as is.
- `NEEDS LAYOUT`: wrapping, widgets, folds, bidi, coordinates; ports against the fake (name the fake parameters).
- `NEEDS GRAMMAR`: uses a Lezer grammar; name it, and list which expected values depend on the grammar (they will be checked against the Ferrum grammar chosen in D3).
- `DOM-ONLY`: asserts DOM structure only; reason. Being in a `webtest` file is not a reason by itself; record the *semantic* the case pins down (for example decoration nesting order), which may be portable even when the assertion is not.

Port notes: helpers used, values that change under the D1/D2 decisions (with their new values computed from the fixture text), loops and randomness to reproduce.

**Helper specifications (T5).** For every cm6 test helper (for example `commands/test/state.ts` `mkState`: `|` for a cursor, `<…>` for a range, `allowMultipleSelections` on), its exact parsing and assertion rules, so the Ferrum helper can mirror it.

**Derived tests (T7).** Every behavior without a cm6 test gets a derived-test specification: the behavior id, input and expected value taken from a golden fixture, marked for `// derived from cm6 source: <file:line>`.

**Cross-crate (A5).** For every capability an item needs from a lower crate (cite the cm6 call site), check the lower crate's public API or approved specification. If it is absent or differs, open an entry in `_cross-crate.md` owned by the lower crate. Such entries feed back into the lower crate before the dependent item is implemented. Capabilities this crate owns that higher cm6 packages use are listed too (they seed later audits).

**Decision evidence.** For each open decision touched (D3, D4, D7, D8, D10…), append to `decisions-evidence.md`: items affected, concrete examples (`file:line`, fixture ids), what each option implies.

---

## 11. Orchestration

- **Main session** keeps everything that needs the whole picture: dispositions, scope, contract grouping, cross-crate entries, decision evidence, rendering, the gate summary. **Subagents** produce facts for one chunk (a source file or an export range of a large file; a test file).
- **Subagent card.** The main session writes `docs/parity/<crate>/subagent-card.md`: the rules of sections 2, 6, 8 and 9 condensed, the JSONL schemas, harness paths, and the output contract. A subagent reads the card, the scope sheet and its sources; not the whole document set.
- **Chunk size and concurrency.** About 40–80 inventory rows or ≤1,000 cm6 source lines per chunk; at most 4 subagents in parallel. Record each chunk's token cost in `progress.md` to calibrate the next crate (state reference: 240k–320k tokens per chunk of 30–77 cm6 items, including Ferrum work that this brief removes).
- **Incremental output.** A subagent appends each finished item to its JSONL file immediately and keeps a `done/remaining` line in `raw/<chunk>.progress`. An interrupted subagent resumes from that line.
- **Fixed schemas** (one JSON object per line; arrays where marked):
  - `inventory.jsonl`: `id, item, kind, file, line, chunk, disposition, reason`
  - `behaviors.jsonl`: `id, item_id, text, cm6, deps[], hazards[], consumer_values[], cost, cm6_tests[], fixtures[], golden_note`
  - `tests.jsonl`: `id, file, line, describe, name, scenario, expected, behaviors[], class, class_reason, port_notes`
  - `fixtures/<chunk>.jsonl`: `id, item_id, behaviors[], input, output, encoding`
  - `xc.jsonl` (cross-crate facts): `capability, cm6, needed_by, searched, exists`
  The main session renders Markdown from these with a script kept in `docs/parity/tools/`.
- **No boilerplate.** A reason shared by many rows is written once as a group entry (for example `group: TS-PHANTOM`) and referenced by id; it must be true of every member.
- Every subagent ends by listing unfinished ids. An unlisted gap counts as a falsified result.

### 11.1 References an AI can follow

Every deliverable is read later by an AI (implementer, conformance auditor, next audit) that has none of this session's context. A reference is valid only if that reader can open the target with the reference alone.

1. **One root.** Every path is relative to `/workspace/`: `CodeMirror6/state/src/text.ts:38`, `ferrum/crates/ferrum-view/src/lib.rs:12`, `ferrum/docs/parity/view/behaviors.md`, `ferrum/docs/fundation/rules.md`. No path relative to an unstated directory, no bare file names (`rules.md`), no `./`.
2. **No scratch paths.** Nothing under `/tmp`, a scratchpad or a home directory appears in a deliverable. Scripts and data a reader needs are copied into `ferrum/docs/parity/<crate>/oracle/` first and referenced there.
3. **Code references carry the symbol and the version.** Write `` `CodeMirror6/view/src/cursor.ts:102` (`byGroup`) ``. Each deliverable states once, at the top, the cm6 package version and the Ferrum commit its line numbers refer to, so a reader can recover a drifted line by symbol.
4. **Qualified evidence ids.** An id is written so that it names its file: `view/raw/C3-fixtures.jsonl#F0042`, `view/behaviors.md#view.moveVertically.B3`, `_cross-crate.md#XC-041`. Each id is defined in exactly one place. Short forms (`R9`, `C1 P7`) are allowed only inside the file that defines them.
5. **Resolution index.** `ferrum/docs/parity/README.md` lists every id scheme (inventory rows, behaviors, tests, fixtures, register entries, decisions, human decisions), the file that defines it and how to resolve it. A new scheme is added there before it is used.
6. **Reading header.** Every deliverable starts with three lines: what the file is, the versions (3), the id schemes it uses (5).
7. **Mechanical check.** Verification (section 12) runs a reference checker: every path exists, every line number is within the file and the cited symbol appears within a few lines of it, every qualified id resolves, no scratch path remains.

---

## 12. Verification (P5)

A fresh context (subagent or new session) that wrote no rows:

1. **Mechanical checks:** the reference checker of section 11.1; totals recounted from the cm6 sources (A1, T1); no duplicate ids; every in-scope item has at least one behavior; every behavior has fixtures or a `golden_note`; every fixture reproduces bit-identically when its generator is re-run with the recorded seed; every test case has a classification; repeated reason texts across rows are listed and checked against each member.
2. **Sample:** at least 10% of items (minimum 20), chosen with a recorded seed, plus every `CONTRACT` item: re-derive the behavior list from the cm6 source and look for missing behaviors; re-run three fixtures per item; check the test classifications of the sampled items.
3. Mismatches are fixed and the sample is extended with a new seed until a sample comes back clean.

---

## 13. Completeness rules

- Declare totals first. Every table has exactly that many rows.
- One item, one row. No "etc.", "similar to above", "same as previous".
- Work in chunks, save after each item, report "n of N done" in `progress.md`. Unfinished items are listed explicitly.
- A row without a cm6 `file:line` is invalid.

---

## 14. Gate 2 deliverable

Deliver to the human, then stop:

- totals and per-disposition counts; items in scope; behaviors (count, with and without cm6 test, with and without fixtures); fixtures per chunk; cm6 test cases per classification;
- the contract questions, grouped (input to D3/D4/D8);
- the hazard summary (which hazards occur, how often);
- open cross-crate entries, in both directions;
- decision evidence added;
- `human-decisions.md`;
- verification result;
- confirmation that every source file and test file listed in the scope sheet is covered.

---

## 15. Do not

- read, test or describe the existing Ferrum code of the crate being audited
- audit READMEs, doc comments, commit messages or other claims
- write Ferrum code or tests
- design traits for contracts (state the questions)
- hand-write golden values
- skip a source file or a test file because it is large or a `webtest`
- port DOM machinery into the specification because cm6 has it, or drop native-relevant behavior because cm6 implements it with the DOM
- decide foundational questions or invent justifications

---

## 16. Retrofitting `ferrum-state`

`docs/parity/state/` was produced by the previous version of this brief. Its cm6 side is a valid specification: `raw/a1-cm6-exports.md`, `dispositions.md`, the cm6 columns of `behaviors.md` (behavior text, cm6 `file:line`, cm6 tests), the cm6 columns of `tests-side-by-side.md` Table 1, `oracle/cm6-probes/`, `decisions-evidence.md` and `_cross-crate.md`. Its Ferrum side (statuses, Ferrum tests, Ferrum-only and placement lists, claims) describes code that will not be kept. Before rebuilding state, run P2 steps 3–4 (fixtures, cost class), section 8 (generators against the checklist) and section 9 (hazards) on the existing behavior list; then implement and run `3-conformance-audit.md`.
