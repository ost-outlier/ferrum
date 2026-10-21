use ferrum_state::*;
fn main() {
    let s = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    // minimal parser: rely on the fixed JSON shape produced by selnorm.ts
    let mut mism = 0; let mut total = 0; let mut first: Option<String> = None;
    for case in s.trim_start_matches("[{").trim_end_matches("}]").split("},{") {
        let get = |k: &str| -> String { let i = case.find(&format!("\"{}\":", k)).unwrap() + k.len() + 3; let rest = &case[i..];
            if rest.starts_with('[') { let mut d = 0; for (j, c) in rest.char_indices() { if c == '[' { d += 1 } if c == ']' { d -= 1; if d == 0 { return rest[..=j].to_string() } } } unreachable!() } else { rest.split(',').next().unwrap().to_string() } };
        let pairs = |v: String| -> Vec<(usize, usize)> { let nums: Vec<usize> = v.split(|c: char| !c.is_ascii_digit()).filter(|t| !t.is_empty()).map(|t| t.parse().unwrap()).collect(); nums.chunks(2).map(|c| (c[0], c[1])).collect() };
        let ranges = pairs(get("ranges")); let main: usize = get("main").parse().unwrap();
        let expect = pairs(get("result")); let expect_main: usize = get("resultMain").parse().unwrap();
        let sel = EditorSelection::create(ranges.iter().map(|&(a, h)| SelectionRange::new(a, h)).collect(), main);
        let got: Vec<(usize, usize)> = sel.ranges().iter().map(|r| (r.anchor, r.head)).collect();
        total += 1;
        if got != expect || sel.main_index() != expect_main { mism += 1; if first.is_none() { first = Some(format!("{:?} main {} -> cm6 {:?} main {} | ferrum {:?} main {}", ranges, main, expect, expect_main, got, sel.main_index())); } }
    }
    println!("selnorm: {} cases, {} mismatches; first: {:?}", total, mism, first);
}
