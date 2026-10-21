// One function per symbol listed in crates/ferrum-state/src/README.md, using the signature stated there.
#![allow(dead_code, unused_variables)]
use ferrum_state::*;
use std::sync::Arc;

fn l10(state: &EditorState) -> &Text { state.doc() }
fn l11(state: &EditorState) -> &EditorSelection { state.selection() }
fn l12<'a>(state: &'a EditorState, facet: &'a Facet<u8, u8>) -> &'a u8 { state.facet(facet) }
fn l13<'a>(state: &'a EditorState, field: &StateField<u8>) -> &'a u8 { state.field(field) }
fn l14(state: &Arc<EditorState>, spec: TransactionSpec) -> Option<Transaction> { state.update(spec) }
fn l15(state: &Arc<EditorState>, tr: Transaction) -> Arc<EditorState> { state.apply(tr) }
fn l16() -> Arc<EditorState> { EditorState::create(EditorStateConfig::default()) }
fn l23(pos: usize) -> SelectionRange { SelectionRange::cursor(pos) }
fn l24(anchor: usize, head: usize) -> SelectionRange { SelectionRange::new(anchor, head) }
fn l25(r: &SelectionRange) -> usize { r.from() }
fn l26(r: &SelectionRange) -> usize { r.to() }
fn l27(r: &SelectionRange) -> bool { r.is_empty() }
fn l28(r: &SelectionRange) -> bool { r.is_inverted() }
fn l29(r: &SelectionRange) -> Option<usize> { r.goal_column }
fn l30(s: &EditorSelection) -> usize { s.main_index() }
fn l41(s: &str) -> Text { Text::from_str(s) }
fn l42(t: &Text) -> usize { t.len_chars() }
fn l43(t: &Text) -> usize { t.len_lines() }
fn l44(t: &Text, idx: usize) -> Line { t.line(idx) }
fn l45(t: &Text, char_pos: usize) -> Line { t.line_at(char_pos) }
fn l46(t: &Text, char_pos: usize) -> usize { t.char_to_line(char_pos) }
fn l47(t: &Text) -> String { t.slice_to_string(0..1) }
fn l48(l: &Line) -> (usize, usize, usize, usize) { (l.from, l.to, l.length, l.number) }
fn l54() -> (ChangeSpec, ChangeSpec, ChangeSpec) { (ChangeSpec::insert(0, "a"), ChangeSpec::delete(0, 1), ChangeSpec::replace(0, 1, "b")) }
fn l56(desc: &ChangeDesc, pos: usize, assoc: i8) -> usize { desc.map_pos(pos, assoc) }
fn l57(desc: &ChangeDesc, other: &ChangeDesc) -> ChangeDesc { desc.compose(other) }
fn l58(desc: &ChangeDesc, other: &ChangeDesc, before: bool) -> ChangeDesc { desc.map(other, before) }
fn l59(cs: &ChangeSet) -> &ChangeDesc { cs.desc() }
fn l60(cs: &ChangeSet, doc: &Text) -> ChangeSet { cs.invert(doc) }
fn l61(cs: &ChangeSet, doc: &Text) -> Text { cs.apply(doc) }
fn l62() -> TransactionSpec { TransactionSpec::new() }
fn l70(f: &Facet<u8, u8>) -> Extension { f.of(1u8) }
fn l71(c: &Compartment, e: Extension) -> StateEffect { c.reconfigure(e) }
fn l72(f: StateField<u8>) {}
fn l73() -> [Prec; 5] { [Prec::Highest, Prec::High, Prec::Default, Prec::Low, Prec::Lowest] }
#[derive(Clone, PartialEq)] struct V; impl RangeValue for V {}
fn l80(rs: &RangeSet<V>, from: usize, to: usize) { rs.between(from, to, |a: usize, b: usize, v: &V| {}) }
fn l81(rs: &RangeSet<V>, cd: &ChangeDesc) -> RangeSet<V> { rs.map(cd) }
fn l82() -> RangeSetBuilder<V> { RangeSetBuilder::new() }
fn main() {}
