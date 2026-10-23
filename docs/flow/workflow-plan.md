# Workflow plan (draft)

> What: a first plan of who does what when a Ferrum crate goes through its phases, and how work is handed from one session to the next. Status: lab draft, to be refined by trying it. Versions: instruction documents v4 (`LEARNINGS.md`); Claude Code documentation read on 2026-10-23. Ids: none.

The phase briefs already say *what* each phase produces and which work is main-session work or subagent work (`1-parity-audit.md` §4, `2-parity-implementation.md` §4, `3-conformance-audit.md` §5). This plan adds the layer they leave open: who starts each session, where a session's work goes when it ends, and what the maintainer must do and when.

## 1. Principles

1. **Files and git are the hand-off.** A session never relies on another session's conversation. Everything the next session needs is in `progress.md`, the deliverables and the commits (`rules.md` §7.2, §7.6). This is also what makes a session resumable after a usage limit, a crash or a move to the cloud.
2. **The maintainer decides, agents produce evidence.** Gates, decisions, approvals of traits and fakes, merges and pushes to `main` are the maintainer's (`rules.md` §7.4). Everything else should be able to run without the maintainer watching.
3. **One branch per unit of work.** A phase of one crate, or one module during implementation, gets its own branch (`audit/<crate>`, `impl/<crate>-<module>`, `conformance/<crate>`). The maintainer reviews and merges; an agent never merges into `main`.
4. **Fresh contexts for independence.** Who writes tests does not implement; who verifies did not produce the rows (`1-parity-audit.md` §12, `2-parity-implementation.md` §4, `3-conformance-audit.md` §4).
5. **The cheapest context that has what it needs.** Subagents read their card, not the whole document set; the main session reads reports and counts, not sources a subagent owns.

## 2. Roles

| Role | Runs as | Reads | Writes | Never |
|---|---|---|---|---|
| Maintainer | human | gate summaries, `human-decisions.md`, `progress.md`, diffs | `decisions.md`, gate approvals, merges, pushes | — |
| Orchestrator (main session) | interactive or background Claude Code session in `/workspace/ferrum` | brief, `progress.md`, reports, JSONL summaries | `progress.md`, synthesis deliverables, cards, skeletons, gate summaries | decides a gate; reads sources a subagent owns |
| Chunk auditor | subagent (`ferrum-spec-chunk`, `ferrum/docs/flow/agents.md`) | its card, its cm6 sources and tests | raw rows, fixtures, probes of its chunk | decides dispositions |
| Verifier | subagent, fresh | card, deliverables, sample | `verification.md` rows | wrote any row it checks |
| Test writer | subagent (`ferrum-test-writer`) | test card, module rows of the specification, skeleton | test files of one module | reads the implementation |
| Implementer | subagent (`ferrum-implementer`) or the orchestrator | implementation card, cm6 sources, module specification | the module's source files, `implementation.md` rows | edits a test file |
| Conformance checker | subagent, per module | conformance card, specification, crate | `raw/conformance.jsonl` rows | fixes code |
| Reviewer (optional) | fresh subagent or GitHub review | a diff and the criteria given | findings | — |

## 3. One crate, end to end

| Stage | Who starts it | Runs | Ends with | Maintainer action |
|---|---|---|---|---|
| P0–P1 setup and dispositions | maintainer (one prompt naming the crate and the brief) | orchestrator; P1 fact subagents for large packages | Gate 1 summary committed on `audit/<crate>` | approve dispositions and `DEFER` list |
| P2–P3 chunks | orchestrator | up to 4 chunk auditors at a time | raw rows and fixtures committed per chunk | none |
| P4 synthesis, P5 verification | orchestrator | orchestrator; one verifier | Gate 2 summary committed | approve specification, sketch, obvious decisions; answer `human-decisions.md`; record decisions |
| Contract traits (if any) | orchestrator | orchestrator | trait and fake committed | approve trait and fake |
| Implementation, per module | orchestrator | skeleton (orchestrator), test writer, implementer, check (orchestrator) | one branch per module, merged in order | review and merge each module branch |
| Conformance | maintainer | orchestrator; conformance checkers; verifier | `conformance.md` committed | approve the report |
| Documentation | orchestrator | orchestrator | READMEs rewritten | review |

Between stages the session may end. The next one starts from `progress.md`, which names the stage, the step, and the brief sections to read (`rules.md` §7.2).

## 4. How a session ends

1. Update `progress.md`: stage, step, done, remaining, open questions, tokens used, the brief sections the next session must read.
2. Commit (`rules.md` §7.6). For unattended runs, push the working branch so a cloud session or the maintainer can see it (allowed only when the task says so).
3. Final message: what was committed (commit ids), what was left uncommitted and why, what the maintainer must decide, and the exact prompt that resumes the work.

The resume prompt is always of the form: *"Continue `<crate>` at the stage recorded in `ferrum/docs/parity/<crate>/progress.md`."* No other context should be needed; if it is, that is a defect of `progress.md`.

## 5. Open questions for the maintainer

- Branch names and whether module branches are merged one by one or as a stack.
- Whether agents may push working branches without asking during unattended runs (needed for cloud sessions to see local work).
- Which role, if any, runs on a cheaper model (`model:` in the agent definitions): chunk auditors and test writers are the largest volume.
- Whether the orchestrator of a long phase runs locally in `tmux`, as a background session, or in the cloud (`ferrum/docs/flow/unattended-and-remote.md`).
