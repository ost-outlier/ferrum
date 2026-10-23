# Audit method: learnings and changelog

> What: what each audit taught about the audit method, and how the instruction documents changed because of it. Versions: entries are dated. Ids: `L<n>` (learning), `v<n>` (version of the instruction documents).

This file is **not** required reading to run an audit or an implementation: the instruction documents (`1-parity-audit.md`, the scope sheets, `2-…`, `3-conformance-audit.md`) contain the conclusions as rules. Read this file when you revise those documents, and add an entry after every audit (what happened, measured where possible, and which rule it changed).

Guiding principle for every revision: **token economy never comes before the quality of the result.** Cost is reduced by calibration (removing duplicated work, sizing chunks, keeping contexts small), never by cutting steps that produce evidence.

**Basis of each learning.** Every entry states what it rests on, so that a later revision knows what may be overturned by a measurement:

- `evidence`: observed or measured in a real run or in the repository; the entry names where (files, commits, counts).
- `docs`: stated by the official documentation of a tool the workflow depends on (URL and date read); true for the tool version documented, and re-checked when the tool changes.
- `analysis`: a conclusion from reasoning (often prompted by the maintainer and refined by an agent), not yet confirmed by a run; the entry says what measurement would confirm or refute it.

Entries L1–L21 predate this column; all of them are `evidence` (L1–L19 from the files of `ferrum/docs/parity/state/`, L20–L21 from reading the v2 documents).

---

## Learnings

### From the `ferrum-state` audit (2026-10-21, method v1)

Evidence for every number below is in `ferrum/docs/parity/state/` (`progress.md`, `oracle.md`, `verification.md`, `raw/`).

| Id | Observation | Rule it produced |
|---|---|---|
| L1 | The valuable output was the cm6 side: inventory, dispositions, behavior lists, test inventory, oracle generators. The Ferrum side (391-row public-item matrix, Ferrum test mapping, 107 `FERRUM-ONLY` candidates, reproductions against the old code) described defects of code that is not kept. The method also gave credibility to an "implemented" crate whose tests, named after cm6 cases, often asserted less or something else. | New crates get a specification audit that reads cm6 only (`1-parity-audit.md`); existing code is evaluated only where it is worth salvaging (`1d-scope-commands.md` quarry). |
| L2 | The claims audit (READMEs, doc comments, commits, vision document) cost ≈300k of ≈2.35M subagent tokens (13%). Of 488 claims, 397 were true, mostly doc comments describing Ferrum's own code. Its one lasting finding was licensing (D9). | No documentation audit; documentation is rewritten after the conformance report is approved. |
| L3 | Running cm6 found defects that reading did not: identity-based deduplication of extensions, the main-index bug of `EditorSelection.create`, side values truncated to `i8` in `RangeSet.map`. | Every behavior gets golden fixtures produced by cm6 (`1-parity-audit.md` §6.2). |
| L4 | One random oracle reported 0 mismatches in 5,000 cases on an input class where another oracle found a bug: its generator never produced two equal ranges. | Shape checklist for generators (`1-parity-audit.md` §8); coverage and sensitivity checks of oracles in conformance (G5). |
| L5 | A `RangeSet` defect appeared only with the side values `@codemirror/view` uses (±1e8…±6e8), never with the values of cm6's own state tests. | Consumer values in behaviors and fixtures (`1-parity-audit.md` §6.3). |
| L6 | Many defects came from translating JavaScript semantics: object identity, lazily cached getters (`Transaction.state` recomputed per call, stack overflow on re-entry), global id counters, ascending order of integer keys, JS numbers in narrow integers, exceptions. | JS→Rust hazard list per item (`1-parity-audit.md` §9); hazard gate G7. |
| L7 | `count_column` was quadratic: 56 s on a 40,000-character line in release, against 0.35 ms in cm6. No test caught it. | Cost class per item (`1-parity-audit.md` §6); scaling gate G8. |
| L8 | Of 948 behaviors, 22% were implemented and tested, 14% implemented but untested, 25% different, 33% missing; many behaviors have no cm6 test at all. | Derived-test specifications from fixtures (`1-parity-audit.md` §10); `// covers:` ids in tests (G2). |
| L9 | Subagents produced Markdown tables with varying columns and `\|` inside code; merging needed a custom parser. One justification copied onto 55 rows was false for some of them. | JSON Lines with fixed schemas, Markdown rendered by script; reasons shared by rows are group entries (`1-parity-audit.md` §11). |
| L10 | All seven parallel subagents hit an account usage limit at the same moment; work survived only because it was already in files. | Incremental output per item, resumable; at most 4 parallel subagents (`1-parity-audit.md` §11). |
| L11 | Each subagent read ≈25k tokens of instruction documents before starting. | One condensed subagent card per crate, from a template (Appendix A of `1-parity-audit.md`). |
| L12 | The 10% verification sample found 5 real errors; the extension sample came back clean. | Verification kept, plus mechanical checks (`1-parity-audit.md` §12). |
| L13 | References were written for the producing session: three unstated path roots, short ids (`C1 R9`), scratch paths under `/tmp`, line numbers without symbol or version. Normalizing ≈8,600 references afterwards was a one-off cost. | Reference rules (`1-parity-audit.md` §11.1). The checker (`ferrum/docs/parity/tools/check_refs.py`) is a model-free script whose run prints one line; with the rules followed from the start it is a cheap safety net, run by the main session at the end of each phase. |
| L14 | About 30 `FERRUM-ONLY` rows were obvious items (builders, `Debug`, `Default`, `From`, `is_empty`). | Obvious decisions listed in the implementation sketch and approved in bulk (`1-parity-audit.md` §16.2). |
| L15 | 55 cm6-internal mechanisms were public in Ferrum, several able to break invariants from outside. | cm6 `@internal` stays `pub(crate)` by default (`1-parity-audit.md` §16.1). |
| L16 | Setting up a Node or cargo environment per subagent was cheap in tokens (a copy and a quiet build). The expensive duplication was that each of six chunk subagents wrote its own PRNG, JSON exchange, comparator and Ferrum replayer. | One shared harness and helper library, built once by the main session (`1-parity-audit.md` §7). Execution stays with each subagent: routing probes through an orchestrator would put every probe output into the largest context, and nothing needs a queue (Node runs do not conflict; in conformance audits cargo's file lock serializes builds of a shared target directory). |
| L17 | The main session read all old Ferrum sources and large tool outputs, growing the most expensive context. | The main session reads JSONL summaries and subagent reports, not the sources a subagent owns; tool output goes to files, the console gets counts (`1-parity-audit.md` §11). |
| L18 | Cost calibration: chunk subagents used 240k–320k tokens for 30–77 cm6 items (including the Ferrum-side work now removed); the verification pass 373k. | Record tokens per chunk in `progress.md` and size chunks from it (`1-parity-audit.md` §11). |
| L19 | The status `PORTED, DEVIATION` absorbed 139 of 264 rows and said little about what to fix. | Conformance verdicts are per behavior id, with binary item verdicts (`3-conformance-audit.md` §0). |

### From the review of method v2 (2026-10-22)

| Id | Observation | Rule it produced |
|---|---|---|
| L20 | v2 of the briefs mixed history with rules: a "what the state audit taught" table, anecdotes in gate descriptions, and pointers to the state audit's files as templates and tool locations. An agent had to read the state results to understand what to deliver. | Instruction documents contain only conclusions and self-contained instructions; history and evidence go to this file; shared tools live in `ferrum/docs/parity/tools/`, not inside one crate's results. |
| L21 | v2 no longer said clearly when the main session uses subagents. | Explicit "who does what" per phase (`1-parity-audit.md` §4) and a subagent card template (Appendix A). |

### From the review of the instruction documents against the Claude Code documentation (2026-10-23, method v4)

Documentation read on 2026-10-23: `https://code.claude.com/docs/en/memory`, `https://code.claude.com/docs/en/features-overview`, `https://code.claude.com/docs/en/context-window`, `https://code.claude.com/docs/en/best-practices`, `https://code.claude.com/docs/en/skills`, `https://code.claude.com/docs/en/sub-agents`, `https://code.claude.com/docs/en/interactive-mode`.

| Id | Basis | Observation | Rule it produced |
|---|---|---|---|
| L22 | docs | The "under 200 lines" guidance is for `CLAUDE.md`, which loads in full in every session (and in every non-fork subagent): longer files cost context on every request and reduce adherence. Procedures and reference material belong in files loaded on demand (skills, or documents read when needed), which have no such target. | `rules.md` is the only always-loaded instruction document (through `ferrum/AGENTS.md`, imported by `ferrum/CLAUDE.md`) and stays short; the phase briefs stay complete and are read on demand. The briefs are not trimmed to a line count. |
| L23 | docs | After a compaction, the project-root `CLAUDE.md` is re-injected from disk, but a file read earlier with the Read tool comes back only as a path reference when it is over 5,000 tokens; `1-parity-audit.md` is about 8,000. A long main session silently loses its brief. | `progress.md` names the current phase and the brief sections to re-read; after a compaction they are re-read (`rules.md` §7.2). Sessions start in `/workspace/ferrum` so that `ferrum/AGENTS.md` loads at startup. |
| L24 | docs | When two instructions contradict each other, Claude may pick one arbitrarily. | One precedence list (`rules.md` §0); a conflict is reported, never resolved by choosing. |
| L25 | evidence | Reading v3 found nine real conflicts, all from the same rule written in more than one place: the precedence list (foundation) left out the briefs and put `rules.md` above approved decisions; `rules.md` §3 had its own five-step test cycle that differed from `2-parity-implementation.md` §4; the `/// cm6…` annotation forms differed between `rules.md`, `2-parity-implementation.md` and `3-conformance-audit.md` (the `rules.md` deviation form had no decision id, which G6 needs); `rules.md` §6.4 let the implementer choose panic or `Result` and add non-panicking variants, while the briefs make the error contract a Gate 2 decision; four different reading lists; "parity matrix" (a v1 term) as the bar for claims; a dependency rule that the GPUI surface cannot meet (its transitive MPL-2.0 crates, `ferrum/docs/parity/decisions-draft.md` D9); a truncated sentence (`2-parity-implementation.md` §6); a section that does not exist (`1c-scope-language.md` cited a section 17.3 of `1-parity-audit.md`). | Each rule lives in one place: phase-independent rules in `rules.md`, procedures in the briefs, which point to `rules.md` sections instead of restating them. |
| L26 | evidence | `check_refs.py` reported 0 findings while `1c-scope-language.md` cited a nonexistent section 17.3 and three bare § referred to another document: the checker did not look at section references. | The checker resolves `<file>.md §N.M` and bare § (`rules.md` §8.5): it finds 4 more references, fixed in v4. |
| L27 | analysis | An outside rewrite of `rules.md` (`rules-draft.md`, 2026-10-23) required "exactly one" annotation per item; a deviating item then could not carry its `cm6:` name, and G1 could not find it. | Annotations are one mapping line plus optional qualifier lines, each qualifier citing its approval (`rules.md` §3.6). |
| L28 | analysis | The maintainer asked whether the per-item implementation cycle (`2-parity-implementation.md` v3 §4: nine steps for each of, for `ferrum-state`, 264 inventory items) is too expensive, and whether one agent should write all tests and another implement. Per-item work rereads the same cm6 module and specification rows for every item of the module. Tests written by a context that has not seen the implementation are a quality gain (the Writer/Reviewer and "one Claude writes tests, another writes code to pass them" patterns of `https://code.claude.com/docs/en/best-practices`). Writing the tests of a whole crate first would make the test writer invent Rust signatures, a design decision, and cause rework when they change. No implementation cost has been measured under the v3/v4 method; the audit numbers (L18) are the only calibration. | Module by module, four roles: skeleton (main session), tests (fresh subagent), implementation (never edits tests), check (main session, from reports and `git`); no test category is dropped; tokens recorded per step (`2-parity-implementation.md` §4). To confirm or refute after the first crate: tokens per module and per step, and the number of test changes requested by implementers. |
| L29 | analysis | The maintainer's hypothesis: ending an interaction with "these files are uncommitted, shall I commit?" costs a maintainer turn and its tokens, and leaves work that another session cannot see. Not measured. | Commit without asking, on a working branch, before stopping (`rules.md` §7.6). |
| L30 | docs | Claude Code continues a task automatically after a usage-limit reset only in an open interactive session; background sessions and `-p` runs do not wait (`https://code.claude.com/docs/en/interactive-mode`, "Wait for a usage limit to reset"). This is why long runs needed an open terminal. | No rule in the instruction documents; options are explored in the lab folder `ferrum/docs/flow/`. |

---

## Changelog of the instruction documents

### v4 (2026-10-23)

- `ferrum/AGENTS.md` (new) imports `rules.md`, and `ferrum/CLAUDE.md` (new) imports `ferrum/AGENTS.md`: the rules load in every session and survive compaction.
- `rules.md`: §0 precedence and conflicts (moved from `ferrum-foundation.md`, decisions first); §3 test invariants and the single annotation vocabulary (the test cycle moved to the implementation brief); §4.1 claims bounded by the approved conformance report; §5 licensing reduced to what to read or copy (MIT/Apache-2.0), the GPL quarantine, the cm6 notice and a dependency rule surfaces can meet; §6.4 error contract from the specification; §6.9 no API beyond the specification; §7 reading via the brief, phase and re-read sections in `progress.md`, commit before stopping, the lab folder; §8 references (moved from `1-parity-audit.md` §11.1).
- `2-parity-implementation.md`: module-by-module workflow with separated test and implementation roles (§4), subagent cards (Appendix A), §6 sentence completed, repeated rules replaced by pointers.
- `1-parity-audit.md`: reading list without the always-loaded documents; §11.1 keeps only the audit-specific reference rules; the card states that it never relaxes `rules.md`.
- `3-conformance-audit.md`: reading list; annotation forms and test names by pointer; G2 checks that test files were not changed after their step-2 commit.
- `2a-implementation-state.md`, `1b-scope-view.md`, `1c-scope-language.md`: section references qualified; `1c` §17.3 corrected to `1-parity-audit.md` §16.3.
- `ferrum/docs/parity/tools/check_refs.py`: section check.

### v3 (2026-10-22)

- `1-parity-audit.md`: self-contained; history moved here; deliverables listed with format, producer and consumer; "who does what" per phase; shared harness and tools (`ferrum/docs/parity/tools/`); subagent card template (Appendix A); cost rules (what may be calibrated, what is never cut); the implementation sketch is §16.
- New shared tools: `harness/setup_cm6.py`, `harness/oracle-lib.ts`, `cm6_inventory.cjs` (config objects detected automatically), `cm6_tests.cjs`, `render.py`, `ferrum_pub.py`, `check_refs.py`.
- `2a-implementation-state.md`: decisive work list; the old audit files are read once in Phase 0 and turned into a standard specification folder; the implementer reads only that folder.
- `1b`, `1c`: inputs from lower crates stated as capabilities to check, not as register ids to look up.
- `3-conformance-audit.md`, `ferrum-foundation.md`: anecdotes and audit numbers moved here.

### v2 (2026-10-21 to 2026-10-22)

- `1-parity-audit.md` rewritten as a specification audit for new crates (cm6 only, golden fixtures, shape checklist, hazards, cost class, JSONL raw rows, no claims audit), then extended with reference rules and an implementation sketch.
- `1d-scope-commands.md`: hybrid audit (specification, quarry, feedback to lower crates).
- `3-conformance-audit.md`: new (gates G1–G10).
- `2-parity-implementation.md` rewritten for specified crates; `2a-implementation-state.md` new; `1b`, `1c` rewritten; `1a` marked historical; `ferrum-foundation.md` §5 and §7 updated to the decided approach (rebuild from cm6 specifications; commands as quarry).
- Instruction documents kept only in `ferrum/docs/fundation/`.

### v1 (until 2026-10-21)

The parity audit comparing cm6 with the existing Ferrum code, used for the `ferrum-state` audit. Read it with `git -C /workspace/ferrum show f63225f:docs/fundation/1-parity-audit.md`. <!-- refs:ignore: git path syntax -->
