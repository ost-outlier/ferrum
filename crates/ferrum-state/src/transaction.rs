use crate::change::{ChangeSet, ChangeSpec};
use crate::facet::{CompartmentId, Extension, Facet};
use crate::selection::EditorSelection;
use crate::state::EditorState;
use crate::text::Text;
use std::any::{Any, TypeId};
use std::sync::Arc;

/// A custom dynamic effect attached to a transaction.
#[derive(Clone)]
pub struct StateEffect {
    pub(crate) type_id: TypeId,
    pub(crate) value: Arc<dyn Any + Send + Sync>,
}

impl StateEffect {
    /// Creates a state effect of a specific type.
    pub fn of<T: 'static + Send + Sync + Clone>(value: T) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            value: Arc::new(value),
        }
    }

    /// Checks if this effect matches the given type.
    pub fn is<T: 'static>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    /// Downcasts the effect value to the given type reference.
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            self.value.downcast_ref::<T>()
        } else {
            None
        }
    }
}

/// An effect representing a dynamic reconfiguration of an extension compartment.
#[derive(Clone)]
pub struct CompartmentReconfigure {
    pub compartment: CompartmentId,
    pub extension: Extension,
}

impl crate::facet::Compartment {
    /// Creates a transaction effect that reconfigures this compartment with a new extension.
    pub fn reconfigure(&self, extension: Extension) -> StateEffect {
        StateEffect::of(CompartmentReconfigure {
            compartment: self.id,
            extension,
        })
    }
}

/// Specification for creating an atomic transaction against an `EditorState`.
#[derive(Default, Clone)]
pub struct TransactionSpec {
    /// Changes to be applied to the document.
    pub changes: Option<ChangeSet>,
    /// New selection to be set.
    pub selection: Option<EditorSelection>,
    /// Effects to be executed.
    pub effects: Vec<StateEffect>,
}

impl TransactionSpec {
    /// Creates an empty transaction specification.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds changes to the transaction specification.
    pub fn with_changes(mut self, changes: ChangeSet) -> Self {
        self.changes = Some(changes);
        self
    }

    /// Adds a single change spec to the transaction specification.
    pub fn with_change(mut self, change: ChangeSpec) -> Self {
        self.changes = Some(ChangeSet::single(change));
        self
    }

    /// Sets the new selection.
    pub fn with_selection(mut self, selection: EditorSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    /// Adds an effect to the transaction specification.
    pub fn with_effect(mut self, effect: StateEffect) -> Self {
        self.effects.push(effect);
        self
    }
}

/// A transaction represents an atomic update to an `EditorState`.
/// In Ferrum, states are immutable — transactions take an existing state and produce a new one.
pub struct Transaction {
    pub(crate) start_state: Arc<EditorState>,
    pub(crate) changes: ChangeSet,
    pub(crate) selection: Option<EditorSelection>,
    pub(crate) effects: Vec<StateEffect>,
    pub(crate) doc: Text,
    pub(crate) doc_changed: bool,
}

impl Transaction {
    /// The state that this transaction starts from.
    pub fn start_state(&self) -> &Arc<EditorState> {
        &self.start_state
    }

    /// The changes applied in this transaction.
    pub fn changes(&self) -> &ChangeSet {
        &self.changes
    }

    /// The new selection produced by this transaction, if specified.
    pub fn selection(&self) -> Option<&EditorSelection> {
        self.selection.as_ref()
    }

    /// All effects attached to this transaction.
    pub fn effects(&self) -> &[StateEffect] {
        &self.effects
    }

    /// The resulting document after applying `changes`.
    pub fn doc(&self) -> &Text {
        &self.doc
    }

    /// True if the document content changed.
    pub fn doc_changed(&self) -> bool {
        self.doc_changed
    }
}

// Facet for transaction filtering: returning `false` aborts or rejects the transaction.
pub type TransactionFilterFn = Arc<dyn Fn(&Transaction) -> bool + Send + Sync>;

/// Facet that registers transaction filters. If any filter returns `false`, the transaction is cancelled.
pub static TRANSACTION_FILTER: std::sync::LazyLock<Facet<TransactionFilterFn, Vec<TransactionFilterFn>>> =
    std::sync::LazyLock::new(|| Facet::define(|inputs| inputs.to_vec()));
