use ferrum_state::*;
fn l(x: Line) -> (usize, usize, usize, String, usize) { (x.from, x.to, x.number, x.text, x.length) }
fn main() {
    let a = Text::of(&["a\r"]); let b = Text::of(&["", "b"]);
    println!("P1 of(['a\\r']).lines={} of(['','b']).lines={} append.lines={} append={:?}", a.len_lines(), b.len_lines(), a.append(&b).len_lines(), a.append(&b).to_string());
    let d = Text::of(&["a\r", "b"]);
    println!("P2 of(['a\\r','b']) len={} lines={} line1={:?} line2={:?}", d.len(), d.len_lines(), l(d.line(1)), l(d.line(2)));
    let d = Text::of(&["a\r", "x", "b"]);
    let r = d.replace(2..4, "");
    println!("P3 of(['a\\r','x','b']).lines={} replace(2..4,'')={:?} lines={}", d.len_lines(), r.to_string(), r.len_lines());
    let d = Text::from_str("a\r\nb");
    let s = d.slice(0..2);
    println!("P4 from_str('a\\r\\nb').slice(0..2)={:?} lines={} ; slice(2..4)={:?} lines={}", s.to_string(), s.len_lines(), d.slice(2..4).to_string(), d.slice(2..4).len_lines());
    let s1 = Text::of(&["ab", "cd"]); let s2 = Text::from_str("ab\r\ncd");
    println!("P5 eq of(['ab','cd']) vs from_str('ab\\r\\ncd') = {}", s1 == s2);
    let d = Text::of(&["x"]); let ins = d.replace(1..1, "\u{2028}y");
    println!("P6 replace insert U+2028: lines={} line1={:?}", ins.len_lines(), l(ins.line(1)));
    // line_by_index allocation: the line text is a fresh String per call
    let big = "x".repeat(1_000_000); let d = Text::of(&[big.as_str(), "y"]);
    let t = std::time::Instant::now(); let mut n = 0; for _ in 0..100 { n += d.line_at(5).from; } 
    println!("P7 100x line_at(5) on a 1,000,000-char line: {:?} (n={})", t.elapsed(), n);
    let t = std::time::Instant::now(); let mut n = 0; for _ in 0..100 { n += d.line_at(1_000_001).from; }
    println!("P7b 100x line_at on the 1-char line: {:?} (n={})", t.elapsed(), n);
}
