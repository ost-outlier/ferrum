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

/// Context passed to indentation services.
pub struct IndentContext<'a> {
    pub state: &'a EditorState,
    pub pos: usize,
}

/// Function type for indentation services.
pub type IndentServiceFn = Arc<dyn Fn(&IndentContext) -> Option<usize> + Send + Sync>;

/// Facet that registers indentation services.
pub static INDENT_SERVICE: LazyLock<Facet<IndentServiceFn, Vec<IndentServiceFn>>> =
    LazyLock::new(|| Facet::define(|inputs| inputs.to_vec()));

/// Computes the appropriate indentation depth, as a column number, at the start of a given position.
pub fn get_indentation(state: &EditorState, pos: usize) -> Option<usize> {
    let ctx = IndentContext { state, pos };
    for service in state.facet(&INDENT_SERVICE) {
        if let Some(indent) = service(&ctx) {
            return Some(indent);
        }
    }
    None
}
