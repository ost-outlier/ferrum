use ferrum_state::change::{ChangeDesc, ChangeSet, ChangeSpec, MapMode};
use ferrum_state::rangeset::{Range, RangeComparator, RangeSet, RangeSetBuilder, RangeSetUpdate, RangeValue, SpanIterator};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
struct V { name: String, s: i32, e: i32, p: bool, mm: MapMode }
impl RangeValue for V {
    fn start_side(&self) -> i32 { self.s }
    fn end_side(&self) -> i32 { self.e }
    fn point(&self) -> bool { self.p }
    fn map_mode(&self) -> MapMode { self.mm }
    fn eq_val(&self, o: &Self) -> bool { self.name == o.name }
}
fn v(n: &str) -> V { V { name: n.into(), s: 0, e: 0, p: false, mm: MapMode::TrackDel } }
fn vs(n: &str, s: i32, e: i32, p: bool) -> V { V { name: n.into(), s, e, p, mm: MapMode::TrackDel } }
fn r(from: usize, to: usize, val: V) -> Range<V> { Range::new(from, to, val) }
fn show(set: &RangeSet<V>) -> String { set.iter().iter().map(|r| format!("{}-{}:{}", r.from, r.to, r.value.name)).collect::<Vec<_>>().join(" ") }
fn log(id: &str, s: impl std::fmt::Display) { println!("{} => {}", id, s); }
fn cs(specs: &[ChangeSpec], len: usize) -> ChangeSet { ChangeSet::of_specs(specs, len) }

#[derive(Default)]
struct Comp { calls: Vec<String> }
impl RangeComparator<V> for Comp {
    fn compare_range(&mut self, f: usize, t: usize, a: &[V], b: &[V]) {
        self.calls.push(format!("range({},{},[{}],[{}])", f, t, a.iter().map(|v| v.name.clone()).collect::<Vec<_>>().join(","), b.iter().map(|v| v.name.clone()).collect::<Vec<_>>().join(",")));
    }
    fn compare_point(&mut self, f: usize, t: usize, a: Option<&V>, b: Option<&V>) {
        self.calls.push(format!("point({},{},{},{})", f, t, a.map_or("null".into(), |v| v.name.clone()), b.map_or("null".into(), |v| v.name.clone())));
    }
}
#[derive(Default)]
struct Spans { calls: Vec<String> }
impl SpanIterator<V> for Spans {
    fn span(&mut self, f: usize, t: usize, a: &[V], o: usize) {
        self.calls.push(format!("span({},{},[{}],{})", f, t, a.iter().map(|v| v.name.clone()).collect::<Vec<_>>().join(","), o));
    }
    fn point(&mut self, f: usize, t: usize, val: &V, a: &[V], o: usize, i: usize) {
        self.calls.push(format!("point({},{},{},[{}],{},{})", f, t, val.name, a.iter().map(|v| v.name.clone()).collect::<Vec<_>>().join(","), o, i));
    }
}
fn none_if_empty(s: String) -> String { if s.is_empty() { "(no calls)".into() } else { s } }

fn main() {
    // R1
    { let s = RangeSet::of(vec![r(0, 5, v("A")), r(0, 3, v("B"))]).unwrap(); log("R1 iter", show(&s)); }
    // R2
    { let s = RangeSet::of(vec![r(5, 5, vs("L", -200_000_000, -200_000_000, true))]).unwrap(); let mut f = vec![]; s.between(5, 10, |a, b, v| { f.push(format!("{}-{}:{}", a, b, v.name)); true }); log("R2a between(5,10) endSide=-2e8 at 5", f.join(" ")); }
    { let s = RangeSet::of(vec![r(5, 5, vs("W", 100_000_001, 100_000_001, true))]).unwrap(); let mut f = vec![]; s.between(0, 5, |a, b, v| { f.push(format!("{}-{}:{}", a, b, v.name)); true }); log("R2b between(0,5) startSide=1e8+1 at 5", f.join(" ")); }
    // R3
    { let s = RangeSet::of(vec![r(5, 5, vs("Wbefore", -100_000_000, -100_000_000, true))]).unwrap(); log("R3a widget side -1e8 at 5, insert at 5", show(&s.map(cs(&[ChangeSpec::insert(5, "x")], 10).desc()))); }
    { let s = RangeSet::of(vec![r(4, 6, vs("Repl", 499_999_999, -599_999_999, true))]).unwrap(); log("R3b replace(4,6) sides 5e8-1/-6e8+1, insert at 4 and 6", show(&s.map(cs(&[ChangeSpec::insert(4, "x"), ChangeSpec::insert(6, "y")], 10).desc()))); }
    { let s = RangeSet::of(vec![r(4, 6, vs("Mark", 500_000_000, -600_000_000, false))]).unwrap(); log("R3c mark(4,6) sides 5e8/-6e8, insert at 6", show(&s.map(cs(&[ChangeSpec::insert(6, "y")], 10).desc()))); }
    { let s = RangeSet::of(vec![r(4, 6, vs("S128", 128, -128, false))]).unwrap(); log("R3d range(4,6) sides 128/-128, insert at 4 and 6", show(&s.map(cs(&[ChangeSpec::insert(4, "x"), ChangeSpec::insert(6, "y")], 10).desc()))); }
    // R4
    { let mut p = vs("P", 0, 0, true); p.mm = MapMode::Simple; let s = RangeSet::of(vec![r(5, 5, p)]).unwrap(); log("R4a Simple point at 5, delete 2..8", show(&s.map(cs(&[ChangeSpec::delete(2, 8)], 10).desc()))); }
    { let s = RangeSet::of(vec![r(4, 6, vs("I", -1, 1, false))]).unwrap(); log("R4b inclusive range(4,6), delete 2..8", show(&s.map(cs(&[ChangeSpec::delete(2, 8)], 10).desc()))); }
    { let s = RangeSet::of(vec![r(0, 1, v("X")), r(4, 6, vs("I", -1, 1, false))]).unwrap(); log("R4c same + (0,1) in chunk, delete 2..8 (touch not cover)", show(&s.map(cs(&[ChangeSpec::delete(2, 8)], 10).desc()))); }
    // R5
    { let s = RangeSet::of(vec![r(0, 1, v("X")), r(3, 3, vs("E", 1, -1, true))]).unwrap(); log("R5 empty(3,3) sides 1/-1, delete 1..2", show(&s.map(cs(&[ChangeSpec::delete(1, 2)], 10).desc()))); }
    // R6
    { let rs: Vec<_> = (0..1000).map(|i| r(i, i + 1, v(&format!("v{}", i)))).collect(); let s = RangeSet::of(rs).unwrap();
      let m = s.map(cs(&[ChangeSpec::insert(1000, "x")], 1001).desc()); log("R6 map change after all chunks: chunk[0] shared", Arc::ptr_eq(&m.chunk[0], &s.chunk[0]));
      let u = s.update(RangeSetUpdate { add: vec![r(999, 999, vs("p", 1, 1, true))], ..Default::default() }).unwrap(); log("R6b update add at end: chunk[0] shared", Arc::ptr_eq(&u.chunk[0], &s.chunk[0])); }
    // R7
    { let s = RangeSet::of(vec![r(1, 2, v("a"))]).unwrap(); let u = s.update(RangeSetUpdate::default()).unwrap(); log("R7 update({}) shares chunk[0]", Arc::ptr_eq(&u.chunk[0], &s.chunk[0]));
      let e = s.update(RangeSetUpdate { filter: Some(Arc::new(|_, _, _| false)), ..Default::default() }).unwrap(); log("R7b filter all -> is_empty / chunk.len / next_layer", format!("{} {} {:?}", e.is_empty(), e.chunk.len(), e.next_layer.is_some())); }
    // R8
    { let s = RangeSet::of(vec![r(5, 6, v("a"))]).unwrap();
      log("R8a update add unsorted sort=false", match s.update(RangeSetUpdate { add: vec![r(8, 9, v("c")), r(1, 2, v("b"))], sort: false, ..Default::default() }) { Ok(x) => show(&x), Err(e) => format!("Err: {}", e) });
      log("R8b update add unsorted sort=false (overlap)", match s.update(RangeSetUpdate { add: vec![r(8, 9, v("c")), r(7, 10, v("b"))], sort: false, ..Default::default() }) { Ok(x) => show(&x), Err(e) => format!("Err: {}", e) });
      log("R8c update add unsorted sort=true", match s.update(RangeSetUpdate { add: vec![r(8, 9, v("c")), r(1, 2, v("b"))], sort: true, ..Default::default() }) { Ok(x) => show(&x), Err(e) => format!("Err: {}", e) });
      log("R8d empty set update add unsorted sort=false", match RangeSet::<V>::empty().update(RangeSetUpdate { add: vec![r(8, 9, v("c")), r(1, 2, v("b"))], sort: false, ..Default::default() }) { Ok(x) => show(&x), Err(e) => format!("Err: {}", e) }); }
    // R9
    { log("R9 of([(0,0,s1/e-1),(0,0,s-1/e-1)])", match RangeSet::of(vec![r(0, 0, vs("a", 1, -1, true)), r(0, 0, vs("b", -1, -1, true))]) { Ok(x) => show(&x), Err(e) => format!("Err: {}", e) }); }
    // R10
    { let mut b = RangeSetBuilder::new(); b.add(8, 9, v("a")); b.add(7, 10, v("b")); let s = b.finish(); log("R10 builder.add(8,9) then add(7,10)", format!("{} (size {}, next_layer {})", show(&s), s.size(), s.next_layer.is_some()));
      let mut b2 = RangeSetBuilder::new(); b2.add_strict(8, 9, v("a")).unwrap(); log("R10b add_strict misordered", format!("{:?}", b2.add_strict(7, 10, v("b")))); }
    // R11
    { let s = RangeSet::of(vec![r(0, 6, v("A")), r(5, 10, v("B"))]).unwrap(); let mut it = Spans::default(); let ret = RangeSet::spans(&[s], 5, 10, &mut it, -1); log("R11 spans([A(0,6),B(5,10)],5,10)", format!("{} ret={}", it.calls.join(" "), ret)); }
    // R12
    { let s0 = RangeSet::of(vec![r(0, 10, v("A"))]).unwrap(); let s1 = RangeSet::of(vec![r(0, 6, v("B"))]).unwrap(); let sets = [s0, s1]; let mut it = Spans::default(); let ret = RangeSet::spans(&sets, 0, 10, &mut it, -1); log("R12 spans([[A(0,10)],[B(0,6)]],0,10)", format!("{} ret={}", it.calls.join(" "), ret));
      let mut it2 = Spans::default(); let ret2 = RangeSet::spans(&sets, 0, 8, &mut it2, -1); log("R12b same to=8", format!("{} ret={}", it2.calls.join(" "), ret2)); }
    // R13
    { let s1 = RangeSet::of(vec![r(2, 4, v("a"))]).unwrap(); let s2 = RangeSet::of(vec![r(0, 10, v("b"))]).unwrap(); let mut c = Comp::default(); RangeSet::compare(&[s1.clone()], &[s1, s2], &ChangeSet::empty(10), &mut c); log("R13 compare([s1],[s1,s2])", c.calls.join(" ")); }
    // R14
    { let s = RangeSet::of(vec![r(0, 0, vs("w", 1, 1, true))]).unwrap(); let mut c = Comp::default(); RangeSet::compare(&[RangeSet::empty()], &[s], &ChangeSet::empty(0), &mut c); log("R14 compare empty doc point", none_if_empty(c.calls.join(" "))); }
    // R15
    { let s = RangeSet::of(vec![r(6, 7, v("a"))]).unwrap(); let mut c = Comp::default(); RangeSet::compare(&[s], &[RangeSet::empty()], &cs(&[ChangeSpec::replace(5, 10, "xyz")], 10), &mut c); log("R15 compare tail change", none_if_empty(c.calls.join(" "))); }
    // R16
    { let s = RangeSet::of(vec![r(0, 0, vs("L", -200_000_000, -200_000_000, true))]).unwrap(); let mut c = Comp::default(); RangeSet::compare(&[s], &[RangeSet::empty()], &ChangeSet::empty(10), &mut c); log("R16 compare point at 0 sides -2e8", none_if_empty(c.calls.join(" "))); }
    // R17
    { let a = RangeSet::of(vec![r(0, 5, v("a")), r(5, 10, v("a"))]).unwrap(); let b = RangeSet::of(vec![r(0, 10, v("a"))]).unwrap(); let mut c = Comp::default(); RangeSet::compare(&[a], &[b], &ChangeSet::empty(10), &mut c); log("R17 (no boundChange in trait)", none_if_empty(c.calls.join(" "))); }
    // R18
    { let s = RangeSet::of(vec![r(0, 5, v("a")), r(0, 5, v("b"))]).unwrap(); let mut it = Spans::default(); RangeSet::spans(&[s.clone()], 0, 5, &mut it, -1); log("R18 spans identical ranges", it.calls.join(" ")); log("R18 iter", show(&s)); }
    // R19
    { let mut pts: Vec<_> = (0..250).map(|_| r(1, 1, vs("q", 0, 0, true))).collect(); pts.push(r(5, 5, vs("X", 0, 0, true))); pts.push(r(6, 6, vs("Y", 0, 0, true)));
      let b = RangeSet::of(pts).unwrap(); let a = RangeSet::of(vec![r(0, 2, vs("P", 1, 1, true))]).unwrap(); let mut it = Spans::default(); RangeSet::spans(&[a, b], 0, 10, &mut it, -1); log("R19 spans forward", it.calls.join(" ")); }
    // R20 default eq_val (PartialEq)
    { #[derive(Clone, Debug, PartialEq)] struct Plain(String); impl RangeValue for Plain {}
      struct C2(Vec<String>); impl RangeComparator<Plain> for C2 { fn compare_range(&mut self, f: usize, t: usize, _: &[Plain], _: &[Plain]) { self.0.push(format!("range({},{})", f, t)); } fn compare_point(&mut self, f: usize, t: usize, _: Option<&Plain>, _: Option<&Plain>) { self.0.push(format!("point({},{})", f, t)); } }
      let s1 = RangeSet::of(vec![Range::new(0, 5, Plain("p".into()))]).unwrap(); let s2 = RangeSet::of(vec![Range::new(0, 5, Plain("p".into()))]).unwrap(); let mut c = C2(vec![]); RangeSet::compare(&[s1], &[s2], &ChangeSet::empty(10), &mut c); log("R20 default eq (distinct instances same data)", none_if_empty(c.0.join(" "))); }
    // R21 iter has no from parameter
    { let s = RangeSet::of(vec![r(0, 2, v("a")), r(3, 4, v("b")), r(5, 9, v("c"))]).unwrap(); log("R21 iter() (no from param)", s.iter().iter().map(|r| r.value.name.clone()).collect::<Vec<_>>().join(",")); }
    // R24 spans minPointSize
    { let s = RangeSet::of(vec![r(0, 2, vs("small", 0, 0, true)), r(3, 9, v("m")), r(5, 8, vs("big", 0, 0, true))]).unwrap(); let mut it = Spans::default(); let ret = RangeSet::spans(&[s], 0, 10, &mut it, 3); log("R24 spans minPointSize=3", format!("{} ret={}", it.calls.join(" "), ret)); }
    // R25
    { let s = RangeSet::of(vec![r(0, 5, v("A")), r(0, 3, v("B"))]).unwrap(); log("R25 size", s.size()); log("R25 length", s.length()); }
    // R26
    { let s = RangeSet::of(vec![r(0, 10, v("A")), r(2, 6, vs("P", 0, 0, true))]).unwrap(); let mut it = Spans::default(); let ret = RangeSet::spans(&[s], 4, 8, &mut it, -1); log("R26 spans from inside point", format!("{} ret={}", it.calls.join(" "), ret)); }
    // R27
    { let s = RangeSet::of(vec![r(0, 5, v("A")), r(0, 3, v("B")), r(6, 7, v("C"))]).unwrap(); let mut f = vec![]; s.between(0, 10, |_, _, v| { f.push(v.name.clone()); v.name != "A" }); log("R27 between stop", f.join(",")); let mut g = vec![]; s.between(0, 10, |_, _, v| { g.push(v.name.clone()); true }); log("R27b between order", g.join(",")); }
    // R28
    { let s = RangeSet::of(vec![r(1, 2, v("a"))]).unwrap(); let m = s.map(ChangeSet::empty(10).desc()); log("R28 map(empty) shares chunk[0]", Arc::ptr_eq(&m.chunk[0], &s.chunk[0])); }
    // R29
    { let s = RangeSet::of(vec![r(0, 2, v("a")), r(2, 4, v("b")), r(4, 6, v("c")), r(6, 8, v("d"))]).unwrap(); let called = Arc::new(Mutex::new(Vec::<String>::new())); let c2 = called.clone();
      let u = s.update(RangeSetUpdate { filter: Some(Arc::new(move |_, _, v: &V| { c2.lock().unwrap().push(v.name.clone()); false })), filter_from: 4, filter_to: 4, ..Default::default() }).unwrap(); log("R29 filter touching 4..4", format!("{} -> {}", called.lock().unwrap().join(","), show(&u))); }
    // R31 map with ChangeDesc built from sections: deletion covering, assoc truncation boundary values
    { for side in [127i32, 128, 255, 256, -129, 100_000_001, -100_000_000, -200_000_000, 499_999_999, -599_999_999] {
        let s = RangeSet::of(vec![r(5, 5, vs("p", side, side, true))]).unwrap(); let m = s.map(&ChangeDesc::create(vec![5, -1, 0, 1, 5, -1]));
        println!("R31 point side {} insert at 5 => {}", side, show(&m)); } }
}
