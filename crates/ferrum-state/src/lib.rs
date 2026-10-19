//! Core editor state, immutable transaction pipeline, and the Facet extension system
//! for the Ferrum editor engine.

pub mod change;
pub mod char;
pub mod column;
pub mod config;
pub mod facet;
pub mod rangeset;
pub mod selection;
pub mod state;
pub mod text;
pub mod transaction;

// Re-export common types at the crate root for ergonomics
pub use change::{ChangeDesc, ChangeSet, ChangeSpec, Changes, MapMode};
pub use column::{count_column, find_column};
pub use char::{
    categorize_char, find_cluster_break, find_subword_break, find_word_break, CharCategory,
};
pub use config::Configuration;
pub use facet::{
    AnyFacet, AnyStateField, Compartment, CompartmentId, Extension, Facet, FacetId, Prec,
    StateField, StateFieldId,
};
pub use rangeset::{Chunk, Range, RangeSet, RangeSetBuilder, RangeSetUpdate, RangeValue};
pub use selection::{EditorSelection, SelectionRange};
pub use state::{ChangeByRangeResult, EditorState, EditorStateConfig, RangeResult};
pub use text::{Line, Text};
pub use transaction::{
    AddToHistory, Annotation, CompartmentReconfigure, IsolateHistory, StateEffect, Transaction,
    TransactionSpec, TransactionTime, UserEvent, TRANSACTION_FILTER, TransactionFilterFn,
};

#[cfg(test)]
mod tests;
