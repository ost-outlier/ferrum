use crate::decoration::{Decoration, DecorationSet, ReplaceSpec, WidgetSpec, WidgetType};
use crate::heightmap::{
    clear_height_change_flag, get_height_change_flag, BlockType, ChangedRange, HeightMap,
    HeightOracle, MeasuredHeights, QueryType,
};
use ferrum_state::rangeset::Range;
use ferrum_state::text::Text;
use std::sync::Arc;

fn o(doc: &Text) -> HeightOracle {
    let mut oracle = HeightOracle::new(false);
    oracle.set_doc(doc.clone());
    oracle
}

fn mk(text: Text, deco: Vec<Range<Decoration>>) -> HeightMap {
    let len = text.len_chars();
    let deco_set = if deco.is_empty() {
        DecorationSet::empty()
    } else {
        DecorationSet::of_sorted(deco, true).unwrap()
    };
    HeightMap::empty().apply_changes(
        &[deco_set],
        &Text::empty(),
        &mut o(&text),
        &[ChangedRange::new(0, 0, 0, len)],
    )
}

fn doc(line_lens: &[usize]) -> Text {
    let lines: Vec<String> = line_lens.iter().map(|&len| "x".repeat(len)).collect();
    let line_slices: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    Text::of(&line_slices)
}

#[derive(Debug)]
struct MyWidget(f64);
impl WidgetType for MyWidget {
    fn estimated_height(&self) -> f64 {
        self.0
    }
}

#[derive(Debug)]
struct NoHeightWidget;
impl WidgetType for NoHeightWidget {}

// =========================================================================
// Canonical CodeMirror 6 HeightMap Tests (from test-heightmap.ts)
// =========================================================================

#[test]
fn test_cm6_heightmap_01_starts_empty() {
    let empty = HeightMap::empty();
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.size(), 1);
}

#[test]
fn test_cm6_heightmap_02_grows_to_match_the_document() {
    assert_eq!(mk(doc(&[10, 10, 8]), vec![]).length(), 30);
}

#[test]
fn test_cm6_heightmap_03_separates_lines_with_decorations_on_them() {
    let map = mk(
        doc(&[10, 10, 20, 5]),
        vec![
            Decoration::widget_spec(
                5,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(20.0))),
                    ..Default::default()
                },
            ),
            Decoration::replace_spec(25..46, ReplaceSpec::default()),
        ],
    );
    assert_eq!(map.length(), 48);
    assert_eq!(map.to_string(), "line(10:20) gap(10) line(26-21)");
}

#[test]
fn test_cm6_heightmap_04_ignores_irrelevant_decorations() {
    let map = mk(
        doc(&[10, 10, 20, 5]),
        vec![
            Decoration::widget_spec(
                5,
                WidgetSpec {
                    widget: Some(Arc::new(NoHeightWidget)),
                    ..Default::default()
                },
            ),
            Decoration::mark(25..46, "ahah"),
        ],
    );
    assert_eq!(map.length(), 48);
    assert_eq!(map.to_string(), "gap(48)");
}

#[test]
fn test_cm6_heightmap_05_drops_decorations_from_the_tree_when_they_are_deleted() {
    let text = doc(&[20]);
    let mut map = mk(
        text.clone(),
        vec![Decoration::widget_spec(
            5,
            WidgetSpec {
                widget: Some(Arc::new(MyWidget(20.0))),
                ..Default::default()
            },
        )],
    );
    assert_eq!(map.to_string(), "line(20:20)");
    let mut oracle = o(&text);
    map = map.apply_changes(&[], &text, &mut oracle, &[ChangedRange::new(5, 5, 5, 5)]);
    assert_eq!(map.to_string(), "line(20)");
}

#[test]
fn test_cm6_heightmap_06_updates_the_length_of_replaced_decorations_for_changes() {
    let text = doc(&[20]);
    let mut map = mk(
        text.clone(),
        vec![Decoration::replace_spec(5..15, ReplaceSpec::default())],
    );
    let mut oracle = o(&text.replace(7..12, ""));
    let deco_set = DecorationSet::of_sorted(
        vec![Decoration::replace_spec(5..10, ReplaceSpec::default())],
        true,
    )
    .unwrap();
    map = map.apply_changes(
        &[deco_set],
        &text,
        &mut oracle,
        &[ChangedRange::new(7, 12, 7, 7)],
    );
    assert_eq!(map.to_string(), "line(15-5)");
}

#[test]
fn test_cm6_heightmap_07_stores_information_about_block_widgets() {
    let text = doc(&[3, 3, 3]);
    let mut oracle = o(&text);
    let mut map = mk(
        text.clone(),
        vec![
            Decoration::widget_spec(
                0,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(10.0))),
                    side: -1,
                    block: true,
                },
            ),
            Decoration::widget_spec(
                0,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(13.0))),
                    side: -1,
                    block: true,
                },
            ),
            Decoration::widget_spec(
                3,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(5.0))),
                    side: 1,
                    block: true,
                },
            ),
        ],
    );
    assert_eq!(map.to_string(), "block(0)-block(0)-line(3)-block(0) gap(7)");
    assert_eq!(map.height(), 28.0 + 3.0 * oracle.line_height);
    let line_info = map.line_at(0, QueryType::ByPos, &mut oracle, 0.0, 0);
    let heights: Vec<f64> = line_info.sub_blocks().iter().map(|b| b.height).collect();
    assert_eq!(heights, vec![10.0, 13.0, oracle.line_height, 5.0]);
    assert_eq!(
        map.line_at(4, QueryType::ByPos, &mut oracle, 0.0, 0).top,
        28.0 + oracle.line_height
    );
    let mut measured = MeasuredHeights::new(0, vec![8.0, 12.0, 10.0, 20.0, 40.0, 20.0]);
    map = map.update_height(&mut oracle, 0, false, Some(&mut measured));
    assert_eq!(
        map.to_string(),
        "block(0)-block(0)-line(3)-block(0) line(3) line(3)"
    );
    assert_eq!(map.height(), 110.0);
}

#[test]
fn test_cm6_heightmap_08_stores_information_about_block_ranges() {
    let text = doc(&[3, 3, 3, 3, 3, 3]);
    let mut map = mk(
        text.clone(),
        vec![
            Decoration::widget_spec(
                4,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(10.0))),
                    side: -1,
                    block: true,
                },
            ),
            Decoration::replace_spec(
                4..11,
                ReplaceSpec {
                    widget: Some(Arc::new(MyWidget(40.0))),
                    block: true,
                },
            ),
            Decoration::widget_spec(
                11,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(15.0))),
                    side: 1,
                    block: true,
                },
            ),
            Decoration::widget_spec(
                16,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(20.0))),
                    side: 1,
                    block: true,
                },
            ),
            Decoration::replace_spec(
                16..19,
                ReplaceSpec {
                    widget: Some(Arc::new(MyWidget(50.0))),
                    block: true,
                },
            ),
            Decoration::widget_spec(
                19,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(10.0))),
                    side: -1,
                    block: true,
                },
            ),
        ],
    );
    assert_eq!(
        map.to_string(),
        "gap(3) block(0)-block(7)-block(0) gap(3) block(3) gap(3)"
    );
    let mut oracle = o(&text);
    let mut measured = MeasuredHeights::new(4, vec![5.0, 5.0, 5.0, 5.0, 10.0, 5.0]);
    map = map.update_height(&mut oracle, 0, false, Some(&mut measured));
    assert_eq!(map.height(), oracle.line_height + 35.0);
}

#[test]
fn test_cm6_heightmap_09_handles_empty_lines_correctly() {
    let text = doc(&[0, 0, 0, 0, 0]);
    let mut map = mk(
        text.clone(),
        vec![
            Decoration::widget_spec(
                1,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(10.0))),
                    side: -1,
                    block: true,
                },
            ),
            Decoration::replace_spec(
                2..2,
                ReplaceSpec {
                    widget: Some(Arc::new(MyWidget(20.0))),
                    block: true,
                },
            ),
            Decoration::widget_spec(
                3,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(30.0))),
                    side: 1,
                    block: true,
                },
            ),
        ],
    );
    assert_eq!(
        map.to_string(),
        "gap(0) block(0)-line(0) block(0) line(0)-block(0) gap(0)"
    );
    let mut oracle = o(&text.replace(1..3, "y"));
    map = map.apply_changes(&[], &text, &mut oracle, &[ChangedRange::new(1, 3, 1, 2)]);
    assert_eq!(map.to_string(), "gap(3)");
}

#[test]
fn test_cm6_heightmap_10_joins_ranges() {
    let text = doc(&[10, 10, 10, 10]);
    let mut map = mk(
        text.clone(),
        vec![Decoration::replace_spec(16..27, ReplaceSpec::default())],
    );
    assert_eq!(map.to_string(), "gap(10) line(21-11) gap(10)");
    let mut oracle = o(&text.replace(5..38, "yyy"));
    map = map.apply_changes(&[], &text, &mut oracle, &[ChangedRange::new(5, 38, 5, 8)]);
    assert_eq!(map.to_string(), "gap(13)");
}

#[test]
fn test_cm6_heightmap_11_joins_lines() {
    let text = doc(&[10, 10, 10]);
    let mut map = mk(
        text.clone(),
        vec![
            Decoration::replace_spec(2..5, ReplaceSpec::default()),
            Decoration::widget_spec(
                24,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(20.0))),
                    ..Default::default()
                },
            ),
        ],
    );
    assert_eq!(map.to_string(), "line(10-3) gap(10) line(10:20)");
    let mut oracle = o(&text.replace(10..22, ""));
    let deco_set = DecorationSet::of_sorted(
        vec![
            Decoration::replace_spec(2..5, ReplaceSpec::default()),
            Decoration::widget_spec(
                12,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(20.0))),
                    ..Default::default()
                },
            ),
        ],
        true,
    )
    .unwrap();
    map = map.apply_changes(&[deco_set], &text, &mut oracle, &[ChangedRange::new(10, 22, 10, 10)]);
    assert_eq!(map.to_string(), "line(20-3:20)");
}

#[test]
fn test_cm6_heightmap_12_materializes_lines_for_measured_heights() {
    let text = doc(&[10, 10, 10, 10]);
    let mut oracle = o(&text);
    let mut measured = MeasuredHeights::new(11, vec![28.0, 14.0, 5.0]);
    let map = mk(text, vec![]).update_height(&mut oracle, 0, false, Some(&mut measured));
    assert_eq!(map.to_string(), "gap(10) line(10) line(10) line(10)");
    assert_eq!(map.height(), 61.0);
}

#[test]
fn test_cm6_heightmap_13_doesnt_set_the_height_changed_flag_for_small_within_line_replacements() {
    let text = doc(&[10, 10, 10]);
    let mut oracle = o(&text);
    let mut measured1 = MeasuredHeights::new(0, vec![12.0, 12.0, 12.0]);
    let mut map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured1));
    let new_text = text.replace(21..21, "!");
    oracle.set_doc(new_text);
    map = map.apply_changes(&[], &text, &mut oracle, &[ChangedRange::new(21, 21, 21, 22)]);
    clear_height_change_flag();
    let mut measured2 = MeasuredHeights::new(0, vec![12.0, 12.0, 12.0]);
    let _ = map.update_height(&mut oracle, 0, false, Some(&mut measured2));
    assert_eq!(get_height_change_flag(), false);
}

#[test]
fn test_cm6_heightmap_14_can_update_lines_across_the_tree() {
    let text = doc(&vec![10; 100]);
    let mut oracle = o(&text);
    let mut measured1 = MeasuredHeights::new(0, vec![12.0; 100]);
    let mut map = mk(text, vec![]).update_height(&mut oracle, 0, false, Some(&mut measured1));
    assert_eq!(map.height(), 1200.0);
    assert_eq!(map.size(), 100);
    let mut measured2 = MeasuredHeights::new(55, vec![10.0; 90]);
    map = map.update_height(&mut oracle, 0, false, Some(&mut measured2));
    assert_eq!(map.height(), 1020.0);
    assert_eq!(map.size(), 100);
}

#[test]
fn test_cm6_heightmap_15_balances_a_big_tree() {
    let text = doc(&vec![30; 100]);
    let mut oracle = o(&text);
    let mut measured1 = MeasuredHeights::new(0, vec![15.0; 100]);
    let mut map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured1));
    assert_eq!(map.height(), 1500.0);
    assert_eq!(map.size(), 100);
    assert!(map.depth() < 9);

    let text2 = text.replace(0..31 * 80, "");
    let mut oracle2 = o(&text2);
    map = map.apply_changes(&[], &text, &mut oracle2, &[ChangedRange::new(0, 31 * 80, 0, 0)]);
    assert_eq!(map.size(), 20);
    assert!(map.depth() < 7);

    let len = text2.len_chars();
    let foo_lines: Vec<String> = (0..200).map(|_| "foo".to_string()).collect();
    let foo_str = format!("\n{}", foo_lines.join("\n"));
    let text3 = text2.replace(len..len, &foo_str);
    let mut oracle3 = o(&text3);
    map = map.apply_changes(
        &[],
        &text2,
        &mut oracle3,
        &[ChangedRange::new(len, len, len, len + 800)],
    );
    let mut measured2 = MeasuredHeights::new(len + 1, vec![10.0; 200]);
    map = map.update_height(&mut oracle3, 0, false, Some(&mut measured2));
    assert_eq!(map.size(), 220);
    assert!(map.depth() < 12);
}

#[test]
fn test_cm6_heightmap_16_can_handle_inserting_a_line_break() {
    let text = doc(&[3, 3, 3]);
    let mut oracle = o(&text);
    let mut measured1 = MeasuredHeights::new(0, vec![10.0, 10.0, 10.0]);
    let mut map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured1));
    assert_eq!(map.size(), 3);

    let text2 = text.replace(3..3, "\n");
    oracle.set_doc(text2);
    let mut measured2 = MeasuredHeights::new(0, vec![10.0, 10.0, 10.0, 10.0]);
    map = map
        .apply_changes(&[], &text, &mut oracle, &[ChangedRange::new(3, 3, 3, 4)])
        .update_height(&mut oracle, 0, false, Some(&mut measured2));
    assert_eq!(map.size(), 4);
    assert_eq!(map.height(), 40.0);
}

#[test]
fn test_cm6_heightmap_17_can_handle_insertion_in_the_middle_of_a_line() {
    let text = doc(&[3, 3, 3]);
    let mut oracle = o(&text);
    let mut measured1 = MeasuredHeights::new(0, vec![10.0, 10.0, 10.0]);
    let mut map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured1));

    let text2 = text.replace(5..5, "foo\nbar\nbaz\nbug");
    let mut oracle2 = o(&text2);
    let mut measured2 = MeasuredHeights::new(0, vec![10.0, 10.0, 10.0, 10.0, 10.0, 10.0]);
    map = map
        .apply_changes(&[], &text, &mut oracle2, &[ChangedRange::new(5, 5, 5, 20)])
        .update_height(&mut oracle2, 0, false, Some(&mut measured2));
    assert_eq!(map.size(), 6);
    assert_eq!(map.height(), 60.0);
}

#[test]
fn test_cm6_heightmap_18_properly_shrinks_gaps_when_partially_replaced() {
    let text = doc(&[9, 9, 9, 9, 9, 9, 9]);
    let mut oracle = o(&text);
    let mut measured = MeasuredHeights::new(20, vec![10.0, 10.0, 10.0]);
    let mut map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured));
    assert_eq!(map.to_string(), "gap(19) line(9) line(9) line(9) gap(19)");
    let deco_set = DecorationSet::of_sorted(
        vec![Decoration::replace_spec(15..55, ReplaceSpec::default())],
        true,
    )
    .unwrap();
    map = map.apply_changes(
        &[deco_set],
        &text,
        &mut oracle,
        &[ChangedRange::new(15, 55, 15, 55)],
    );
    assert_eq!(map.to_string(), "gap(9) line(49-40) gap(9)");
}

#[test]
fn test_cm6_heightmap_19_block_at_finds_blocks_in_a_gap() {
    let text = doc(&[3, 3, 3, 3, 3]);
    let map = mk(text.clone(), vec![]);
    let mut oracle = o(&text);
    let block1 = map.block_at(0.0, &mut oracle, 0.0, 0);
    assert_eq!(block1.from, 0);
    assert_eq!(block1.to(), 3);
    assert_eq!(block1.top, 0.0);
    assert!(block1.bottom() > 0.0);
    assert_eq!(block1.block_type, BlockType::Text);
    let block2 = map.block_at(block1.bottom() + 1.0, &mut oracle, 0.0, 0);
    assert_eq!(block2.from, 4);
    assert_eq!(block2.to(), 7);
    assert_eq!(block2.top, block1.bottom());
    assert!(block2.bottom() > block1.bottom());
    let block3 = map.block_at(1e9, &mut oracle, 0.0, 0);
    assert_eq!(block3.from, 16);
    assert_eq!(block3.to(), 19);
    assert_eq!(block3.bottom(), map.height());
}

#[test]
fn test_cm6_heightmap_20_block_at_finds_blocks_in_lines() {
    let text = doc(&[3, 3, 3, 3]);
    let mut oracle = o(&text);
    let mut measured = MeasuredHeights::new(0, vec![10.0, 20.0, 10.0, 30.0]);
    let map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured));
    let block1 = map.block_at(-100.0, &mut oracle, 0.0, 0);
    assert_eq!(block1.from, 0);
    assert_eq!(block1.to(), 3);
    assert_eq!(block1.top, 0.0);
    assert_eq!(block1.bottom(), 10.0);
    assert_eq!(block1.block_type, BlockType::Text);
    let block2 = map.block_at(39.0, &mut oracle, 0.0, 0);
    assert_eq!(block2.from, 8);
    assert_eq!(block2.to(), 11);
    assert_eq!(block2.top, 30.0);
    assert_eq!(block2.bottom(), 40.0);
    let block3 = map.block_at(77.0, &mut oracle, 0.0, 0);
    assert_eq!(block3.from, 12);
    assert_eq!(block3.to(), 15);
    assert_eq!(block3.top, 40.0);
    assert_eq!(block3.bottom(), 70.0);
}

#[test]
fn test_cm6_heightmap_21_block_at_finds_widget_blocks() {
    let text = doc(&[3, 3, 3, 3]);
    let map = mk(
        text.clone(),
        vec![
            Decoration::widget_spec(
                4,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(100.0))),
                    side: -1,
                    block: true,
                },
            ),
            Decoration::replace_spec(
                8..11,
                ReplaceSpec {
                    widget: Some(Arc::new(MyWidget(30.0))),
                    block: true,
                },
            ),
            Decoration::widget_spec(
                15,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(0.0))),
                    side: 1,
                    block: true,
                },
            ),
        ],
    );
    let mut oracle = o(&text);
    let block1 = map.block_at(0.0, &mut oracle, 0.0, 0);
    assert_eq!(block1.from, 0);
    assert_eq!(block1.to(), 3);
    let block2 = map.block_at(block1.height + 1.0, &mut oracle, 0.0, 0);
    assert_eq!(block2.from, 4);
    assert_eq!(block2.to(), 4);
    assert_eq!(block2.top, block1.height);
    assert_eq!(block2.height, 100.0);
    assert_eq!(block2.block_type, BlockType::WidgetBefore);
    let top3 = block2.bottom() + block1.height;
    let block3 = map.block_at(top3 + 10.0, &mut oracle, 0.0, 0);
    assert_eq!(block3.from, 8);
    assert_eq!(block3.to(), 11);
    assert_eq!(block3.top, top3);
    assert_eq!(block3.height, 30.0);
    assert_eq!(block3.block_type, BlockType::WidgetRange);
    let block4 = map.block_at(block3.bottom() + block1.height, &mut oracle, 0.0, 0);
    assert_eq!(block4.block_type, BlockType::WidgetAfter);
}

#[test]
fn test_cm6_heightmap_22_line_at_finds_lines_in_gaps() {
    let text = doc(&[3, 3, 3, 3]);
    let map = mk(text.clone(), vec![]);
    let mut oracle = o(&text);
    let line1 = map.line_at(0, QueryType::ByPos, &mut oracle, 0.0, 0);
    assert_eq!(line1.from, 0);
    assert_eq!(line1.to(), 3);
    assert_eq!(line1.top, 0.0);
    assert_eq!(
        map.line_at(0, QueryType::ByHeight, &mut oracle, 0.0, 0),
        line1
    );
    let line2 = map.line_at(line1.to() + 1, QueryType::ByPos, &mut oracle, 0.0, 0);
    assert_eq!(line2.from, 4);
    assert_eq!(line2.to(), 7);
    assert_eq!(line2.top, line1.bottom());
    assert_eq!(
        map.line_at(
            (line1.bottom() + 1.0) as usize,
            QueryType::ByHeight,
            &mut oracle,
            0.0,
            0
        ),
        line2
    );
    let line3 = map.line_at(15, QueryType::ByPos, &mut oracle, 0.0, 0);
    assert_eq!(line3.from, 12);
    assert_eq!(line3.to(), 15);
    assert_eq!(line3.bottom(), map.height());
    assert_eq!(
        map.line_at(1_000_000_000, QueryType::ByHeight, &mut oracle, 0.0, 0),
        line3
    );
}

#[test]
fn test_cm6_heightmap_23_line_at_finds_lines_in_lines() {
    let text = doc(&[3, 3, 3, 3]);
    let mut oracle = o(&text);
    let mut measured = MeasuredHeights::new(0, vec![10.0, 10.0, 20.0, 10.0]);
    let map =
        mk(text.clone(), vec![]).update_height(&mut oracle, 0, false, Some(&mut measured));
    let line1 = map.line_at(0, QueryType::ByPos, &mut oracle, 0.0, 0);
    assert_eq!(line1.from, 0);
    assert_eq!(line1.to(), 3);
    assert_eq!(line1.top, 0.0);
    assert_eq!(line1.bottom(), 10.0);
    assert_eq!(
        map.line_at(9, QueryType::ByHeight, &mut oracle, 0.0, 0),
        line1
    );
    let line2 = map.line_at(9, QueryType::ByPos, &mut oracle, 0.0, 0);
    assert_eq!(line2.from, 8);
    assert_eq!(line2.to(), 11);
    assert_eq!(line2.top, 20.0);
    assert_eq!(line2.bottom(), 40.0);
    assert_eq!(
        map.line_at(39, QueryType::ByHeight, &mut oracle, 0.0, 0),
        line2
    );
}

#[test]
fn test_cm6_heightmap_24_line_at_includes_adjacent_widgets_in_lines() {
    let text = doc(&[3, 3, 3, 3]);
    let mut oracle = o(&text);
    let map = mk(
        text.clone(),
        vec![
            Decoration::widget_spec(
                4,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(100.0))),
                    side: -1,
                    block: true,
                },
            ),
            Decoration::replace_spec(
                7..8,
                ReplaceSpec {
                    widget: Some(Arc::new(MyWidget(30.0))),
                    block: true,
                },
            ),
            Decoration::widget_spec(
                15,
                WidgetSpec {
                    widget: Some(Arc::new(MyWidget(0.0))),
                    side: 1,
                    block: true,
                },
            ),
        ],
    );
    let line1 = map.line_at(4, QueryType::ByPos, &mut oracle, 0.0, 0);
    assert_eq!(line1.from, 4);
    assert_eq!(line1.to(), 11);
    assert_eq!(line1.sub_blocks().len(), 4);
    assert_eq!(
        map.line_at(
            (line1.top + 1.0) as usize,
            QueryType::ByHeight,
            &mut oracle,
            0.0,
            0
        ),
        line1
    );
    assert_eq!(
        map.line_at(
            (line1.bottom() - 1.0) as usize,
            QueryType::ByHeight,
            &mut oracle,
            0.0,
            0
        ),
        line1
    );
    assert_eq!(
        map.line_at(
            (line1.top + line1.height / 2.0) as usize,
            QueryType::ByHeight,
            &mut oracle,
            0.0,
            0
        ),
        line1
    );
    assert_eq!(map.line_at(5, QueryType::ByPos, &mut oracle, 0.0, 0), line1);
    assert_eq!(map.line_at(7, QueryType::ByPos, &mut oracle, 0.0, 0), line1);
    assert_eq!(
        map.line_at(11, QueryType::ByPos, &mut oracle, 0.0, 0),
        line1
    );

    let line2 = map.line_at(
        map.height() as usize,
        QueryType::ByHeight,
        &mut oracle,
        0.0,
        0,
    );
    assert_eq!(line2.from, 12);
    assert_eq!(line2.to(), 15);
    assert_eq!(line2.sub_blocks().len(), 2);
    assert_eq!(
        map.line_at(
            (line2.top + 1.0) as usize,
            QueryType::ByHeight,
            &mut oracle,
            0.0,
            0
        ),
        line2
    );
}
