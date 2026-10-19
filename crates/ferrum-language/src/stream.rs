//! Stream parser tokenizer and line scanner.
//! Faithful port of CodeMirror 6's `@codemirror/streamparser`.

use crate::tree::{SyntaxNode, SyntaxTree};
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
pub struct StreamLanguage<State: Clone + 'static> {
    pub parser: Arc<dyn StreamParser<State>>,
}

impl<State: Clone + 'static> StreamLanguage<State> {
    pub fn define(parser: impl StreamParser<State> + 'static) -> Self {
        Self {
            parser: Arc::new(parser),
        }
    }

    /// Parses text into a syntax tree using the stream parser.
    pub fn parse_text(&self, text: &str) -> SyntaxTree {
        let mut root = SyntaxNode::new("Document", 0, text.len());
        let mut state = self.parser.start_state(2);

        let mut offset = 0;
        for line in text.split('\n') {
            let mut stream = StringStream::new(line, 4, 2);
            while !stream.eol() {
                stream.start = stream.pos;
                let token_start = offset + stream.pos;
                if let Some(tag) = self.parser.token(&mut stream, &mut state) {
                    let token_end = offset + stream.pos;
                    root.children.push(SyntaxNode::new(tag, token_start, token_end));
                } else if stream.pos == stream.start {
                    stream.next();
                }
            }
            offset += line.len() + 1;
        }

        SyntaxTree::new(root, text.len())
    }
}
