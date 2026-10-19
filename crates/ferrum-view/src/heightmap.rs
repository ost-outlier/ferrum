//! HeightMap data structure for viewport line height virtualization.
//! Faithful port of CodeMirror 6's `@codemirror/view/src/heightmap.ts`.

/// Query type when querying positions in a HeightMap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    ByHeight,
    ByPos,
}

/// Block extent information returned by HeightMap queries.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockInfo {
    pub from: usize,
    pub length: usize,
    pub top: f64,
    pub height: f64,
}

/// Oracle providing baseline line height and character width measurements.
#[derive(Debug, Clone)]
pub struct HeightOracle {
    pub line_height: f64,
    pub char_width: f64,
}

impl Default for HeightOracle {
    fn default() -> Self {
        Self {
            line_height: 14.0,
            char_width: 7.0,
        }
    }
}

/// Virtualized height map storing line heights and gaps in a tree structure.
#[derive(Debug, Clone, Default)]
pub struct HeightMap {
    pub length: usize,
    pub height: f64,
    pub size: usize,
}

impl HeightMap {
    /// Creates an empty height map.
    pub fn empty() -> Self {
        Self {
            length: 0,
            height: 0.0,
            size: 1,
        }
    }

    /// Queries block info at the given position or height.
    pub fn line_at(&self, _pos: usize, _query: QueryType, _oracle: &HeightOracle) -> BlockInfo {
        // TDD RED stub
        BlockInfo {
            from: 0,
            length: 0,
            top: 0.0,
            height: 0.0,
        }
    }
}
