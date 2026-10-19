use crate::change::{ChangeDesc, ChangeSet, ChangeSpec, MapMode};
use crate::text::Text;

fn mk(spec: &str) -> ChangeDesc {
    ChangeDesc::from_string(spec)
}

// -------------------------------------------------------------------------
// ChangeDesc: composition
// -------------------------------------------------------------------------

#[test]
fn test_cm6_changedesc_composition() {
    fn comp(specs: &[&str], expected: &str) {
        let mut curr = mk(specs[0]);
        for &s in &specs[1..] {
            curr = curr.compose_desc(&mk(s));
        }
        assert_eq!(curr.to_string(), expected);
    }

    // Ported directly from CodeMirror 6 test-change.ts lines 39-61
    comp(&["5 0:2", "1 2:0 4"], "1 2:0 2 0:2");
    comp(&["2 0:2 2", "2 2:0 2"], "4");
    comp(&["2 0:2 2", "4 0:3 2"], "2 0:5 2");
    comp(&["2 5:0", "1 1:0"], "1 6:0");
    comp(&["2 2:0 0:3", "5:0"], "4:0");
    comp(&["", "0:8", "8:0", "", ""], "");
    comp(&["2 2:2 2:2 2", "1 2:2 2:2 2:2 1"], "1 6:6 1");
    comp(&["1 2:2 2:2 2:2 1", "2 2:2 2:2 2"], "1 6:6 1");
    comp(&["1 2:3 3:2 1", "2 3:1 2"], "1 5:3 1");
}

#[test]
#[should_panic(expected = "Mismatched")]
fn test_cm6_changedesc_composition_throws_for_inconsistent_lengths_1() {
    mk("2 0:2").compose_desc(&mk("1 0:1"));
}

#[test]
#[should_panic(expected = "Mismatched")]
fn test_cm6_changedesc_composition_throws_for_inconsistent_lengths_2() {
    mk("2 0:2").compose_desc(&mk("30 0:1"));
}

#[test]
#[should_panic(expected = "Mismatched")]
fn test_cm6_changedesc_composition_throws_for_inconsistent_lengths_3() {
    mk("2 2:0 0:3").compose_desc(&mk("7:0"));
}

// -------------------------------------------------------------------------
// ChangeDesc: mapping
// -------------------------------------------------------------------------

#[test]
fn test_cm6_changedesc_mapping() {
    fn over(a: &str, b: &str, expected: &str) {
        let res = mk(a).map_desc(&mk(b), false);
        assert_eq!(res.to_string(), expected);
    }
    fn under(a: &str, b: &str, expected: &str) {
        let res = mk(a).map_desc(&mk(b), true);
        assert_eq!(res.to_string(), expected);
    }

    // Ported directly from CodeMirror 6 test-change.ts lines 78-129
    over("4 0:1", "0:3 4", "7 0:1");
    over("4 0:1", "2:0 2", "2 0:1");

    over("2 0:1 2", "2 0:1 2", "3 0:1 2");
    under("2 0:1 2", "2 0:1 2", "2 0:1 3");

    over("2 2:0", "2 1:2 1", "4 1:0");
    under("2 2:0", "2 1:2 1", "4 1:0");

    over("0:1 2:0 8", "6 1:0 0:5 3", "0:1 2:0 12");
    over("5:0 2 3:0 2", "4 4:0 4", "6:0 2");

    under("2 0:1 2", "4:0", "0:1");
    over("4 0:1 4", "2 4:0 2", "2 0:1 2");

    over("2 2:2 2", "0:2 6", "4 2:2 2");
    over("2 2:2 2", "3:0 3", "1:2 2");
    over("1 4:4 1", "3 0:2 3", "1 2:4 2 2:0 1");
    over("1 4:4 1", "2 2:0 2", "1 2:4 1");
    over("2 2:2 2", "3 2:0 1", "2 1:2 1");

    over("2:2 2 2:2", "2 2:0 2", "2:2 2:2");
    under("2 2:0 2", "2 2:0 2", "4");
    over("2 2:0 2", "2 2:0 2", "4");

    over("1 1:2 1", "1 1:1 1", "2 0:2 1");
    under("1 1:2 1", "1 1:1 1", "1 0:2 2");
    over("1 1:2 2", "1 2:1 1", "1 0:2 2");
    over("2 1:2 1", "1 2:1 1", "2 0:2 1");
    over("2:1 1", "1 2:2", "1:1 2");
    over("1 2:1", "2:2 1", "2 1:1");
}

// -------------------------------------------------------------------------
// ChangeDesc: mapPos
// -------------------------------------------------------------------------

#[test]
fn test_cm6_changedesc_map_pos() {
    let set = mk("4 0:2 4");
    assert_eq!(set.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(set.map_pos(4, -1, MapMode::Simple), Some(4));
    assert_eq!(set.map_pos(4, 1, MapMode::Simple), Some(6));
    assert_eq!(set.map_pos(5, -1, MapMode::Simple), Some(7));
    assert_eq!(set.map_pos(8, -1, MapMode::Simple), Some(10));

    let del_set = mk("4 4:0 4");
    assert_eq!(del_set.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(del_set.map_pos(4, -1, MapMode::Simple), Some(4));
    assert_eq!(del_set.map_pos(4, -1, MapMode::TrackDel), Some(4));
    assert_eq!(del_set.map_pos(4, -1, MapMode::TrackBefore), Some(4));
    assert_eq!(del_set.map_pos(4, -1, MapMode::TrackAfter), None);
    assert_eq!(del_set.map_pos(5, -1, MapMode::Simple), Some(4));
    assert_eq!(del_set.map_pos(5, -1, MapMode::TrackDel), None);
    assert_eq!(del_set.map_pos(5, -1, MapMode::TrackBefore), None);
    assert_eq!(del_set.map_pos(5, -1, MapMode::TrackAfter), None);
    assert_eq!(del_set.map_pos(7, -1, MapMode::Simple), Some(4));
    assert_eq!(del_set.map_pos(8, -1, MapMode::Simple), Some(4));
    assert_eq!(del_set.map_pos(8, -1, MapMode::TrackDel), Some(4));
    assert_eq!(del_set.map_pos(8, -1, MapMode::TrackBefore), None);
    assert_eq!(del_set.map_pos(8, -1, MapMode::TrackAfter), Some(4));
    assert_eq!(del_set.map_pos(9, -1, MapMode::Simple), Some(5));
    assert_eq!(del_set.map_pos(12, -1, MapMode::Simple), Some(8));

    let multi_ins = mk("0:2 2 0:2 2 0:2");
    assert_eq!(multi_ins.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(multi_ins.map_pos(0, 1, MapMode::Simple), Some(2));
    assert_eq!(multi_ins.map_pos(1, -1, MapMode::Simple), Some(3));
    assert_eq!(multi_ins.map_pos(2, -1, MapMode::Simple), Some(4));
    assert_eq!(multi_ins.map_pos(2, 1, MapMode::Simple), Some(6));
    assert_eq!(multi_ins.map_pos(3, -1, MapMode::Simple), Some(7));
    assert_eq!(multi_ins.map_pos(4, -1, MapMode::Simple), Some(8));
    assert_eq!(multi_ins.map_pos(4, 1, MapMode::Simple), Some(10));

    let multi_del = mk("2:0 2 2:0 2 2:0");
    assert_eq!(multi_del.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(multi_del.map_pos(1, -1, MapMode::Simple), Some(0));
    assert_eq!(multi_del.map_pos(2, -1, MapMode::Simple), Some(0));
    assert_eq!(multi_del.map_pos(3, -1, MapMode::Simple), Some(1));
    assert_eq!(multi_del.map_pos(4, -1, MapMode::Simple), Some(2));
    assert_eq!(multi_del.map_pos(5, -1, MapMode::Simple), Some(2));
    assert_eq!(multi_del.map_pos(6, -1, MapMode::Simple), Some(2));
    assert_eq!(multi_del.map_pos(7, -1, MapMode::Simple), Some(3));
    assert_eq!(multi_del.map_pos(8, -1, MapMode::Simple), Some(4));
    assert_eq!(multi_del.map_pos(9, -1, MapMode::Simple), Some(4));
    assert_eq!(multi_del.map_pos(10, -1, MapMode::Simple), Some(4));

    let mixed = mk("2 0:2 2:0 0:2 2 2:0 0:2");
    assert_eq!(mixed.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(mixed.map_pos(2, -1, MapMode::Simple), Some(2));
    assert_eq!(mixed.map_pos(2, 1, MapMode::Simple), Some(4));
    assert_eq!(mixed.map_pos(3, -1, MapMode::Simple), Some(4));
    assert_eq!(mixed.map_pos(4, -1, MapMode::Simple), Some(4));
    assert_eq!(mixed.map_pos(4, 1, MapMode::Simple), Some(6));
    assert_eq!(mixed.map_pos(5, -1, MapMode::Simple), Some(7));
    assert_eq!(mixed.map_pos(6, -1, MapMode::Simple), Some(8));
    assert_eq!(mixed.map_pos(7, -1, MapMode::Simple), Some(8));
    assert_eq!(mixed.map_pos(8, -1, MapMode::Simple), Some(8));
    assert_eq!(mixed.map_pos(8, 1, MapMode::Simple), Some(10));
}

#[test]
fn test_cm6_changedesc_map_pos_replacements() {
    let set = mk("2 2:2 2");
    assert_eq!(set.map_pos(2, 1, MapMode::Simple), Some(2));
    assert_eq!(set.map_pos(2, -1, MapMode::Simple), Some(2));
    assert_eq!(set.map_pos(2, -1, MapMode::TrackDel), Some(2));
    assert_eq!(set.map_pos(2, -1, MapMode::TrackBefore), Some(2));
    assert_eq!(set.map_pos(2, -1, MapMode::TrackAfter), None);

    assert_eq!(set.map_pos(3, -1, MapMode::Simple), Some(2));
    assert_eq!(set.map_pos(3, 1, MapMode::Simple), Some(4));
    assert_eq!(set.map_pos(3, -1, MapMode::TrackDel), None);
    assert_eq!(set.map_pos(3, -1, MapMode::TrackBefore), None);
    assert_eq!(set.map_pos(3, -1, MapMode::TrackAfter), None);

    assert_eq!(set.map_pos(4, 1, MapMode::Simple), Some(4));
    assert_eq!(set.map_pos(4, -1, MapMode::Simple), Some(4));
    assert_eq!(set.map_pos(4, -1, MapMode::TrackDel), Some(4));
    assert_eq!(set.map_pos(4, -1, MapMode::TrackBefore), None);
    assert_eq!(set.map_pos(4, -1, MapMode::TrackAfter), Some(4));

    let set2 = mk("0:1 2:2 0:1");
    assert_eq!(set2.map_pos(0, -1, MapMode::Simple), Some(0));
    assert_eq!(set2.map_pos(0, 1, MapMode::Simple), Some(1));
    assert_eq!(set2.map_pos(1, -1, MapMode::Simple), Some(1));
    assert_eq!(set2.map_pos(1, 1, MapMode::Simple), Some(3));
    assert_eq!(set2.map_pos(2, -1, MapMode::Simple), Some(3));
    assert_eq!(set2.map_pos(2, 1, MapMode::Simple), Some(4));

    let set3 = mk("2:2 2:2");
    assert_eq!(set3.map_pos(2, -1, MapMode::Simple), Some(2));
    assert_eq!(set3.map_pos(2, 1, MapMode::Simple), Some(2));
}

// -------------------------------------------------------------------------
// ChangeSet tests
// -------------------------------------------------------------------------

#[test]
fn test_cm6_changeset_creation_and_apply() {
    let doc10 = Text::from("0123456789");

    let cs1 = ChangeSet::of_specs(&[ChangeSpec::insert(5, "hi")], 10);
    assert_eq!(cs1.desc().to_string(), "5 0:2 5");

    let cs2 = ChangeSet::of_specs(&[ChangeSpec::delete(5, 7)], 10);
    assert_eq!(cs2.desc().to_string(), "5 2:0 3");

    let cs3 = ChangeSet::of_specs(
        &[
            ChangeSpec::insert(5, "hi"),
            ChangeSpec::insert(5, "ok"),
            ChangeSpec::delete(0, 3),
            ChangeSpec::delete(4, 6),
            ChangeSpec::insert(8, "boo"),
        ],
        10,
    );
    assert_eq!(cs3.desc().to_string(), "3:0 1 1:0 0:4 1:0 2 0:3 2");

    assert_eq!(
        ChangeSet::of_specs(&[ChangeSpec::insert(2, "ok")], 10)
            .apply(&doc10)
            .to_string(),
        "01ok23456789"
    );
    assert_eq!(
        ChangeSet::of_specs(&[ChangeSpec::delete(1, 9)], 10)
            .apply(&doc10)
            .to_string(),
        "09"
    );
    assert_eq!(
        ChangeSet::of_specs(
            &[ChangeSpec::delete(2, 8), ChangeSpec::insert(1, "hi")],
            10
        )
        .apply(&doc10)
        .to_string(),
        "0hi189"
    );
}

#[test]
fn test_cm6_changeset_composed_and_mapped_sets() {
    let doc10 = Text::from("0123456789");

    let set_a = ChangeSet::of_specs(&[ChangeSpec::insert(8, "ABCD")], 10);
    let set_b = ChangeSet::of_specs(&[ChangeSpec::delete(8, 11)], 14);
    assert_eq!(set_a.compose(&set_b).apply(&doc10).to_string(), "01234567D89");

    let set_a2 = ChangeSet::of_specs(
        &[ChangeSpec::insert(2, "hi"), ChangeSpec::insert(8, "ok")],
        10,
    );
    let set_b2 = ChangeSet::of_specs(
        &[
            ChangeSpec::insert(4, "!"),
            ChangeSpec::delete(6, 8),
            ChangeSpec::insert(12, "?"),
        ],
        14,
    );
    assert_eq!(
        set_a2.compose(&set_b2).apply(&doc10).to_string(),
        "01hi!2367ok?89"
    );

    // Can clip inserted strings on compose
    let set1 = ChangeSet::of_specs(
        &[ChangeSpec::insert(2, "abc"), ChangeSpec::insert(4, "def")],
        10,
    );
    let set2 = ChangeSet::of_spec(ChangeSpec::delete(4, 8), 16);
    assert_eq!(set1.compose(&set2).apply(&doc10).to_string(), "01abef456789");

    // Can apply mapped sets
    let set0 = ChangeSet::of_specs(
        &[ChangeSpec::insert(5, "hi"), ChangeSpec::delete(8, 10)],
        10,
    );
    let set1 = ChangeSet::of_specs(
        &[ChangeSpec::insert(10, "ok"), ChangeSpec::delete(6, 7)],
        10,
    );
    let mapped = set1.map(set0.desc(), false);
    assert_eq!(set0.compose(&mapped).apply(&doc10).to_string(), "01234hi57ok");

    // Can apply inverted sets
    let inverted = set0.invert(&doc10);
    assert_eq!(inverted.apply(&set0.apply(&doc10)).to_string(), doc10.to_string());
}

#[test]
fn test_cm6_changeset_iteration() {
    let set = ChangeSet::of_specs(
        &[ChangeSpec::insert(4, "ok"), ChangeSpec::delete(6, 8)],
        10,
    );
    let mut result_changes: Vec<(usize, usize, usize, usize, String)> = Vec::new();
    set.iter_changes(
        |from_a, to_a, from_b, to_b, inserted| {
            result_changes.push((from_a, to_a, from_b, to_b, inserted.to_string()));
        },
        false,
    );
    assert_eq!(
        result_changes,
        vec![
            (4, 4, 4, 6, "ok".to_string()),
            (6, 8, 8, 8, "".to_string()),
        ]
    );

    let mut result_gaps: Vec<(usize, usize, usize)> = Vec::new();
    set.iter_gaps(|from_a, to_a, len| {
        result_gaps.push((from_a, to_a, len));
    });
    assert_eq!(
        result_gaps,
        vec![
            (0, 0, 4),
            (4, 6, 2),
            (8, 8, 2),
        ]
    );
}

#[test]
fn test_cm6_changeset_randomized_invariants() {
    struct Lcg(u64);
    impl Lcg {
        fn next_u32(&mut self) -> u32 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            (self.0 >> 32) as u32
        }
        fn gen_range(&mut self, max: usize) -> usize {
            if max == 0 { 0 } else { (self.next_u32() as usize) % max }
        }
        fn gen_str(&mut self, len: usize) -> String {
            (0..len).map(|_| (b'a' + (self.gen_range(26) as u8)) as char).collect()
        }
        fn gen_change(&mut self, doc_len: usize) -> ChangeSpec {
            if doc_len == 0 || self.gen_range(3) == 0 {
                let from = self.gen_range(doc_len + 1);
                let ins_len = self.gen_range(5) + 1;
                let ins = self.gen_str(ins_len);
                ChangeSpec::insert(from, ins.as_str())
            } else {
                let from = self.gen_range(doc_len);
                let to_len = self.gen_range(5) + 1;
                let to = std::cmp::min(from + to_len, doc_len);
                let is_empty = self.gen_range(2) != 0;
                let ins = if is_empty {
                    String::new()
                } else {
                    let ins_len = self.gen_range(2) + 1;
                    self.gen_str(ins_len)
                };
                ChangeSpec::replace(from, to, ins.as_str())
            }
        }
        fn gen_changes(&mut self, doc_len: usize, count: usize) -> Vec<ChangeSpec> {
            let mut changes = Vec::with_capacity(count);
            for _ in 0..count {
                changes.push(self.gen_change(doc_len));
            }
            changes
        }
    }

    let mut rng = Lcg(123456789);

    // 1. Compose is associative: (A . B) . C == A . (B . C)
    for _ in 0..50 {
        let size = rng.gen_range(20) + 1;
        let doc_str = rng.gen_str(size);
        let doc = Text::from(doc_str);
        let cnt_a = rng.gen_range(4) + 1;
        let changes_a = rng.gen_changes(size, cnt_a);
        let a = ChangeSet::of_specs(&changes_a, size);

        let cnt_b = rng.gen_range(4) + 1;
        let changes_b = rng.gen_changes(a.new_length(), cnt_b);
        let b = ChangeSet::of_specs(&changes_b, a.new_length());

        let cnt_c = rng.gen_range(4) + 1;
        let changes_c = rng.gen_changes(b.new_length(), cnt_c);
        let c = ChangeSet::of_specs(&changes_c, b.new_length());

        let left = a.compose(&b).compose(&c);
        let right = a.compose(&b.compose(&c));
        assert_eq!(left.apply(&doc).to_string(), right.apply(&doc).to_string());
    }

    // 2. Compose produces same result as individual changes
    for _ in 0..50 {
        let size = rng.gen_range(20) + 1;
        let doc_str = rng.gen_str(size);
        let doc = Text::from(doc_str);
        let cnt_a = rng.gen_range(4) + 1;
        let changes_a = rng.gen_changes(size, cnt_a);
        let a = ChangeSet::of_specs(&changes_a, size);

        let cnt_b = rng.gen_range(4) + 1;
        let changes_b = rng.gen_changes(a.new_length(), cnt_b);
        let b = ChangeSet::of_specs(&changes_b, a.new_length());

        let seq = b.apply(&a.apply(&doc));
        let comp = a.compose(&b).apply(&doc);
        assert_eq!(seq.to_string(), comp.to_string());
    }

    // 3. Operational Transformation confluence property TP1:
    // A . B.map(A, false) == B . A.map(B, true)
    for _ in 0..50 {
        let size = rng.gen_range(20) + 1;
        let doc_str = rng.gen_str(size);
        let doc = Text::from(doc_str);
        let cnt_a = rng.gen_range(3) + 1;
        let a_specs = rng.gen_changes(size, cnt_a);
        let cnt_b = rng.gen_range(3) + 1;
        let b_specs = rng.gen_changes(size, cnt_b);

        let set_a = ChangeSet::of_specs(&a_specs, size);
        let set_b = ChangeSet::of_specs(&b_specs, size);

        let set_a1 = set_a.map(set_b.desc(), true);
        let set_b1 = set_b.map(set_a.desc(), false);

        let ab = set_a.compose(&set_b1);
        let ba = set_b.compose(&set_a1);

        assert_eq!(ab.apply(&doc).to_string(), ba.apply(&doc).to_string());
    }
}
