//! HeightMap data structure for viewport line height virtualization.
//! Faithful port of CodeMirror 6's `@codemirror/view/src/heightmap.ts`.

use crate::decoration::{Decoration, WidgetType};
use ferrum_state::rangeset::{RangeSet, RangeValue, SpanIterator};
use ferrum_state::text::Text;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub const FLAG_BREAK: u8 = 1;
pub const FLAG_OUTDATED: u8 = 2;
pub const FLAG_SINGLE_LINE: u8 = 4;
pub const EPSILON: f64 = 1e-3;

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
    ByPos,
    ByHeight,
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
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub from: usize,
    pub length: usize,
    pub top: f64,
    pub height: f64,
    pub block_type: BlockType,
    pub widget: Option<Arc<dyn WidgetType>>,
    pub widget_line_breaks: usize,
    pub sub_blocks: Vec<BlockInfo>,
}

impl PartialEq for BlockInfo {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from
            && self.length == other.length
            && (self.top - other.top).abs() < EPSILON
            && (self.height - other.height).abs() < EPSILON
            && self.block_type == other.block_type
            && self.widget_line_breaks == other.widget_line_breaks
            && self.sub_blocks == other.sub_blocks
            && match (&self.widget, &other.widget) {
                (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

impl BlockInfo {
    pub fn new(
        from: usize,
        length: usize,
        top: f64,
        height: f64,
        block_type: BlockType,
        widget: Option<Arc<dyn WidgetType>>,
        widget_line_breaks: usize,
        sub_blocks: Vec<BlockInfo>,
    ) -> Self {
        Self {
            from,
            length,
            top,
            height,
            block_type,
            widget,
            widget_line_breaks,
            sub_blocks,
        }
    }

    pub fn to(&self) -> usize {
        self.from + self.length
    }

    pub fn bottom(&self) -> f64 {
        self.top + self.height
    }

    pub fn sub_blocks(&self) -> &[BlockInfo] {
        &self.sub_blocks
    }

    pub fn join(&self, other: BlockInfo) -> BlockInfo {
        let other_len = other.length;
        let other_h = other.height;
        let mut sub = if !self.sub_blocks.is_empty() {
            self.sub_blocks.clone()
        } else {
            vec![self.clone()]
        };
        if !other.sub_blocks.is_empty() {
            sub.extend(other.sub_blocks);
        } else {
            sub.push(other);
        }
        BlockInfo {
            from: self.from,
            length: self.length + other_len,
            top: self.top,
            height: self.height + other_h,
            block_type: BlockType::Text,
            widget: None,
            widget_line_breaks: 0,
            sub_blocks: sub,
        }
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
    pub height_samples: HashMap<i64, bool>,
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
            height_samples: HashMap::new(),
        }
    }

    pub fn set_doc(&mut self, doc: Text) -> &mut Self {
        self.doc = doc;
        self
    }

    pub fn height_for_gap(&self, from: usize, to: usize) -> f64 {
        let lines = self.doc.line_at(to).number - self.doc.line_at(from).number + 1;
        let mut total_lines = lines as f64;
        if self.line_wrapping {
            let extra = ((to.saturating_sub(from) as f64) - (lines as f64 * self.line_length as f64 * 0.5))
                / self.line_length as f64;
            total_lines += extra.ceil().max(0.0);
        }
        self.line_height * total_lines
    }

    pub fn height_for_line(&self, length: usize) -> f64 {
        if !self.line_wrapping {
            return self.line_height;
        }
        let denom = (self.line_length.saturating_sub(5)).max(1) as f64;
        let extra = (length.saturating_sub(self.line_length) as f64 / denom).ceil().max(0.0);
        (1.0 + extra) * self.line_height
    }
}

fn replace_node(old: HeightMap, val: HeightMap) -> HeightMap {
    if old == val {
        return old;
    }
    if std::mem::discriminant(&old) != std::mem::discriminant(&val) {
        set_height_change_flag(true);
    }
    val
}

fn merge_gaps(nodes: &mut Vec<Option<HeightMap>>, around: usize) {
    if around > 0 && around + 1 < nodes.len() && nodes[around].is_none() {
        if let (Some(HeightMap::Gap(before)), Some(HeightMap::Gap(after))) =
            (&nodes[around - 1], &nodes[around + 1])
        {
            let merged = HeightMapGap::new(before.length + 1 + after.length);
            nodes.splice(around - 1..=around + 1, [Some(HeightMap::Gap(merged))]);
        }
    }
}

/// Leaf node representing a block-level widget or range replacement.
#[derive(Clone, Debug, PartialEq)]
pub struct HeightMapBlock {
    pub length: usize,
    pub height: f64,
    pub flags: u8,
    pub space_above: f64,
    pub deco: Option<Decoration>,
}

impl HeightMapBlock {
    pub fn new(length: usize, height: f64, deco: Option<Decoration>) -> Self {
        Self {
            length,
            height,
            flags: FLAG_OUTDATED,
            space_above: 0.0,
            deco,
        }
    }

    pub fn main_block(&self, top: f64, offset: usize) -> BlockInfo {
        let (block_type, widget) = if let Some(ref d) = self.deco {
            (d.block_type(), d.get_widget())
        } else {
            (BlockType::Text, None)
        };
        BlockInfo::new(
            offset,
            self.length,
            top + self.space_above,
            self.height - self.space_above,
            block_type,
            widget,
            0,
            vec![],
        )
    }

    pub fn block_at(&self, height: f64, _oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        if self.space_above > 0.0 && height < top + self.space_above {
            BlockInfo::new(
                offset,
                0,
                top,
                self.space_above,
                BlockType::WidgetRange,
                None,
                0,
                vec![],
            )
        } else {
            self.main_block(top, offset)
        }
    }

    pub fn line_at(&self, _value: usize, _type: QueryType, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        let main = self.main_block(top, offset);
        if self.space_above > 0.0 {
            self.block_at(0.0, oracle, top, offset).join(main)
        } else {
            main
        }
    }

    pub fn for_each_line<F: FnMut(BlockInfo)>(
        &self,
        from: usize,
        to: usize,
        oracle: &mut HeightOracle,
        top: f64,
        offset: usize,
        f: &mut F,
    ) {
        if from <= offset + self.length && to >= offset {
            f(self.line_at(0, QueryType::ByPos, oracle, top, offset));
        }
    }

    pub fn set_measured_height(&mut self, measured: &mut MeasuredHeights) {
        let mut next = measured.heights[measured.index];
        measured.index += 1;
        if next < 0.0 {
            self.space_above = -next;
            next = measured.heights[measured.index];
            measured.index += 1;
        } else {
            self.space_above = 0.0;
        }
        self.set_height(next);
    }

    pub fn set_height(&mut self, height: f64) {
        if self.height != height {
            if (self.height - height).abs() > EPSILON {
                set_height_change_flag(true);
            }
            self.height = height;
        }
    }

    pub fn set_outdated(&mut self, val: bool) {
        self.flags = (if val { FLAG_OUTDATED } else { 0 }) | (self.flags & !FLAG_OUTDATED);
    }

    pub fn update_height(
        &mut self,
        _oracle: &mut HeightOracle,
        offset: usize,
        _force: bool,
        measured: Option<&mut MeasuredHeights>,
    ) -> HeightMap {
        if let Some(m) = measured {
            if m.from <= offset && m.more() {
                self.set_measured_height(m);
            }
        }
        self.set_outdated(false);
        HeightMap::Block(self.clone())
    }
}

/// Leaf node representing a line of text (possibly with inline widgets/collapsed ranges).
#[derive(Clone, Debug, PartialEq)]
pub struct HeightMapText {
    pub length: usize,
    pub height: f64,
    pub flags: u8,
    pub space_above: f64,
    pub collapsed: usize,
    pub widget_height: f64,
    pub breaks: usize,
}

impl HeightMapText {
    pub fn new(length: usize, height: f64, space_above: f64) -> Self {
        Self {
            length,
            height,
            flags: FLAG_OUTDATED,
            space_above,
            collapsed: 0,
            widget_height: 0.0,
            breaks: 0,
        }
    }

    pub fn main_block(&self, top: f64, offset: usize) -> BlockInfo {
        BlockInfo::new(
            offset,
            self.length,
            top + self.space_above,
            self.height - self.space_above,
            BlockType::Text,
            None,
            self.breaks,
            vec![],
        )
    }

    pub fn block_at(&self, height: f64, _oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        if self.space_above > 0.0 && height < top + self.space_above {
            BlockInfo::new(
                offset,
                0,
                top,
                self.space_above,
                BlockType::WidgetRange,
                None,
                0,
                vec![],
            )
        } else {
            self.main_block(top, offset)
        }
    }

    pub fn line_at(&self, _value: usize, _type: QueryType, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        let main = self.main_block(top, offset);
        if self.space_above > 0.0 {
            self.block_at(0.0, oracle, top, offset).join(main)
        } else {
            main
        }
    }

    pub fn for_each_line<F: FnMut(BlockInfo)>(
        &self,
        from: usize,
        to: usize,
        oracle: &mut HeightOracle,
        top: f64,
        offset: usize,
        f: &mut F,
    ) {
        if from <= offset + self.length && to >= offset {
            f(self.line_at(0, QueryType::ByPos, oracle, top, offset));
        }
    }

    pub fn set_measured_height(&mut self, measured: &mut MeasuredHeights) {
        let mut next = measured.heights[measured.index];
        measured.index += 1;
        if next < 0.0 {
            self.space_above = -next;
            next = measured.heights[measured.index];
            measured.index += 1;
        } else {
            self.space_above = 0.0;
        }
        self.set_height(next);
    }

    pub fn set_height(&mut self, height: f64) {
        if self.height != height {
            if (self.height - height).abs() > EPSILON {
                set_height_change_flag(true);
            }
            self.height = height;
        }
    }

    pub fn set_outdated(&mut self, val: bool) {
        self.flags = (if val { FLAG_OUTDATED } else { 0 }) | (self.flags & !FLAG_OUTDATED);
    }

    pub fn replace(&self, _from: usize, _to: usize, nodes: Vec<Option<HeightMap>>) -> HeightMap {
        if nodes.len() == 1 {
            if let Some(Some(ref node)) = nodes.first() {
                match node {
                    HeightMap::Text(t) if (self.length as isize - t.length as isize).abs() < 10 => {
                        let mut updated = t.clone();
                        updated.height = self.height;
                        if (self.flags & FLAG_OUTDATED) == 0 {
                            updated.set_outdated(false);
                        }
                        return HeightMap::Text(updated);
                    }
                    HeightMap::Gap(g)
                        if (g.flags & FLAG_SINGLE_LINE) != 0
                            && (self.length as isize - g.length as isize).abs() < 10 =>
                    {
                        let mut updated = HeightMapText::new(g.length, self.height, self.space_above);
                        if (self.flags & FLAG_OUTDATED) == 0 {
                            updated.set_outdated(false);
                        }
                        return HeightMap::Text(updated);
                    }
                    _ => {}
                }
            }
        }
        HeightMap::of(nodes)
    }

    pub fn update_height(
        &mut self,
        oracle: &mut HeightOracle,
        offset: usize,
        force: bool,
        measured: Option<&mut MeasuredHeights>,
    ) -> HeightMap {
        if let Some(m) = measured {
            if m.from <= offset && m.more() {
                self.set_measured_height(m);
            }
        } else if force || (self.flags & FLAG_OUTDATED) != 0 {
            self.space_above = 0.0;
            let line_len = self.length.saturating_sub(self.collapsed);
            let line_h = oracle.height_for_line(line_len);
            let h = self.widget_height.max(line_h) + self.breaks as f64 * oracle.line_height;
            self.set_height(h);
        }
        self.set_outdated(false);
        HeightMap::Text(self.clone())
    }
}

/// Leaf node representing an unmeasured gap covering multiple plain text lines.
#[derive(Clone, Debug, PartialEq)]
pub struct HeightMapGap {
    pub length: usize,
    pub height: f64,
    pub flags: u8,
}

impl HeightMapGap {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            height: 0.0,
            flags: FLAG_OUTDATED,
        }
    }

    pub fn set_height(&mut self, height: f64) {
        if self.height != height {
            if (self.height - height).abs() > EPSILON {
                set_height_change_flag(true);
            }
            self.height = height;
        }
    }

    pub fn set_outdated(&mut self, val: bool) {
        self.flags = (if val { FLAG_OUTDATED } else { 0 }) | (self.flags & !FLAG_OUTDATED);
    }

    pub fn height_metrics(&self, oracle: &HeightOracle, offset: usize) -> (usize, usize, f64, f64) {
        let first_line = oracle.doc.line_at(offset).number;
        let last_line = oracle.doc.line_at(offset + self.length).number;
        let lines = last_line - first_line + 1;
        let per_line: f64;
        let mut per_char: f64 = 0.0;
        if oracle.line_wrapping {
            let total_per_line = self.height.min(oracle.line_height * lines as f64);
            per_line = total_per_line / lines as f64;
            if self.length > lines + 1 {
                per_char = (self.height - total_per_line) / (self.length - lines - 1) as f64;
            }
        } else {
            per_line = self.height / lines as f64;
        }
        (first_line, last_line, per_line, per_char)
    }

    pub fn block_at(&self, height: f64, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        let (first_line, last_line, per_line, per_char) = self.height_metrics(oracle, offset);
        if oracle.line_wrapping {
            let guess = if height < oracle.line_height {
                offset
            } else {
                let frac = if self.height > 0.0 {
                    ((height - top) / self.height).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                offset + (frac * self.length as f64).round() as usize
            };
            let line = oracle.doc.line_at(guess);
            let line_height = per_line + line.length as f64 * per_char;
            let line_top = top.max(height - line_height / 2.0);
            BlockInfo::new(line.from, line.length, line_top, line_height, BlockType::Text, None, 0, vec![])
        } else {
            let line_idx = if per_line > 0.0 {
                ((height - top) / per_line).floor() as isize
            } else {
                0
            };
            let clamped = line_idx.clamp(0, (last_line - first_line) as isize) as usize;
            let line = oracle.doc.line(first_line + clamped);
            BlockInfo::new(line.from, line.length, top + per_line * clamped as f64, per_line, BlockType::Text, None, 0, vec![])
        }
    }

    pub fn line_at(&self, value: usize, query: QueryType, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        if query == QueryType::ByHeight {
            return self.block_at(value as f64, oracle, top, offset);
        }
        if query == QueryType::ByPosNoHeight {
            let line = oracle.doc.line_at(value);
            return BlockInfo::new(line.from, line.length, 0.0, 0.0, BlockType::Text, None, 0, vec![]);
        }
        let (first_line, _last_line, per_line, per_char) = self.height_metrics(oracle, offset);
        let line = oracle.doc.line_at(value);
        let line_height = per_line + line.length as f64 * per_char;
        let lines_above = line.number - first_line;
        let line_top = top + per_line * lines_above as f64 + per_char * (line.from.saturating_sub(offset + lines_above)) as f64;
        let clamped_top = top.max((top + self.height - line_height).min(line_top));
        BlockInfo::new(line.from, line.length, clamped_top, line_height, BlockType::Text, None, 0, vec![])
    }

    pub fn for_each_line<F: FnMut(BlockInfo)>(
        &self,
        from: usize,
        to: usize,
        oracle: &mut HeightOracle,
        top: f64,
        offset: usize,
        f: &mut F,
    ) {
        let from = from.max(offset);
        let to = to.min(offset + self.length);
        let (first_line, _last_line, per_line, per_char) = self.height_metrics(oracle, offset);
        let mut pos = from;
        let mut line_top = top;
        while pos <= to {
            let line = oracle.doc.line_at(pos);
            if pos == from {
                let lines_above = line.number - first_line;
                line_top += per_line * lines_above as f64 + per_char * (from.saturating_sub(offset + lines_above)) as f64;
            }
            let line_height = per_line + per_char * line.length as f64;
            f(BlockInfo::new(line.from, line.length, line_top, line_height, BlockType::Text, None, 0, vec![]));
            line_top += line_height;
            pos = line.to + 1;
        }
    }

    pub fn replace(&self, from: usize, to: usize, mut nodes: Vec<Option<HeightMap>>) -> HeightMap {
        let after = self.length.saturating_sub(to);
        if after > 0 {
            let last = nodes.last_mut().and_then(|opt| opt.as_mut());
            if let Some(HeightMap::Gap(g)) = last {
                g.length += after;
            } else {
                nodes.push(None);
                nodes.push(Some(HeightMap::Gap(HeightMapGap::new(after - 1))));
            }
        }
        if from > 0 {
            let first = nodes.first_mut().and_then(|opt| opt.as_mut());
            if let Some(HeightMap::Gap(g)) = first {
                g.length += from;
            } else {
                nodes.insert(0, None);
                nodes.insert(0, Some(HeightMap::Gap(HeightMapGap::new(from - 1))));
            }
        }
        HeightMap::of(nodes)
    }

    pub fn decompose_left(&self, to: usize, result: &mut Vec<Option<HeightMap>>) {
        result.push(Some(HeightMap::Gap(HeightMapGap::new(to - 1))));
        result.push(None);
    }

    pub fn decompose_right(&self, from: usize, result: &mut Vec<Option<HeightMap>>) {
        result.push(None);
        result.push(Some(HeightMap::Gap(HeightMapGap::new(self.length - from - 1))));
    }

    pub fn update_height(
        &mut self,
        oracle: &mut HeightOracle,
        offset: usize,
        force: bool,
        measured: Option<&mut MeasuredHeights>,
    ) -> HeightMap {
        let end = offset + self.length;
        if let Some(measured) = measured {
            if measured.from <= offset + self.length && measured.more() {
                let mut nodes: Vec<Option<HeightMap>> = Vec::new();
                let mut pos = offset.max(measured.from);
                let mut single_height: f64 = -1.0;
                if measured.from > offset {
                    let mut gap = HeightMapGap::new(measured.from - offset - 1);
                    let updated = gap.update_height(oracle, offset, false, None);
                    nodes.push(Some(updated));
                }
                while pos <= end && measured.more() {
                    let len = oracle.doc.line_at(pos).length;
                    if !nodes.is_empty() {
                        nodes.push(None);
                    }
                    let mut height = measured.heights[measured.index];
                    measured.index += 1;
                    let mut above = 0.0;
                    if height < 0.0 {
                        above = -height;
                        height = measured.heights[measured.index];
                        measured.index += 1;
                    }
                    if single_height == -1.0 {
                        single_height = height;
                    } else if (height - single_height).abs() >= EPSILON {
                        single_height = -2.0;
                    }
                    let mut line = HeightMapText::new(len, height, above);
                    line.set_outdated(false);
                    nodes.push(Some(HeightMap::Text(line)));
                    pos += len + 1;
                }
                if pos <= end {
                    let mut gap = HeightMapGap::new(end - pos);
                    let updated = gap.update_height(oracle, pos, false, None);
                    nodes.push(None);
                    nodes.push(Some(updated));
                }
                let result = HeightMap::of(nodes);
                let (_first, _last, per_line, _per_char) = self.height_metrics(oracle, offset);
                if single_height < 0.0
                    || (result.height() - self.height).abs() >= EPSILON
                    || (single_height - per_line).abs() >= EPSILON
                {
                    set_height_change_flag(true);
                }
                return replace_node(HeightMap::Gap(self.clone()), result);
            }
        }
        if force || (self.flags & FLAG_OUTDATED) != 0 {
            self.set_height(oracle.height_for_gap(offset, offset + self.length));
            self.set_outdated(false);
        }
        HeightMap::Gap(self.clone())
    }
}

/// Internal branch node combining two subtrees with or without a line break.
#[derive(Clone, Debug, PartialEq)]
pub struct HeightMapBranch {
    pub length: usize,
    pub height: f64,
    pub flags: u8,
    pub size: usize,
    pub break_flag: bool,
    pub left: Box<HeightMap>,
    pub right: Box<HeightMap>,
}

impl HeightMapBranch {
    pub fn break_char(&self) -> usize {
        if self.break_flag { 1 } else { 0 }
    }

    pub fn set_height(&mut self, height: f64) {
        if self.height != height {
            if (self.height - height).abs() > EPSILON {
                set_height_change_flag(true);
            }
            self.height = height;
        }
    }

    pub fn set_outdated(&mut self, val: bool) {
        self.flags = (if val { FLAG_OUTDATED } else { 0 }) | (self.flags & !FLAG_OUTDATED);
    }

    pub fn block_at(&self, height: f64, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        let mid = top + self.left.height();
        if height < mid {
            self.left.block_at(height, oracle, top, offset)
        } else {
            self.right.block_at(height, oracle, mid, offset + self.left.length() + self.break_char())
        }
    }

    pub fn line_at(&self, value: usize, query: QueryType, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        let right_top = top + self.left.height();
        let right_offset = offset + self.left.length() + self.break_char();
        let left = match query {
            QueryType::ByHeight => (value as f64) < right_top,
            _ => value < right_offset,
        };
        let base = if left {
            self.left.line_at(value, query, oracle, top, offset)
        } else {
            self.right.line_at(value, query, oracle, right_top, right_offset)
        };
        if self.break_flag || if left { base.to() < right_offset } else { base.from > right_offset } {
            return base;
        }
        let sub_query = if query == QueryType::ByPosNoHeight { QueryType::ByPosNoHeight } else { QueryType::ByPos };
        if left {
            base.join(self.right.line_at(right_offset, sub_query, oracle, right_top, right_offset))
        } else {
            self.left.line_at(right_offset, sub_query, oracle, top, offset).join(base)
        }
    }

    pub fn for_each_line<F: FnMut(BlockInfo)>(
        &self,
        from: usize,
        to: usize,
        oracle: &mut HeightOracle,
        top: f64,
        offset: usize,
        f: &mut F,
    ) {
        let right_top = top + self.left.height();
        let right_offset = offset + self.left.length() + self.break_char();
        if self.break_flag {
            if from < right_offset {
                self.left.for_each_line(from, to, oracle, top, offset, f);
            }
            if to >= right_offset {
                self.right.for_each_line(from, to, oracle, right_top, right_offset, f);
            }
        } else {
            let mid = self.line_at(right_offset, QueryType::ByPos, oracle, top, offset);
            if from < mid.from {
                self.left.for_each_line(from, to.min(mid.from.saturating_sub(1)), oracle, top, offset, f);
            }
            if mid.to() >= from && mid.from <= to {
                f(mid.clone());
            }
            if to > mid.to() {
                self.right.for_each_line(from.max(mid.to() + 1), to, oracle, right_top, right_offset, f);
            }
        }
    }

    pub fn replace(&self, from: usize, to: usize, nodes: Vec<Option<HeightMap>>) -> HeightMap {
        let right_start = self.left.length() + self.break_char();
        if to < right_start {
            let new_left = self.left.replace(from, to, nodes);
            let mut branch = self.clone();
            return branch.balanced(new_left, (*self.right).clone());
        }
        if from > self.left.length() {
            let new_right = self.right.replace(from - right_start, to - right_start, nodes);
            let mut branch = self.clone();
            return branch.balanced((*self.left).clone(), new_right);
        }
        let mut result: Vec<Option<HeightMap>> = Vec::new();
        if from > 0 {
            self.decompose_left(from, &mut result);
        }
        let left_len = result.len();
        for node in nodes {
            result.push(node);
        }
        if from > 0 {
            merge_gaps(&mut result, left_len.saturating_sub(1));
        }
        if to < self.length {
            let right_len = result.len();
            self.decompose_right(to, &mut result);
            merge_gaps(&mut result, right_len);
        }
        HeightMap::of(result)
    }

    pub fn decompose_left(&self, to: usize, result: &mut Vec<Option<HeightMap>>) {
        let mut left = self.left.length();
        if to <= left {
            return self.left.decompose_left(to, result);
        }
        result.push(Some((*self.left).clone()));
        if self.break_flag {
            left += 1;
            if to >= left {
                result.push(None);
            }
        }
        if to > left {
            self.right.decompose_left(to - left, result);
        }
    }

    pub fn decompose_right(&self, from: usize, result: &mut Vec<Option<HeightMap>>) {
        let left = self.left.length();
        let right = left + self.break_char();
        if from >= right {
            return self.right.decompose_right(from - right, result);
        }
        if from < left {
            self.left.decompose_right(from, result);
        }
        if self.break_flag && from < right {
            result.push(None);
        }
        result.push(Some((*self.right).clone()));
    }

    pub fn balanced(&mut self, left: HeightMap, right: HeightMap) -> HeightMap {
        if left.size() > 2 * right.size() || right.size() > 2 * left.size() {
            return HeightMap::of(if self.break_flag {
                vec![Some(left), None, Some(right)]
            } else {
                vec![Some(left), Some(right)]
            });
        }
        let replaced_left = replace_node((*self.left).clone(), left);
        let replaced_right = replace_node((*self.right).clone(), right);
        let h = replaced_left.height() + replaced_right.height();
        let sz = replaced_left.size() + replaced_right.size();
        let len = replaced_left.length() + self.break_char() + replaced_right.length();
        let outdated = replaced_left.outdated() || replaced_right.outdated();
        self.left = Box::new(replaced_left);
        self.right = Box::new(replaced_right);
        self.set_height(h);
        self.set_outdated(outdated);
        self.size = sz;
        self.length = len;
        HeightMap::Branch(self.clone())
    }

    pub fn update_height(
        &mut self,
        oracle: &mut HeightOracle,
        offset: usize,
        force: bool,
        mut measured: Option<&mut MeasuredHeights>,
    ) -> HeightMap {
        let right_start = offset + self.left.length() + self.break_char();
        let mut rebalance = false;
        let new_left = if let Some(ref mut m) = measured {
            if m.from <= offset + self.left.length() && m.more() {
                rebalance = true;
                self.left.update_height(oracle, offset, force, Some(m))
            } else {
                self.left.update_height(oracle, offset, force, None)
            }
        } else {
            self.left.update_height(oracle, offset, force, None)
        };
        let new_right = if let Some(ref mut m) = measured {
            if m.from <= right_start + self.right.length() && m.more() {
                rebalance = true;
                self.right.update_height(oracle, right_start, force, Some(m))
            } else {
                self.right.update_height(oracle, right_start, force, None)
            }
        } else {
            self.right.update_height(oracle, right_start, force, None)
        };
        if rebalance {
            return self.balanced(new_left, new_right);
        }
        self.left = Box::new(new_left);
        self.right = Box::new(new_right);
        self.height = self.left.height() + self.right.height();
        self.set_outdated(false);
        HeightMap::Branch(self.clone())
    }
}

/// Virtualized height map storing line heights and gaps in a tree structure.
#[derive(Debug, Clone, PartialEq)]
pub enum HeightMap {
    Block(HeightMapBlock),
    Text(HeightMapText),
    Gap(HeightMapGap),
    Branch(HeightMapBranch),
}

impl Default for HeightMap {
    fn default() -> Self {
        Self::empty()
    }
}

impl HeightMap {
    /// Creates an empty height map.
    pub fn empty() -> Self {
        HeightMap::Text(HeightMapText::new(0, 0.0, 0.0))
    }

    pub fn length(&self) -> usize {
        match self {
            HeightMap::Block(b) => b.length,
            HeightMap::Text(t) => t.length,
            HeightMap::Gap(g) => g.length,
            HeightMap::Branch(b) => b.length,
        }
    }

    pub fn height(&self) -> f64 {
        match self {
            HeightMap::Block(b) => b.height,
            HeightMap::Text(t) => t.height,
            HeightMap::Gap(g) => g.height,
            HeightMap::Branch(b) => b.height,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            HeightMap::Block(_) | HeightMap::Text(_) | HeightMap::Gap(_) => 1,
            HeightMap::Branch(b) => b.size,
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            HeightMap::Block(_) | HeightMap::Text(_) | HeightMap::Gap(_) => 1,
            HeightMap::Branch(b) => b.left.depth().max(b.right.depth()) + 1,
        }
    }

    pub fn outdated(&self) -> bool {
        match self {
            HeightMap::Block(b) => (b.flags & FLAG_OUTDATED) != 0,
            HeightMap::Text(t) => (t.flags & FLAG_OUTDATED) != 0,
            HeightMap::Gap(g) => (g.flags & FLAG_OUTDATED) != 0,
            HeightMap::Branch(b) => (b.flags & FLAG_OUTDATED) != 0,
        }
    }

    pub fn set_outdated(&mut self, val: bool) {
        match self {
            HeightMap::Block(b) => b.set_outdated(val),
            HeightMap::Text(t) => t.set_outdated(val),
            HeightMap::Gap(g) => g.set_outdated(val),
            HeightMap::Branch(b) => b.set_outdated(val),
        }
    }

    pub fn set_height(&mut self, height: f64) {
        match self {
            HeightMap::Block(b) => b.set_height(height),
            HeightMap::Text(t) => t.set_height(height),
            HeightMap::Gap(g) => g.set_height(height),
            HeightMap::Branch(b) => b.set_height(height),
        }
    }

    pub fn block_at(&self, height: f64, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        match self {
            HeightMap::Block(b) => b.block_at(height, oracle, top, offset),
            HeightMap::Text(t) => t.block_at(height, oracle, top, offset),
            HeightMap::Gap(g) => g.block_at(height, oracle, top, offset),
            HeightMap::Branch(b) => b.block_at(height, oracle, top, offset),
        }
    }

    pub fn line_at(&self, value: usize, query: QueryType, oracle: &mut HeightOracle, top: f64, offset: usize) -> BlockInfo {
        match self {
            HeightMap::Block(b) => b.line_at(value, query, oracle, top, offset),
            HeightMap::Text(t) => t.line_at(value, query, oracle, top, offset),
            HeightMap::Gap(g) => g.line_at(value, query, oracle, top, offset),
            HeightMap::Branch(b) => b.line_at(value, query, oracle, top, offset),
        }
    }

    pub fn for_each_line<F: FnMut(BlockInfo)>(
        &self,
        from: usize,
        to: usize,
        oracle: &mut HeightOracle,
        top: f64,
        offset: usize,
        f: &mut F,
    ) {
        match self {
            HeightMap::Block(b) => b.for_each_line(from, to, oracle, top, offset, f),
            HeightMap::Text(t) => t.for_each_line(from, to, oracle, top, offset, f),
            HeightMap::Gap(g) => g.for_each_line(from, to, oracle, top, offset, f),
            HeightMap::Branch(b) => b.for_each_line(from, to, oracle, top, offset, f),
        }
    }

    pub fn update_height(
        &mut self,
        oracle: &mut HeightOracle,
        offset: usize,
        force: bool,
        measured: Option<&mut MeasuredHeights>,
    ) -> HeightMap {
        match self {
            HeightMap::Block(b) => b.update_height(oracle, offset, force, measured),
            HeightMap::Text(t) => t.update_height(oracle, offset, force, measured),
            HeightMap::Gap(g) => g.update_height(oracle, offset, force, measured),
            HeightMap::Branch(b) => b.update_height(oracle, offset, force, measured),
        }
    }

    pub fn replace(&self, from: usize, to: usize, nodes: Vec<Option<HeightMap>>) -> HeightMap {
        match self {
            HeightMap::Block(_) => HeightMap::of(nodes),
            HeightMap::Text(t) => t.replace(from, to, nodes),
            HeightMap::Gap(g) => g.replace(from, to, nodes),
            HeightMap::Branch(b) => b.replace(from, to, nodes),
        }
    }

    pub fn decompose_left(&self, to: usize, result: &mut Vec<Option<HeightMap>>) {
        match self {
            HeightMap::Block(_) | HeightMap::Text(_) => result.push(Some(self.clone())),
            HeightMap::Gap(g) => g.decompose_left(to, result),
            HeightMap::Branch(b) => b.decompose_left(to, result),
        }
    }

    pub fn decompose_right(&self, from: usize, result: &mut Vec<Option<HeightMap>>) {
        match self {
            HeightMap::Block(_) | HeightMap::Text(_) => result.push(Some(self.clone())),
            HeightMap::Gap(g) => g.decompose_right(from, result),
            HeightMap::Branch(b) => b.decompose_right(from, result),
        }
    }

    pub fn branch(left: HeightMap, brk: bool, right: HeightMap) -> HeightMap {
        let length = left.length() + (if brk { 1 } else { 0 }) + right.length();
        let height = left.height() + right.height();
        let size = left.size() + right.size();
        let flags = (if brk { FLAG_BREAK } else { 0 })
            | (if left.outdated() || right.outdated() { FLAG_OUTDATED } else { 0 });
        HeightMap::Branch(HeightMapBranch {
            length,
            height,
            flags,
            size,
            break_flag: brk,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn of(mut nodes: Vec<Option<HeightMap>>) -> HeightMap {
        if nodes.len() == 1 {
            return nodes.pop().unwrap().unwrap();
        }

        let mut i = 0;
        let mut j = nodes.len();
        let mut before = 0;
        let mut after = 0;

        loop {
            if i == j {
                if before > after * 2 {
                    if let Some(HeightMap::Branch(split)) = nodes.get(i - 1).cloned().flatten() {
                        nodes.remove(i - 1);
                        i -= 1;
                        if split.break_flag {
                            nodes.insert(i, Some(*split.right));
                            nodes.insert(i, None);
                            nodes.insert(i, Some(*split.left));
                            j += 2;
                        } else {
                            nodes.insert(i, Some(*split.right));
                            nodes.insert(i, Some(*split.left));
                            j += 1;
                        }
                        before -= split.size;
                        continue;
                    }
                    break;
                } else if after > before * 2 {
                    if let Some(HeightMap::Branch(split)) = nodes.get(j).cloned().flatten() {
                        nodes.remove(j);
                        if split.break_flag {
                            nodes.insert(j, Some(*split.right));
                            nodes.insert(j, None);
                            nodes.insert(j, Some(*split.left));
                            j += 3;
                        } else {
                            nodes.insert(j, Some(*split.right));
                            nodes.insert(j, Some(*split.left));
                            j += 2;
                        }
                        after -= split.size;
                        continue;
                    }
                    break;
                } else {
                    break;
                }
            } else if before < after {
                if let Some(ref next) = nodes[i] {
                    before += next.size();
                }
                i += 1;
            } else {
                j -= 1;
                if let Some(ref next) = nodes[j] {
                    after += next.size();
                }
            }
        }

        let mut brk = false;
        if i > 0 && nodes[i - 1].is_none() {
            brk = true;
            i -= 1;
        } else if j < nodes.len() && nodes[j].is_none() {
            brk = true;
            j += 1;
        }
        let left = HeightMap::of(nodes[0..i].to_vec());
        let right = HeightMap::of(nodes[j..].to_vec());
        HeightMap::branch(left, brk, right)
    }

    pub fn apply_changes(
        &self,
        decorations: &[crate::decoration::DecorationSet],
        old_doc: &Text,
        oracle: &mut HeightOracle,
        changes: &[ChangedRange],
    ) -> HeightMap {
        let mut me = self.clone();
        let doc = oracle.doc.clone();
        let mut i = changes.len();
        while i > 0 {
            i -= 1;
            let mut from_a = changes[i].from_a;
            let mut to_a = changes[i].to_a;
            let mut from_b = changes[i].from_b;
            let mut to_b = changes[i].to_b;

            oracle.set_doc(old_doc.clone());
            let mut start = me.line_at(from_a, QueryType::ByPosNoHeight, oracle, 0.0, 0);
            let end = if start.to() >= to_a {
                start.clone()
            } else {
                me.line_at(to_a, QueryType::ByPosNoHeight, oracle, 0.0, 0)
            };
            to_b += end.to() - to_a;
            to_a = end.to();

            while i > 0 && start.from <= changes[i - 1].to_a {
                from_a = changes[i - 1].from_a;
                from_b = changes[i - 1].from_b;
                i -= 1;
                if from_a < start.from {
                    start = me.line_at(from_a, QueryType::ByPosNoHeight, oracle, 0.0, 0);
                }
            }
            from_b = from_b.saturating_sub(from_a.saturating_sub(start.from));
            from_a = start.from;

            oracle.set_doc(doc.clone());
            let nodes = NodeBuilder::build(oracle, decorations, from_b, to_b);
            let replaced = me.replace(from_a, to_a, nodes);
            me = replace_node(me, replaced);
        }
        me.update_height(oracle, 0, false, None)
    }
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeightMap::Block(b) => write!(f, "block({})", b.length),
            HeightMap::Text(t) => {
                let col = if t.collapsed > 0 {
                    format!("-{}", t.collapsed)
                } else {
                    String::new()
                };
                let wh = if t.widget_height > 0.0 {
                    format!(":{}", t.widget_height)
                } else {
                    String::new()
                };
                write!(f, "line({}{}{})", t.length, col, wh)
            }
            HeightMap::Gap(g) => write!(f, "gap({})", g.length),
            HeightMap::Branch(b) => {
                let sep = if b.break_flag { " " } else { "-" };
                write!(f, "{}{}{}", b.left, sep, b.right)
            }
        }
    }
}

pub struct NodeBuilder {
    pub pos: usize,
    pub oracle: HeightOracle,
    pub nodes: Vec<Option<HeightMap>>,
    pub written_to: usize,
    pub line_start: isize,
    pub line_end: isize,
    pub covering: Option<HeightMapBlock>,
}

impl NodeBuilder {
    pub fn new(pos: usize, oracle: HeightOracle) -> Self {
        Self {
            pos,
            oracle,
            nodes: Vec::new(),
            written_to: pos,
            line_start: -1,
            line_end: -1,
            covering: None,
        }
    }

    pub fn is_covered(&self) -> bool {
        self.covering.is_some()
            && self
                .nodes
                .last()
                .and_then(|opt| opt.as_ref())
                .map_or(false, |last| match last {
                    HeightMap::Block(b) => Some(b) == self.covering.as_ref(),
                    _ => false,
                })
    }

    pub fn enter_line(&mut self) {
        if self.line_start > -1 {
            return;
        }
        let line = self.oracle.doc.line_at(self.pos);
        let from = line.from;
        let to = line.to;
        self.line_start = from as isize;
        self.line_end = to as isize;
        if self.written_to < from {
            if self.written_to < from.saturating_sub(1) || self.nodes.last().map_or(true, |opt| opt.is_none()) {
                self.nodes.push(Some(self.blank_content(self.written_to, from - 1)));
            }
            self.nodes.push(None);
        }
        if self.pos > from {
            self.nodes.push(Some(HeightMap::Text(HeightMapText::new(self.pos - from, -1.0, 0.0))));
        }
        self.written_to = self.pos;
    }

    pub fn blank_content(&self, from: usize, to: usize) -> HeightMap {
        let mut gap = HeightMapGap::new(to - from);
        if self.oracle.doc.line_at(from).to == to {
            gap.flags |= FLAG_SINGLE_LINE;
        }
        HeightMap::Gap(gap)
    }

    pub fn ensure_line(&mut self) {
        self.enter_line();
        if let Some(Some(HeightMap::Text(_))) = self.nodes.last() {
            // Already a text line
        } else {
            self.nodes.push(Some(HeightMap::Text(HeightMapText::new(0, -1.0, 0.0))));
        }
    }

    pub fn add_block(&mut self, block: HeightMapBlock) {
        self.enter_line();
        let deco = block.deco.clone();
        if let Some(ref d) = deco {
            if d.start_side() > 0 && !self.is_covered() {
                self.ensure_line();
            }
        }
        let block_len = block.length;
        self.nodes.push(Some(HeightMap::Block(block.clone())));
        self.pos += block_len;
        self.written_to = self.pos;
        if let Some(ref d) = deco {
            if d.end_side() > 0 {
                self.covering = Some(block);
            }
        }
    }

    pub fn add_line_deco(&mut self, height: f64, breaks: usize, length: usize) {
        self.ensure_line();
        if let Some(Some(HeightMap::Text(ref mut line))) = self.nodes.last_mut() {
            line.length += length;
            line.collapsed += length;
            line.widget_height = line.widget_height.max(height);
            line.breaks += breaks;
        }
        self.pos += length;
        self.written_to = self.pos;
    }

    pub fn finish(&mut self, from: usize) -> Vec<Option<HeightMap>> {
        let last = self.nodes.last();
        let is_text = last.map_or(false, |opt| matches!(opt, Some(HeightMap::Text(_))));
        let is_cov = self.is_covered();
        if self.line_start > -1 && !is_text && !is_cov {
            self.nodes.push(Some(HeightMap::Text(HeightMapText::new(0, -1.0, 0.0))));
        } else if self.written_to < self.pos || last.map_or(true, |opt| opt.is_none()) {
            let blank = self.blank_content(self.written_to, self.pos);
            self.nodes.push(Some(blank));
        }
        let mut pos = from;
        for opt in &mut self.nodes {
            if let Some(ref mut node) = opt {
                if let HeightMap::Text(ref mut t) = node {
                    let updated = t.update_height(&mut self.oracle, pos, false, None);
                    *node = updated;
                }
                pos += node.length();
            } else {
                pos += 1;
            }
        }
        std::mem::take(&mut self.nodes)
    }

    pub fn build(
        oracle: &mut HeightOracle,
        decorations: &[crate::decoration::DecorationSet],
        from: usize,
        to: usize,
    ) -> Vec<Option<HeightMap>> {
        let mut builder = NodeBuilder::new(from, oracle.clone());
        RangeSet::spans(decorations, from, to, &mut builder, 0);
        builder.finish(from)
    }
}

impl SpanIterator<Decoration> for NodeBuilder {
    fn span(&mut self, _from: usize, to: usize, _active: &[Decoration], _open_start: usize) {
        if self.line_start > -1 {
            let end = to.min(self.line_end as usize);
            let is_cov = self.is_covered();
            let last = self.nodes.last_mut().and_then(|opt| opt.as_mut());
            match last {
                Some(HeightMap::Text(t)) => {
                    t.length += end - self.pos;
                }
                _ => {
                    if end > self.pos || !is_cov {
                        self.nodes.push(Some(HeightMap::Text(HeightMapText::new(end - self.pos, -1.0, 0.0))));
                    }
                }
            }
            self.written_to = end;
            if to > end {
                self.nodes.push(None);
                self.written_to += 1;
                self.line_start = -1;
            }
        }
        self.pos = to;
    }

    fn point(
        &mut self,
        from: usize,
        to: usize,
        deco: &Decoration,
        _active: &[Decoration],
        _open_start: usize,
        _index: usize,
    ) {
        if from < to || deco.height_relevant() {
            let mut height = deco.get_widget().as_ref().map_or(0.0, |w| w.estimated_height());
            let breaks = deco.get_widget().as_ref().map_or(0, |w| w.line_breaks());
            if height < 0.0 {
                height = self.oracle.line_height;
            }
            let len = to - from;
            if deco.block() {
                self.add_block(HeightMapBlock::new(len, height, Some(deco.clone())));
            } else if len > 0 || breaks > 0 || height >= 5.0 {
                self.add_line_deco(height, breaks, len);
            }
        } else if to > from {
            self.span(from, to, &[], 0);
        }
        if self.line_end > -1 && (self.line_end as usize) < self.pos {
            self.line_end = self.oracle.doc.line_at(self.pos).to as isize;
        }
    }
}
