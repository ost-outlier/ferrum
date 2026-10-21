use ferrum_state::*;
fn main() {
    let num = Facet::<usize>::define_list();
    let c = Compartment::new();
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![num.of(1), c.of(num.compute(|s| s.doc().len_chars()))]), ..Default::default() });
    let st = st.apply(st.update(TransactionSpec::new().with_effect(c.reconfigure(Extension::Group(vec![])))).unwrap());
    println!("R67 {:?}", st.facet(&num));
    // R66-like: no init override API; R64
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![num.of(1), num.of(1)]), ..Default::default() });
    println!("R64 {:?}", st.facet(&num));
}
