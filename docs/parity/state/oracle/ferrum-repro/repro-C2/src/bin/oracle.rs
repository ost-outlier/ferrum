// Ferrum side of the C2 differential oracle. Usage: oracle <cases.json> > ferrum-out.json
use ferrum_state::*;
use serde_json::{json, Value};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn specs(v: &Value) -> Vec<ChangeSpec> {
    v.as_array().unwrap().iter().map(|s| {
        ChangeSpec::replace(s["from"].as_u64().unwrap() as usize, s["to"].as_u64().unwrap() as usize, s["insert"].as_str().unwrap())
    }).collect()
}

fn changes(set: &ChangeSet, individual: bool) -> Value {
    let mut out = vec![];
    set.iter_changes(|fa, ta, fb, tb, t| out.push(json!([fa, ta, fb, tb, t.to_string()])), individual);
    Value::Array(out)
}

fn run(cs: &Value) -> Value {
    let mut res = serde_json::Map::new();
    let r = catch_unwind(AssertUnwindSafe(|| {
        let size = cs["size"].as_u64().unwrap() as usize;
        let doc = Text::from(cs["doc"].as_str().unwrap());
        let a = ChangeSet::of_specs(&specs(&cs["a"]), size);
        let b = ChangeSet::of_specs(&specs(&cs["b"]), size);
        res.insert("A".into(), json!(a.to_string()));
        res.insert("Alen".into(), json!([a.length(), a.new_length(), a.is_empty()]));
        res.insert("Ach".into(), changes(&a, false));
        res.insert("AchI".into(), changes(&a, true));
        let mut gaps = vec![];
        a.iter_gaps(|x, y, l| gaps.push(json!([x, y, l])));
        res.insert("Agaps".into(), Value::Array(gaps));
        res.insert("Aapply".into(), json!(a.apply(&doc).to_string()));
        res.insert("B".into(), json!(b.to_string()));
        let mf = a.map(b.desc(), false);
        let mt = a.map(b.desc(), true);
        res.insert("mapF".into(), json!(mf.to_string()));
        res.insert("mapT".into(), json!(mt.to_string()));
        res.insert("mapFch".into(), changes(&mf, true));
        res.insert("mapTch".into(), changes(&mt, true));
        res.insert("mapDescF".into(), json!(a.desc().map_desc(b.desc(), false).to_string()));
        res.insert("mapDescT".into(), json!(a.desc().map_desc(b.desc(), true).to_string()));
        res.insert("conv".into(), json!([
            a.compose(&b.map(a.desc(), false)).apply(&doc).to_string(),
            b.compose(&a.map(b.desc(), true)).apply(&doc).to_string()
        ]));
        let inv = a.invert(&doc);
        res.insert("inv".into(), json!(inv.to_string()));
        res.insert("invch".into(), changes(&inv, true));
        res.insert("invApply".into(), json!(inv.apply(&a.apply(&doc)).to_string()));
        res.insert("invDesc".into(), json!(a.desc().inverted_desc().to_string()));
        let c = ChangeSet::of_specs(&specs(&cs["c"]), cs["cSize"].as_u64().unwrap() as usize);
        let ac = a.compose(&c);
        res.insert("comp".into(), json!(ac.to_string()));
        res.insert("compch".into(), changes(&ac, true));
        res.insert("compApply".into(), json!(ac.apply(&doc).to_string()));
        res.insert("compDesc".into(), json!(a.desc().compose_desc(c.desc()).to_string()));
        let modes = [MapMode::Simple, MapMode::TrackDel, MapMode::TrackBefore, MapMode::TrackAfter];
        let mut mp = vec![];
        for pos in 0..=size {
            for assoc in [-1i8, 1] {
                for m in modes {
                    mp.push(match a.desc().map_pos(pos, assoc, m) { Some(p) => json!(p), None => Value::Null });
                }
            }
        }
        res.insert("mapPos".into(), Value::Array(mp));
        let mut cr = vec![];
        a.desc().iter_changed_ranges(|w, x, y, z| cr.push(json!([w, x, y, z])), true);
        res.insert("ranges".into(), Value::Array(cr));
    }));
    if let Err(e) = r {
        let msg = if let Some(s) = e.downcast_ref::<String>() { s.clone() } else if let Some(s) = e.downcast_ref::<&str>() { s.to_string() } else { "?".into() };
        res.insert("error".into(), json!(msg));
    }
    Value::Object(res)
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let path = std::env::args().nth(1).unwrap();
    let cases: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let out: Vec<Value> = cases.as_array().unwrap().iter().map(run).collect();
    println!("{}", serde_json::to_string(&out).unwrap());
}
