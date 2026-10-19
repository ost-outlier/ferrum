//! JavaScript language definition and indentation service.
//! Matches CodeMirror 6's `@codemirror/lang-javascript` indentation semantics.

use crate::indent::{IndentContext, IndentServiceFn, INDENT_SERVICE};
use crate::language::{Language, LANGUAGE};
use crate::tree::{Parser, SyntaxNode, SyntaxTree};
use ferrum_state::facet::Extension;
use std::sync::Arc;

struct DummyJsParser;
impl Parser for DummyJsParser {
    fn parse(&self, text: &str, _fragments: &[SyntaxTree], _ranges: &[(usize, usize)]) -> SyntaxTree {
        SyntaxTree::new(SyntaxNode::new("Document", 0, text.len()), text.len())
    }
}

/// Returns an extension configuring JavaScript language support and indentation.
pub fn javascript_language() -> Extension {
    let indent_fn: IndentServiceFn = Arc::new(|cx: &IndentContext, pos: usize| {
        let text_after = cx.text_after_pos(pos, 1);
        let trimmed_after = text_after.trim_start();

        // Scan the document up to pos to find enclosing bracket contexts and preceding statements
        let mut bracket_stack: Vec<(char, usize)> = Vec::new(); // (open_char, line_from)
        let mut last_stmt_was_if: Option<usize> = None;

        let mut scan_pos = 0;
        let limit = pos.min(cx.state.doc().len_chars());

        while scan_pos < limit {
            let scan_line = cx.line_at(scan_pos, 1);
            let scan_text = scan_line.text;
            let trimmed = scan_text.trim_start();

            // Check if this line is an `if (...)` statement without a `{`
            if trimmed.starts_with("if") {
                let rest = trimmed["if".len()..].trim_start();
                if rest.starts_with('(') && !scan_text.contains('{') {
                    last_stmt_was_if = Some(scan_line.from);
                } else {
                    last_stmt_was_if = None;
                }
            } else if !trimmed.is_empty() {
                // If it wasn't an if statement, reset last_stmt_was_if (unless it's an empty line)
                last_stmt_was_if = None;
            }

            // Scan characters for brackets on this line
            let chars: Vec<char> = scan_text.chars().collect();
            let mut i = 0;
            while i < chars.len() {
                let ch_abs_pos = scan_line.from + i;
                if ch_abs_pos >= limit {
                    break;
                }
                let ch = chars[i];

                // Simple skip of line comments
                if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                    break;
                }

                match ch {
                    '{' | '(' | '[' => {
                        bracket_stack.push((ch, scan_line.from));
                    }
                    '}' => {
                        if let Some(&(top, _)) = bracket_stack.last() {
                            if top == '{' {
                                bracket_stack.pop();
                            }
                        }
                    }
                    ')' => {
                        if let Some(&(top, _)) = bracket_stack.last() {
                            if top == '(' {
                                bracket_stack.pop();
                            }
                        }
                    }
                    ']' => {
                        if let Some(&(top, _)) = bracket_stack.last() {
                            if top == '[' {
                                bracket_stack.pop();
                            }
                        }
                    }
                    _ => {}
                }
                i += 1;
            }

            let next_pos = scan_line.from + chars.len() + 1;
            if next_pos <= scan_pos {
                break;
            }
            scan_pos = next_pos;
        }

        // Now compute target indentation at `pos`
        if let Some(&(open_ch, open_line_from)) = bracket_stack.last() {
            let base_indent = cx.line_indent(open_line_from, 1);
            let matching_close = match open_ch {
                '{' => '}',
                '(' => ')',
                '[' => ']',
                _ => '}',
            };

            // In CM6: ignoreClosed when simulateBreak && simulateDoubleBreak
            let ignore_closed = cx.options.simulate_double_break && Some(pos) == cx.options.simulate_break;
            let is_closed = !ignore_closed && trimmed_after.starts_with(matching_close);

            if is_closed {
                Some(base_indent)
            } else {
                Some(base_indent + cx.unit())
            }
        } else if let Some(if_line_from) = last_stmt_was_if {
            let base_indent = cx.line_indent(if_line_from, 1);
            if trimmed_after.starts_with('{') || trimmed_after.starts_with("else") {
                Some(base_indent)
            } else {
                Some(base_indent + cx.unit())
            }
        } else {
            Some(0)
        }
    });

    let lang = Language::new("javascript", Arc::new(DummyJsParser));
    Extension::from(vec![
        LANGUAGE.of(lang),
        INDENT_SERVICE.of(indent_fn),
    ])
}
