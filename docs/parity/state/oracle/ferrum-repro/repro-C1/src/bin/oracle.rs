// Replays cm6-generated cases (probe-C1/oracle.ts) against ferrum_state::Text and counts mismatches.
use ferrum_state::*;
use serde_json::Value;
use std::collections::BTreeMap;

fn lv(l: &Line) -> Value { serde_json::json!([l.from, l.to, l.number, l.text]) }

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let data: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let mut mism: BTreeMap<&str, (usize, String)> = BTreeMap::new();
    let mut checks = 0usize;
    let mut note = |k: &'static str, detail: String| { let e = mism.entry(k).or_insert((0, detail)); e.0 += 1; };
    for case in data.as_array().unwrap() {
        let init: Vec<String> = case["init"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
        let mut doc = Text::of(&init);
        let mut prev_text = doc.to_string();
        for op in case["ops"].as_array().unwrap() {
            let from = op["from"].as_u64().unwrap() as usize;
            let to = op["to"].as_u64().unwrap() as usize;
            let ins: Vec<String> = op["ins"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
            let ins_s = ins.join("\n");
            doc = doc.replace(from..to, &ins_s);
            checks += 4;
            let exp_text = op["text"].as_str().unwrap();
            if doc.to_string() != exp_text { note("replace/toString", format!("before={:?} replace({},{},{:?}) cm6={:?} ferrum={:?}", prev_text, from, to, ins_s, exp_text, doc.to_string())); }
            if doc.len() as u64 != op["length"].as_u64().unwrap() { note("length", format!("text={:?} cm6={} ferrum={}", exp_text, op["length"], doc.len())); }
            if doc.len_lines() as u64 != op["lines"].as_u64().unwrap() { note("lines", format!("text={:?} cm6={} ferrum={}", exp_text, op["lines"], doc.len_lines())); }
            let json_lines: Vec<String> = (1..=doc.len_lines()).map(|n| doc.line(n).text).collect();
            if serde_json::json!(json_lines) != op["json"] { note("toJSON-equivalent (line texts)", format!("text={:?} cm6={} ferrum={:?}", exp_text, op["json"], json_lines)); }
            for p in op["probes"].as_array().unwrap() {
                checks += 5;
                let pos = p["pos"].as_u64().unwrap() as usize;
                let n = p["n"].as_u64().unwrap() as usize;
                let a = p["a"].as_u64().unwrap() as usize;
                let b = p["b"].as_u64().unwrap() as usize;
                let la = std::panic::catch_unwind(|| lv(&doc.line_at(pos))).unwrap_or(Value::from("PANIC"));
                if la != p["lineAt"] { note("lineAt", format!("text={:?} pos={} cm6={} ferrum={}", exp_text, pos, p["lineAt"], la)); }
                let ln = std::panic::catch_unwind(|| lv(&doc.line(n))).unwrap_or(Value::from("PANIC"));
                if ln != p["line"] { note("line", format!("text={:?} n={} cm6={} ferrum={}", exp_text, n, p["line"], ln)); }
                let ss = doc.slice_string(a, b);
                if ss != p["sliceString"].as_str().unwrap() { note("sliceString", format!("text={:?} ({},{}) cm6={} ferrum={:?}", exp_text, a, b, p["sliceString"], ss)); }
                let sl = doc.slice(a..b);
                if sl.to_string() != p["slice"].as_str().unwrap() { note("slice.toString", format!("text={:?} ({},{}) cm6={} ferrum={:?}", exp_text, a, b, p["slice"], sl.to_string())); }
                if sl.len_lines() as u64 != p["sliceLines"].as_u64().unwrap() { note("slice.lines", format!("({},{}) cm6={} ferrum={}", a, b, p["sliceLines"], sl.len_lines())); }
            }
            prev_text = doc.to_string();
        }
    }
    println!("checks={} mismatching categories={}", checks, mism.len());
    for (k, (c, d)) in mism { let d: String = d.chars().take(400).collect(); println!("  {}: {} mismatches; first: {}", k, c, d); }
}
