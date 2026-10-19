use crate::selection::{EditorSelection, SelectionRange};

#[test]
fn test_cm6_selection_stores_ranges_with_a_primary_range() {
    let sel = EditorSelection::create(
        vec![
            SelectionRange::new(0, 1),
            SelectionRange::new(3, 2),
            SelectionRange::new(4, 5),
        ],
        1,
    );
    assert_eq!(sel.main().from(), 2);
    assert_eq!(sel.main().to(), 3);
    assert_eq!(sel.main().anchor, 3);
    assert_eq!(sel.main().head, 2);
    let s: Vec<String> = sel.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect();
    assert_eq!(s.join(","), "0/1,3/2,4/5");
}

#[test]
fn test_cm6_selection_merges_and_sorts_ranges_when_normalizing() {
    let sel = EditorSelection::create(
        vec![
            SelectionRange::new(10, 12),
            SelectionRange::new(6, 7),
            SelectionRange::new(4, 5),
            SelectionRange::new(3, 4),
            SelectionRange::new(0, 6),
            SelectionRange::new(7, 8),
            SelectionRange::new(9, 13),
            SelectionRange::new(13, 14),
        ],
        0,
    );
    let s: Vec<String> = sel.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect();
    assert_eq!(s.join(","), "0/6,6/7,7/8,9/13,13/14");
}

#[test]
fn test_cm6_selection_merges_adjacent_point_ranges_when_normalizing() {
    let sel = EditorSelection::create(
        vec![
            SelectionRange::new(10, 12),
            SelectionRange::new(12, 12),
            SelectionRange::new(12, 12),
            SelectionRange::new(10, 10),
            SelectionRange::new(8, 10),
        ],
        0,
    );
    let s: Vec<String> = sel.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect();
    assert_eq!(s.join(","), "8/10,10/12");
}

#[test]
fn test_cm6_selection_preserves_the_direction_of_the_last_range_when_merging_ranges() {
    let sel = EditorSelection::create(
        vec![
            SelectionRange::new(0, 2),
            SelectionRange::new(10, 1),
        ],
        0,
    );
    let s: Vec<String> = sel.ranges().iter().map(|r| format!("{}/{}", r.anchor, r.head)).collect();
    assert_eq!(s.join(","), "10/0");
}
