# Ferrum development rules

This file is loaded into every agent session through `AGENTS.md`. Keep it short: every line here costs context in every session, including every subagent.

Source mirrors live in `mirrors/` inside the repository, which git ignores: cm6 in `mirrors/cm6/<package>/`, each package at the commit pinned in `mirrors/mirrors.json`; Zed (`mirrors/zed/`) and markdown-oxide (`mirrors/markdown-oxide/`). A mirror is read-only: never edit it.

---

## 0. Precedence and conflicts

1. **Approved decisions** in `docs/parity/decisions.md` come first. A decision that contradicts a rule here means the rule must be updated; report it.
2. **This file** comes next. A phase brief or scope sheet adds requirements for its phase and may be more specific; it never relaxes a rule here.
3. **The approved specification** of a crate (`docs/parity/<crate>/`) overrides anything marked *preliminary* in `ferrum-foundation.md`.
4. **Claims** (READMEs, doc comments, test names, test counts, `ferrum_strategic_architecture_vision.md`) are never facts (§1.2).

If two instruction documents disagree, or a document disagrees with an approved decision, **stop and report the conflict** (both locations, quoted) to the maintainer. Do not pick one.

---

## 1. Sources of truth

1. cm6's source code and tests are the **specification of behavior** for every item whose disposition is `CORE` or `CONTRACT` (`ferrum-foundation.md`, section 2). cm6's READMEs and docs are secondary; when they disagree with the code, the code wins.
2. Approved decisions and approved audit outputs in `docs/parity/` are facts about Ferrum; the approved specification of a crate is its work list. Everything else written about Ferrum is a claim until verified.
3. Do not port cm6 code that exists only for the DOM, CSS or browsers. Do not drop behavior a native editor needs just because cm6 implements it with the DOM. When unsure, ask.

---

## 2. Architecture rules

1. **Dependency direction.** The foundation is layered as in cm6:
   `ferrum-state` ← `ferrum-view` ← `ferrum-language` ← `ferrum-commands`.
   A crate may depend only on crates to its left. Feature crates (search, autocomplete, lint, collab, merge, languages) sit above the foundation; surfaces (`ferrum-view-gpui`, `ferrum-view-wgpu`, …) sit above everything they render. Nothing in the foundation depends on a feature crate or a surface.
2. **Headless foundation.** Foundation crates never depend on a UI framework, a windowing or input library, a text shaper or font library (`cosmic-text`, HarfBuzz bindings), or a GPU library. Geometry and input reach the foundation only through traits the foundation defines.
3. **Placement follows cm6.** An item lives in the Ferrum crate that corresponds to the cm6 package that defines it, unless an approved decision says otherwise. Example: keymaps and key dispatch belong in `ferrum-view`, as in `@codemirror/view`.
4. **Facade.** The `ferrum` crate only re-exports. Its feature flags follow the dependency graph.
5. **Applications are not engine crates.** PKM indexing (wikilinks, backlinks, vaults) and other application logic do not live in foundation or feature crates.

---

## 3. Tests and annotations

The order of work (what to write first, what to run, who writes what) belongs to the phase brief; for implementation it is `2-parity-implementation.md` §4. What no phase may break:

1. **A ported test is the same test**: same scenario, expected values and assertions as the cm6 test. Never change an expected value, weaken an assertion or add `#[ignore]` to make a test pass; a failing ported test is a finding to report.
2. An expected value may differ from cm6 only for an approved cause (for example the position unit of decision D1), written next to the assertion.
3. **Names.** Ported tests are named `test_cm6_<cm6 file>_<nn>_<slug of the it() name>`. A test derived from cm6 source because cm6 has none is marked `// derived from cm6 source: <file:line>`. Every test lists the behavior ids it proves: `// covers: <crate>.<item>.B<n>, …`.
4. **Helpers** mirror cm6's (`mirrors/cm6/commands/test/state.ts`: `|` marks a cursor, `<…>` a selected range) and have tests of their own.
5. A change to a crate that has an approved specification starts from a failing test derived from that specification, never from the code.
6. **Annotations.** Every public item of a foundation or feature crate carries exactly one *mapping* line, plus the *qualifier* lines that apply. cm6 names are written exactly as cm6 exports them.
   - Mapping: `/// cm6: <name>` (direct counterpart: `/// cm6: EditorState.changeByRange`); `/// cm6 contract: <cm6 API it replaces>` (trait methods that replace a DOM or Lezer query); `/// ferrum-only: <reason>, see <approval id>` (no counterpart).
   - Qualifiers: `/// cm6 deviation: <what differs>, see decisions.md <id>`; `/// placement: cm6 has this in <package>, see decisions.md <id>`.
   - An approval id names its file (§8.4): a decision in `docs/parity/decisions.md`, or an `OBVIOUS-<n>` entry of the crate's `implementation-plan.md`.

---

## 4. Honesty and evidence

1. **No claim without evidence.** READMEs and doc comments state only what code and tests prove. No "100%", "1:1" or "parity reached" unless an approved conformance report (`docs/parity/<crate>/conformance.md`) says so. Test counts appear only next to the report's numbers.
2. **No invented justifications.** If a difference from cm6 has no concrete, verifiable reason, record it as an open question instead of rationalizing it.
3. **Report faithfully.** If tests fail, say so with the output. If something was skipped or approximated, say so. A partially done item is reported as partial.
4. **Measured performance only.** Do not state frame rates, latencies or complexities that have not been measured or verified in code.

---

## 5. Licensing

1. **Outside code and tests are read or copied only if their license is MIT or Apache-2.0** (or a choice that includes one of them). Check the license (the crate's `Cargo.toml` `license` field, or the repository's license file) before opening anything outside cm6 and Ferrum.
2. **cm6 (MIT).** Porting cm6 code or tests creates a derived work: keep cm6's copyright and permission notice in the repository (decision D9 says where).
3. **Zed's GPL/AGPL crates are quarantined** (for example `editor`, `text`, `rope`, `language`, `multi_buffer`): do not open, read, copy, translate or paraphrase them. Zed's license is per crate; every other Zed crate falls under item 1. The Zed mirror fetches only the Apache-2.0 crates listed in its manifest entry and refuses any other; do not add a Zed checkout by other means.
4. **Apache-2.0 code** (for example Zed's `gpui`, `sum_tree`, `collections`, `util`, and markdown-oxide): prefer depending on it over copying; copied code keeps its Apache-2.0 attribution.
5. **Dependencies.** Foundation and feature crates take no copyleft dependency, direct or transitive; a new direct dependency whose license is not MIT or Apache-2.0 needs the maintainer's approval. Surfaces follow decision D9.

---

## 6. Rust code standards

1. **English everywhere in the repository:** code, comments, doc comments, READMEs, commit messages. Conversations with the maintainer can be in Portuguese.
2. **Positions.** Use the position unit decided in D1 everywhere. Conversions between units go through named functions; never mix units implicitly.
3. **No hidden whole-document work.** Do not convert the document to a `String` (or iterate all of it) in operations that cm6 performs locally (on a line, a range or a chunk). If an operation must touch the whole document, document why.
4. **Errors and panics.** The error contract of each precondition comes from the approved specification and decisions (`contracts.md`, `human-decisions.md`, `decisions.md`), not from the implementer; where they say nothing, ask. Never `unwrap()`/`expect()` on conditions valid input can trigger. A documented panic is listed under `# Panics`.
5. **Modules:** prefer `src/module.rs` over `src/module/mod.rs`.
6. **Zero warnings:** `cargo build`, `cargo test` and `cargo clippy` produce no warnings in the workspace.
7. **Comments explain why**, not what: non-obvious decisions, cm6 correspondence, Unicode and boundary edge cases. No comments that narrate the code.
8. **Commits:** conventional style (`feat(view): …`, `fix(state): …`, `test(state): …`, `docs(fundation): …`), one reviewable unit per commit: an item, a tightly coupled group, or the tests of one module.
9. **No API beyond the specification.** Public items exist only if the approved specification, its sketch or a decision lists them.

---

## 7. Working sessions (AI agents)

1. Before starting, read `ferrum-foundation.md` and everything the current phase brief lists under "Read before starting".
2. **State lives in files**, not in the conversation. Record the current phase, step and brief section in the crate's `progress.md`, together with done, remaining, totals and open questions; update it before ending a session. After a context compaction or in a new session, re-read the brief sections that `progress.md` names before continuing.
3. Work in small units with explicit totals ("n of N done"). Never summarize a list with "etc." or "similar to above"; list remaining items explicitly.
4. Do not take foundational decisions (D1–D10 in `ferrum-foundation.md`) or approve your own dispositions, statuses or exceptions; ask the maintainer.
5. Decisions and synthesis stay in the main session. Subagents report facts and do bounded work; each brief says which.
6. **Commit before you stop.** When a logical change to the repository is complete, commit it without asking, on a working branch (never `main`), so that another session or the maintainer can review it. Do not push unless the task says so. Report what was committed and what was left uncommitted, and why.
7. Instruction documents hold rules only. What a phase taught about the method goes to `LEARNINGS.md` (not required reading to run a phase), with the basis of each learning.
8. Experimental workflow material (agents, remote and unattended runs) lives in `docs/flow/`, status *lab*: it binds only a session the maintainer starts with it, and never overrides this file or a brief.

---

## 8. References

Deliverables, progress files, doc comments and commit messages are read later by another AI agent that has none of this session's context. That agent finds a target with `grep` and file reads, starting from the repository root, its working directory. A reference is valid only if such an agent can reach the target from the reference alone.

1. **Paths** are relative to the repository root: `mirrors/cm6/state/src/text.ts`, `crates/ferrum-view/src/lib.rs`, `docs/parity/view/behaviors.md`. Never an absolute path, never a path relative to another directory, never `./`. The only thing outside the repository that a document may name is `../ferrum_strategic_architecture_vision.md` (a source of claims). Instruction documents may be named alone (`rules.md`): they live in `docs/fundation/`.
2. **Nothing from the session environment** (`/tmp`, a scratchpad, a home directory) appears in a deliverable. A script a reader needs lives in the repository. <!-- refs:ignore -->
3. **Symbols, not only lines.** A code reference gives the line, the symbol and the version: `mirrors/cm6/view/src/cursor.ts:102` (`byGroup`). Each file states once, in its header, the Ferrum commit its line numbers refer to; that commit also fixes the mirror commits (`mirrors/mirrors.json`). Give the cm6 package version too, as the manifest records it.
4. **Ids name their file:** `docs/parity/view/behaviors.md#view.moveVertically.B3`. A short form is allowed only inside the file that defines the id. New id schemes are added to `docs/parity/README.md` before use.
5. **Sections name their file:** `1-parity-audit.md` §11.1, never a bare `§11.1` outside the file it belongs to.
6. **Write for grep.** Use identifiers and ids exactly as they appear in the target; never paraphrase a name. Point to a named file, section or id, never to the conversation or to "above" or "as discussed".
7. At the end of each phase, and before handing off, run `python3 docs/parity/tools/check_refs.py docs/parity docs/fundation docs/flow AGENTS.md --quiet` from the repository root and fix what it reports.
