use ferrum_state::*;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn r<T: std::fmt::Debug>(id: &str, f: impl FnOnce() -> T) {
    let res = catch_unwind(AssertUnwindSafe(f));
    match res {
        Ok(v) => println!("{} => {:?}", id, v),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "?".into()
            };
            println!("{} => PANIC: {}", id, msg)
        }
    }
}

fn sel(s: &EditorSelection) -> String {
    let v: Vec<String> = s.ranges().iter().map(|r| format!("{}:{}", r.anchor, r.head)).collect();
    format!("{} main={}", v.join(","), s.main_index())
}

fn st(doc: &str) -> Arc<EditorState> {
    EditorState::create(EditorStateConfig { doc: Some(Text::from(doc)), ..Default::default() })
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    r("R1 create cursor beyond doc", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abc")),
            selection: Some(EditorSelection::cursor(10)),
            ..Default::default()
        });
        sel(s.selection())
    });
    r("R1x create cursor beyond doc then line_at(head)", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abc")),
            selection: Some(EditorSelection::cursor(10)),
            ..Default::default()
        });
        s.doc().line_at(s.selection().main().head).number
    });
    r("R2 create 2 cursors no allowMulti", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("123")),
            selection: Some(EditorSelection::create(vec![SelectionRange::cursor(0), SelectionRange::cursor(1)], 1)),
            ..Default::default()
        });
        sel(s.selection())
    });
    r("R3 update 2 cursors no allowMulti", || {
        let s = st("123");
        let tr = s
            .update(TransactionSpec::new().with_selection(EditorSelection::create(
                vec![SelectionRange::cursor(0), SelectionRange::cursor(1)],
                0,
            )))
            .unwrap();
        sel(tr.state().selection())
    });
    r("R4 tr.state identity + field update count", || {
        static N: AtomicUsize = AtomicUsize::new(0);
        let f = StateField::<usize>::define(|_| 0, |v, _| {
            N.fetch_add(1, Ordering::SeqCst);
            v + 1
        });
        let s = EditorState::create(EditorStateConfig { extensions: Some(vec![f.extension()]), ..Default::default() });
        let tr = s.update(TransactionSpec::new()).unwrap();
        let a = tr.state();
        let b = tr.state();
        (Arc::ptr_eq(&a, &b), N.load(Ordering::SeqCst), *b.field(&f).unwrap())
    });
    r("R5 tr.selection when doc changed, no selection", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abc")),
            selection: Some(EditorSelection::cursor(1)),
            ..Default::default()
        });
        let tr = s.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "x"))).unwrap();
        tr.selection().map(sel)
    });
    r("R6 tr.selection no change no sel", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abc")),
            selection: Some(EditorSelection::cursor(1)),
            ..Default::default()
        });
        let tr = s.update(TransactionSpec::new()).unwrap();
        tr.selection().map(sel)
    });
    r("R7 annotation with two same-type annotations", || {
        let tr = st("").update(TransactionSpec::new().with_annotation(1usize).with_annotation(2usize)).unwrap();
        tr.annotation::<usize>().copied()
    });
    r("R7b user time kept", || {
        let tr = st("").update(TransactionSpec::new().with_annotation(TransactionTime(42))).unwrap();
        tr.annotation::<TransactionTime>().copied()
    });
    r("R8 auto time present", || {
        let tr = st("").update(TransactionSpec::new()).unwrap();
        tr.annotation::<TransactionTime>().map(|t| t.0 > 1_600_000_000_000)
    });
    r("R9 two 'effect types' with same value type usize", || {
        let e = StateEffect::of(1usize);
        // there is no way to define a second effect type for usize: is::<usize>() matches every usize effect
        (e.is::<usize>(), e.is::<u32>())
    });
    r("R9b two 'annotation types' with same value type", || {
        let tr = st("").update(TransactionSpec::new().with_annotation(5usize)).unwrap();
        // any consumer reading usize annotations sees it
        tr.annotation::<usize>().copied()
    });
    r("R9c UserEvent exact only (no isUserEvent)", || {
        let tr = st("").update(TransactionSpec::new().with_annotation(UserEvent("select.pointer".into()))).unwrap();
        (tr.annotation::<UserEvent>().map(|u| u.0.clone()), tr.annotation::<UserEvent>().map(|u| u.0 == "select"))
    });
    r("R10 transaction filter returning false", || {
        let f: TransactionFilterFn = Arc::new(|tr| tr.changes().new_length() <= tr.changes().length());
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("one two")),
            extensions: Some(vec![TRANSACTION_FILTER.of(f)]),
            ..Default::default()
        });
        let tr1 = s.update(TransactionSpec::new().with_change(ChangeSpec::insert(3, " three")).with_selection(EditorSelection::cursor(13)));
        let tr2 = s.update(TransactionSpec::new().with_change(ChangeSpec::replace(4, 7, "2")));
        (tr1.is_none(), tr2.map(|t| t.state().doc().to_string()))
    });
    r("R10b filter order", || {
        static LOG: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());
        let a: TransactionFilterFn = Arc::new(|_| { LOG.lock().unwrap().push('A'); true });
        let b: TransactionFilterFn = Arc::new(|_| { LOG.lock().unwrap().push('B'); true });
        let s = EditorState::create(EditorStateConfig { extensions: Some(vec![TRANSACTION_FILTER.of(a), TRANSACTION_FILTER.of(b)]), ..Default::default() });
        let _ = s.update(TransactionSpec::new());
        LOG.lock().unwrap().clone()
    });
    r("R10c filter reads tr.state() -> state built (fields run)", || {
        static N: AtomicUsize = AtomicUsize::new(0);
        let fld = StateField::<usize>::define(|_| 0, |v, _| { N.fetch_add(1, Ordering::SeqCst); v + 1 });
        let flt: TransactionFilterFn = Arc::new(|tr| { let _ = tr.state(); true });
        let s = EditorState::create(EditorStateConfig { extensions: Some(vec![fld.extension(), TRANSACTION_FILTER.of(flt)]), ..Default::default() });
        let tr = s.update(TransactionSpec::new()).unwrap();
        let _ = s.apply(tr);
        N.load(Ordering::SeqCst)
    });
    r("R15 tabSize default and of(2)", || {
        let d = EditorState::create(EditorStateConfig::default());
        let two = EditorState::create(EditorStateConfig { extensions: Some(vec![EditorState::tab_size().of(2)]), ..Default::default() });
        (*d.facet(EditorState::tab_size()), *two.facet(EditorState::tab_size()))
    });
    r("R16 replaceSelection CRLF text", || {
        let s = st("xy");
        let n = s.apply(s.update(s.replace_selection("a\r\nb")).unwrap());
        (n.doc().to_string(), n.doc().len_chars(), sel(n.selection()), n.selection().main().assoc)
    });
    r("R16b replaceSelection with lineSeparator \\r\\n", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("xy")),
            extensions: Some(vec![EditorState::line_separator().of("\r\n".to_string())]),
            ..Default::default()
        });
        let n = s.apply(s.update(s.replace_selection("a\r\nb\nc")).unwrap());
        (n.doc().to_string(), n.doc().len_chars(), n.doc().len_lines(), sel(n.selection()))
    });
    r("R16c replaceSelection ranges", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abcdef")),
            selection: Some(EditorSelection::create(vec![SelectionRange::new(0, 2), SelectionRange::new(4, 3)], 1)),
            extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
        });
        let n = s.apply(s.update(s.replace_selection("XYZ")).unwrap());
        (n.doc().to_string(), sel(n.selection()))
    });
    r("R16d replaceSelection emoji", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("ab")),
            selection: Some(EditorSelection::cursor(1)),
            ..Default::default()
        });
        let n = s.apply(s.update(s.replace_selection("😀")).unwrap());
        (n.doc().to_string(), sel(n.selection()))
    });
    r("R17 changeByRange effects not mapped", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abcd")),
            selection: Some(EditorSelection::create(vec![SelectionRange::cursor(0), SelectionRange::cursor(2)], 0)),
            extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
        });
        let res = s.change_by_range(|r| {
            RangeResult::range(SelectionRange::cursor(r.from() + 2))
                .with_changes(ChangeSpec::insert(r.from(), "XX"))
                .with_effect(StateEffect::of(r.from() + 2))
        });
        let effs: Vec<usize> = res.effects.iter().map(|e| *e.downcast_ref::<usize>().unwrap()).collect();
        (format!("{}", res.changes), sel(&res.selection), effs)
    });
    r("R17c changeByRange main index + ranges", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("1 2 3 4")),
            selection: Some(EditorSelection::create(
                vec![SelectionRange::new(0, 1), SelectionRange::new(2, 3), SelectionRange::new(4, 5), SelectionRange::new(6, 7)],
                2,
            )),
            extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
        });
        let res = s.change_by_range(|r| {
            let count = (r.from() >> 1) + 1;
            RangeResult::range(SelectionRange::new(r.from(), r.from() + count))
                .with_changes(ChangeSpec::replace(r.from(), r.to(), "-".repeat(count).as_str()))
        });
        let n = s.apply(s.update(res.into_spec()).unwrap());
        let v: Vec<String> = n.selection().ranges().iter().map(|r| format!("{}-{}", r.from(), r.to())).collect();
        (n.doc().to_string(), v.join(" "), n.selection().main_index())
    });
    r("R17d changeByRange overlapping changes", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("abcdef")),
            selection: Some(EditorSelection::create(vec![SelectionRange::cursor(2), SelectionRange::cursor(4)], 0)),
            extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
        });
        let res = s.change_by_range(|r| {
            RangeResult::range(SelectionRange::cursor(r.from() - 1)).with_changes(ChangeSpec::replace(r.from() - 2, r.from() + 1, "_"))
        });
        (format!("{}", res.changes), sel(&res.selection), res.changes.apply(s.doc()).to_string())
    });
    r("R18 changes() CRLF insert", || {
        let s = st("abc");
        let c = s.changes(ChangeSpec::insert(0, "a\r\nb"));
        (format!("{}", c), c.new_length())
    });
    r("R18c changes(Changes::Set) — no counterpart; state.changes takes ChangeSpec only", || "n/a");
    r("R20 doc slice of CRLF doc (no sliceDoc)", || {
        let s = st("a\r\nb\nc");
        (s.doc().slice_string(0, s.doc().len_chars()), s.doc().len_chars(), s.doc().len_lines())
    });
    r("R21 create doc string split", || {
        let s = st("a\r\nb\rc\u{2028}d");
        (s.doc().len_lines(), s.doc().len_chars())
    });
    r("R21c create doc with lineSeparator \\r\\n, doc a\\nb\\r\\nc", || {
        let s = EditorState::create(EditorStateConfig {
            doc: Some(Text::from("a\nb\r\nc")),
            extensions: Some(vec![EditorState::line_separator().of("\r\n".to_string())]),
            ..Default::default()
        });
        (s.doc().len_lines(), s.doc().len_chars())
    });
    r("R23 field missing", || {
        let f = StateField::<usize>::define(|_| 1, |v, _| *v);
        st("").field(&f).copied()
    });
    r("R24 readOnly combine", || {
        let a = EditorState::create(EditorStateConfig {
            extensions: Some(vec![EditorState::read_only().of(false), EditorState::read_only().of(true)]),
            ..Default::default()
        });
        let b = EditorState::create(EditorStateConfig::default());
        let c = EditorState::create(EditorStateConfig { extensions: Some(vec![EditorState::read_only().of(true)]), ..Default::default() });
        (a.is_read_only(), b.is_read_only(), c.is_read_only())
    });
    r("R25 allowMultipleSelections combine", || {
        let a = EditorState::create(EditorStateConfig {
            extensions: Some(vec![EditorState::allow_multiple_selections().of(false), EditorState::allow_multiple_selections().of(true)]),
            ..Default::default()
        });
        *a.facet(EditorState::allow_multiple_selections())
    });
    r("R26 tabSize combine first", || {
        let a = EditorState::create(EditorStateConfig {
            extensions: Some(vec![EditorState::tab_size().of(2), EditorState::tab_size().of(8)]),
            ..Default::default()
        });
        *a.facet(EditorState::tab_size())
    });
    r("R29 charCategorizer", || {
        let c = st("").char_categorizer(0);
        vec![c("-"), c(""), c("!a"), c("a"), c(" "), c("\u{0085}"), c("\u{feff}"), c("_")]
    });
    r("R31a invalid change 2..1", || st("1234").update(TransactionSpec::new().with_change(ChangeSpec::replace(2, 1, ""))).is_some());
    r("R31b invalid change 2..10", || st("1234").update(TransactionSpec::new().with_change(ChangeSpec::replace(2, 10, "x"))).is_some());
    r("R32 selection beyond doc in update", || {
        let s = st("abc");
        let tr = s.update(TransactionSpec::new().with_selection(EditorSelection::cursor(10))).unwrap();
        sel(tr.state().selection())
    });
    r("R33 field init reads earlier field", || {
        let f1 = StateField::<usize>::define(|_| 5, |v, _| *v);
        let f1c = f1.clone();
        let f2 = StateField::<Option<usize>>::define(move |s| s.field(&f1c).map(|v| v + 10), |v, _| *v);
        let s = EditorState::create(EditorStateConfig { extensions: Some(vec![f1.extension(), f2.extension()]), ..Default::default() });
        *s.field(&f2).unwrap()
    });
    r("R33 (test#08 init as written: unwrap_or(0)+10 with f1=5)", || {
        let f1 = StateField::<usize>::define(|_| 5, |v, _| *v);
        let f1c = f1.clone();
        let f2 = StateField::<usize>::define(move |s| s.field(&f1c).copied().unwrap_or(0) + 10, |v, _| *v);
        let s = EditorState::create(EditorStateConfig { extensions: Some(vec![f1.extension(), f2.extension()]), ..Default::default() });
        *s.field(&f2).unwrap()
    });
    r("R33d dynamic facet from dynamic facet", || {
        let a = Facet::<usize>::define_list();
        let b = Facet::<usize>::define_list();
        let ac = a.clone();
        let s = EditorState::create(EditorStateConfig {
            extensions: Some(vec![b.compute(move |s| s.facet(&ac)[0] * 2), a.compute(|_| 21)]),
            ..Default::default()
        });
        s.facet(&b).clone()
    });
    r("R33e field init reads dynamic facet", || {
        let a = Facet::<usize>::define_list();
        let ac = a.clone();
        let f = StateField::<usize>::define(move |s| s.facet(&ac)[0], |v, _| *v);
        let s = EditorState::create(EditorStateConfig { extensions: Some(vec![f.extension(), a.compute(|_| 7)]), ..Default::default() });
        *s.field(&f).unwrap()
    });
    r("R34 facet identity across update (test#16)", || {
        let field = StateField::<Vec<usize>>::define(|_| vec![0], |v, tr| if tr.doc_changed() { vec![tr.doc().len_chars()] } else { v.clone() });
        let fc = field.clone();
        let facet = Facet::<usize>::define_list();
        let s = EditorState::create(EditorStateConfig {
            extensions: Some(vec![field.extension(), facet.compute(move |s| s.field(&fc).unwrap()[0]), facet.of(1)]),
            ..Default::default()
        });
        let s2 = s.apply(s.update(TransactionSpec::new()).unwrap());
        let s3 = s.apply(s.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "hi"))).unwrap());
        (s.facet(&facet).clone(), std::ptr::eq(s2.facet(&facet), s.facet(&facet)), s2.facet(&facet) == s.facet(&facet), s3.facet(&facet).clone())
    });
    r("R35 new field via compartment init sees which config", || {
        let c = Compartment::new();
        let t = Facet::<usize>::define_list();
        let tc = t.clone();
        let f = StateField::<String>::define(
            move |s| { let v: Vec<String> = s.facet(&tc).iter().map(|x| x.to_string()).collect(); if v.is_empty() { "none".into() } else { v.join("|") } },
            |v, _| format!("{}+u", v),
        );
        let s = EditorState::create(EditorStateConfig {
            extensions: Some(vec![c.of(Extension::Group(vec![])), t.of(1)]),
            ..Default::default()
        });
        let n = s.apply(s.update(TransactionSpec::new().with_effect(c.reconfigure(Extension::Group(vec![f.extension(), t.of(2)])))).unwrap());
        n.field(&f).cloned()
    });
    r("R38 doc_changed", || {
        let s = st("a");
        (
            s.update(TransactionSpec::new()).unwrap().doc_changed(),
            s.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "b"))).unwrap().doc_changed(),
            s.update(TransactionSpec::new().with_change(ChangeSpec::replace(0, 0, ""))).unwrap().doc_changed(),
        )
    });
    r("R40 start_state identity", || {
        let s = st("a");
        Arc::ptr_eq(s.update(TransactionSpec::new()).unwrap().start_state(), &s)
    });
    r("R42 replace_selection(\"\") cursor assoc", || {
        let s = st("ab");
        let n = s.apply(s.update(s.replace_selection("")).unwrap());
        (sel(n.selection()), n.selection().main().assoc)
    });
    r("R44 apply ignores ...: apply(tr) twice gives different Arcs", || {
        let s = st("a");
        let tr = s.update(TransactionSpec::new()).unwrap();
        let a = s.apply(tr.clone());
        let b = s.apply(tr);
        Arc::ptr_eq(&a, &b)
    });
    r("R45 apply tr to a different state than start_state", || {
        let s = st("abc");
        let other = st("xyz123");
        let tr = s.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "Q"))).unwrap();
        let n = other.apply(tr);
        (n.doc().to_string(), sel(n.selection()))
    });
    r("R46 into_spec annotations dropped / scroll", || {
        let s = st("ab");
        let spec = s.change_by_range(|r| RangeResult::range(*r));
        let sp = spec.into_spec();
        (sp.annotations.len(), sp.selection.is_some(), sp.changes.is_some())
    });
    r("R47 Text.of ignores lineSeparator; Text::from lone CR line", || {
        let t = Text::from("a\rb");
        (t.len_lines(), t.len_chars())
    });
}
