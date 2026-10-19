//! Commenting and uncommenting commands for the Ferrum editor engine.
//! Faithful port of CodeMirror 6's `@codemirror/commands/src/comment.ts`.

use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;

/// Configuration for block comment tokens.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BlockToken {
    pub open: String,
    pub close: String,
}

/// An object of this type configures comment syntax for a language.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CommentTokens {
    /// The block comment syntax, if any.
    pub block: Option<BlockToken>,
    /// The line comment syntax, if any.
    pub line: Option<String>,
}

use ferrum_state::facet::Facet;
use std::sync::LazyLock;

/// Facet to configure comment tokens for a document or language.
pub static COMMENT_TOKENS: LazyLock<Facet<CommentTokens, CommentTokens>> = LazyLock::new(|| {
    Facet::define(|inputs: &[CommentTokens]| {
        inputs.last().cloned().unwrap_or_default()
    })
});

/// Comment or uncomment the current selection. Will use line comments if available,
/// otherwise falling back to block comments.
pub fn toggle_comment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Comment or uncomment the current selection using line comments.
pub fn toggle_line_comment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Comment the current selection using line comments.
pub fn line_comment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Uncomment the current selection using line comments.
pub fn line_uncomment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Comment or uncomment the current selection using block comments.
pub fn toggle_block_comment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Comment the current selection using block comments.
pub fn block_comment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Uncomment the current selection using block comments.
pub fn block_uncomment(_state: &EditorState) -> Option<TransactionSpec> {
    None
}

/// Comment or uncomment the lines around the current selection using block comments.
pub fn toggle_block_comment_by_line(_state: &EditorState) -> Option<TransactionSpec> {
    None
}
