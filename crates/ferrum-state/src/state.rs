use crate::change::ChangeSet;
use crate::config::Configuration;
use crate::facet::{CompartmentId, Extension, Facet, StateField};
use crate::selection::EditorSelection;
use crate::text::Text;
use crate::transaction::{CompartmentReconfigure, Transaction, TransactionSpec, TRANSACTION_FILTER};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration options passed to `EditorState::create`.
#[derive(Default)]
pub struct EditorStateConfig {
    /// Initial document text.
    pub doc: Option<Text>,
    /// Initial selection.
    pub selection: Option<EditorSelection>,
    /// Extensions to configure the editor.
    pub extensions: Option<Vec<Extension>>,
}

/// The core editor state class.
/// `EditorState` is a persistent (immutable) data structure.
/// Updating the state produces a new `EditorState` via atomic `Transaction`s.
pub struct EditorState {
    doc: Text,
    selection: EditorSelection,
    config: Arc<Configuration>,
    dynamic_values: Vec<Arc<dyn Any + Send + Sync>>,
    field_values: Vec<Arc<dyn Any + Send + Sync>>,
}

impl EditorState {
    /// Creates a new `EditorState` wrapped in an `Arc`.
    pub fn create(config: EditorStateConfig) -> Arc<Self> {
        let doc = config.doc.unwrap_or_default();
        let selection = config.selection.unwrap_or_default();
        let exts = config.extensions.unwrap_or_default();

        let resolved_config = Configuration::resolve(&exts, None);

        let mut state = Self {
            doc,
            selection,
            config: Arc::new(resolved_config),
            dynamic_values: Vec::new(),
            field_values: Vec::new(),
        };

        // Initialize state fields
        let mut field_values = Vec::with_capacity(state.config.state_fields.len());
        for field in &state.config.state_fields {
            field_values.push(field.init(&state));
        }
        state.field_values = field_values;

        // Initialize dynamic facets
        let mut dynamic_values = Vec::with_capacity(state.config.dynamic_facet_slots.len());
        for slot in &state.config.dynamic_facet_slots {
            dynamic_values.push((slot.compute)(&state));
        }
        state.dynamic_values = dynamic_values;

        Arc::new(state)
    }

    /// Returns the current text document.
    pub fn doc(&self) -> &Text {
        &self.doc
    }

    /// Returns the current editor selection.
    pub fn selection(&self) -> &EditorSelection {
        &self.selection
    }

    /// Reads the value of a facet from this state.
    /// If the facet is not registered, returns its default value at zero cost.
    pub fn facet<'a, Input: 'static + Send + Sync + Clone, Output: 'static + Send + Sync + Clone>(
        &'a self,
        facet: &'a Facet<Input, Output>,
    ) -> &'a Output {
        self.config.facet(facet, Some(&self.dynamic_values))
    }

    /// Reads the current value of a registered state field.
    pub fn field<Value: 'static + Send + Sync + Clone>(
        &self,
        field: &StateField<Value>,
    ) -> Option<&Value> {
        self.config.field(field, &self.field_values)
    }

    /// Creates a transaction from a specification.
    /// Runs all registered transaction filters. Returns `None` if any filter rejects the transaction.
    pub fn update(self: &Arc<Self>, spec: TransactionSpec) -> Option<Transaction> {
        let changes = spec.changes.unwrap_or_else(ChangeSet::empty);
        let doc_changed = !changes.is_empty();
        let new_doc = if doc_changed {
            changes.apply(&self.doc)
        } else {
            self.doc.clone()
        };

        let new_selection = spec.selection.or_else(|| {
            if doc_changed {
                // Map current selection ranges forward through the changes
                let mapped_ranges = self
                    .selection
                    .ranges()
                    .iter()
                    .map(|r| {
                        let anchor = changes.map_pos(r.anchor, r.assoc);
                        let head = changes.map_pos(r.head, r.assoc);
                        let (from, to) = (anchor.min(head), anchor.max(head));
                        let clamped_from = from.min(new_doc.len_chars());
                        let clamped_to = to.min(new_doc.len_chars());
                        if r.is_inverted() {
                            crate::selection::SelectionRange::new(clamped_to, clamped_from)
                        } else {
                            crate::selection::SelectionRange::new(clamped_from, clamped_to)
                        }
                    })
                    .collect();
                Some(EditorSelection::create(mapped_ranges, self.selection.main_index()))
            } else {
                None
            }
        });

        let tr = Transaction {
            start_state: self.clone(),
            changes,
            selection: new_selection,
            effects: spec.effects,
            doc: new_doc,
            doc_changed,
        };

        // Run transaction filters
        let filters = self.facet(&TRANSACTION_FILTER);
        for filter in filters {
            if !(filter)(&tr) {
                return None; // Transaction rejected by filter
            }
        }

        Some(tr)
    }

    /// Applies a validated transaction, producing a new immutable `EditorState`.
    pub fn apply(self: &Arc<Self>, tr: Transaction) -> Arc<Self> {
        // Check for compartment reconfiguration effects
        let mut compartment_overrides: HashMap<CompartmentId, Extension> = HashMap::new();
        for effect in tr.effects() {
            if let Some(reconfig) = effect.downcast_ref::<CompartmentReconfigure>() {
                compartment_overrides.insert(reconfig.compartment, reconfig.extension.clone());
            }
        }

        let new_config = if !compartment_overrides.is_empty() {
            // Reconfigure compartments
            let mut merged_compartments = self.config.compartments.clone();
            for (cid, ext) in compartment_overrides {
                merged_compartments.insert(cid, ext);
            }
            Arc::new(Configuration::resolve(
                &self.config.base_extensions,
                Some(&merged_compartments),
            ))
        } else {
            self.config.clone()
        };

        // Update state fields before moving fields out of tr
        let mut new_field_values = Vec::with_capacity(new_config.state_fields.len());
        for (i, field) in new_config.state_fields.iter().enumerate() {
            let old_val = self.field_values.get(i).map(|v| v.as_ref() as &dyn Any);
            if let Some(old) = old_val {
                new_field_values.push(field.update(old, &tr));
            }
        }

        let final_selection = tr
            .selection
            .unwrap_or_else(|| self.selection.clone());
        let final_doc = tr.doc;

        let mut next_state = Self {
            doc: final_doc,
            selection: final_selection,
            config: new_config,
            dynamic_values: Vec::new(),
            field_values: new_field_values,
        };

        // If any newly added state fields need initialization
        while next_state.field_values.len() < next_state.config.state_fields.len() {
            let idx = next_state.field_values.len();
            let init_val = next_state.config.state_fields[idx].init(&next_state);
            next_state.field_values.push(init_val);
        }

        // Recompute dynamic facets
        let mut new_dynamic_values =
            Vec::with_capacity(next_state.config.dynamic_facet_slots.len());
        for slot in &next_state.config.dynamic_facet_slots {
            new_dynamic_values.push((slot.compute)(&next_state));
        }
        next_state.dynamic_values = new_dynamic_values;

        Arc::new(next_state)
    }
}
