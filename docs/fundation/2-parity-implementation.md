# Implementation of a specified Ferrum crate

This brief implements a crate from its approved specification: `1-parity-audit.md` (new crates: `ferrum-view`, `ferrum-language`, feature crates) or `1d-scope-commands.md` (`ferrum-commands`, with its quarry). The result is checked by `3-conformance-audit.md`.

For `ferrum-state` use `2a-implementation-state.md`: its audit followed the previous brief, so its specification must first be completed, and it has crate-specific rules. Everything below applies to it as well unless `2a` says otherwise.

Test, annotation, error-contract and commit rules are in `rules.md` (§3, §6); this brief does not repeat them. Paths are relative to `/workspace/`; instruction documents named alone live in `ferrum/docs/fundation/` (`ferrum/docs/parity/README.md`).

---

## 0. Preconditions

Do not write code for a crate until:

1. its specification passed **Gate 2** (`1-parity-audit.md` §4): dispositions, behaviors, tests, contracts, hazards and the implementation sketch approved, and the list of obvious decisions (`1-parity-audit.md` §16.2, kept in the crate's `implementation-plan.md`) approved in one decision;
2. every decision the crate depends on is recorded in `ferrum/docs/parity/decisions.md` (the behaviors list their `deps`), and the crate's `human-decisions.md` is answered;
3. the crates it depends on have an approved conformance report for the capabilities it needs. A capability still open in `ferrum/docs/parity/_cross-crate.md` makes the items that need it `BLOCKED`: they are not approximated.

**Read before starting (main session):** this brief, `ferrum/docs/parity/decisions.md`, `ferrum/docs/parity/README.md`, the crate's specification folder `ferrum/docs/parity/<crate>/` (start with `progress.md`, `implementation-plan.md`, then `contracts.md`; read `behaviors.md`, `tests.md` and `hazards.md` as counts and summaries, since the subagents read the rows of their module), and `ferrum/docs/parity/_cross-crate.md`. Subagents read only their card (Appendix A).

---

## 1. Order across crates

Bottom-up along the dependency graph: `ferrum-state`, `ferrum-view`, `ferrum-language`, `ferrum-commands`; feature crates after the foundation. A crate is implemented only after the crates below it are conformant for what it needs.

Surfaces (`ferrum-view-gpui`, later `ferrum-view-wgpu`) and the facade are updated after the foundation crate they consume is conformant, only enough to compile and keep their tests meaningful. Surface work is a separate phase.

---

## 2. Principles

1. **The specification is the work list.** Implement the approved in-scope items, nothing else (`rules.md` §6.9). A behavior found while implementing that the specification missed is added to `behaviors.md` with its cm6 `file:line` and a fixture, flagged `added during implementation` so that the conformance audit checks it, and gets a test by the same route as every other test (§4, step 2).
2. **Nothing diverges silently.** A function that carries a cm6 name but behaves differently is worse than a missing one. Either it behaves like cm6, or the difference is an approved decision, annotated as `rules.md` §3.6 states.
3. **Every public item names its counterpart** (`rules.md` §3.6), so the mapping can be found with grep.
4. **Approved decisions are law.** Position unit, line breaks, syntax abstraction, layout contract, command type, threading, style model come from `decisions.md`. If an item cannot be implemented under them, stop and report.
5. **Idiomatic Rust is welcome when behavior is identical.** The sketch's native opportunities with "no behavior impact" may be adopted, with a proof test; anything that changes observable behavior needs an approved decision.
6. **No approximations.** An item that is `BLOCKED`, `DEFER` or `DROP` is documented, not approximated under a cm6 name.
7. **No invented justifications.** A difference without a concrete, verifiable reason is reported as a question.
8. **Purpose filter.** No DOM concepts because cm6 has them; no surface concerns (fonts, shaping, windows, GPU) in foundation crates.
9. **Tests are written by a context that has not seen the implementation**, and the implementation never edits them (§4).

---

## 3. Starting the crate

- Start from an empty module tree in the existing crate directory, following the module map of `implementation-plan.md`. The previous code of the crate stays readable in git history; it is not read for the implementation.
- Exceptions, and only these: rows of a quarry (`ferrum/docs/parity/commands/quarry.md`, `ferrum/docs/parity/view/quarry.md`) with verdict `REUSE` or `ADAPT` may be copied and adapted; tests with verdict `KEEP` or `FIX` may be reused after the stated fix. Cite the quarry row in the commit message.
- Types and invariants first (what everything else uses), then modules in the order of `implementation-plan.md`, items in cm6 source order, so that reviewers can read both side by side.

### 3.1 Contract-first items

Items whose disposition is `CONTRACT` need a trait before code: the layout contract (D4) in `ferrum-view`, the syntax abstraction (D3) in `ferrum-language`, the command/view interface (D5) in `ferrum-commands`.

1. Write the trait from `contracts.md` (and the sketch), with doc comments that list, for each method, the cm6 call sites it replaces (`file:line`).
2. Write the deterministic fake that `contracts.md` names (fixed-width cells and fixed line height; hand-built syntax trees).
3. Get human approval of the trait and the fake before implementing items on top of them.
4. Port the logic on top of the trait and the cm6 tests classified `NEEDS LAYOUT` / `NEEDS GRAMMAR` against the fake or the D3 grammar, through the workflow of §4.

---

## 4. Workflow, module by module

The unit of work is a **module**: one cm6 source file in scope, or the group of files that `implementation-plan.md` treats as one module. Each module goes through four steps, split by role so that the tests come from the specification and not from the code that must pass them.

| Step | Who | Work | Done when |
|---|---|---|---|
| 1. Skeleton | main session | The module's public items from `implementation-plan.md`: types, private fields, signatures with `todo!()` bodies, annotations (`rules.md` §3.6). It compiles. | committed (`feat(<crate>): <module> skeleton`) |
| 2. Tests | test subagent, fresh context | For the module's rows of `tests.md`, `behaviors.md`, `hazards.md` and its fixtures: every cm6 case ported (`rules.md` §3.1–3.4); a derived test, from its fixture, for every behavior without a cm6 test; a test for every listed hazard (identity, caching, consumer values, error path, ordering); a scaling probe (n and 10n) for every item whose cost class is local or logarithmic. Every test compiles and fails at a `todo!()`. | committed (`test(<crate>): <module>`); report lists counts, behaviors without a test (must be none), tests that pass against the skeleton (vacuous or testing a default: each is fixed or explained), questions |
| 3. Implementation | implementer: the main session for a small module, otherwise a subagent | Implement until every test of the module passes, following the cm6 algorithm and every behavior the cm6 code shows (§2.1). Never edit a test file: a test that looks wrong is reported to the main session, which decides with the test subagent or the maintainer. Run the fixture replayer for the module (Ferrum side kept in `ferrum/docs/parity/<crate>/oracle/ferrum/`): 0 mismatches. Run the scaling probes: a superlinear result is a defect, not a performance note. `cargo test --workspace` and `cargo clippy --workspace` with zero warnings. | committed per item or tightly coupled group, messages citing behavior ids; module rows in `ferrum/docs/parity/<crate>/implementation.md` (item done, tests, fixtures, open points) |
| 4. Check | main session | From the reports and `git`, not from the sources: every behavior id of the module is named by a `// covers:` line; the test files are unchanged since the step-2 commit (`git diff <commit> -- <test files>` is empty, or each change was approved and is listed); counts recorded. | `progress.md` updated: module done, tokens used per step |

- **Parallelism.** The test subagent of the next module may run while the current module is implemented, within the limit of 4 subagents at a time (`1-parity-audit.md` §11). A module whose types are still changing waits.
- **Small modules** may be grouped into one step-2 run and one step-3 run; the roles stay separate.
- **Calibration.** Record tokens per step and per module in `progress.md`. Adjust grouping from those numbers; never drop a test category to save tokens (`LEARNINGS.md` principle).
- **Report after each module:** done, skipped and why, behaviors added, register entries opened or closed.

**Unblock pass.** When a lower crate closes a register entry, return to every item that references it and run its module's steps for that item. Before ending a session, list every open register entry that references the crate.

---

## 5. Tests

- No wall-clock time or randomness without a recorded seed; time-dependent behavior (history grouping, `Transaction.time`) uses explicit times.
- Test helpers are specified in `tests.md` and mirror cm6's (`rules.md` §3.4).
- Expected values that differ from cm6 for an approved cause (D1 unit translation, D2 separators, D3 grammar) are computed from the fixture text, with the cause next to the assertion (`rules.md` §3.2).
- Oracle drivers: write the Ferrum side of every generator in `oracle/` as you go; the conformance audit runs them at full size (`3-conformance-audit.md` G5).

---

## 6. Documentation

During implementation, only the mapping annotations (`rules.md` §3.6) and the doc comments needed to use the API. The crate README, `src/README.md`, root README and facade examples are rewritten **after the conformance report is approved**: parity status as the report states it, a table of everything not implemented (`DEFER`, `DROP`, `BLOCKED`, approved deviations) findable by cm6 name, test counts only next to the report's numbers, examples that compile (doctests or examples built in CI). Claims are limited to what the report states (`rules.md` §4.1); references follow `rules.md` §8.

---

## 7. Hand-off

The crate is ready for the conformance audit when every in-scope item is implemented or `BLOCKED`, and the implementer's self-check passes: annotation scan (every in-scope item found, every public item annotated), test-name scan (every cm6 case ported), every `// covers:` id exists, test files unchanged since their step-2 commits except approved changes, fixture replay at 0 mismatches, reference check clean (`rules.md` §8.7), `cargo test`/`clippy` clean. Then run `3-conformance-audit.md`.

---

## 8. Do not

- implement items outside the approved specification
- read the previous code of the crate, or copy code or tests not marked `REUSE`/`ADAPT`/`KEEP`/`FIX` in a quarry
- write the tests and the implementation of a module in the same context, or modify tests to turn them green
- approximate blocked, deferred or dropped items
- work around an approved decision
- invent justifications
- batch several items into one unreviewable change
- rewrite READMEs before the conformance report is approved

---

## Appendix A. Cards for implementation subagents (templates)

The main session copies these into `ferrum/docs/parity/<crate>/implementation-cards.md`, fills in the bracketed parts, and gives each subagent its card plus its module assignment.

```
# Test card: [crate] implementation, module [module]

Binding rules: ferrum/docs/fundation/rules.md (loaded through ferrum/AGENTS.md; if it is not in your context, read it first).
This card adds to those rules and never relaxes them; if the two disagree, stop and report.

You write the tests of module [module] of [crate] from its approved specification. You write no implementation code
and you do not read [crate source files of the module] beyond the skeleton's signatures.
Read: rows of module [module] in ferrum/docs/parity/[crate]/tests.md, behaviors.md, hazards.md; fixtures [files];
test helper specification in tests.md; the skeleton at commit [commit].
Write: [test files]. Each test: rules.md §3 (same test as cm6; test_cm6_ names; // covers: ids; derived tests marked).
Every behavior gets a test; every listed hazard gets a test; local/log items get a scaling probe (n, 10n).
Run: cargo test -p [crate] [filter]. Every test must compile and fail at a todo!(). A test that passes is reported.
Commit: test([crate]): [module]. Final report: counts per category, behaviors without a test (ids or "none"),
tests passing against the skeleton, questions. Facts only.
```

```
# Implementation card: [crate] implementation, module [module]

Binding rules: ferrum/docs/fundation/rules.md (loaded through ferrum/AGENTS.md; if it is not in your context, read it first).
This card adds to those rules and never relaxes them; if the two disagree, stop and report.

You implement module [module] of [crate] so that its tests pass. Never edit [test files]; a test that looks wrong
is reported, not changed. Read: the cm6 source [CodeMirror6/... files] and every helper it uses; rows of module
[module] in behaviors.md, hazards.md, contracts.md; decisions [ids].
Done when: all tests of the module pass; fixture replayer [command] reports 0 mismatches; scaling probes are not
superlinear; cargo test --workspace and cargo clippy --workspace show zero warnings; implementation.md rows written.
Commit per item or tightly coupled group, citing behavior ids. Final report: items done, items BLOCKED with register
ids, behaviors added during implementation, tests reported as wrong, tokens if known.
```
