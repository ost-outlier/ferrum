use ferrum_state::*;
fn fmt(s: &EditorSelection) -> String {
    let v: Vec<String> = s.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect();
    format!("{} main={}", v.join(","), s.main_index())
}
fn fmt_a(s: &EditorSelection) -> String {
    let v: Vec<String> = s.ranges().iter().map(|r| r.assoc.to_string()).collect();
    v.join(",")
}
fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(format!("{}/sel-cases.txt", dir)).unwrap();
    let mut out = String::new();
    for line in data.lines() {
        let p: Vec<&str> = line.split('\t').collect();
        let rs: Vec<SelectionRange> = p[1].split(',').map(|x| { let ah: Vec<usize> = x.split('/').map(|v| v.parse().unwrap()).collect(); SelectionRange::new(ah[0], ah[1]) }).collect();
        let main: usize = p[2].parse().unwrap();
        let ch: Vec<usize> = p[3].split(',').map(|v| v.parse().unwrap()).collect();
        let assoc: i8 = p[4].parse().unwrap();
        let sel = EditorSelection::create(rs, main);
        let cs = ChangeSet::of_spec(ChangeSpec::replace(ch[0], ch[1], "x".repeat(ch[2]).as_str()), 20);
        let mapped = sel.map(cs.desc(), assoc);
        out.push_str(&format!("{}\tcreate:{}\tmap:{}\tassocCreate:{}\tassocMap:{}\n", p[0], fmt(&sel), fmt(&mapped), fmt_a(&sel), fmt_a(&mapped)));
    }
    std::fs::write(format!("{}/sel-ferrum.txt", dir), out).unwrap();
}
