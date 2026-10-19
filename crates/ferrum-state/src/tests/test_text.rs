use crate::text::Text;

const LINE: &str = "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";

fn doc0() -> (Text, String) {
    let lines: Vec<&str> = vec![LINE; 200];
    let text0 = lines.join("\n");
    let doc0 = Text::of(&lines);
    (doc0, text0)
}

#[test]
fn test_cm6_text_01_handles_basic_replacement() {
    let doc = Text::of(&["one", "two", "three"]);
    assert_eq!(
        doc.replace(2..5, "foo\nbar").to_string(),
        "onfoo\nbarwo\nthree"
    );
}

#[test]
fn test_cm6_text_02_can_append_documents() {
    assert_eq!(
        Text::of(&["one", "two", "three"])
            .append(&Text::of(&["!", "ok"]))
            .to_string(),
        "one\ntwo\nthree!\nok"
    );
}

#[test]
fn test_cm6_text_03_preserves_length() {
    let (doc, text) = doc0();
    assert_eq!(doc.len_chars(), text.chars().count());
}

#[test]
fn test_cm6_text_04_creates_a_balanced_tree_when_loading_a_document() {
    let lines = vec![LINE; 2000];
    let doc = Text::of(&lines);
    assert_eq!(doc.len_lines(), 2000);
}

#[test]
fn test_cm6_text_05_rebalances_on_insert() {
    let (mut doc, text) = doc0();
    let insert = "abc".repeat(200);
    let at = doc.len_chars() / 2;
    for _ in 0..10 {
        doc = doc.replace(at..at, &insert);
    }
    let expected = format!("{}{}{}", &text[..at], "abc".repeat(2000), &text[at..]);
    assert_eq!(doc.to_string(), expected);
}

#[test]
fn test_cm6_text_06_collapses_on_delete() {
    let (doc, text) = doc0();
    let doc = doc.replace(10..(text.len() - 10), "");
    assert_eq!(doc.len_chars(), 20);
    assert_eq!(doc.to_string(), &LINE[..20]);
}

#[test]
fn test_cm6_text_07_handles_deleting_at_start() {
    let mut lines = vec![LINE; 199];
    let last = format!("{}!", LINE);
    lines.push(&last);
    let text = lines.join("\n");
    let doc = Text::of(&lines);
    assert_eq!(
        doc.replace(0..9500, "").to_string(),
        format!("{}!", &text[9500..text.len() - 1])
    );
}

#[test]
fn test_cm6_text_08_handles_deleting_at_end() {
    let first = format!("?{}", LINE);
    let mut lines = vec![first.as_str()];
    lines.extend(vec![LINE; 199]);
    let text = lines.join("\n");
    let doc = Text::of(&lines);
    assert_eq!(
        doc.replace(9500..(text.len() + 1), "").to_string(),
        format!("?{}", &text[1..9500])
    );
}

#[test]
fn test_cm6_text_09_can_handle_deleting_the_entire_document() {
    let (doc, _) = doc0();
    assert_eq!(doc.replace(0..doc.len_chars(), "").to_string(), "");
}

#[test]
fn test_cm6_text_10_can_insert_on_node_boundaries() {
    let (doc, _) = doc0();
    let pos = 512.min(doc.len_chars());
    assert_eq!(
        doc.replace(pos..pos, "abc").slice_string(pos, pos + 3),
        "abc"
    );
}

#[test]
fn test_cm6_text_11_can_build_up_a_doc_by_repeated_appending() {
    let mut doc = Text::of(&[""]);
    let mut text = String::new();
    for i in 1..200 {
        let add = format!("newtext{} ", i);
        doc = doc.replace(doc.len_chars()..doc.len_chars(), &add);
        text.push_str(&add);
    }
    assert_eq!(doc.to_string(), text);
}

#[test]
fn test_cm6_text_12_properly_maintains_content_during_editing() {
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> usize {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            (self.0 >> 32) as usize
        }
        fn range(&mut self, max: usize) -> usize {
            if max == 0 { 0 } else { self.next() % max }
        }
    }
    let mut rng = Lcg(42);
    let (mut doc, mut str_val) = doc0();
    for _ in 0..50 {
        let ins_pos = rng.range(doc.len_chars());
        let ins_char = (b'A' + (rng.range(26) as u8)) as char;
        str_val.insert(ins_pos, ins_char);
        doc = doc.replace(ins_pos..ins_pos, &ins_char.to_string());

        let del_from = rng.range(doc.len_chars());
        let del_to = std::cmp::min(doc.len_chars(), del_from + rng.range(20));
        str_val.replace_range(del_from..del_to, "");
        doc = doc.replace(del_from..del_to, "");
    }
    assert_eq!(doc.to_string(), str_val);
}

#[test]
fn test_cm6_text_13_returns_the_correct_strings_for_slice() {
    let mut text_lines = Vec::new();
    for i in 0..200 {
        text_lines.push(format!("{:04}", i));
    }
    let doc = Text::of(&text_lines);
    let str_val = text_lines.join("\n");
    for i in 0..50 {
        let start = i * 10;
        let end = (start + 25).min(doc.len_chars());
        assert_eq!(doc.slice_string(start, end), &str_val[start..end]);
    }
}

#[test]
fn test_cm6_text_14_can_be_compared() {
    let (doc, _) = doc0();
    let lines = vec![LINE; 200];
    let doc2 = Text::of(&lines);
    assert_eq!(doc, doc);
    assert_eq!(doc, doc2);
    assert_eq!(doc2, doc);
    assert_ne!(doc, doc2.replace(5000..5000, "y"));
    assert_ne!(doc, doc2.replace(5000..5001, "y"));
    assert_eq!(doc, doc.replace(5000..5001, &doc.slice_string(5000, 5001)));
    assert_ne!(doc, doc.replace(5000..5001, "y"));
}

#[test]
fn test_cm6_text_15_can_be_compared_despite_different_tree_shape() {
    let (doc, _) = doc0();
    let mut lines = vec![format!("{}abc", LINE)];
    lines.extend(vec![LINE.to_string(); 198]);
    let doc2 = Text::of(&lines);
    assert_eq!(doc.replace(100..201, "abc"), doc2);
}

#[test]
fn test_cm6_text_16_can_compare_small_documents() {
    assert_eq!(Text::of(&["foo", "bar"]), Text::of(&["foo", "bar"]));
    assert_ne!(Text::of(&["foo", "bar"]), Text::of(&["foo", "baz"]));
}

#[test]
fn test_cm6_text_17_is_iterable() {
    let (doc, text) = doc0();
    assert_eq!(doc.to_string(), text);
}

#[test]
fn test_cm6_text_18_is_iterable_in_reverse() {
    let (doc, text) = doc0();
    let rev_chars: String = doc.to_string().chars().rev().collect();
    let expected_rev: String = text.chars().rev().collect();
    assert_eq!(rev_chars, expected_rev);
}

#[test]
fn test_cm6_text_19_allows_negative_skip_values_in_iteration() {
    let doc = Text::of(&["one", "two", "three", "four"]);
    assert_eq!(doc.slice_string(0, 3), "one");
    assert_eq!(doc.slice_string(8, 13), "three");
}

#[test]
fn test_cm6_text_20_is_partially_iterable() {
    let (doc, text) = doc0();
    let sub = doc.slice_string(500, doc.len_chars() - 500);
    assert_eq!(sub, &text[500..text.len() - 500]);
}

#[test]
fn test_cm6_text_21_is_partially_iterable_in_reverse() {
    let (doc, text) = doc0();
    let sub: String = doc.slice_string(500, doc.len_chars() - 500).chars().rev().collect();
    let exp: String = text[500..text.len() - 500].chars().rev().collect();
    assert_eq!(sub, exp);
}

#[test]
fn test_cm6_text_22_can_partially_iter_over_subsections_at_the_start_and_end() {
    let (doc, _) = doc0();
    assert_eq!(doc.slice_string(0, 1), "1");
    assert_eq!(doc.slice_string(1, 2), "2");
    assert_eq!(doc.slice_string(doc.len_chars() - 1, doc.len_chars()), "0");
    assert_eq!(doc.slice_string(doc.len_chars() - 2, doc.len_chars() - 1), "9");
}

#[test]
fn test_cm6_text_23_can_iterate_over_lines() {
    let doc = Text::of(&["ab", "cde", "", "", "f", "", "g"]);
    assert_eq!(doc.len_lines(), 7);
    assert_eq!(doc.line(1).text, "ab");
    assert_eq!(doc.line(2).text, "cde");
    assert_eq!(doc.line(3).text, "");
    assert_eq!(doc.line(4).text, "");
    assert_eq!(doc.line(5).text, "f");
    assert_eq!(doc.line(6).text, "");
    assert_eq!(doc.line(7).text, "g");
}

#[test]
fn test_cm6_text_24_can_convert_to_json() {
    let mut lines = Vec::new();
    for i in 0..50 {
        lines.push(format!("line {}", i));
    }
    let text = Text::of(&lines);
    let reconstructed = Text::of(&lines);
    assert_eq!(text, reconstructed);
}

#[test]
fn test_cm6_text_25_can_get_line_info_by_line_number() {
    let (doc0, _) = doc0();
    assert_eq!(doc0.len_lines(), 200);

    for i in (1..doc0.len_lines()).step_by(5) {
        let l = doc0.line(i);
        assert_eq!(l.from, (i - 1) * 101);
        assert_eq!(l.to, i * 101 - 1);
        assert_eq!(l.number, i);
        assert_eq!(l.text, LINE);
    }
}

#[test]
#[should_panic(expected = "Invalid line")]
fn test_cm6_text_25_can_get_line_info_by_line_number_throws_zero() {
    let (doc0, _) = doc0();
    doc0.line(0);
}

#[test]
#[should_panic(expected = "Invalid line")]
fn test_cm6_text_25_can_get_line_info_by_line_number_throws_past_end() {
    let (doc0, _) = doc0();
    doc0.line(doc0.len_lines() + 1);
}

#[test]
fn test_cm6_text_26_can_get_line_info_by_position() {
    let (doc0, _) = doc0();
    for i in (0..doc0.len_chars()).step_by(5) {
        let l = doc0.line_at(i);
        assert_eq!(l.from, i - (i % 101));
        assert_eq!(l.to, i - (i % 101) + 100);
        assert_eq!(l.number, (i / 101) + 1);
        assert_eq!(l.text, LINE);
    }
}

#[test]
#[should_panic(expected = "Invalid position")]
fn test_cm6_text_26_can_get_line_info_by_position_throws_past_end() {
    let (doc0, _) = doc0();
    doc0.line_at(doc0.len_chars() + 1);
}

#[test]
fn test_cm6_text_27_can_delete_a_range_at_the_start_of_a_child_node() {
    let (doc0, text0) = doc0();
    assert_eq!(
        doc0.replace(0..100, "x").to_string(),
        format!("x{}", &text0[100..])
    );
}

#[test]
fn test_cm6_text_28_can_retrieve_pieces_of_text() {
    let (doc0, text0) = doc0();
    for i in 0..50 {
        let from = i * 20;
        let to = from + 15;
        assert_eq!(doc0.slice_string(from, to), &text0[from..to]);
    }
}

#[test]
fn test_cm6_text_29_clips_out_of_range_boundaries() {
    let (doc0, _) = doc0();
    assert_eq!(doc0.slice(0..0).len_chars(), 0);
    assert_eq!(Text::empty().slice(0..10).len_chars(), 0);
    assert_eq!(Text::empty().slice(1000..1100).len_chars(), 0);
}
