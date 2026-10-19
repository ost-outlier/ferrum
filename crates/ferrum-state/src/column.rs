//! Column measurement and navigation with tab and unicode grapheme cluster support.
//! 1:1 parity with CodeMirror 6 `@codemirror/state/src/column.ts`.

use crate::char::find_cluster_break;

/// Count the column position at the given offset into the string,
/// taking extending characters and tab size into account.
pub fn count_column(string: &str, tab_size: usize, to: Option<usize>) -> usize {
    let limit = to.unwrap_or_else(|| string.chars().count());
    let mut n = 0;
    let mut i = 0;
    let chars: Vec<char> = string.chars().collect();

    while i < limit && i < chars.len() {
        if chars[i] == '\t' {
            n += tab_size - (n % tab_size);
            i += 1;
        } else {
            n += 1;
            let next_i = find_cluster_break(string, i, true);
            i = if next_i > i { next_i } else { i + 1 };
        }
    }
    n
}

/// Find the offset that corresponds to the given column position in a string.
pub fn find_column(string: &str, col: usize, tab_size: usize, strict: bool) -> Option<usize> {
    let chars: Vec<char> = string.chars().collect();
    let mut n = 0;
    let mut i = 0;

    loop {
        if n >= col {
            return Some(i);
        }
        if i >= chars.len() {
            break;
        }
        if chars[i] == '\t' {
            n += tab_size - (n % tab_size);
            i += 1;
        } else {
            n += 1;
            let next_i = find_cluster_break(string, i, true);
            i = if next_i > i { next_i } else { i + 1 };
        }
    }

    if strict {
        None
    } else {
        Some(chars.len())
    }
}
