use crate::heightmap::{HeightMap, HeightOracle, QueryType};
use ferrum_state::text::Text;

#[test]
fn test_cm6_heightmap_starts_empty() {
    let empty = HeightMap::empty();
    assert_eq!(empty.length, 0);
    assert_eq!(empty.size, 1);
    assert_eq!(empty.height, 0.0);
}

#[test]
fn test_cm6_heightmap_query_line_at() {
    let map = HeightMap::empty();
    let oracle = HeightOracle::default();
    let info = map.line_at(0, QueryType::ByPos, &oracle);
    assert_eq!(info.top, 0.0);
}

#[test]
fn test_cm6_heightmap_grows_to_match_document_length() {
    let doc = Text::from("xxxxxxxxxx\nxxxxxxxxxx\nxxxxxxxx");
    let _oracle = HeightOracle::default();
    // In TDD RED phase: HeightMap must be updated with the document length (30 chars)
    let map = HeightMap {
        length: 0, // Stub: will fail until heightmap builder from document is implemented
        height: 0.0,
        size: 1,
    };
    assert_eq!(map.length, doc.len_chars(), "HeightMap length must match document length");
}

#[test]
fn test_cm6_heightmap_computes_line_heights_and_positions() {
    let _doc = Text::from("line 1\nline 2\nline 3");
    let oracle = HeightOracle {
        line_height: 20.0,
        char_width: 8.0,
    };
    let map = HeightMap::empty();
    let line2_info = map.line_at(7, QueryType::ByPos, &oracle); // 'l' in "line 2"
    // Second line should start at top = 20.0 and have height = 20.0
    assert_eq!(line2_info.top, 20.0, "Line 2 top coordinate must equal line_height");
    assert_eq!(line2_info.height, 20.0, "Line 2 height must equal line_height");
}
