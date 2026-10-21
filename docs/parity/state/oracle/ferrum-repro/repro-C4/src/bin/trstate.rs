use ferrum_state::*;
fn main() {
    let num = Facet::<usize>::define_list();
    let n2 = num.clone();
    let fld = StateField::define(|_| String::new(), move |_v: &String, tr: &Transaction| format!("{:?}", tr.state().facet(&n2)));
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![fld.extension(), num.compute(|s| s.doc().len_chars())]), ..Default::default() });
    println!("created");
    let tr = st.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "ab"))).unwrap();
    let st2 = st.apply(tr);
    println!("{:?}", st2.field(&fld));
}
