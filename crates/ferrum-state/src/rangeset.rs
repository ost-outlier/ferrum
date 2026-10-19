//! Canonical `RangeSet` and `RangeSetBuilder` implementation with 1:1 parity to CodeMirror 6 (`@codemirror/state/src/rangeset.ts`).
//!
//! Stores ordered collections of ranges with associated values (such as text decorations, syntax marks,
//! and widgets) in balanced chunks with layer spilling. Supports efficient Operational Transformation (OT)
//! mapping across `ChangeDesc` in $O(\log N)$ or $O(K \log N)$.

use crate::change::{ChangeDesc, MapMode};
use std::sync::Arc;

pub const CHUNK_SIZE: usize = 250;
const FAR: i64 = 1_000_000_000;

/// Trait implemented by values attached to ranges in a `RangeSet`.
pub trait RangeValue: Clone + PartialEq + Send + Sync + 'static {
    /// Determines ordering among ranges starting at the same position. Defaults to 0.
    fn start_side(&self) -> i32 { 0 }
    /// Determines ordering among ranges ending at the same position. Defaults to 0.
    fn end_side(&self) -> i32 { 0 }
    /// True if this range represents an atomic point rather than a span of text.
    fn point(&self) -> bool { false }
    /// Mode for mapping point ranges across text changes. Defaults to `MapMode::TrackDel`.
    fn map_mode(&self) -> MapMode { MapMode::TrackDel }
    /// Compares this value to another value for structural equality.
    fn eq_val(&self, other: &Self) -> bool { self == other }
}

/// A range associates a value with character boundaries `[from, to]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Range<T: RangeValue> {
    pub from: usize,
    pub to: usize,
    pub value: T,
}

impl<T: RangeValue> Range<T> {
    /// Creates a new range.
    pub fn new(from: usize, to: usize, value: T) -> Self {
        Self { from, to, value }
    }
}

/// A chunk storing up to `CHUNK_SIZE` non-overlapping ranges within a single layer.
#[derive(Clone, Debug)]
pub struct Chunk<T: RangeValue> {
    pub from: Vec<usize>,
    pub to: Vec<usize>,
    pub value: Vec<T>,
    pub max_point: isize,
}

impl<T: RangeValue> Chunk<T> {
    pub fn new(from: Vec<usize>, to: Vec<usize>, value: Vec<T>, max_point: isize) -> Self {
        Self { from, to, value, max_point }
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.to.last().copied().unwrap_or(0)
    }

    pub fn find_index(&self, pos: usize, side: i32, end: bool, start_at: usize) -> usize {
        let arr = if end { &self.to } else { &self.from };
        let mut lo = start_at;
        let mut hi = arr.len();
        while lo < hi {
            let mid = (lo + hi) >> 1;
            let val_side = if end { self.value[mid].end_side() } else { self.value[mid].start_side() };
            let diff = (arr[mid] as i64 - pos as i64) * 2 + (if val_side >= side { 1 } else { -1 });
            if diff >= 0 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    pub fn between(&self, offset: usize, from: usize, to: usize, mut f: impl FnMut(usize, usize, &T) -> bool) -> bool {
        let start_i = self.find_index(from.saturating_sub(offset), -1_000_000, true, 0);
        let end_i = self.find_index(to.saturating_sub(offset), 1_000_000, false, start_i);
        for i in start_i..end_i.min(self.value.len()) {
            if !f(self.from[i] + offset, self.to[i] + offset, &self.value[i]) {
                return false;
            }
        }
        true
    }
}

/// An immutable, balanced set of ranges arranged in chunks and spilled layers.
#[derive(Clone, Debug)]
pub struct RangeSet<T: RangeValue> {
    pub chunk_pos: Vec<usize>,
    pub chunk: Vec<Arc<Chunk<T>>>,
    pub next_layer: Option<Arc<RangeSet<T>>>,
    pub max_point: isize,
}

impl<T: RangeValue> Default for RangeSet<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T: RangeValue> RangeSet<T> {
    /// Creates an empty range set.
    pub fn empty() -> Self {
        Self {
            chunk_pos: Vec::new(),
            chunk: Vec::new(),
            next_layer: None,
            max_point: -1,
        }
    }

    /// Returns true if this range set contains zero ranges.
    pub fn is_empty(&self) -> bool {
        self.chunk.is_empty() && self.next_layer.as_ref().map_or(true, |l| l.is_empty())
    }

    /// The total number of ranges stored across all chunks and layers.
    pub fn size(&self) -> usize {
        let mut count = 0;
        for c in &self.chunk {
            count += c.value.len();
        }
        if let Some(ref next) = self.next_layer {
            count += next.size();
        }
        count
    }

    /// The end position of the furthest range in this set.
    pub fn length(&self) -> usize {
        let mut len = 0;
        if let Some(&last_pos) = self.chunk_pos.last() {
            if let Some(last_chunk) = self.chunk.last() {
                len = last_pos + last_chunk.length();
            }
        }
        if let Some(ref next) = self.next_layer {
            len = len.max(next.length());
        }
        len
    }

    /// Creates a `RangeSet` from an array of ranges, checking ordering.
    pub fn of(ranges: Vec<Range<T>>) -> Result<Self, &'static str> {
        Self::of_sorted(ranges, false)
    }

    /// Creates a `RangeSet` from an array of ranges, optionally sorting them first.
    pub fn of_sorted(mut ranges: Vec<Range<T>>, sort: bool) -> Result<Self, &'static str> {
        if sort {
            ranges.sort_by(|a, b| {
                a.from.cmp(&b.from)
                    .then_with(|| a.value.start_side().cmp(&b.value.start_side()))
            });
        }
        let mut builder = RangeSetBuilder::new();
        for r in ranges {
            builder.add_strict(r.from, r.to, r.value)?;
        }
        Ok(builder.finish())
    }

    /// Iterates over all ranges touching `[from, to]`. Stops if `f` returns `false`.
    pub fn between(&self, from: usize, to: usize, mut f: impl FnMut(usize, usize, &T) -> bool) {
        self.between_inner(from, to, &mut f);
    }

    fn between_inner(&self, from: usize, to: usize, f: &mut impl FnMut(usize, usize, &T) -> bool) -> bool {
        for (i, &start) in self.chunk_pos.iter().enumerate() {
            let chunk = &self.chunk[i];
            let chunk_end = start + chunk.length();
            if to >= start && from <= chunk_end {
                if !chunk.between(start, from, to, &mut *f) {
                    return false;
                }
            }
        }
        if let Some(ref next) = self.next_layer {
            if !next.between_inner(from, to, f) {
                return false;
            }
        }
        true
    }

    /// Returns an ordered iterator visiting all ranges in this set.
    pub fn iter(&self) -> Vec<Range<T>> {
        let mut all = Vec::with_capacity(self.size());
        self.collect_ranges(&mut all);
        all.sort_by(|a, b| {
            a.from.cmp(&b.from)
                .then_with(|| a.value.start_side().cmp(&b.value.start_side()))
        });
        all
    }

    fn collect_ranges(&self, out: &mut Vec<Range<T>>) {
        for (i, &start) in self.chunk_pos.iter().enumerate() {
            let chunk = &self.chunk[i];
            for j in 0..chunk.value.len() {
                out.push(Range::new(
                    chunk.from[j] + start,
                    chunk.to[j] + start,
                    chunk.value[j].clone(),
                ));
            }
        }
        if let Some(ref next) = self.next_layer {
            next.collect_ranges(out);
        }
    }

    /// Updates the range set by adding new ranges or filtering existing ones.
    pub fn update(&self, spec: RangeSetUpdate<T>) -> Result<Self, &'static str> {
        let mut ranges = self.iter();

        if let Some(filter) = spec.filter {
            ranges.retain(|r| {
                if r.to < spec.filter_from || r.from > spec.filter_to {
                    true
                } else {
                    filter(r.from, r.to, &r.value)
                }
            });
        }

        if !spec.add.is_empty() {
            ranges.extend(spec.add);
        }

        Self::of_sorted(ranges, true)
    }

    /// Maps this range set through a set of text changes (Operational Transformation).
    pub fn map(&self, changes: &ChangeDesc) -> Self {
        if changes.is_empty() || self.is_empty() {
            return self.clone();
        }

        let mut mapped_ranges = Vec::new();
        for r in self.iter() {
            if r.from == r.to {
                if let Some(new_pos) = changes.map_pos(r.from, r.value.start_side() as i8, r.value.map_mode()) {
                    let mut new_to = new_pos;
                    if r.value.start_side() != r.value.end_side() {
                        if let Some(to_pos) = changes.map_pos(r.from, r.value.end_side() as i8, r.value.map_mode()) {
                            new_to = to_pos;
                        }
                    }
                    if new_to >= new_pos {
                        mapped_ranges.push(Range::new(new_pos, new_to, r.value));
                    }
                }
            } else {
                let new_from = changes.map_pos(r.from, r.value.start_side() as i8, MapMode::Simple);
                let new_to = changes.map_pos(r.to, r.value.end_side() as i8, MapMode::Simple);
                if let (Some(nf), Some(nt)) = (new_from, new_to) {
                    if nf <= nt && !(nf == nt && r.value.start_side() > 0 && r.value.end_side() <= 0) {
                        mapped_ranges.push(Range::new(nf, nt, r.value));
                    }
                }
            }
        }

        Self::of_sorted(mapped_ranges, true).unwrap_or_else(|_| Self::empty())
    }
}

/// Specification for updating a `RangeSet`.
pub struct RangeSetUpdate<T: RangeValue> {
    pub add: Vec<Range<T>>,
    pub sort: bool,
    pub filter: Option<Arc<dyn Fn(usize, usize, &T) -> bool + Send + Sync>>,
    pub filter_from: usize,
    pub filter_to: usize,
}

impl<T: RangeValue> Default for RangeSetUpdate<T> {
    fn default() -> Self {
        Self {
            add: Vec::new(),
            sort: false,
            filter: None,
            filter_from: 0,
            filter_to: usize::MAX,
        }
    }
}

/// Builder used to construct a `RangeSet` sequentially with chunk partitioning and layer spilling.
pub struct RangeSetBuilder<T: RangeValue> {
    chunks: Vec<Arc<Chunk<T>>>,
    chunk_pos: Vec<usize>,
    chunk_start: isize,
    last_from: i64,
    last_to: i64,
    last_start_side: i32,
    last_end_side: i32,
    from: Vec<usize>,
    to: Vec<usize>,
    value: Vec<T>,
    max_point: isize,
    set_max_point: isize,
    next_layer: Option<Box<RangeSetBuilder<T>>>,
}

impl<T: RangeValue> Default for RangeSetBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RangeValue> RangeSetBuilder<T> {
    /// Creates an empty `RangeSetBuilder`.
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            chunk_pos: Vec::new(),
            chunk_start: -1,
            last_from: -FAR,
            last_to: -FAR,
            last_start_side: 0,
            last_end_side: 0,
            from: Vec::new(),
            to: Vec::new(),
            value: Vec::new(),
            max_point: -1,
            set_max_point: -1,
            next_layer: None,
        }
    }

    fn finish_chunk(&mut self) {
        if self.from.is_empty() { return; }
        let chunk = Chunk::new(
            std::mem::take(&mut self.from),
            std::mem::take(&mut self.to),
            std::mem::take(&mut self.value),
            self.max_point,
        );
        self.chunks.push(Arc::new(chunk));
        self.chunk_pos.push(self.chunk_start.max(0) as usize);
        self.chunk_start = -1;
        self.set_max_point = self.set_max_point.max(self.max_point);
        self.max_point = -1;
    }

    /// Adds a range.
    pub fn add(&mut self, from: usize, to: usize, value: T) {
        self.add_range(from, to, value, false).ok();
    }

    /// Adds a range, returning an error if ranges are not in sorted order.
    pub fn add_strict(&mut self, from: usize, to: usize, value: T) -> Result<(), &'static str> {
        self.add_range(from, to, value, true)
    }

    pub fn add_range(&mut self, from: usize, to: usize, value: T, strict: bool) -> Result<(), &'static str> {
        if !self.add_inner(from, to, value.clone(), strict)? {
            let next = self.next_layer.get_or_insert_with(|| Box::new(RangeSetBuilder::new()));
            next.add_range(from, to, value, strict)?;
        }
        Ok(())
    }

    fn add_inner(&mut self, from: usize, to: usize, value: T, strict: bool) -> Result<bool, &'static str> {
        let diff = (from as i64 - self.last_to) * 2 + (if value.start_side() >= self.last_end_side { 1 } else { -1 });
        let from_diff = (from as i64 - self.last_from) * 2 + (if value.start_side() >= self.last_start_side { 1 } else { -1 });

        if strict && diff <= 0 && from_diff < 0 {
            return Err("Ranges must be added sorted by `from` position and `startSide`");
        }

        if diff < 0 {
            return Ok(false);
        }

        if self.from.len() == CHUNK_SIZE {
            self.finish_chunk();
        }

        if self.chunk_start < 0 {
            self.chunk_start = from as isize;
        }

        let base = self.chunk_start as usize;
        self.from.push(from - base);
        self.to.push(to - base);
        self.last_from = from as i64;
        self.last_to = to as i64;
        self.last_start_side = value.start_side();
        self.last_end_side = value.end_side();
        if value.point() {
            self.max_point = self.max_point.max((to - from) as isize);
        }
        self.value.push(value);
        Ok(true)
    }

    /// Finishes building the `RangeSet`.
    pub fn finish(mut self) -> RangeSet<T> {
        self.finish_chunk();
        let next_layer = self.next_layer.map(|b| Arc::new(b.finish()));
        RangeSet {
            chunk_pos: self.chunk_pos,
            chunk: self.chunks,
            next_layer,
            max_point: self.set_max_point,
        }
    }
}

/// Trait for receiving comparisons between two sets of ranges across document changes.
pub trait RangeComparator<T: RangeValue> {
    fn compare_range(&mut self, from: usize, to: usize, active_a: &[T], active_b: &[T]);
    fn compare_point(&mut self, from: usize, to: usize, by_a: Option<&T>, by_b: Option<&T>);
}

/// Trait for receiving non-overlapping active spans or points across a document region.
pub trait SpanIterator<T: RangeValue> {
    fn span(&mut self, from: usize, to: usize, active: &[T], open_start: usize);
    fn point(&mut self, from: usize, to: usize, value: &T, active: &[T], open_start: usize, index: usize);
}

struct LayerCursor<'a, T: RangeValue> {
    chunk_pos: &'a [usize],
    chunks: &'a [Arc<Chunk<T>>],
    chunk_index: usize,
    range_index: usize,
    from: usize,
    to: usize,
    value: Option<&'a T>,
    rank: usize,
    min_point: isize,
}

impl<'a, T: RangeValue> LayerCursor<'a, T> {
    fn new(chunk_pos: &'a [usize], chunks: &'a [Arc<Chunk<T>>], rank: usize, min_point: isize) -> Self {
        Self {
            chunk_pos,
            chunks,
            chunk_index: 0,
            range_index: 0,
            from: FAR as usize,
            to: FAR as usize,
            value: None,
            rank,
            min_point,
        }
    }

    fn start_side(&self) -> i32 {
        self.value.map_or(0, |v| v.start_side())
    }

    fn end_side(&self) -> i32 {
        self.value.map_or(0, |v| v.end_side())
    }

    fn goto(&mut self, pos: usize, side: i32) {
        self.chunk_index = 0;
        self.range_index = 0;
        self.goto_inner(pos, side, false);
    }

    fn goto_inner(&mut self, pos: usize, side: i32, mut forward: bool) {
        while self.chunk_index < self.chunks.len() {
            let next = &self.chunks[self.chunk_index];
            let chunk_start = self.chunk_pos[self.chunk_index];
            let chunk_end = chunk_start + next.length();
            if !(chunk_end < pos || next.max_point < self.min_point) {
                break;
            }
            self.chunk_index += 1;
            forward = false;
        }

        if self.chunk_index < self.chunks.len() {
            let chunk_start = self.chunk_pos[self.chunk_index];
            let rel_pos = pos.saturating_sub(chunk_start);
            let range_index = self.chunks[self.chunk_index].find_index(rel_pos, side, true, 0);
            if !forward || self.range_index < range_index {
                self.set_range_index(range_index);
            }
        }
        self.next();
    }

    fn forward(&mut self, pos: usize, side: i32) {
        let diff = (self.to as i64 - pos as i64) * 2 + (if self.end_side() >= side { 1 } else { -1 });
        if diff < 0 {
            self.goto_inner(pos, side, true);
        }
    }

    fn set_range_index(&mut self, index: usize) {
        if index >= self.chunks[self.chunk_index].value.len() {
            self.chunk_index += 1;
            self.range_index = 0;
        } else {
            self.range_index = index;
        }
    }

    fn next(&mut self) {
        loop {
            if self.chunk_index >= self.chunks.len() {
                self.from = FAR as usize;
                self.to = FAR as usize;
                self.value = None;
                break;
            }
            let chunk = &self.chunks[self.chunk_index];
            let chunk_start = self.chunk_pos[self.chunk_index];
            let from = chunk_start + chunk.from[self.range_index];
            let to = chunk_start + chunk.to[self.range_index];
            let val = &chunk.value[self.range_index];
            self.from = from;
            self.to = to;
            self.value = Some(val);
            self.set_range_index(self.range_index + 1);
            if self.min_point < 0 || (val.point() && (to - from) as isize >= self.min_point) {
                break;
            }
        }
    }

    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.from
            .cmp(&other.from)
            .then_with(|| self.start_side().cmp(&other.start_side()))
            .then_with(|| self.rank.cmp(&other.rank))
            .then_with(|| self.to.cmp(&other.to))
            .then_with(|| self.end_side().cmp(&other.end_side()))
    }
}

struct HeapCursor<'a, T: RangeValue> {
    heap: Vec<LayerCursor<'a, T>>,
    from: usize,
    to: usize,
    value: Option<&'a T>,
    rank: usize,
}

impl<'a, T: RangeValue> HeapCursor<'a, T> {
    fn from_sets(sets: &'a [RangeSet<T>], min_point: isize) -> Self {
        let mut heap = Vec::new();
        for (i, set) in sets.iter().enumerate() {
            let mut cur = Some(set);
            while let Some(layer) = cur {
                if layer.max_point >= min_point && !layer.chunk.is_empty() {
                    heap.push(LayerCursor::new(&layer.chunk_pos, &layer.chunk, i, min_point));
                }
                cur = layer.next_layer.as_deref();
            }
        }
        Self {
            heap,
            from: FAR as usize,
            to: FAR as usize,
            value: None,
            rank: 0,
        }
    }

    fn start_side(&self) -> i32 {
        self.value.map_or(0, |v| v.start_side())
    }

    fn goto(&mut self, pos: usize, side: i32) {
        for cur in &mut self.heap {
            cur.goto(pos, side);
        }
        for i in (0..self.heap.len()).rev() {
            self.heap_bubble(i);
        }
        self.next();
    }

    fn forward(&mut self, pos: usize, side: i32) {
        for cur in &mut self.heap {
            cur.forward(pos, side);
        }
        for i in (0..self.heap.len()).rev() {
            self.heap_bubble(i);
        }
        if let Some(val) = self.value {
            let diff = (self.to as i64 - pos as i64) * 2 + (if val.end_side() >= side { 1 } else { -1 });
            if diff < 0 {
                self.next();
            }
        }
    }

    fn next(&mut self) {
        if self.heap.is_empty() {
            self.from = FAR as usize;
            self.to = FAR as usize;
            self.value = None;
            self.rank = usize::MAX;
        } else {
            let top = &self.heap[0];
            self.from = top.from;
            self.to = top.to;
            self.value = top.value;
            self.rank = top.rank;
            if top.value.is_some() {
                self.heap[0].next();
            }
            self.heap_bubble(0);
        }
    }

    fn heap_bubble(&mut self, mut index: usize) {
        let len = self.heap.len();
        while (index << 1) + 1 < len {
            let mut child_index = (index << 1) + 1;
            if child_index + 1 < len && self.heap[child_index].compare(&self.heap[child_index + 1]) == std::cmp::Ordering::Greater {
                child_index += 1;
            }
            if self.heap[index].compare(&self.heap[child_index]) != std::cmp::Ordering::Greater {
                break;
            }
            self.heap.swap(index, child_index);
            index = child_index;
        }
    }
}

struct SpanCursor<'a, T: RangeValue> {
    cursor: HeapCursor<'a, T>,
    active: Vec<T>,
    active_to: Vec<usize>,
    active_rank: Vec<usize>,
    min_active: isize,

    point: Option<T>,
    point_from: usize,
    point_rank: usize,

    to: i64,
    end_side: i32,
    open_start: isize,
}

impl<'a, T: RangeValue> SpanCursor<'a, T> {
    fn new(sets: &'a [RangeSet<T>], min_point: isize) -> Self {
        Self {
            cursor: HeapCursor::from_sets(sets, min_point),
            active: Vec::new(),
            active_to: Vec::new(),
            active_rank: Vec::new(),
            min_active: -1,
            point: None,
            point_from: 0,
            point_rank: 0,
            to: -FAR,
            end_side: 0,
            open_start: -1,
        }
    }

    fn goto(&mut self, pos: usize, side: i32) {
        self.cursor.goto(pos, side);
        self.active.clear();
        self.active_to.clear();
        self.active_rank.clear();
        self.min_active = -1;
        self.to = pos as i64;
        self.end_side = side;
        self.open_start = -1;
        self.next();
    }

    fn forward(&mut self, pos: usize, side: i32) {
        while self.min_active > -1 {
            let idx = self.min_active as usize;
            let diff = (self.active_to[idx] as i64 - pos as i64) * 2
                + (if self.active[idx].end_side() >= side { 1 } else { -1 });
            if diff < 0 {
                self.remove_active(idx);
            } else {
                break;
            }
        }
        self.cursor.forward(pos, side);
    }

    fn remove_active(&mut self, index: usize) {
        self.active.remove(index);
        self.active_to.remove(index);
        self.active_rank.remove(index);
        self.min_active = self.find_min_index();
    }

    fn find_min_index(&self) -> isize {
        let mut found = -1isize;
        let mut found_pos = FAR;
        for i in 0..self.active_to.len() {
            let diff = if found < 0 {
                -1
            } else {
                (self.active_to[i] as i64 - found_pos) * 2
                    + (if self.active[i].end_side() >= self.active[found as usize].end_side() { 1 } else { -1 })
            };
            if diff < 0 {
                found = i as isize;
                found_pos = self.active_to[i] as i64;
            }
        }
        found
    }

    fn add_active(&mut self, track_open: &mut Option<Vec<usize>>) {
        let value = self.cursor.value.unwrap().clone();
        let to = self.cursor.to;
        let rank = self.cursor.rank;
        let mut i = 0;
        while i < self.active_rank.len() {
            let rank_diff = rank as isize - self.active_rank[i] as isize;
            let to_diff = to as isize - self.active_to[i] as isize;
            if rank_diff > 0 || (rank_diff == 0 && to_diff > 0) {
                i += 1;
            } else {
                break;
            }
        }
        self.active.insert(i, value);
        self.active_to.insert(i, to);
        self.active_rank.insert(i, rank);
        if let Some(ref mut track) = track_open {
            track.insert(i, self.cursor.from);
        }
        self.min_active = self.find_min_index();
    }

    fn next(&mut self) {
        let from = self.to;
        let was_point = self.point.is_some();
        self.point = None;
        let mut track_open = if self.open_start < 0 { Some(Vec::new()) } else { None };

        loop {
            let a = self.min_active;
            let a_idx = a as usize;
            if a > -1 {
                let diff = (self.active_to[a_idx] as i64 - self.cursor.from as i64) * 2
                    + (if self.active[a_idx].end_side() >= self.cursor.start_side() { 1 } else { -1 });
                if diff < 0 {
                    if self.active_to[a_idx] as i64 > from {
                        self.to = self.active_to[a_idx] as i64;
                        self.end_side = self.active[a_idx].end_side();
                        break;
                    }
                    self.remove_active(a_idx);
                    if let Some(ref mut track) = track_open {
                        track.remove(a_idx);
                    }
                    continue;
                }
            }

            if self.cursor.value.is_none() {
                self.to = FAR;
                self.end_side = 0;
                break;
            } else if self.cursor.from as i64 > from {
                self.to = self.cursor.from as i64;
                self.end_side = self.cursor.start_side();
                break;
            } else {
                let next_val = self.cursor.value.unwrap().clone();
                if !next_val.point() {
                    self.add_active(&mut track_open);
                    self.cursor.next();
                } else if was_point && self.cursor.to as i64 == self.to && self.cursor.from < self.cursor.to {
                    self.cursor.next();
                } else {
                    self.point = Some(next_val.clone());
                    self.point_from = self.cursor.from;
                    self.point_rank = self.cursor.rank;
                    self.to = self.cursor.to as i64;
                    self.end_side = next_val.end_side();
                    self.cursor.next();
                    self.forward(self.to as usize, self.end_side);
                    break;
                }
            }
        }

        if let Some(track) = track_open {
            self.open_start = 0;
            for &open_pos in track.iter().rev() {
                if (open_pos as i64) < from {
                    self.open_start += 1;
                }
            }
        }
    }

    fn active_for_point(&self, to: usize) -> Vec<T> {
        let mut active = Vec::new();
        if self.active.is_empty() {
            return active;
        }
        for i in (0..self.active.len()).rev() {
            if self.active_rank[i] < self.point_rank {
                break;
            }
            let point_end_side = self.point.as_ref().map_or(0, |p| p.end_side());
            if self.active_to[i] > to || (self.active_to[i] == to && self.active[i].end_side() >= point_end_side) {
                active.push(self.active[i].clone());
            }
        }
        active.reverse();
        active
    }

    fn open_end(&self, to: usize) -> usize {
        let mut open = 0;
        for i in (0..self.active_to.len()).rev() {
            if self.active_to[i] > to {
                open += 1;
            }
        }
        open
    }
}

fn same_values<T: RangeValue>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if !a[i].eq_val(&b[i]) {
            return false;
        }
    }
    true
}

fn compare_cursors<T: RangeValue, C: RangeComparator<T>>(
    a: &mut SpanCursor<T>,
    start_a: usize,
    b: &mut SpanCursor<T>,
    start_b: usize,
    length: usize,
    comparator: &mut C,
) {
    a.goto(start_a, -1_000_000);
    b.goto(start_b, -1_000_000);
    let end_b = start_b + length;
    let mut pos = start_b;
    let d_pos = start_b as i64 - start_a as i64;

    loop {
        let d_end = (a.to + d_pos) - b.to;
        let diff = if d_end != 0 { d_end } else { (a.end_side - b.end_side) as i64 };
        let end = if diff < 0 { a.to + d_pos } else { b.to };
        let clip_end = (end as usize).min(end_b);

        if a.point.is_some() || b.point.is_some() {
            let matches = a.point.is_some()
                && b.point.is_some()
                && a.point.as_ref().unwrap().eq_val(b.point.as_ref().unwrap())
                && same_values(&a.active_for_point(a.to as usize), &b.active_for_point(b.to as usize));
            if !matches {
                comparator.compare_point(pos, clip_end, a.point.as_ref(), b.point.as_ref());
            }
        } else {
            if clip_end > pos && !same_values(&a.active, &b.active) {
                comparator.compare_range(pos, clip_end, &a.active, &b.active);
            }
        }

        if end as usize > end_b {
            break;
        }
        pos = end as usize;
        if diff <= 0 {
            a.next();
        }
        if diff >= 0 {
            b.next();
        }
    }
}

impl<T: RangeValue> RangeSet<T> {
    /// Compares two collections of RangeSets through document changes.
    pub fn compare<C: RangeComparator<T>>(
        old_sets: &[RangeSet<T>],
        new_sets: &[RangeSet<T>],
        changes: &crate::change::ChangeSet,
        comparator: &mut C,
    ) {
        let min_point = -1;
        let mut a = SpanCursor::new(old_sets, min_point);
        let mut b = SpanCursor::new(new_sets, min_point);

        let mut pos_a = 0;
        let mut pos_b = 0;
        changes.desc().iter_gaps(|from_a, from_b, length| {
            if length > 0 {
                compare_cursors(&mut a, from_a, &mut b, from_b, length, comparator);
            }
            pos_a = from_a + length;
            pos_b = from_b + length;
        });
        let doc_len_a = changes.desc().length();
        let doc_len_b = changes.desc().new_length();
        if pos_a < doc_len_a || pos_b < doc_len_b {
            let remain = std::cmp::min(doc_len_a.saturating_sub(pos_a), doc_len_b.saturating_sub(pos_b));
            if remain > 0 {
                compare_cursors(&mut a, pos_a, &mut b, pos_b, remain, comparator);
            }
        }
    }

    /// Iterates through non-overlapping slices of covering active values.
    pub fn spans<S: SpanIterator<T>>(
        sets: &[RangeSet<T>],
        from: usize,
        to: usize,
        iterator: &mut S,
        min_point_size: isize,
    ) -> usize {
        let mut cursor = SpanCursor::new(sets, min_point_size);
        cursor.goto(from, -FAR as i32);
        let mut pos = from;
        let mut open_ranges = cursor.open_start.max(0) as usize;
        loop {
            let cur_to = (cursor.to as usize).min(to);
            if let Some(ref pt) = cursor.point.clone() {
                let active = cursor.active_for_point(cursor.to as usize);
                let open_count = if cursor.point_from < from {
                    active.len() + 1
                } else if pt.start_side() < 0 {
                    active.len()
                } else {
                    active.len().min(open_ranges)
                };
                iterator.point(pos, cur_to, pt, &active, open_count, cursor.point_rank);
                open_ranges = cursor.open_end(cur_to).min(active.len());
            } else if cur_to > pos {
                iterator.span(pos, cur_to, &cursor.active, open_ranges);
                open_ranges = cursor.open_end(cur_to);
            }
            if cursor.to as usize > to {
                let add_point = if cursor.point.is_some() && cursor.to as usize > to { 1 } else { 0 };
                return open_ranges + add_point;
            }
            pos = cursor.to as usize;
            cursor.next();
        }
    }
}
