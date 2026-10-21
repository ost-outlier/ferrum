# Parity audit of the Ferrum foundation

This is the common audit brief for the four foundation crates. It is used together with one scope sheet per crate:

| Order | Crate | cm6 package | Scope sheet |
|---|---|---|---|
| 1 | `ferrum-state` | `@codemirror/state` | `1a-scope-state.md` |
| 2 | `ferrum-view` | `@codemirror/view` (platform-independent part) | `1b-scope-view.md` |
| 3 | `ferrum-language` | `@codemirror/language` | `1c-scope-language.md` |
| 4 | `ferrum-commands` | `@codemirror/commands` | `1d-scope-commands.md` |

This phase is **audit only**: do not change any code or test. Implementation follows in a separate phase (`2-parity-implementation.md`), after a human has reviewed the audit and taken the foundational decisions.

**Read before starting any session:** `ferrum-foundation.md` (purpose, fidelity kinds, dependency graph, decisions D1–D10), `rules.md`, this brief, the scope sheet of the crate, `ferrum/docs/parity/_cross-crate.md`, `ferrum/docs/parity/decisions.md`, and the crate's `progress.md` if it exists.

Paths: cm6 sources are under `/workspace/CodeMirror6/`, Ferrum under `/workspace/ferrum/`. Outputs go to `/workspace/ferrum/docs/parity/`.

---

## 0. What this audit is for

The audit answers three questions per crate, with evidence:

1. **What does cm6 do that Ferrum must do?** Every cm6 item gets a *disposition* (section 3) so that DOM-only machinery is excluded explicitly and nothing relevant is silently dropped.
2. **How far is Ferrum from it?** Every relevant item and every test gets a *status* (section 4) backed by citations and a reproduction.
3. **What does the human need to decide?** Open questions, evidence for decisions D1–D10, and the data to choose between repairing and rebuilding the crate.

It is **not** a plan to convert cm6 into Rust. Ferrum ports cm6's behavior where it is platform-independent, ports it behind a Rust contract where cm6 asks the DOM or Lezer, and does not port the DOM (`ferrum-foundation.md`, section 2).

---

## 1. Orchestration

### 1.1 Order

Audit in dependency order: state, view, language, commands. Each audit uses the approved results of the crates below it (a capability that `commands` needs from `view` can only be classified once the view audit says whether it exists). The view audit and the language audit may overlap once the view inventory (A1–A3) is approved.

Each crate ends at a **human gate** (section 10). The next crate's behavior audit starts after that gate.

### 1.2 Sessions, subagents and files

The failure this structure prevents is the one seen before: a long task, a context window that fills up, and an agent that starts summarizing ("similar to above", "etc.") and skipping items. The defenses are small units of work, explicit totals and progress written to files.

- **Files are the memory.** Every session starts by reading `ferrum/docs/parity/<crate>/progress.md` and ends by updating it: chunks done, chunks left, totals, open questions. Nothing important lives only in a conversation.
- **One crate = one campaign of several sessions.** A session handles one chunk: an inventory, or the behavior audit of one cm6 source file (or a part of a large one), or the test audit of one cm6 test file. Large files (`view/src/editorview.ts`, `commands/src/commands.ts`, `state/src/facet.ts`) are split by export ranges declared in `progress.md`.
- **Use subagents for work that is mechanical or independent:**
  - inventories (A1, A2, T1, T2): one subagent per source or test file, each writing its own file under `ferrum/docs/parity/<crate>/raw/`;
  - behavior audits of independent modules, each writing its own section file;
  - the **verification pass** (section 9): a fresh-context subagent (or a new session) that did not produce the rows it checks.
- **Keep in the main session** what needs the whole picture: dispositions, placement, cross-crate entries, merging raw files into the deliverables, the summary for the human. Subagents report facts; they do not decide `FERRUM-ONLY`, `PLACEMENT DEVIATION`, dispositions or cross-crate status.
- Every subagent prompt includes: this brief, the scope sheet, the exact list of items it owns (with the total), the output file, and the instruction to list explicitly any item it did not finish.

### 1.3 Phases per crate

1. **Inventory** (A1–A3, T1–T2) and **dispositions** (section 3). Human approves the dispositions before step 2 for `view` and `language`; for `state` and `commands` nearly everything is `CORE` and the approval can happen at the final gate.
2. **Behavior audit** (section 6) and **test audit** (section 7) for every item whose disposition is not `DROP`.
3. **Claims audit** (A4), **cross-crate register** (A5), **decision evidence** (section 8).
4. **Verification pass** (section 9).
5. **Human gate** (section 10).

---

## 2. Ground rules

1. **Nothing in Ferrum is evidence of itself.** Code, comments, doc comments, READMEs, test names, test counts and placement decisions are *claims to verify against cm6*. This includes everything produced by previous AI sessions, and `ferrum_strategic_architecture_vision.md`.
2. **Do not invent justifications.** If a difference cannot be justified with a concrete, verifiable reason, mark it `HUMAN DECISION`, list the options and move on.
3. **Idiomatic Rust is welcome when documented.** A different structure is acceptable if the **observable behavior is the same**. Record what changed, the concrete benefit ("more idiomatic" alone is not a benefit), and the test that proves equivalence. If observable behavior changes, it is a deviation.
4. **Missing DOM is not an automatic excuse.** Every `DROP` and every `NOT APPLICABLE` needs a reason specific to that item. If the item's *behavior* matters natively (input semantics, composition, selection drawing, scrolling), it is `SURFACE` or `CONTRACT`, not `DROP`.
5. **Explain differences; do not fix them.** Where Ferrum differs from cm6, record what differs, why (if known) and what behavior is kept or lost. Do not propose code in this phase.
6. **Purpose filter.** Ferrum is a native engine (see `ferrum-foundation.md`). Do not mark something `CORE` only because cm6 has it, and do not mark it `DROP` only because it touches the DOM. Ask what the item does for a native editor.
7. **Do not decide foundational questions.** Where an item depends on D1–D10, record the evidence and the dependency; do not pick an answer.

---

## 3. Dispositions (what cm6 item belongs where)

Every cm6 export, and every cm6 source file as a whole, gets one disposition:

| Disposition | Meaning |
|---|---|
| `CORE` | Platform-independent logic. Belongs in this foundation crate with behavioral parity. |
| `CONTRACT` | Logic belongs in the foundation, but it needs geometry, input or syntax that cm6 obtains from the DOM or Lezer. Name the question asked (for example "position → screen coordinates") and the contract it implies. |
| `SURFACE` | Belongs to a surface crate (drawing, measuring, OS integration). The foundation may own a model or facet for it; say which part. |
| `ELSEWHERE: <crate>` | Belongs to another Ferrum crate (foundation or feature crate). |
| `DEFER` | Relevant to a native editor but not needed for the foundation's first version (decision D10 decides). Reason required. |
| `DROP` | Exists only because of the DOM, CSS or browser behavior; no native equivalent needed. Reason required. |

Dispositions are proposals. The human approves them.

---

## 4. Statuses (use exactly these)

For every item whose disposition is not `DROP`, and for every Ferrum `pub` item:

| Status | Meaning |
|---|---|
| `PORTED` | Every observable behavior of the cm6 item is implemented **and** proven by a test |
| `PORTED, RUST-IDIOMATIC` | Same observable behavior, different structure (documented per rule 3) |
| `PORTED, DEVIATION` | Exists, but some observable behavior differs or is untested. List exactly which |
| `MISSING` | No Ferrum counterpart anywhere in the workspace (state what was searched) |
| `BLOCKED: <crate>` | Cannot be correct until another crate provides a capability that the search proved absent |
| `BLOCKED: D<n>` | Cannot be classified until foundational decision D<n> is taken |
| `PLACEMENT DEVIATION` | Exists, but in a different crate than where cm6 has it (or than its disposition says) |
| `FERRUM-ONLY` | Ferrum `pub` item with no cm6 counterpart. Verifiable justification, or `HUMAN DECISION` |
| `HUMAN DECISION` | You cannot decide alone. Describe the options |

An item is `PORTED` only if **all** of its behaviors from section 6 are covered by code and test. Partial coverage is `PORTED, DEVIATION`.

---

## 5. Inventories

**A1. cm6 public API.** Derive it from every `export` in the package (including re-exports, types, interfaces, enums, constants, static members and instance methods of exported classes), not from the README. Compare with the package README symbol list and report every discrepancy in either direction. Assign each item a disposition.

**A2. Ferrum public API.** Every `pub` item of the crate: functions, types, fields, methods, consts, statics, traits, macros, re-exports, and items exposed through the `ferrum` facade.

**A3. Two-way cross-reference, including placement.**
- For each cm6 item: where is it in Ferrum? Search **every crate in the workspace**. Something that looks missing may live elsewhere.
- For each Ferrum `pub` item: where is it in cm6, and in which package? Different package (or different from its disposition) means `PLACEMENT DEVIATION`. No counterpart means `FERRUM-ONLY`.

**A4. Claims audit.** Extract every verifiable statement from the crate README, `src/README.md`, the root README, the facade, doc comments and `ferrum_strategic_architecture_vision.md` where it concerns this crate (behavior, complexity, Unicode/CRLF support, key binding lists, test counts, "100%" and parity statements, example code). Verify each. Output: claim, source `file:line`, evidence, verdict (`true` / `false` / `unverifiable`). Example code in READMEs is a claim that it compiles and behaves as stated.

**A5. Cross-crate register.** The audit's scope is one crate; do not audit other crates. But whenever an item's status depends on a capability owned by another crate:
1. Name the capability and cite where cm6 uses it (`file:line`).
2. Search the whole Ferrum workspace for it. Record what was searched and where.
3. Open or update an entry in `ferrum/docs/parity/_cross-crate.md`: id, requesting crate and item, capability, cm6 evidence, owning Ferrum crate (per the dependency graph in `ferrum-foundation.md`), what exists there (`file:line`) or "absent after searching X", state (`open` / `resolved` / `wontfix`).
4. Use `BLOCKED: <crate>` **only** if the search proves the capability absent. If it exists but is unused, record that and mark the item `HUMAN DECISION`.
5. Reference the entry id in the matrix row. An item blocked by an open entry cannot be `PORTED`.

Entries may point upward too: the state audit may find that something cm6 puts in `state` is needed by `view` and absent. Before finishing, list every entry touching this crate and its state.

---

## 6. Behavior audit (the core)

For every cm6 item whose disposition is `CORE`, `CONTRACT` or `ELSEWHERE` (pointing to this crate), including items with no Ferrum counterpart (to describe what is missing):

1. Read the whole cm6 function and, transitively, every helper it uses.
2. List its observable behaviors as numbered items B1…Bn: each branch, guard, early return, edge case and side effect. Ids are stable: `<crate>.<item>.B<n>`.
3. For each Bn, locate the Ferrum code that implements it and the test that proves it. No code means missing; no test means unproven.
4. Classify the item with section 4.
5. For `CONTRACT` items, also write the question the logic asks of the layout or syntax layer, in cm6 terms (for example: "`moveVertically` asks for the screen coordinates of the head and for the position at (x, y ± line height)").

Dimensions to consider (prompts, not an exhaustive list; add everything the cm6 code shows):

- return-value semantics, including the "did nothing" case
- resulting selection: anchor/head, association (`assoc`), goal column, bidi level, main index, ordering, merging of overlapping or adjacent ranges
- generated changes: exact ranges, order, merging, multiple ranges
- transaction metadata: user events, `scrollIntoView`, effects, annotations, filters, time
- configuration consulted: read-only, tab size, indent unit, line separator, language data, facets, compartments, precedence
- boundaries: empty document, document start/end, empty line, whitespace-only line, last line without a line break
- text semantics: position unit (cm6 uses UTF-16 code units; Ferrum currently uses code points; see D1), line separators (see D2), grapheme clusters, ZWJ sequences, combining marks, variation selectors, surrogate pairs, CRLF, RTL/bidi, tab columns, word categories
- view/layout dependencies: visual line wrapping, folds, widgets, atomic ranges, viewport, text direction
- syntax/language dependencies
- interaction with the rest of the system (for example what history reads from transactions)
- complexity and allocation, wherever someone claims something or cm6 clearly avoids a cost (for example, materializing the whole document)

---

## 7. Test audit: same rigor, side by side

Tests get the same status vocabulary, evidence rule and completeness rules as code.

- **T1. cm6 inventory.** Every test case in every file of the package's `test/` directory, **including `webtest-*.ts`**, with a unique id (file, block, name, index). Expand parametrized and loop-generated tests into individual cases. Record purpose, input (document, selection, configuration and extensions, including languages loaded), action, expected result.
- **T2. Ferrum inventory.** The same for every Ferrum test and doctest of the crate.
- **T3. Side-by-side mapping, by behavior, not by name.** Verdicts: `EQUIVALENT` / `WEAKER` (asserts less, e.g. document but not selection) / `DIFFERENT` (same name, different scenario or expected values) / `MISSING` / `NOT APPLICABLE` (reason) / `FERRUM-ONLY` (justify). For every `webtest-*` case add: `LOGIC-ONLY` (depends only on state, can run headless), `NEEDS LAYOUT` (wrapping, widgets, folds, bidi, coordinates: reproducible against a fake layout; open a cross-crate entry for the contract) or `DOM-ONLY` (reason). Being in a `webtest` file is not a reason by itself.
- **T4. Changed expectations.** Every expected value that differs from cm6 needs a documented cause, verified independently (for example, the position-unit difference). "To make the test pass" is not a cause. Tests that load a Lezer grammar (`@lezer/javascript`, `lang-html`, …) and whose Ferrum version uses another parser are `DIFFERENT` unless proven otherwise; record which grammar-dependent values would change.
- **T5. Test helpers.** Verify that Ferrum's helpers (cursor/selection markers, state construction, comparison) parse and assert the way cm6's do. Each helper gets its own row.
- **T6. Count reconciliation.** Actual counts vs counts advertised in READMEs; cm6 cases vs ported cases; the difference listed case by case.
- **T7. Behavior coverage.** Every Bn from section 6 links to at least one cm6 test and one Ferrum test, or records that none exists. Where cm6 has no test for a behavior, record that fact; do not write one in this phase.
- **T8. Differential oracle (recommended).** For items that depend only on state, run the cm6 implementation (Node; the `CodeMirror6/dev` tooling installs the packages) and the Ferrum implementation on the same generated inputs and compare document, selection and return value. Record the seed and the position-unit mapping. Describe the harness in `oracle.md`. If it cannot be built in this phase, say why.

Behavior rows list their tests and test rows list the behaviors they cover.

---

## 8. Evidence for foundational decisions

The audit does not decide D1–D10 (`ferrum-foundation.md`, section 6), but it is the best moment to collect evidence. For each decision touched by the crate, append to `ferrum/docs/parity/decisions-evidence.md`: decision id, the items affected, concrete examples (`file:line`), and what each option would imply for them. Example: for D1, which tests have non-ASCII positions and what their expected values become under each unit.

---

## 9. Verification pass

Before the human gate, a fresh context (a subagent or a new session that did not write the rows) checks the work:

- re-derive a sample of at least 10% of matrix rows (minimum 20, all rows if fewer) from the sources, choosing rows randomly and including every `PORTED` row of a non-trivial item;
- re-run every reproduction in the sample;
- re-count totals from the raw inventories;
- report every mismatch in `verification.md`. Mismatches are fixed in the deliverables and the sample is extended until a sample comes back clean.

---

## 10. Completeness rules

- **Declare totals first:** number of cm6 exports, Ferrum `pub` items, cm6 test cases (after expansion), Ferrum tests. Every matrix has exactly that many rows.
- **No summarizing.** No "etc.", "similar to above", "same as previous", no grouping of different items into one row. One item, one row.
- **Work in chunks**, saving after each, and report progress as "n of N done" in `progress.md`. If you cannot finish, list the unaudited items explicitly. An unlisted gap counts as a falsified result.
- A row without citations is invalid.

---

## 11. Deliverables and evidence

Per crate, in `ferrum/docs/parity/<crate>/` (English):

- `progress.md`: chunks, totals, n of N done, open questions
- `dispositions.md`: one row per cm6 export and per cm6 source file, with disposition and reason
- `api-matrix.md`: one row per cm6 export (non-`DROP`) and per Ferrum `pub` item, with status
- `behaviors.md`: B1…Bn per item, with code and test links; for `CONTRACT` items, the question asked of the contract
- `tests-side-by-side.md`: Table 1, one row per cm6 test case (cm6 id, `file:line`, scenario | Ferrum id, `file:line`, scenario | verdict | behaviors covered); Table 2, Ferrum tests with no cm6 counterpart; Table 3, helper equivalence (T5); Table 4, count reconciliation (T6)
- `claims.md`: A4
- `ferrum-only.md`, `placement.md`: with justification
- `human-decisions.md`: everything left as `HUMAN DECISION`
- `oracle.md`: T8, if attempted
- `verification.md`: section 9
- `raw/`: raw inventories produced by subagents

Workspace-wide, in `ferrum/docs/parity/`:

- `_cross-crate.md`: the A5 register (one file for the workspace)
- `decisions-evidence.md`: section 8
- `decisions.md`: written by the human; read-only for the auditor

**Every row cites** the cm6 `file:line`, the Ferrum `file:line` and test ids, and includes a **minimal reproduction** (input, cm6 output, Ferrum output if it exists) that a human can check quickly.

---

## 12. Human gate

When the crate is done, deliver:

- totals and per-status counts for the API matrix and the test matrix, and per-disposition counts;
- the `HUMAN DECISION` list;
- the cross-crate entries touching this crate, with their state;
- the decision evidence added;
- the verification result;
- confirmation that sections 5 to 7 cover every source file of the package listed in the scope sheet.

Then stop. A human spot-checks the reproductions, approves dispositions and statuses, and decides `FERRUM-ONLY`, `PLACEMENT DEVIATION`, `NOT APPLICABLE`, `DROP`, and any `RUST-IDIOMATIC` whose behavior might differ. The per-status counts are the input to the repair-or-rebuild decision for the crate (`ferrum-foundation.md`, section 7).

---

## 13. Do not

- trust names, comments, READMEs, test names or test counts
- mark `PORTED` from reading a signature
- stop at the first divergence found in a function
- skip a source file or a test file because it is large or because it is a `webtest`
- port DOM machinery into the plan because cm6 has it, or drop native-relevant behavior because cm6 implements it with the DOM
- fix anything during the audit
- audit or modify other crates (read them only to establish dependency facts, A5)
- decide foundational questions D1–D10
- invent justifications
