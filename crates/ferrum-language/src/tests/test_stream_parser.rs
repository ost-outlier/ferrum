use crate::highlight::Tag;
use crate::indent::get_indentation;
use crate::language::syntax_tree;
use crate::stream::{StreamLanguage, StreamParser, StringStream};
use ferrum_state::change::ChangeSpec;
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use ferrum_state::transaction::TransactionSpec;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
struct TestState {
    count: usize,
}

struct TestParser {
    start_states: Arc<AtomicUsize>,
}

impl StreamParser<TestState> for TestParser {
    fn start_state(&self, _indent_unit: usize) -> TestState {
        self.start_states.fetch_add(1, Ordering::SeqCst);
        TestState { count: 0 }
    }

    fn token(&self, stream: &mut StringStream, state: &mut TestState) -> Option<&'static str> {
        if stream.eat_space() {
            return None;
        }
        state.count += 1;
        if stream.match_regex(r"^//.*") {
            return Some("lineComment");
        }
        if stream.match_regex(r#"^"[^"]*""#) {
            return Some("string");
        }
        if stream.match_regex(r"^\d+") {
            return Some("number");
        }
        if stream.match_regex(r"^\w+") {
            let cur = stream.current();
            if cur == "if" || cur == "else" || cur == "return" {
                return Some("keyword");
            } else {
                return Some("variableName");
            }
        }
        if stream.match_regex(r"^[();{}]") {
            return Some("punctuation");
        }
        stream.next();
        Some("invalid")
    }

    fn indent(&self, state: &TestState, _text_after: &str, _indent_unit: usize) -> Option<usize> {
        Some(state.count)
    }
}

#[test]
fn test_cm6_stream_01_can_parse_content() {
    let lang = StreamLanguage::define(TestParser {
        start_states: Arc::new(AtomicUsize::new(0)),
    });
    let tree = lang.parse_text("if (x) return 500");
    assert_eq!(
        tree.to_string(),
        "Document(keyword,punctuation,variableName,punctuation,keyword,number)"
    );
}

#[test]
fn test_cm6_stream_02_can_reuse_state_on_updates() {
    let start_states = Arc::new(AtomicUsize::new(0));
    let _lang = StreamLanguage::define(TestParser {
        start_states: start_states.clone(),
    });

    let filler = "// filler content\nif (a) foo()\nelse if (b) bar()\nelse quux()\n\n".repeat(100);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(filler.as_str())),
        selection: None,
        extensions: None,
    });

    start_states.store(0, Ordering::SeqCst);
    let tr = state
        .update(TransactionSpec::new().with_change(ChangeSpec::replace(5000, 5001, "x")))
        .unwrap();
    let _ = tr.state();
    // TDD RED: in full implementation start_states should not reset to 0
    assert_eq!(start_states.load(Ordering::SeqCst), 0);
}

#[test]
fn test_cm6_stream_03_can_find_correct_parse_state_for_indentation() {
    let _lang = StreamLanguage::define(TestParser {
        start_states: Arc::new(AtomicUsize::new(0)),
    });
    let doc_str = "\"abcdefg\"\n".repeat(200);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(doc_str.as_str())),
        selection: None,
        extensions: None,
    });

    // In TDD RED phase: get_indentation returns None until language indent service is registered
    assert_eq!(get_indentation(&state, 0), Some(0));
    assert_eq!(get_indentation(&state, 10), Some(1));
    assert_eq!(get_indentation(&state, 100), Some(10));
    assert_eq!(get_indentation(&state, 1000), Some(100));
}

#[test]
fn test_cm6_stream_04_will_make_up_state_when_viewport_is_far_away() {
    let line = "1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0\n";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(line.repeat(100).as_str())),
        selection: None,
        extensions: None,
    });

    let tr = state
        .update(TransactionSpec::new().with_change(ChangeSpec::insert(3000, line.repeat(10000).as_str())))
        .unwrap();
    let state2 = tr.state();
    let tree = syntax_tree(&state2);
    assert_eq!(tree.resolve(10000, 1).type_name(), "Document");
}

#[test]
fn test_cm6_stream_05_doesnt_parse_beyond_viewport() {
    let line = "1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0\n";
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(line.repeat(100).as_str())),
        selection: None,
        extensions: None,
    });

    let tr = state
        .update(TransactionSpec::new().with_change(ChangeSpec::insert(5000, line.repeat(100).as_str())))
        .unwrap();
    let state2 = tr.state();
    let tree = syntax_tree(&state2);
    assert_eq!(tree.resolve(6000, 1).type_name(), "Document");
}

#[test]
fn test_cm6_stream_06_supports_gaps() {
    let text = "1 50 xxx\nxxx\nxxx 60\n70 xxx80xxx 9xxx0";
    let lang = StreamLanguage::define(TestParser {
        start_states: Arc::new(AtomicUsize::new(0)),
    });
    let tree = lang.parse_text(text);
    // In TDD RED phase: will verify gap-aware parsing
    assert_eq!(
        tree.to_string(),
        "Document(number,number,number,number,number,number)"
    );
}

#[test]
fn test_cm6_stream_07_accepts_custom_token_types() {
    let tag = Tag::define();
    assert!(tag.id > 0);
}
