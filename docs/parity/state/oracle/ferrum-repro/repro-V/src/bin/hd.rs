use ferrum_state::*;
fn main() {
    // fe.105: pub Compartment.id lets a caller alias two compartments
    let a = Compartment::new();
    let mut b = Compartment::new();
    println!("fe.105 before: a == b {}", a == b);
    b.id = a.id;
    println!("fe.105 after b.id = a.id: a == b {}", a == b);
    let num = Facet::<usize>::define_list();
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![a.of(num.of(1))]), ..Default::default() });
    let st2 = st.apply(st.update(TransactionSpec::new().with_effect(b.reconfigure(num.of(9)))).unwrap());
    println!("fe.105 reconfiguring b replaces a's content: {:?}", st2.facet(&num));
    // fe.079: default_erased just returns the default
    let d = AnyFacet::default_erased(&num);
    println!("fe.079 default_erased downcast: {:?}", d.downcast_ref::<Vec<usize>>());
    // fe.102: AnyStateField::init computes a value, nothing is stored
    let f = StateField::<usize>::define(|s| s.doc().len_chars(), |v, _| *v);
    let st3 = EditorState::create(EditorStateConfig { doc: Some(Text::from("abc")), extensions: Some(vec![f.extension()]), ..Default::default() });
    let v = AnyStateField::init(&f, &st3);
    println!("fe.102 init -> {:?}; field unchanged {:?}", v.downcast_ref::<usize>(), st3.field(&f));
}
