# Common instructions for ferrum-state audit subagents

> What: historical: instructions given to the chunk subagents of the state audit. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`.

You are one subagent in the parity audit of the Rust crate `ferrum-state` against CodeMirror 6 `@codemirror/state` 6.7.6. This is an AUDIT ONLY phase.

## Read first (in full)

(Historical: the versions these subagents read are those at commit `f63225f`; read them with `git show f63225f:docs/fundation/<file>`.) <!-- refs:ignore: git path syntax -->

1. `docs/fundation/ferrum-foundation.md`
2. `docs/fundation/rules.md`
3. `docs/fundation/1-parity-audit.md` (the brief: sections 2, 4, 6, 7, 10, 11, 13 apply directly to you)
4. `docs/fundation/1a-scope-state.md` (scope sheet; its "Special attention" section is mandatory)
5. Your raw inventories: `docs/parity/state/raw/a1-cm6-exports.md`, `docs/parity/state/raw/a2-ferrum-pub.md`, `docs/parity/state/raw/t1-cm6-tests.md`, `docs/parity/state/raw/t2-ferrum-tests.md` (filter by your chunk id).

`docs/parity/decisions.md` does not exist: no foundational decision (D1–D10) has been taken. Do not take any.

## Hard rules

- **Do not modify anything under `ferrum/` or `mirrors/cm6/`**, except creating your own output files listed in your task under `docs/parity/state/raw/`. No code fixes, no test changes, no new tests in the crate.
- Nothing in Ferrum is evidence of itself: doc comments, test names (`test_cm6_*`), README text and "1:1 parity" claims are claims. Read the code.
- `PORTED` only if **every** behavior Bn is implemented **and** proven by a Ferrum test. Code without test = unproven → at best `PORTED, DEVIATION` (say "untested: Bn, Bm").
- Status vocabulary, exactly: `PORTED`, `PORTED, RUST-IDIOMATIC`, `PORTED, DEVIATION`, `MISSING`, `BLOCKED: <crate>`, `BLOCKED: D<n>`, `PLACEMENT DEVIATION`, `FERRUM-ONLY`, `HUMAN DECISION`. You may only *propose* statuses. For Ferrum `pub` items with no cm6 counterpart, or in a different crate than cm6 puts them, report the facts (what you searched, where it is) and write `candidate: FERRUM-ONLY` / `candidate: PLACEMENT DEVIATION`; the main session decides.
- Where a status depends on D1 (position unit) or D2 (line breaks / Text model) or another decision, record the evidence; use `BLOCKED: D<n>` only when the item cannot be classified at all without the decision. Otherwise classify against current cm6 behavior and note the dependency.
- `MISSING` requires stating what was searched: grep the **whole Ferrum workspace** (`crates/*`, all crates, not only ferrum-state) for the item under plausible Rust names (snake_case of the cm6 name, synonyms). Something may live in `ferrum-view`, `ferrum-commands`, `ferrum-language`, `ferrum-view-gpui`, `ferrum-pkm` or `ferrum`.
- No summarizing: no "etc.", "similar to above", "same as previous"; one item, one row; one test, one row. If you cannot finish, list every unfinished item id explicitly at the end of your file under `## Unfinished`. An unlisted gap counts as a falsified result.
- Every row cites cm6 `file:line` (relative to `mirrors/cm6/`, e.g. `mirrors/cm6/state/src/text.ts:38`), Ferrum `file:line` (relative to `ferrum/`, e.g. `crates/ferrum-state/src/text.rs:97`), and test ids (cm6 ids like `test-text#03`, Ferrum ids like `test_text#05`, from the raw T1/T2 inventories).
- Do not invent justifications. If a difference has no concrete, verifiable reason, say "no verifiable reason found" and propose `HUMAN DECISION` where appropriate.
- English only in all output.

## Reproductions (mandatory evidence)

Every API row and every behavior row with a status other than `PORTED` needs a **minimal reproduction** a human can check quickly: input, cm6 output, Ferrum output (or "no Ferrum counterpart"). Actually run both sides; do not guess outputs.

- **cm6 side:** a ready harness exists at `docs/parity/state/oracle.md` (copy of `state/src` in `src/`, deps installed). Put your scripts in `cm6/probe-<chunk>/` and run with `cd <that cm6 dir> && tsx probe-<chunk>/x.ts`, importing from `"../src/index.ts"`. Internal (non-exported) helpers can be imported from their module files (e.g. `"../src/change.ts"`) when needed.
- **Ferrum side:** copy the template crate: `cp -r docs/parity/state/oracle/ferrum-repro/repro-template <scratch dir>/repro-<chunk>` and write `src/main.rs` (or `src/bin/*.rs`, or `#[test]`s in that crate) there; it depends on `ferrum-state` by path. Use `cargo run -q` / `cargo test -q` in that directory. Only `pub` items are reachable; if a behavior can only be reached through a private item, say so.
- Panics are results: record the panic message.
- Record reproductions inline in the row as: `repro: <input> → cm6: <output> | ferrum: <output>` (short), or as a numbered block under `## Reproductions` (R1, R2…) referenced from the row when longer. Include the exact script snippet for anything non-obvious.

## Differential oracle (T8, where your chunk allows)

For pure-state items in your chunk, if feasible within your budget, write a randomized differential check: generate inputs from a fixed seed (state the seed and PRNG), run cm6 (Node) and Ferrum on the same inputs (e.g. exchange JSON through files), compare outputs. Use ASCII-only inputs for the main comparison (positions coincide in UTF-16 and code points) and, separately, a small non-ASCII sample to document D1/D2 differences. Keep harness sources in your scratch dirs and describe them in your output under `## Oracle` (files, seed, iterations, mismatches found, with one minimal mismatching case each). If not feasible, say why.

## Position unit and line breaks (feed decisions-evidence)

For every API in your chunk that takes or returns positions or lengths, record the unit in cm6 (UTF-16 code units) and in Ferrum (currently `char` = code point), and every place where the difference changes results (surrogate pairs, emoji, `findClusterBreak`, `countColumn`, JSON offsets). For text/line APIs, record line-break semantics (cm6: lines split on `\r\n?|\n` or `lineSeparator`, stored without separators; Ferrum: ropey defaults with `unicode_lines`). Put these under `## Decision evidence` with D-number, items affected, `file:line`, concrete example values, and what each option (UTF-8 bytes / code points / UTF-16) would imply.

## Cross-crate facts

If a cm6 item's status depends on a capability another Ferrum crate owns, or something cm6 puts in `state` is needed by `view`/`language`/`commands` and absent (or present in the wrong crate), record under `## Cross-crate facts`: capability, cm6 evidence `file:line` (cm6 view/language/commands sources are under `mirrors/cm6/{view,language,commands}/src/`), what you searched in Ferrum and where, what exists (`file:line`). Do not assign cross-crate status; the main session does.

## Output format

Write exactly the files named in your task. Use these sections, in this order:

### `docs/parity/state/raw/<chunk>-api.md`
```
# <chunk> API rows
Totals: cm6 items owned N (DROP k), Ferrum items owned M. Rows written: N + M.
## cm6 items
| id | cm6 item | cm6 file:line | Ferrum counterpart (file:line) or "none (searched: …)" | proposed status | behaviors (ids) | tests cm6 / Ferrum | position unit / D-deps | notes + repro |
## Ferrum pub items
| id | Ferrum item | file:line | cm6 counterpart (package, file:line) or "none (searched: …)" | proposed status or candidate | notes + repro |
## Reproductions
## Unfinished
```
For `DROP` cm6 items write the row with status `—` and the disposition reason.

### `docs/parity/state/raw/<chunk>-behaviors.md`
For each owned cm6 item that is not DROP (including items with no Ferrum counterpart, to describe what is missing):
```
### state.cm6.NNN `<item>` — cm6 `file:line`
Helpers read: <list of cm6 helpers read transitively, file:line>
| id | behavior (branch/guard/edge case/side effect) | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict (implemented+tested / implemented, untested / differs: … / missing) |
Proposed status: … because …
```
Behavior ids are `state.<item>.B<n>` (e.g. `state.Text.lineAt.B3`), stable and unique. Cover the dimensions in brief section 6 (return values, "did nothing" case, selections, changes, metadata, configuration, boundaries, text semantics incl. D1/D2, interaction, complexity/allocation where cm6 avoids a cost). Read every helper the cm6 function uses.

### `docs/parity/state/raw/<chunk>-tests.md`
```
# <chunk> tests
Totals: cm6 cases owned N, Ferrum tests owned M.
## Table 1: cm6 cases (one row per cm6 case)
| cm6 id | file:line | scenario (purpose; input doc/selection/config/extensions; action; expected) | Ferrum id(s) | Ferrum file:line | Ferrum scenario | verdict | behaviors covered | T4 changed expectations (value cm6 → value Ferrum, documented cause or "none documented") |
## Table 2: Ferrum tests with no cm6 counterpart (one row per test)
| Ferrum id | file:line | scenario | candidate verdict (FERRUM-ONLY / maps to cm6 id …) | behaviors covered |
## Table 3: helpers (T5): one row per test helper used in your test files, cm6 helper vs Ferrum helper, parse/assert equivalence verdict with evidence
## Table 4: counts (cm6 cases, Ferrum tests, mapped EQUIVALENT/WEAKER/DIFFERENT/MISSING/NOT APPLICABLE/FERRUM-ONLY)
## Unfinished
```
Verdicts: `EQUIVALENT` / `WEAKER` (asserts less) / `DIFFERENT` (same name, different scenario or expected values) / `MISSING` / `NOT APPLICABLE` (reason) / `FERRUM-ONLY` (candidate only). Map **by behavior, not by name**: open both test bodies and compare scenario, inputs, assertions and expected values line by line. A Ferrum test named `test_cm6_*` that asserts less or uses different inputs is `WEAKER`/`DIFFERENT`. Every Ferrum test must appear exactly once in Table 1 (as a mapped Ferrum id) or in Table 2.

### `docs/parity/state/raw/<chunk>-notes.md`
```
## Decision evidence
## Cross-crate facts
## Oracle
## Open questions
## Unfinished
```

Finish with a short final message: files written, row counts per file, list of unfinished ids (or "none"), and the 5 most important findings with their row ids.
