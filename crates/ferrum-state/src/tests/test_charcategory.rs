use crate::char::CharCategory;
use crate::state::{EditorState, EditorStateConfig};

#[test]
fn test_cm6_char_categorizer_categorises_into_alphanumeric() {
    let state = EditorState::create(EditorStateConfig::default());
    let categorizer = state.char_categorizer(0);
    assert_eq!(categorizer("1"), CharCategory::Word);
    assert_eq!(categorizer("a"), CharCategory::Word);
}

#[test]
fn test_cm6_char_categorizer_categorises_into_whitespace() {
    let state = EditorState::create(EditorStateConfig::default());
    let categorizer = state.char_categorizer(0);
    assert_eq!(categorizer(" "), CharCategory::Space);
}

#[test]
fn test_cm6_char_categorizer_categorises_into_other() {
    let state = EditorState::create(EditorStateConfig::default());
    let categorizer = state.char_categorizer(0);
    assert_eq!(categorizer("/"), CharCategory::Other);
    assert_eq!(categorizer("<"), CharCategory::Other);
}
