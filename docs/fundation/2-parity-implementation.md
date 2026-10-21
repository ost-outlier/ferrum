# Parity implementation of the Ferrum foundation

**Preconditions.** Do not start work on a crate until:

1. its audit in `ferrum/docs/parity/<crate>/` has been delivered and **approved by a human** (dispositions, statuses, human decisions);
2. the foundational decisions that the crate depends on (`ferrum-foundation.md`, section 6) are recorded in `ferrum/docs/parity/decisions.md`;
3. the human has chosen the crate's **mode**: `repair` or `rebuild` (`ferrum-foundation.md`, section 7), and recorded it in `decisions.md`.

The approved audit is the work list. Do not re-decide dispositions or statuses, and do not work on items the human has not approved.

**Read before starting any session:** `ferrum-foundation.md`, `rules.md`, this brief, `ferrum/docs/parity/decisions.md`, the crate's audit folder (start with `progress.md`, `api-matrix.md`, `human-decisions.md`), and `ferrum/docs/parity/_cross-crate.md`.

---

## 0. Order across crates

Work bottom-up along the dependency graph: `ferrum-state`, `ferrum-view`, `ferrum-language`, `ferrum-commands`. A crate is implemented only after the crates it depends on have passed their acceptance criteria for the capabilities it needs.

Anything in `_cross-crate.md` owned by another crate is **not** done here: the item stays `BLOCKED: <crate>`, is listed in the README table, and keeps its register entry open. When the owning crate closes the entry, run the **unblock pass** (section 3).

`ferrum-view-gpui` and the facade are updated only after the foundation crate they consume changes its API, and only enough to compile and keep their tests meaningful. Surface work is a separate phase.

---

## 1. Principles

1. **Nothing diverges silently.** A function that carries a cm6 name but behaves differently is worse than a missing one, because callers will trust it. Either make it behave like cm6, or document the exact deviation next to it (`/// cm6 deviation: ...`) and in the matrix.
2. **Every item names its cm6 counterpart**, so the mapping can be found with grep: `/// cm6: cursorCharLeft` (for methods: `/// cm6: EditorState.changeByRange`). Items without a counterpart carry `/// ferrum-only: <reason>, see docs/parity/<crate>/ferrum-only.md`. Items placed differently from cm6 carry `/// placement: cm6 has this in <package>`. Items implementing a contract carry `/// cm6 contract: <question>` (for example `/// cm6 contract: EditorView.coordsAtPos`).
3. **Approved decisions are law.** Position unit, line breaks, syntax abstraction, layout contract, command type, threading and style model come from `decisions.md`. If an item cannot be implemented under them, stop and report; do not work around a decision.
4. **Idiomatic Rust is welcome when documented**: same observable behavior, documented reason, proof test. If observable behavior changes, it is a deviation (principle 1).
5. **No invented justifications.** If a difference cannot be justified with a concrete, verifiable reason, leave it as `HUMAN DECISION` and ask.
6. **No approximations to raise coverage.** An item that is `BLOCKED`, `DEFER` or `DROP` is documented, not approximated. No "close enough" implementation under a cm6 name.
7. **Purpose filter.** Implement what the approved dispositions say belongs in the foundation. Do not add DOM concepts because cm6 has them, and do not add surface concerns (fonts, shaping, windows, GPU) to foundation crates.

---

## 2. Modes

### 2.1 Repair mode

For crates whose existing code is kept.

- Items the audit marked `PORTED` or `PORTED, RUST-IDIOMATIC` are not modified, except where a test written in this phase proves them wrong, or an approved decision changes their types (for example D1). In both cases report before changing.
- Every other item follows the workflow in section 3.

### 2.2 Rebuild mode

For crates written again from cm6 and the approved behavior lists.

- Start from an empty module tree in the same crate (keep the crate name and its place in the workspace). The old code stays readable in git history; it is not the base.
- Implement item by item with the workflow in section 3. Existing code may be copied **only** for an item the audit marked `PORTED` or `PORTED, RUST-IDIOMATIC` and only if it still fits the approved decisions; cite the audit row in the commit message.
- Existing tests may be reused only if the audit marked them `EQUIVALENT`. All other tests are ported again from cm6.
- Order inside the crate: types and invariants first (the things everything else uses), then items in the order of the cm6 source file, so that reviewers can read both side by side.

### 2.3 Contract-first items (`CONTRACT` disposition)

Items whose disposition is `CONTRACT` need a trait before code: the layout contract (D4) in `ferrum-view`, the syntax abstraction (D3) in `ferrum-language`, the command/view interface (D5) in `ferrum-commands`.

1. Write the trait with doc comments that list, for each method, the cm6 call sites it replaces (`file:line`).
2. Write a deterministic fake implementation for tests (for example fixed-width cells and fixed line height; hand-built syntax trees).
3. Get human approval of the trait and the fake before implementing items on top of them.
4. Port the logic on top of the trait; port the cm6 tests (including `webtest-*` cases classified `NEEDS LAYOUT`) against the fake.

---

## 3. Workflow, one item at a time

For each approved item:

1. Re-read the cm6 source of the item and its behavior list (B1…Bn) in `behaviors.md`.
2. Port the cm6 tests for the item first, with identical scenario and expected values (section 4). Run them and confirm they fail for the expected reason (red), unless the item is already correct.
3. Implement or fix the code so that **every** Bn holds. If you notice a behavior the audit missed, add it to `behaviors.md` with its cm6 `file:line` and handle it. Do not stop at the behaviors already listed.
4. Add tests for behaviors that cm6 does not test, marked `// derived from cm6 source: <file:line>`.
5. Add or update the doc comment (principle 2).
6. Update the item's rows in `api-matrix.md`, `behaviors.md` and `tests-side-by-side.md`, with citations and a minimal reproduction.
7. `cargo test --workspace` and `cargo clippy --workspace` clean.
8. One commit per item, or per tightly coupled group, so a human can review each in isolation.

Report after each group of items: items done, items skipped and why, new findings, register entries opened or closed.

**Unblock pass.** When an owning crate closes a register entry, return to every item that references it and run steps 1 to 8. Before ending a session, list every open register entry that references the crate. A session may not end with an item silently left waiting.

---

## 4. Tests

- **A ported test is the same test:** same scenario, same expected values, same assertions. Never weaken an assertion, change an expected value, or add `#[ignore]` to make it pass. A failing ported test is a finding: report it.
- Every expected value that must differ from cm6 (position unit after D1, line separators after D2, grammar differences after D3) is documented next to the assertion with its cause, verified independently. "To make the test pass" is not a cause.
- A test with a cm6 name but a different scenario is renamed. Ferrum-only tests are listed as `FERRUM-ONLY` with justification.
- Test helpers (`mk_state`, `state_str`, …) match cm6's helpers (`commands/test/state.ts`: `|` for a cursor, `<…>` for a range) and have their own tests.
- Where possible, keep or build the differential oracle (audit T8) and run it for every state-only item touched.
- When the crate is done, every cm6 test case has a verdict in `tests-side-by-side.md`, and no `MISSING` or `DIFFERENT` row lacks a documented reason.

---

## 5. Documentation

**README wording.** Parity stays the stated goal. Until every row of the crate's matrix is resolved, the crate README says: "Target: parity with `@codemirror/<package>` as defined in `ferrum-foundation.md`. Status: not yet reached, see `docs/parity/<crate>/`." When the matrix is resolved, it may say parity is reached.

Also in the crate README:

- a short table of everything not `PORTED`, with status and reason (`BLOCKED`, `DEFER`, `DROP`, `PLACEMENT DEVIATION`, `FERRUM-ONLY`), so cm6 items that Ferrum does not implement stay findable by their cm6 name;
- correction or removal of every claim the claims audit found false or unverifiable;
- test counts only next to the matrix numbers, because "N tests pass" says nothing about items that were never ported.

Root README and facade: updated when a crate's public API changes; examples must compile (make them doctests or examples built in CI).

**Consistency check (recommended).** A script or test that collects all `/// cm6:` annotations, verifies that each name exists in the cm6 export list and has a matrix row, and that every matrix row marked `PORTED` or `PORTED, DEVIATION` has an annotation.

---

## 6. Acceptance criteria (per crate)

- Every cm6 export has an approved disposition, and every non-`DROP` export has an allowed status with a reason.
- No item carries a cm6 name unless it behaves like cm6 or has a documented `cm6 deviation`.
- Every Bn of every implemented item is covered by code and by a test.
- Every cm6 test case has a verdict in `tests-side-by-side.md`; no ported test was altered to pass.
- Every `FERRUM-ONLY`, `PLACEMENT DEVIATION`, `DEFER` and `DROP` item is justified, or listed in `human-decisions.md`.
- The crate depends only on crates below it in the graph, and on no UI, windowing, shaping or GPU library.
- The README matches what the matrix proves.

---

## 7. Do not

- change items outside the approved list
- rewrite what the audit marked correct (repair mode) or copy code the audit did not mark correct (rebuild mode)
- modify tests to turn them green
- implement approximations of blocked, deferred or dropped items
- work around an approved foundational decision
- invent justifications
- batch several items into one unreviewable change
