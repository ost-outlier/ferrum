use crate::indent::{get_indent_unit, indent_string, INDENT_UNIT};
use crate::tree::{ParseContext, Parser, SyntaxNode, SyntaxTree};
use ferrum_state::change::{ChangeSet, ChangeSpec};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use std::sync::Arc;

struct MockJsParser;

impl Parser for MockJsParser {
    fn parse(&self, text: &str, _fragments: &[SyntaxTree], _ranges: &[(usize, usize)]) -> SyntaxTree {
        if text.starts_with("let x = 10") {
            let mut script = SyntaxNode::new("Script", 0, text.len());
            let mut var_decl = SyntaxNode::new("VariableDeclaration", 0, text.len());
            var_decl.children.push(SyntaxNode::new("let", 0, 3));
            var_decl.children.push(SyntaxNode::new("VariableDefinition", 4, 5));
            var_decl.children.push(SyntaxNode::new("Equals", 6, 7));
            var_decl.children.push(SyntaxNode::new("Number", 8, 10));
            script.children.push(var_decl);
            SyntaxTree::new(script, text.len())
        } else if text.starts_with("let") {
            let mut script = SyntaxNode::new("Script", 0, text.len());
            let mut var_decl = SyntaxNode::new("VariableDeclaration", 0, text.len());
            var_decl.children.push(SyntaxNode::new("let", 0, 3));
            var_decl.children.push(SyntaxNode::new("ObjectPattern", 4, 14));
            script.children.push(var_decl);
            SyntaxTree::new(script, text.len())
        } else {
            let script = SyntaxNode::new("Script", 0, text.len());
            SyntaxTree::new(script, text.len())
        }
    }
}

fn p_context(doc: Text) -> ParseContext {
    let len = doc.len_chars();
    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: None,
        extensions: None,
    });
    ParseContext::create(Arc::new(MockJsParser), state, Some((0, len)))
}

#[test]
fn test_cm6_parse_context_can_parse_a_document() {
    let mut cx = p_context(Text::from("let x = 10"));
    cx.work(100_000_000);
    assert_eq!(
        cx.tree.to_string(),
        "Script(VariableDeclaration(let,VariableDefinition,Equals,Number))"
    );
}

#[test]
fn test_cm6_parse_context_can_parse_incrementally() {
    let base_lines = [
        "const {readFile} = require(\"fs\");",
        "readFile(\"package.json\", \"utf8\", (err, data) => {",
        "  console.log(data);",
        "});",
    ];
    let mut lines = Vec::new();
    for i in 0..5000 {
        lines.push(base_lines[i % base_lines.len()]);
    }
    let doc = Text::of(&lines);
    let mut cx = p_context(doc.clone());
    cx.work(100_000_000);
    assert_eq!(cx.tree.length, doc.len_chars());

    let change = ChangeSet::of_spec(ChangeSpec::replace(0, 5, "let"), doc.len_chars());
    let new_doc = change.apply(&doc);
    let new_state = EditorState::create(EditorStateConfig {
        doc: Some(new_doc.clone()),
        selection: None,
        extensions: None,
    });
    cx = cx.changes(&change, new_state);
    cx.work(50);
    assert_eq!(cx.tree.length, new_doc.len_chars());
    assert!(cx.tree.to_string().starts_with("Script(VariableDeclaration(let,"));
}

#[test]
fn test_cm6_indent_tracks_indent_units() {
    let s0 = EditorState::create(EditorStateConfig::default());
    assert_eq!(get_indent_unit(&s0), 2);
    assert_eq!(indent_string(&s0, 4), "    ");

    let s1 = EditorState::create(EditorStateConfig {
        extensions: Some(vec![INDENT_UNIT.of("   ".to_string())]),
        ..Default::default()
    });
    assert_eq!(get_indent_unit(&s1), 3);
    assert_eq!(indent_string(&s1, 4), "    ");

    let s2 = EditorState::create(EditorStateConfig {
        extensions: Some(vec![
            INDENT_UNIT.of("\t".to_string()),
            EditorState::tab_size().of(8),
        ]),
        ..Default::default()
    });
    assert_eq!(get_indent_unit(&s2), 8);
    assert_eq!(indent_string(&s2, 16), "\t\t");

    let s3 = EditorState::create(EditorStateConfig {
        extensions: Some(vec![INDENT_UNIT.of("　".to_string())]),
        ..Default::default()
    });
    assert_eq!(get_indent_unit(&s3), 1);
    assert_eq!(indent_string(&s3, 2), "　　");
}

#[test]
#[should_panic(expected = "Invalid indent unit")]
fn test_cm6_indent_errors_for_empty_unit() {
    let _ = EditorState::create(EditorStateConfig {
        extensions: Some(vec![INDENT_UNIT.of("".to_string())]),
        ..Default::default()
    });
}

#[test]
#[should_panic(expected = "Invalid indent unit")]
fn test_cm6_indent_errors_for_mixed_unit() {
    let _ = EditorState::create(EditorStateConfig {
        extensions: Some(vec![INDENT_UNIT.of("\t ".to_string())]),
        ..Default::default()
    });
}

#[test]
#[should_panic(expected = "Invalid indent unit")]
fn test_cm6_indent_errors_for_non_whitespace_unit() {
    let _ = EditorState::create(EditorStateConfig {
        extensions: Some(vec![INDENT_UNIT.of("hello".to_string())]),
        ..Default::default()
    });
}
