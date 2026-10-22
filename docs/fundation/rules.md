# Ferrum development rules

These rules are binding for all work on Ferrum, by humans and AI agents. They say *how* to work. *What* Ferrum is and why is in `ferrum-foundation.md`; read it first. The phase briefs add phase-specific rules on top of these: specification audit (`1-parity-audit.md` with the scope sheets `1a`–`1d`), implementation (`2-parity-implementation.md`; `2a-implementation-state.md` for `ferrum-state`), conformance audit (`3-conformance-audit.md`).

Local mirrors: cm6 in `/workspace/CodeMirror6/` (upstream: `https://code.haverbeke.berlin/codemirror`), Ferrum in `/workspace/ferrum/`, Zed in `/workspace/zed/`, markdown-oxide in `/workspace/markdown-oxide/`.

---

## 1. Sources of truth

1. cm6's source code and tests are the **specification of behavior** for every item whose disposition is `CORE` or `CONTRACT` (`ferrum-foundation.md`, section 2). cm6's READMEs and docs are secondary; when they disagree with the code, the code wins.
2. Approved decisions in `ferrum/docs/parity/decisions.md` and approved audit outputs in `ferrum/docs/parity/` are facts about Ferrum. Everything else written about Ferrum (READMEs, doc comments, test names, test counts, `ferrum_strategic_architecture_vision.md`) is a claim until verified.
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

## 3. Parity workflow (test-first)

Every change to a foundation item follows this cycle:

1. **Find the cm6 tests** for the item in `/workspace/CodeMirror6/<package>/test/` (including `webtest-*.ts`). If cm6 has none, derive tests from the cm6 source and mark them `// derived from cm6 source: <file:line>`.
2. **Port them first**, with identical scenario, expected values and assertions. Name ported tests `test_cm6_<cm6 file>_<nn>_<slug of the it() name>` so the mapping is greppable. Test helpers mirror cm6's: in `CodeMirror6/commands/test/state.ts`, `|` marks a cursor and `<…>` marks a selected range.
3. **Red.** Run the test (`cargo test -p <crate> <test name>`) and confirm it fails for the expected reason. With new API, add the minimum signature to compile.
4. **Green.** Implement by following the cm6 algorithm, without shortcuts or approximations.
5. **Refactor.** Remove duplication, keep it idiomatic, then run `cargo test --workspace` and `cargo clippy --workspace`. Everything passes with zero warnings.

Never change a ported expected value, weaken an assertion or add `#[ignore]` to make a test pass. An expected value may differ from cm6 only for a documented, independently verified cause (for example the position unit chosen in decision D1); write the cause next to the assertion.

Annotate items with their cm6 counterpart so the mapping is greppable:

- `/// cm6: <name>` for a direct counterpart (`/// cm6: EditorState.changeByRange`);
- `/// cm6 deviation: <what differs and why>` when behavior differs;
- `/// cm6 contract: <cm6 API it replaces>` for trait methods that replace a DOM or Lezer query;
- `/// placement: cm6 has this in <package>` when placed differently;
- `/// ferrum-only: <reason>` when there is no counterpart.

---

## 4. Honesty and evidence

1. **No claim without evidence.** READMEs and doc comments state only what code and tests prove. No "100%", "1:1" or "parity reached" until the parity matrix says so. Test counts appear only next to matrix numbers.
2. **No invented justifications.** If a difference from cm6 has no concrete, verifiable reason, record it as an open question instead of rationalizing it.
3. **Report faithfully.** If tests fail, say so with the output. If something was skipped or approximated, say so. A partially done item is reported as partial.
4. **Measured performance only.** Do not state frame rates, latencies or complexities that have not been measured or verified in code.

---

## 5. Licensing and clean room

1. **Ferrum's license** is the one recorded in `decisions.md` (decision D9; currently the workspace declares MIT, the GPUI surface Apache-2.0). Every crate declares its license in `Cargo.toml`, and license files exist at the repository root.
2. **cm6 (MIT).** Porting cm6 creates a derived work of MIT-licensed code. Keep cm6's copyright and permission notice in the repository (for example in a `NOTICE` or `THIRD-PARTY` file), as the MIT license requires.
3. **Zed has mixed licenses, per crate.** Check the `license` field in each crate's `Cargo.toml` before looking at it.
   - **GPL/AGPL crates** (for example `editor`, `text`, `rope`, `language`, `multi_buffer`): quarantined. Never copy, translate or paraphrase their code into Ferrum. Use them only for high-level concepts, and do not have them open while writing the corresponding Ferrum code.
   - **Apache-2.0 crates** (for example `gpui`, `sum_tree`, `collections`, `util`): may be used as dependencies of crates whose license permits it. Prefer depending over copying; copied code must keep Apache-2.0 attribution.
   - No public Ferrum type exposes a type from a GPL crate.
4. **markdown-oxide (Apache-2.0).** Same rule as Zed's Apache-2.0 crates.
5. **New dependencies** must have a license compatible with the crate that adds them. Copyleft dependencies are not allowed in any Ferrum crate.

---

## 6. Rust code standards

1. **English everywhere in the repository:** code, comments, doc comments, READMEs, commit messages. Conversations with the maintainer can be in Portuguese.
2. **Positions.** Use the position unit decided in D1 everywhere. Conversions between units go through named functions; never mix units implicitly.
3. **No hidden whole-document work.** Do not convert the document to a `String` (or iterate all of it) in operations that cm6 performs locally (on a line, a range or a chunk). If an operation must touch the whole document, document why.
4. **Errors and panics.** Do not `unwrap()`/`expect()` on conditions that valid input can trigger. Recoverable conditions return `Result` or `Option`. Precondition violations that cm6 reports by throwing (for example an out-of-range line number) may panic, like std indexing, if documented under `# Panics`; offer a non-panicking variant where callers need one.
5. **Modules:** prefer `src/module.rs` over `src/module/mod.rs`.
6. **Zero warnings:** `cargo build`, `cargo test` and `cargo clippy` produce no warnings in the workspace.
7. **Comments explain why**, not what: non-obvious decisions, cm6 correspondence, Unicode and boundary edge cases. No comments that narrate the code.
8. **Commits:** conventional style (`feat(view): …`, `fix(state): …`), one reviewable item or tightly coupled group per commit.

---

## 7. Working sessions (AI agents)

1. Start by reading, in order: `ferrum-foundation.md`, this file, the brief for the current phase, `ferrum/docs/parity/decisions.md`, and the current crate's `progress.md`.
2. State lives in files, not in the conversation. Instruction documents hold rules only; what an audit taught about the method, and how the documents changed, goes to `LEARNINGS.md` (not required reading to run a phase). Update `progress.md` (done, remaining, totals, open questions) before ending a session.
3. Work in small units with explicit totals ("n of N done"). Never summarize a list with "etc." or "similar to above"; list remaining items explicitly.
4. Do not take foundational decisions (D1–D10 in `ferrum-foundation.md`) or approve your own dispositions, statuses or exceptions; ask the maintainer.
5. Subagents may produce inventories, independent per-module work and verification passes. Decisions and synthesis stay in the main session.
6. Write every reference for a reader with no context of your session: paths relative to `/workspace/` (`CodeMirror6/…`, `ferrum/…`), never scratch or `/tmp` paths, `file:line` together with the symbol name and the version (cm6 package version, Ferrum commit) it refers to, and ids qualified by the file that defines them (instruction documents such as `rules.md` may be named alone: they live in `ferrum/docs/fundation/`). Details: `ferrum/docs/fundation/1-parity-audit.md` §11.1.
