use ferrum_state::change::ChangeSpec;
use ferrum_state::selection::{EditorSelection, SelectionRange};
use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;

/// Command that inserts `text` at each selection range, replacing selected text.
pub fn insert_text(state: &EditorState, text: &str) -> Option<TransactionSpec> {
    let mut spec = TransactionSpec::new();
    let mut changes = Vec::new();
    let mut new_ranges = Vec::new();

    let mut shift: isize = 0;
    for range in state.selection().ranges() {
        let from = range.from();
        let to = range.to();
        changes.push(ChangeSpec::replace(from, to, text));

        let text_len = text.chars().count();
        let deleted_len = to - from;
        let new_pos = ((from as isize) + shift + (text_len as isize)).max(0) as usize;
        new_ranges.push(SelectionRange::cursor(new_pos));

        shift += (text_len as isize) - (deleted_len as isize);
    }

    spec = spec.with_changes(ferrum_state::change::ChangeSet::from_specs(changes));
    spec = spec.with_selection(EditorSelection::create(
        new_ranges,
        state.selection().main_index(),
    ));

    Some(spec)
}

/// Command that deletes the character immediately preceding the cursor (Backspace).
pub fn delete_backward(state: &EditorState) -> Option<TransactionSpec> {
    let mut changes = Vec::new();
    let mut new_ranges = Vec::new();
    let mut has_edits = false;

    let mut shift: isize = 0;
    for range in state.selection().ranges() {
        if !range.is_empty() {
            // Delete the selection range
            let from = range.from();
            let to = range.to();
            changes.push(ChangeSpec::delete(from, to));
            let new_pos = ((from as isize) + shift).max(0) as usize;
            new_ranges.push(SelectionRange::cursor(new_pos));
            shift -= (to - from) as isize;
            has_edits = true;
        } else if range.from() > 0 {
            // Delete 1 character backward
            let from = range.from() - 1;
            let to = range.from();
            changes.push(ChangeSpec::delete(from, to));
            let new_pos = ((from as isize) + shift).max(0) as usize;
            new_ranges.push(SelectionRange::cursor(new_pos));
            shift -= 1;
            has_edits = true;
        } else {
            new_ranges.push(*range);
        }
    }

    if !has_edits {
        return None;
    }

    Some(
        TransactionSpec::new()
            .with_changes(ferrum_state::change::ChangeSet::from_specs(changes))
            .with_selection(EditorSelection::create(
                new_ranges,
                state.selection().main_index(),
            )),
    )
}

/// Command that deletes the character immediately following the cursor (Delete).
pub fn delete_forward(state: &EditorState) -> Option<TransactionSpec> {
    let mut changes = Vec::new();
    let mut new_ranges = Vec::new();
    let mut has_edits = false;

    let total_len = state.doc().len_chars();
    let mut shift: isize = 0;
    for range in state.selection().ranges() {
        if !range.is_empty() {
            let from = range.from();
            let to = range.to();
            changes.push(ChangeSpec::delete(from, to));
            let new_pos = ((from as isize) + shift).max(0) as usize;
            new_ranges.push(SelectionRange::cursor(new_pos));
            shift -= (to - from) as isize;
            has_edits = true;
        } else if range.to() < total_len {
            let from = range.to();
            let to = range.to() + 1;
            changes.push(ChangeSpec::delete(from, to));
            let new_pos = ((from as isize) + shift).max(0) as usize;
            new_ranges.push(SelectionRange::cursor(new_pos));
            shift -= 1;
            has_edits = true;
        } else {
            new_ranges.push(*range);
        }
    }

    if !has_edits {
        return None;
    }

    Some(
        TransactionSpec::new()
            .with_changes(ferrum_state::change::ChangeSet::from_specs(changes))
            .with_selection(EditorSelection::create(
                new_ranges,
                state.selection().main_index(),
            )),
    )
}

/// Moves the cursor left by one character. If `select` is true, extends selection.
pub fn move_cursor_left(state: &EditorState, select: bool) -> Option<TransactionSpec> {
    let new_ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            if select {
                let new_head = r.head.saturating_sub(1);
                SelectionRange::new(r.anchor, new_head)
            } else if !r.is_empty() {
                SelectionRange::cursor(r.from())
            } else {
                SelectionRange::cursor(r.head.saturating_sub(1))
            }
        })
        .collect();

    Some(TransactionSpec::new().with_selection(EditorSelection::create(
        new_ranges,
        state.selection().main_index(),
    )))
}

/// Moves the cursor right by one character. If `select` is true, extends selection.
pub fn move_cursor_right(state: &EditorState, select: bool) -> Option<TransactionSpec> {
    let max_len = state.doc().len_chars();
    let new_ranges: Vec<SelectionRange> = state
        .selection()
        .ranges()
        .iter()
        .map(|r| {
            if select {
                let new_head = (r.head + 1).min(max_len);
                SelectionRange::new(r.anchor, new_head)
            } else if !r.is_empty() {
                SelectionRange::cursor(r.to())
            } else {
                SelectionRange::cursor((r.head + 1).min(max_len))
            }
        })
        .collect();

    Some(TransactionSpec::new().with_selection(EditorSelection::create(
        new_ranges,
        state.selection().main_index(),
    )))
}

/// Selects the entire document.
pub fn select_all(state: &EditorState) -> Option<TransactionSpec> {
    let len = state.doc().len_chars();
    Some(TransactionSpec::new().with_selection(EditorSelection::single(0, len)))
}
