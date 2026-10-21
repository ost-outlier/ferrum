use ferrum_state::*;
use std::any::Any;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn mk(exts: Vec<Extension>) -> Arc<EditorState> {
    EditorState::create(EditorStateConfig { extensions: Some(exts), ..Default::default() })
}
fn up(st: &Arc<EditorState>, spec: TransactionSpec) -> Arc<EditorState> {
    st.apply(st.update(spec).unwrap())
}
fn run(name: &str, f: impl FnOnce() -> String) {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(s) => println!("{}: {}", name, s),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() { s.clone() } else if let Some(s) = e.downcast_ref::<&str>() { s.to_string() } else { "?".into() };
            println!("{}: PANIC {}", name, msg)
        }
    }
}
fn j(v: &[usize]) -> String { v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",") }

#[derive(Clone)]
struct Add(usize);

struct FakeFacet(FacetId);
impl AnyFacet for FakeFacet {
    fn id(&self) -> FacetId { self.0 }
    fn is_static(&self) -> bool { false }
    fn combine_erased(&self, _inputs: &[Arc<dyn Any + Send + Sync>]) -> Arc<dyn Any + Send + Sync> { Arc::new("fake") }
    fn default_erased(&self) -> Arc<dyn Any + Send + Sync> { Arc::new("fake") }
}
struct FakeField(StateFieldId);
impl AnyStateField for FakeField {
    fn id(&self) -> StateFieldId { self.0 }
    fn init(&self, _s: &EditorState) -> Arc<dyn Any + Send + Sync> { Arc::new(String::from("fake")) }
    fn update(&self, v: &dyn Any, _tr: &Transaction) -> Arc<dyn Any + Send + Sync> { Arc::new(v.downcast_ref::<String>().unwrap().clone()) }
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let num = Facet::<usize>::define_list();
    let strf = Facet::<String>::define_list();

    run("R1 dedup same value", || { let e = num.of(50); j(mk(vec![num.of(1), e.clone(), num.of(4), e]).facet(&num)) });
    run("R2 dedup keeps highest prec", || { let e = num.of(50); j(mk(vec![num.of(1), e.clone(), e.with_prec(Prec::Highest)]).facet(&num)) });
    run("R2b dedup lower prec later", || { let e = num.of(50); j(mk(vec![e.clone().with_prec(Prec::Highest), num.of(1), e]).facet(&num)) });
    run("R2c dedup of group", || { let e = Extension::Group(vec![num.of(7), num.of(8)]); j(mk(vec![e.clone(), num.of(1), e]).facet(&num)) });
    run("R3 nested precedence", || j(mk(vec![num.of(0), Extension::Group(vec![num.of(1), num.of(2).with_prec(Prec::Lowest), num.of(3)]).with_prec(Prec::High), num.of(4).with_prec(Prec::Low).with_prec(Prec::Highest)]).facet(&num)));
    run("R4 static compute", || { let s = Facet::<usize>::define_static(|v| v.to_vec()); let _ = s.compute(|_| 1); "no panic".into() });
    run("R4b static compute_multi", || { let s = Facet::<usize>::define_static(|v| v.to_vec()); let _ = s.compute_multi(|_| vec![1]); "no panic".into() });
    run("R5 computed reads computed", || { let s2 = strf.clone(); j(mk(vec![num.compute(move |s| s.facet(&s2).join("").len()), strf.compute(|_| "ab".to_string())]).facet(&num)) });
    run("R5b computed reads computed (reverse)", || { let s2 = strf.clone(); j(mk(vec![strf.compute(|_| "ab".to_string()), num.compute(move |s| s.facet(&s2).join("").len())]).facet(&num)) });
    run("R6 field init reads computed facet", || { let n2 = num.clone(); let fld = StateField::define(move |s| j(s.facet(&n2)), |v: &String, _| v.clone()); mk(vec![fld.extension(), num.compute(|_| 7)]).field(&fld).cloned().unwrap() });
    run("R6b field init reads static facet", || { let n2 = num.clone(); let fld = StateField::define(move |s| j(s.facet(&n2)), |v: &String, _| v.clone()); mk(vec![fld.extension(), num.of(7)]).field(&fld).cloned().unwrap() });
    run("R7 field init reads later field", || { let f1 = StateField::define(|_| 5usize, |v: &usize, _| *v); let f1c = f1.clone(); let f2 = StateField::define(move |s| format!("{:?}", s.field(&f1c)), |v: &String, _| v.clone()); mk(vec![f2.extension(), f1.extension()]).field(&f2).cloned().unwrap() });
    run("R7b field init reads earlier field", || { let f1 = StateField::define(|_| 5usize, |v: &usize, _| *v); let f1c = f1.clone(); let f2 = StateField::define(move |s| format!("{:?}", s.field(&f1c)), |v: &String, _| v.clone()); mk(vec![f1.extension(), f2.extension()]).field(&f2).cloned().unwrap() });
    run("R8 computed reads field", || { let fld = StateField::define(|_| 3usize, |v: &usize, _| v + 1); let fc = fld.clone(); let st = mk(vec![fld.extension(), num.compute(move |s| *s.field(&fc).unwrap())]); let st2 = up(&st, TransactionSpec::new()); format!("{} -> {}", j(st.facet(&num)), j(st2.facet(&num))) });
    run("R9 recompute count (doc-like)", || { let c = Arc::new(AtomicUsize::new(0)); let c2 = c.clone(); let st = mk(vec![num.compute(move |_| c2.fetch_add(1, Ordering::SeqCst))]); let mut r = vec![j(st.facet(&num))]; let st = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "hello"))); r.push(j(st.facet(&num))); let st = up(&st, TransactionSpec::new()); r.push(j(st.facet(&num))); let st = up(&st, TransactionSpec::new().with_selection(EditorSelection::cursor(1))); r.push(j(st.facet(&num))); format!("{} calls={}", r.join(" | "), c.load(Ordering::SeqCst)) });
    run("R9b recompute count 3 doc changes", || { let c = Arc::new(AtomicUsize::new(0)); let c2 = c.clone(); let mut st = mk(vec![num.compute(move |_| c2.fetch_add(1, Ordering::SeqCst))]); for _ in 0..3 { st = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "x"))); } format!("{} calls={}", j(st.facet(&num)), c.load(Ordering::SeqCst)) });
    run("R10 identity empty tr (dynamic)", || { let s2 = strf.clone(); let st = mk(vec![num.of(1), num.compute(move |s| s.facet(&s2).join("").len()), strf.of("hello".into())]); let st2 = up(&st, TransactionSpec::new()); format!("{} ptr_eq={}", j(st.facet(&num)), std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R10b identity doc change (dynamic)", || { let s2 = strf.clone(); let st = mk(vec![num.of(1), num.compute(move |s| s.facet(&s2).join("").len()), strf.of("hello".into())]); let st2 = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "x"))); format!("ptr_eq={}", std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R10c identity dynamic equal value", || { let st = mk(vec![num.compute(|_| 1)]); let st2 = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "x"))); format!("ptr_eq={}", std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R10d identity static across tr", || { let st = mk(vec![num.of(1)]); let st2 = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "x"))); format!("ptr_eq={}", std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R11 compartment reconf: unrelated static identity", || { let c = Compartment::new(); let st = mk(vec![num.of(1), num.of(2), c.of(strf.of("a".into()))]); let st2 = up(&st, TransactionSpec::new().with_effect(c.reconfigure(strf.of("b".into())))); format!("{} ptr_eq={}", j(st2.facet(&num)), std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R11b compartment reconf: dynamic identity", || { let c = Compartment::new(); let st = mk(vec![num.compute(|s| s.doc().len_chars()), c.of(strf.of("a".into()))]); let st2 = up(&st, TransactionSpec::new().with_effect(c.reconfigure(strf.of("b".into())))); format!("ptr_eq={}", std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R12 combine count", || { let calls = Arc::new(AtomicUsize::new(0)); let c2 = calls.clone(); let f = Facet::<usize, usize>::define(move |v| { c2.fetch_add(1, Ordering::SeqCst); v.len() }); let c = Compartment::new(); let st = mk(vec![f.of(1), c.of(strf.of("a".into()))]); let before = calls.load(Ordering::SeqCst); let _ = up(&st, TransactionSpec::new().with_effect(c.reconfigure(strf.of("b".into())))); format!("atDefine/create={} afterReconf={}", before, calls.load(Ordering::SeqCst)) });
    run("R12b combine count, empty tr", || { let calls = Arc::new(AtomicUsize::new(0)); let c2 = calls.clone(); let f = Facet::<usize, usize>::define(move |v| { c2.fetch_add(1, Ordering::SeqCst); v.len() }); let st = mk(vec![f.of(1), f.compute(|_| 2)]); let before = calls.load(Ordering::SeqCst); let _ = up(&st, TransactionSpec::new()); format!("define+create={} afterEmptyTr={}", before, calls.load(Ordering::SeqCst)) });
    run("R13 dup compartment", || { let c = Compartment::new(); j(mk(vec![c.of(num.of(1)), c.of(num.of(2))]).facet(&num)) });
    run("R13c same compartment instance twice", || { let c = Compartment::new(); let inst = c.of(num.of(1)); j(mk(vec![inst.clone(), inst]).facet(&num)) });
    run("R14 forgets dropped compartment", || { let outer = Compartment::new(); let inner = Compartment::new(); let st = mk(vec![outer.of(Extension::Group(vec![inner.of(num.of(10)), num.of(20)]))]); let st = up(&st, TransactionSpec::new().with_effect(inner.reconfigure(num.of(0)))); let a = j(st.facet(&num)); let st = up(&st, TransactionSpec::new().with_effect(outer.reconfigure(num.of(2)))); let b = j(st.facet(&num)); let st = up(&st, TransactionSpec::new().with_effect(outer.reconfigure(Extension::Group(vec![inner.of(num.of(10)), num.of(2)])))); format!("{} | {} | {}", a, b, j(st.facet(&num))) });
    run("R14b reconf absent compartment then add", || { let outer = Compartment::new(); let inner = Compartment::new(); let st = mk(vec![outer.of(num.of(1))]); let st = up(&st, TransactionSpec::new().with_effect(inner.reconfigure(num.of(5)))); let a = j(st.facet(&num)); let st = up(&st, TransactionSpec::new().with_effect(outer.reconfigure(inner.of(num.of(9))))); format!("{} | {}", a, j(st.facet(&num))) });
    run("R16 field preserved across compartment reconf", || { let c = Compartment::new(); let fld = StateField::define(|_| 0usize, |v: &usize, _| v + 1); let st = up(&mk(vec![fld.extension(), c.of(num.of(1))]), TransactionSpec::new()); let a = *st.field(&fld).unwrap(); let st = up(&st, TransactionSpec::new().with_effect(c.reconfigure(num.of(2)))); format!("{} -> {:?}", a, st.field(&fld)) });
    run("R16b field in compartment removed", || { let c = Compartment::new(); let fld = StateField::define(|_| 0usize, |v: &usize, _| v + 1); let st = mk(vec![c.of(fld.extension())]); let st = up(&st, TransactionSpec::new().with_effect(c.reconfigure(Extension::Group(vec![])))); format!("{:?}", st.field(&fld)) });
    run("R17 new field via compartment reads new-config facet", || { let c = Compartment::new(); let n2 = num.clone(); let fld = StateField::define(move |s| s.facet(&n2).first().copied().unwrap_or(0), |v: &usize, tr| tr.effects().iter().fold(*v, |acc, e| acc + e.downcast_ref::<Add>().map(|a| a.0).unwrap_or(0))); let st = mk(vec![c.of(Extension::Group(vec![]))]); let st = up(&st, TransactionSpec::new().with_effect(c.reconfigure(Extension::Group(vec![fld.extension(), num.of(10)]))).with_effect(StateEffect::of(Add(5)))); format!("{:?}", st.field(&fld)) });
    run("R17b new field create sees doc", || { let c = Compartment::new(); let fld = StateField::define(|s| s.doc().to_string(), |v: &String, _| v.clone()); let st = mk(vec![c.of(Extension::Group(vec![]))]); let st = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "new")).with_effect(c.reconfigure(fld.extension()))); format!("{:?}", st.field(&fld)) });
    run("R18 events new field", || { let ev = Arc::new(Mutex::new(Vec::<String>::new())); let (e1, e2) = (ev.clone(), ev.clone()); let c = Compartment::new(); let fld = StateField::define(move |_| { e1.lock().unwrap().push("create".into()); 0usize }, move |v: &usize, _| { e2.lock().unwrap().push(format!("update {}", v)); v + 1 }); let st = mk(vec![c.of(Extension::Group(vec![]))]); let st = up(&st, TransactionSpec::new().with_effect(c.reconfigure(fld.extension()))); let x = format!("{} => {:?}", ev.lock().unwrap().join(", "), st.field(&fld)); x });
    run("R19 cyclic facets", || { let s2 = strf.clone(); let n2 = num.clone(); j(mk(vec![num.compute(move |s| s.facet(&s2).len()), strf.compute(move |s| j(s.facet(&n2)))]).facet(&num)) });
    run("R19b field reads itself", || { let holder: Arc<Mutex<Option<StateField<usize>>>> = Arc::new(Mutex::new(None)); let h2 = holder.clone(); let fld = StateField::define(move |s| { let g = h2.lock().unwrap(); g.as_ref().and_then(|f| s.field(f).copied()).map(|x| x + 1).unwrap_or(0) }, |v: &usize, _| *v); *holder.lock().unwrap() = Some(fld.clone()); format!("{:?}", mk(vec![fld.extension()]).field(&fld)) });
    run("R20 compute_multi after doc change", || { let st = mk(vec![num.compute_multi(|s| if s.doc().len_chars() % 2 == 1 { vec![100, 10] } else { vec![] }), num.of(1)]); let a = j(st.facet(&num)); let st = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "hello"))); format!("{} | {}", a, j(st.facet(&num))) });
    run("R22 combine called at define", || { let calls = Arc::new(AtomicUsize::new(0)); let c2 = calls.clone(); let _f = Facet::<usize, usize>::define(move |v| { c2.fetch_add(1, Ordering::SeqCst); v.len() }); format!("{}", calls.load(Ordering::SeqCst)) });
    run("R28 slot order side effects", || { let log = Arc::new(Mutex::new(Vec::<&str>::new())); let a = Facet::<usize>::define_list(); let b = Facet::<usize>::define_list(); let (l1, l2) = (log.clone(), log.clone()); let _ = mk(vec![b.compute(move |_| { l1.lock().unwrap().push("b"); 1 }), a.compute(move |_| { l2.lock().unwrap().push("a"); 1 })]); let x = log.lock().unwrap().join(","); x });
    run("R29 prec dynamic order", || j(mk(vec![num.of(1), num.compute(|_| 2).with_prec(Prec::Highest)]).facet(&num)));
    run("R39 test-facet#09 scenario", || { let s2 = strf.clone(); let b = Facet::<bool>::define_list(); let b2 = b.clone(); let st = mk(vec![num.compute(move |s| s.facet(&s2).join("").len()), strf.compute(move |s| format!("{:?}", s.facet(&b2)))]); j(up(&st, TransactionSpec::new()).facet(&num)) });
    run("R40 default identity absent facet", || { let a = mk(vec![]); let b = mk(vec![]); format!("ptr_eq={}", std::ptr::eq(a.facet(&num), b.facet(&num))) });
    run("R41 field dedup", || { let n = Arc::new(AtomicUsize::new(0)); let n2 = n.clone(); let fld = StateField::define(move |_| n2.fetch_add(1, Ordering::SeqCst) + 1, |v: &usize, _| *v); let st = mk(vec![fld.extension(), fld.extension(), fld.extension().with_prec(Prec::Highest)]); format!("{:?} creates={}", st.field(&fld), n.load(Ordering::SeqCst)) });
    run("R42 reconfigure same compartment twice", || { let c = Compartment::new(); let st = mk(vec![c.of(num.of(1))]); j(up(&st, TransactionSpec::new().with_effect(c.reconfigure(num.of(2))).with_effect(c.reconfigure(num.of(3)))).facet(&num)) });
    run("R44 reconfigure new dynamic provider", || { let c = Compartment::new(); let st = mk(vec![c.of(num.compute(|_| 1))]); j(up(&st, TransactionSpec::new().with_effect(c.reconfigure(num.compute(|_| 2)))).facet(&num)) });
    run("R45 FacetStatic with mismatched value type", || { let ext = Extension::FacetStatic(Arc::new(num.clone()), Arc::new("oops")); j(mk(vec![ext]).facet(&num)) });
    run("R45b FacetDynamic with mismatched value type", || { let ext = Extension::FacetDynamic(Arc::new(num.clone()), Arc::new(|_s: &EditorState| Arc::new("oops") as Arc<dyn Any + Send + Sync>)); j(mk(vec![ext]).facet(&num)) });
    run("R46 custom AnyFacet colliding id", || { let ext = Extension::FacetStatic(Arc::new(FakeFacet(num.id())), Arc::new(1usize)); j(mk(vec![ext]).facet(&num)) });
    run("R46b custom AnyFacet colliding id, is_static flag ignored", || { let fs = Facet::<usize>::define_static(|v| v.to_vec()); let ext = Extension::FacetDynamic(Arc::new(fs.clone()), Arc::new(|_s: &EditorState| Arc::new(9usize) as Arc<dyn Any + Send + Sync>)); j(mk(vec![ext]).facet(&fs)) });
    run("R47 Configuration public, dynamic facet read with None", || { let cfg = Configuration::resolve(&[num.compute(|_| 5)], None); format!("{:?}", cfg.facet(&num, None)) });
    run("R47b Configuration::facet with empty dynamic slice", || { let cfg = Configuration::resolve(&[num.compute(|_| 5)], None); format!("{:?}", cfg.facet(&num, Some(&[]))) });
    run("R48 Configuration::field with empty slice", || { let fld = StateField::define(|_| 1usize, |v: &usize, _| *v); let cfg = Configuration::resolve(&[fld.extension()], None); format!("{:?}", cfg.field(&fld, &[])) });
    run("R50 custom AnyStateField colliding id", || { let real = StateField::define(|_| 1usize, |v: &usize, _| *v); let ext = Extension::StateField(Arc::new(FakeField(real.id()))); format!("{:?}", mk(vec![ext]).field(&real)) });
    run("R50b AnyStateField::update with wrong type", || { let real = StateField::define(|_| 1usize, |v: &usize, _| *v); let st = mk(vec![]); let tr = st.update(TransactionSpec::new()).unwrap(); let _ = AnyStateField::update(&real, &"x", &tr); "no panic".into() });
    run("R51 ids", || { let a = Facet::<usize>::define_list(); let b = Facet::<usize>::define_list(); let ts = EditorState::tab_size().id(); let c = Facet::<usize>::define_list(); format!("num={:?} a={:?} b={:?} tab_size(lazy)={:?} c={:?} field={:?} comp={:?}", num.id(), a.id(), b.id(), ts, c.id(), StateField::define(|_| 0usize, |v: &usize, _| *v).id(), Compartment::new().id) });
    run("R51b ids across threads", || { let hs: Vec<_> = (0..4).map(|_| std::thread::spawn(|| (0..3).map(|_| Facet::<usize>::define_list().id().clone()).collect::<Vec<_>>())).collect(); let r: Vec<_> = hs.into_iter().map(|h| h.join().unwrap()).collect(); format!("{:?}", r) });
    run("R52 Facet clone shares id", || { let n2 = num.clone(); j(mk(vec![n2.of(3), num.of(4)]).facet(&num)) });
    run("R53 Extension::from(Vec)", || { let e: Extension = vec![num.of(1), num.of(2)].into(); j(mk(vec![e]).facet(&num)) });
    run("R54 combined static facet (define, not define_static)", || { let f = Facet::<usize, usize>::define(|ns| ns.iter().sum()); format!("{}", mk(vec![f.of(1), f.of(2), f.of(3)]).facet(&f)) });
    run("R55 dynamic combined after doc change", || { let f = Facet::<usize, usize>::define(|ns| ns.iter().sum()); let st = mk(vec![f.of(1), f.compute(|s| s.doc().len_chars()), f.of(3)]); let st2 = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "hello"))); format!("{} -> {}", st.facet(&f), st2.facet(&f)) });
    run("R56 selection-like recompute count", || { let c = Arc::new(AtomicUsize::new(0)); let c2 = c.clone(); let st = mk(vec![num.compute(move |_| c2.fetch_add(1, Ordering::SeqCst))]); let st = up(&st, TransactionSpec::new().with_change(ChangeSpec::insert(0, "hello"))); let st = up(&st, TransactionSpec::new().with_selection(EditorSelection::cursor(2))); let st = up(&st, TransactionSpec::new()); format!("{} calls={}", j(st.facet(&num)), c.load(Ordering::SeqCst)) });
    run("R57 static facet value with compartment reconf same providers", || { let c = Compartment::new(); let inner = num.of(1); let st = mk(vec![c.of(inner.clone())]); let st2 = up(&st, TransactionSpec::new().with_effect(c.reconfigure(inner))); format!("{} ptr_eq={}", j(st2.facet(&num)), std::ptr::eq(st.facet(&num), st2.facet(&num))) });
    run("R58 field identity after update returning clone", || { let fld = StateField::define(|_| vec![1usize], |v: &Vec<usize>, _| v.clone()); let st = mk(vec![fld.extension()]); let st2 = up(&st, TransactionSpec::new()); format!("ptr_eq={}", std::ptr::eq(st.field(&fld).unwrap(), st2.field(&fld).unwrap())) });
}
