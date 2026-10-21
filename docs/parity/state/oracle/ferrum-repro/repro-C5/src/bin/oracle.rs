// Ferrum side of the C5 changeByRange/replaceSelection oracle. Reads <cases>, writes <cases>.ferrum
use ferrum_state::*;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn unesc(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() { Some('n') => out.push('\n'), Some('t') => out.push('\t'), Some('\\') => out.push('\\'), Some(o) => { out.push('\\'); out.push(o) }, None => out.push('\\') }
        } else { out.push(c) }
    }
    out
}
fn esc(s: &str) -> String { s.replace('\\', "\\\\").replace('\n', "\\n").replace('\t', "\\t") }
fn sel(s: &EditorSelection) -> String { s.ranges().iter().map(|r| format!("{}:{}", r.anchor, r.head)).collect::<Vec<_>>().join(",") }

fn run(op: &str, doc: &str, ranges: Vec<SelectionRange>, main: usize, text: &str) -> String {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(doc)),
        selection: Some(EditorSelection::create(ranges, main)),
        extensions: Some(vec![EditorState::allow_multiple_selections().of(true)]),
    });
    let len = state.doc().len_chars();
    let spec = match op {
        "rs" => state.replace_selection(text),
        "cbr1" => state.change_by_range(|r| { let n = (r.from() + 2 * r.to()) % 3; RangeResult::range(SelectionRange::new(r.from(), r.from() + n)).with_changes(ChangeSpec::replace(r.from(), r.to(), "Z".repeat(n).as_str())) }).into_spec(),
        "cbr2" => state.change_by_range(|r| { let f = r.from().saturating_sub(1); RangeResult::range(SelectionRange::cursor(f + 1)).with_changes(ChangeSpec::replace(f, (r.to() + 1).min(len), "W")) }).into_spec(),
        "cbr3" => state.change_by_range(|r| RangeResult::range(SelectionRange::new(r.head, r.anchor))).into_spec(),
        _ => state.change_by_range(|r| RangeResult::range(SelectionRange::new(r.from() + 1, r.to() + 1)).with_changes(vec![ChangeSpec::insert(r.from(), "<"), ChangeSpec::insert(r.to(), ">")])).into_spec(),
    };
    let desc = match &spec.changes { Some(Changes::Set(cs)) => format!("{}", cs.desc()), _ => "?".into() };
    let tr = state.update(spec).expect("no filters");
    let next = state.apply(tr);
    let desc = if std::env::var("NODESC").is_ok() { "-".to_string() } else { desc };
    format!("{}\t{}\t{}\t{}\t{}", esc(&next.doc().to_string()), sel(next.selection()), next.selection().main_index(), desc, sel(state.selection()))
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let file = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(&file).unwrap();
    let mut out = String::new();
    for line in src.lines() {
        let p: Vec<&str> = line.split('\t').collect();
        let (op, doc, rs, main, text) = (p[0], unesc(p[1]), p[2], p[3].parse::<usize>().unwrap(), unesc(p.get(4).copied().unwrap_or("")));
        let ranges: Vec<SelectionRange> = rs.split(',').map(|x| { let v: Vec<usize> = x.split(':').map(|n| n.parse().unwrap()).collect(); SelectionRange::new(v[0], v[1]) }).collect();
        let res = catch_unwind(AssertUnwindSafe(|| run(op, &doc, ranges, main, &text)));
        out.push_str(&match res { Ok(s) => s, Err(e) => format!("THROWS {}", e.downcast_ref::<String>().cloned().or(e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()) });
        out.push('\n');
    }
    std::fs::write(format!("{}.ferrum", file), out).unwrap();
}
