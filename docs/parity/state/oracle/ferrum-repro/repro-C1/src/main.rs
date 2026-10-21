use ferrum_state::*;
use std::panic;

fn t<T: std::fmt::Debug>(name: &str, f: impl FnOnce() -> T + panic::UnwindSafe) {
    match panic::catch_unwind(f) {
        Ok(v) => println!("{} => {:?}", name, v),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() { s.clone() } else if let Some(s) = e.downcast_ref::<&str>() { s.to_string() } else { "?".into() };
            println!("{} => PANIC {:?}", name, msg)
        }
    }
}
fn l(x: Line) -> (usize, usize, usize, String, usize) { (x.from, x.to, x.number, x.text, x.length) }

fn main() {
    panic::set_hook(Box::new(|_| {}));
    let line = "1234567890".repeat(10);
    let lines: Vec<&str> = vec![line.as_str(); 200];
    let doc0 = Text::of(&lines);
    let empty: [&str; 0] = [];
    t("R1 of([])", || { let d = Text::of(&empty); (d.len(), d.len_lines()) });
    t("R2 of(['']) == empty", || Text::of(&[""]) == Text::empty());
    t("R3 of(['a\\nb'])", || { let d = Text::of(&["a\nb"]); (d.len(), d.len_lines(), d.to_string(), l(d.line_at(2))) });
    t("R4 of(['a\\r\\nb'])", || { let d = Text::of(&["a\r\nb"]); (d.len(), d.len_lines(), l(d.line(1))) });
    t("R5 of(['a\\u2028b','c'])", || { let d = Text::of(&["a\u{2028}b", "c"]); (d.len(), d.len_lines(), l(d.line(1)), l(d.line(2)), l(d.line(3))) });
    t("R5b line_at(1) in a\\u2028b", || { let d = Text::of(&["a\u{2028}b"]); (l(d.line_at(1)), l(d.line_at(2))) });
    t("R6 of(['a\\u0085b\\u000bc\\u000cd\\re'])", || { let d = Text::of(&["a\u{0085}b\u{000b}c\u{000c}d\re"]); (d.len(), d.len_lines(), (1..=d.len_lines()).map(|n| l(d.line(n))).collect::<Vec<_>>()) });
    t("R6b from_str lone CR", || { let d = Text::from_str("a\rb"); (d.len(), d.len_lines(), l(d.line(1)), l(d.line(2))) });
    t("R6c from_str CRLF", || { let d = Text::from_str("a\r\nb"); (d.len(), d.len_lines(), l(d.line(1)), l(d.line(2)), l(d.line_at(1)), l(d.line_at(2)), l(d.line_at(3)), d.to_string()) });
    t("R7 of(['a',''])", || { let d = Text::of(&["a", ""]); (d.len(), d.len_lines(), l(d.line(2))) });
    t("R8 empty", || (Text::empty().len(), Text::empty().len_lines(), Text::empty().is_empty()));
    t("R9 of(['a😀b'])", || { let d = Text::of(&["a😀b"]); (d.len(), l(d.line_at(2)), d.slice_string(1, 3), d.slice_string(1, 2), d.slice_string(3, 100)) });
    t("R9b of(['😀','x']) line2", || { let d = Text::of(&["😀", "x"]); (d.len(), l(d.line(2)), l(d.line_at(2))) });
    t("R10 line(0)", || l(doc0.line(0)));
    t("R11 line(lines+1)", || l(doc0.line(201)));
    t("R13 line_at(len+1)", || l(doc0.line_at(doc0.len() + 1)));
    t("R13b line_at(len)", || l(doc0.line_at(doc0.len())));
    t("R14 line_at on break", || { let d = Text::of(&["ab", "cd"]); (l(d.line_at(2)), l(d.line_at(3))) });
    t("R15 slice_string (no lineSep param)", || Text::of(&["a", "b", "c"]).slice_string(0, 5));
    t("R15d slice_string of CRLF doc", || Text::from_str("a\r\nb\r\nc").slice_string(0, 7));
    t("R16 slice_string clip", || { let d = Text::of(&["abc"]); (d.slice_string(0, 2), d.slice_string(2, 1), d.slice_string(1, 100)) });
    t("R16b slice(5..0) len", || doc0.slice(5..0).len());
    #[allow(clippy::reversed_empty_ranges)]
    t("R16e slice structure", || { let s = Text::of(&["abc", "de"]).slice(1..5); (s.len(), s.len_lines(), s.to_string()) });
    t("R17 replace from>to", || Text::of(&["abcdef"]).replace(4..2, "X").to_string());
    t("R17b replace out of range", || Text::of(&["abc"]).replace(10..20, "X").to_string());
    t("R17d replace multi-line", || { let d = Text::of(&["abc"]).replace(1..2, "X\nY"); (d.len_lines(), d.to_string()) });
    t("R18 eq", || (Text::of(&["a", "b"]) == Text::of(&["a\nb"]), Text::from_str("a\r\nb") == Text::of(&["a", "b"])));
    t("R19 Display CRLF doc", || Text::from_str("ab\r\n\r\nc").to_string());
    t("R19c Debug", || format!("{:?}", Text::of(&["ab", "c"])));
    t("R20 char_to_line clamp", || { let d = Text::of(&["ab", "cd"]); (d.char_to_line(2), d.char_to_line(3), d.char_to_line(100)) });
    t("R20b line_to_char clamp", || { let d = Text::of(&["ab", "cd"]); (d.line_to_char(0), d.line_to_char(1), d.line_to_char(100)) });
    t("R20c line_by_index clamp", || { let d = Text::of(&["ab", "cd"]); l(d.line_by_index(100)) });
    t("R20d char_to_byte/byte_to_char", || { let d = Text::of(&["a😀b"]); (d.char_to_byte(2), d.char_to_byte(100), d.byte_to_char(2), d.byte_to_char(5), d.byte_to_char(100), d.len_bytes()) });
    t("R22 Line.length stored", || { let mut x = Text::of(&["abc", "de"]).line(2); x.length = 99; (x.length, x.to - x.from) });
    t("R23 append empty", || { let a = Text::of(&["x"]); (a.append(&Text::empty()).to_string(), Text::empty().append(&a).to_string()) });
    t("R24 as_rope", || Text::of(&["ab", "c"]).as_rope().len_lines());
    t("R25 From<&str>/From<String>", || (Text::from("a\nb").len_lines(), Text::from(String::from("a\u{2029}b")).len_lines()));
    t("R26 default", || Text::default() == Text::empty());
    t("R27 len vs len_chars", || { let d = Text::of(&["a😀b"]); (d.len(), d.len_chars()) });
}
