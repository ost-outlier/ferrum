# ferrum-language / src — API Reference

Language support infrastructure for the Ferrum text editor engine. Direct architectural port of CodeMirror 6's [`@codemirror/language`](https://code.haverbeke.berlin/codemirror/language) and [`@codemirror/streamparser`](https://code.haverbeke.berlin/codemirror/streamparser).

## Features and Subsystems

### 1. Language Definitions & Syntax Trees
- `Language`: Core language extension defining syntax tree parsers and language data facets.
- `LanguageSupport`: Pairs a language with its companion support extensions.
- `ParseContext`: Controls asynchronous and incremental parsing budgets.
- `syntax_tree`, `ensure_syntax_tree`: Safe accessors for current document syntax trees.

### 2. Indentation Infrastructure
- `indent_unit`: Facet specifying the string unit for indentation (defaults to `"  "`).
- `get_indent_unit`: Calculates the effective column width of an indent unit.
- `indent_string`: Formats an indentation depth into tabs and spaces.
- `get_indentation`: Resolves contextual line indentation via syntax trees and `indent_service`.
- `indent_service`: Facet registering language-specific indent calculators.

### 3. Code Folding
- `fold_state`: `StateField<DecorationSet>` managing active folded regions.
- `fold_effect` / `unfold_effect`: Transaction effects triggering fold/unfold actions.
- `fold_code` / `unfold_code` / `toggle_fold`: High-level folding commands.
- `fold_gutter`: Gutter integration rendering fold markers and handles.

### 4. Stream Parser (`@codemirror/streamparser`)
- `StringStream`: Line scanner and tokenizer with regex and character predicates.
- `StreamParser`: State-based tokenizer interface producing syntax tokens and indentation.
- `StreamLanguage`: Adapts a `StreamParser` into a first-class Ferrum `Language` with an incremental syntax tree.

### 5. Syntax Highlighting
- `Tag`: Standard highlighting tags (`keyword`, `number`, `string`, `line_comment`, etc.).
- `HighlightStyle`: Maps tags to visual styling specifications.
- `syntax_highlighting`: Extension providing syntax highlighting via `DecorationSet`.

### 6. Bracket Matching
- `match_brackets`: Finds matching pairs of parenthesis, brackets, and braces.
