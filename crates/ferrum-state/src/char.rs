use unicode_segmentation::UnicodeSegmentation;

/// Distinguishes character categories for word and group navigation (matches CM6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharCategory {
    Word,
    Space,
    Other,
}

/// Categorizes a character according to CodeMirror 6 rules.
pub fn categorize_char(ch: char) -> CharCategory {
    if ch.is_whitespace() {
        CharCategory::Space
    } else if ch.is_alphanumeric() || ch == '_' {
        CharCategory::Word
    } else {
        CharCategory::Other
    }
}

/// Returns the next grapheme cluster break after `pos` (if `forward == true`),
/// or before `pos` (if `forward == false`).
///
/// Handles multi-character Unicode clusters:
/// - CRLF sequences (`\r\n`)
/// - Combining accent marks (e.g. `e` + `\u{0301}`)
/// - Emoji sequences with Zero-Width-Joiners (e.g. `👨‍👩‍👧‍👦`)
/// - Flag emojis (regional indicator pairs)
/// - Skin-tone modifiers
pub fn find_cluster_break(text: &str, pos: usize, forward: bool) -> usize {
    let len = text.chars().count();
    if (forward && pos >= len) || (!forward && pos == 0) {
        return pos;
    }

    // Convert char offset to byte offset
    let byte_offset = text
        .char_indices()
        .nth(pos)
        .map(|(b, _)| b)
        .unwrap_or(text.len());

    let grapheme_indices: Vec<(usize, &str)> = text.grapheme_indices(true).collect();

    if forward {
        for (b_idx, g_str) in grapheme_indices {
            if b_idx > byte_offset {
                let char_idx = text[..b_idx].chars().count();
                return char_idx;
            } else if b_idx == byte_offset {
                let char_idx = text[..b_idx + g_str.len()].chars().count();
                return char_idx;
            }
        }
        len
    } else {
        let mut prev_char_idx = 0;
        for (b_idx, _) in grapheme_indices {
            if b_idx >= byte_offset {
                return prev_char_idx;
            }
            prev_char_idx = text[..b_idx].chars().count();
        }
        prev_char_idx
    }
}

/// Finds the next or previous word/group boundary from `pos`.
pub fn find_word_break(text: &str, pos: usize, forward: bool) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if (forward && pos >= len) || (!forward && pos == 0) {
        return pos;
    }

    let mut current = pos;

    if forward {
        let start_cat = categorize_char(chars[current]);
        while current < len && categorize_char(chars[current]) == start_cat {
            current = find_cluster_break(text, current, true);
        }
        current
    } else {
        let prev_pos = find_cluster_break(text, current, false);
        let start_cat = categorize_char(chars[prev_pos]);
        current = prev_pos;
        while current > 0 {
            let next_prev = find_cluster_break(text, current, false);
            if categorize_char(chars[next_prev]) != start_cat {
                break;
            }
            current = next_prev;
        }
        current
    }
}

/// Finds the next or previous camelCase/snake_case subword boundary from `pos`.
pub fn find_subword_break(text: &str, pos: usize, forward: bool) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if (forward && pos >= len) || (!forward && pos == 0) {
        return pos;
    }

    if forward {
        let mut curr = pos;
        let start_ch = chars[curr];
        let start_cat = categorize_char(start_ch);
        if start_cat != CharCategory::Word || start_ch == '_' {
            while curr < len {
                let ch = chars[curr];
                if (start_ch == '_' && ch == '_')
                    || (start_ch != '_' && categorize_char(ch) == start_cat && ch != '_')
                {
                    curr += 1;
                } else {
                    break;
                }
            }
            return curr;
        }

        let mut saw_upper = false;
        let mut saw_lower = false;

        while curr < len {
            let ch = chars[curr];
            if categorize_char(ch) != CharCategory::Word || ch == '_' {
                break;
            }
            if ch.is_lowercase() {
                saw_lower = true;
            } else if saw_lower {
                break;
            } else {
                if saw_upper && curr + 1 < len {
                    let ahead = chars[curr + 1];
                    if categorize_char(ahead) == CharCategory::Word && ahead.is_lowercase() {
                        break;
                    }
                }
                saw_upper = true;
            }
            curr += 1;
        }
        curr
    } else {
        let mut curr = pos;
        let prev_ch = chars[curr - 1];
        let prev_cat = categorize_char(prev_ch);
        if prev_cat != CharCategory::Word || prev_ch == '_' {
            while curr > 0 {
                let ch = chars[curr - 1];
                if (prev_ch == '_' && ch == '_')
                    || (prev_ch != '_' && categorize_char(ch) == prev_cat && ch != '_')
                {
                    curr -= 1;
                } else {
                    break;
                }
            }
            return curr;
        }

        let mut saw_lower = false;
        let mut saw_upper = false;

        while curr > 0 {
            let ch = chars[curr - 1];
            if categorize_char(ch) != CharCategory::Word || ch == '_' {
                break;
            }
            if ch.is_lowercase() {
                if saw_upper {
                    break;
                }
                saw_lower = true;
            } else {
                curr -= 1;
                saw_upper = true;
                if saw_lower {
                    break;
                }
                continue;
            }
            curr -= 1;
        }
        curr
    }
}

