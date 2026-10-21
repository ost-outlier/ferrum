#![allow(unused)]
use ferrum_state::*;
use ferrum_state::transaction::*;
use std::sync::Arc;

fn send_sync<T: Send + Sync>() {}

#[derive(Clone, PartialEq, Debug)]
struct Side { s: i32, e: i32, p: bool }
impl RangeValue for Side {
    fn start_side(&self) -> i32 { self.s }
    fn end_side(&self) -> i32 { self.e }
    fn point(&self) -> bool { self.p }
}

fn main() {
    // P1 line breaks / D2
    for s in ["a\u{2028}b", "a\u{85}b", "a\u{0b}b", "a\u{0c}b", "a\rb", "a\r\nb", "a\nb"] {
        let t = Text::from_str(s);
        let l1 = t.line(1);
        println!("P1 {:?}: len={} lines={} line1.text={:?} line1.to={}", s, t.len(), t.len_lines(), l1.text, l1.to);
    }
    // P2 position unit
    let t = Text::from_str("😀a");
    println!("P2 len(\"😀a\")={} line_at(1).from={}", t.len(), t.line_at(1).from);
    // P3 line(0)
    let r = std::panic::catch_unwind(|| Text::from_str("a\nb").line(0));
    println!("P3 line(0) panics={}", r.is_err());
    println!("P3b line(1).text={:?}", Text::from_str("a\nb").line(1).text);
    // P4 find_cluster_break
    let cases: Vec<(&str, usize)> = vec![
        ("\r\n", 0), ("e\u{301}x", 0), ("👨‍👩‍👧‍👦x", 0), ("🇩🇪🇫🇷", 0), ("👍🏽x", 0),
        ("\u{1100}\u{1161}\u{11a8}x", 0), ("क्षa", 0), ("\u{600}a", 0), ("a\u{200d}b", 0),
    ];
    for (s, p) in &cases {
        println!("P4 fcb({:?},{}, fwd)={} chars={}", s, p, find_cluster_break(s, *p, true), s.chars().count());
    }
    let n = "👨‍👩‍👧‍👦".chars().count();
    println!("P4b fcb(family, end, back)={}", find_cluster_break("👨‍👩‍👧‍👦", n, false));
    // P5 categorize
    for c in ['\u{feff}', '\u{a0}', '\u{200b}', '\u{1680}', '\u{180e}', '_', 'é', '٣', '\u{301}', '$', '中'] {
        println!("P5 categorize({:?})={:?}", c, categorize_char(c));
    }
    // P6 subword with combining mark
    let s = "ae\u{301}b";
    println!("P6 find_subword_break({:?},0,fwd)={} (cluster ends at {})", s, find_subword_break(s, 0, true), find_cluster_break(s, 1, true));
    println!("P6b find_word_break({:?},0,fwd)={}", "ae\u{301} b", find_word_break("ae\u{301} b", 0, true));
    println!("P6c find_subword_break(\"fooBar\",0)={} (\"foo_bar\",0)={}", find_subword_break("fooBar", 0, true), find_subword_break("foo_bar", 0, true));
    // P7 columns
    println!("P7 count_column(\"😀\\tx\",4,None)={} find_column(\"😀a\",1,4,false)={:?} find_column(\"ab\",5,4,true)={:?}",
        count_column("😀\tx", 4, None), find_column("😀a", 1, 4, false), find_column("ab", 5, 4, true));
    let r = std::panic::catch_unwind(|| count_column("\tx", 0, None));
    println!("P7b count_column tab_size 0 panics={}", r.is_err());
    // P8 selection create empty
    let s = EditorSelection::create(vec![], 0);
    println!("P8 create([]) -> {:?}", s.ranges().iter().map(|r| (r.anchor, r.head)).collect::<Vec<_>>());
    let s = EditorSelection::create(vec![SelectionRange::new(6,7), SelectionRange::new(7,8), SelectionRange::cursor(3), SelectionRange::cursor(3), SelectionRange::new(10, 12), SelectionRange::new(13, 11)], 5);
    println!("P8b normalize -> {:?} main={}", s.ranges().iter().map(|r| (r.anchor, r.head)).collect::<Vec<_>>(), s.main_index());
    let s = EditorSelection::create(vec![SelectionRange::cursor(1)], 7);
    println!("P8c create([cursor 1], main 7) main_index={}", s.main_index());
    // P9 readOnly combine
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![EditorState::read_only().of(false), EditorState::read_only().of(true)]), ..Default::default() });
    println!("P9 readOnly [false,true] -> {}", st.is_read_only());
    // P10 allowMultipleSelections not enforced
    let st = EditorState::create(EditorStateConfig { doc: Some("abcdef".into()), selection: Some(EditorSelection::create(vec![SelectionRange::cursor(1), SelectionRange::cursor(4)], 0)), extensions: None });
    println!("P10 ranges without allowMultipleSelections: {}", st.selection().ranges().len());
    // P11 lineSeparator ignored
    let st = EditorState::create(EditorStateConfig { doc: Some("a|b".into()), selection: None, extensions: Some(vec![EditorState::line_separator().of("|".to_string())]) });
    println!("P11 lineSeparator \"|\" doc lines={}", st.doc().len_lines());
    // P12 duplicate extension instance
    let f = Facet::<usize>::define_list();
    let e = f.of(1);
    let st = EditorState::create(EditorStateConfig { extensions: Some(vec![e.clone(), e]), ..Default::default() });
    println!("P12 same extension twice -> {:?}", st.facet(&f));
    // P13 dynamic facet depending on another dynamic facet
    let a = Facet::<usize>::define_list();
    let b = Facet::<usize>::define_list();
    let b2 = b.clone();
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let st = EditorState::create(EditorStateConfig { extensions: Some(vec![a.compute(move |s| s.facet(&b2).len()), b.compute(|_| 5)]), ..Default::default() });
        format!("{:?}", st.facet(&a))
    }));
    println!("P13 dynamic facet reading another dynamic facet -> {:?}", r.map_err(|e| e.downcast_ref::<String>().cloned().or(e.downcast_ref::<&str>().map(|s| s.to_string()))));
    // P14 annotation order
    let st = EditorState::create(EditorStateConfig::default());
    let tr = st.update(TransactionSpec::new().with_annotation(UserEvent("a".into())).with_annotation(UserEvent("b".into()))).unwrap();
    println!("P14 two UserEvent annotations -> {:?}", tr.annotation::<UserEvent>());
    // P15 Transaction::state identity
    let st = EditorState::create(EditorStateConfig::default());
    let tr = st.update(TransactionSpec::new()).unwrap();
    println!("P15 Arc::ptr_eq(tr.state(), tr.state()) = {}", Arc::ptr_eq(&tr.state(), &tr.state()));
    // P16 RangeSet between with large side
    let set = RangeSet::of(vec![Range::new(5, 6, Side { s: 2_000_000, e: 0, p: false })]).unwrap();
    let mut hits = vec![];
    set.between(0, 5, |f, t, _| { hits.push((f, t)); true });
    println!("P16 between(0,5) on range 5..6 startSide 2e6 -> {:?}", hits);
    // P17 RangeSet map with side -256 (as i8 truncation)
    let set = RangeSet::of(vec![Range::new(2, 2, Side { s: -256, e: -256, p: true })]).unwrap();
    let cs = ChangeSet::of_spec(ChangeSpec::insert(2, "xx"), 4);
    let m = set.map(cs.desc());
    println!("P17 point at 2 side -256 mapped over insert at 2 -> {:?}", m.iter().iter().map(|r| (r.from, r.to)).collect::<Vec<_>>());
    let set = RangeSet::of(vec![Range::new(2, 2, Side { s: -1, e: -1, p: true })]).unwrap();
    let m = set.map(cs.desc());
    println!("P17b same with side -1 -> {:?}", m.iter().iter().map(|r| (r.from, r.to)).collect::<Vec<_>>());
    // P18 RangeSetBuilder add unsorted
    let mut b = RangeSetBuilder::new();
    b.add(5, 6, Side { s: 0, e: 0, p: false });
    b.add(1, 2, Side { s: 0, e: 0, p: false });
    let rs = b.finish();
    println!("P18 builder.add unsorted -> size {} (no error)", rs.size());
    // P19 Send/Sync
    send_sync::<EditorState>();
    send_sync::<Text>();
    println!("P19 EditorState: Send + Sync (compiles)");
    // P20 OT confluence & inversion property (xorshift64, seed 0x9E3779B97F4A7C15)
    let mut x: u64 = 0x9E3779B97F4A7C15;
    let mut rnd = |n: u64| { x ^= x << 13; x ^= x >> 7; x ^= x << 17; x % n };
    let mut conf_fail = 0; let mut inv_fail = 0; let mut iters = 0;
    for _ in 0..20000 {
        let len = rnd(20) as usize;
        let doc: String = (0..len).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let doc = Text::from_str(&doc);
        let mut mk = |rnd: &mut dyn FnMut(u64) -> u64| {
            let n = rnd(4) as usize; let mut specs = vec![];
            for _ in 0..n { let a = rnd(len as u64 + 1) as usize; let b = (a + rnd(4) as usize).min(len); let ins: String = (0..rnd(3)).map(|_| 'X').collect(); specs.push(ChangeSpec::replace(a, b, ins.as_str())); }
            ChangeSet::of_specs(&specs, len)
        };
        let a = mk(&mut rnd); let b = mk(&mut rnd);
        let left = a.compose(&b.map(a.desc(), false)).apply(&doc);
        let right = b.compose(&a.map(b.desc(), true)).apply(&doc);
        // deletions only commute in content; compare docs
        if left.to_string() != right.to_string() { conf_fail += 1; }
        let applied = a.apply(&doc);
        if a.invert(&doc).apply(&applied).to_string() != doc.to_string() { inv_fail += 1; }
        iters += 1;
    }
    println!("P20 confluence failures {}/{} ; inversion failures {}/{}", conf_fail, iters, inv_fail, iters);
}
