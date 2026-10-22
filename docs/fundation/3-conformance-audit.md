# Conformance audit of an implemented Ferrum crate

This brief checks a crate implemented with `2-parity-implementation.md` against its approved specification (`1-parity-audit.md`, or `1d-scope-commands.md` for commands) and answers one question: **does the crate implement the specification with 100% precision?** If not, it lists exactly where not, with reproductions.

It applies to every foundation and feature crate, including `ferrum-state` once rebuilt (its specification is the cm6 side of `ferrum/docs/parity/state/`, see `1-parity-audit.md` §16).

This phase is **audit only**: findings are reported, not fixed. The implementer fixes them and the failing gates are re-run.

---

## 0. What "100%" means

A crate is `CONFORMANT` when **every gate in section 3 passes**. There is no percentage threshold: one unapproved difference makes the crate `NOT CONFORMANT`. A difference is approved only when `decisions.md` (or an approved `human-decisions.md` answer) names it, and the code carries `/// cm6 deviation: <what>, see decisions.md <id>`.

Item verdicts:

| Verdict | Meaning |
|---|---|
| `CONFORMANT` | Every behavior of the item holds, each is proven by a passing Ferrum test, and every fixture of the item replays exactly. |
| `APPROVED DEVIATION` | Differs from cm6 only as an approved decision allows (cite it); everything else conforms. |
| `NONCONFORMANT` | At least one behavior fails, is untested, or a fixture mismatches. List behavior ids and reproductions. |
| `BLOCKED: <register id>` | Depends on a capability of another crate that is still missing (open entry in `_cross-crate.md`). |
| `UNVERIFIED` | Could not be checked (say why); counts as not passing. |

Test verdicts (cm6 case → Ferrum test): `EQUIVALENT`, `CHANGED` (an expected value differs for an approved, documented cause), `WEAKER`, `DIFFERENT`, `MISSING`. Only `EQUIVALENT` and `CHANGED` pass.

No documentation audit: READMEs and doc comments are not checked, except the `/// cm6…` annotations used for mapping (G1). The implementer updates the documentation after this audit is approved.

---

## 1. Inputs

- The approved specification in `ferrum/docs/parity/<crate>/`: `inventory`, `dispositions`, `behaviors`, `contracts`, `tests`, `hazards`, `fixtures/`, `oracle/` (cm6 generators).
- `decisions.md`, `human-decisions.md` (answered), `_cross-crate.md`.
- The implemented crate at a recorded commit. Record `cargo --version`, Node version and the cm6 package version.

Outputs: `ferrum/docs/parity/<crate>/conformance.md` (rendered from `raw/conformance.jsonl`: `id, kind (item|behavior|test|fixture|gate), verdict, evidence, repro`), plus the Ferrum replayers and oracle drivers in `ferrum/docs/parity/<crate>/oracle/ferrum/`.

---

## 2. Method: mechanical first, reading second

The state audit showed that mechanical checks (inventories from the TypeScript compiler API and from rustdoc JSON, oracles, a replay of cm6 tests) are cheap and find what reading misses. Run every mechanical gate first, for the whole crate; then spend reading effort only where they cannot decide (test assertions, behaviors without runnable fixtures, contracts).

Tools to build once and keep in `ferrum/docs/parity/tools/`: rustdoc JSON item lister (`RUSTC_BOOTSTRAP=1 cargo rustdoc -p <crate> -- -Z unstable-options --output-format json`; script `ferrum/docs/parity/state/oracle/tools/ferrum-pub.py`), annotation scanner (`/// cm6:`, `/// cm6 contract:`, `/// cm6 deviation:`, `/// ferrum-only:`, `/// placement:`), test-name and `// covers:` scanner, fixture replayer, literal extractor for test bodies (cm6 TS and Rust).

---

## 3. Gates

### G1. Public surface

- Every in-scope inventory item (not `DROP`, not `DEFER`) has exactly one Ferrum counterpart annotated `/// cm6: <name>` (or `/// cm6 contract: <name>`), found by the annotation scanner and confirmed in rustdoc JSON as public.
- Every public Ferrum item (rustdoc JSON, including re-exports and trait impls with behavior) carries one annotation; `ferrum-only` items reference an approved justification; `placement` items reference an approved decision.
- No public item exposes a cm6 `@internal` mechanism unless approved.
- `DEFER`/`DROP` items have no counterpart carrying their cm6 name (no "close enough" stand-ins).

### G2. Behaviors

- Every behavior id has at least one passing Ferrum test that names it in a comment (`// covers: view.moveVertically.B3`). The scanner lists ids with no covering test.
- Reading (sample, section 4): the covering test really asserts the behavior: the right input class, and an assertion on the observable result the behavior states. A test that merely runs the code, or asserts only part of the result (document but not selection, length but not content), is `WEAKER`.
- Every derived test listed in the specification exists, marked `// derived from cm6 source: <file:line>`, with the fixture's expected value.
- Recommended: run a mutation tool (for example `cargo-mutants`) on a sample of core functions; a surviving mutant whose change a behavior forbids is a missing or weak test.

### G3. Ported cm6 tests

- Every cm6 case classified `LOGIC-ONLY`, `NEEDS LAYOUT` or `NEEDS GRAMMAR` has a Ferrum test named `test_cm6_<file>_<nn>_<slug>` (rules.md §3).
- **Mechanical comparison** of each pair: extract the literal inputs and expected values from the cm6 case and from the Ferrum test and diff them. Every difference must be documented next to the assertion with an approved cause (D1 unit translation, D2 separators, D3 grammar); the auditor recomputes translated values from the fixture text instead of trusting the comment.
- Reading: same scenario and the same assertions (the state audit found tests named after cm6 cases that asserted less, used other inputs, or asserted nothing at all, for example a "serialization" test that serialized nothing).
- No `#[ignore]`; `#[should_panic]` only where cm6 throws, with the panic documented as the approved error contract.
- `DOM-ONLY` cases: confirm the semantic each one pins down (from `tests.md`) is covered by some other test.
- Grammar-dependent cases: values that depend on the grammar are compared against the D3 grammar; every changed value is listed in the report.

### G4. Golden fixtures

- Replay every fixture of the specification against the Ferrum crate (positions mapped from UTF-16 to the D1 unit using the stored text; separators per D2). Required result: **0 mismatches**. Each mismatch is a `NONCONFORMANT` finding with the fixture as reproduction.
- Fixtures handed down by higher crates (`fixtures/from-<crate>.jsonl`, for example from the commands audit) are replayed as well.

### G5. Differential oracle

- For every generator in `oracle/`, write the Ferrum driver; run cm6 and Ferrum on the same seeded inputs. Default size: at least 5,000 cases per item group, or what the specification states; record seeds.
- Required result: **0 mismatches** after the documented unit mapping. Order-only differences are mismatches unless the specification says the order is unspecified.
- **Coverage check:** confirm that the generated inputs actually contain every entry of the shape checklist (`1-parity-audit.md` §8): count, per checklist entry, how many cases had it. A generator that never produced equal ranges, consumer values or astral characters does not pass, whatever its mismatch count.
- **Sensitivity check:** in a scratch copy of the crate, introduce one deliberate deviation per item group (for example flip an `assoc` sign, drop deduplication) and confirm the oracle reports it. An oracle that stays at 0 does not pass.

### G6. Deviations

- Every `/// cm6 deviation:` annotation maps to an approved decision; every approved decision that requires a deviation is implemented where it applies (for example D1 conversions at every position-taking API).
- No other difference exists: anything found by G2–G5 without an approved decision is `NONCONFORMANT`.

### G7. Hazards

For every hazard listed in `hazards.md` for an item, a test exercises it, and the auditor runs a targeted check:

- identity: values cm6 compares with `===` keep their identity (or the approved D6 replacement holds) across the operations cm6 preserves it;
- cached computations computed once, and re-entrant access during computation behaves as in cm6;
- numeric ranges: consumer values (for example decoration sides ±1e8…±6e8) go through every API without truncation;
- error paths: every cm6 throw has the approved Ferrum contract (panic documented under `# Panics`, or `Result`), with a test;
- ordering: integer-key and insertion orders that cm6 relies on;
- global state: ids and counters deterministic across runs and test threads, if D6 requires it.

### G8. Cost

For every item whose cost class in `behaviors.md` is local or logarithmic, measure Ferrum on inputs of size n, 10n and 100n (release build; document length, line length or set size as relevant) and compare the growth with cm6 run on the same inputs. Growth worse than cm6's class is `NONCONFORMANT` (state reference: `count_column` took 56 s on a 40,000-character line in release, against 0.35 ms in cm6). Also grep for whole-document materialization (`to_string()` on the document, full iteration) in operations cm6 performs locally (rules.md §6.3).

### G9. Rules

- Dependencies follow the graph (`cargo tree`); no UI, windowing, shaping or GPU crate in the foundation.
- `cargo test --workspace` and `cargo clippy --workspace` with zero warnings; record the counts.
- No `unwrap()`/`expect()` on conditions valid input can trigger (grep, then read each site).
- License fields and notices as decided in D9; cm6's MIT notice present.

### G10. Cross-crate

- Every `_cross-crate.md` entry owned by this crate is resolved, with the item and test that close it.
- Every entry this crate depends on is resolved, or the dependent items are `BLOCKED` and listed.

---

## 4. Sampling and verification

- Reading-based checks (G2 assertion quality, G3 scenario identity, contracts) cover **all** items when the crate has fewer than 100 in-scope items; otherwise at least 25% of items chosen with a recorded seed, plus every item with a hazard, every `CONTRACT` item and every item touched by an approved deviation.
- A fresh context that did not run the gates re-checks at least 10% of the verdicts (minimum 20), re-runs their reproductions and the gate scripts, and records mismatches in `verification.md`. Mismatches are corrected and the sample extended with a new seed until it comes back clean.
- No shared boilerplate: a verdict reason shared by several rows is a group entry that must hold for each member (the state audit found one sentence copied onto 55 rows that was false for some of them).

---

## 5. Orchestration

As in `1-parity-audit.md` §11: main session runs the mechanical gates and writes the report; subagents (at most 4 in parallel, one subagent card) do the reading-based checks per module; each appends findings to `raw/conformance.jsonl` as it goes and lists unchecked ids at the end.

References follow `1-parity-audit.md` §11.1 (one root, no scratch paths, symbol and version with every `file:line`, qualified ids, reading header). Every reproduction in the report names the fixture or the replayer file it comes from, so the implementer can re-run it without this session. The reference checker runs as part of verification.

---

## 6. Report and gate

`conformance.md` starts with a gate table (G1–G10: pass/fail, counts, link to findings), then:

- item verdict counts; behavior counts (covered, uncovered, failing); test verdict counts; fixture and oracle results with seeds, case counts, checklist coverage and sensitivity results; cost measurements;
- every `NONCONFORMANT` and `UNVERIFIED` row with its reproduction (input, cm6 output, Ferrum output);
- open cross-crate entries;
- the verification result.

Verdict: `CONFORMANT` only if every gate passes. Then stop. A human approves the report. Only an approved conformant crate may be used by the specification and implementation of the crates above it; after approval the implementer updates the crate's documentation (README, doc comments, parity status).

---

## 7. Do not

- fix code or tests during the audit
- accept a difference without an approved decision, or a test named after a cm6 case without comparing its body
- trust an oracle without the coverage and sensitivity checks
- count `UNVERIFIED` as passing
- audit READMEs, doc comments or other claims
