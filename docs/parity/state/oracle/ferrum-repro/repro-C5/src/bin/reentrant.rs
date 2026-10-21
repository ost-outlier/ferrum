use ferrum_state::*;
// cm6 test-state.ts:142 field update reads tr.state.doc.length
fn main() {
    let field = StateField::<usize>::define(|_| 0, |v, tr| if tr.doc_changed() { tr.state().doc().len_chars() } else { *v });
    let s = EditorState::create(EditorStateConfig { extensions: Some(vec![field.extension()]), ..Default::default() });
    let tr = s.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "hi"))).unwrap();
    let n = s.apply(tr);
    println!("field = {:?}", n.field(&field));
}
