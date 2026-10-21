//! Ferrum side of the C6 RangeSet differential oracle. Reads cases written by
//! cm6/probe-C6/oracle.ts and prints results in the same line format.
use ferrum_state::change::{ChangeDesc, ChangeSet, MapMode};
use ferrum_state::rangeset::{Range, RangeComparator, RangeSet, RangeSetUpdate, RangeValue, SpanIterator};
use ferrum_state::text::Text;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
struct V { name: String, num: usize, s: i32, e: i32, p: bool, mm: MapMode }
impl RangeValue for V {
    fn start_side(&self) -> i32 { self.s }
    fn end_side(&self) -> i32 { self.e }
    fn point(&self) -> bool { self.p }
    fn map_mode(&self) -> MapMode { self.mm }
    fn eq_val(&self, o: &Self) -> bool { self.name == o.name }
}
fn parse_range(l: &str) -> Range<V> {
    let t: Vec<&str> = l.split_whitespace().collect();
    let mm = match t[6] { "0" => MapMode::Simple, "1" => MapMode::TrackDel, "2" => MapMode::TrackBefore, _ => MapMode::TrackAfter };
    Range::new(t[0].parse().unwrap(), t[1].parse().unwrap(), V { name: t[2].into(), num: t[2][1..].parse().unwrap(), s: t[3].parse().unwrap(), e: t[4].parse().unwrap(), p: t[5] == "1", mm })
}
fn iter_str(s: &RangeSet<V>) -> String { s.iter().iter().map(|r| format!("{}-{}:{}", r.from, r.to, r.value.name)).collect::<Vec<_>>().join(" ") }
fn names(a: &[V]) -> String { a.iter().map(|v| v.name.clone()).collect::<Vec<_>>().join(",") }
struct Sp(Vec<String>);
impl SpanIterator<V> for Sp {
    fn span(&mut self, f: usize, t: usize, a: &[V], o: usize) { self.0.push(format!("s({},{},[{}],{})", f, t, names(a), o)) }
    fn point(&mut self, f: usize, t: usize, v: &V, a: &[V], o: usize, i: usize) { self.0.push(format!("p({},{},{},[{}],{},{})", f, t, v.name, names(a), o, i)) }
}
struct Cp(Vec<String>);
impl RangeComparator<V> for Cp {
    fn compare_range(&mut self, f: usize, t: usize, a: &[V], b: &[V]) { self.0.push(format!("r({},{},[{}],[{}])", f, t, names(a), names(b))) }
    fn compare_point(&mut self, f: usize, t: usize, a: Option<&V>, b: Option<&V>) { self.0.push(format!("p({},{},{},{})", f, t, a.map_or("-".into(), |v| v.name.clone()), b.map_or("-".into(), |v| v.name.clone()))) }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = text.lines();
    while let Some(head) = lines.next() {
        let h: Vec<&str> = head.split_whitespace().collect();
        if h.is_empty() { continue; }
        assert_eq!(h[0], "CASE");
        let id = h[1];
        let mut sets_r: Vec<Vec<Range<V>>> = vec![];
        let mut new_r: Vec<Vec<Range<V>>> = vec![];
        let mut sections: Vec<i64> = vec![];
        let (mut f_from, mut f_to, mut f_mod) = (0usize, 0usize, 0usize);
        let mut add = vec![];
        let (mut b_from, mut b_to, mut s_from, mut s_to, mut min_ps) = (0usize, 0usize, 0usize, 0usize, -1isize);
        loop {
            let l = lines.next().unwrap();
            let t: Vec<&str> = l.split_whitespace().collect();
            match t[0] {
                "S" => { let n: usize = t[1].parse().unwrap(); sets_r.push((0..n).map(|_| parse_range(lines.next().unwrap())).collect()); }
                "C" => { sections = t[1..].iter().map(|x| x.parse().unwrap()).collect(); }
                "U" => { f_from = t[1].parse().unwrap(); f_to = t[2].parse().unwrap(); f_mod = t[3].parse().unwrap(); let n: usize = t[4].parse().unwrap(); add = (0..n).map(|_| parse_range(lines.next().unwrap())).collect(); }
                "B" => { b_from = t[1].parse().unwrap(); b_to = t[2].parse().unwrap(); }
                "P" => { s_from = t[1].parse().unwrap(); s_to = t[2].parse().unwrap(); min_ps = t[3].parse().unwrap(); }
                "Q" => {}
                "N" => { let n: usize = t[1].parse().unwrap(); new_r.push((0..n).map(|_| parse_range(lines.next().unwrap())).collect()); }
                "END" => break,
                x => panic!("bad line {}", x),
            }
        }
        let out = |op: &str, s: String| println!("{} {} {}", id, op, s);
        let sets: Vec<RangeSet<V>> = sets_r.into_iter().map(|r| RangeSet::of(r).expect("cm6 accepted this set")).collect();
        for (k, s) in sets.iter().enumerate() { out(&format!("iter{}", k), iter_str(s)); }
        for (k, s) in sets.iter().enumerate() { out(&format!("size{}", k), s.size().to_string()); }
        { let mut f = vec![]; sets[0].between(b_from, b_to, |a, b, v| { f.push(format!("{}-{}:{}", a, b, v.name)); true }); f.sort(); out("between", f.join(" ")); }
        let desc = ChangeDesc::create(sections.clone());
        let mut ins = vec![]; for i in (0..sections.len()).step_by(2) { if sections[i + 1] >= 0 { ins.push(Text::from("x".repeat(sections[i + 1] as usize).as_str())); } }
        let cs = ChangeSet::create_set(sections.clone(), ins);
        let mapped: Vec<RangeSet<V>> = sets.iter().map(|s| s.map(&desc)).collect();
        out("map", iter_str(&mapped[0]));
        let filter: Option<Arc<dyn Fn(usize, usize, &V) -> bool + Send + Sync>> = if f_mod > 0 { let m = f_mod; Some(Arc::new(move |_, _, v: &V| v.num % m != 0)) } else { None };
        let updated = sets[0].update(RangeSetUpdate { add, sort: false, filter, filter_from: f_from, filter_to: f_to });
        match &updated { Ok(u) => out("update", iter_str(u)), Err(_) => out("update", "THROWS".into()) }
        { let mut sp = Sp(vec![]); let ret = RangeSet::spans(&sets, s_from, s_to, &mut sp, min_ps); out("spans", format!("{} ret={}", sp.0.join(" "), ret)); }
        { let mut new_sets = mapped.clone(); if let Ok(u) = &updated { new_sets[0] = u.map(&desc); }
          let mut cp = Cp(vec![]); RangeSet::compare(&sets, &new_sets, &cs, &mut cp); out("compare", cp.0.join(" "));
          let given: Vec<RangeSet<V>> = new_r.iter().map(|r| RangeSet::of(r.clone()).expect("cm6 new set")).collect();
          let mut cp2 = Cp(vec![]); RangeSet::compare(&sets, &given, &cs, &mut cp2); out("compareN", cp2.0.join(" ")); }
    }
}
