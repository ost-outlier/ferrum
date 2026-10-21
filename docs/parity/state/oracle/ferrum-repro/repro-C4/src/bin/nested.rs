use ferrum_state::*;
fn main() {
    let num = Facet::<usize>::define_list();
    let c = Compartment::new();
    println!("start nested compartment");
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![c.of(c.of(num.of(1)))]), ..Default::default() });
    println!("{:?}", st.facet(&num));
}
