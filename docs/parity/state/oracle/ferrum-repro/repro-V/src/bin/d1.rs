use ferrum_state::*;
fn main() {
    let d = Text::of(&["a😀b"]);
    println!("D1a length {} (bytes {})", d.len_chars(), d.len_bytes());
    let cs = ChangeSet::of_spec(ChangeSpec::insert(1, "😀"), 3);
    println!("D1b insert section {} new_length {}", cs, cs.new_length());
    println!("D1c find_cluster_break(😀,0) {} flags {}", find_cluster_break("😀", 0, true), find_cluster_break("🇫🇷🇩🇪", 0, true));
    println!("D1d find_column(😀b,1,4) {:?}", find_column("😀b", 1, 4, false));
    let st = EditorState::create(EditorStateConfig { doc: Some(Text::from("ab")), selection: Some(EditorSelection::cursor(1)), ..Default::default() });
    let tr = st.update(st.replace_selection("😀")).unwrap();
    println!("D1f replace_selection cursor {}", tr.state().selection().main().head);
    let r = std::panic::catch_unwind(|| { let d = Text::of(&["a😀b"]); ChangeSet::of_spec(ChangeSpec::delete(1, 3), 4).apply(&d).to_string() });
    println!("D1h cm6 delete(1,3) replayed: {:?}", r.map_err(|e| e.downcast_ref::<String>().cloned()));
}
