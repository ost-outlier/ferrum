use std::any::Any;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

static NEXT_FACET_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_FIELD_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_COMPARTMENT_ID: AtomicUsize = AtomicUsize::new(1);

/// Unique identifier for a Facet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FacetId(pub(crate) usize);

/// Unique identifier for a StateField.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StateFieldId(pub(crate) usize);

/// Unique identifier for an Extension Compartment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CompartmentId(pub(crate) usize);

/// Precedence ordering for extensions (matches CM6: 0 = Highest, 4 = Lowest).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prec {
    Highest = 0,
    High = 1,
    Default = 2,
    Low = 3,
    Lowest = 4,
}

// Forward declarations of EditorState and Transaction
use crate::state::EditorState;
use crate::transaction::Transaction;

/// Trait to erase types from a Facet for homogeneous storage and combination in Configuration.
pub trait AnyFacet: Send + Sync {
    fn id(&self) -> FacetId;
    fn is_static(&self) -> bool;
    fn combine_erased(&self, inputs: &[Arc<dyn Any + Send + Sync>]) -> Arc<dyn Any + Send + Sync>;
    fn default_erased(&self) -> Arc<dyn Any + Send + Sync>;
}

/// Trait to erase type from StateField for heterogeneous storage in Configuration.
pub trait AnyStateField: Send + Sync {
    fn id(&self) -> StateFieldId;
    fn init(&self, state: &EditorState) -> Arc<dyn Any + Send + Sync>;
    fn update(&self, value: &dyn Any, tr: &Transaction) -> Arc<dyn Any + Send + Sync>;
}

/// A strongly typed point of extension. Multiple extensions contribute `Input` values,
/// and the facet automatically combines them into a single `Output` value.
#[derive(Clone)]
pub struct Facet<Input: 'static, Output: 'static = Vec<Input>> {
    pub(crate) id: FacetId,
    pub(crate) combine_fn: Arc<dyn Fn(&[Input]) -> Output + Send + Sync>,
    pub(crate) default_value: Arc<Output>,
    pub(crate) is_static: bool,
}

impl<Input: 'static + Send + Sync + Clone, Output: 'static + Send + Sync + Clone>
    Facet<Input, Output>
{
    /// Defines a new facet with a custom combine function.
    pub fn define<F>(combine: F) -> Self
    where
        F: Fn(&[Input]) -> Output + 'static + Send + Sync,
    {
        let default_val = combine(&[]);
        Self {
            id: FacetId(NEXT_FACET_ID.fetch_add(1, Ordering::Relaxed)),
            combine_fn: Arc::new(combine),
            default_value: Arc::new(default_val),
            is_static: false,
        }
    }

    /// Defines a static facet (its values cannot be computed dynamically from EditorState).
    pub fn define_static<F>(combine: F) -> Self
    where
        F: Fn(&[Input]) -> Output + 'static + Send + Sync,
    {
        let mut facet = Self::define(combine);
        facet.is_static = true;
        facet
    }

    /// Returns the unique FacetId.
    pub fn id(&self) -> FacetId {
        self.id
    }

    /// Returns the default value computed when no inputs are present.
    pub fn default_value(&self) -> &Output {
        &self.default_value
    }

    /// Returns an extension that contributes a static value to this facet.
    pub fn of(&self, value: Input) -> Extension {
        Extension::FacetStatic(Arc::new(self.clone()), Arc::new(value))
    }

    /// Returns an extension that computes a dynamic value for this facet from the `EditorState`.
    pub fn compute<F>(&self, compute: F) -> Extension
    where
        F: Fn(&EditorState) -> Input + 'static + Send + Sync,
    {
        assert!(!self.is_static, "Cannot attach dynamic provider to static facet");
        let closure = Arc::new(move |state: &EditorState| -> Arc<dyn Any + Send + Sync> {
            Arc::new(compute(state))
        });
        Extension::FacetDynamic(Arc::new(self.clone()), closure)
    }

    /// Returns an extension that computes multiple dynamic values for this facet from the `EditorState`.
    pub fn compute_multi<F>(&self, compute: F) -> Extension
    where
        F: Fn(&EditorState) -> Vec<Input> + 'static + Send + Sync,
    {
        assert!(!self.is_static, "Cannot attach dynamic provider to static facet");
        let closure = Arc::new(move |state: &EditorState| -> Vec<Arc<dyn Any + Send + Sync>> {
            compute(state)
                .into_iter()
                .map(|v| Arc::new(v) as Arc<dyn Any + Send + Sync>)
                .collect()
        });
        Extension::FacetDynamicMulti(Arc::new(self.clone()), closure)
    }
}

impl<Input: 'static + Send + Sync + Clone> Facet<Input, Vec<Input>> {
    /// Convenience constructor for facets that aggregate all inputs into a Vec.
    pub fn define_list() -> Self {
        Self::define(|inputs| inputs.to_vec())
    }
}

impl<Input: 'static + Send + Sync + Clone, Output: 'static + Send + Sync + Clone> AnyFacet
    for Facet<Input, Output>
{
    fn id(&self) -> FacetId {
        self.id
    }

    fn is_static(&self) -> bool {
        self.is_static
    }

    fn combine_erased(&self, inputs: &[Arc<dyn Any + Send + Sync>]) -> Arc<dyn Any + Send + Sync> {
        let typed_inputs: Vec<Input> = inputs
            .iter()
            .map(|b| {
                b.downcast_ref::<Input>()
                    .expect("Facet input type mismatch")
                    .clone()
            })
            .collect();
        let output = (self.combine_fn)(&typed_inputs);
        Arc::new(output)
    }

    fn default_erased(&self) -> Arc<dyn Any + Send + Sync> {
        self.default_value.clone()
    }
}

/// A persistent piece of state associated with an `EditorState`, updated on every transaction.
#[derive(Clone)]
pub struct StateField<Value: 'static + Send + Sync + Clone> {
    pub(crate) id: StateFieldId,
    pub(crate) init_fn: Arc<dyn Fn(&EditorState) -> Value + Send + Sync>,
    pub(crate) update_fn: Arc<dyn Fn(&Value, &Transaction) -> Value + Send + Sync>,
}

impl<Value: 'static + Send + Sync + Clone> StateField<Value> {
    /// Defines a new state field with an initializer and an update function.
    pub fn define<Init, Update>(init: Init, update: Update) -> Self
    where
        Init: Fn(&EditorState) -> Value + 'static + Send + Sync,
        Update: Fn(&Value, &Transaction) -> Value + 'static + Send + Sync,
    {
        Self {
            id: StateFieldId(NEXT_FIELD_ID.fetch_add(1, Ordering::Relaxed)),
            init_fn: Arc::new(init),
            update_fn: Arc::new(update),
        }
    }

    /// Returns the unique StateFieldId.
    pub fn id(&self) -> StateFieldId {
        self.id
    }

    /// Returns an extension that enables this field in an editor state.
    pub fn extension(&self) -> Extension {
        Extension::StateField(Arc::new(self.clone()))
    }
}

impl<Value: 'static + Send + Sync + Clone> AnyStateField for StateField<Value> {
    fn id(&self) -> StateFieldId {
        self.id
    }

    fn init(&self, state: &EditorState) -> Arc<dyn Any + Send + Sync> {
        Arc::new((self.init_fn)(state))
    }

    fn update(&self, value: &dyn Any, tr: &Transaction) -> Arc<dyn Any + Send + Sync> {
        let current = value
            .downcast_ref::<Value>()
            .expect("StateField type mismatch during update");
        Arc::new((self.update_fn)(current, tr))
    }
}

/// Compartments allow isolating a sub-tree of extensions so it can be dynamically replaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Compartment {
    pub id: CompartmentId,
}

impl Default for Compartment {
    fn default() -> Self {
        Self::new()
    }
}

impl Compartment {
    /// Creates a new unique compartment.
    pub fn new() -> Self {
        Self {
            id: CompartmentId(NEXT_COMPARTMENT_ID.fetch_add(1, Ordering::Relaxed)),
        }
    }

    /// Wraps an extension inside this compartment.
    pub fn of(&self, extension: Extension) -> Extension {
        Extension::Compartment(self.id, Box::new(extension))
    }
}

/// Declarative extension value for configuring editor behaviors.
#[derive(Clone)]
pub enum Extension {
    /// Contributes a static value to a facet.
    FacetStatic(Arc<dyn AnyFacet>, Arc<dyn Any + Send + Sync>),
    /// Contributes a dynamic computed value to a facet.
    FacetDynamic(
        Arc<dyn AnyFacet>,
        Arc<dyn Fn(&EditorState) -> Arc<dyn Any + Send + Sync> + Send + Sync>,
    ),
    /// Contributes multiple dynamic computed values to a facet.
    FacetDynamicMulti(
        Arc<dyn AnyFacet>,
        Arc<dyn Fn(&EditorState) -> Vec<Arc<dyn Any + Send + Sync>> + Send + Sync>,
    ),
    /// Registers a state field.
    StateField(Arc<dyn AnyStateField + Send + Sync>),
    /// Assigns precedence to a sub-tree of extensions.
    Precedence(Prec, Box<Extension>),
    /// Assigns an extension to a dynamic compartment.
    Compartment(CompartmentId, Box<Extension>),
    /// A group of extensions.
    Group(Vec<Extension>),
}

impl Extension {
    /// Wraps this extension with an explicit precedence.
    pub fn with_prec(self, prec: Prec) -> Self {
        Extension::Precedence(prec, Box::new(self))
    }
}

impl From<Vec<Extension>> for Extension {
    fn from(extensions: Vec<Extension>) -> Self {
        Extension::Group(extensions)
    }
}
