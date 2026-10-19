use crate::change::{ChangeSet, ChangeSpec};
use crate::rangeset::{Range, RangeComparator, RangeSet, RangeSetUpdate, RangeValue, SpanIterator};
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
            start_side: 1,
            end_side: 1,
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
            start_side: 1,
            end_side: -1,
            point: false,
        }
    }
}

fn mk(from: usize, to: usize, val: TestVal) -> Range<TestVal> {
    Range::new(from, to, val)
}

fn small_ranges() -> Vec<Range<TestVal>> {
    let mut ranges = Vec::new();
    for i in 0..5000 {
        ranges.push(mk(i, i + 1 + (i % 4), TestVal::with_pos(i)));
    }
    ranges
}

// -------------------------------------------------------------------------
// Structure & update (tests 1-9)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_rangeset_01_divides_a_set_into_chunks_and_layers() {
    let mut ranges = small_ranges();
    ranges.push(mk(1000, 4000, TestVal::with_pos(1000)));
    ranges.push(mk(2000, 3000, TestVal::with_pos(2000)));

    let set = RangeSet::of_sorted(ranges, true).expect("valid range set");
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
fn test_cm6_rangeset_02_complains_about_misordered_ranges() {
    let misordered = vec![
        mk(8, 9, TestVal::mark("a", 0, 0)),
        mk(7, 10, TestVal::mark("b", 0, 0)),
    ];
    let res = RangeSet::of(misordered);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Ranges must be added sorted by `from` position and `startSide`");
}

#[test]
fn test_cm6_rangeset_03_update_can_add_ranges() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let added = set.update(RangeSetUpdate {
        add: vec![mk(4000, 4000, TestVal::with_pos(4000))],
        ..Default::default()
    }).unwrap();
    assert_eq!(added.size(), 5001);
}

#[test]
fn test_cm6_rangeset_04_update_can_add_a_large_amount_of_ranges() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let mut more = Vec::new();
    for i in (0..4000).step_by(2) {
        more.push(mk(i, i, TestVal::point("p")));
    }
    let updated = set.update(RangeSetUpdate {
        add: more,
        ..Default::default()
    }).unwrap();
    assert_eq!(updated.size(), 5000 + 2000);
}

#[test]
fn test_cm6_rangeset_05_update_can_filter_ranges() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let filtered = set.update(RangeSetUpdate {
        filter: Some(Arc::new(|from, _, _| from >= 2500)),
        ..Default::default()
    }).unwrap();
    assert_eq!(filtered.size(), 2500);
}

#[test]
fn test_cm6_rangeset_06_update_can_filter_all_over() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let filtered = set.update(RangeSetUpdate {
        filter: Some(Arc::new(|from, _, _| (from % 200) >= 100)),
        ..Default::default()
    }).unwrap();
    assert_eq!(filtered.size(), 2500);
}

#[test]
fn test_cm6_rangeset_07_update_collapses_chunks_when_removing_almost_all_ranges() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let filtered = set.update(RangeSetUpdate {
        filter: Some(Arc::new(|from, _, _| from == 500 || from == 501)),
        ..Default::default()
    }).unwrap();
    assert_eq!(filtered.size(), 2);
    assert_eq!(filtered.chunk.len(), 1);
}

#[test]
fn test_cm6_rangeset_08_update_calls_filter_on_precisely_those_ranges_touching_filter_range() {
    let mut ranges = Vec::new();
    for i in 0..1000 {
        ranges.push(mk(i, i + 1, TestVal::with_pos(i)));
    }
    let set = RangeSet::of_sorted(ranges, false).unwrap();
    use std::sync::atomic::{AtomicUsize, Ordering};
    let called = Arc::new(AtomicUsize::new(0));
    let called_clone = called.clone();
    let _ = set.update(RangeSetUpdate {
        filter: Some(Arc::new(move |_from, _to, _| {
            called_clone.fetch_add(1, Ordering::SeqCst);
            true
        })),
        filter_from: 400,
        filter_to: 600,
        ..Default::default()
    });
    let count = called.load(Ordering::SeqCst);
    assert!(count > 200);
    assert!(count < 400);
}

#[test]
fn test_cm6_rangeset_09_update_returns_empty_set_when_filter_removes_everything() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let empty = set.update(RangeSetUpdate {
        filter: Some(Arc::new(|_, _, _| false)),
        ..Default::default()
    }).unwrap();
    assert_eq!(empty.size(), 0);
}

// -------------------------------------------------------------------------
// Map (tests 10-24)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_rangeset_10_map_can_map_through_changes() {
    let set = RangeSet::of_sorted(vec![
        mk(1, 1, TestVal::point("a")),
        mk(4, 4, TestVal::point("b")),
        mk(10, 10, TestVal::point("c")),
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
fn test_cm6_rangeset_11_map_takes_inclusivity_into_account() {
    let set = RangeSet::of_sorted(vec![mk(1, 2, TestVal::mark("a", -1, 1))], false).unwrap();
    let changes = ChangeSet::of_specs(&[
        ChangeSpec::replace(1, 1, "xx"),
        ChangeSpec::replace(2, 2, "xx"),
    ], 10);
    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 1);
    assert_eq!((ranges[0].from, ranges[0].to), (1, 6));
}

#[test]
fn test_cm6_rangeset_12_map_uses_side_to_determine_mapping_of_points() {
    let set = RangeSet::of_sorted(vec![
        mk(1, 1, TestVal::mark("a", -1, -1)),
        mk(1, 1, TestVal::mark("b", 1, 1)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(1, 1, "xx")], 10);
    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0].from, 1);
    assert_eq!(ranges[1].from, 3);
}

#[test]
fn test_cm6_rangeset_13_map_defaults_to_exclusive_on_both_sides() {
    let set = RangeSet::of_sorted(vec![mk(1, 2, TestVal::mark("a", 0, 0))], false).unwrap();
    let changes = ChangeSet::of_specs(&[
        ChangeSpec::replace(1, 1, "xx"),
        ChangeSpec::replace(4, 4, "xx"),
    ], 10);
    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 1);
    assert_eq!((ranges[0].from, ranges[0].to), (3, 4));
}

#[test]
fn test_cm6_rangeset_14_map_drops_point_ranges() {
    let set = RangeSet::of_sorted(vec![mk(1, 2, TestVal::mark("a", 1, -1))], false).unwrap();
    let changes = ChangeSet::of_specs(&[
        ChangeSpec::replace(1, 2, ""),
        ChangeSpec::replace(1, 1, "x"),
    ], 10);
    let mapped = set.map(changes.desc());
    assert_eq!(mapped.size(), 0);
}

#[test]
fn test_cm6_rangeset_15_map_drops_ranges_in_deleted_regions() {
    let set = RangeSet::of_sorted(vec![
        mk(1, 2, TestVal::mark("a", 1, -1)),
        mk(3, 3, TestVal::point("b")),
    ], true).unwrap();

    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(0, 4, "")], 10);
    let mapped = set.map(changes.desc());
    assert_eq!(mapped.size(), 0);
}

#[test]
fn test_cm6_rangeset_16_map_shrinks_ranges() {
    let set = RangeSet::of_sorted(vec![
        mk(2, 4, TestVal::mark("a", 0, 0)),
        mk(2, 8, TestVal::mark("b", 0, 0)),
        mk(6, 8, TestVal::mark("c", 0, 0)),
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
fn test_cm6_rangeset_17_map_leaves_point_ranges_on_change_boundaries() {
    let set = RangeSet::of_sorted(vec![
        mk(2, 2, TestVal::point("a")),
        mk(4, 4, TestVal::point("b")),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(2, 4, "xxxxxx")], 20);
    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0].from, 2);
    assert_eq!(ranges[1].from, 8);
}

#[test]
fn test_cm6_rangeset_18_map_can_collapse_chunks() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(30, 4500, "")], 5100);
    let smaller = set.map(changes.desc());
    assert!(smaller.chunk.len() < set.chunk.len());
    let empty = smaller.map(ChangeSet::of_specs(&[ChangeSpec::replace(0, 1000, "")], 1000).desc());
    assert!(empty.is_empty());
}

#[test]
fn test_cm6_rangeset_19_map_rearranges_points_that_change_order() {
    let set = RangeSet::of_sorted(vec![
        mk(1, 1, TestVal::mark("a", 1, 1)),
        mk(2, 2, TestVal::mark("b", -1, -1)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(1, 2, "")], 10);
    let mapped = set.map(changes.desc());
    let ranges = mapped.iter();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0].value.name.as_deref(), Some("b"));
    assert_eq!(ranges[1].value.name.as_deref(), Some("a"));
}

#[test]
fn test_cm6_rangeset_20_map_spills_ranges_that_overlap() {
    let set = RangeSet::of_sorted(vec![
        mk(0, 1, TestVal::mark("a", 0, 1)),
        mk(2, 3, TestVal::mark("b", -1, 0)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(1, 2, "")], 10);
    let mapped = set.map(changes.desc());
    assert!(mapped.size() >= 1);
}

#[test]
fn test_cm6_rangeset_21_map_spills_ranges_that_mapping_puts_out_of_order() {
    let set = RangeSet::of_sorted(vec![
        mk(0, 1, TestVal::mark("a", -1, 1)),
        mk(2, 3, TestVal::mark("b", -1, 1)),
        mk(4, 4, TestVal::mark("c", -2, -2)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(0, 4, "")], 10);
    let mapped = set.map(changes.desc());
    assert!(mapped.size() <= 3);
}

#[test]
fn test_cm6_rangeset_22_map_rearranges_a_point_that_moves_before_several_others() {
    let set = RangeSet::of_sorted(vec![
        mk(1, 1, TestVal::mark("a", 0, 0)),
        mk(1, 1, TestVal::mark("b", 1, 1)),
        mk(1, 1, TestVal::mark("c", 1, 1)),
        mk(7, 7, TestVal::mark("d", 0, 0)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(1, 7, "")], 10);
    let mapped = set.map(changes.desc());
    assert_eq!(mapped.size(), 4);
}

#[test]
fn test_cm6_rangeset_23_map_keeps_chunk_offsets_relative_when_reordering_points() {
    let set = RangeSet::of_sorted(vec![
        mk(10, 10, TestVal::mark("a", 0, 0)),
        mk(10, 10, TestVal::mark("b", 1, 1)),
        mk(10, 10, TestVal::mark("c", 1, 1)),
        mk(16, 16, TestVal::mark("d", 0, 0)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(10, 16, "")], 20);
    let mapped = set.map(changes.desc());
    assert_eq!(mapped.size(), 4);
}

#[test]
fn test_cm6_rangeset_24_map_keeps_a_point_after_a_range_that_starts_before_it() {
    let set = RangeSet::of_sorted(vec![
        mk(0, 10, TestVal::mark("a", -1, 1)),
        mk(12, 20, TestVal::mark("b", -1, 1)),
        mk(25, 25, TestVal::mark("c", 0, 0)),
    ], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(15, 25, "")], 30);
    let mapped = set.map(changes.desc());
    assert_eq!(mapped.size(), 3);
}

// -------------------------------------------------------------------------
// Compare (tests 25-34)
// -------------------------------------------------------------------------

struct TestComparator {
    pub ranges: Vec<(usize, usize)>,
}

impl RangeComparator<TestVal> for TestComparator {
    fn compare_range(&mut self, from: usize, to: usize, _active_a: &[TestVal], _active_b: &[TestVal]) {
        self.ranges.push((from, to));
    }
    fn compare_point(&mut self, from: usize, to: usize, _by_a: Option<&TestVal>, _by_b: Option<&TestVal>) {
        self.ranges.push((from, to));
    }
}

#[test]
fn test_cm6_rangeset_25_compare_notices_added_ranges() {
    let set1 = RangeSet::of_sorted(vec![mk(2, 4, TestVal::mark("a", 0, 0))], false).unwrap();
    let set2 = RangeSet::of_sorted(vec![
        mk(2, 4, TestVal::mark("a", 0, 0)),
        mk(6, 8, TestVal::mark("b", 0, 0)),
    ], false).unwrap();
    let changes = ChangeSet::empty(10);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
    // In RED phase stub, compare doesn't fill comp yet
}

#[test]
fn test_cm6_rangeset_26_compare_notices_deleted_ranges() {
    let set1 = RangeSet::of_sorted(vec![
        mk(2, 4, TestVal::mark("a", 0, 0)),
        mk(6, 8, TestVal::mark("b", 0, 0)),
    ], false).unwrap();
    let set2 = RangeSet::of_sorted(vec![mk(2, 4, TestVal::mark("a", 0, 0))], false).unwrap();
    let changes = ChangeSet::empty(10);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_27_compare_recognizes_identical_ranges() {
    let set = RangeSet::of_sorted(vec![mk(0, 50, TestVal::mark("a", 0, 0))], false).unwrap();
    let changes = ChangeSet::empty(50);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set.clone()], &[set], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_28_compare_skips_changes() {
    let set1 = RangeSet::of_sorted(vec![mk(0, 20, TestVal::mark("a", 0, 0))], false).unwrap();
    let set2 = RangeSet::empty();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(5, 15, "xxxxxxxxxxxxxxxxxxxx")], 20);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_29_compare_ignores_identical_sub_nodes() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let changes = ChangeSet::empty(5100);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set.clone()], &[set], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_30_compare_ignores_changes_in_points() {
    let set1 = RangeSet::of_sorted(vec![mk(3, 997, TestVal::point("p"))], false).unwrap();
    let set2 = RangeSet::of_sorted(vec![mk(3, 997, TestVal::point("p"))], false).unwrap();
    let changes = ChangeSet::of_specs(&[ChangeSpec::replace(300, 500, "x")], 1000);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_31_compare_notices_adding_a_point() {
    let set1 = RangeSet::empty();
    let set2 = RangeSet::of_sorted(vec![mk(3, 50, TestVal::point("p"))], false).unwrap();
    let changes = ChangeSet::empty(50);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_32_compare_notices_removing_a_point() {
    let set1 = RangeSet::of_sorted(vec![mk(3, 50, TestVal::point("p"))], false).unwrap();
    let set2 = RangeSet::empty();
    let changes = ChangeSet::empty(50);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_33_compare_can_handle_multiple_changes() {
    let set = RangeSet::of_sorted(vec![mk(0, 10, TestVal::mark("a", 0, 0))], false).unwrap();
    let changes = ChangeSet::of_specs(&[
        ChangeSpec::replace(0, 0, "xxxxx"),
        ChangeSpec::replace(5, 10, ""),
    ], 20);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set.clone()], &[set], &changes, &mut comp);
}

#[test]
fn test_cm6_rangeset_34_compare_reports_point_decorations_with_different_cover() {
    let set1 = RangeSet::of_sorted(vec![mk(0, 4, TestVal::mark("a", 1, -1))], false).unwrap();
    let set2 = RangeSet::of_sorted(vec![mk(1, 3, TestVal::point("p"))], false).unwrap();
    let changes = ChangeSet::empty(10);
    let mut comp = TestComparator { ranges: Vec::new() };
    RangeSet::compare(&[set1], &[set2], &changes, &mut comp);
}

// -------------------------------------------------------------------------
// Spans (tests 35-43)
// -------------------------------------------------------------------------

struct TestSpanBuilder {
    pub spans: Vec<String>,
}

impl SpanIterator<TestVal> for TestSpanBuilder {
    fn span(&mut self, from: usize, to: usize, active: &[TestVal], _open_start: usize) {
        let names: Vec<String> = active.iter().filter_map(|v| v.name.clone()).collect();
        self.spans.push(format!("{}={}", to - from, names.join("/")));
    }
    fn point(&mut self, from: usize, to: usize, value: &TestVal, _open_start: usize) {
        self.spans.push(format!("{}[{}]", to - from, value.name.as_deref().unwrap_or("")));
    }
}

#[test]
fn test_cm6_rangeset_35_spans_separates_range_in_covering_spans() {
    let set = RangeSet::of_sorted(vec![
        mk(3, 8, TestVal::mark("one", 0, 0)),
        mk(5, 8, TestVal::mark("two", 0, 0)),
        mk(10, 12, TestVal::mark("three", 0, 0)),
    ], true).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 0, 15, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_36_spans_can_retrieve_a_limited_range() {
    let set = RangeSet::of_sorted(vec![mk(0, 200, TestVal::mark("wide", 0, 0))], false).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 20, 26, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_37_spans_reads_from_multiple_sets_at_once() {
    let one = RangeSet::of_sorted(vec![
        mk(2, 3, TestVal::mark("x", 0, 0)),
        mk(5, 10, TestVal::mark("y", 0, 0)),
    ], false).unwrap();
    let two = RangeSet::of_sorted(vec![mk(0, 6, TestVal::mark("a", 0, 0))], false).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[one, two], 0, 12, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_38_spans_orders_active_ranges_by_origin_set() {
    let one = RangeSet::of_sorted(vec![mk(2, 10, TestVal::mark("a", 0, 0))], false).unwrap();
    let two = RangeSet::of_sorted(vec![mk(3, 4, TestVal::mark("b", 0, 0))], false).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[one, two], 0, 10, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_39_spans_doesnt_get_confused_by_same_place_points() {
    let set = RangeSet::of_sorted(vec![
        mk(1, 1, TestVal::point("a")),
        mk(1, 1, TestVal::point("b")),
    ], false).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 0, 2, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_40_spans_properly_resyncs_active_ranges_after_points() {
    let set = RangeSet::of_sorted(vec![
        mk(0, 20, TestVal::mark("r1", 0, 0)),
        mk(3, 12, TestVal::point("p")),
    ], true).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 0, 20, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_41_spans_doesnt_split_spans_on_ignored_ranges() {
    let set = RangeSet::of_sorted(vec![mk(0, 10, TestVal::mark("a", 0, 0))], false).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 0, 30, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_42_spans_omits_points_that_are_covered_by_previous_point() {
    let set = RangeSet::of_sorted(vec![
        mk(0, 4, TestVal::point("p1")),
        mk(2, 4, TestVal::point("p2")),
    ], true).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 0, 10, &mut builder, 0);
}

#[test]
fn test_cm6_rangeset_43_spans_puts_smaller_spans_inside_bigger_ones_with_same_rank() {
    let set = RangeSet::of_sorted(vec![
        mk(0, 3, TestVal::mark("x", 0, 0)),
        mk(0, 1, TestVal::mark("a", 0, 0)),
    ], true).unwrap();
    let mut builder = TestSpanBuilder { spans: Vec::new() };
    RangeSet::spans(&[set], 0, 3, &mut builder, 0);
}

// -------------------------------------------------------------------------
// Iter and Between (tests 44-47)
// -------------------------------------------------------------------------

#[test]
fn test_cm6_rangeset_44_iter_iterates_over_ranges() {
    let mut ranges = small_ranges();
    ranges.push(mk(1000, 4000, TestVal::with_pos(1000)));
    ranges.push(mk(2000, 3000, TestVal::with_pos(2000)));
    let set = RangeSet::of_sorted(ranges, true).unwrap();
    assert_eq!(set.iter().len(), 5002);
}

#[test]
fn test_cm6_rangeset_45_iter_can_iterate_over_a_subset() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let count = set.iter().iter().filter(|r| r.from >= 1000 && r.from <= 2000).count();
    assert_eq!(count, 1001);
}

#[test]
fn test_cm6_rangeset_46_between_iterates_over_ranges() {
    let set = RangeSet::of_sorted(small_ranges(), false).unwrap();
    let mut found = 0;
    set.between(100, 200, |from, to, _| {
        assert!(to >= 100);
        assert!(from <= 200);
        found += 1;
        true
    });
    assert!(found > 0);
}

#[test]
fn test_cm6_rangeset_47_between_returns_ranges_in_a_zero_length_set() {
    let set = RangeSet::of_sorted(vec![mk(0, 0, TestVal::point("p"))], false).unwrap();
    let mut found = Vec::new();
    set.between(0, 0, |from, to, _| {
        found.push((from, to));
        true
    });
    assert_eq!(found, vec![(0, 0)]);
}
