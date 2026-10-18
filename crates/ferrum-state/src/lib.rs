//! Core editor state, immutable transaction pipeline, and the Facet extension system
//! for the Ferrum editor engine.

pub mod change;
pub mod config;
pub mod facet;
pub mod selection;
pub mod state;
pub mod text;
pub mod transaction;

// Re-export common types at the crate root for ergonomics
pub use change::{ChangeSet, ChangeSpec};
pub use config::Configuration;
pub use facet::{
    AnyFacet, AnyStateField, Compartment, CompartmentId, Extension, Facet, FacetId, Prec,
    StateField, StateFieldId,
};
pub use selection::{EditorSelection, SelectionRange};
pub use state::{EditorState, EditorStateConfig};
pub use text::{Line, Text};
pub use transaction::{
    CompartmentReconfigure, StateEffect, Transaction, TransactionSpec, TRANSACTION_FILTER,
    TransactionFilterFn,
};

#[cfg(test)]
mod tests;
