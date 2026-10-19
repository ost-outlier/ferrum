use crate::change::{ChangeSet, ChangeSpec};
use crate::rangeset::{Range, RangeSet, RangeSetUpdate, RangeValue};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TestVal {
    name: Option<String>,
    pos: Option<usize>,
    start_side: i32,
    end_side: i32,
    point: bool,
}

impl RangeValue for TestVal {
    fn start_side(&self) -> i32 { self.start_side }
    fn end_side(&self) -> i32 { self.end_side }
    fn point(&self) -> bool { self.point }
}

impl TestVal {
    fn point(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            pos: None,
            start_side: 0,
            end_side: 0,
            point: true,
        }
    }

    fn mark(name: &str, start_side: i32, end_side: i32) -> Self {
        Self {
            name: Some(name.to_string()),
            pos: None,
            start_side,
            end_side,
            point: false,
        }
    }

    fn with_pos(pos: usize) -> Self {
        Self {
            name: None,
            pos: Some(pos),
            start_side: 0,
            end_side: 0,
            point: false,
        }
    }
}

fn mk_val_range(from: usize, to: usize, val: TestVal) -> Range<TestVal> {
    Range::new(from, to, val)
}

#[test]
fn test_cm6_rangeset_divides_chunks_and_layers() {
    let mut small_ranges = Vec::new();
    for i in 0..5000 {
        small_ranges.push(mk_val_range(i, i + 1 + (i % 4), TestVal::with_pos(i)));
    }
    small_ranges.push(mk_val_range(1000, 4000, TestVal::with_pos(1000)));
    small_ranges.push(mk_val_range(2000, 3000, TestVal::with_pos(2000)));

    let set = RangeSet::of_sorted(small_ranges, true).expect("valid range set");
    assert_eq!(set.size(), 5002);
    assert!(set.chunk.len() > 1, "Expected multiple chunks for 5000 ranges");
    assert!(set.next_layer.is_some(), "Expected overlapping ranges to spill to next layer");

    let mut count = 0;
    set.between(0, set.length(), |from, _, val| {
        count += 1;
        if let Some(pos) = val.pos {
            assert_eq!(from, pos);
        }
        true
    });
    assert_eq!(count, set.size());
}

#[test]
fn test_cm6_rangeset_complains_about_misordered_ranges() {
    let misordered = vec![
        mk_val_range(8, 9, TestVal::mark("a", 0, 0)),
        mk_val_range(7, 10, TestVal::mark("b", 0, 0)),
    ];
    let res = RangeSet::of(misordered);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Ranges must be added sorted by `from` position and `startSide`");
}

#[test]
fn test_cm6_rangeset_update_add_and_filter() {
    let mut initial = Vec::new();
    for i in 0..1000 {
        initial.push(mk_val_range(i, i + 1, TestVal::with_pos(i)));
    }
    let set = RangeSet::of_sorted(initial, false).unwrap();
    assert_eq!(set.size(), 1000);

    // Add ranges
    let added = set.update(RangeSetUpdate {
        add: vec![mk_val_range(1500, 1502, TestVal::with_pos(1500))],
        ..Default::default()
    }).unwrap();
    assert_eq!(added.size(), 1001);

    // Filter ranges (keep only >= 500)
    let filtered = set.update(RangeSetUpdate {
        filter: Some(Arc::new(|from, _, _| from >= 500)),
        ..Default::default()
    }).unwrap();
    assert_eq!(filtered.size(), 500);

    // Filter removes everything
    let empty = set.update(RangeSetUpdate {
        filter: Some(Arc::new(|_, _, _| false)),
        ..Default::default()
    }).unwrap();
    assert_eq!(empty.size(), 0);
}

#[test]
fn test_cm6_rangeset_map_through_changes() {
    let set = RangeSet::of_sorted(vec![
        mk_val_range(1, 1, TestVal::point("a")),
        mk_val_range(4, 4, TestVal::point("b")),
        mk_val_range(10, 10, TestVal::point("c")),
    ], false).unwrap();

    let changes = ChangeSet::of_specs(&[
        ChangeSpec::replace(0, 0, "x"),
        ChangeSpec::replace(2, 3, ""),
        ChangeSpec::replace(8, 8, "xxxxxxxxxxxxxxxxxxxx"),
    ], 50);

    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 3);
    assert_eq!(ranges[0].from, 2);
    assert_eq!(ranges[1].from, 4);
    assert_eq!(ranges[2].from, 30);
}

#[test]
fn test_cm6_rangeset_map_shrinks_and_drops() {
    let set = RangeSet::of_sorted(vec![
        mk_val_range(2, 4, TestVal::mark("a", 0, 0)),
        mk_val_range(2, 8, TestVal::mark("b", 0, 0)),
        mk_val_range(6, 8, TestVal::mark("c", 0, 0)),
    ], true).unwrap();

    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(3, 7, "")], 20);
    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 3);
    assert_eq!((ranges[0].from, ranges[0].to), (2, 3));
    assert_eq!((ranges[1].from, ranges[1].to), (2, 4));
    assert_eq!((ranges[2].from, ranges[2].to), (3, 4));
}

#[test]
fn test_cm6_rangeset_drops_ranges_in_deleted_regions() {
    let set = RangeSet::of_sorted(vec![
        mk_val_range(1, 2, TestVal::mark("a", 0, 0)),
        mk_val_range(3, 3, TestVal::point("b")),
    ], true).unwrap();

    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(0, 4, "")], 10);
    let mapped = set.map(changes.desc());
    assert_eq!(mapped.size(), 0);
}

#[test]
fn test_cm6_rangeset_between_and_iter() {
    let set = RangeSet::of_sorted(vec![
        mk_val_range(10, 20, TestVal::mark("a", 0, 0)),
        mk_val_range(25, 30, TestVal::mark("b", 0, 0)),
        mk_val_range(50, 60, TestVal::mark("c", 0, 0)),
    ], false).unwrap();

    let mut found = Vec::new();
    set.between(15, 35, |from, to, _| {
        found.push((from, to));
        true
    });
    assert_eq!(found, vec![(10, 20), (25, 30)]);

    // zero-length set
    let zero_set = RangeSet::of_sorted(vec![mk_val_range(0, 0, TestVal::point("p"))], false).unwrap();
    let mut zero_found = Vec::new();
    zero_set.between(0, 0, |from, to, _| {
        zero_found.push((from, to));
        true
    });
    assert_eq!(zero_found, vec![(0, 0)]);
}
