// Differential oracle consumer: reads "<sexp>\t<cm6 result>" lines, rebuilds the extension tree
// with ferrum-state, compares the facet output. Shared nodes "(d K x)"/"(r K)" become Extension clones.
use ferrum_state::*;
use std::collections::HashMap;
fn toks(s: &str) -> Vec<String> { s.replace('(', " ( ").replace(')', " ) ").split_whitespace().map(String::from).collect() }
fn parse(t: &[String], i: &mut usize, num: &Facet<usize>, defs: &mut HashMap<usize, Extension>) -> Extension {
    let tok = &t[*i]; *i += 1;
    if tok != "(" { return num.of(tok[1..].parse().unwrap()); }
    let head = t[*i].clone(); *i += 1;
    let e = match head.as_str() {
        "p" => { let p: usize = t[*i].parse().unwrap(); *i += 1; let inner = parse(t, i, num, defs);
                 inner.with_prec([Prec::Highest, Prec::High, Prec::Default, Prec::Low, Prec::Lowest][p]) }
        "g" => { let mut v = Vec::new(); while t[*i] != ")" { v.push(parse(t, i, num, defs)); } Extension::Group(v) }
        "d" => { let k: usize = t[*i].parse().unwrap(); *i += 1; let inner = parse(t, i, num, defs); defs.insert(k, inner.clone()); inner }
        "r" => { let k: usize = t[*i].parse().unwrap(); *i += 1; defs[&k].clone() }
        _ => panic!("bad"),
    };
    assert_eq!(t[*i], ")"); *i += 1; e
}
fn main() {
    let path = std::env::args().nth(1).unwrap();
    let num = Facet::<usize>::define_list();
    let (mut ok, mut bad) = (0, 0);
    let mut first: Option<(String, String, String)> = None;
    let mut shortest: Option<(String, String, String)> = None;
    for line in std::fs::read_to_string(&path).unwrap().lines() {
        let (sx, want) = line.split_once('\t').unwrap();
        let t = toks(sx); let mut i = 0; let mut defs = HashMap::new();
        let ext = parse(&t, &mut i, &num, &mut defs);
        let st = EditorState::create(EditorStateConfig { extensions: Some(vec![ext]), ..Default::default() });
        let got = st.facet(&num).iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        if got == want { ok += 1 } else {
            bad += 1;
            let c = (sx.to_string(), want.to_string(), got);
            if first.is_none() { first = Some(c.clone()) }
            if shortest.as_ref().map_or(true, |s| s.0.len() > c.0.len()) { shortest = Some(c) }
        }
    }
    println!("{}: match={} mismatch={}", path, ok, bad);
    if let Some(s) = shortest { println!("shortest mismatch: {}\n  cm6: {}\n  ferrum: {}", s.0, s.1, s.2) }
}
