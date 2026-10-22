# Implementation of a specified Ferrum crate

This brief implements a crate from its approved specification: `1-parity-audit.md` (new crates: `ferrum-view`, `ferrum-language`, feature crates) or `1d-scope-commands.md` (`ferrum-commands`, with its quarry). The result is checked by `3-conformance-audit.md`.

For `ferrum-state` use `2a-implementation-state.md`: its audit followed the previous brief, so its specification must first be completed, and it has crate-specific rules. Everything below applies to it as well unless `2a` says otherwise.

Paths are relative to `/workspace/`; instruction documents named alone live in `ferrum/docs/fundation/` (`ferrum/docs/parity/README.md`).

---

## 0. Preconditions

Do not write code for a crate until:

1. its specification passed **Gate 2** (`1-parity-audit.md` §4): dispositions, behaviors, tests, contracts, hazards and the implementation sketch approved, and the list of obvious decisions (`1-parity-audit.md` §17.2, kept in the crate's `implementation-plan.md`) approved in one decision;
2. every decision the crate depends on is recorded in `ferrum/docs/parity/decisions.md` (the behaviors list their `deps`), and the crate's `human-decisions.md` is answered;
3. the crates it depends on have an approved conformance report for the capabilities it needs. A capability still open in `ferrum/docs/parity/_cross-crate.md` makes the items that need it `BLOCKED`: they are not approximated.

**Read before starting any session:** `ferrum-foundation.md`, `rules.md`, this brief, `ferrum/docs/parity/decisions.md`, `ferrum/docs/parity/README.md`, the crate's specification folder `ferrum/docs/parity/<crate>/` (start with `progress.md`, `implementation-plan.md`, `behaviors.md`, `tests.md`, `contracts.md`, `hazards.md`), and `ferrum/docs/parity/_cross-crate.md`.

---

## 1. Order across crates

Bottom-up along the dependency graph: `ferrum-state`, `ferrum-view`, `ferrum-language`, `ferrum-commands`; feature crates after the foundation. A crate is implemented only after the crates below it are conformant for what it needs.

Surfaces (`ferrum-view-gpui`, later `ferrum-view-wgpu`) and the facade are updated after the foundation crate they consume is conformant, only enough to compile and keep their tests meaningful. Surface work is a separate phase.

---

## 2. Principles

1. **The specification is the work list.** Implement the approved in-scope items, nothing else. A behavior found while implementing that the specification missed is added to `behaviors.md` with its cm6 `file:line` and a fixture, flagged `added during implementation` so that the conformance audit checks it.
2. **Nothing diverges silently.** A function that carries a cm6 name but behaves differently is worse than a missing one. Either it behaves like cm6, or the difference is an approved decision, written next to it (`/// cm6 deviation: <what>, see decisions.md <id>`).
3. **Every public item names its counterpart**, so the mapping can be found with grep: `/// cm6: EditorState.changeByRange`, `/// cm6 contract: EditorView.coordsAtPos`, `/// ferrum-only: <reason>` (approved in the sketch), `/// placement: cm6 has this in <package>` (approved decision).
4. **Approved decisions are law.** Position unit, line breaks, syntax abstraction, layout contract, command type, threading, style model come from `decisions.md`. If an item cannot be implemented under them, stop and report.
5. **Idiomatic Rust is welcome when behavior is identical.** The sketch's native opportunities with "no behavior impact" may be adopted, with a proof test; anything that changes observable behavior needs an approved decision.
6. **No approximations.** An item that is `BLOCKED`, `DEFER` or `DROP` is documented, not approximated under a cm6 name.
7. **No invented justifications.** A difference without a concrete, verifiable reason is reported as a question.
8. **Purpose filter.** No DOM concepts because cm6 has them; no surface concerns (fonts, shaping, windows, GPU) in foundation crates.

---

## 3. Starting the crate

- Start from an empty module tree in the existing crate directory, following the module map of `implementation-plan.md`. The previous code of the crate stays readable in git history; it is not read for the implementation.
- Exceptions, and only these: rows of a quarry (`ferrum/docs/parity/commands/quarry.md`, `ferrum/docs/parity/view/quarry.md`) with verdict `REUSE` or `ADAPT` may be copied and adapted; tests with verdict `KEEP` or `FIX` may be reused after the stated fix. Cite the quarry row in the commit message.
- Types and invariants first (what everything else uses), then items in cm6 source order, so that reviewers can read both side by side.

### 3.1 Contract-first items

Items whose disposition is `CONTRACT` need a trait before code: the layout contract (D4) in `ferrum-view`, the syntax abstraction (D3) in `ferrum-language`, the command/view interface (D5) in `ferrum-commands`.

1. Write the trait from `contracts.md` (and the sketch), with doc comments that list, for each method, the cm6 call sites it replaces (`file:line`).
2. Write the deterministic fake that `contracts.md` names (fixed-width cells and fixed line height; hand-built syntax trees).
3. Get human approval of the trait and the fake before implementing items on top of them.
4. Port the logic on top of the trait and the cm6 tests classified `NEEDS LAYOUT` / `NEEDS GRAMMAR` against the fake or the D3 grammar.

---

## 4. Workflow, one item at a time

For each item:

1. Re-read the cm6 source of the item, its behaviors, hazards, consumer values, cost class and fixtures.
2. **Port its cm6 tests first** (`tests.md`), with identical scenario, expected values and assertions, named `test_cm6_<file>_<nn>_<slug>` (`rules.md` §3). Each test lists what it proves: `// covers: <crate>.<item>.B<n>, …`. Run them and confirm they fail for the expected reason (red).
3. Add the **derived tests** for behaviors without a cm6 test, from their fixtures, marked `// derived from cm6 source: <file:line>` and `// covers: …`.
4. Add a test for every **hazard** listed for the item (identity, caching, consumer values, error path, ordering).
5. Implement so that **every** behavior holds. Do not stop at the listed behaviors when the cm6 code shows more (principle 1).
6. Run the **fixture replayer** for the item (Ferrum side of the fixtures, kept in `ferrum/docs/parity/<crate>/oracle/ferrum/`): 0 mismatches.
7. For items whose cost class is local or logarithmic, run a quick **scaling check** (n and 10n). A superlinear result is a defect, not a performance note.
8. Annotate (principle 3), update `ferrum/docs/parity/<crate>/implementation.md` (item done, tests, fixtures, open points), run `cargo test --workspace` and `cargo clippy --workspace` (zero warnings).
9. One commit per item or tightly coupled group (`rules.md` §6.8), message citing the behavior ids.

Report after each group of items: done, skipped and why, behaviors added, register entries opened or closed.

**Unblock pass.** When a lower crate closes a register entry, return to every item that references it and run steps 1–9. Before ending a session, list every open register entry that references the crate.

---

## 5. Tests

- **A ported test is the same test**: same scenario, expected values and assertions. Never weaken an assertion, change an expected value or add `#[ignore]` to make it pass; a failing ported test is a finding to report.
- An expected value may differ from cm6 only for an approved cause (D1 unit translation, D2 separators, D3 grammar), written next to the assertion, with the translated value computed from the fixture text.
- No wall-clock time or randomness without a recorded seed; time-dependent behavior (history grouping, `Transaction.time`) uses explicit times.
- Test helpers mirror cm6's (`CodeMirror6/commands/test/state.ts`: `|` cursor, `<…>` range) as specified in `tests.md`, and have their own tests.
- Oracle drivers: write the Ferrum side of every generator in `oracle/` as you go; the conformance audit runs them at full size (`3-conformance-audit.md` G5).

---

## 6. Documentation

During implementation, only the mapping annotations (principle 3) and the doc comments needed to use the API. The crate README, `src/README.md`, root README and facade examples are rewritten **after the conformance report is approved**: parity status as the report states it, a table of everything not implemented (`DEFER`, `DROP`, `BLOCKED`, approved deviations) findable by cm6 name, test counts only next to the report's numbers, examples that compile (doctests or examples built in CI). The state audit's `ferrum/docs/parity/state/claims.md` lists the kinds of claims that were false before; do not repeat them.

References in commits, annotations and reports follow `1-parity-audit.md` §11.1.

---

## 7. Hand-off

The crate is ready for the conformance audit when every in-scope item is implemented or `BLOCKED`, and the implementer's self-check passes: annotation scan (every in-scope item found, every public item annotated), test-name scan (every cm6 case ported), every `// covers:` id exists, fixture replay at 0 mismatches, `ferrum/docs/parity/tools/check_refs.py` clean, `cargo test`/`clippy` clean. Then run `3-conformance-audit.md`.

---

## 8. Do not

- implement items outside the approved specification
- read the previous code of the crate, or copy code or tests not marked `REUSE`/`ADAPT`/`KEEP`/`FIX` in a quarry
- modify tests to turn them green
- approximate blocked, deferred or dropped items
- work around an approved decision
- invent justifications
- batch several items into one unreviewable change
- rewrite READMEs before the conformance report is approved
