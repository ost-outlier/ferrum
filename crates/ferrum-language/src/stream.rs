//! Stream parser tokenizer and line scanner.
//! Faithful port of CodeMirror 6's `@codemirror/streamparser`.

use crate::indent::{IndentContext, IndentServiceFn, INDENT_SERVICE};
use crate::language::{Language, LANGUAGE};
use crate::tree::{Parser, SyntaxNode, SyntaxTree};
use ferrum_state::facet::Extension;
use regex::Regex;
use std::sync::Arc;

/// Encapsulates a single line of input for stream syntax tokenizers.
#[derive(Debug, Clone)]
pub struct StringStream {
    pub string: String,
    pub pos: usize,
    pub start: usize,
    pub tab_size: usize,
    pub indent_unit: usize,
}

impl StringStream {
    pub fn new(string: impl Into<String>, tab_size: usize, indent_unit: usize) -> Self {
        Self {
            string: string.into(),
            pos: 0,
            start: 0,
            tab_size,
            indent_unit,
        }
    }

    /// True if we are at the end of the line.
    pub fn eol(&self) -> bool {
        self.pos >= self.string.len()
    }

    /// True if we are at the start of the line.
    pub fn sol(&self) -> bool {
        self.pos == 0
    }

    /// Peek at the next character without advancing the stream.
    pub fn peek(&self) -> Option<char> {
        self.string[self.pos..].chars().next()
    }

    /// Read the next character and advance position.
    pub fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    /// Match the next character against a predicate function.
    pub fn eat<F>(&mut self, mut predicate: F) -> Option<char>
    where
        F: FnMut(char) -> bool,
    {
        if let Some(ch) = self.peek() {
            if predicate(ch) {
                self.pos += ch.len_utf8();
                return Some(ch);
            }
        }
        None
    }

    /// Consume whitespace characters. Returns true if any whitespace was consumed.
    pub fn eat_space(&mut self) -> bool {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
        self.pos > start
    }

    /// Continue matching characters while predicate holds true.
    pub fn eat_while<F>(&mut self, mut predicate: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if predicate(ch) {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
        self.pos > start
    }

    /// Checks if the remaining text matches the given prefix string. Advances pos if matched.
    pub fn match_str(&mut self, pattern: &str) -> bool {
        if self.string[self.pos..].starts_with(pattern) {
            self.pos += pattern.len();
            true
        } else {
            false
        }
    }

    /// Checks if the remaining text matches the given regex pattern. Advances pos if matched.
    pub fn match_regex(&mut self, pattern: &str) -> bool {
        if let Ok(re) = Regex::new(&format!("^{}", pattern)) {
            if let Some(mat) = re.find(&self.string[self.pos..]) {
                self.pos += mat.end();
                return true;
            }
        }
        false
    }

    /// Returns the text slice of the current token being read.
    pub fn current(&self) -> &str {
        &self.string[self.start..self.pos]
    }
}

/// Interface for state-based stream tokenizers.
pub trait StreamParser<State: Clone>: Send + Sync {
    /// Produces a start state for the parser.
    fn start_state(&self, indent_unit: usize) -> State;

    /// Reads one token, advancing the stream past it, returning a style tag name.
    fn token(&self, stream: &mut StringStream, state: &mut State) -> Option<&'static str>;

    /// Computes automatic indentation for the line starting with given state and text.
    fn indent(&self, state: &State, text_after: &str, indent_unit: usize) -> Option<usize> {
        let _ = (state, text_after, indent_unit);
        None
    }

    /// Notifies the parser of a blank line.
    fn blank_line(&self, state: &mut State, indent_unit: usize) {
        let _ = (state, indent_unit);
    }
}

/// Adapts a `StreamParser` into a full Ferrum `Language`.
#[derive(Clone)]
pub struct StreamLanguage<State: Clone + Send + Sync + 'static> {
    pub parser: Arc<dyn StreamParser<State>>,
}

impl<State: Clone + Send + Sync + 'static> StreamLanguage<State> {
    pub fn define(parser: impl StreamParser<State> + 'static) -> Self {
        Self {
            parser: Arc::new(parser),
        }
    }

    /// Returns an extension configuring this language and its indentation service on an editor state.
    pub fn extension(&self) -> Extension {
        let parser = self.parser.clone();
        let lang = Language::new("stream", Arc::new(self.clone()));
        let indent_fn: IndentServiceFn = Arc::new(move |cx: &IndentContext, pos: usize| {
            let line = cx.state.doc().line_at(pos);
            let mut state = parser.start_state(2);
            let mut cur_line_start = 0;
            while cur_line_start < line.from {
                let cur_line = cx.state.doc().line_at(cur_line_start);
                let cur_text = &cur_line.text;
                if !cur_text.is_empty() {
                    let mut stream = StringStream::new(cur_text.as_str(), 4, 2);
                    while !stream.eol() {
                        parser.token(&mut stream, &mut state);
                    }
                } else {
                    parser.blank_line(&mut state, 2);
                }
                cur_line_start = cur_line.to + 1;
            }
            let line_text = &line.text;
            let trimmed = line_text.trim_start();
            parser.indent(&state, trimmed, 2)
        });

        Extension::from(vec![
            LANGUAGE.of(lang),
            INDENT_SERVICE.of(indent_fn),
        ])
    }

    /// Parses text into a syntax tree using the stream parser.
    pub fn parse_text(&self, text: &str) -> SyntaxTree {
        self.parse_with_ranges(text, &[])
    }

    /// Parses text into a syntax tree using the stream parser, optionally restricting to given ranges (supporting gaps).
    pub fn parse_with_ranges(&self, text: &str, ranges: &[(usize, usize)]) -> SyntaxTree {
        let mut root = SyntaxNode::new("Document", 0, text.len());
        let mut state = self.parser.start_state(2);

        let default_range = [(0, text.len())];
        let active_ranges = if ranges.is_empty() {
            &default_range[..]
        } else {
            ranges
        };

        let mut offset = 0;
        for line in text.split('\n') {
            let line_len = line.len();
            let line_start = offset;
            let line_end = offset + line_len;

            // Collect intersections with active ranges on this line
            let mut stitched_line = String::new();
            // Vector of (stitched_start, stitched_len, doc_start)
            let mut pieces: Vec<(usize, usize, usize)> = Vec::new();

            for &(r_from, r_to) in active_ranges {
                if r_from < line_end && r_to > line_start {
                    let inter_from = r_from.max(line_start);
                    let inter_to = r_to.min(line_end);
                    if inter_to > inter_from {
                        let rel_from = inter_from - line_start;
                        let rel_to = inter_to - line_start;
                        let piece_str = &line[rel_from..rel_to];
                        let stitched_start = stitched_line.len();
                        stitched_line.push_str(piece_str);
                        pieces.push((stitched_start, piece_str.len(), inter_from));
                    }
                }
            }

            if pieces.is_empty() {
                offset += line_len + 1;
                continue;
            }

            let pos_to_doc = |stitched_pos: usize, is_end: bool| -> usize {
                if stitched_pos == 0 {
                    return pieces[0].2;
                }
                for &(s_start, s_len, d_start) in &pieces {
                    if is_end {
                        if stitched_pos <= s_start + s_len {
                            return d_start + (stitched_pos - s_start);
                        }
                    } else {
                        if stitched_pos < s_start + s_len {
                            return d_start + (stitched_pos - s_start);
                        }
                    }
                }
                let last = pieces.last().unwrap();
                last.2 + last.1
            };

            let mut stream = StringStream::new(&stitched_line, 4, 2);
            while !stream.eol() {
                stream.start = stream.pos;
                let token_start = pos_to_doc(stream.pos, false);
                if let Some(tag) = self.parser.token(&mut stream, &mut state) {
                    let token_end = pos_to_doc(stream.pos, true);
                    root.children.push(SyntaxNode::new(tag, token_start, token_end));
                } else if stream.pos == stream.start {
                    stream.next();
                }
            }

            offset += line_len + 1;
        }

        SyntaxTree::new(root, text.len())
    }
}

impl<State: Clone + Send + Sync + 'static> Parser for StreamLanguage<State> {
    fn parse(&self, text: &str, _fragments: &[SyntaxTree], ranges: &[(usize, usize)]) -> SyntaxTree {
        self.parse_with_ranges(text, ranges)
    }
}
