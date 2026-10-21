use ferrum_state::*;
fn main() {
    let st = EditorState::create(EditorStateConfig::default());
    let cat = st.char_categorizer(0);
    println!("Q1 categorizer(\"\\u{{301}}e\")={:?} categorizer(\"\")={:?}", cat("\u{301}e"), cat(""));
    println!("Q2 Text::empty().len_lines()={}", Text::empty().len_lines());
    let r = std::panic::catch_unwind(|| Text::from_str("ab").line_at(3));
    println!("Q3 line_at(3) on len 2 panics={}", r.is_err());
}
