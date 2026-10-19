use crate::text::Text;

#[test]
fn test_cm6_text_handles_basic_replacement() {
    let doc = Text::from("one\ntwo\nthree");
    let replaced = doc.replace(2..5, "foo\nbar");
    assert_eq!(replaced.to_string(), "onfoo\nbarwo\nthree");
}

#[test]
fn test_cm6_text_can_append_documents() {
    let doc1 = Text::from("one\ntwo\nthree");
    let doc2 = Text::from("!\nok");
    let appended = doc1.append(&doc2);
    assert_eq!(appended.to_string(), "one\ntwo\nthree!\nok");
}

#[test]
fn test_cm6_text_preserves_length() {
    let line = "1234567890".repeat(10);
    let lines: Vec<&str> = vec![&line; 200];
    let text0 = lines.join("\n");
    let doc0 = Text::from(text0.as_str());
    assert_eq!(doc0.len_chars(), text0.chars().count());
}

#[test]
fn test_cm6_text_handles_deleting_at_start() {
    let line = "1234567890".repeat(10);
    let mut lines: Vec<String> = vec![line.clone(); 199];
    lines.push(format!("{}!", line));
    let text0 = lines.join("\n");
    let doc = Text::from(text0.as_str());
    let replaced = doc.replace(0..9500, "");
    let expected = format!("{}!", &text0[9500..text0.len() - 1]);
    assert_eq!(replaced.to_string(), expected);
}

#[test]
fn test_cm6_text_can_handle_deleting_the_entire_document() {
    let doc = Text::from("one\ntwo\nthree");
    let empty = doc.replace(0..doc.len_chars(), "");
    assert_eq!(empty.to_string(), "");
    assert_eq!(empty.len_chars(), 0);
}

#[test]
fn test_cm6_text_can_build_up_a_doc_by_repeated_appending() {
    let mut doc = Text::from("");
    let mut text = String::new();
    for i in 1..200 {
        let add = format!("newtext{} ", i);
        doc = doc.replace(doc.len_chars()..doc.len_chars(), &add);
        text.push_str(&add);
    }
    assert_eq!(doc.to_string(), text);
}

#[test]
fn test_cm6_text_can_be_compared() {
    let doc1 = Text::from("foo\nbar");
    let doc2 = Text::from("foo\nbar");
    let doc3 = Text::from("foo\nbaz");
    assert_eq!(doc1, doc2);
    assert_ne!(doc1, doc3);
}

#[test]
fn test_cm6_text_can_get_line_info_by_line_number() {
    let doc = Text::from("line 1\nline 2\nline 3");
    assert_eq!(doc.len_lines(), 3);

    // Line numbers in CM6 are 1-based (lines 1..=3)
    let l1 = doc.line(0);
    assert_eq!(l1.number, 1);
    assert_eq!(l1.from, 0);
    assert_eq!(l1.to, 6);
    assert_eq!(doc.slice_to_string(l1.from..l1.to), "line 1");

    let l2 = doc.line(1);
    assert_eq!(l2.number, 2);
    assert_eq!(l2.from, 7);
    assert_eq!(l2.to, 13);
    assert_eq!(doc.slice_to_string(l2.from..l2.to), "line 2");
}

#[test]
fn test_cm6_text_can_get_line_info_by_position() {
    let doc = Text::from("abc\ndefgh\nijk");
    let l0 = doc.line_at(1);
    assert_eq!(l0.number, 1);
    assert_eq!(l0.from, 0);
    assert_eq!(l0.to, 3);

    let l1 = doc.line_at(5);
    assert_eq!(l1.number, 2);
    assert_eq!(l1.from, 4);
    assert_eq!(l1.to, 9);
}

#[test]
fn test_cm6_text_can_retrieve_pieces_of_text() {
    let doc = Text::from("The quick brown fox jumps over the lazy dog");
    assert_eq!(doc.slice_to_string(4..9), "quick");
    assert_eq!(doc.slice_to_string(10..15), "brown");
}
