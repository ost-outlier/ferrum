use ferrum_state::*;
use std::panic;
fn t<F: FnOnce() -> String + panic::UnwindSafe>(label: &str, f: F) {
    match panic::catch_unwind(f) { Ok(v) => println!("{} => {}", label, v), Err(e) => println!("{} => PANIC {}", label, e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()) }
}
fn main() {
    panic::set_hook(Box::new(|_| {}));
    t("W01 find_word_break('foo bar',3,true)", || find_word_break("foo bar", 3, true).to_string());
    t("W02 find_word_break('foo bar',0,true)", || find_word_break("foo bar", 0, true).to_string());
    t("W03 find_word_break('foo bar',4,false)", || find_word_break("foo bar", 4, false).to_string());
    t("W04 find_word_break('foo  ..bar',3,true)", || find_word_break("foo  ..bar", 3, true).to_string());
    t("W05 find_word_break('ab',5,false) pos>len", || find_word_break("ab", 5, false).to_string());
    t("W06 find_word_break('e\\u0301e',0,true)", || find_word_break("e\u{301}e", 0, true).to_string());
    t("S01 find_subword_break('fooBarBaz',0,true)", || find_subword_break("fooBarBaz", 0, true).to_string());
    t("S02 find_subword_break('fooBarBaz',9,false)", || find_subword_break("fooBarBaz", 9, false).to_string());
    t("S03 find_subword_break('HTMLParser',0,true)", || find_subword_break("HTMLParser", 0, true).to_string());
    t("S04 find_subword_break('foo_bar',0,true)", || find_subword_break("foo_bar", 0, true).to_string());
    t("S05 find_subword_break('ab',5,false) pos>len", || find_subword_break("ab", 5, false).to_string());
    t("S06 find_subword_break('  foo',0,true)", || find_subword_break("  foo", 0, true).to_string());
}
