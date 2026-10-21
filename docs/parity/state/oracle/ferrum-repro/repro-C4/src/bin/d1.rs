use ferrum_state::*;
fn main() {
    let num = Facet::<usize>::define_list();
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![num.compute_multi(|s| if s.doc().len_chars() % 2 == 1 { vec![100, 10] } else { vec![] }), num.of(1)]), ..Default::default() });
    let st = st.apply(st.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "😀"))).unwrap());
    println!("D1 compute_multi parity after inserting U+1F600: {:?} len_chars={}", st.facet(&num), st.doc().len_chars());
}
