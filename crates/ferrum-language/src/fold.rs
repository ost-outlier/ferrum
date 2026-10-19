//! Code folding state field, transaction effects, and folding services.
//! Faithful port of CodeMirror 6's `@codemirror/language/src/fold.ts`.

use ferrum_state::facet::StateField;
use ferrum_state::transaction::StateEffect;
use ferrum_view::decoration::{Decoration, DecorationSet, ReplaceSpec};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A range of text that is folded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldRange {
    pub from: usize,
    pub to: usize,
}

impl FoldRange {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

/// Transaction effect constructor to fold a range.
pub struct FoldEffect;

impl FoldEffect {
    pub fn of(range: FoldRange) -> StateEffect {
        StateEffect::of(range)
    }
}

/// Transaction effect constructor to unfold a range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnfoldRange {
    pub from: usize,
    pub to: usize,
}

pub struct UnfoldEffect;

impl UnfoldEffect {
    pub fn of(range: UnfoldRange) -> StateEffect {
        StateEffect::of(range)
    }
}

/// Creates the `StateField` storing active folded ranges as a `DecorationSet`.
pub fn fold_state() -> Arc<StateField<DecorationSet>> {
    Arc::new(StateField::define(
        |_state| DecorationSet::empty(),
        |folded, tr| {
            let mut set = folded.map(tr.changes().desc());

            for effect in tr.effects() {
                if let Some(fold) = effect.downcast_ref::<FoldRange>() {
                    let deco = Decoration::replace_spec(
                        fold.from..fold.to,
                        ReplaceSpec {
                            widget: None,
                            block: false,
                        },
                    );
                    if let Ok(new_set) = set.update(ferrum_state::rangeset::RangeSetUpdate {
                        add: vec![deco],
                        filter: None,
                        filter_from: 0,
                        filter_to: usize::MAX,
                        sort: true,
                    }) {
                        set = new_set;
                    }
                } else if let Some(unfold) = effect.downcast_ref::<UnfoldRange>() {
                    let u_from = unfold.from;
                    let u_to = unfold.to;
                    if let Ok(new_set) = set.update(ferrum_state::rangeset::RangeSetUpdate {
                        add: vec![],
                        filter: Some(Arc::new(move |from, to, _| from != u_from || to != u_to)),
                        filter_from: u_from,
                        filter_to: u_to,
                        sort: true,
                    }) {
                        set = new_set;
                    }
                }
            }

            set
        },
    ))
}
