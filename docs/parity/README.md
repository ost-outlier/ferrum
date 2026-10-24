# Parity documents: index and reference rules

> What: entry point for every parity audit document; how to resolve paths and ids. Versions: see each document's reading header. Ids: this file defines the schemes.

Read this first when you follow a reference from any document under `docs/parity/` or `docs/fundation/`.

## Paths

- Every path is relative to the repository root: `mirrors/cm6/…` is the cm6 mirror (gitignored, pinned in `docs/parity/tools/mirrors.json`, fetched with `docs/parity/tools/fetch_mirrors.py`), everything else is the Ferrum repository. Example: `mirrors/cm6/state/src/text.ts:38`, `crates/ferrum-state/src/text.rs:97`.
- Instruction documents live only in `docs/fundation/` (`ferrum-foundation.md`, `rules.md`, the `1-…` audit briefs and scope sheets, the implementation briefs `2-parity-implementation.md` and `2a-implementation-state.md`, `3-conformance-audit.md`, and `LEARNINGS.md`, the history of the method). There are no copies elsewhere. Inside instruction documents, a bare file name (`rules.md`) means that folder.
- `ferrum_strategic_architecture_vision.md` is not an instruction document: it lives outside the repository at `../ferrum_strategic_architecture_vision.md` and is a source of claims only (`docs/fundation/ferrum-foundation.md` §9).
- `file:line` references carry the version they were written against in the document's reading header. When a line has drifted, find the symbol named next to it at that version (`git show <commit>:<path>`).
- External sources: Rust crates as `<crate> <version> src/<file>:<line>` (for example `ropey 1.6.1 src/str_utils.rs:12-18`, from crates.io); npm packages as `<package> <version> <file>` (for example `@marijn/find-cluster-break 1.0.4 src/index.js:48`).
- No document points to scratch or `/tmp` paths. Scripts and small data used as evidence are archived under `docs/parity/<crate>/oracle/`; large generated data is not archived and is regenerated from the seed given in `docs/parity/<crate>/oracle.md`.

## Folders

| Folder | Content |
|---|---|
| `docs/fundation/` | instruction documents (what to do) |
| `docs/parity/` | this index, `_cross-crate.md` (register), `decisions-evidence.md`, `decisions.md` (written by a human; not yet written) |
| `docs/parity/<crate>/` | one audit: deliverables, `raw/` (evidence produced by chunk subagents), `oracle/` (harness sources) |
| `docs/parity/tools/` | shared audit tools: harness setup, inventories, rendering, reference checker (`docs/parity/tools/README.md`) |

## Id schemes

| Id | Example | Defined in | Notes |
|---|---|---|---|
| cm6 inventory row | `state.cm6.212` | `docs/parity/state/raw/a1-cm6-exports.md` | also Table A of `docs/parity/state/api-matrix.md` |
| Ferrum inventory row | `state.fe.248` | `docs/parity/state/raw/a2-ferrum-pub.md` | state audit only; obsolete after the rebuild |
| cm6 test case | `test-text#04` | `docs/parity/state/raw/t1-cm6-tests.md` | `<cm6 test file without .ts>#<nn>` |
| Ferrum test | `test_text#005` | `docs/parity/state/raw/t2-ferrum-tests.md` | state audit only |
| behavior | `state.Text.lineAt.B3` | `docs/parity/state/behaviors.md` (section of the item) | new audits: `<crate>.<item>.B<n>` in `docs/parity/<crate>/behaviors.md` |
| chunk | `C3` | `docs/parity/state/progress.md` (chunk table) | files `docs/parity/state/raw/C<n>-{api,behaviors,tests,notes}.md` |
| reproduction | `R9`, `P7`, `S97`, `R-D1` | `## Reproductions` of `docs/parity/state/raw/C<n>-api.md` | written qualified elsewhere: "`docs/parity/state/raw/C1-api.md` R9". Inside `raw/C<n>-*.md` and in rows whose evidence column names a file, a bare id refers to that file. |
| claims reproduction | `R5` (A4) | `## Reproductions` of `docs/parity/state/raw/A4-claims.md` | |
| claim | `state.claim.132` | `docs/parity/state/claims.md` | corrected copy of `raw/A4-claims.md` |
| cross-crate entry | `XC-010` | `docs/parity/_cross-crate.md` | supersedes chunk-local ids such as `CC-C5-1` (`raw/C5-notes.md`) |
| human decision group | `HD-INTERNALS` | `docs/parity/state/human-decisions.md` | |
| open question | `Q3` | `## Open questions` of `docs/parity/state/raw/C<n>-notes.md` | cited with its chunk ("C1 Q3") |
| merge question | `Q-merge-1` | `docs/parity/state/progress.md` | |
| verification finding | finding 3, check 4a | `docs/parity/state/verification.md` | |
| foundational decision | `D1`…`D10` | `docs/fundation/ferrum-foundation.md` §6 | evidence: `docs/parity/decisions-evidence.md`; decisions: `docs/parity/decisions.md` (not yet written) |
| draft decision | `APPROACH`, `WORKSPACE`, `OBVIOUS-STATE`, and `D…`/`HD-…` entries | `docs/parity/decisions-draft.md` | proposals; they bind only once copied into `docs/parity/decisions.md` |
| conformance gate | `G4` | `docs/fundation/3-conformance-audit.md` §3 | |
| new-audit raw rows | `F0042` (fixture) | `docs/parity/<crate>/fixtures/<chunk>.jsonl` | schemas: `docs/fundation/1-parity-audit.md` §11 |

A new id scheme is added to this table before it is used.

## Checking references

```sh
python3 docs/parity/tools/check_refs.py docs/parity docs/fundation docs/flow AGENTS.md
```

Shell commands run from the repository root. Old-root checks are skipped inside code blocks. It reports scratch paths, paths relative to an unstated root, missing files, line numbers beyond the end of a file, qualified reproduction ids that do not exist in the named file, and section references (`rules.md` §8.5) that name no heading or numbered item of the named file. It runs against the working tree, so line checks for `crates/` are only meaningful while the crates are at the commit stated in the document header.
