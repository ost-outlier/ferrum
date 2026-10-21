use ferrum_state::*;
fn main() {
    let d = Text::of(&["a\u{2028}b", "c"]);
    println!("{} {} {:?}", d.len_lines(), d.len(), d.line(1).text);
}
