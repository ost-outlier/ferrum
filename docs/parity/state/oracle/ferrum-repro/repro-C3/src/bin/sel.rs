use ferrum_state::*;
use std::panic;

fn r(x: &SelectionRange) -> String {
    format!(
        "{{from:{},to:{},anchor:{},head:{},assoc:{},goal:{:?},empty:{},inverted:{}}}",
        x.from(), x.to(), x.anchor, x.head, x.assoc, x.goal_column, x.is_empty(), x.is_inverted()
    )
}
fn s(x: &EditorSelection) -> String {
    let v: Vec<String> = x
        .ranges()
        .iter()
        .map(|r| format!("{}/{}{}", r.anchor, r.head, if r.assoc != 0 { format!("@{}", r.assoc) } else { String::new() }))
        .collect();
    format!("[{}] main={}", v.join(","), x.main_index())
}
fn t<F: FnOnce() -> String + panic::UnwindSafe>(label: &str, f: F) {
    match panic::catch_unwind(f) {
        Ok(v) => println!("{} => {}", label, v),
        Err(e) => {
            let msg = e
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            println!("{} => PANIC {}", label, msg)
        }
    }
}
fn ins(at: usize, txt: &str) -> ChangeDesc {
    ChangeSet::of_spec(ChangeSpec::insert(at, txt), 10).desc().clone()
}
fn del(from: usize, to: usize) -> ChangeDesc {
    ChangeSet::of_spec(ChangeSpec::delete(from, to), 10).desc().clone()
}
fn cur(pos: usize, assoc: i8, goal: Option<usize>) -> SelectionRange {
    let mut c = SelectionRange::cursor(pos);
    c.assoc = assoc;
    c.goal_column = goal;
    c
}

fn main() {
    panic::set_hook(Box::new(|_| {}));
    t("S01 new(3,2)", || r(&SelectionRange::new(3, 2)));
    t("S02 new(2,3)", || r(&SelectionRange::new(2, 3)));
    t("S03 new(4,4)", || r(&SelectionRange::new(4, 4)));
    t("S04 new(2,3).with_goal_column(Some(7))", || r(&SelectionRange::new(2, 3).with_goal_column(Some(7))));
    t("S07 SelectionRange::cursor(5)", || r(&SelectionRange::cursor(5)));
    t("S07b EditorSelection::cursor(5)", || s(&EditorSelection::cursor(5)));
    // map
    t("S20 cursor(5).map(ins5,-1)", || r(&SelectionRange::cursor(5).map(&ins(5, "x"), -1)));
    t("S21 cursor(5).map(ins5,1)", || r(&SelectionRange::cursor(5).map(&ins(5, "x"), 1)));
    t("S22 cursor(5,assoc=1).map(ins5,-1)", || r(&cur(5, 1, None).map(&ins(5, "x"), -1)));
    t("S23 new(2,5).map(ins2,-1)", || r(&SelectionRange::new(2, 5).map(&ins(2, "x"), -1)));
    t("S24 new(2,5).map(ins5,-1)", || r(&SelectionRange::new(2, 5).map(&ins(5, "x"), -1)));
    t("S25 new(2,5).map(ins2,1)", || r(&SelectionRange::new(2, 5).map(&ins(2, "x"), 1)));
    t("S26 new(5,2).map(ins2,-1)", || r(&SelectionRange::new(5, 2).map(&ins(2, "x"), -1)));
    t("S27 new(2,5).map(del1-6,-1)", || r(&SelectionRange::new(2, 5).map(&del(1, 6), -1)));
    t("S28 new(5,2).map(del1-6,-1)", || r(&SelectionRange::new(5, 2).map(&del(1, 6), -1)));
    t("S29 cursor(5,1,goal 8).map(ins0)", || r(&cur(5, 1, Some(8)).map(&ins(0, "ab"), -1)));
    t("S32 new(2,4).map(ins3)", || r(&SelectionRange::new(2, 4).map(&ins(3, "xy"), -1)));
    t("S33 cursor(4).map(del2-6)", || r(&SelectionRange::cursor(4).map(&del(2, 6), -1)));
    t("S34 cursor(12).map(ins0)", || r(&SelectionRange::cursor(12).map(&ins(0, "x"), -1)));
    t("S35 new(2,5).map(ins5,1)", || r(&SelectionRange::new(2, 5).map(&ins(5, "x"), 1)));
    // extend
    t("S40 new(5,8).extend(2,2)", || r(&SelectionRange::new(5, 8).extend(2, 2)));
    t("S41 new(5,8).extend(3,10)", || r(&SelectionRange::new(5, 8).extend(3, 10)));
    t("S42 new(5,8).extend(6,9)", || r(&SelectionRange::new(5, 8).extend(6, 9)));
    t("S43 new(5,8).extend(1,9)", || r(&SelectionRange::new(5, 8).extend(1, 9)));
    t("S44 new(5,8).extend(7,1)", || r(&SelectionRange::new(5, 8).extend(7, 1)));
    t("S46 cursor(5,goal 7).extend(9,9)", || r(&cur(5, 0, Some(7)).extend(9, 9)));
    t("S47 new(8,5).extend(3,3)", || r(&SelectionRange::new(8, 5).extend(3, 3)));
    t("S48 new(5,8).extend(9,2)", || r(&SelectionRange::new(5, 8).extend(9, 2)));
    t("S49 cursor(5,assoc 1).extend(5,5)", || r(&cur(5, 1, None).extend(5, 5)));
    // eq
    t("S50 cursor(5,-1)==cursor(5,1)", || format!("{}", cur(5, -1, None) == cur(5, 1, None)));
    t("S52 new(2,5) assoc1 == new(2,5) assoc -1", || { let mut a = SelectionRange::new(2, 5); a.assoc = 1; let mut b = SelectionRange::new(2, 5); b.assoc = -1; format!("{}", a == b) });
    t("S53 cursor goal 3 == cursor", || format!("{}", cur(5, 0, Some(3)) == SelectionRange::cursor(5)));
    t("S54 new(2,5)==new(5,2)", || format!("{}", SelectionRange::new(2, 5) == SelectionRange::new(5, 2)));
    t("S57 sel eq different main", || format!("{}", EditorSelection::create(vec![SelectionRange::cursor(1), SelectionRange::cursor(3)], 0) == EditorSelection::create(vec![SelectionRange::cursor(1), SelectionRange::cursor(3)], 1)));
    t("S58 sel eq assoc", || format!("{}", EditorSelection::single(1, 1) == EditorSelection::create(vec![cur(1, 1, None)], 0)));
    // single/create
    t("S70 single(3,3)", || s(&EditorSelection::single(3, 3)));
    t("S71 single(3,1)", || r(EditorSelection::single(3, 1).main()));
    t("S73 create([])", || s(&EditorSelection::create(vec![], 0)));
    t("S74 create([c1,c3],5)", || { let x = EditorSelection::create(vec![SelectionRange::cursor(1), SelectionRange::cursor(3)], 5); format!("{} main={}", s(&x), r(x.main())) });
    t("S75 create([c3,c1],5)", || { let x = EditorSelection::create(vec![SelectionRange::cursor(3), SelectionRange::cursor(1)], 5); format!("{} main={}", s(&x), r(x.main())) });
    t("S76 create touch nonempty", || s(&EditorSelection::create(vec![SelectionRange::new(0, 2), SelectionRange::new(2, 4)], 0)));
    t("S77 create cursor at end", || s(&EditorSelection::create(vec![SelectionRange::new(0, 2), SelectionRange::cursor(2)], 0)));
    t("S78 create cursor at start", || s(&EditorSelection::create(vec![SelectionRange::cursor(2), SelectionRange::new(2, 4)], 0)));
    t("S79 create c0,c0", || s(&EditorSelection::create(vec![SelectionRange::cursor(0), SelectionRange::cursor(0)], 0)));
    t("S7A merge", || { let x = EditorSelection::create(vec![SelectionRange::new(0, 3), SelectionRange::new(2, 5)], 0); format!("{} {}", s(&x), r(x.main())) });
    t("S7B merge inverted later", || { let x = EditorSelection::create(vec![SelectionRange::new(0, 3), SelectionRange::new(5, 2)], 0); format!("{} {}", s(&x), r(x.main())) });
    t("S7C merge drops goal", || { let x = EditorSelection::create(vec![cur(3, 1, Some(7)), SelectionRange::new(1, 4)], 0); format!("{} {}", s(&x), r(x.main())) });
    t("S7D merge cursors same pos", || { let x = EditorSelection::create(vec![cur(3, 1, Some(7)), cur(3, -1, None)], 0); format!("{} {}", s(&x), r(x.main())) });
    t("S7E create main tracking", || s(&EditorSelection::create(vec![SelectionRange::cursor(9), SelectionRange::new(0, 3), SelectionRange::new(2, 5), SelectionRange::cursor(7)], 3)));
    t("S7F create main merged", || s(&EditorSelection::create(vec![SelectionRange::cursor(9), SelectionRange::new(0, 3), SelectionRange::new(2, 5), SelectionRange::cursor(7)], 2)));
    t("S7G create sorted keeps fields", || r(EditorSelection::create(vec![cur(3, 1, Some(7))], 0).main()));
    t("S7H create main dup identity", || s(&EditorSelection::create(vec![SelectionRange::new(1, 3), SelectionRange::cursor(1), SelectionRange::new(1, 3)], 2)));
    let multi = EditorSelection::create(vec![SelectionRange::cursor(1), SelectionRange::new(3, 5), SelectionRange::cursor(8)], 1);
    let m2 = multi.clone();
    t("S80 main", move || r(m2.main()));
    let m3 = multi.clone();
    t("S90 sel.map(empty)", move || { let e = ChangeDesc::empty(10); format!("{}", m3.map(&e, -1) == m3) });
    let m4 = multi.clone();
    t("S91 sel.map(del0-6)", move || s(&m4.map(&del(0, 6), -1)));
    let m5 = multi.clone();
    t("S92 sel.map(ins1,1)", move || s(&m5.map(&ins(1, "xx"), 1)));
    let m6 = multi.clone();
    t("S93 sel.map(ins1,-1)", move || s(&m6.map(&ins(1, "xx"), -1)));
    t("S94 Default", || s(&EditorSelection::default()));
    t("S97 EditorState create sel out of doc", || {
        let st = EditorState::create(EditorStateConfig { doc: Some("abc".into()), selection: Some(EditorSelection::single(5, 5)), ..Default::default() });
        format!("{}", st.selection().main().head)
    });
}
