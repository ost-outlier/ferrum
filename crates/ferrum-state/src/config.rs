use crate::facet::{
    AnyFacet, AnyStateField, CompartmentId, Extension, Facet, FacetId, Prec, StateField,
    StateFieldId,
};
use crate::state::EditorState;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[allow(dead_code)]
pub(crate) struct DynamicFacetSlot {
    pub(crate) facet_id: FacetId,
    pub(crate) compute: Arc<dyn Fn(&EditorState) -> Arc<dyn Any + Send + Sync> + Send + Sync>,
}

enum FlattenedProvider {
    Static(Arc<dyn Any + Send + Sync>),
    Dynamic(Arc<dyn Fn(&EditorState) -> Arc<dyn Any + Send + Sync> + Send + Sync>),
    DynamicMulti(Arc<dyn Fn(&EditorState) -> Vec<Arc<dyn Any + Send + Sync>> + Send + Sync>),
}

/// Resolved configuration of an `EditorState`.
/// Computes all static facets upfront, and sets up slots for dynamic facets and state fields.
pub struct Configuration {
    pub(crate) base_extensions: Vec<Extension>,
    pub(crate) compartments: HashMap<CompartmentId, Extension>,
    pub(crate) static_facets: HashMap<FacetId, Arc<dyn Any + Send + Sync>>,
    pub(crate) dynamic_facet_slots: Vec<DynamicFacetSlot>,
    pub(crate) dynamic_facet_map: HashMap<FacetId, usize>,
    pub(crate) state_fields: Vec<Arc<dyn AnyStateField + Send + Sync>>,
    pub(crate) field_map: HashMap<StateFieldId, usize>,
}

impl Configuration {
    /// Resolves an array of extensions and an optional map of active compartments into a `Configuration`.
    pub fn resolve(
        extensions: &[Extension],
        compartment_overrides: Option<&HashMap<CompartmentId, Extension>>,
    ) -> Self {
        let mut active_compartments = HashMap::new();
        if let Some(overrides) = compartment_overrides {
            active_compartments.clone_from(overrides);
        }

        // 5 buckets corresponding to Prec levels 0 to 4
        let mut buckets: [Vec<FlattenedItem>; 5] = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        for ext in extensions {
            flatten_extension(ext, &mut active_compartments, &mut buckets, Prec::Default);
        }

        // Linearize all items in order of precedence (0 = Highest, 4 = Lowest)
        let mut facet_providers: HashMap<FacetId, (Arc<dyn AnyFacet>, Vec<FlattenedProvider>)> =
            HashMap::new();
        let mut facet_order: Vec<FacetId> = Vec::new();
        let mut fields: Vec<Arc<dyn AnyStateField + Send + Sync>> = Vec::new();
        let mut field_ids = HashMap::new();

        for bucket in buckets {
            for item in bucket {
                match item {
                    FlattenedItem::FacetStatic(facet, val) => {
                        let id = facet.id();
                        let entry = facet_providers.entry(id).or_insert_with(|| {
                            facet_order.push(id);
                            (facet, Vec::new())
                        });
                        entry.1.push(FlattenedProvider::Static(val));
                    }
                    FlattenedItem::FacetDynamic(facet, closure) => {
                        let id = facet.id();
                        let entry = facet_providers.entry(id).or_insert_with(|| {
                            facet_order.push(id);
                            (facet, Vec::new())
                        });
                        entry.1.push(FlattenedProvider::Dynamic(closure));
                    }
                    FlattenedItem::FacetDynamicMulti(facet, closure) => {
                        let id = facet.id();
                        let entry = facet_providers.entry(id).or_insert_with(|| {
                            facet_order.push(id);
                            (facet, Vec::new())
                        });
                        entry.1.push(FlattenedProvider::DynamicMulti(closure));
                    }
                    FlattenedItem::StateField(field) => {
                        let id = field.id();
                        if !field_ids.contains_key(&id) {
                            field_ids.insert(id, fields.len());
                            fields.push(field);
                        }
                    }
                }
            }
        }

        let mut static_facets = HashMap::new();
        let mut dynamic_facet_slots = Vec::new();
        let mut dynamic_facet_map = HashMap::new();

        for id in facet_order {
            if let Some((facet, providers)) = facet_providers.remove(&id) {
                let has_dynamic = providers.iter().any(|p| !matches!(p, FlattenedProvider::Static(_)));

                if !has_dynamic {
                    // All providers are static: combine once and store in static_facets
                    let static_inputs: Vec<Arc<dyn Any + Send + Sync>> = providers
                        .into_iter()
                        .map(|p| match p {
                            FlattenedProvider::Static(val) => val,
                            _ => unreachable!(),
                        })
                        .collect();

                    let combined = facet.combine_erased(&static_inputs);
                    static_facets.insert(id, combined);
                } else {
                    // Dynamic facet: compile slot closure
                    let facet_clone = facet.clone();
                    let providers_arc: Arc<[FlattenedProvider]> = providers.into();

                    let compute_closure = Arc::new(move |state: &EditorState| -> Arc<dyn Any + Send + Sync> {
                        let mut inputs = Vec::new();
                        for provider in providers_arc.iter() {
                            match provider {
                                FlattenedProvider::Static(val) => {
                                    inputs.push(val.clone());
                                }
                                FlattenedProvider::Dynamic(f) => {
                                    inputs.push((f)(state));
                                }
                                FlattenedProvider::DynamicMulti(f) => {
                                    inputs.extend((f)(state));
                                }
                            }
                        }
                        facet_clone.combine_erased(&inputs)
                    });

                    let slot_idx = dynamic_facet_slots.len();
                    dynamic_facet_slots.push(DynamicFacetSlot {
                        facet_id: id,
                        compute: compute_closure,
                    });
                    dynamic_facet_map.insert(id, slot_idx);
                }
            }
        }

        Self {
            base_extensions: extensions.to_vec(),
            compartments: active_compartments,
            static_facets,
            dynamic_facet_slots,
            dynamic_facet_map,
            state_fields: fields,
            field_map: field_ids,
        }
    }

    /// Reads the value of a facet from this configuration and the dynamic values of an editor state.
    pub fn facet<'a, Input: 'static + Send + Sync + Clone, Output: 'static + Send + Sync + Clone>(
        &'a self,
        facet: &'a Facet<Input, Output>,
        dynamic_values: Option<&'a [Arc<dyn Any + Send + Sync>]>,
    ) -> &'a Output {
        if let Some(&slot_idx) = self.dynamic_facet_map.get(&facet.id) {
            if let Some(dyn_vals) = dynamic_values {
                return dyn_vals[slot_idx]
                    .downcast_ref::<Output>()
                    .expect("Dynamic facet type mismatch");
            }
        }

        if let Some(val) = self.static_facets.get(&facet.id) {
            return val
                .downcast_ref::<Output>()
                .expect("Static facet type mismatch");
        }

        // Return the static default value directly (zero allocation!)
        facet.default_value()
    }

    /// Reads the value of a state field from dynamic field values.
    pub fn field<'a, Value: 'static + Send + Sync + Clone>(
        &self,
        field: &StateField<Value>,
        field_values: &'a [Arc<dyn Any + Send + Sync>],
    ) -> Option<&'a Value> {
        let &idx = self.field_map.get(&field.id)?;
        field_values.get(idx).map(|v| {
            v.downcast_ref::<Value>()
                .expect("StateField type mismatch")
        })
    }
}

enum FlattenedItem {
    FacetStatic(Arc<dyn AnyFacet>, Arc<dyn Any + Send + Sync>),
    FacetDynamic(
        Arc<dyn AnyFacet>,
        Arc<dyn Fn(&EditorState) -> Arc<dyn Any + Send + Sync> + Send + Sync>,
    ),
    FacetDynamicMulti(
        Arc<dyn AnyFacet>,
        Arc<dyn Fn(&EditorState) -> Vec<Arc<dyn Any + Send + Sync>> + Send + Sync>,
    ),
    StateField(Arc<dyn AnyStateField + Send + Sync>),
}

fn flatten_extension(
    ext: &Extension,
    compartments: &mut HashMap<CompartmentId, Extension>,
    buckets: &mut [Vec<FlattenedItem>; 5],
    current_prec: Prec,
) {
    match ext {
        Extension::FacetStatic(facet, val) => {
            buckets[current_prec as usize].push(FlattenedItem::FacetStatic(facet.clone(), val.clone()));
        }
        Extension::FacetDynamic(facet, f) => {
            buckets[current_prec as usize].push(FlattenedItem::FacetDynamic(facet.clone(), f.clone()));
        }
        Extension::FacetDynamicMulti(facet, f) => {
            buckets[current_prec as usize].push(FlattenedItem::FacetDynamicMulti(facet.clone(), f.clone()));
        }
        Extension::StateField(field) => {
            buckets[current_prec as usize].push(FlattenedItem::StateField(field.clone()));
        }
        Extension::Precedence(prec, inner) => {
            flatten_extension(inner, compartments, buckets, *prec);
        }
        Extension::Compartment(cid, inner) => {
            let active = if let Some(overridden) = compartments.get(cid) {
                overridden.clone()
            } else {
                compartments.insert(*cid, (**inner).clone());
                (**inner).clone()
            };
            flatten_extension(&active, compartments, buckets, current_prec);
        }
        Extension::Group(group) => {
            for item in group {
                flatten_extension(item, compartments, buckets, current_prec);
            }
        }
    }
}
