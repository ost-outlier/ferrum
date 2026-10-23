# Agents, skills and hooks for Claude Code (sketches)

> What: sketches of Claude Code configuration that could run the roles of `ferrum/docs/flow/workflow-plan.md`. Status: lab; nothing here is installed. Versions: Claude Code documentation read on 2026-10-23: `https://code.claude.com/docs/en/sub-agents`, `https://code.claude.com/docs/en/skills`, `https://code.claude.com/docs/en/features-overview`, `https://code.claude.com/docs/en/context-window`. Ids: none.

The phase briefs and their cards stay the source of truth: they also serve agents other than Claude Code. A definition below only packages a role, so that the orchestrator can call it by name with the right tools, model and isolation, and so that a rule that must always hold is enforced by a hook instead of requested in prose.

## 1. What each mechanism gives (from the documentation)

| Mechanism | Loads | Useful here for | Limits to know |
|---|---|---|---|
| `CLAUDE.md` / `AGENTS.md` | every session and every non-fork subagent; root file re-injected after compaction | `rules.md`, already in place (`ferrum/AGENTS.md`) | everything in it costs context in every request; keep it short |
| Subagent (`.claude/agents/<name>.md`) | when spawned; fresh context with its own system prompt, `CLAUDE.md`, and the skills listed in `skills:` | one role with fixed tools, model, `isolation: worktree`, `maxTurns` | does not see the parent conversation; built-in Explore and Plan skip `CLAUDE.md`; at most three nesting levels by default |
| Skill (`.claude/skills/<name>/SKILL.md`) | description at startup, body when invoked; after compaction the first 5,000 tokens are re-attached | a `/ferrum-phase <crate>` entry point that resumes from `progress.md` | not loaded in Projects cloud threads from the repository; body is not re-read on later turns |
| Hook (`.claude/settings.json`, or `hooks:` in an agent definition) | runs on an event, outside the model | guarantees: block edits to test files by the implementer; re-inject the resume instruction after compaction | output, if any, enters the context |

## 2. Subagent sketches

Each definition would live in `ferrum/.claude/agents/`, which does not exist yet. The body is short on purpose: the card the orchestrator passes in the prompt carries the task.

```markdown
---
name: ferrum-spec-chunk
description: Audits one chunk of a cm6 package for a Ferrum specification audit. Use for P2-P3 chunks of 1-parity-audit.md.
tools: Read, Grep, Glob, Bash, Write, Edit
model: inherit
maxTurns: 400
---
You are a chunk auditor. The card in your prompt is your task; ferrum/docs/fundation/rules.md is in your context.
Write only the files the card names. Report facts; do not decide dispositions, cross-crate status or decisions.
Append each finished item to your files immediately and keep raw/<chunk>.progress current, so that you can be resumed.
```

```markdown
---
name: ferrum-test-writer
description: Writes the tests of one module of a Ferrum crate from its approved specification. Use for step 2 of 2-parity-implementation.md §4.
tools: Read, Grep, Glob, Bash, Write, Edit
model: inherit
isolation: worktree
---
You write tests only. The test card in your prompt is your task. Do not open the module's implementation files.
```

```markdown
---
name: ferrum-implementer
description: Implements one module of a Ferrum crate until its tests pass. Use for step 3 of 2-parity-implementation.md §4.
tools: Read, Grep, Glob, Bash, Write, Edit
model: inherit
isolation: worktree
hooks:
  PreToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "python3 /workspace/ferrum/docs/flow/hooks/deny_test_edits.py"
---
You implement one module. The implementation card in your prompt is your task. Test files are read-only for you:
a test that looks wrong is reported to the orchestrator, never changed.
```

`deny_test_edits.py` does not exist yet. Its job: read the hook's JSON from stdin, and if `tool_input.file_path` is under a `tests/` directory or ends in `_test.rs`/`tests.rs`, exit with code 2 and a message on stderr, which blocks the call and tells the agent why. The exact input fields and exit-code semantics must be checked against `https://code.claude.com/docs/en/hooks` before installing.

```markdown
---
name: ferrum-verifier
description: Fresh-context verification pass of a Ferrum audit (1-parity-audit.md §12, 3-conformance-audit.md §4). Never used by a session that produced the rows.
tools: Read, Grep, Glob, Bash, Write
model: inherit
---
You verify; you did not produce any row you check. The card in your prompt names the sample seed and the checks.
```

## 3. A phase skill

```markdown
---
name: ferrum-phase
description: Resume Ferrum work for one crate from its progress file.
disable-model-invocation: true
arguments: [crate]
---
Read ferrum/docs/parity/$crate/progress.md. It names the stage, the step and the brief sections to read.
Read those sections, then continue the step. Before stopping, follow rules.md §7.2 and §7.6.
```

Invoked as `/ferrum-phase view`. It replaces typing the resume prompt of `ferrum/docs/flow/workflow-plan.md` §4; it adds nothing the prompt does not already say, so its value is convenience and fewer typing mistakes.

## 4. Hooks worth trying

- **Re-inject after compaction.** A `SessionStart` hook matching the `compact` source can print one line ("Re-read the brief sections listed in the crate's progress.md before continuing"); the documentation says its output is added to the compacted context. This turns `rules.md` §7.2 from a request into something that always happens.
- **Block test edits during implementation** (above).
- **Reference check before stopping.** A `Stop` hook could run `check_refs.py --quiet` and block the end of a turn while it reports findings. Risk: it also blocks sessions that never touched documents; scope it to sessions that changed files under `ferrum/docs/`.

## 5. Not recommended yet

- **Agent teams** and **dynamic workflows**: useful for many parallel workers, but the briefs cap parallel subagents at 4 for usage-limit reasons (`LEARNINGS.md` L10), and the orchestrator already coordinates; try them only after the plain subagent flow works.
- **Moving the briefs into skills**: they are read by agents other than Claude Code, and a skill body is cut to 5,000 tokens after compaction, which is shorter than `1-parity-audit.md`.
