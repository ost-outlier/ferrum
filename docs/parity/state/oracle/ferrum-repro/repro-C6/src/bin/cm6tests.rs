//! Replays every cm6 `state/test/test-rangeset.ts` case with cm6's exact inputs and expected
//! values against Ferrum's public API. cm6 `Value` semantics are reproduced exactly
//! (default sides, point = empty || spec.point, eq by name). ChangeSets use the exact
//! sections produced by cm6's `changeSet` helper (printed by probe-C6/sections.ts).
use ferrum_state::change::{ChangeDesc, ChangeSet};
use ferrum_state::rangeset::{Range, RangeComparator, RangeSet, RangeSetUpdate, RangeValue, SpanIterator};
use ferrum_state::text::Text;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
struct Value { start_side: i32, end_side: i32, point: bool, name: Option<String>, pos: Option<usize> }
impl RangeValue for Value {
    fn start_side(&self) -> i32 { self.start_side }
    fn end_side(&self) -> i32 { self.end_side }
    fn point(&self) -> bool { self.point }
    fn eq_val(&self, o: &Self) -> bool { o.name == self.name }
}
#[derive(Default, Clone)]
struct Spec { start_side: Option<i32>, end_side: Option<i32>, point: bool, name: Option<String>, pos: Option<usize> }
fn name(n: &str) -> Spec { Spec { name: Some(n.into()), ..Default::default() } }
fn mkv(spec: Spec, empty: bool) -> Value {
    let start_side = spec.start_side.unwrap_or(1);
    let end_side = spec.end_side.or(spec.start_side).unwrap_or(if empty { 1 } else { -1 });
    Value { start_side, end_side, point: empty || spec.point, name: spec.name, pos: spec.pos }
}
fn mk(from: usize, to: usize, spec: Spec) -> Range<Value> { Range::new(from, to, mkv(spec, from == to)) }
fn mk1(at: usize, spec: Spec) -> Range<Value> { mk(at, at, spec) }
fn names(v: &[Value]) -> String { v.iter().filter(|v| v.name.is_some() || v.point).map(|v| v.name.clone().unwrap_or("POINT".into())).collect::<Vec<_>>().join("/") }
fn mk_set(r: Vec<Range<Value>>) -> Result<RangeSet<Value>, &'static str> { RangeSet::of(r) }
fn small_ranges() -> Vec<Range<Value>> { (0..5000).map(|i| mk(i, i + 1 + (i % 4), Spec { pos: Some(i), ..Default::default() })).collect() }
fn set0() -> RangeSet<Value> { mk_set(small_ranges()).unwrap() }
fn desc(s: &[i64]) -> ChangeDesc { ChangeDesc::create(s.to_vec()) }
fn cset(s: &[i64]) -> ChangeSet {
    let mut ins = vec![];
    for i in (0..s.len()).step_by(2) { if s[i + 1] >= 0 { ins.push(Text::from("x".repeat(s[i + 1] as usize).as_str())); } }
    ChangeSet::create_set(s.to_vec(), ins)
}
const EMPTY: &[i64] = &[5100, -1];
fn check_set(set: &RangeSet<Value>) -> bool {
    let mut count = 0; let mut ok = true;
    set.between(0, set.length(), |from, _, v| { count += 1; if let Some(p) = v.pos { if from != p { ok = false; } } true });
    ok && count == set.size()
}
fn sorted_by_from(mut r: Vec<Range<Value>>) -> Vec<Range<Value>> { r.sort_by(|a, b| a.from.cmp(&b.from)); r }
fn iter_str(set: &RangeSet<Value>) -> String { set.iter().iter().map(|r| format!("{}-{}", r.from, r.to)).collect::<Vec<_>>().join(",") }
fn pts(p: &[(usize, usize)]) -> String { p.iter().map(|(a, b)| format!("{}-{}", a, b)).collect::<Vec<_>>().join(",") }

struct Comparator { ranges: Vec<usize> }
impl Comparator { fn add(&mut self, f: usize, t: usize) { if self.ranges.last() == Some(&f) { *self.ranges.last_mut().unwrap() = t } else { self.ranges.push(f); self.ranges.push(t) } } }
impl RangeComparator<Value> for Comparator {
    fn compare_range(&mut self, f: usize, t: usize, _: &[Value], _: &[Value]) { self.add(f, t) }
    fn compare_point(&mut self, f: usize, t: usize, _: Option<&Value>, _: Option<&Value>) { self.add(f, t) }
}
fn cmp_test(set: RangeSet<Value>, changes: &[i64], filter: Option<Arc<dyn Fn(usize, usize, &Value) -> bool + Send + Sync>>, add: Vec<Range<Value>>) -> String {
    let doc = cset(changes);
    let mut new_set = set.clone();
    if changes != EMPTY { new_set = new_set.map(doc.desc()); }
    if filter.is_some() || !add.is_empty() { new_set = new_set.update(RangeSetUpdate { add, filter, ..Default::default() }).unwrap(); }
    let mut c = Comparator { ranges: vec![] };
    RangeSet::compare(&[set], &[new_set], &doc, &mut c);
    format!("{:?}", c.ranges)
}
struct Builder { spans: Vec<String> }
impl SpanIterator<Value> for Builder {
    fn span(&mut self, f: usize, t: usize, a: &[Value], _: usize) { let n = names(a); self.spans.push(format!("{}{}", t - f, if n.is_empty() { String::new() } else { format!("={}", n) })) }
    fn point(&mut self, f: usize, t: usize, v: &Value, _: &[Value], _: usize, _: usize) { self.spans.push(format!("{}{}", if t > f { format!("{}=", t - f) } else { String::new() }, v.name.as_ref().map(|n| format!("[{}]", n)).unwrap_or("ø".into()))) }
}
fn spans_test(sets: &[RangeSet<Value>], start: usize, end: usize) -> String { let mut b = Builder { spans: vec![] }; RangeSet::spans(sets, start, end, &mut b, -1); b.spans.join(" ") }

fn report(id: &str, got: String, exp: &str) { println!("{} {} | expected(cm6): {} | ferrum: {}", if got == exp { "PASS" } else { "FAIL" }, id, exp, got); }

fn main() {
    // #01
    { let mut r = small_ranges(); r.push(mk(1000, 4000, Spec { pos: Some(1000), ..Default::default() })); r.push(mk(2000, 3000, Spec { pos: Some(2000), ..Default::default() }));
      let set = mk_set(sorted_by_from(r)).unwrap();
      report("test-rangeset#01", format!("{} {} {} {}", set.size(), set.chunk.len() > 1, set.next_layer.as_ref().map_or(0, |l| l.size()) > 0, check_set(&set)), "5002 true true true"); }
    // #02
    { let a = mk_set(vec![mk(8, 9, Spec::default()), mk(7, 10, Spec::default())]).err().unwrap_or("ok");
      let b = mk_set(vec![mk(1, 1, Spec { start_side: Some(1), ..Default::default() }), mk(1, 1, Spec { start_side: Some(-1), ..Default::default() })]).err().unwrap_or("ok");
      report("test-rangeset#02", format!("{} {}", a.contains("sorted"), b.contains("sorted")), "true true"); }
    // #03
    { let s0 = set0(); let s = s0.update(RangeSetUpdate { add: vec![mk1(4000, Spec { pos: Some(4000), ..Default::default() })], ..Default::default() }).unwrap();
      report("test-rangeset#03", format!("{} {}", s.size(), Arc::ptr_eq(&s.chunk[0], &s0.chunk[0])), "5001 true"); }
    // #04
    { let s0 = set0(); let ranges: Vec<_> = (0..4000).step_by(2).map(|i| mk1(i, Spec::default())).collect(); let s = s0.update(RangeSetUpdate { add: ranges, ..Default::default() }).unwrap();
      report("test-rangeset#04", format!("{} {}", s.size(), check_set(&s)), "7000 true"); }
    // #05
    { let s0 = set0(); let s = s0.update(RangeSetUpdate { filter: Some(Arc::new(|f, _, _| f >= 2500)), ..Default::default() }).unwrap();
      report("test-rangeset#05", format!("{} {} {}", s.size(), s.chunk.len() < s0.chunk.len(), check_set(&s)), "2500 true true"); }
    // #06
    { let s = set0().update(RangeSetUpdate { filter: Some(Arc::new(|f, _, _| (f % 200) >= 100)), ..Default::default() }).unwrap();
      report("test-rangeset#06", format!("{} {}", s.size(), check_set(&s)), "2500 true"); }
    // #07
    { let s = set0().update(RangeSetUpdate { filter: Some(Arc::new(|f, _, _| f == 500 || f == 501)), ..Default::default() }).unwrap();
      report("test-rangeset#07", format!("{} {}", s.size(), s.chunk.len()), "2 1"); }
    // #08
    { let set = mk_set((0..1000).map(|i| mk(i, i + 1, Spec { pos: Some(i), ..Default::default() })).collect()).unwrap();
      let called = Arc::new(Mutex::new(Vec::<String>::new())); let c2 = called.clone();
      let _ = set.update(RangeSetUpdate { filter: Some(Arc::new(move |f, t, _| { c2.lock().unwrap().push(format!("{},{}", f, t)); true })), filter_from: 400, filter_to: 600, ..Default::default() });
      let c = called.lock().unwrap(); let exp: Vec<String> = (399..=600).map(|i| format!("{},{}", i, i + 1)).collect();
      report("test-rangeset#08", format!("{} {}", c.len(), *c == exp), "202 true"); }
    // #09
    { let s = set0().update(RangeSetUpdate { filter: Some(Arc::new(|_, _, _| false)), ..Default::default() }).unwrap();
      report("test-rangeset#09 (identity with RangeSet.empty approximated by is_empty+no chunks)", format!("{} {}", s.is_empty(), s.chunk.is_empty() && s.next_layer.is_none()), "true true"); }
    // map tests
    let map_test = |id: &str, pos: Vec<Range<Value>>, ch: &[i64], exp: &str| { let s = mk_set(pos).unwrap(); report(id, iter_str(&s.map(&desc(ch))), exp) };
    map_test("test-rangeset#10", vec![mk1(1, Spec::default()), mk1(4, Spec::default()), mk1(10, Spec::default())], &[0, 1, 2, -1, 1, 0, 5, -1, 0, 20, 5092, -1], &pts(&[(2, 2), (4, 4), (30, 30)]));
    map_test("test-rangeset#11", vec![mk(1, 2, Spec { start_side: Some(-1), end_side: Some(1), ..Default::default() })], &[1, -1, 0, 2, 1, -1, 0, 2, 5098, -1], &pts(&[(1, 6)]));
    map_test("test-rangeset#12", vec![mk(1, 1, Spec { start_side: Some(-1), end_side: Some(-1), ..Default::default() }), mk(1, 1, Spec { start_side: Some(1), end_side: Some(1), ..Default::default() })], &[1, -1, 0, 2, 5099, -1], &pts(&[(1, 1), (3, 3)]));
    map_test("test-rangeset#13", vec![mk(1, 2, Spec::default())], &[1, -1, 0, 2, 3, -1, 0, 2, 5096, -1], &pts(&[(3, 4)]));
    map_test("test-rangeset#14", vec![mk(1, 2, Spec::default())], &[1, -1, 1, 0, 0, 1, 5098, -1], "");
    map_test("test-rangeset#15", vec![mk(1, 2, Spec::default()), mk1(3, Spec::default())], &[4, 0, 5096, -1], "");
    map_test("test-rangeset#16", vec![mk(2, 4, Spec::default()), mk(2, 8, Spec::default()), mk(6, 8, Spec::default())], &[3, -1, 4, 0, 5093, -1], &pts(&[(2, 3), (2, 4), (3, 4)]));
    map_test("test-rangeset#17", vec![mk1(2, Spec::default()), mk1(4, Spec::default())], &[2, -1, 2, 6, 5096, -1], &pts(&[(2, 2), (8, 8)]));
    // #18
    { let s0 = set0(); let smaller = s0.map(&desc(&[30, -1, 4470, 0, 600, -1])); let empty = smaller.map(&desc(&[1000, 0, 4100, -1]));
      report("test-rangeset#18", format!("{} {}", smaller.chunk.len() < s0.chunk.len(), empty.is_empty()), "true true"); }
    // #19
    { let s = mk_set(vec![mk1(1, Spec { name: Some("a".into()), start_side: Some(1), ..Default::default() }), mk1(2, Spec { name: Some("b".into()), start_side: Some(-1), ..Default::default() })]).unwrap().map(&desc(&[1, -1, 1, 0, 5098, -1]));
      report("test-rangeset#19", s.iter().iter().map(|r| format!("{}:{}", r.from, r.value.name.clone().unwrap())).collect::<Vec<_>>().join(","), "1:b,1:a"); }
    // #20
    { let s = mk_set(vec![mk(0, 1, Spec { name: Some("a".into()), end_side: Some(1), ..Default::default() }), mk(2, 3, Spec { name: Some("b".into()), start_side: Some(-1), ..Default::default() })]).unwrap()
        .map(&desc(&[1, -1, 1, 0, 5098, -1])).map(&desc(&[1, -1, 0, 1, 5099, -1]));
      report("test-rangeset#20", format!("{}", s.next_layer.as_ref().map_or(false, |l| !l.is_empty())), "true"); }
    map_test("test-rangeset#21", vec![mk(0, 1, Spec { start_side: Some(-1), end_side: Some(1), ..Default::default() }), mk(2, 3, Spec { start_side: Some(-1), end_side: Some(1), ..Default::default() }), mk1(4, Spec { start_side: Some(-2), ..Default::default() })], &[4, 0, 5096, -1], &pts(&[(0, 0), (0, 0), (0, 0)]));
    // #22
    { let s = mk_set(vec![mk1(1, Spec { name: Some("a".into()), start_side: Some(0), ..Default::default() }), mk1(1, Spec { name: Some("b".into()), start_side: Some(1), ..Default::default() }), mk1(1, Spec { name: Some("c".into()), start_side: Some(1), ..Default::default() }), mk1(7, Spec { name: Some("d".into()), start_side: Some(0), ..Default::default() })]).unwrap().map(&desc(&[1, -1, 6, 0, 5093, -1]));
      let it = s.iter(); let all_at_1 = it.iter().all(|r| r.from == 1 && r.to == 1);
      report("test-rangeset#22", format!("{} {}", all_at_1, it.iter().map(|r| r.value.name.clone().unwrap()).collect::<Vec<_>>().join(",")), "true a,d,b,c"); }
    map_test("test-rangeset#23", vec![mk1(10, Spec { start_side: Some(0), ..Default::default() }), mk1(10, Spec { start_side: Some(1), ..Default::default() }), mk1(10, Spec { start_side: Some(1), ..Default::default() }), mk1(16, Spec { start_side: Some(0), ..Default::default() })], &[10, -1, 6, 0, 5084, -1], &pts(&[(10, 10), (10, 10), (10, 10), (10, 10)]));
    map_test("test-rangeset#24", vec![mk(0, 10, Spec { start_side: Some(-1), end_side: Some(1), ..Default::default() }), mk(12, 20, Spec { start_side: Some(-1), end_side: Some(1), ..Default::default() }), mk1(25, Spec { start_side: Some(0), ..Default::default() })], &[15, -1, 10, 0, 5075, -1], &pts(&[(0, 10), (12, 15), (15, 15)]));
    // compare tests
    report("test-rangeset#25", cmp_test(mk_set(vec![mk(2, 4, name("a")), mk(8, 11, name("a"))]).unwrap(), EMPTY, None, vec![mk(3, 9, name("b")), mk(106, 107, name("b"))]), "[3, 9, 106, 107]");
    report("test-rangeset#26", cmp_test(mk_set(vec![mk(4, 6, name("a")), mk(5, 7, name("b")), mk(6, 8, name("c")), mk(20, 30, name("d"))]).unwrap(), EMPTY, Some(Arc::new(|f, _, _| f != 5 && f != 20)), vec![]), "[5, 7, 20, 30]");
    report("test-rangeset#27", cmp_test(mk_set(vec![mk(0, 50, name("a"))]).unwrap(), EMPTY, Some(Arc::new(|_, _, _| false)), vec![mk(10, 40, name("a"))]), "[0, 10, 40, 50]");
    report("test-rangeset#28", cmp_test(mk_set(vec![mk(0, 20, name("a"))]).unwrap(), &[5, -1, 10, 20, 5085, -1], Some(Arc::new(|_, _, _| false)), vec![]), "[0, 5, 25, 30]");
    report("test-rangeset#29", cmp_test(mk_set((0..1000).map(|i| mk(i, i + 1, name("a"))).collect()).unwrap(), &[900, -1, 100, 0, 4100, -1], None, vec![mk(850, 860, name("b"))]), "[850, 860]");
    { let mut r = vec![mk(3, 997, Spec { point: true, ..Default::default() })]; for i in (0..1000).step_by(2) { r.push(mk(i, i + 1, name("a"))); }
      report("test-rangeset#30", cmp_test(mk_set(sorted_by_from(r)).unwrap(), &[300, -1, 200, 100, 4600, -1], None, vec![]), "[]"); }
    report("test-rangeset#31", cmp_test(mk_set(vec![mk(3, 50, Spec { point: true, ..Default::default() })]).unwrap(), EMPTY, None, vec![mk(40, 80, Spec { point: true, ..Default::default() })]), "[50, 80]");
    report("test-rangeset#32", cmp_test(mk_set(vec![mk(3, 50, Spec { point: true, ..Default::default() })]).unwrap(), EMPTY, Some(Arc::new(|_, _, _| false)), vec![]), "[3, 50]");
    { let mut r = vec![]; for i in (0..200usize).step_by(2) { let end = i + 1 + (i + 49) / 50; r.push(mk(i, end, name(&format!("{}-{}", i, end)))); }
      report("test-rangeset#33", cmp_test(mk_set(r).unwrap(), &[0, 50, 50, -1, 50, 0, 50, -1, 50, 0, 4900, -1], Some(Arc::new(|f, _, _| f % 50 > 0)), vec![]), "[50, 51, 100, 103, 148, 153]"); }
    report("test-rangeset#34", cmp_test(mk_set(vec![mk(0, 4, Spec { start_side: Some(1), end_side: Some(-1), ..Default::default() }), mk(1, 3, Spec { point: true, start_side: Some(-1), end_side: Some(1), ..Default::default() })]).unwrap(), &[2, -1, 2, 0, 5096, -1], None, vec![]), "[1, 2]");
    // spans tests
    report("test-rangeset#35", spans_test(&[mk_set(vec![mk(3, 8, name("one")), mk(5, 8, name("two")), mk(10, 12, name("three"))]).unwrap()], 0, 15), "3 2=one 3=two/one 2 2=three 3");
    { let mut d = vec![mk(0, 200, name("wide"))]; for i in 0..100 { d.push(mk(i * 2, i * 2 + 2, name(&format!("span{}", i)))); }
      let (start, end) = (20usize, 26usize); let mut exp = String::new(); let mut pos = start;
      while pos < end { let step = if pos % 2 == 1 { 1 } else { 2 }; if !exp.is_empty() { exp.push(' '); } exp.push_str(&format!("{}=span{}/wide", (end.min(pos + step)) - pos, pos / 2)); pos += step; }
      report("test-rangeset#36", spans_test(&[mk_set(d).unwrap()], start, end), &exp); }
    report("test-rangeset#37", spans_test(&[mk_set(vec![mk(2, 3, name("x")), mk(5, 10, name("y")), mk(10, 12, name("z"))]).unwrap(), mk_set(vec![mk(0, 6, name("a")), mk(10, 12, name("b"))]).unwrap()], 0, 12), "2=a 1=x/a 2=a 1=y/a 4=y 2=z/b");
    report("test-rangeset#38", spans_test(&[mk_set(vec![mk(2, 10, name("a")), mk(20, 30, name("a"))]).unwrap(), mk_set(vec![mk(3, 4, name("b")), mk(8, 12, name("b")), mk(18, 22, name("b"))]).unwrap(), mk_set(vec![mk(0, 25, name("c"))]).unwrap()], 0, 25), "2=c 1=a/c 1=a/b/c 4=a/c 2=a/b/c 2=b/c 6=c 2=b/c 2=a/b/c 3=a/c");
    report("test-rangeset#39", spans_test(&[mk_set(vec![mk1(1, name("a")), mk1(1, name("b")), mk1(1, name("c"))]).unwrap()], 0, 2), "1 [a] [b] [c] 1");
    report("test-rangeset#40", spans_test(&[mk_set(vec![mk(0, 20, name("r1")), mk(1, 10, name("r2")), mk(3, 12, Spec { name: Some("p".into()), point: true, ..Default::default() }), mk(4, 8, name("r3")), mk(5, 20, name("r4"))]).unwrap()], 0, 20), "1=r1 2=r2/r1 9=[p] 8=r4/r1");
    { struct C(Vec<usize>); impl SpanIterator<Value> for C { fn span(&mut self, f: usize, t: usize, _: &[Value], _: usize) { self.0.push(f); self.0.push(t) } fn point(&mut self, f: usize, t: usize, _: &Value, _: &[Value], _: usize, _: usize) { self.0.push(f); self.0.push(t) } }
      let mut c = C(vec![]); RangeSet::spans(&[mk_set(vec![mk(0, 10, name("a")), mk(20, 30, Spec { name: Some("b".into()), point: true, ..Default::default() })]).unwrap()], 0, 30, &mut c, 0);
      report("test-rangeset#41", c.0.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","), "0,20,20,30"); }
    { struct C(usize); impl SpanIterator<Value> for C { fn span(&mut self, _: usize, _: usize, _: &[Value], _: usize) {} fn point(&mut self, _: usize, _: usize, _: &Value, _: &[Value], _: usize, _: usize) { self.0 += 1 } }
      let mut c = C(0); RangeSet::spans(&[mk_set(vec![mk(0, 4, Spec { point: true, ..Default::default() }), mk(1, 5, name("a")), mk(2, 4, Spec { point: true, ..Default::default() })]).unwrap()], 0, 10, &mut c, -1);
      report("test-rangeset#42", c.0.to_string(), "1"); }
    report("test-rangeset#43", spans_test(&[mk_set(vec![mk(0, 3, name("x")), mk(0, 1, name("a")), mk(1, 2, name("b")), mk(2, 3, name("c"))]).unwrap()], 0, 3), "1=a/x 1=b/x 1=c/x");
    // #44
    { let mut r = small_ranges(); r.push(mk(1000, 4000, Spec { pos: Some(1000), ..Default::default() })); r.push(mk(2000, 3000, Spec { pos: Some(2000), ..Default::default() }));
      let set = mk_set(sorted_by_from(r)).unwrap(); let it = set.iter(); let mut ok = true;
      for (count, r) in it.iter().enumerate() { let exp = if count > 2001 { count - 2 } else if count > 1000 { count - 1 } else { count }; if r.from != exp { ok = false; } }
      report("test-rangeset#44", format!("{} {}", ok, it.len()), "true 5002"); }
    // #45 (no iter(from) in Ferrum: emulate by skipping ranges with to < 1000, the cm6 semantics)
    { let s = set0(); let mut count = 0; let mut ok = true;
      for r in s.iter().iter().filter(|r| r.to >= 1000) { if r.from > 2000 { break; } if r.to != r.from + 1 + (r.from % 4) { ok = false; } count += 1; }
      report("test-rangeset#45 (emulated iter(1000) via filter to>=1000)", format!("{} {}", ok, count), "true 1003"); }
    // #46
    { let mut found = 0; let mut ok = true; set0().between(100, 200, |f, t, _| { if t != f + 1 + (f % 4) || t < 100 || f > 200 { ok = false; } found += 1; true });
      report("test-rangeset#46", format!("{} {}", ok, found), "true 103"); }
    // #47
    { let s = RangeSet::of(vec![mk(0, 0, Spec::default())]).unwrap(); let mut f = vec![]; s.between(0, 0, |a, b, _| { f.push(a.to_string()); f.push(b.to_string()); true });
      report("test-rangeset#47", f.join(","), "0,0"); }
}
