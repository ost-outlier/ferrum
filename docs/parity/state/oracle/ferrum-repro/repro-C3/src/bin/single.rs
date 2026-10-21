use ferrum_state::*;
fn main() {
    let cursors = EditorSelection::create(vec![SelectionRange::cursor(0), SelectionRange::cursor(1)], 0);
    let st = EditorState::create(EditorStateConfig { doc: Some(Text::from("123")), selection: Some(cursors.clone()), ..Default::default() });
    println!("ranges after create without allowMultipleSelections: {}", st.selection().ranges().len());
    let tr = st.update(TransactionSpec::new().with_selection(EditorSelection::cursor(9)));
    match tr { Some(tr) => { let n = st.apply(tr); println!("update with selection 9 on doc len 3 => head {}", n.selection().main().head) }, None => println!("update returned None") }
}
