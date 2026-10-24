# Specification audit for a new Ferrum crate

This brief turns a cm6 package into the **implementation specification** of a new Ferrum crate: what the crate must do, item by item and behavior by behavior, with golden values produced by running cm6, the cm6 tests to port, the questions the crate must ask a surface or a parser, and a first sketch of the Rust implementation.

It applies to `ferrum-view`, `ferrum-language` and every later crate (search, autocomplete, lint, collab, merge, `lang-*`). `ferrum-commands` uses it too, through `1d-scope-commands.md`, which adds an evaluation of the existing commands code. Use this brief together with the crate's scope sheet (`1b-scope-view.md`, `1c-scope-language.md`, or a new one written in the same form).

The audit reads **cm6 only**, plus the public API of the Ferrum crates below the audited one. It never reads, tests or describes existing Ferrum code of the crate being audited. It writes no Ferrum code. After Gate 2 the crate is implemented with `2-parity-implementation.md` and checked with `3-conformance-audit.md`.

Paths are relative to the repository root (`mirrors/cm6/…` is the cm6 mirror, `rules.md` §8.1); instruction documents named alone live in `docs/fundation/`; id schemes are resolved in `docs/parity/README.md`.

---

## 1. Preconditions and inputs

- **Decisions.** D1 (position unit) and D2 (line breaks) are recorded in `docs/parity/decisions.md` before phase P2. Crate-specific decisions (D3 for language, D4/D7/D8 for view) may still be open: the audit collects their evidence and states the dependent behaviors without assuming an answer.
- **Lower crates.** For each Ferrum crate the audited crate depends on: its approved conformance report (`docs/parity/<crate>/conformance.md`) and its public API. If a lower crate is not implemented yet, use its approved specification (`inventory`, `dispositions`, `behaviors`).
- **Read before starting (main session):** this brief, the scope sheet, `docs/parity/decisions.md`, `docs/parity/_cross-crate.md`, the crate's `progress.md` if it exists (it names the current phase and the sections to re-read after a compaction, `rules.md` §7.2). Subagents read only their card (Appendix A); `rules.md` reaches them through `AGENTS.md`.

---

## 2. Ground rules

1. **cm6's code is the specification.** Its README and docs are secondary; when they disagree, the code wins.
2. **Purpose filter.** Ferrum is a native, headless engine (`ferrum-foundation.md` §1–2). Do not make something `CORE` only because cm6 has it, and do not `DROP` it only because cm6 implements it with the DOM: ask what it does for a native editor.
3. **Missing DOM is not an excuse.** Every `DROP` needs an item-specific reason. If the *behavior* matters natively (input semantics, composition, selection drawing, scrolling, measuring), it is `CONTRACT` or `SURFACE`.
4. **The specification is decision-neutral.** For `CONTRACT` items, state the question asked and what the logic does with the answer. Rust shapes (types, traits, modules) appear only in the implementation sketch (§16), marked as proposals.
5. **Do not decide foundational questions**, approve your own dispositions, or invent justifications. Unresolvable points go to `human-decisions.md` with their options.
6. **Do not read the existing Ferrum crate being audited**, its tests or its docs.

---

## 3. Deliverables

All in `docs/parity/<crate>/`. The JSON Lines files under `raw/` and `fixtures/` are the source of truth; the Markdown files with the same content are rendered from them with `docs/parity/tools/render.py`. Schemas: §11.2.

```
docs/parity/<crate>/
  progress.md              totals, chunks, state, tokens per chunk, open questions
  subagent-card.md         the card given to every subagent (Appendix A, filled in)
  raw/inventory.jsonl      A1: one row per cm6 export and public member, with disposition
  raw/behaviors.jsonl      behaviors B1…Bn per in-scope item
  raw/tests.jsonl          T1: one row per cm6 test case
  raw/xc.jsonl             capabilities needed from lower crates (facts)
  fixtures/<chunk>.jsonl   golden values produced by cm6
  oracle/cm6/<chunk>/      probe and generator sources (no generated data)
  inventory.md, behaviors.md, tests.md     rendered views of the raw files
  dispositions.md          per export and per source file: disposition and reason (rendered + reasons per file)
  contracts.md             per CONTRACT item: the question, the logic around it, the fake needed
  hazards.md               JS→Rust hazards per item
  implementation-plan.md   the implementation sketch (§16)
  human-decisions.md       questions for the human, with options
  verification.md          the verification pass (§12)
```

Workspace-wide, shared by all audits: `docs/parity/_cross-crate.md` (register of capabilities between crates) and `docs/parity/decisions-evidence.md`.

| Deliverable | Used by |
|---|---|
| `inventory`, `dispositions` | Gate 1; implementation scope; conformance G1 |
| `behaviors`, `fixtures` | implementation (what to build, derived tests); conformance G2, G4 |
| `tests` | implementation (tests to port first); conformance G3 |
| `contracts` | implementation of contract-first items; decisions D3/D4/D8 |
| `hazards` | implementation (hazard tests); conformance G7 |
| `oracle/cm6/` | conformance G5 (the Ferrum side is written during implementation) |
| `implementation-plan` | implementation; Gate 2 (obvious decisions approved in bulk) |
| `human-decisions`, `decisions-evidence` | the human, before implementation |

---

## 4. Phases, gates, and who does what

| Phase | Who | Work | Output |
|---|---|---|---|
| P0 Setup | main session | build the shared harness (§7); run the inventory tools; declare totals; split the package into chunks; write the subagent card | `progress.md`, `raw/inventory.jsonl`, `raw/tests.jsonl` (ids and locations only), `subagent-card.md` |
| P1 Dispositions | main session; for packages too large to read in one context, one subagent per group of source files reports facts (what each export does, what it uses from the DOM, who uses it), and the main session decides | disposition per export and per file; first-version scope (`DEFER` list, D10) | `dispositions.md`, `raw/inventory.jsonl` with dispositions |
| **Gate 1** | human | approves dispositions and the `DEFER` list | |
| P2–P3 Chunks | one subagent per chunk, at most 4 at a time | behaviors, cost class, consumer values, hazards, fixtures, contract questions, test catalog rows of the chunk | `raw/behaviors.jsonl`, `fixtures/<chunk>.jsonl`, `raw/tests.jsonl` rows, `raw/xc.jsonl` rows, `oracle/cm6/<chunk>/` |
| P4 Synthesis | main session | merge and render; `contracts.md`, `hazards.md`, cross-crate check, decision evidence, `human-decisions.md`, implementation sketch | the remaining deliverables |
| P5 Verification | one fresh subagent that wrote no rows | §12 | `verification.md` |
| **Gate 2** | human | approves the specification, the sketch, and the list of obvious decisions in one decision | implementation may start |

**When the main session uses subagents:** for work bounded by a chunk of sources (P1 facts for large packages, P2–P3) and for verification (P5). Everything that needs the whole picture stays in the main session: dispositions, scope, contract grouping, cross-crate entries, decision evidence, the sketch, rendering, the gate summaries. Subagents report facts; they do not decide dispositions, cross-crate status or decisions.

P2–P3 cover only items whose approved disposition is `CORE`, `CONTRACT`, `SURFACE` (the core-owned part) or `ELSEWHERE` pointing to this crate, and that are not `DEFER`.

---

## 5. Inventory and dispositions (P0–P1)

**Inventory (A1).** One row per top-level export reachable from the package's `src/index.ts` (class, interface, type, enum, function, const), per public instance and static member (methods, getters, fields, constructor parameter properties), per enum member, and per field of every non-exported object type that an exported function or method takes as a parameter (config objects such as a `define` spec: their fields carry behavior). `private` and `@internal` members are excluded from the rows but read as helpers. Generate the rows with `docs/parity/tools/cm6_inventory.cjs` and the test rows with `docs/parity/tools/cm6_tests.cjs` (`docs/parity/tools/README.md`); compare with the package README symbol list and report every discrepancy. Declare the totals in `progress.md` before any other work.

**Dispositions** (one per row and per source file; proposals until Gate 1):

| Disposition | Meaning |
|---|---|
| `CORE` | Platform-independent logic; belongs in this crate with behavioral parity. |
| `CONTRACT` | Logic belongs here but needs geometry, input or syntax that cm6 gets from the DOM or Lezer. Name the question. |
| `SURFACE` | Drawing, measuring or OS integration belongs to a surface crate. Name the model or facet this crate still owns. |
| `ELSEWHERE: <crate>` | Belongs to another Ferrum crate. |
| `DEFER` | Needed by a native editor but not in the first version (D10). Reason and who needs it. |
| `DROP` | Exists only because of the DOM, CSS, browsers or the TypeScript type system. Item-specific reason. |

---

## 6. Behavior specification (P2)

For every item in scope:

1. Read the whole cm6 function and, transitively, every helper it uses (list them with `file:line`).
2. List its observable behaviors as `B1…Bn`, ids `<crate>.<item>.B<n>`, stable once published: one per branch, guard, early return, edge case and side effect. Each states its input class, observable result, cm6 `file:line`, decision dependencies (`D1`, `D2`, …) and the cm6 tests that cover it (or `none`).
3. Produce **golden fixtures** (§6.2) for every behavior that can run headless.
4. State the item's **cost class** as cm6 implements it (O(1), O(log n), local to a line, range or chunk, linear in the document) and what cm6 avoids (for example materializing the document as a string).
5. List the item's **hazards** (§9) and **consumer values** (§6.3).
6. For `CONTRACT` items: the question in cm6 terms (for example: "`moveVertically` asks for the screen coordinates of the head and for the position at (x, y ± line height)"), what the logic does with each answer, and the smallest fake that reproduces the cm6 tests (fixed-width cells, fixed line height, hand-built syntax tree).

Dimensions to consider (add whatever the code shows): return value including "did nothing"; resulting selection (anchor/head, `assoc`, goal column, bidi level, main index, ordering, merging); generated changes; transaction metadata (user events, `scrollIntoView`, effects, annotations, filters, time); configuration consulted (read-only, tab size, indent unit, line separator, language data, facets, precedence); boundaries (empty document, document start/end, empty line, whitespace-only line, last line without break); text semantics (position unit, separators, grapheme clusters, ZWJ, combining marks, surrogate pairs, CRLF, RTL, tabs, word categories); layout dependencies (wrapping, folds, widgets, atomic ranges, viewport, direction); syntax dependencies; identity (does any caller compare results with `===`?); interaction with other packages.

### 6.1 Behaviors without a headless entry point

Some behaviors need a DOM (view) or a grammar (language). Describe their inputs in terms of the fake from `contracts.md` and take expected values from cm6 test assertions where they exist. If a headless browser runner is available in the harness, `webtest-*` scenarios may be run to produce golden values (record the browser version). Otherwise set `golden_note` to the reason (for example `needs DOM`).

### 6.2 Golden fixtures

A fixture is one cm6 run, written with `fixtureWriter` from `@harness/lib`: `{id, item_id, behaviors, input, output, encoding}`.

- Produced by running cm6 in the harness, never written by hand. The recorded input is the input actually used.
- Positions and lengths as cm6 produces them (UTF-16 code units); the text is stored too, so positions can be mapped to the unit chosen in D1.
- For every behavior that depends on D1 or D2: at least one ASCII fixture and one with astral characters, CRLF or U+2028 as relevant.
- Exceptions are outputs (`capture` records the error class and message).
- Strings must be representable in Rust: `encode` turns strings with lone surrogates into arrays of UTF-16 code units.

### 6.3 Consumer values

For every item other cm6 packages use, grep those packages for the constants and value ranges they pass (for example decoration sides, precedence levels, large positions, `minPointSize`), and include them in the behaviors and fixtures. Values from consumers are often far outside the values of the package's own tests.

---

## 7. Shared harness

The main session builds the harness once, in P0. First the sources: `python3 docs/parity/tools/fetch_mirrors.py --check`. A package the audit reads that the manifest does not list yet (a feature crate's package, a `lang-*` package its tests load) is pinned first, at the release the audit specifies: `python3 docs/parity/tools/fetch_mirrors.py --pin cm6/<package> refs/tags/<version>`; commit `docs/parity/tools/mirrors.json` before writing any deliverable that cites it (`docs/parity/tools/README.md`, section Mirrors). Then:

```sh
python3 docs/parity/tools/harness/setup_cm6.py <package> [--npm <extra-package@version> …]
```

It creates `.harness/<package>/` with the sources of the package and of every `@codemirror/*` package it depends on, their npm dependencies, TypeScript, and a `tsconfig.json` that maps `@codemirror/<name>` to those sources and `@harness/lib` to the shared helpers (`docs/parity/tools/harness/oracle-lib.ts`: seeded PRNG, exception capture, Rust-safe encoding, fixture writer). Extra packages (Lezer grammars, `w3c-keyname`) go after `--npm`. Record in `progress.md` the mirror commits (`.mirror-commit` of each package) and the versions from `harness.json`.

Every subagent uses the same harness and runs its own probes: `tsx --tsconfig .harness/<package>/tsconfig.json <probe>.ts`. Each writes only inside its own `oracle/cm6/<chunk>/` and `fixtures/<chunk>.jsonl`, so parallel subagents never conflict and no queue is needed. Subagents do not install packages or create other environments; a missing dependency is reported to the main session, which adds it to the harness.

---

## 8. Generators and the shape checklist

For every pure item (state-only, or reachable through a fake), write a seeded generator whose outputs are fixtures. Its header states the seed, the case count and how it covers each entry below (or why an entry does not apply):

- empty input, single element, many elements;
- equal and duplicate elements (the same range twice; the same object reused twice);
- adjacent, overlapping and nested elements; unsorted input;
- boundaries: 0, document end, end of line, positions just inside and outside a range;
- consumer values (§6.3) and extreme values of every numeric parameter;
- ASCII, BMP non-ASCII, astral characters, CRLF, lone CR, U+2028 (where text is involved);
- at least one case 100× the typical size (for the conformance scaling check).

---

## 9. JS→Rust hazards

For each item, list which of these its cm6 implementation relies on, with `file:line`. Each listed hazard gets a test during implementation and a check in conformance.

- object identity (`===`, `indexOf`, `Map`/`WeakMap` keys) used for equality, deduplication or change detection;
- lazily computed and cached getters; re-entrant access during a computation;
- mutation of shared objects, aliasing;
- exceptions (`RangeError`, `Error`) and their messages; `undefined`/`null`/`NaN` results;
- JS numbers: values that do not fit a narrow Rust integer, negative or fractional values, `Infinity`, `-1` sentinels;
- integer-keyed object iteration order (ascending), `Array.sort` stability, `Map` insertion order;
- UTF-16 string operations (`length`, `charCodeAt`, `slice` inside a surrogate pair), regex `\p{…}` and the `u` flag, `Intl.Segmenter`;
- module-level counters and global state, `Date.now()`;
- prototype dispatch, `instanceof`, duck typing, union-typed parameters, optional fields with defaults;
- closures capturing mutable state.

---

## 10. Tests, cross-crate capabilities, decision evidence (P3–P4)

**Test catalog (T1).** Every `it()` in the package's `test/` directory, **including `webtest-*.ts`**, id `<file>#<nn>`; loop-generated cases expanded to one row each. Per case: scenario (document, selection, configuration and extensions, languages loaded), action, expected result, behaviors covered, and classification:

- `LOGIC-ONLY`: depends only on state; ports as is.
- `NEEDS LAYOUT`: wrapping, widgets, folds, bidi, coordinates; ports against the fake (name its parameters).
- `NEEDS GRAMMAR`: uses a Lezer grammar; name it and list which expected values depend on it.
- `DOM-ONLY`: asserts DOM structure only; give the reason and record the *semantic* the case pins down (for example decoration nesting order), which may be portable even when the assertion is not.

Port notes: helpers used, values that change under the D1/D2 decisions (computed from the fixture text), loops and randomness to reproduce.

**Helper specifications.** For every cm6 test helper (for example `mirrors/cm6/commands/test/state.ts` `mkState`: `|` for a cursor, `<…>` for a range, `allowMultipleSelections` on), its exact parsing and assertion rules, so the Ferrum helper can mirror it.

**Derived tests.** Every behavior without a cm6 test gets a derived-test specification: behavior id, input and expected value from a fixture.

**Cross-crate.** For every capability an item needs from a lower crate (cite the cm6 call site), check the lower crate's public API or approved specification. If it is absent or differs, the main session opens an entry in `docs/parity/_cross-crate.md` owned by the lower crate, with a fixture showing the expected behavior; the dependent items wait for it. Capabilities of this crate that higher cm6 packages use are listed too, for the later audits.

**Decision evidence.** For each open decision touched, the main session appends to `docs/parity/decisions-evidence.md`: items affected, examples (`file:line`, fixture ids), what each option implies.

---

## 11. Orchestration and cost

Quality comes first: the steps that produce evidence (reading every helper, one behavior per branch, running cm6 for fixtures, the shape checklist, verification) are never shortened to save tokens. Cost is reduced by not repeating work and by keeping contexts small:

- **One harness, one helper library** (§7): no subagent builds its own environment, PRNG, serialization or comparator.
- **One card** per crate (Appendix A) instead of the whole document set for every subagent.
- **Each fact written once.** A reproduction is a fixture; rows cite the fixture id instead of repeating input and output.
- **Small console output.** Tools and probes write results to files and print counts; long outputs are read only where needed.
- **Small main context.** The main session reads the JSONL files, `progress.md` lines and the subagents' short final reports; it does not read the sources a subagent owns.
- **Chunk size from measurement.** Start with about 40–80 inventory rows or up to 1,000 cm6 source lines per chunk; record each chunk's token use in `progress.md` and adjust for the next chunks. Very small chunks waste the fixed cost of reading the card and the sources' context; very large ones fill the subagent's context.
- **At most 4 subagents in parallel**, so that a usage limit or failure interrupts few at once.
- **Incremental output.** A subagent appends each finished item to its files and keeps `done`/`remaining` in `raw/<chunk>.progress`; an interrupted subagent resumes from there.
- **Short final reports** (counts, unfinished ids, the five most important findings with ids).
- **No boilerplate.** A reason shared by several rows is written once as a group entry and referenced; it must be true of every member.

### 11.1 References

Deliverables follow `rules.md` §8. In addition, for audit deliverables:

1. Every deliverable starts with a reading header: what the file is, the cm6 package version and Ferrum commit its line numbers refer to, the id schemes it uses.
2. The harness directory appears only in `subagent-card.md` and `progress.md`, which describe the environment. Scripts a reader needs are under `oracle/cm6/`.
3. Inside a table whose evidence column names a file, a short id refers to that file (`F0042` in a row citing `docs/parity/view/fixtures/C3.jsonl`).
4. The main session runs the reference checker (`rules.md` §8.7) at the end of each phase.

### 11.2 Schemas

One JSON object per line; `[]` marks arrays.

- `raw/inventory.jsonl`: `id, item, kind, file, line, chunk, disposition, reason`
- `raw/behaviors.jsonl`: `id, item_id, text, cm6, deps[], hazards[], consumer_values[], cost, cm6_tests[], fixtures[], golden_note`
- `raw/tests.jsonl`: `id, file, line, describe, name, generated, scenario, expected, behaviors[], class, class_reason, port_notes`
- `fixtures/<chunk>.jsonl`: `id, item_id, behaviors[], input, output, encoding`
- `raw/xc.jsonl`: `capability, cm6, needed_by, searched, exists`

---

## 12. Verification (P5)

A fresh subagent that wrote no rows:

1. **Mechanical checks:** totals recounted with the inventory tools; no duplicate ids; every in-scope item has behaviors; every behavior has fixtures or a `golden_note`; every generator, re-run with its recorded seed, reproduces its fixtures exactly; every test case has a classification; repeated reason texts are listed and checked against each member; `check_refs.py` clean.
2. **Sample:** at least 10% of the in-scope items (minimum 20), chosen with a recorded seed, plus every `CONTRACT` item. For each: re-derive the behavior list from the cm6 source and look for missing behaviors; re-run three fixtures; check the test classifications.
3. Mismatches are fixed by the main session and the sample is extended with a new seed until a sample comes back clean.

---

## 13. Completeness rules

- Declare totals first; every table has exactly that many rows.
- One item, one row. No "etc.", "similar to above", "same as previous".
- Save after each item; report "n of N done" in `progress.md`; list unfinished items explicitly. An unlisted gap counts as a falsified result.
- A row without a cm6 `file:line` is invalid.

---

## 14. Gate deliverables

**Gate 1:** totals, per-disposition counts, the `DEFER` list with reasons, the draft list of contract questions.

**Gate 2:** items in scope; behaviors (count, with and without cm6 test, with and without fixtures); fixtures per chunk; test cases per classification; contract questions grouped (input to D3/D4/D8); hazard summary; open cross-crate entries in both directions; decision evidence added; `human-decisions.md`; the implementation sketch with its list of obvious decisions and the native opportunities with their behavior impact; verification result; confirmation that every source and test file of the scope sheet is covered. Then stop.

---

## 15. Do not

- read, test or describe the existing Ferrum code of the crate being audited
- audit READMEs, doc comments, commit messages or other claims
- write Ferrum code or tests
- present a sketch as decided, or let a Rust shape change a specified behavior
- hand-write golden values
- skip a source or test file because it is large or a `webtest`
- port DOM machinery into the specification because cm6 has it, or drop native-relevant behavior because cm6 implements it with the DOM
- decide foundational questions or invent justifications

---

## 16. Implementation sketch (P4)

The crate is new, so the audit ends with a first idea of how it will be built in Rust. It is a sketch for the implementer and the human, not a design approval: contract traits are approved separately (`2-parity-implementation.md` §3.1), and nothing in the sketch may change a behavior. The main session writes it after P2–P3, from the specification and the hazards.

### 16.1 Shape of the crate

- **Module map.** One Rust module per cm6 source file in scope (`src/module.rs`), with its public items and what stays `pub(crate)`. cm6 `@internal` and non-exported mechanisms stay private by default.
- **Core types and invariants first**: which types everything else uses, their invariants, and how they are kept (constructors, private fields).
- **Rust shapes for cm6 idioms**, per item group, chosen with the hazards of §9:

| cm6 idiom | Usual Rust shape | Watch for |
|---|---|---|
| object literal with optional fields | struct with `Default`, or builder | defaults equal cm6's |
| union-typed parameter | enum, `impl Into<…>` | every union member stays expressible (nesting, mixed lists) |
| class hierarchy, `instanceof`, duck typing | enum for closed sets, trait for open sets | open construction must not break invariants |
| identity (`===`, `indexOf`, `Map` keys) | `Arc` + `Arc::ptr_eq`, or ids allocated per configuration | value equality is not identity |
| lazily cached getter | `OnceLock`/`OnceCell` | computed once; re-entrant access as in cm6 |
| exceptions | `Result`, or a documented panic (`rules.md` §6.4) | the error contract decided per precondition |
| JS `number` | integer width chosen from consumer values; `f64` for geometry | narrow types truncate |
| callback with early stop | iterator, or `FnMut -> ControlFlow` | keep the stop semantics |
| module-level counter, `Date.now()` | per-instance ids, injected clock | determinism in tests (D6) |
| strings and UTF-16 offsets | rope slices, positions in the D1 unit | no whole-document `String` (`rules.md` §6.3) |

- **Ownership and threading** per D6: what is shared (`Arc`), what is cloned, what needs `Send + Sync`, where structural sharing is part of observable behavior.
- **Error contract** per precondition, aligned with the answers in `human-decisions.md`.
- **Dependencies**: candidate crates with license and reason (`rules.md` §5.5); foundation crates stay headless (`rules.md` §2.2).
- **Order of implementation**: types and invariants, then items in cm6 source order; contract-first items get trait and fake first.
- **Test plan**: port order, the fakes of `contracts.md`, the Ferrum fixture replayer and oracle drivers that the conformance audit runs.

### 16.2 Obvious decisions

Decisions with one reasonable answer, listed as `OBVIOUS-<n>: <decision> — <reason>` and approved by the human as one list at Gate 2. Typical entries:

- names are the snake_case of cm6 names; methods stay on the type that has them in cm6;
- `Debug` on every public type; `Default` where cm6 has a default value (returning exactly that value); `is_empty` next to `len`; `Default` for types with `new()`;
- builders or `Default` structs stand in for object literals; `From`/`Into` stand in for implicit union conversions;
- `Iterator`/`IntoIterator` where cm6 iterates with a callback and the stop semantics allow it;
- cm6 `@internal` items are `pub(crate)`.

Anything with two reasonable answers, or with an observable difference, goes to `human-decisions.md` instead.

### 16.3 Native opportunities

Places where a native Rust engine could do better than cm6, which often works around single-threaded JavaScript, the garbage collector or the DOM. For each: what cm6 does (`file:line`), what Rust could do, the expected benefit, and its **behavior impact**:

- **none** (identical observable behavior, for example a better data structure or an avoided copy): the implementer may adopt it, with a proof test;
- **changes behavior** (for example parsing off the main thread instead of time-budgeted parsing, a typed style model instead of CSS class strings): a human decision, listed in `human-decisions.md`.

Benefits are hypotheses until measured (`rules.md` §4.4): give the measurement that would confirm each.

---

## Appendix A. Subagent card (template)

The main session copies this into `docs/parity/<crate>/subagent-card.md`, fills in the bracketed parts, and gives each subagent the card plus its chunk assignment. Keep it under about 150 lines.

```
# Subagent card: [crate] specification audit

You audit one chunk of cm6 [package] [version] to produce the specification of the new Ferrum crate [crate].
Audit only: write no Ferrum code; never read crates/[crate]/.
Binding rules: docs/fundation/rules.md (loaded through AGENTS.md; if it is not in your context, read it first).
This card adds to those rules and never relaxes them; if the two disagree, stop and report.

## Your chunk
[chunk id]: sources [mirrors/cm6/... files or export ranges]; tests [test files or case ids].
Items you own: [inventory ids, with the total]. Approved dispositions: [in raw/inventory.jsonl].

## Environment (already set up; do not install anything)
Run probes: tsx --tsconfig .harness/[package]/tsconfig.json <file>.ts
Import cm6 as "@codemirror/[name]" and the helpers as "@harness/lib" (rng, capture, encode, fixtureWriter).
Write only inside docs/parity/[crate]/oracle/cm6/[chunk]/, fixtures/[chunk].jsonl and your raw rows.
Missing dependency: report it, do not install it.

## For every item you own
1. Read the whole cm6 function and every helper it uses.
2. Behaviors B1..Bn (one per branch, guard, early return, edge case, side effect): id [crate].<item>.B<n>,
   input class, observable result, cm6 file:line, deps (D1, D2, ...), cm6 tests or none.
3. Fixtures for every behavior that runs headless (fixtureWriter; never hand-written; exceptions via capture;
   ASCII plus astral/CRLF/U+2028 variants when the behavior depends on D1/D2).
4. Cost class, hazards (list below), consumer values (grep [consumer packages] for values they pass).
5. CONTRACT items: the question asked, what the logic does with the answer, the smallest fake.
[Generators: seeded, header lists the shape checklist coverage: empty/single/many; equal and duplicate
 elements; adjacent/overlapping/nested; unsorted; boundaries; consumer and extreme values;
 ASCII/BMP/astral/CRLF/CR/U+2028; one case 100x typical size.]
[Hazards: identity; cached getters and re-entrancy; shared mutation; exceptions and undefined/NaN;
 JS numbers vs narrow integers; key and sort order; UTF-16 string ops, regex \p, Intl.Segmenter;
 global counters and Date.now; instanceof and unions; closures with mutable state.]

## For every test case you own
Scenario, action, expected, behaviors covered, class (LOGIC-ONLY / NEEDS LAYOUT / NEEDS GRAMMAR / DOM-ONLY
with reason and the semantic it pins down), port notes.

## Output (append each finished item immediately; keep raw/[chunk].progress with done/remaining)
raw/behaviors.jsonl, raw/tests.jsonl, raw/xc.jsonl rows; schemas: [paste §11.2].
References as rules.md §8: paths from the repository root (mirrors/cm6/..., crates/..., docs/...), file:line with the symbol, ids with their file.
Facts only: do not decide dispositions, cross-crate status or decisions. No "etc.", one item per row.

## Final report (short)
Counts per file, unfinished ids (or "none"), the five most important findings with ids, tokens if known.
```
