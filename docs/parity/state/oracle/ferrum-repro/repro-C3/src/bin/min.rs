use ferrum_state::*;
fn main() {
    let s = EditorSelection::create(vec![SelectionRange::cursor(12), SelectionRange::new(12, 18), SelectionRange::cursor(12)], 2);
    let v: Vec<String> = s.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect();
    println!("{} main={}", v.join(","), s.main_index());
}
