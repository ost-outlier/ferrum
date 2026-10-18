use crate::text::Text;
use std::fmt;

/// Distinguishes different ways in which positions can be mapped across deletions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    /// Map a position to a valid new position, even when its context was deleted.
    Simple,
    /// Return None if deletion happens across the position.
    TrackDel,
    /// Return None if the character before the position is deleted.
    TrackBefore,
    /// Return None if the character after the position is deleted.
    TrackAfter,
}

/// A change description is a lightweight variant of `ChangeSet` that doesn't store inserted text.
/// It encodes changes as a sequence of integer pairs:
/// `(len, ins)` where `ins == -1` indicates an unchanged gap of length `len`,
/// and `ins >= 0` indicates that `len` characters are replaced by `ins` characters.
#[derive(Clone, PartialEq, Eq)]
pub struct ChangeDesc {
    pub(crate) sections: Vec<i64>,
}

impl ChangeDesc {
    /// Creates a `ChangeDesc` from a raw sections vector.
    pub fn create(sections: Vec<i64>) -> Self {
        Self { sections }
    }

    /// Creates an empty change description for a document of the given length.
    pub fn empty(length: usize) -> Self {
        if length > 0 {
            Self {
                sections: vec![length as i64, -1],
            }
        } else {
            Self {
                sections: Vec::new(),
            }
        }
    }

    /// The length of the document before these changes.
    pub fn length(&self) -> usize {
        let mut result = 0;
        let mut i = 0;
        while i < self.sections.len() {
            result += self.sections[i] as usize;
            i += 2;
        }
        result
    }

    /// The length of the document after these changes.
    pub fn new_length(&self) -> usize {
        let mut result = 0;
        let mut i = 0;
        while i < self.sections.len() {
            let ins = self.sections[i + 1];
            result += if ins < 0 {
                self.sections[i] as usize
            } else {
                ins as usize
            };
            i += 2;
        }
        result
    }

    /// Returns true if this description contains no actual modifications.
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty() || (self.sections.len() == 2 && self.sections[1] < 0)
    }

    /// Iterates over unchanged sections (gaps).
    /// `f(pos_a, pos_b, length)`
    pub fn iter_gaps<F: FnMut(usize, usize, usize)>(&self, mut f: F) {
        let mut i = 0;
        let mut pos_a = 0;
        let mut pos_b = 0;
        while i < self.sections.len() {
            let len = self.sections[i] as usize;
            let ins = self.sections[i + 1];
            i += 2;
            if ins < 0 {
                f(pos_a, pos_b, len);
                pos_b += len;
            } else {
                pos_b += ins as usize;
            }
            pos_a += len;
        }
    }

    /// Iterates over changed ranges in the document.
    /// `f(from_a, to_a, from_b, to_b)`
    pub fn iter_changed_ranges<F: FnMut(usize, usize, usize, usize)>(&self, mut f: F, individual: bool) {
        iter_changes(self, None, |from_a, to_a, from_b, to_b, _| {
            f(from_a, to_a, from_b, to_b);
        }, individual);
    }

    /// Produces a description of the inverted form of these changes.
    pub fn inverted_desc(&self) -> Self {
        let mut sections = Vec::with_capacity(self.sections.len());
        let mut i = 0;
        while i < self.sections.len() {
            let len = self.sections[i];
            let ins = self.sections[i + 1];
            i += 2;
            if ins < 0 {
                sections.push(len);
                sections.push(ins);
            } else {
                sections.push(ins);
                sections.push(len);
            }
        }
        Self { sections }
    }

    /// Computes the combined effect of applying `other` after `self`.
    /// `self.new_length()` must equal `other.length()`.
    pub fn compose_desc(&self, other: &Self) -> Self {
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            compose_impl(self, None, other, None).0
        }
    }

    /// Maps this description over another set of concurrent changes (Operational Transformation).
    /// `before` specifies tie-breaking precedence when both changes insert at the same location.
    pub fn map_desc(&self, other: &Self, before: bool) -> Self {
        if other.is_empty() {
            self.clone()
        } else {
            map_impl(self, None, other, before).0
        }
    }

    /// Maps a given character position through these changes.
    /// `assoc`: bias when landing inside an edit (-1 = before insertion, 1 = after insertion).
    /// `mode`: how deletions across `pos` should be reported.
    pub fn map_pos(&self, pos: usize, assoc: i8, mode: MapMode) -> Option<usize> {
        let mut pos_a = 0;
        let mut pos_b = 0;
        let mut i = 0;

        while i < self.sections.len() {
            let len = self.sections[i] as usize;
            let ins = self.sections[i + 1];
            i += 2;
            let end_a = pos_a + len;

            if ins < 0 {
                if end_a > pos {
                    return Some(pos_b + (pos - pos_a));
                }
                pos_b += len;
            } else {
                let ins_len = ins as usize;
                if mode != MapMode::Simple && end_a >= pos {
                    let is_del = match mode {
                        MapMode::TrackDel => pos_a < pos && end_a > pos,
                        MapMode::TrackBefore => pos_a < pos,
                        MapMode::TrackAfter => end_a > pos,
                        MapMode::Simple => false,
                    };
                    if is_del {
                        return None;
                    }
                }

                if end_a > pos || (end_a == pos && assoc < 0 && len == 0) {
                    return Some(if pos == pos_a || assoc < 0 {
                        pos_b
                    } else {
                        pos_b + ins_len
                    });
                }
                pos_b += ins_len;
            }
            pos_a = end_a;
        }

        if pos > pos_a {
            panic!("Position {} is out of range for changeset of length {}", pos, pos_a);
        }
        Some(pos_b)
    }

    /// Parses a string specification in the CodeMirror format: `"5 0:2 2:0 4"`.
    pub fn from_string(spec: &str) -> Self {
        let mut sections = Vec::new();
        for token in spec.split_whitespace() {
            if let Some((len_str, ins_str)) = token.split_once(':') {
                let len: i64 = len_str.parse().expect("Invalid length in spec");
                let ins: i64 = ins_str.parse().expect("Invalid ins in spec");
                sections.push(len);
                sections.push(ins);
            } else if !token.is_empty() {
                let len: i64 = token.parse().expect("Invalid length in spec");
                sections.push(len);
                sections.push(-1);
            }
        }
        Self { sections }
    }
}

impl fmt::Display for ChangeDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        let mut i = 0;
        while i < self.sections.len() {
            let len = self.sections[i];
            let ins = self.sections[i + 1];
            i += 2;
            if !first {
                write!(f, " ")?;
            }
            first = false;
            if ins < 0 {
                write!(f, "{}", len)?;
            } else {
                write!(f, "{}:{}", len, ins)?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for ChangeDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChangeDesc(\"{}\")", self)
    }
}

/// A specification for an atomic text change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSpec {
    pub from: usize,
    pub to: usize,
    pub insert: Text,
}

impl ChangeSpec {
    pub fn delete(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            insert: Text::empty(),
        }
    }

    pub fn insert(at: usize, text: impl Into<Text>) -> Self {
        Self {
            from: at,
            to: at,
            insert: text.into(),
        }
    }

    pub fn replace(from: usize, to: usize, text: impl Into<Text>) -> Self {
        Self {
            from,
            to,
            insert: text.into(),
        }
    }
}

/// Flexible container for changes passed to a transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Changes {
    /// A pre-computed change set.
    Set(ChangeSet),
    /// A single change specification.
    Single(ChangeSpec),
    /// Multiple change specifications.
    Multiple(Vec<ChangeSpec>),
}

impl Changes {
    /// Resolves this into a `ChangeSet` given the document length.
    pub fn to_changeset(self, doc_len: usize) -> ChangeSet {
        match self {
            Changes::Set(cs) => cs,
            Changes::Single(spec) => ChangeSet::of_spec(spec, doc_len),
            Changes::Multiple(specs) => ChangeSet::of_specs(&specs, doc_len),
        }
    }
}

impl From<ChangeSet> for Changes {
    fn from(cs: ChangeSet) -> Self {
        Changes::Set(cs)
    }
}

impl From<ChangeSpec> for Changes {
    fn from(spec: ChangeSpec) -> Self {
        Changes::Single(spec)
    }
}

impl From<Vec<ChangeSpec>> for Changes {
    fn from(specs: Vec<ChangeSpec>) -> Self {
        Changes::Multiple(specs)
    }
}

/// A full set of changes, containing both the change description and the inserted text.
#[derive(Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub(crate) desc: ChangeDesc,
    pub(crate) inserted: Vec<Text>,
}

impl ChangeSet {
    /// Creates a `ChangeSet` from sections and inserted texts.
    pub fn create_set(sections: Vec<i64>, inserted: Vec<Text>) -> Self {
        Self {
            desc: ChangeDesc::create(sections),
            inserted,
        }
    }

    /// Creates an empty `ChangeSet` for a document of length `length`.
    pub fn empty(length: usize) -> Self {
        Self {
            desc: ChangeDesc::empty(length),
            inserted: Vec::new(),
        }
    }

    /// Document length before applying these changes.
    pub fn length(&self) -> usize {
        self.desc.length()
    }

    /// Document length after applying these changes.
    pub fn new_length(&self) -> usize {
        self.desc.new_length()
    }

    /// True if there are no modifications in this set.
    pub fn is_empty(&self) -> bool {
        self.desc.is_empty()
    }

    /// The change description corresponding to this set.
    pub fn desc(&self) -> &ChangeDesc {
        &self.desc
    }

    /// Maps a character position through these changes.
    pub fn map_pos(&self, pos: usize, assoc: i8) -> usize {
        self.desc.map_pos(pos, assoc, MapMode::Simple).unwrap_or(pos)
    }

    /// Iterates over all changes with inserted text.
    pub fn iter_changes<F: FnMut(usize, usize, usize, usize, &Text)>(&self, f: F, individual: bool) {
        iter_changes(&self.desc, Some(&self.inserted), f, individual);
    }

    /// Iterates over unchanged sections (gaps) in the document.
    /// `f(pos_a, pos_b, length)`
    pub fn iter_gaps<F: FnMut(usize, usize, usize)>(&self, f: F) {
        self.desc.iter_gaps(f);
    }

    /// Applies this change set to a `Text` document in a single linear pass.
    pub fn apply(&self, doc: &Text) -> Text {
        if self.length() != doc.len_chars() {
            panic!(
                "Mismatched document length: changeset expected {}, doc is {}",
                self.length(),
                doc.len_chars()
            );
        }
        if self.is_empty() {
            return doc.clone();
        }

        let mut result = Text::empty();
        let mut last_pos = 0;

        self.iter_changes(|from_a, to_a, _, _, inserted| {
            if from_a > last_pos {
                result = result.append(&doc.slice(last_pos..from_a));
            }
            if !inserted.is_empty() {
                result = result.append(inserted);
            }
            last_pos = to_a;
        }, false);

        if last_pos < doc.len_chars() {
            result = result.append(&doc.slice(last_pos..doc.len_chars()));
        }

        result
    }

    /// Returns the inverted change set that rolls back these changes on `doc`.
    pub fn invert(&self, doc: &Text) -> Self {
        let mut sections = self.desc.sections.clone();
        let mut inserted = Vec::new();
        let mut pos = 0;
        let mut i = 0;

        while i < sections.len() {
            let len = sections[i] as usize;
            let ins = sections[i + 1];
            if ins >= 0 {
                sections[i] = ins;
                sections[i + 1] = len as i64;
                let index = i >> 1;
                while inserted.len() < index {
                    inserted.push(Text::empty());
                }
                let old_slice = if len > 0 {
                    doc.slice(pos..pos + len)
                } else {
                    Text::empty()
                };
                inserted.push(old_slice);
            }
            pos += len;
            i += 2;
        }

        Self {
            desc: ChangeDesc::create(sections),
            inserted,
        }
    }

    /// Combines this change set with `other` sequentially (Operational Transformation compose).
    pub fn compose(&self, other: &Self) -> Self {
        if self.is_empty() {
            other.clone()
        } else if other.is_empty() {
            self.clone()
        } else {
            let (desc, inserted) = compose_impl(
                &self.desc,
                Some(&self.inserted),
                &other.desc,
                Some(&other.inserted),
            );
            Self {
                desc,
                inserted: inserted.unwrap(),
            }
        }
    }

    /// Maps this change set over another concurrent change set (Operational Transformation map).
    pub fn map(&self, other: &ChangeDesc, before: bool) -> Self {
        if other.is_empty() {
            self.clone()
        } else {
            let (desc, inserted) = map_impl(&self.desc, Some(&self.inserted), other, before);
            Self {
                desc,
                inserted: inserted.unwrap(),
            }
        }
    }

    /// Creates a `ChangeSet` from a single `ChangeSpec` for a document of length `length`.
    pub fn of_spec(spec: ChangeSpec, length: usize) -> Self {
        Self::of_specs(&[spec], length)
    }

    /// Creates a `ChangeSet` from multiple `ChangeSpec`s for a document of length `length`.
    pub fn of_specs(specs: &[ChangeSpec], length: usize) -> Self {
        let mut sections: Vec<i64> = Vec::new();
        let mut inserted: Vec<Text> = Vec::new();
        let mut pos = 0;
        let mut total: Option<ChangeSet> = None;

        let flush = |sections: &mut Vec<i64>,
                     inserted: &mut Vec<Text>,
                     pos: &mut usize,
                     total: &mut Option<ChangeSet>,
                     force: bool| {
            if !force && sections.is_empty() {
                return;
            }
            if *pos < length {
                add_section(sections, length - *pos, -1, false);
            }
            let set = ChangeSet::create_set(std::mem::take(sections), std::mem::take(inserted));
            *total = match total.take() {
                Some(prev) => {
                    let mapped = set.map(prev.desc(), false);
                    Some(prev.compose(&mapped))
                }
                None => Some(set),
            };
            *pos = 0;
        };

        for spec in specs {
            if spec.from > length || spec.to > length || spec.from > spec.to {
                panic!(
                    "Invalid change range {}..{} for document of length {}",
                    spec.from, spec.to, length
                );
            }
            let ins_len = spec.insert.len_chars();
            if spec.from == spec.to && ins_len == 0 {
                continue;
            }
            if spec.from < pos {
                flush(&mut sections, &mut inserted, &mut pos, &mut total, false);
            }
            if spec.from > pos {
                add_section(&mut sections, spec.from - pos, -1, false);
            }
            add_section(&mut sections, spec.to - spec.from, ins_len as i64, false);
            add_insert(&mut inserted, &sections, spec.insert.clone());
            pos = spec.to;
        }

        let needs_force = total.is_none();
        flush(&mut sections, &mut inserted, &mut pos, &mut total, needs_force);

        total.unwrap_or_else(|| ChangeSet::empty(length))
    }
}

impl fmt::Display for ChangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl fmt::Debug for ChangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChangeSet(\"{}\")", self.desc)
    }
}

fn add_section(sections: &mut Vec<i64>, len: usize, ins: i64, force_join: bool) {
    if len == 0 && ins <= 0 {
        return;
    }
    if sections.len() >= 2 {
        let last_idx = sections.len() - 2;
        let last_len = sections[last_idx];
        let last_ins = sections[last_idx + 1];

        if ins <= 0 && ins == last_ins {
            sections[last_idx] += len as i64;
            return;
        } else if len == 0 && last_len == 0 {
            sections[last_idx + 1] += ins;
            return;
        } else if force_join {
            sections[last_idx] += len as i64;
            sections[last_idx + 1] += ins;
            return;
        }
    }
    sections.push(len as i64);
    sections.push(ins);
}

fn add_insert(values: &mut Vec<Text>, sections: &[i64], value: Text) {
    if value.is_empty() {
        return;
    }
    let index = (sections.len() - 2) >> 1;
    if index < values.len() {
        let last_idx = values.len() - 1;
        let combined = values[last_idx].append(&value);
        values[last_idx] = combined;
    } else {
        while values.len() < index {
            values.push(Text::empty());
        }
        values.push(value);
    }
}

fn iter_changes<F: FnMut(usize, usize, usize, usize, &Text)>(
    desc: &ChangeDesc,
    inserted: Option<&[Text]>,
    mut f: F,
    individual: bool,
) {
    let mut pos_a = 0;
    let mut pos_b = 0;
    let mut i = 0;

    while i < desc.sections.len() {
        let mut len = desc.sections[i] as usize;
        let mut ins = desc.sections[i + 1];
        i += 2;

        if ins < 0 {
            pos_a += len;
            pos_b += len;
        } else {
            let mut end_a = pos_a;
            let mut end_b = pos_b;
            let mut text = Text::empty();

            loop {
                end_a += len;
                end_b += ins as usize;
                if ins > 0 && inserted.is_some() {
                    let ins_idx = (i - 2) >> 1;
                    if let Some(ins_list) = inserted {
                        if ins_idx < ins_list.len() {
                            text = text.append(&ins_list[ins_idx]);
                        }
                    }
                }
                if individual || i == desc.sections.len() || desc.sections[i + 1] < 0 {
                    break;
                }
                len = desc.sections[i] as usize;
                ins = desc.sections[i + 1];
                i += 2;
            }

            f(pos_a, end_a, pos_b, end_b, &text);
            pos_a = end_a;
            pos_b = end_b;
        }
    }
}

struct SectionIter<'a> {
    sections: &'a [i64],
    inserted: Option<&'a [Text]>,
    i: usize,
    len: usize,
    off: usize,
    ins: i64,
}

impl<'a> SectionIter<'a> {
    fn new(desc: &'a ChangeDesc, inserted: Option<&'a [Text]>) -> Self {
        let mut iter = Self {
            sections: &desc.sections,
            inserted,
            i: 0,
            len: 0,
            off: 0,
            ins: -2,
        };
        iter.next();
        iter
    }

    fn next(&mut self) {
        if self.i < self.sections.len() {
            self.len = self.sections[self.i] as usize;
            self.ins = self.sections[self.i + 1];
            self.i += 2;
        } else {
            self.len = 0;
            self.ins = -2;
        }
        self.off = 0;
    }

    fn done(&self) -> bool {
        self.ins == -2
    }

    fn len2(&self) -> usize {
        if self.ins < 0 {
            self.len
        } else {
            self.ins as usize
        }
    }

    fn text(&self) -> Text {
        if let Some(inserted) = self.inserted {
            let index = (self.i - 2) >> 1;
            if index < inserted.len() {
                return inserted[index].clone();
            }
        }
        Text::empty()
    }

    fn text_bit(&self, len: Option<usize>) -> Text {
        if let Some(inserted) = self.inserted {
            let index = (self.i - 2) >> 1;
            if index < inserted.len() {
                let full = &inserted[index];
                let end = match len {
                    Some(l) => (self.off + l).min(full.len_chars()),
                    None => full.len_chars(),
                };
                return full.slice(self.off..end);
            }
        }
        Text::empty()
    }

    fn forward(&mut self, len: usize) {
        if len == self.len {
            self.next();
        } else {
            self.len -= len;
            self.off += len;
        }
    }

    fn forward2(&mut self, len: usize) {
        if self.ins == -1 {
            self.forward(len);
        } else if len == self.ins as usize {
            self.next();
        } else {
            self.ins -= len as i64;
            self.off += len;
        }
    }
}

fn compose_impl(
    set_a: &ChangeDesc,
    insert_a: Option<&[Text]>,
    set_b: &ChangeDesc,
    insert_b: Option<&[Text]>,
) -> (ChangeDesc, Option<Vec<Text>>) {
    let mut sections = Vec::new();
    let insert = insert_a.is_some() || insert_b.is_some();
    let mut inserted = if insert { Some(Vec::new()) } else { None };

    let mut a = SectionIter::new(set_a, insert_a);
    let mut b = SectionIter::new(set_b, insert_b);
    let mut open = false;

    loop {
        if a.done() && b.done() {
            return (ChangeDesc::create(sections), inserted);
        } else if a.ins == 0 {
            // Deletion in A
            add_section(&mut sections, a.len, 0, open);
            a.next();
        } else if b.len == 0 && !b.done() {
            // Insertion in B
            add_section(&mut sections, 0, b.ins, open);
            if let Some(ref mut ins_list) = inserted {
                add_insert(ins_list, &sections, b.text());
            }
            b.next();
        } else if a.done() || b.done() {
            panic!("Mismatched change set lengths in compose");
        } else {
            let len = a.len2().min(b.len);
            let section_len = sections.len();

            if a.ins == -1 {
                let ins_b = if b.ins == -1 {
                    -1
                } else if b.off > 0 {
                    0
                } else {
                    b.ins
                };
                add_section(&mut sections, len, ins_b, open);
                if let Some(ref mut ins_list) = inserted {
                    if ins_b > 0 {
                        add_insert(ins_list, &sections, b.text());
                    }
                }
            } else if b.ins == -1 {
                let a_len = if a.off > 0 { 0 } else { a.len };
                add_section(&mut sections, a_len, len as i64, open);
                if let Some(ref mut ins_list) = inserted {
                    add_insert(ins_list, &sections, a.text_bit(Some(len)));
                }
            } else {
                let a_len = if a.off > 0 { 0 } else { a.len };
                let b_ins = if b.off > 0 { 0 } else { b.ins };
                add_section(&mut sections, a_len, b_ins, open);
                if let Some(ref mut ins_list) = inserted {
                    if b.off == 0 {
                        add_insert(ins_list, &sections, b.text());
                    }
                }
            }

            open = (a.ins > len as i64 || (b.ins >= 0 && b.len > len))
                && (open || sections.len() > section_len);
            a.forward2(len);
            b.forward(len);
        }
    }
}

fn map_impl(
    set_a: &ChangeDesc,
    insert_a: Option<&[Text]>,
    set_b: &ChangeDesc,
    before: bool,
) -> (ChangeDesc, Option<Vec<Text>>) {
    let mut sections = Vec::new();
    let mut inserted = if insert_a.is_some() {
        Some(Vec::new())
    } else {
        None
    };

    let mut a = SectionIter::new(set_a, insert_a);
    let mut b = SectionIter::new(set_b, None);
    let mut inserted_idx: isize = -1;

    loop {
        if a.done() && b.len > 0 || b.done() && a.len > 0 {
            panic!("Mismatched change set lengths in map");
        } else if a.ins == -1 && b.ins == -1 {
            let len = a.len.min(b.len);
            add_section(&mut sections, len, -1, false);
            a.forward(len);
            b.forward(len);
        } else if b.ins >= 0
            && (a.ins < 0
                || inserted_idx == (a.i as isize)
                || (a.off == 0 && (b.len < a.len || (b.len == a.len && !before))))
        {
            let mut len = b.len;
            add_section(&mut sections, b.ins as usize, -1, false);
            while len > 0 {
                let piece = a.len.min(len);
                if a.ins >= 0 && inserted_idx < (a.i as isize) && a.len <= piece {
                    add_section(&mut sections, 0, a.ins, false);
                    if let Some(ref mut ins_list) = inserted {
                        add_insert(ins_list, &sections, a.text());
                    }
                    inserted_idx = a.i as isize;
                }
                a.forward(piece);
                len -= piece;
            }
            b.next();
        } else if a.ins >= 0 {
            let mut len = 0;
            let mut left = a.len;
            while left > 0 {
                if b.ins == -1 {
                    let piece = left.min(b.len);
                    len += piece;
                    left -= piece;
                    b.forward(piece);
                } else if b.ins == 0 && b.len < left {
                    left -= b.len;
                    b.next();
                } else {
                    break;
                }
            }
            let a_ins = if inserted_idx < (a.i as isize) { a.ins } else { 0 };
            add_section(&mut sections, len, a_ins, false);
            if let Some(ref mut ins_list) = inserted {
                if inserted_idx < (a.i as isize) {
                    add_insert(ins_list, &sections, a.text());
                }
            }
            inserted_idx = a.i as isize;
            let forward_len = a.len - left;
            a.forward(forward_len);
        } else if a.done() && b.done() {
            return (ChangeDesc::create(sections), inserted);
        } else {
            panic!("Mismatched change set lengths in map");
        }
    }
}
