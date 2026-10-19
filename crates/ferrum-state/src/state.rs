use crate::change::{ChangeSet, ChangeSpec, Changes};
use crate::config::Configuration;
use crate::facet::{CompartmentId, Extension, Facet, StateField};
use crate::selection::{EditorSelection, SelectionRange};
use crate::text::Text;
use crate::transaction::{
    CompartmentReconfigure, StateEffect, Transaction, TransactionSpec, TRANSACTION_FILTER,
};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

static ALLOW_MULTIPLE_SELECTIONS: LazyLock<Facet<bool, bool>> = LazyLock::new(|| {
    Facet::define_static(|values: &[bool]| values.iter().any(|&v| v))
});

static TAB_SIZE: LazyLock<Facet<usize, usize>> = LazyLock::new(|| {
    Facet::define(|values: &[usize]| values.first().copied().unwrap_or(4))
});

static LINE_SEPARATOR: LazyLock<Facet<String, Option<String>>> = LazyLock::new(|| {
    Facet::define_static(|values: &[String]| values.first().cloned())
});

/// Result of running a function on a selection range in `change_by_range`.
#[derive(Clone, Debug)]
pub struct RangeResult {
    pub range: SelectionRange,
    pub changes: Option<Changes>,
    pub effects: Vec<StateEffect>,
}

impl RangeResult {
    /// Creates a range result with no changes and the specified new range.
    pub fn range(range: SelectionRange) -> Self {
        Self {
            range,
            changes: None,
            effects: Vec::new(),
        }
    }

    /// Attaches changes to this range result.
    pub fn with_changes(mut self, changes: impl Into<Changes>) -> Self {
        self.changes = Some(changes.into());
        self
    }

    /// Attaches an effect to this range result.
    pub fn with_effect(mut self, effect: StateEffect) -> Self {
        self.effects.push(effect);
        self
    }
}

/// The aggregated result of `change_by_range`.
#[derive(Clone)]
pub struct ChangeByRangeResult {
    pub changes: ChangeSet,
    pub selection: EditorSelection,
    pub effects: Vec<StateEffect>,
}

impl ChangeByRangeResult {
    /// Converts this result into a `TransactionSpec`.
    pub fn into_spec(self) -> TransactionSpec {
        TransactionSpec {
            changes: Some(Changes::Set(self.changes)),
            selection: Some(self.selection),
            effects: self.effects,
            annotations: Vec::new(),
        }
    }
}


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

    /// Accessor for the canonical `allowMultipleSelections` facet.
    pub fn allow_multiple_selections() -> &'static Facet<bool, bool> {
        &ALLOW_MULTIPLE_SELECTIONS
    }

    /// Accessor for the canonical `tabSize` facet.
    pub fn tab_size() -> &'static Facet<usize, usize> {
        &TAB_SIZE
    }

    /// Accessor for the canonical `lineSeparator` facet.
    pub fn line_separator() -> &'static Facet<String, Option<String>> {
        &LINE_SEPARATOR
    }

    /// Returns a function that can categorize characters into CharCategory at the given position.
    pub fn char_categorizer(&self, _at: usize) -> impl Fn(&str) -> crate::char::CharCategory {
        |ch: &str| {
            if let Some(c) = ch.chars().next() {
                crate::char::categorize_char(c)
            } else {
                crate::char::CharCategory::Other
            }
        }
    }

    /// Creates a `ChangeSet` from a `ChangeSpec` for this state's document.
    pub fn changes(&self, spec: ChangeSpec) -> ChangeSet {
        ChangeSet::of_spec(spec, self.doc.len_chars())
    }

    /// Creates a set of changes and a new selection by running the given function
    /// for each range in the active selection, merging concurrent changes using
    /// Operational Transformation (matching CM6's `changeByRange`).
    pub fn change_by_range<F>(&self, mut f: F) -> ChangeByRangeResult
    where
        F: FnMut(&SelectionRange) -> RangeResult,
    {
        let sel = &self.selection;
        let result1 = f(&sel.ranges()[0]);
        let mut changes = match result1.changes {
            Some(c) => c.to_changeset(self.doc.len_chars()),
            None => ChangeSet::empty(self.doc.len_chars()),
        };
        let mut ranges = vec![result1.range];
        let mut effects = result1.effects;

        for i in 1..sel.ranges().len() {
            let result = f(&sel.ranges()[i]);
            let new_changes = match result.changes {
                Some(c) => c.to_changeset(self.doc.len_chars()),
                None => ChangeSet::empty(self.doc.len_chars()),
            };
            let new_mapped = new_changes.map(changes.desc(), false);
            for j in 0..i {
                ranges[j] = ranges[j].map(new_mapped.desc(), -1);
            }
            let map_by = changes.desc().map_desc(new_changes.desc(), true);
            ranges.push(result.range.map(&map_by, -1));
            changes = changes.compose(&new_mapped);
            effects.extend(result.effects);
        }

        ChangeByRangeResult {
            changes,
            selection: EditorSelection::create(ranges, sel.main_index()),
            effects,
        }
    }

    /// Creates a transaction spec that replaces every selection range with the given text.
    pub fn replace_selection(&self, text: impl Into<Text>) -> TransactionSpec {
        let text = text.into();
        let len = text.len_chars();
        self.change_by_range(|range| {
            RangeResult::range(SelectionRange::cursor(range.from() + len))
                .with_changes(Changes::Single(ChangeSpec::replace(range.from(), range.to(), text.clone())))
        })
        .into_spec()
    }

    /// Creates a transaction from a specification.
    /// Runs all registered transaction filters. Returns `None` if any filter rejects the transaction.
    pub fn update(self: &Arc<Self>, spec: TransactionSpec) -> Option<Transaction> {
        let changes = spec
            .changes
            .map(|c| c.to_changeset(self.doc.len_chars()))
            .unwrap_or_else(|| ChangeSet::empty(self.doc.len_chars()));
        let doc_changed = !changes.is_empty();
        let new_doc = if doc_changed {
            changes.apply(&self.doc)
        } else {
            self.doc.clone()
        };

        let new_selection = spec.selection.or_else(|| {
            if doc_changed {
                Some(self.selection.map(changes.desc(), -1))
            } else {
                None
            }
        });

        let mut annotations = spec.annotations;
        if !annotations.iter().any(|a| a.downcast_ref::<crate::transaction::TransactionTime>().is_some()) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            annotations.push(crate::transaction::Annotation::of(crate::transaction::TransactionTime(now)));
        }

        let tr = Transaction {
            start_state: self.clone(),
            changes,
            selection: new_selection,
            effects: spec.effects,
            annotations,
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

        // Update state fields: look up existing fields by id, or initialize and update newly added fields
        let mut new_field_values = Vec::with_capacity(new_config.state_fields.len());
        for field in new_config.state_fields.iter() {
            if let Some(&old_idx) = self.config.field_map.get(&field.id()) {
                let old = self.field_values[old_idx].as_ref();
                new_field_values.push(field.update(old, &tr));
            } else {
                let init_val = field.init(self);
                new_field_values.push(field.update(init_val.as_ref(), &tr));
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
