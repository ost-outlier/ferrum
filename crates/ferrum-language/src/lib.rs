//! `ferrum-language` - Language support infrastructure, syntax trees, indentation, and folding.
//! Direct architectural port of CodeMirror 6's `@codemirror/language` and `@codemirror/streamparser`.

pub mod fold;
pub mod highlight;
pub mod indent;
pub mod language;
pub mod stream;
pub mod tree;

pub use fold::{fold_state, FoldEffect, FoldRange, UnfoldEffect, UnfoldRange};
pub use highlight::Tag;
pub use indent::{get_indent_unit, get_indentation, indent_string, IndentContext, INDENT_SERVICE, INDENT_UNIT};
pub use language::{ensure_syntax_tree, syntax_tree, Language, LanguageSupport, LANGUAGE};
pub use stream::{StreamLanguage, StreamParser, StringStream};
pub use tree::{NodeType, ParseContext, Parser, SyntaxNode, SyntaxTree};

#[cfg(test)]
mod tests;
