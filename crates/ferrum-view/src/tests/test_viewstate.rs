use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use crate::viewstate::{PixelRect, ViewState};
use crate::HeightOracle;

fn make_doc(lines: usize) -> Text {
    let mut s = String::new();
    for i in 1..=lines {
        s.push_str(&format!("Line number {}\n", i));
    }
    Text::from_str(&s)
}

#[test]
fn test_cm6_viewstate_starts_with_initial_viewport() {
    let doc = make_doc(1000);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: None,
        extensions: None,
    });

    let mut oracle = HeightOracle::new(false);
    oracle.line_height = 20.0;
    oracle.char_width = 8.0;

    // Viewport of 600px height (about 30 lines)
    let pixel_viewport = PixelRect {
        left: 0.0,
        right: 800.0,
        top: 0.0,
        bottom: 600.0,
    };

    let view_state = ViewState::new(state.clone(), oracle, pixel_viewport);

    // Initial viewport should start at position 0
    assert_eq!(view_state.viewport.from, 0);

    // Viewport must NOT encompass the entire 1000 lines (which is ~15,000 chars)
    // It should only encompass the visible lines plus overscan margin
    let total_len = state.doc().len_chars();
    assert!(
        view_state.viewport.to < total_len / 2,
        "Viewport to ({}) should be much smaller than total ({})",
        view_state.viewport.to,
        total_len
    );
    assert_eq!(view_state.scroll_top, 0.0);
}

#[test]
fn test_cm6_viewstate_scroll_updates_viewport() {
    let doc = make_doc(1000);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: None,
        extensions: None,
    });

    let mut oracle = HeightOracle::new(false);
    oracle.line_height = 20.0;
    oracle.char_width = 8.0;

    let pixel_viewport = PixelRect {
        left: 0.0,
        right: 800.0,
        top: 0.0,
        bottom: 600.0,
    };

    let mut view_state = ViewState::new(state.clone(), oracle, pixel_viewport);

    // Scroll down to 4000px (line 200)
    view_state.set_scroll_top(4000.0);

    // Viewport start must have moved far past the beginning of the document
    assert!(
        view_state.viewport.from > 0,
        "Viewport from should be > 0 after scrolling"
    );

    let line_at_from = state.doc().char_to_line(view_state.viewport.from);
    assert!(
        line_at_from >= 100 && line_at_from <= 220,
        "Line at viewport from ({}) should be near scroll position (~200)",
        line_at_from
    );

    // Viewport range must remain bounded
    let visible_lines = state.doc().char_to_line(view_state.viewport.to) - line_at_from;
    assert!(
        visible_lines < 200,
        "Visible lines ({}) must be virtualized and small",
        visible_lines
    );
}

#[test]
fn test_cm6_viewstate_scroll_into_view_when_typing_at_end() {
    let doc = make_doc(1000);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: None,
        extensions: None,
    });

    let mut oracle = HeightOracle::new(false);
    oracle.line_height = 20.0;
    oracle.char_width = 8.0;

    let pixel_viewport = PixelRect {
        left: 0.0,
        right: 800.0,
        top: 0.0,
        bottom: 600.0,
    };

    let mut view_state = ViewState::new(state.clone(), oracle, pixel_viewport);
    assert_eq!(view_state.scroll_top, 0.0);

    // Target position at line 500 (approx Y = 10,000px)
    let line_500 = state.doc().line_by_index(500);
    view_state.scroll_into_view(line_500.from);

    // Scroll top must have advanced to bring line 500 into the viewport
    assert!(
        view_state.scroll_top > 8000.0,
        "Scroll top ({}) should have advanced to near 10,000",
        view_state.scroll_top
    );
    assert!(
        view_state.viewport.from <= line_500.from,
        "Viewport from ({}) should include target ({})",
        view_state.viewport.from,
        line_500.from
    );
    assert!(
        view_state.viewport.to >= line_500.from,
        "Viewport to ({}) should include target ({})",
        view_state.viewport.to,
        line_500.from
    );
}

#[test]
fn test_cm6_viewstate_coords_and_pos_at_coords_roundtrip() {
    let doc = make_doc(500);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: None,
        extensions: None,
    });

    let mut oracle = HeightOracle::new(false);
    oracle.line_height = 20.0;
    oracle.char_width = 8.0;

    let pixel_viewport = PixelRect {
        left: 0.0,
        right: 800.0,
        top: 0.0,
        bottom: 600.0,
    };

    let mut view_state = ViewState::new(state.clone(), oracle, pixel_viewport);

    // Check pos 0
    let coords_0 = view_state.coords_at_pos(0).expect("coords for 0");
    assert_eq!(coords_0.top, 0.0);
    assert_eq!(coords_0.left, 0.0);

    let back_pos = view_state.pos_at_coords(coords_0.left, coords_0.top + 5.0);
    assert_eq!(back_pos, 0);

    // Check line 10, col 5
    let line_10 = state.doc().line_by_index(10);
    let pos_10_5 = line_10.from + 5;
    let coords = view_state.coords_at_pos(pos_10_5).expect("coords");
    assert_eq!(coords.top, 10.0 * 20.0);
    assert_eq!(coords.left, 5.0 * 8.0);

    let roundtrip_pos = view_state.pos_at_coords(coords.left + 2.0, coords.top + 5.0);
    assert_eq!(roundtrip_pos, pos_10_5);
}

#[test]
fn test_cm6_viewstate_huge_document_virtualization() {
    // 50,000 lines document
    let doc = make_doc(50000);
    let state = EditorState::create(EditorStateConfig {
        doc: Some(doc),
        selection: None,
        extensions: None,
    });

    let mut oracle = HeightOracle::new(false);
    oracle.line_height = 20.0;
    oracle.char_width = 8.0;

    let pixel_viewport = PixelRect {
        left: 0.0,
        right: 800.0,
        top: 0.0,
        bottom: 600.0,
    };

    let start = std::time::Instant::now();
    let view_state = ViewState::new(state.clone(), oracle, pixel_viewport);
    let elapsed = start.elapsed();

    // Virtualization setup should be nearly instantaneous (< 100ms in debug)
    assert!(
        elapsed.as_millis() < 500,
        "ViewState creation took too long: {:?}",
        elapsed
    );

    // Total lines in document is 50,000
    assert_eq!(state.doc().len_lines(), 50001); // with trailing newline

    // Lines in visible viewport must be small (< 200 lines)
    let from_line = state.doc().char_to_line(view_state.viewport.from);
    let to_line = state.doc().char_to_line(view_state.viewport.to);
    let visible_lines = to_line - from_line;

    assert!(
        visible_lines < 200,
        "Visible lines in 50k doc must be under 200, got: {}",
        visible_lines
    );
}
