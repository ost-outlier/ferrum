# Unattended and remote runs

> What: how to keep Ferrum work running without the maintainer at an open terminal, including across a usage-limit reset. Status: lab; the options are compared from the documentation, none has been tried on Ferrum yet. Versions: Claude Code documentation read on 2026-10-23: `https://code.claude.com/docs/en/interactive-mode` ("Wait for a usage limit to reset"), `https://code.claude.com/docs/en/agent-view`, `https://code.claude.com/docs/en/sub-agents`, `https://code.claude.com/docs/en/claude-code-on-the-web`, `https://code.claude.com/docs/en/routines`, `https://code.claude.com/docs/en/claude-projects`, `https://code.claude.com/docs/en/github-actions`. Ids: none.

## 1. The problem

The maintainer has had to keep a terminal open for about five hours doing nothing, because closing it would stop the subagents working in the background, and the session continued by itself once the usage limit reset (`docs/flow/learning_flow.md` FL1). The documentation explains why:

- An **interactive** session signed in with a claude.ai subscription waits for the limit to reset and then continues the task on its own (on by default since v2.1.234; setting `autoContinueAtUsageLimit`). The session must stay open. Exiting ends the wait, and it does not restart on resume. After a sleep of more than about 30 minutes it asks for Enter instead. It re-arms at most twice in a row, and does not wait for a reset more than 24 hours away.
- **Background sessions** (`claude agents`, `claude --bg`) and **`-p` runs** never wait: the option is not offered.
- A **background subagent** cut off by a usage limit is marked failed, and its last output is returned to the parent, so partial work is not lost. Once the limit clears it can be resumed (`SendMessage` to its id) or retried.

## 2. Options

| Option | Where it runs | Survives closing the terminal | Continues after a limit reset | Needs | Fits |
|---|---|---|---|---|---|
| Interactive session inside `tmux` | your machine (or the dev-sandbox container) | yes: detach `tmux`, the session keeps running | **yes**, automatically (interactive) | machine awake; a sleep over ~30 min needs Enter | long orchestrator sessions today, with no other change |
| Background session (`claude --bg "<prompt>"`, `claude agents`) | your machine, under a supervisor process | yes, and across sleep; not across shutdown | no: re-prompt it (`claude attach <id>`) after the reset | nothing new | dispatching several independent tasks; check them later |
| Cloud session (`claude --cloud "<prompt>"`, claude.ai/code) | Anthropic VM | yes, laptop can be closed | not stated for plain cloud sessions; idle sessions expire and background work is not restored | repository on GitHub, branch pushed; setup script for the environment (§3) | a self-contained task whose state is in the repository |
| Project thread (claude.ai/code Projects, beta, Pro/Max) | cloud sessions coordinated by one conversation | yes | **yes**: a thread keeps retrying and continues when the limit resets (not threads a scheduled task started) | as cloud sessions; repository `CLAUDE.md` is loaded, repository skills are not | many related tasks over days, steered from a phone |
| Scheduled task (`/schedule`, claude.ai/code/routines) | cloud session per run | yes | no wait: a run over the limit is rejected until the window resets | as cloud sessions; a self-contained prompt | "resume `<crate>` from `progress.md` at 15:10": a one-off run timed after the reset replaces the alarm clock |
| GitHub Actions (`anthropics/claude-code-action@v1`) | GitHub runner | yes | no | `CLAUDE_CODE_OAUTH_TOKEN` (subscription) or `ANTHROPIC_API_KEY` as a repository secret; runner minutes | reviews and small tasks triggered by `@claude` or a PR; not long audits |
| Remote Control (`/remote-control`) | your machine | no: it steers a local session | as the local session | machine awake | answering a permission prompt or a question from a phone |

## 3. What Ferrum needs before cloud runs work

These are requirements found by reading the documents against the repository; none has been tried.

1. **Mirrors.** Every reference is relative to the repository root; the cm6 sources are fetched into `mirrors/` by `docs/parity/tools/fetch_mirrors.py`, and the harness is built into `.harness/`, both gitignored. A cloud setup script only runs `python3 docs/parity/tools/fetch_mirrors.py` and `python3 docs/parity/tools/harness/setup_cm6.py <package>`; the network level must allow `code.haverbeke.berlin`. Setup scripts are cached when they finish in about five minutes.
2. **State pushed.** A cloud session sees only what is pushed. With `rules.md` §7.6 (commit before stopping) the remaining step is pushing the working branch, which today needs the maintainer's permission in the task.
3. **Toolchain.** Node 22, `tsx`, TypeScript and a Rust toolchain matching `Cargo.toml`. Check the cloud environment's installed tools first; add the rest in the setup script.
4. **Network.** The default *Trusted* network level allows package registries; cloning CodeMirror from `https://code.haverbeke.berlin/codemirror` may need a custom allowed domain, or a GitHub mirror.
5. **Usage.** Cloud and local sessions share the same plan limits. Running more in parallel uses them faster; the briefs' cap of 4 parallel subagents still applies.

## 4. Recommended order of trials

1. **Now, no setup:** run the orchestrator as an interactive session inside `tmux` (the dev-sandbox image has it), detach, and come back after the reset. This is the documented behavior that already worked with an open terminal, without the open terminal.
2. **Next:** a scheduled one-off task timed just after a reset, with the resume prompt of `docs/flow/workflow-plan.md` §4, once §3 above works in one manual cloud session.
3. **Later:** a Project for a whole crate, if the beta is available on the account.

Record each trial in `docs/flow/learning_flow.md`: what was run, what happened at the limit, tokens used, what had to be fixed.
