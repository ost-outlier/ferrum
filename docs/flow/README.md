# Workflow lab

> What: experimental material on *how* Ferrum work is run by AI agents and the maintainer: roles, hand-offs, agent configuration, unattended and remote runs. Status: **lab**, untested as a whole. Versions: Claude Code documentation read on 2026-10-23 (URLs in each file). Ids: `FL<n>` (learnings, `ferrum/docs/flow/learning_flow.md`).

**Coupling.** Nothing in `ferrum/docs/fundation/` or `ferrum/docs/parity/` depends on this folder. The only pointer to it is `rules.md` §7.8, which states that this material binds only a session the maintainer starts with it and never overrides `rules.md` or a phase brief. To drop the lab: delete this folder and that one line. To adopt a piece: move it into the phase brief or `rules.md` it belongs to, record the change in `LEARNINGS.md`, and delete it here.

**Nothing here is installed.** Agent definitions, hooks and workflow files are written as sketches inside these Markdown files. Installing one (copying it into `ferrum/.claude/` or `.github/workflows/`; that folder does not exist yet) is a maintainer decision, recorded in `ferrum/docs/flow/learning_flow.md` with what was observed.

| File | Content |
|---|---|
| `ferrum/docs/flow/workflow-plan.md` | Roles (maintainer, main session, subagents), the flow of one crate through its phases, hand-offs, what needs the maintainer |
| `ferrum/docs/flow/agents.md` | Sketches of subagent definitions, a phase skill and hooks for Claude Code, with what each one would replace in the briefs |
| `ferrum/docs/flow/unattended-and-remote.md` | Running work without an open terminal or a human waiting for a usage limit to reset: local, background, cloud, scheduled and GitHub options, compared |
| `ferrum/docs/flow/learning_flow.md` | What trying these taught, with the basis of each entry |
