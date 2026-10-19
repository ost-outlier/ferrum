use ropey::Rope;
use std::fmt;
use std::ops::Range;

/// Immutable, persistent text buffer backed by `ropey::Rope`.
/// All position coordinates are in Unicode codepoints (`char` indices)
/// unless explicitly specified as byte offsets.
#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    rope: Rope,
}

impl Default for Text {
    fn default() -> Self {
        Self::empty()
    }
}

impl Text {
    /// Creates an empty `Text` buffer.
    pub fn empty() -> Self {
        Self {
            rope: Rope::new(),
        }
    }

    /// Creates a `Text` buffer from a string slice.
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
        }
    }

    /// Returns the length of the text in Unicode codepoints (`char`).
    #[inline]
    pub fn len(&self) -> usize {
        self.rope.len_chars()
    }

    /// Returns the length of the text in Unicode codepoints (`char`).
    #[inline]
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Returns the length of the text in UTF-8 bytes.
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Returns the number of lines in the document (at least 1, even if empty).
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns true if the document contains no characters.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Converts a character index to a 0-indexed line number.
    pub fn char_to_line(&self, char_idx: usize) -> usize {
        let char_idx = char_idx.min(self.len_chars());
        self.rope.char_to_line(char_idx)
    }

    /// Converts a 0-indexed line number to the starting character index of that line.
    pub fn line_to_char(&self, line_idx: usize) -> usize {
        let line_idx = line_idx.min(self.len_lines().saturating_sub(1));
        self.rope.line_to_char(line_idx)
    }

    /// Converts a character index to its byte offset in UTF-8.
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        let char_idx = char_idx.min(self.len_chars());
        self.rope.char_to_byte(char_idx)
    }

    /// Converts a byte offset in UTF-8 to a character index.
    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        let byte_idx = byte_idx.min(self.len_bytes());
        self.rope.byte_to_char(byte_idx)
    }

    /// Creates a `Text` document from an array of line strings.
    pub fn of<S: AsRef<str>>(lines: &[S]) -> Self {
        let joined: String = lines
            .iter()
            .map(|s| s.as_ref())
            .collect::<Vec<_>>()
            .join("\n");
        Self::from_str(&joined)
    }

    /// Retrieves information about a 1-indexed line number (CM6 convention).
    /// Panics with "Invalid line" if line number is 0 or exceeds total lines.
    pub fn line(&self, number: usize) -> Line {
        if number == 0 || number > self.len_lines() {
            panic!("Invalid line: {}", number);
        }
        self.line_by_index(number - 1)
    }

    /// Retrieves information about a 0-indexed line.
    pub fn line_by_index(&self, line_idx: usize) -> Line {
        let line_idx = line_idx.min(self.len_lines().saturating_sub(1));
        let from = self.rope.line_to_char(line_idx);
        let line_slice = self.rope.line(line_idx);
        let len_chars = line_slice.len_chars();

        // Calculate `to` excluding trailing newline characters (\r, \n)
        let mut text_len = len_chars;
        let mut line_str = line_slice.to_string();
        if line_str.ends_with("\r\n") {
            line_str.truncate(line_str.len() - 2);
            text_len = text_len.saturating_sub(2);
        } else if line_str.ends_with('\n') || line_str.ends_with('\r') {
            line_str.truncate(line_str.len() - 1);
            text_len = text_len.saturating_sub(1);
        }

        Line {
            number: line_idx + 1, // 1-indexed for display / CM6 convention
            from,
            to: from + text_len,
            length: text_len,
            text: line_str,
        }
    }

    /// Retrieves the line that contains the given character position.
    /// Panics with "Invalid position" if position exceeds document length.
    pub fn line_at(&self, char_pos: usize) -> Line {
        if char_pos > self.len_chars() {
            panic!("Invalid position: {}", char_pos);
        }
        let line_idx = self.char_to_line(char_pos);
        self.line_by_index(line_idx)
    }

    /// Returns a slice of the text as a String.
    pub fn slice_to_string(&self, range: Range<usize>) -> String {
        let start = range.start.min(self.len_chars());
        let end = range.end.min(self.len_chars()).max(start);
        self.rope.slice(start..end).to_string()
    }

    /// Returns a slice of the text as a String between from and to.
    pub fn slice_string(&self, from: usize, to: usize) -> String {
        self.slice_to_string(from..to)
    }

    /// Returns a new `Text` replacing the given character range with `replacement`.
    pub fn replace(&self, range: Range<usize>, replacement: &str) -> Self {
        let mut new_rope = self.rope.clone();
        let start = range.start.min(new_rope.len_chars());
        let end = range.end.min(new_rope.len_chars()).max(start);

        if start < end {
            new_rope.remove(start..end);
        }
        if !replacement.is_empty() {
            new_rope.insert(start, replacement);
        }

        Self { rope: new_rope }
    }

    /// Returns a new `Text` containing the slice of this text in the given character range.
    pub fn slice(&self, range: Range<usize>) -> Self {
        let start = range.start.min(self.len_chars());
        let end = range.end.min(self.len_chars()).max(start);
        if start == end {
            return Self::empty();
        }
        Self {
            rope: self.rope.slice(start..end).into(),
        }
    }

    /// Appends another `Text` to the end of this one, returning a new `Text`.
    pub fn append(&self, other: &Self) -> Self {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }
        let mut new_rope = self.rope.clone();
        new_rope.append(other.rope.clone());
        Self { rope: new_rope }
    }

    /// Returns the underlying `ropey::Rope` reference.
    pub fn as_rope(&self) -> &Rope {
        &self.rope
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rope)
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Text({:?}, chars={}, lines={})",
            self.slice_to_string(0..self.len_chars().min(50)),
            self.len_chars(),
            self.len_lines()
        )
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

/// Represents line coordinates and extent in a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// 1-indexed line number (CM6 convention).
    pub number: usize,
    /// The starting character offset of the line.
    pub from: usize,
    /// The ending character offset of the line (excluding newline delimiter).
    pub to: usize,
    /// Length of the line text in characters (excluding newline delimiter).
    pub length: usize,
    /// The text of the line (excluding newline delimiter).
    pub text: String,
}
