# ferrum-language

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io)
[![Documentation](https://docs.rs/ferrum-language/badge.svg)](https://docs.rs/ferrum-language)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

> **Language support infrastructure, syntax trees, and indentation engine for Ferrum.**  
> Direct architectural counterpart to [`@codemirror/language`](https://code.haverbeke.berlin/codemirror/language) and [`@codemirror/streamparser`](https://code.haverbeke.berlin/codemirror/streamparser).

---

## Overview

`ferrum-language` provides the language infrastructure for the Ferrum editor suite:
- Contextual, syntax-driven indentation (`get_indentation`, `indent_unit`, `IndentContext`).
- Bracket matching and block delimiter analysis.
- Language configuration and facets (`Language`, `LanguageSupport`, `language_data`).
- Incremental stream-based tokenizer and parser (`StreamLanguage`, `StringStream`, `StreamParser`).
- Syntax highlighting and code folding foundations.

---

## Key Features

### 1. Contextual Indentation Engine
- `indent_service`: Facet registering language-specific indent calculators.
- `INDENT_UNIT`: Facet controlling indent string and tab widths (default `"  "`).
- `get_indentation(state, pos)`: Resolves column indentation for a given line based on syntax context.
- `IndentContext`: Context supporting virtual breaks (`simulate_break`, `simulate_double_break`) and relative indentation overrides (`override_indentation`).

### 2. Stream Parser (`@codemirror/streamparser`)
- `StringStream`: Gap-aware line scanner and tokenizer with character predicates and regex matches.
- `StreamParser`: State-machine tokenizer defining tokens and block structure.
- `StreamLanguage`: Adapts any `StreamParser` into an incremental Ferrum `Language`.

### 3. Syntax Trees & Code Folding
- StateField-driven tree snapshots.
- `fold_code` / `unfold_code` and fold gutters managed via `DecorationSet`.

---

## Testing & Parity

All test suites are ported directly from CodeMirror 6 and pass with 100% behavioral parity (15/15 tests):
- Contextual indentation & bracket nesting $\rightarrow$ `tests::test_syntax`
- Code folding $\rightarrow$ `tests::test_fold`
- Stream parser tokenization $\rightarrow$ `tests::test_stream_parser`

Run test suite:
```bash
cargo test -p ferrum-language --lib
```

---

## License

Dual-licensed under either of:
- MIT License ([LICENSE-MIT](https://github.com/your-org/ferrum/blob/main/LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/your-org/ferrum/blob/main/LICENSE-APACHE))

Strict Clean-Room implementation: contains zero copyleft or GPL code.
