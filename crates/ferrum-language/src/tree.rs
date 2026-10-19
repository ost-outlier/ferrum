//! Syntax tree representations and incremental parsing context.
//! Faithful port of CodeMirror 6 / Lezer syntax tree interfaces.

use ferrum_state::change::ChangeSet;
use ferrum_state::state::EditorState;
use std::sync::Arc;

/// A syntax node type descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeType {
    pub name: String,
    pub is_error: bool,
}

impl NodeType {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let is_error = name == "⚠" || name == "error";
        Self { name, is_error }
    }
}

/// A node within the syntax tree.
#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub node_type: Arc<NodeType>,
    pub from: usize,
    pub to: usize,
    pub children: Vec<SyntaxNode>,
}

impl SyntaxNode {
    pub fn new(name: impl Into<String>, from: usize, to: usize) -> Self {
        Self {
            node_type: Arc::new(NodeType::new(name)),
            from,
            to,
            children: Vec::new(),
        }
    }

    pub fn type_name(&self) -> &str {
        &self.node_type.name
    }

    pub fn first_child(&self) -> Option<&SyntaxNode> {
        self.children.first()
    }

    pub fn last_child(&self) -> Option<&SyntaxNode> {
        self.children.last()
    }

    pub fn resolve(&self, pos: usize, _assoc: i32) -> SyntaxNode {
        for child in &self.children {
            if child.from <= pos && child.to >= pos {
                return child.resolve(pos, _assoc);
            }
        }
        self.clone()
    }
}

/// An immutable, hierarchical syntax tree.
#[derive(Debug, Clone, Default)]
pub struct SyntaxTree {
    pub root: Option<SyntaxNode>,
    pub length: usize,
}

impl SyntaxTree {
    pub fn empty() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    pub fn new(root: SyntaxNode, length: usize) -> Self {
        Self {
            root: Some(root),
            length,
        }
    }

    pub fn resolve(&self, pos: usize, assoc: i32) -> SyntaxNode {
        if let Some(root) = &self.root {
            root.resolve(pos, assoc)
        } else {
            SyntaxNode::new("Document", 0, 0)
        }
    }

    pub fn iterate<F>(&self, mut f: F)
    where
        F: FnMut(&SyntaxNode),
    {
        if let Some(root) = &self.root {
            Self::walk_iterate(root, &mut f);
        }
    }

    fn walk_iterate<F>(node: &SyntaxNode, f: &mut F)
    where
        F: FnMut(&SyntaxNode),
    {
        f(node);
        for child in &node.children {
            Self::walk_iterate(child, f);
        }
    }
}

impl std::fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            write_node(root, f)
        } else {
            write!(f, "Document()")
        }
    }
}

fn write_node(node: &SyntaxNode, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", node.type_name())?;
    if !node.children.is_empty() {
        write!(f, "(")?;
        for (i, child) in node.children.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write_node(child, f)?;
        }
        write!(f, ")")?;
    }
    Ok(())
}

/// Trait implemented by language parsers.
pub trait Parser: Send + Sync {
    fn parse(&self, text: &str, fragments: &[SyntaxTree], ranges: &[(usize, usize)]) -> SyntaxTree;
}

/// Parsing context controlling incremental parsing budgets.
pub struct ParseContext {
    pub tree: SyntaxTree,
    pub parser: Arc<dyn Parser>,
    pub state: Arc<EditorState>,
    pub from: usize,
    pub to: usize,
}

impl ParseContext {
    pub fn create(
        parser: Arc<dyn Parser>,
        state: Arc<EditorState>,
        range: Option<(usize, usize)>,
    ) -> Self {
        let (from, to) = range.unwrap_or((0, state.doc().len_chars()));
        let tree = parser.parse(&state.doc().to_string(), &[], &[(from, to)]);
        Self {
            tree,
            parser,
            state,
            from,
            to,
        }
    }

    /// Performs work within the given budget. Returns true if parsing is complete.
    pub fn work(&mut self, _budget: usize) -> bool {
        true
    }

    /// Notifies the context of changes to the document.
    pub fn changes(&self, _changes: &ChangeSet, new_state: Arc<EditorState>) -> Self {
        let tree = self.parser.parse(&new_state.doc().to_string(), &[self.tree.clone()], &[(0, new_state.doc().len_chars())]);
        Self {
            tree,
            parser: self.parser.clone(),
            state: new_state,
            from: self.from,
            to: self.to,
        }
    }
}
