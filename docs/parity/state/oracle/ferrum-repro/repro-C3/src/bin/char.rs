use ferrum_state::*;
use std::panic;

fn t<F: FnOnce() -> String + panic::UnwindSafe>(label: &str, f: F) {
    match panic::catch_unwind(f) {
        Ok(v) => println!("{} => {}", label, v),
        Err(e) => {
            let msg = e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default();
            println!("{} => PANIC {}", label, msg)
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));
    t("C02 fcb('e\\u0301x',0,true)", || find_cluster_break("e\u{301}x", 0, true).to_string());
    t("C04 fcb('e\\u0301x',2,false)", || find_cluster_break("e\u{301}x", 2, false).to_string());
    t("C08 fcb('\\u{1F600}',0) (chars)", || find_cluster_break("\u{1F600}", 0, true).to_string());
    t("C12 fcb('ab',5) pos>len fwd", || find_cluster_break("ab", 5, true).to_string());
    t("C13 fcb('ab',5,false) pos>len bwd", || find_cluster_break("ab", 5, false).to_string());
    t("C14 fcb('ab',2) at end fwd", || find_cluster_break("ab", 2, true).to_string());
    t("C15 fcb('ab',0,false)", || find_cluster_break("ab", 0, false).to_string());
    t("C16 fcb('',0)", || find_cluster_break("", 0, true).to_string());
    t("C17 fcb('',0,false)", || find_cluster_break("", 0, false).to_string());
    t("C21 fcb flags 0 (chars)", || find_cluster_break("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 0, true).to_string());
    t("C22 fcb flags bwd from 4 (chars)", || find_cluster_break("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 4, false).to_string());
    t("C23 fcb flags from 1 (mid flag, chars)", || find_cluster_break("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 1, true).to_string());
    t("C24 fcb flags bwd from 3 (mid flag, chars)", || find_cluster_break("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 3, false).to_string());
    t("K01 count_column('\\tab',4,None)", || count_column("\tab", 4, None).to_string());
    t("K02 count_column('ab\\tc',4,None)", || count_column("ab\tc", 4, None).to_string());
    t("K03 count_column('a\\t\\t',2,None)", || count_column("a\t\t", 2, None).to_string());
    t("K04 count_column('abcd',4,Some(2))", || count_column("abcd", 4, Some(2)).to_string());
    t("K05 count_column('abcd',4,Some(10))", || count_column("abcd", 4, Some(10)).to_string());
    t("K07 count_column('\\u{1F600}b',4,None)", || count_column("\u{1F600}b", 4, None).to_string());
    t("K08 count_column('\\t',0,None)", || count_column("\t", 0, None).to_string());
    t("K10 count_column('\\u4E2Dx',4,None)", || count_column("\u{4E2D}x", 4, None).to_string());
    t("J01 find_column('\\tab',5,4,false)", || format!("{:?}", find_column("\tab", 5, 4, false)));
    t("J02 find_column('\\tab',2,4,false)", || format!("{:?}", find_column("\tab", 2, 4, false)));
    t("J03 find_column('ab',5,4,false)", || format!("{:?}", find_column("ab", 5, 4, false)));
    t("J04 find_column('ab',5,4,true)", || format!("{:?}", find_column("ab", 5, 4, true)));
    t("J05 find_column('ab',2,4,true)", || format!("{:?}", find_column("ab", 2, 4, true)));
    t("J06 find_column('\\u{1F600}b',1,4,false) (chars)", || format!("{:?}", find_column("\u{1F600}b", 1, 4, false)));
    t("J07 find_column('e\\u0301b',1,4,false)", || format!("{:?}", find_column("e\u{301}b", 1, 4, false)));
    t("J08 find_column('ab',0,4,true)", || format!("{:?}", find_column("ab", 0, 4, true)));
    t("J10 find_column('\\t',1,0,false)", || format!("{:?}", find_column("\t", 1, 0, false)));
    t("J11 find_column('',0,4,true)", || format!("{:?}", find_column("", 0, 4, true)));
    t("J12 find_column('',1,4,true)", || format!("{:?}", find_column("", 1, 4, true)));
    let st = EditorState::create(EditorStateConfig::default());
    let cat = st.char_categorizer(0);
    let cases: &[(&str, &str)] = &[("empty", ""), ("space", " "), ("tab", "\t"), ("NBSP", "\u{a0}"), ("NEL U+0085", "\u{85}"), ("BOM U+FEFF", "\u{feff}"), ("ZWSP U+200B", "\u{200b}"), ("MVS U+180E", "\u{180e}"), ("ideographic space", "\u{3000}"), ("LS U+2028", "\u{2028}"),
        ("a", "a"), ("1", "1"), ("_", "_"), ("$", "$"), ("-", "-"), ("Arabic-Indic 3 U+0663", "\u{663}"), ("Devanagari 3 U+0969", "\u{969}"), ("Roman numeral U+2163", "\u{2163}"), ("superscript 2", "\u{b2}"), ("fraction 1/2", "\u{bd}"),
        ("combining acute alone", "\u{301}"), ("e+acute", "e\u{301}"), ("space+acute", " \u{301}"), ("acute+space", "\u{301} "), ("Devanagari virama", "\u{94d}"), ("Devanagari vowel sign i", "\u{93f}"), ("CJK", "\u{4e2d}"), ("Hiragana", "\u{3042}"), ("Hangul", "\u{d55c}"),
        ("emoji", "\u{1F600}"), ("emoji ZWJ seq", "\u{1F468}\u{200d}\u{1F469}"), ("flag FR", "\u{1F1EB}\u{1F1F7}"), ("ZWJ", "\u{200d}"), ("a+space (two chars)", "a "), (" a (space first)", " a"), ("/a", "/a"), ("sharp s", "\u{df}"), ("ordinal a", "\u{aa}"), ("circled A", "\u{24b6}"), ("math bold A U+1D400", "\u{1D400}")];
    for (l, s) in cases {
        println!("G {} => {:?}", l, cat(s));
    }
    println!("UNICODE_VERSION {:?}", char::UNICODE_VERSION);
}
