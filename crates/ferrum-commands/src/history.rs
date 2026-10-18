use ferrum_state::change::{ChangeDesc, ChangeSet};
use ferrum_state::facet::{Extension, Facet, StateField};
use ferrum_state::selection::EditorSelection;
use ferrum_state::state::EditorState;
use ferrum_state::transaction::{
    AddToHistory, IsolateHistory, Transaction, TransactionSpec, TransactionTime, UserEvent,
};
use std::sync::{Arc, LazyLock};

/// Identifies the undo or redo branch in the history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchName {
    Done,
    Undone,
}

/// Annotation indicating a transaction was produced by an undo or redo action.
#[derive(Clone)]
struct FromHistory {
    side: BranchName,
    rest: Vec<HistEvent>,
    selection: Option<EditorSelection>,
}

/// A stored event in the history representing an invertible group of changes.
#[derive(Clone, Debug)]
pub struct HistEvent {
    pub changes: Option<ChangeSet>,
    pub mapped: Option<ChangeDesc>,
    pub start_selection: Option<EditorSelection>,
    pub selections_after: Vec<EditorSelection>,
}

impl HistEvent {
    pub fn new(
        changes: Option<ChangeSet>,
        mapped: Option<ChangeDesc>,
        start_selection: Option<EditorSelection>,
        selections_after: Vec<EditorSelection>,
    ) -> Self {
        Self {
            changes,
            mapped,
            start_selection,
            selections_after,
        }
    }

    pub fn from_transaction(tr: &Transaction, selection: Option<EditorSelection>) -> Option<Self> {
        if tr.changes().is_empty() {
            return None;
        }
        let inverted = tr.changes().invert(tr.start_state().doc());
        let start_sel = selection.or_else(|| Some(tr.start_state().selection().clone()));
        Some(Self {
            changes: Some(inverted),
            mapped: None,
            start_selection: start_sel,
            selections_after: Vec::new(),
        })
    }

    pub fn selection(selections: Vec<EditorSelection>) -> Self {
        Self {
            changes: None,
            mapped: None,
            start_selection: None,
            selections_after: selections,
        }
    }

    pub fn with_selections_after(&self, selections: Vec<EditorSelection>) -> Self {
        Self {
            changes: self.changes.clone(),
            mapped: self.mapped.clone(),
            start_selection: self.start_selection.clone(),
            selections_after: selections,
        }
    }
}

/// Configuration options for the history extension.
#[derive(Clone)]
pub struct HistoryConfig {
    /// The minimum depth of events to store (default: 100).
    pub min_depth: usize,
    /// The maximum delay in milliseconds between adjacent events to merge them (default: 500).
    pub new_group_delay: u64,
    /// Custom predicate to determine whether an adjacent transaction should join the previous event.
    pub join_to_event: Option<Arc<dyn Fn(&Transaction, bool) -> bool + Send + Sync>>,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            min_depth: 100,
            new_group_delay: 500,
            join_to_event: None,
        }
    }
}

/// Facet that combines history configuration options.
pub static HISTORY_CONFIG: LazyLock<Facet<HistoryConfig, HistoryConfig>> = LazyLock::new(|| {
    Facet::define(|inputs: &[HistoryConfig]| {
        let mut min_depth = None;
        let mut new_group_delay = None;
        let mut join_fn: Option<Arc<dyn Fn(&Transaction, bool) -> bool + Send + Sync>> = None;
        for c in inputs {
            min_depth = Some(match min_depth {
                Some(prev) => (prev as usize).max(c.min_depth),
                None => c.min_depth,
            });
            new_group_delay = Some(match new_group_delay {
                Some(prev) => (prev as u64).min(c.new_group_delay),
                None => c.new_group_delay,
            });
            if let Some(j) = &c.join_to_event {
                let j_clone = j.clone();
                join_fn = match join_fn {
                    Some(prev) => Some(Arc::new(move |tr, adj| prev(tr, adj) || j_clone(tr, adj))),
                    None => Some(j.clone()),
                };
            }
        }
        HistoryConfig {
            min_depth: min_depth.unwrap_or(100),
            new_group_delay: new_group_delay.unwrap_or(500),
            join_to_event: join_fn,
        }
    })
});

/// The internal state of the history extension tracking undo and redo branches.
#[derive(Clone)]
pub struct HistoryState {
    pub done: Vec<HistEvent>,
    pub undone: Vec<HistEvent>,
    pub prev_time: u64,
    pub prev_user_event: Option<String>,
}

impl HistoryState {
    pub fn empty() -> Self {
        Self {
            done: Vec::new(),
            undone: Vec::new(),
            prev_time: 0,
            prev_user_event: None,
        }
    }

    pub fn isolate(&self) -> Self {
        if self.prev_time != 0 {
            Self {
                done: self.done.clone(),
                undone: self.undone.clone(),
                prev_time: 0,
                prev_user_event: None,
            }
        } else {
            self.clone()
        }
    }

    pub fn add_changes(
        &self,
        event: HistEvent,
        time: u64,
        user_event: Option<String>,
        config: &HistoryConfig,
        tr: &Transaction,
    ) -> Self {
        let mut done = self.done.clone();
        let last_idx = if done.is_empty() { None } else { Some(done.len() - 1) };

        if let Some(idx) = last_idx {
            let last = &done[idx];
            let can_merge = match (&last.changes, &event.changes) {
                (Some(last_changes), Some(event_changes)) => {
                    let is_joinable_user_event = match &user_event {
                        None => true,
                        Some(ue) => {
                            ue.starts_with("input.type") || ue.starts_with("delete")
                        }
                    };
                    let is_compose = user_event.as_deref() == Some("input.type.compose");
                    let time_close = self.prev_time > 0
                        && time >= self.prev_time
                        && (time - self.prev_time) < config.new_group_delay;
                    let adjacent = is_adjacent(last_changes.desc(), event_changes.desc());
                    let joinable = match &config.join_to_event {
                        Some(f) => f(tr, adjacent),
                        None => adjacent,
                    };
                    is_compose || (is_joinable_user_event && last.selections_after.is_empty() && time_close && joinable)
                }
                _ => false,
            };

            if can_merge {
                let last = &done[idx];
                let last_changes = last.changes.as_ref().unwrap();
                let event_changes = event.changes.as_ref().unwrap();
                // Inverted changes compose in reverse order: event . last
                let composed = event_changes.compose(last_changes);
                done[idx] = HistEvent {
                    changes: Some(composed),
                    mapped: last.mapped.clone(),
                    start_selection: last.start_selection.clone(),
                    selections_after: Vec::new(),
                };
                return Self {
                    done,
                    undone: Vec::new(),
                    prev_time: time,
                    prev_user_event: user_event,
                };
            }
        }

        done = update_branch(&done, done.len(), config.min_depth, event);
        Self {
            done,
            undone: Vec::new(),
            prev_time: time,
            prev_user_event: user_event,
        }
    }

    pub fn add_selection(
        &self,
        selection: &EditorSelection,
        time: u64,
        user_event: Option<String>,
        new_group_delay: u64,
    ) -> Self {
        if let Some(last) = self.done.last() {
            if let Some(last_sel) = last.selections_after.last() {
                if time.saturating_sub(self.prev_time) < new_group_delay
                    && user_event == self.prev_user_event
                    && last_sel.ranges().len() == selection.ranges().len()
                {
                    return self.clone();
                }
            }
        }
        Self {
            done: add_selection_to_branch(&self.done, selection),
            undone: self.undone.clone(),
            prev_time: time,
            prev_user_event: user_event,
        }
    }

    pub fn add_mapping(&self, mapping: &ChangeDesc) -> Self {
        Self {
            done: add_mapping_to_branch(&self.done, mapping),
            undone: add_mapping_to_branch(&self.undone, mapping),
            prev_time: self.prev_time,
            prev_user_event: self.prev_user_event.clone(),
        }
    }

    pub fn pop(&self, side: BranchName, state: &EditorState, only_selection: bool) -> Option<TransactionSpec> {
        let branch = match side {
            BranchName::Done => &self.done,
            BranchName::Undone => &self.undone,
        };
        if branch.is_empty() {
            return None;
        }

        let event = branch.last().unwrap();
        let selection = event.selections_after.first().cloned().or_else(|| {
            event.start_selection.as_ref().map(|s| {
                if let Some(changes) = &event.changes {
                    s.map(&changes.desc().inverted_desc(), 1)
                } else {
                    s.clone()
                }
            })
        }).unwrap_or_else(|| state.selection().clone());

        if only_selection && !event.selections_after.is_empty() {
            let mut rest = branch.clone();
            let last = rest.last_mut().unwrap();
            last.selections_after.pop();
            Some(
                TransactionSpec::new()
                    .with_selection(selection.clone())
                    .with_annotation(FromHistory {
                        side,
                        rest,
                        selection: Some(selection),
                    }),
            )
        } else if let Some(changes) = &event.changes {
            let mut rest = if branch.len() == 1 {
                Vec::new()
            } else {
                branch[..branch.len() - 1].to_vec()
            };
            if let Some(mapped) = &event.mapped {
                rest = add_mapping_to_branch(&rest, mapped);
            }
            let mut spec = TransactionSpec::new()
                .with_changes(changes.clone())
                .with_annotation(FromHistory {
                    side,
                    rest,
                    selection: Some(selection),
                });
            if let Some(start_sel) = &event.start_selection {
                spec = spec.with_selection(start_sel.clone());
            }
            Some(spec)
        } else {
            None
        }
    }
}

fn update_branch(branch: &[HistEvent], to: usize, max_len: usize, new_event: HistEvent) -> Vec<HistEvent> {
    let start = if to + 1 > max_len + 20 { to - max_len - 1 } else { 0 };
    let mut new_branch = branch[start..to.min(branch.len())].to_vec();
    new_branch.push(new_event);
    new_branch
}

fn add_selection_to_branch(branch: &[HistEvent], selection: &EditorSelection) -> Vec<HistEvent> {
    if branch.is_empty() {
        vec![HistEvent::selection(vec![selection.clone()])]
    } else {
        let mut new_branch = branch.to_vec();
        let last = new_branch.last_mut().unwrap();
        if let Some(prev_sel) = last.selections_after.last() {
            if prev_sel.eq(selection) {
                return branch.to_vec();
            }
        }
        last.selections_after.push(selection.clone());
        new_branch
    }
}

fn add_mapping_to_branch(branch: &[HistEvent], mapping: &ChangeDesc) -> Vec<HistEvent> {
    if branch.is_empty() {
        return Vec::new();
    }
    let mut length = branch.len();
    let mut current_mapping = mapping.clone();
    let mut selections: Vec<EditorSelection> = Vec::new();

    while length > 0 {
        let event = map_event(&branch[length - 1], &current_mapping, &selections);
        if event.changes.as_ref().map(|c| !c.is_empty()).unwrap_or(false) {
            let mut result = branch[..length].to_vec();
            result[length - 1] = event;
            return result;
        } else {
            if let Some(m) = event.mapped {
                current_mapping = m;
            }
            length -= 1;
            selections = event.selections_after;
        }
    }
    if !selections.is_empty() {
        vec![HistEvent::selection(selections)]
    } else {
        Vec::new()
    }
}

fn map_event(event: &HistEvent, mapping: &ChangeDesc, extra_selections: &[EditorSelection]) -> HistEvent {
    let mut selections: Vec<EditorSelection> = event
        .selections_after
        .iter()
        .map(|s| s.map(mapping, -1))
        .collect();
    selections.extend_from_slice(extra_selections);

    if let Some(changes) = &event.changes {
        let mapped_changes = changes.map(mapping, false);
        let before = mapping.map_desc(changes.desc(), true);
        let start_selection = event.start_selection.as_ref().map(|s| s.map(&before, -1));
        let full_mapping = match &event.mapped {
            Some(m) => m.compose_desc(&before),
            None => before,
        };
        HistEvent {
            changes: Some(mapped_changes),
            mapped: Some(full_mapping),
            start_selection,
            selections_after: selections,
        }
    } else {
        HistEvent {
            changes: None,
            mapped: None,
            start_selection: None,
            selections_after: selections,
        }
    }
}

fn is_adjacent(a: &ChangeDesc, b: &ChangeDesc) -> bool {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    a.iter_changed_ranges(|f, t, _, _| ranges.push((f, t)), false);
    let mut adjacent = false;
    b.iter_changed_ranges(
        |_, _, f, t| {
            for &(from, to) in &ranges {
                if t >= from && f <= to {
                    adjacent = true;
                }
            }
        },
        false,
    );
    adjacent
}

/// StateField that stores and manages the history state for an EditorState.
pub static HISTORY_FIELD: LazyLock<StateField<HistoryState>> = LazyLock::new(|| {
    StateField::define(
        |_state| HistoryState::empty(),
        |state, tr| {
            let config = tr.start_state().facet(&HISTORY_CONFIG);

            if let Some(from_hist) = tr.annotation::<FromHistory>() {
                let item = HistEvent::from_transaction(tr, from_hist.selection.clone());
                let from_side = from_hist.side;
                let other = match from_side {
                    BranchName::Done => &state.undone,
                    BranchName::Undone => &state.done,
                };
                let updated_other = if let Some(item) = item {
                    update_branch(other, other.len(), config.min_depth, item)
                } else {
                    add_selection_to_branch(other, tr.start_state().selection())
                };
                return match from_side {
                    BranchName::Done => HistoryState {
                        done: from_hist.rest.clone(),
                        undone: updated_other,
                        prev_time: 0,
                        prev_user_event: None,
                    },
                    BranchName::Undone => HistoryState {
                        done: updated_other,
                        undone: from_hist.rest.clone(),
                        prev_time: 0,
                        prev_user_event: None,
                    },
                };
            }

            let mut current = state.clone();

            if let Some(isolate) = tr.annotation::<IsolateHistory>() {
                if *isolate == IsolateHistory::Full || *isolate == IsolateHistory::Before {
                    current = current.isolate();
                }
            }

            if let Some(add) = tr.annotation::<AddToHistory>() {
                if !add.0 {
                    if !tr.changes().is_empty() {
                        return current.add_mapping(tr.changes().desc());
                    } else {
                        return current;
                    }
                }
            }

            let event = HistEvent::from_transaction(tr, None);
            let time = tr.annotation::<TransactionTime>().map(|t| t.0).unwrap_or(0);
            let user_event = tr.annotation::<UserEvent>().map(|u| u.0.clone());

            if let Some(ev) = event {
                current = current.add_changes(ev, time, user_event, config, tr);
            } else if tr.selection().is_some() {
                current = current.add_selection(
                    tr.start_state().selection(),
                    time,
                    user_event,
                    config.new_group_delay,
                );
            }

            if let Some(isolate) = tr.annotation::<IsolateHistory>() {
                if *isolate == IsolateHistory::Full || *isolate == IsolateHistory::After {
                    current = current.isolate();
                }
            }

            current
        },
    )
});

/// Creates an extension that enables undo/redo history with the given configuration.
pub fn history(config: HistoryConfig) -> Extension {
    Extension::Group(vec![
        HISTORY_FIELD.extension(),
        HISTORY_CONFIG.of(config),
    ])
}

/// Undo the most recent edit event.
pub fn undo(state: &EditorState) -> Option<TransactionSpec> {
    let hist = state.field(&HISTORY_FIELD)?;
    hist.pop(BranchName::Done, state, false)
}

/// Redo the most recently undone edit event.
pub fn redo(state: &EditorState) -> Option<TransactionSpec> {
    let hist = state.field(&HISTORY_FIELD)?;
    hist.pop(BranchName::Undone, state, false)
}

/// Undo the most recent selection change.
pub fn undo_selection(state: &EditorState) -> Option<TransactionSpec> {
    let hist = state.field(&HISTORY_FIELD)?;
    hist.pop(BranchName::Done, state, true)
}

/// Redo the most recently undone selection change.
pub fn redo_selection(state: &EditorState) -> Option<TransactionSpec> {
    let hist = state.field(&HISTORY_FIELD)?;
    hist.pop(BranchName::Undone, state, true)
}

/// Returns the number of undoable events currently stored.
pub fn undo_depth(state: &EditorState) -> usize {
    state
        .field(&HISTORY_FIELD)
        .map(|h| h.done.iter().filter(|e| e.changes.is_some()).count())
        .unwrap_or(0)
}

/// Returns the number of redoable events currently stored.
pub fn redo_depth(state: &EditorState) -> usize {
    state
        .field(&HISTORY_FIELD)
        .map(|h| h.undone.iter().filter(|e| e.changes.is_some()).count())
        .unwrap_or(0)
}
