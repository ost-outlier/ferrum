use ferrum_state::*;
fn show(s: &EditorSelection) -> String { format!("{} main={}", s.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect::<Vec<_>>().join(","), s.main_index()) }
fn main() {
    let c = EditorSelection::create(vec![SelectionRange::cursor(12), SelectionRange::new(12, 18), SelectionRange::cursor(12)], 2);
    println!("4a case: {}", show(&c));
    let data = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (mut total, mut mism, mut pos_mism) = (0, 0, 0); let mut first = None;
    for l in data.lines() {
        let parts: Vec<&str> = l.split(" | ").collect();
        let inp: Vec<usize> = parts[0].split(' ').map(|t| t.parse().unwrap()).collect();
        let exp_r: Vec<usize> = parts[1].split(' ').map(|t| t.parse().unwrap()).collect();
        let exp_m: usize = parts[2].parse().unwrap();
        let ranges: Vec<SelectionRange> = inp[..6].chunks(2).map(|c| SelectionRange::new(c[0], c[1])).collect();
        let sel = EditorSelection::create(ranges, inp[6]);
        let got: Vec<usize> = sel.ranges().iter().flat_map(|r| [r.anchor, r.head]).collect();
        total += 1;
        if got != exp_r { pos_mism += 1 }
        if got != exp_r || sel.main_index() != exp_m { mism += 1; if first.is_none() { first = Some(l.to_string() + &format!(" || ferrum {:?} main {}", got, sel.main_index())) } }
    }
    println!("exhaustive: {} cases, {} mismatches ({} in ranges, rest main only); first: {:?}", total, mism, pos_mism, first);
}
