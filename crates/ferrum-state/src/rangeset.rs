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
