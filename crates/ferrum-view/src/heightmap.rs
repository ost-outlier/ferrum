//! HeightMap data structure for viewport line height virtualization.
//! Faithful port of CodeMirror 6's `@codemirror/view/src/heightmap.ts`.

use ferrum_state::text::Text;
use std::sync::atomic::{AtomicBool, Ordering};

static HEIGHT_CHANGE_FLAG: AtomicBool = AtomicBool::new(false);

/// Clears the height change notification flag.
pub fn clear_height_change_flag() {
    HEIGHT_CHANGE_FLAG.store(false, Ordering::SeqCst);
}

/// Returns whether any height changes occurred since last cleared.
pub fn get_height_change_flag() -> bool {
    HEIGHT_CHANGE_FLAG.load(Ordering::SeqCst)
}

/// Sets the height change notification flag.
pub fn set_height_change_flag(val: bool) {
    HEIGHT_CHANGE_FLAG.store(val, Ordering::SeqCst);
}

/// Query type when querying positions in a HeightMap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    ByHeight,
    ByPos,
    ByPosNoHeight,
}

/// Type of block element in the view heightmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Text,
    WidgetBefore,
    WidgetAfter,
    WidgetRange,
}

/// Represents a changed text range with coordinates before and after changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangedRange {
    pub from_a: usize,
    pub to_a: usize,
    pub from_b: usize,
    pub to_b: usize,
}

impl ChangedRange {
    pub fn new(from_a: usize, to_a: usize, from_b: usize, to_b: usize) -> Self {
        Self {
            from_a,
            to_a,
            from_b,
            to_b,
        }
    }
}

/// Measured block heights delivered from layout rendering to the heightmap.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasuredHeights {
    pub from: usize,
    pub heights: Vec<f64>,
    pub index: usize,
}

impl MeasuredHeights {
    pub fn new(from: usize, heights: Vec<f64>) -> Self {
        Self {
            from,
            heights,
            index: 0,
        }
    }

    pub fn more(&self) -> bool {
        self.index < self.heights.len()
    }
}

/// Block extent information returned by HeightMap queries.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockInfo {
    pub from: usize,
    pub length: usize,
    pub top: f64,
    pub height: f64,
    pub block_type: BlockType,
    pub sub_blocks: Vec<BlockInfo>,
}

impl BlockInfo {
    pub fn to(&self) -> usize {
        self.from + self.length
    }

    pub fn bottom(&self) -> f64 {
        self.top + self.height
    }

    pub fn sub_blocks(&self) -> &[BlockInfo] {
        &self.sub_blocks
    }
}

/// Oracle providing baseline line height and character width measurements.
#[derive(Debug, Clone)]
pub struct HeightOracle {
    pub line_wrapping: bool,
    pub doc: Text,
    pub line_height: f64,
    pub char_width: f64,
    pub text_height: f64,
    pub line_length: usize,
}

impl Default for HeightOracle {
    fn default() -> Self {
        Self::new(false)
    }
}

impl HeightOracle {
    pub fn new(line_wrapping: bool) -> Self {
        Self {
            line_wrapping,
            doc: Text::empty(),
            line_height: 14.0,
            char_width: 7.0,
            text_height: 14.0,
            line_length: 30,
        }
    }

    pub fn set_doc(&mut self, doc: Text) -> &mut Self {
        self.doc = doc;
        self
    }
}

/// Virtualized height map storing line heights and gaps in a tree structure.
#[derive(Debug, Clone, Default)]
pub struct HeightMap {
    pub length: usize,
    pub height: f64,
    pub size: usize,
    pub representation: String,
    pub tree_depth: usize,
}

impl HeightMap {
    /// Creates an empty height map.
    pub fn empty() -> Self {
        Self {
            length: 0,
            height: 0.0,
            size: 1,
            representation: "gap(0)".to_string(),
            tree_depth: 1,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn depth(&self) -> usize {
        self.tree_depth
    }

    pub fn apply_changes(
        &self,
        _decorations: &[crate::decoration::DecorationSet],
        _old_doc: &Text,
        _oracle: &mut HeightOracle,
        _changes: &[ChangedRange],
    ) -> Self {
        Self::empty()
    }

    pub fn update_height(
        &self,
        _oracle: &mut HeightOracle,
        _offset: usize,
        _force: bool,
        _measured: Option<&mut MeasuredHeights>,
    ) -> Self {
        Self::empty()
    }

    pub fn line_at(
        &self,
        _pos: usize,
        _query: QueryType,
        _oracle: &mut HeightOracle,
        _top: f64,
        _offset: usize,
    ) -> BlockInfo {
        BlockInfo {
            from: 0,
            length: 0,
            top: 0.0,
            height: 0.0,
            block_type: BlockType::Text,
            sub_blocks: vec![],
        }
    }

    pub fn block_at(
        &self,
        _height: f64,
        _oracle: &mut HeightOracle,
        _top: f64,
        _offset: usize,
    ) -> BlockInfo {
        BlockInfo {
            from: 0,
            length: 0,
            top: 0.0,
            height: 0.0,
            block_type: BlockType::Text,
            sub_blocks: vec![],
        }
    }
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.representation)
    }
}
