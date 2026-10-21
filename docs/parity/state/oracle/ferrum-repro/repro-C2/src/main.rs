use ferrum_state::*;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn t<R: std::fmt::Debug>(label: &str, f: impl FnOnce() -> R) {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(v) => println!("{} => {:?}", label, v),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() { s.clone() } else if let Some(s) = e.downcast_ref::<&str>() { s.to_string() } else { "?".into() };
            println!("{} => PANIC: {}", label, msg)
        }
    }
}

fn s(c: &ChangeSet) -> String { c.to_string() }

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let doc10 = Text::from("0123456789");
    let mk = |v: Vec<i64>| ChangeDesc::create(v);

    // R1 input forms
    t("R1b ChangeSet inside specs: not expressible (Changes::Multiple(Vec<ChangeSpec>) only)", || ());
    t("R1c single", || s(&ChangeSet::of_spec(ChangeSpec::replace(3, 5, "q"), 10)));
    t("R1d Changes::Set", || s(&Changes::Set(ChangeSet::of_spec(ChangeSpec::insert(3, "q"), 10)).to_changeset(10)));
    t("R1d2 Changes::Set wrong length accepted", || { let c = Changes::Set(ChangeSet::empty(5)).to_changeset(10); (s(&c), c.length()) });
    t("R1e insert as Text of lines", || { let c = ChangeSet::of_spec(ChangeSpec::insert(0, Text::of(&["a", "b"])), 10); (s(&c), c.apply(&doc10).to_string()) });
    t("R1f insert \\r\\n", || { let c = ChangeSet::of_spec(ChangeSpec::insert(0, "a\r\nb"), 10); (s(&c), c.apply(&doc10).to_string(), c.new_length()) });
    t("R1g insert lone \\r", || { let c = ChangeSet::of_spec(ChangeSpec::insert(0, "a\rb"), 10); (s(&c), c.apply(&doc10).to_string(), c.apply(&doc10).len_lines()) });
    t("R1i insert u2028", || { let c = ChangeSet::of_spec(ChangeSpec::insert(0, "a\u{2028}b"), 10); (s(&c), c.apply(&doc10).len_lines()) });
    t("R1j empty inserts", || s(&ChangeSet::of_specs(&[ChangeSpec::delete(3, 3), ChangeSpec::insert(3, "")], 10)));
    t("R1k empty specs", || s(&ChangeSet::of_specs(&[], 10)));
    t("R1l empty specs len 0", || s(&ChangeSet::of_specs(&[], 0)));
    t("R1m unsorted", || s(&ChangeSet::of_specs(&[ChangeSpec::insert(8, "a"), ChangeSpec::delete(2, 4)], 10)));
    t("R1n overlapping", || { let c = ChangeSet::of_specs(&[ChangeSpec::replace(2, 6, "A"), ChangeSpec::replace(4, 8, "B")], 10); (s(&c), c.apply(&doc10).to_string()) });
    t("R1o Changes::Multiple", || s(&Changes::Multiple(vec![ChangeSpec::insert(1, "a"), ChangeSpec::delete(3, 4)]).to_changeset(10)));

    // R2 errors
    t("R2a from>to", || ChangeSet::of_spec(ChangeSpec::delete(5, 3), 10));
    t("R2b to>len", || ChangeSet::of_spec(ChangeSpec::delete(5, 11), 10));
    t("R2e apply wrong length", || ChangeSet::of_spec(ChangeSpec::insert(1, "x"), 5).apply(&doc10));
    t("R2f map_pos past end", || mk(vec![4, -1, 0, 2]).map_pos(7, -1, MapMode::Simple));
    t("R2f2 map_pos past end empty", || ChangeDesc::empty(0).map_pos(1, -1, MapMode::Simple));
    t("R2f2b ChangeSet::map_pos past end", || ChangeSet::empty(0).map_pos(1, -1));
    t("R2f3 map_pos pos==len", || mk(vec![4, -1, 0, 2]).map_pos(4, 1, MapMode::Simple));
    t("R2g compose mismatch", || ChangeSet::of_spec(ChangeSpec::insert(1, "x"), 5).compose(&ChangeSet::of_spec(ChangeSpec::insert(1, "x"), 5)));
    t("R2h map mismatch", || ChangeSet::of_spec(ChangeSpec::insert(1, "x"), 5).map(ChangeSet::of_spec(ChangeSpec::insert(1, "x"), 6).desc(), false));
    t("R2i compose_desc mismatch", || mk(vec![2, -1, 0, 2]).compose_desc(&mk(vec![1, -1, 0, 1])));
    t("R2i2 compose_desc mismatch 2", || mk(vec![2, -1, 0, 2]).compose_desc(&mk(vec![30, -1, 0, 1])));
    t("R2i3 compose_desc mismatch 3", || mk(vec![2, -1, 2, 0, 0, 3]).compose_desc(&mk(vec![7, 0])));
    t("R2o invert short doc", || { let c = ChangeSet::of_spec(ChangeSpec::delete(2, 8), 10); s(&c.invert(&Text::from("abc"))) });
    t("R2o2 invert short doc inserted", || { let c = ChangeSet::of_spec(ChangeSpec::delete(2, 8), 10); c.invert(&Text::from("abc")).apply(&Text::from("01")).to_string() });
    t("R2p map shorter desc", || ChangeSet::of_spec(ChangeSpec::insert(1, "x"), 5).map(&mk(vec![3, -1]), false));
    t("R2q from_string invalid", || ChangeDesc::from_string("a b"));
    t("R2r from_string negative accepted", || ChangeDesc::from_string("-5").to_string());
    t("R2s create unvalidated odd", || { let d = mk(vec![3]); d.length() });

    // R3 empty
    t("R3a ChangeSet::empty(0)", || { let c = ChangeSet::empty(0); (s(&c), c.is_empty(), c.length()) });
    t("R3b ChangeSet::empty(5)", || { let c = ChangeSet::empty(5); (s(&c), c.is_empty()) });
    t("R3b2 ChangeDesc::empty(5)", || { let c = ChangeDesc::empty(5); (c.to_string(), c.is_empty()) });
    t("R3d is_empty insertion", || ChangeSet::of_spec(ChangeSpec::insert(0, "x"), 0).is_empty());
    t("R3e is_empty unnormalized", || mk(vec![3, -1, 2, -1]).is_empty());
    t("R3f length/new_length", || { let c = ChangeSet::of_spec(ChangeSpec::replace(1, 3, "abc"), 10); (c.length(), c.new_length()) });

    // R4 iter
    {
        let cs = ChangeSet::of_specs(&[ChangeSpec::replace(2, 4, "A"), ChangeSpec::replace(4, 6, "B")], 10);
        println!("R4 sections => {}", cs);
        let mut out = vec![]; cs.iter_changes(|a, b, c, d, txt| out.push((a, b, c, d, txt.to_string())), false);
        println!("R4a => {:?}", out);
        let mut out = vec![]; cs.iter_changes(|a, b, c, d, txt| out.push((a, b, c, d, txt.to_string())), true);
        println!("R4b individual => {:?}", out);
        let mut out = vec![]; cs.desc().iter_changed_ranges(|a, b, c, d| out.push((a, b, c, d)), false);
        println!("R4c => {:?}", out);
        let mut out = vec![]; cs.desc().iter_changed_ranges(|a, b, c, d| out.push((a, b, c, d)), true);
        println!("R4d individual => {:?}", out);
        let mut out = vec![]; cs.iter_gaps(|a, b, l| out.push((a, b, l)));
        println!("R4f gaps => {:?}", out);
    }

    // R7 map_pos assoc values
    {
        let d = mk(vec![4, -1, 0, 2, 4, -1]);
        t("R7b map_pos(4,0)", || d.map_pos(4, 0, MapMode::Simple));
        t("R7c map_pos(4,5)", || d.map_pos(4, 5, MapMode::Simple));
        t("R7d map_pos(4,-100)", || d.map_pos(4, -100, MapMode::Simple));
        t("R7f ChangeSet TrackDel via desc", || ChangeSet::of_spec(ChangeSpec::delete(2, 6), 10).desc().map_pos(4, 1, MapMode::TrackDel));
        t("R7g insertion at end", || d.map_pos(8, 1, MapMode::Simple));
        t("R7i ChangeSet::map_pos", || ChangeSet::of_spec(ChangeSpec::insert(4, "xx"), 8).map_pos(4, 1));
        t("R7j i32 side -100_000_000 as i8", || (-100_000_000i32) as i8);
        t("R7k i32 side -100_000_001 as i8", || (-100_000_001i32) as i8);
        t("R7l i32 side -600_000_000 as i8", || (-600_000_000i32) as i8);
        t("R7m i32 side 100_000_000 as i8", || (100_000_000i32) as i8);
        t("R7n i32 side -300_000_000 as i8", || (-300_000_000i32) as i8);
        t("R7o i32 side 200_000_000 as i8", || (200_000_000i32) as i8);
    }

    // R8 map/compose/invert
    {
        let a = ChangeSet::of_spec(ChangeSpec::insert(1, "zz"), 5);
        let b = ChangeSet::of_spec(ChangeSpec::delete(0, 2), 5);
        t("R8a a.map(b) as set", || s(&a.map(b.desc(), false)));
        t("R8a2 a.desc().map_desc(b)", || a.desc().map_desc(b.desc(), false).to_string());
        t("R8e compose with empty", || (s(&ChangeSet::empty(5).compose(&ChangeSet::of_spec(ChangeSpec::insert(0, "q"), 5)))));
        t("R8g inverted_desc", || mk(vec![2, -1, 3, 1, 0, 4, 2, -1]).inverted_desc().to_string());
        t("R8h invert", || { let c = ChangeSet::of_specs(&[ChangeSpec::replace(1, 3, "XYZ"), ChangeSpec::insert(5, "q")], 10); let inv = c.invert(&doc10); let mut ins = vec![]; inv.iter_changes(|_, _, _, _, t| ins.push(t.to_string()), true); (s(&inv), ins, inv.apply(&c.apply(&doc10)).to_string()) });
        t("R8i compose_desc mixed", || mk(vec![1, -1, 0, 2]).compose_desc(ChangeSet::of_spec(ChangeSpec::delete(0, 1), 3).desc()).to_string());
    }

    // R9 non-ASCII
    {
        let d = Text::from("a😀b");
        t("R9a len", || d.len_chars());
        t("R9b insert emoji", || { let c = ChangeSet::of_spec(ChangeSpec::insert(1, "😀"), 3); (s(&c), c.new_length()) });
        t("R9c delete emoji 1..2", || ChangeSet::of_spec(ChangeSpec::delete(1, 2), 3).apply(&d).to_string());
        t("R9c2 delete 1..3 (cm6 unit) on ferrum doc len 3", || ChangeSet::of_spec(ChangeSpec::delete(1, 3), 4).apply(&d).to_string());
        t("R9e map_pos(3) over insert é", || ChangeSet::of_spec(ChangeSpec::insert(0, "é"), 4).map_pos(3, -1));
        t("R9f CRLF in insert length", || ChangeSet::of_spec(ChangeSpec::insert(0, "x\r\ny"), 0).new_length());
    }
    t("R10 Display", || s(&ChangeSet::of_specs(&[ChangeSpec::delete(1, 2), ChangeSpec::insert(4, "ab")], 6)));
    t("R10b Debug", || format!("{:?} {:?}", ChangeSet::of_specs(&[ChangeSpec::delete(1, 2)], 6), ChangeDesc::from_string("1 1:0 4")));
    t("R11 from_string", || ChangeDesc::from_string("5 0:2 2:0 4").to_string());
    t("R11b create_set", || { let c = ChangeSet::create_set(vec![2, -1, 0, 2], vec![Text::empty(), Text::from("ab")]); c.apply(&Text::from("xy")).to_string() });
    t("R11c create_set missing inserted", || { let c = ChangeSet::create_set(vec![2, -1, 0, 2], vec![]); (c.apply(&Text::from("xy")).to_string(), c.new_length()) });
    t("R12 Changes From impls", || (Changes::from(ChangeSpec::insert(0, "a")), Changes::from(vec![ChangeSpec::insert(0, "a")]), Changes::from(ChangeSet::empty(0))));
}
