//! Indentation services and unit configuration.
//! Faithful port of CodeMirror 6's `@codemirror/language/src/indent.ts`.

use ferrum_state::facet::Facet;
use ferrum_state::state::EditorState;
use std::sync::{Arc, LazyLock};

/// Facet for configuring the indentation unit string (e.g. `"  "`, `"    "`, `"\t"`).
/// Defaults to 2 spaces (`"  "`).
pub static INDENT_UNIT: LazyLock<Facet<String, String>> = LazyLock::new(|| {
    Facet::define(|values: &[String]| {
        if values.is_empty() {
            return "  ".to_string();
        }
        let unit = &values[0];
        if unit.is_empty() {
            panic!("Invalid indent unit: empty");
        }
        let first = unit.chars().next().unwrap();
        if !first.is_whitespace() || unit.chars().any(|c| c != first) {
            panic!("Invalid indent unit: {:?}", unit);
        }
        unit.clone()
    })
});

/// Returns the visual column width of an indent unit for the given state.
pub fn get_indent_unit(state: &EditorState) -> usize {
    let unit = state.facet(&INDENT_UNIT);
    if unit.starts_with('\t') {
        *state.facet(EditorState::tab_size()) * unit.chars().count()
    } else {
        unit.chars().count()
    }
}

/// Creates an indentation string that covers `cols` columns.
pub fn indent_string(state: &EditorState, cols: usize) -> String {
    let unit = state.facet(&INDENT_UNIT);
    let mut result = String::new();
    let tab_size = *state.facet(EditorState::tab_size());

    let mut remaining = cols;
    if unit.starts_with('\t') {
        while remaining >= tab_size {
            result.push('\t');
            remaining -= tab_size;
        }
    }

    let pad_char = if unit.starts_with('\t') {
        ' '
    } else {
        unit.chars().next().unwrap_or(' ')
    };

    for _ in 0..remaining {
        result.push(pad_char);
    }
    result
}

/// Description of a line as reported by IndentContext.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineDescription {
    pub text: String,
    pub from: usize,
}

/// Options controlling IndentContext behavior, such as simulated line breaks and indentation overrides.
#[derive(Clone, Default)]
pub struct IndentContextOptions {
    pub override_indentation: Option<Arc<dyn Fn(usize) -> Option<isize> + Send + Sync>>,
    pub simulate_break: Option<usize>,
    pub simulate_double_break: bool,
}

/// Context passed to indentation services.
pub struct IndentContext<'a> {
    pub state: &'a EditorState,
    pub pos: usize,
    pub options: IndentContextOptions,
}

impl<'a> IndentContext<'a> {
    pub fn new(state: &'a EditorState, pos: usize) -> Self {
        Self {
            state,
            pos,
            options: IndentContextOptions::default(),
        }
    }

    pub fn with_options(state: &'a EditorState, options: IndentContextOptions) -> Self {
        Self {
            state,
            pos: options.simulate_break.unwrap_or(0),
            options,
        }
    }

    /// The visual column width of an indent unit.
    pub fn unit(&self) -> usize {
        get_indent_unit(self.state)
    }

    /// Get a description of the line at the given position, taking simulated line breaks into account.
    pub fn line_at(&self, pos: usize, bias: isize) -> LineDescription {
        let line = self.state.doc().line_at(pos);
        if let Some(sim) = self.options.simulate_break {
            if sim >= line.from && sim <= line.to {
                if self.options.simulate_double_break && sim == pos {
                    return LineDescription {
                        text: String::new(),
                        from: pos,
                    };
                } else if if bias < 0 { sim < pos } else { sim <= pos } {
                    let rel = sim - line.from;
                    let text: String = line.text.chars().skip(rel).collect();
                    return LineDescription {
                        text,
                        from: sim,
                    };
                } else {
                    let rel = sim - line.from;
                    let text: String = line.text.chars().take(rel).collect();
                    return LineDescription {
                        text,
                        from: line.from,
                    };
                }
            }
        }
        LineDescription {
            text: line.text,
            from: line.from,
        }
    }

    /// Get the text directly after `pos`, either the entire line or the next 100 characters.
    pub fn text_after_pos(&self, pos: usize, bias: isize) -> String {
        if self.options.simulate_double_break && Some(pos) == self.options.simulate_break {
            return String::new();
        }
        let line_desc = self.line_at(pos, bias);
        let rel_pos = pos.saturating_sub(line_desc.from);
        line_desc
            .text
            .chars()
            .skip(rel_pos)
            .take(100)
            .collect()
    }

    /// Count visual columns in the given string up to `pos`.
    pub fn count_column(&self, line: &str, pos: usize) -> usize {
        let tab_size = *self.state.facet(EditorState::tab_size());
        ferrum_state::column::count_column(line, tab_size, Some(pos))
    }

    /// Find the column for the given position taking indentation overrides into account.
    pub fn column(&self, pos: usize, bias: isize) -> usize {
        let line_desc = self.line_at(pos, bias);
        let rel_pos = pos.saturating_sub(line_desc.from);
        let mut result = self.count_column(&line_desc.text, rel_pos);
        if let Some(ref override_fn) = self.options.override_indentation {
            if let Some(overridden) = override_fn(line_desc.from) {
                if overridden >= 0 {
                    let non_ws = line_desc
                        .text
                        .find(|c: char| !c.is_whitespace())
                        .unwrap_or(line_desc.text.len());
                    result = (result as isize + overridden - self.count_column(&line_desc.text, non_ws) as isize)
                        .max(0) as usize;
                }
            }
        }
        result
    }

    /// Find the indentation column of the line at the given point.
    pub fn line_indent(&self, pos: usize, bias: isize) -> usize {
        let line_desc = self.line_at(pos, bias);
        if let Some(ref override_fn) = self.options.override_indentation {
            if let Some(overridden) = override_fn(line_desc.from) {
                if overridden >= 0 {
                    return overridden as usize;
                }
            }
        }
        let non_ws = line_desc
            .text
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(line_desc.text.len());
        self.count_column(&line_desc.text, non_ws)
    }
}

/// Function type for indentation services.
pub type IndentServiceFn = Arc<dyn Fn(&IndentContext, usize) -> Option<usize> + Send + Sync>;

/// Facet that registers indentation services.
pub static INDENT_SERVICE: LazyLock<Facet<IndentServiceFn, Vec<IndentServiceFn>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.to_vec()));

/// Computes the appropriate indentation depth, as a column number, at the start of a given position.
pub fn get_indentation(state: &EditorState, pos: usize) -> Option<usize> {
    let ctx = IndentContext::new(state, pos);
    get_indentation_with_context(&ctx, pos)
}

/// Computes the appropriate indentation depth using an explicit IndentContext.
pub fn get_indentation_with_context(context: &IndentContext, pos: usize) -> Option<usize> {
    for service in context.state.facet(&INDENT_SERVICE) {
        if let Some(indent) = service(context, pos) {
            return Some(indent);
        }
    }
    None
}
