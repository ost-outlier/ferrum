# Ferrum: instructions for AI agents

Ferrum is a native, headless Rust text-editor engine whose core reproduces the architecture and observable behavior of CodeMirror 6 (cm6). Purpose, architecture and open decisions: `ferrum/docs/fundation/ferrum-foundation.md`.

The rules below are binding in every session and every phase. Claude Code loads them through the import on the next line; other agents must read `ferrum/docs/fundation/rules.md` before any task.

@docs/fundation/rules.md

Before any task, identify its phase and read that phase's brief in `ferrum/docs/fundation/` (specification audit `1-parity-audit.md` with a scope sheet `1a`–`1d`; implementation `2-parity-implementation.md`, `2a-implementation-state.md` for `ferrum-state`; conformance audit `3-conformance-audit.md`). All paths in the documents, including this one, are relative to `/workspace/`, except the import line, which is relative to this file.

Start agent sessions with `/workspace/ferrum` as the working directory, so this file loads at startup and again after every context compaction.
