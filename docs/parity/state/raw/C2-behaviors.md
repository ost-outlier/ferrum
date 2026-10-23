# C2 behaviors (`mirrors/cm6/state/src/change.ts` vs `crates/ferrum-state/src/change.rs`)

> What: chunk C2 behavior sections. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `docs/parity/state/raw/C2-api.md` § Reproductions.

31 cm6 items, none DROP; 31 sections below. Repro ids (R1–R12) refer to `docs/parity/state/raw/C2-api.md` `## Reproductions`. "Oracle" refers to the differential oracle in `docs/parity/state/raw/C2-notes.md` `## Oracle` (25,000 ASCII cases, 0 mismatches); the oracle is evidence, not a Ferrum test, so behaviors proven only by the oracle are still marked "implemented, untested".

Shared helpers read (cm6), used by several items below:
- `addSection` `mirrors/cm6/state/src/change.ts:384-391`: drops `len==0 && ins<=0`; joins with the previous section when both are gaps or both deletions with the same `ins` (`ins<=0 && ins==last ins`), joins a zero-length insertion into a previous zero-length section, joins anything when `forceJoin`. Ferrum `add_section` `crates/ferrum-state/src/change.rs:553-576`: same four branches, same order.
- `addInsert` `mirrors/cm6/state/src/change.ts:393-402`: ignores empty text; appends to the last text when the section was joined, else pads with `Text.empty` up to the section index. Ferrum `add_insert` `change.rs:578-593`: same.
- `iterChanges` fn `mirrors/cm6/state/src/change.ts:404-424`. Ferrum `iter_changes` fn `change.rs:595-642`: same, plus a bounds guard on `inserted` (`change.rs:624`).
- `mapSet` `mirrors/cm6/state/src/change.ts:428-490`. Ferrum `map_impl` `change.rs:810-887`: same branch structure; panics `Mismatched change set lengths in map` where cm6 throws `Error("Mismatched change set lengths")`.
- `composeSets` `mirrors/cm6/state/src/change.ts:494-528`. Ferrum `compose_impl` `change.rs:737-808`: same; cm6 `if (insert && insB)` is truthy for `insB == -1` too (calls `addInsert` with an always-empty text for unchanged sections), Ferrum `if ins_b > 0` (`change.rs:781`): no observable difference (addInsert ignores empty text). Panic text `Mismatched change set lengths in compose`.
- `SectionIter` `mirrors/cm6/state/src/change.ts:530-576` (`next`, `done`, `len2`, `text`, `textBit`, `forward`, `forward2`). Ferrum `SectionIter` `change.rs:644-735`: same; `text_bit` returns empty text when the index is past `inserted` even with a `len` (cm6 would throw on `undefined.slice`, unreachable with well-formed sets) and clamps `end` (cm6 `Text.slice` clamps too).
- `Text.of`, `Text.slice`, `Text.append`, `Text.replace`, `Text.empty` (`mirrors/cm6/state/src/text.ts`, C1 items) for inserted text. Ferrum `Text::from_str`/`of`/`slice`/`append` (`crates/ferrum-state/src/text.rs:28`, `:86`, `:170`, `:182`).

---

### state.cm6.135 `ChangeSpec` — cm6 `mirrors/cm6/state/src/change.ts:188`
Helpers read: `ChangeSet.of` `process()` `mirrors/cm6/state/src/change.ts:323-344`; `DefaultSplit` `mirrors/cm6/state/src/change.ts:3`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSpec.B1 | Object form `{from, to?, insert?}`; `to` defaults to `from`, missing `insert` = no text | `mirrors/cm6/state/src/change.ts:189`, `:332`, `:335` | `ChangeSpec` `change.rs:246`; default `to` only via `ChangeSpec::insert` `change.rs:261`; struct literal requires `to` | test-change#28–#34 | test_change#030–#036 | differs: `to` not optional in the type (constructor emulates the default) |
| state.ChangeSpec.B2 | `insert` as string: split on `lineSep` or `DefaultSplit` (`/\r\n?\|\n/`) into lines | `mirrors/cm6/state/src/change.ts:335` | none (`From<&str> for Text` `text.rs:218` keeps `\r\n`/`\r` raw) | none | none | differs (R1f, R1g, R1h) |
| state.ChangeSpec.B3 | `insert` as `Text` used as-is | `mirrors/cm6/state/src/change.ts:335` | `ChangeSpec.insert: Text` `change.rs:249` | none | none (test_change uses `&str`) | implemented, untested (R1e same) |
| state.ChangeSpec.B4 | A `ChangeSet` may appear as a spec or inside an array | `mirrors/cm6/state/src/change.ts:190`, `:326` | `Changes::Set` `change.rs:282` (top level only) | none | test_state#002 (top level via `.into()`) | differs: not inside a list (R1b) |
| state.ChangeSpec.B5 | Arrays nest recursively | `mirrors/cm6/state/src/change.ts:191`, `:324-325` | none (`Changes::Multiple(Vec<ChangeSpec>)` flat) | none | none | missing (R1a) |
| state.ChangeSpec.B6 | A `ChangeSet` spec must have the target length (RangeError otherwise) | `mirrors/cm6/state/src/change.ts:327-328` | none (`Changes::to_changeset` `change.rs:293` returns it unchecked) | none | none | missing (R1d2) |
Proposed status: PORTED, DEVIATION because B2, B4, B5, B6 differ or are missing and B1's default is only available through a constructor.

### state.cm6.136 `ChangeSet` — cm6 `mirrors/cm6/state/src/change.ts:196`
Helpers read: `ChangeDesc` (`:21-179`), `createSet` `:379`, `filter` `:269` (internal).
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.B1 | Stores sections plus `inserted: Text[]` indexed by section index (`i >> 1`), padded with `Text.empty` | `mirrors/cm6/state/src/change.ts:197-203` | `ChangeSet {desc, inserted}` `change.rs:320-323` | test-change#28–#40 | test_change#030–#042 | implemented+tested |
| state.ChangeSet.B2 | Inherits all `ChangeDesc` members (`length`, `newLength`, `empty`, `iterGaps`, `iterChangedRanges`, `invertedDesc`, `composeDesc`, `mapPos`, `touchesRange`) | `mirrors/cm6/state/src/change.ts:196` (`extends`) | `length`/`new_length`/`is_empty`/`iter_gaps`/`map_pos` re-implemented `change.rs:343-376`; the rest only through `desc()`; `touchesRange` nowhere | test-change#34 | test_change#036 | differs: `ChangeSet::map_pos` has no mode (state.fe.039); `touches_range` missing |
| state.ChangeSet.B3 | Constructor private; creation via `of`, `empty`, `fromJSON` and internal `createSet` | `mirrors/cm6/state/src/change.ts:197`, `:379` | `create_set` is `pub` `change.rs:327` | none | none | differs: raw constructor public, unvalidated (R11c) |
| state.ChangeSet.B4 | Internal `filter(ranges)` used by change filters | `mirrors/cm6/state/src/change.ts:269-296` | none in `change.rs` (C5 owns `changeFilter`) | (test-state changeFilter cases, C5) | (C5) | not audited here (not a public item); recorded in notes |
| state.ChangeSet.B5 | Immutable value; no equality semantics | `mirrors/cm6/state/src/change.ts:200` (`readonly`) | `#[derive(Clone, PartialEq, Eq)]` `change.rs:319` | none | none | differs (extra structural equality; padding entries in `inserted` take part in it) |
Proposed status: PORTED, DEVIATION because B2, B3, B5 differ and `toJSON`/`fromJSON` are missing (rows 144, 147).

### state.cm6.137 `ChangeSet.apply` — cm6 `mirrors/cm6/state/src/change.ts:207`
Helpers read: `iterChanges` fn `:404`; `Text.replace` (`mirrors/cm6/state/src/text.ts`).
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.apply.B1 | `this.length != doc.length` → `RangeError("Applying change set to a document with the wrong length")` | `mirrors/cm6/state/src/change.ts:208` | `change.rs:380-386` panic `Mismatched document length: changeset expected {}, doc is {}` | none | none | differs: message; untested (R2e) |
| state.ChangeSet.apply.B2 | Applies each (joined) change in order, replacing `fromB..fromB+(toA-fromA)` in the progressively updated doc with the inserted text | `mirrors/cm6/state/src/change.ts:209` | `change.rs:391-408` (builds a new text from slices of the old doc and inserted texts, using `fromA/toA`) | test-change#29–#33, #35–#39 | test_change#031–#035, #037–#041, test_state#002 | implemented+tested (different algorithm, same result; oracle `Aapply`, `compApply`, `invApply`) |
| state.ChangeSet.apply.B3 | Empty set: no iteration, returns the same `doc` | `mirrors/cm6/state/src/change.ts:209-210` | `change.rs:387-389` returns `doc.clone()` | none | none directly (test_change#041 composes from `ChangeSet::empty`, then applies non-empty) | implemented, untested (equal value, R12) |
| state.ChangeSet.apply.B4 | Multi-line inserted text keeps its lines | `mirrors/cm6/state/src/change.ts:209` | `Text::append` `text.rs:182` | none | none (random tests are single-line) | implemented, untested (oracle documents and inserts contain `\n`: 0 mismatches) |
| state.ChangeSet.apply.B5 | Cost: one `Text.replace` per joined change (tree path copy, O(log n) each) | `mirrors/cm6/state/src/change.ts:209` | one `slice` + `append` per change; ropey `From<RopeSlice> for Rope` shares nodes (`ropey-1.6.1/src/rope.rs:1851-1881`), `append` O(log N) (`rope.rs:561-564`) | n/a | n/a | implemented (comparable cost; no whole-document copy) |
Text semantics: lengths are UTF-16 (cm6) vs code points (Ferrum) (D1, R9); line breaks of the doc follow the `Text` model (D2, C1).
Proposed status: PORTED, DEVIATION because B1 differs (message) and B1, B3, B4 are untested.

### state.cm6.138 `ChangeSet.mapDesc` — cm6 `mirrors/cm6/state/src/change.ts:213`
Helpers read: `mapSet` `:428-490` with `mkSet = true`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.mapDesc.B1 | Maps the set over `other` and returns a `ChangeSet` (keeps inserted text); unlike `map`, no `other.empty` shortcut | `mirrors/cm6/state/src/change.ts:213` | `ChangeSet::map` `change.rs:466-476` (has the shortcut, returns a clone) | none | test_change#034, #037, #038 | implemented+tested via `map` (identity difference only, R8b) |
| state.ChangeSet.mapDesc.B2 | `before` defaults to `false` | `mirrors/cm6/state/src/change.ts:213` | `before: bool` required | none | test_change#034 | differs (Rust has no defaults) |
Proposed status: PORTED, RUST-IDIOMATIC because the behavior is available as `ChangeSet::map` with the same results; the only difference (object identity when `other` is empty) is not observable in Rust value semantics. `ChangeSet::desc().map_desc()` must not be used as the counterpart: it drops the inserted text.

### state.cm6.139 `ChangeSet.invert` — cm6 `mirrors/cm6/state/src/change.ts:219`
Helpers read: `Text.slice`, `Text.empty`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.invert.B1 | Swaps `(len, ins)` of every change section, keeps gaps | `mirrors/cm6/state/src/change.ts:220-224` | `change.rs:413-423` | test-change#33, #39 | test_change#035, #041 | implemented+tested |
| state.ChangeSet.invert.B2 | Inserted text of each inverted section = `doc.slice(pos, pos+len)` of the original doc; `Text.empty` for pure insertions | `mirrors/cm6/state/src/change.ts:227` | `change.rs:428-433` | test-change#33, #39 | test_change#035, #041 | implemented+tested |
| state.ChangeSet.invert.B3 | `inserted` padded with `Text.empty` up to the section index | `mirrors/cm6/state/src/change.ts:225-226` | `change.rs:424-427` | test-change#33 | test_change#035 | implemented+tested (indirectly through apply) |
| state.ChangeSet.invert.B4 | `doc` not validated against `length`; a shorter doc yields an inconsistent set without error | `mirrors/cm6/state/src/change.ts:219-231` | `change.rs:412-443` (`Text::slice` clamps, `text.rs:170-180`) | none | none | implemented, untested (R2o) |
Proposed status: PORTED, DEVIATION because B4 is untested (the rest is line-by-line equal and oracle-verified).

### state.cm6.140 `ChangeSet.compose` — cm6 `mirrors/cm6/state/src/change.ts:238`
Helpers read: `composeSets` `:494-528`, `SectionIter` `:530-576`, `addSection`, `addInsert`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.compose.B1 | `this.empty` → returns `other` (no length check) | `mirrors/cm6/state/src/change.ts:238` | `change.rs:447-448` | test-change#39 (fold starts at `ChangeSet.of([], len)`) | test_change#041 (fold starts at `ChangeSet::empty`) | implemented+tested (R2r same) |
| state.ChangeSet.compose.B2 | `other.empty` → returns `this` | `mirrors/cm6/state/src/change.ts:238` | `change.rs:449-450` | test-change#37, #38 (`r(6)` may give 0 changes) | none (Ferrum generators never produce an empty set) | implemented, untested |
| state.ChangeSet.compose.B3 | Combined sections with text: deletions in A pass, insertions in B pass, A's inserted text clipped by B's deletions (`textBit`), B's text replaces overlapping parts | `mirrors/cm6/state/src/change.ts:501-526` | `change.rs:754-806` | test-change#30, #31, #37, #38 | test_change#032, #033, #039, #040 | implemented+tested |
| state.ChangeSet.compose.B4 | `open` flag force-joins pieces of one replacement | `mirrors/cm6/state/src/change.ts:523` | `change.rs:802-803` | test-change#07 (desc level) | test_change#007 | implemented+tested |
| state.ChangeSet.compose.B5 | Length mismatch → `Error("Mismatched change set lengths")` | `mirrors/cm6/state/src/change.ts:508-509` | `change.rs:765-766` panic `Mismatched change set lengths in compose` | test-change#08 (desc level) | test_change#008–#010 (desc level only) | differs: message suffix; untested at set level (R2g) |
Proposed status: PORTED, DEVIATION because B5's message differs and B2/B5 are untested on `ChangeSet`.

### state.cm6.141 `ChangeSet.map` — cm6 `mirrors/cm6/state/src/change.ts:251`
Helpers read: `mapSet` `:428-490`, `SectionIter`, `addSection`, `addInsert`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.map.B1 | `other.empty` → returns `this` (no length check) | `mirrors/cm6/state/src/change.ts:251` | `change.rs:467-468` | none | none | implemented, untested (R2p same) |
| state.ChangeSet.map.B2 | Gaps in both → gap; B's changes shift A; A's changes keep their text | `mirrors/cm6/state/src/change.ts:437-483` | `change.rs:827-880` | test-change#32 | test_change#034 | implemented+tested |
| state.ChangeSet.map.B3 | `before` decides order of insertions at the same position (B first unless `before`) | `mirrors/cm6/state/src/change.ts:446` | `change.rs:835-838` | test-change#11, #35 | test_change#013, #037 | implemented+tested |
| state.ChangeSet.map.B4 | Convergence: `A.compose(B.map(A))` and `B.compose(A.map(B, true))` give the same doc; also over chains | `mirrors/cm6/state/src/change.ts:246-251` (doc) | same code | test-change#35, #36 | test_change#037, #038 (weaker parameters, see `docs/parity/state/raw/C2-tests.md`) | implemented+tested |
| state.ChangeSet.map.B5 | Length mismatch → `Error("Mismatched change set lengths")` | `mirrors/cm6/state/src/change.ts:438-439`, `:487` | `change.rs:828-829`, `:884` panic `… in map` | none | none | differs: message; untested (R2h) |
| state.ChangeSet.map.B6 | `other` may be a `ChangeDesc` or a `ChangeSet` | `mirrors/cm6/state/src/change.ts:251` (`other: ChangeDesc`) | `other: &ChangeDesc` (pass `set.desc()`) | test-change#32 | test_change#034 | implemented+tested (Rust shape) |
Proposed status: PORTED, DEVIATION because B5's message differs and B1/B5 are untested.

### state.cm6.142 `ChangeSet.iterChanges` — cm6 `mirrors/cm6/state/src/change.ts:260`
Helpers read: `iterChanges` fn `:404-424`, `Text.append`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.iterChanges.B1 | Calls `f(fromA, toA, fromB, toB, inserted)` for each change in order, skipping gaps | `mirrors/cm6/state/src/change.ts:408-422` | `change.rs:605-641` | test-change#34 | test_change#036 | implemented+tested |
| state.ChangeSet.iterChanges.B2 | Adjacent change sections (no gap in between) are joined into one call with concatenated text | `mirrors/cm6/state/src/change.ts:414-419` | `change.rs:618-635` | none | none | implemented, untested (R4a; oracle `Ach`) |
| state.ChangeSet.iterChanges.B3 | `individual = true` reports adjacent changes separately | `mirrors/cm6/state/src/change.ts:417` | `change.rs:629` | none | none | implemented, untested (R4b; oracle `AchI`) |
| state.ChangeSet.iterChanges.B4 | Deletions report an empty `Text` | `mirrors/cm6/state/src/change.ts:413`, `:416` | `change.rs:616`, `:621` | test-change#34 | test_change#036 | implemented+tested |
| state.ChangeSet.iterChanges.B5 | `individual` defaults to `false` | `mirrors/cm6/state/src/change.ts:260` | `individual: bool` required | test-change#34 | test_change#036 (passes `false`) | differs (no defaults) |
Proposed status: PORTED, DEVIATION because B2/B3 are untested (cm6 does not test them either) and B5 has no default.

### state.cm6.143 `ChangeSet.desc` — cm6 `mirrors/cm6/state/src/change.ts:266`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.desc.B1 | Returns a `ChangeDesc` with the same sections, without text | `mirrors/cm6/state/src/change.ts:266` | `change.rs:358-360` (`&self.desc`) | test-change#28 | test_change#030 | implemented+tested |
| state.ChangeSet.desc.B2 | New object per call (not `instanceof ChangeSet`) | `mirrors/cm6/state/src/change.ts:266` | borrow of the embedded desc | none | none | Rust shape (no allocation; R8f) |
Proposed status: PORTED, RUST-IDIOMATIC because the returned description is identical and the borrow avoids an allocation.

### state.cm6.144 `ChangeSet.toJSON` — cm6 `mirrors/cm6/state/src/change.ts:299`
Helpers read: `Text.toJSON` (`mirrors/cm6/state/src/text.ts`, lines array).
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.toJSON.B1 | Gap → plain number `len` | `mirrors/cm6/state/src/change.ts:303` | none | test-change#40 | none | missing |
| state.ChangeSet.toJSON.B2 | Deletion (`ins == 0`) → `[len]` | `mirrors/cm6/state/src/change.ts:304` | none | test-change#40 | none | missing |
| state.ChangeSet.toJSON.B3 | Insertion/replacement → `[len, ...lines]` (inserted text as lines) | `mirrors/cm6/state/src/change.ts:305` | none | test-change#40 | none | missing (R6a) |
| state.ChangeSet.toJSON.B4 | Empty set: `[]` for length 0, `[n]` otherwise | `mirrors/cm6/state/src/change.ts:299-307` | none | none | none | missing (R1k, R1l) |
Proposed status: MISSING (searched: see `docs/parity/state/raw/C2-api.md` header).

### state.cm6.145 `ChangeSet^of` — cm6 `mirrors/cm6/state/src/change.ts:312`
Helpers read: `flush`/`process` closures `:316-344`, `addSection`, `addInsert`, `compose` `:238`, `map` `:251`, `Text.of`, `DefaultSplit` `:3`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.of.B1 | Single object spec | `mirrors/cm6/state/src/change.ts:331-343` | `of_spec` `change.rs:479` | test-state#05 | test_state#005, test_change#033 | implemented+tested |
| state.ChangeSet.of.B2 | Array of specs | `mirrors/cm6/state/src/change.ts:324-325` | `of_specs` `change.rs:484`, `Changes::Multiple` | test-change#28–#40 | test_change#030–#042 | implemented+tested |
| state.ChangeSet.of.B3 | Nested arrays processed recursively | `mirrors/cm6/state/src/change.ts:324-325` | none | none | none | missing (R1a) |
| state.ChangeSet.of.B4 | `ChangeSet` spec: length check (`RangeError: Mismatched change set length (got X, expected Y)`), flush pending sections, compose `spec.map(total)` | `mirrors/cm6/state/src/change.ts:326-330` | `Changes::Set` returned as-is `change.rs:293`; not allowed inside a list | none | test_state#002 (top-level set, right length) | differs (R1b, R1d2) |
| state.ChangeSet.of.B5 | `from > to \|\| from < 0 \|\| to > length` → `RangeError("Invalid change range F to T (in doc of length L)")` | `mirrors/cm6/state/src/change.ts:333-334` | `change.rs:513-518` panic `Invalid change range F..T for document of length L` (`from<0` unrepresentable) | test-state#05 | test_state#005 (any panic, `from > to` only) | differs: message (R2a, R2b) |
| state.ChangeSet.of.B6 | String insert split on `lineSep \|\| DefaultSplit`; `lineSep` parameter | `mirrors/cm6/state/src/change.ts:312`, `:335` | none (no parameter; `Text::from` keeps separators) | none | none | missing (R1f, R1g, R1h) |
| state.ChangeSet.of.B7 | `Text` insert used directly | `mirrors/cm6/state/src/change.ts:335` | `change.rs:519`, `:530` | none | none | implemented, untested (R1e) |
| state.ChangeSet.of.B8 | Empty spec (`from == to` and no text) skipped | `mirrors/cm6/state/src/change.ts:337` | `change.rs:520-522` | none | none | implemented, untested (R1j) |
| state.ChangeSet.of.B9 | Specs with `from >= pos` accumulate in one section list; `from < pos` flushes, and later groups are mapped over earlier ones (positions always refer to the original doc) | `mirrors/cm6/state/src/change.ts:316-322`, `:338-342` | `change.rs:490-510`, `:523-531` | test-change#28, #29 | test_change#030, #031 | implemented+tested (R1m) |
| state.ChangeSet.of.B10 | Overlapping specs both apply (second mapped over first) | `mirrors/cm6/state/src/change.ts:320` | `change.rs:502-508` | test-change#28 | test_change#030 | implemented+tested (R1n) |
| state.ChangeSet.of.B11 | No effective changes → empty set of `length` (`flush(!total)`) | `mirrors/cm6/state/src/change.ts:346-348` | `change.rs:534-537` | test-change#39 (`ChangeSet.of([], 10)`) | none (test_change#041 uses `ChangeSet::empty`) | implemented, untested (R1k, R1l) |
| state.ChangeSet.of.B12 | Sections normalized by `addSection` and texts merged by `addInsert` (e.g. two inserts at 5 → one `0:4`) | `mirrors/cm6/state/src/change.ts:339-341` | `change.rs:526-530` | test-change#28 | test_change#030 | implemented+tested |
| state.ChangeSet.of.B13 | `to` defaults to `from` | `mirrors/cm6/state/src/change.ts:332` | `ChangeSpec::insert` `change.rs:261` | test-change#28 | test_change#030 | implemented+tested (constructor) |
| state.ChangeSet.of.B14 | Insert length measured with `Text.length` (UTF-16; line break = 1) | `mirrors/cm6/state/src/change.ts:336` | `spec.insert.len_chars()` `change.rs:519` (code points; `\r\n` = 2) | none | none | differs for astral chars (D1, R9b) and CR (D2, R1f, R9f) |
Proposed status: PORTED, DEVIATION because B3 and B6 are missing, B4, B5, B14 differ, and B7, B8, B11 are untested.

### state.cm6.146 `ChangeSet^empty` — cm6 `mirrors/cm6/state/src/change.ts:352`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.empty.B1 | `length > 0` → sections `[length, -1]`, no text | `mirrors/cm6/state/src/change.ts:353` | `change.rs:335-340` → `ChangeDesc::empty` `change.rs:33-43` | none | test_change#041 | implemented+tested |
| state.ChangeSet.empty.B2 | `length == 0` → sections `[]` | `mirrors/cm6/state/src/change.ts:353` | `change.rs:38-41` | none | none | implemented, untested (R3a) |
Proposed status: PORTED, DEVIATION because B2 is untested.

### state.cm6.147 `ChangeSet^fromJSON` — cm6 `mirrors/cm6/state/src/change.ts:358`
Helpers read: `Text.of`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeSet.fromJSON.B1 | Non-array → `RangeError("Invalid JSON representation of ChangeSet")` | `mirrors/cm6/state/src/change.ts:359` | none | none | none | missing (R2j) |
| state.ChangeSet.fromJSON.B2 | Number part → gap | `mirrors/cm6/state/src/change.ts:363-364` | none | test-change#40 | none | missing |
| state.ChangeSet.fromJSON.B3 | `[len]` → deletion | `mirrors/cm6/state/src/change.ts:367-368` | none | test-change#40 | none | missing |
| state.ChangeSet.fromJSON.B4 | `[len, ...lines]` → replacement with `Text.of(lines)`, `ins = text.length`, padding | `mirrors/cm6/state/src/change.ts:369-372` | none | test-change#40 | none | missing (R6b) |
| state.ChangeSet.fromJSON.B5 | Malformed part (not array, non-number head, non-string line) → RangeError | `mirrors/cm6/state/src/change.ts:365-366` | none | none | none | missing (R2k) |
| state.ChangeSet.fromJSON.B6 | No semantic validation (negative lengths accepted) | `mirrors/cm6/state/src/change.ts:358-376` | none | none | none | missing |
Proposed status: MISSING (searched: see `docs/parity/state/raw/C2-api.md` header).

### state.cm6.148 `ChangeDesc` — cm6 `mirrors/cm6/state/src/change.ts:21`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.B1 | Sections `[len, ins]*`: `ins == -1` gap, `ins >= 0` replacement length | `mirrors/cm6/state/src/change.ts:22-31` | `change.rs:17-24` (`Vec<i64>`) | test-change#01–#27 | test_change#001–#029 | implemented+tested |
| state.ChangeDesc.B2 | Constructor protected; internal `create` | `mirrors/cm6/state/src/change.ts:28`, `:178` | `ChangeDesc::create` `pub` `change.rs:28`, `from_string` `pub` `change.rs:198` | none | test_change#001–#029 (through `from_string`) | differs: raw construction public (R2s) |
| state.ChangeDesc.B3 | `toString`: `len` for gaps, `len:ins` otherwise, space separated | `mirrors/cm6/state/src/change.ts:157-164` | `Display` `change.rs:216-236` | test-change#01–#19 | test_change#001–#021 | implemented+tested (R10) |
| state.ChangeDesc.B4 | No equality semantics | — | `#[derive(Clone, PartialEq, Eq)]` `change.rs:21` | none | none | differs (extra raw-section equality; unnormalized sections compare unequal) |
Proposed status: PORTED, DEVIATION because B2 and B4 differ and `touchesRange`/`toJSON`/`fromJSON` are missing.

### state.cm6.149 `ChangeDesc.length` — cm6 `mirrors/cm6/state/src/change.ts:34`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.length.B1 | Sum of section lengths (0 for no sections), in the position unit | `mirrors/cm6/state/src/change.ts:34-38` | `change.rs:46-54` | test-change#29–#39 (apply guard) | test_change#031–#041 (apply guard `change.rs:380`) | implemented+tested (indirect; oracle `Alen`) |
Proposed status: PORTED (unit dependency D1 recorded).

### state.cm6.150 `ChangeDesc.newLength` — cm6 `mirrors/cm6/state/src/change.ts:41`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.newLength.B1 | Sum of `len` for gaps and `ins` for changes | `mirrors/cm6/state/src/change.ts:41-48` | `change.rs:57-70` | test-change#36–#38 | test_change#038–#040 | implemented+tested (indirect) |
Proposed status: PORTED (D1 recorded).

### state.cm6.151 `ChangeDesc.empty` — cm6 `mirrors/cm6/state/src/change.ts:51`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.empty.B1 | No sections → `true` | `mirrors/cm6/state/src/change.ts:51` | `change.rs:74` | none | none | implemented, untested |
| state.ChangeDesc.empty.B2 | Exactly one gap section → `true` | `mirrors/cm6/state/src/change.ts:51` | `change.rs:74` | none | none | implemented, untested (R3b) |
| state.ChangeDesc.empty.B3 | Anything else → `false` (also unnormalized two-gap descs) | `mirrors/cm6/state/src/change.ts:51` | `change.rs:74` | none | test_change#032 (indirect: a wrong `true` would short-circuit compose) | implemented+tested (indirect; R3e) |
Proposed status: PORTED, DEVIATION because B1, B2 are untested (oracle: 4,160 empty sets, 0 mismatches). Renamed `is_empty`.

### state.cm6.152 `ChangeDesc.iterGaps` — cm6 `mirrors/cm6/state/src/change.ts:56`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.iterGaps.B1 | `f(posA, posB, len)` for each gap in order | `mirrors/cm6/state/src/change.ts:57-61` | `change.rs:83-89` | test-change#34 | test_change#036 | implemented+tested |
| state.ChangeDesc.iterGaps.B2 | `posB` advances by `ins` over changes, `posA` by `len` | `mirrors/cm6/state/src/change.ts:62-65` | `change.rs:90-93` | test-change#34 | test_change#036 | implemented+tested |
Proposed status: PORTED.

### state.cm6.153 `ChangeDesc.iterChangedRanges` — cm6 `mirrors/cm6/state/src/change.ts:79`
Helpers read: `iterChanges` fn `:404`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.iterChangedRanges.B1 | `f(fromA, toA, fromB, toB)` per change, no text | `mirrors/cm6/state/src/change.ts:79-81` | `change.rs:99-103` (passes `None` for text) | none | none | implemented, untested (R4c) |
| state.ChangeDesc.iterChangedRanges.B2 | Adjacent changes joined unless `individual` | `mirrors/cm6/state/src/change.ts:417` | `change.rs:629` | none | none | implemented, untested (R4c/R4d; oracle `ranges`) |
| state.ChangeDesc.iterChangedRanges.B3 | `individual` defaults to `false` | `mirrors/cm6/state/src/change.ts:79` | required `bool` | none | none | differs (no defaults) |
Proposed status: PORTED, DEVIATION because B1, B2 are untested and B3 has no default.

### state.cm6.154 `ChangeDesc.invertedDesc` — cm6 `mirrors/cm6/state/src/change.ts:84`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.invertedDesc.B1 | Swaps `(len, ins)` for changes, keeps gaps; returns a `ChangeDesc` (getter) | `mirrors/cm6/state/src/change.ts:84-92` | `change.rs:106-122` (method) | none | none | implemented, untested (R8g; oracle `invDesc`) |
Proposed status: PORTED, DEVIATION because B1 is untested.

### state.cm6.155 `ChangeDesc.composeDesc` — cm6 `mirrors/cm6/state/src/change.ts:97`
Helpers read: `composeSets` `:494`, `SectionIter` `:530`, `addSection`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.composeDesc.B1 | `this.empty` → `other` | `mirrors/cm6/state/src/change.ts:97` | `change.rs:127-128` | test-change#06 | test_change#006 | implemented+tested |
| state.ChangeDesc.composeDesc.B2 | `other.empty` → `this` (no length check) | `mirrors/cm6/state/src/change.ts:97` | `change.rs:129-130` | none | none | implemented, untested (R2q) |
| state.ChangeDesc.composeDesc.B3 | Deletion in A passes through; insertion in B passes through | `mirrors/cm6/state/src/change.ts:501-507` | `change.rs:754-764` | test-change#01, #04, #05 | test_change#001, #004, #005 | implemented+tested |
| state.ChangeDesc.composeDesc.B4 | Insertion in A deleted by B cancels | `mirrors/cm6/state/src/change.ts:519-521` | `change.rs:791-800` | test-change#02, #05 | test_change#002, #005 | implemented+tested |
| state.ChangeDesc.composeDesc.B5 | Adjacent insertions/deletions/replacements joined (`addSection`, `open` force-join) | `mirrors/cm6/state/src/change.ts:511-525` | `change.rs:768-805` | test-change#03, #04, #07 | test_change#003, #004, #007 | implemented+tested |
| state.ChangeDesc.composeDesc.B6 | Mismatched lengths → `Error("Mismatched change set lengths")` | `mirrors/cm6/state/src/change.ts:508-509` | `change.rs:765-766` panic `… in compose` | test-change#08 | test_change#008–#010 | differs: message suffix (tests check prefix `Mismatched` only) |
| state.ChangeDesc.composeDesc.B7 | Accepts a `ChangeSet` as `other` (subclass), result is a plain desc | `mirrors/cm6/state/src/change.ts:97`, `:493` | takes `&ChangeDesc` (`set.desc()`) | none | none | implemented, untested (R8i) |
Proposed status: PORTED, DEVIATION because B6's message differs and B2, B7 are untested.

### state.cm6.156 `ChangeDesc.mapDesc` — cm6 `mirrors/cm6/state/src/change.ts:103`
Helpers read: `mapSet` `:428` (`mkSet = false`), `SectionIter`, `addSection`.
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.mapDesc.B1 | `other.empty` → `this` | `mirrors/cm6/state/src/change.ts:103` | `change.rs:139-140` | none | none | implemented, untested |
| state.ChangeDesc.mapDesc.B2 | Gaps of both sets → gap | `mirrors/cm6/state/src/change.ts:440-445` | `change.rs:830-834` | test-change#09, #13 | test_change#011, #015 | implemented+tested |
| state.ChangeDesc.mapDesc.B3 | B's change before A's next change (order: start, length, `before`) is skipped as a gap of `b.ins` | `mirrors/cm6/state/src/change.ts:446-462` | `change.rs:835-854` | test-change#09–#11 | test_change#011–#013 | implemented+tested |
| state.ChangeDesc.mapDesc.B4 | A's change processed up to B's next non-deletion change | `mirrors/cm6/state/src/change.ts:463-483` | `change.rs:855-880` | test-change#12, #16, #19 | test_change#014, #018, #021 | implemented+tested |
| state.ChangeDesc.mapDesc.B5 | Insertions of A inside a deletion of B are kept | `mirrors/cm6/state/src/change.ts:454-458` | `change.rs:844-850` | test-change#15 | test_change#017 | implemented+tested |
| state.ChangeDesc.mapDesc.B6 | Deletion already done by B is dropped; adjacent deletions joined; replacements not joined | `mirrors/cm6/state/src/change.ts:480`, `:384-391` | `change.rs:871-872`, `:553-576` | test-change#14, #17, #18 | test_change#016, #019, #020 | implemented+tested |
| state.ChangeDesc.mapDesc.B7 | Mismatched lengths → `Error("Mismatched change set lengths")` | `mirrors/cm6/state/src/change.ts:438-439`, `:487` | `change.rs:828-829`, `:884` panic `… in map` | none | none | differs: message; untested |
| state.ChangeDesc.mapDesc.B8 | `before` defaults to `false` | `mirrors/cm6/state/src/change.ts:103` | required `bool` | test-change#09–#19 | test_change#011–#021 | differs (no defaults) |
Proposed status: PORTED, DEVIATION because B7 differs and B1/B7 are untested.

### state.cm6.157 `ChangeDesc.mapPos` — cm6 `mirrors/cm6/state/src/change.ts:120`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.mapPos.B1 | Inside a gap (`endA > pos`) → `posB + (pos - posA)` | `mirrors/cm6/state/src/change.ts:126-127` | `change.rs:160-163` | test-change#20, #21 | test_change#022, #023 | implemented+tested |
| state.ChangeDesc.mapPos.B2 | Insertion at `pos` (`len == 0`, `endA == pos`): `assoc < 0` → before (`posB`); `assoc >= 0` → continue past it | `mirrors/cm6/state/src/change.ts:134-136` | `change.rs:179-186` | test-change#20, #22, #26 | test_change#022, #024, #028 | implemented+tested |
| state.ChangeDesc.mapPos.B3 | `pos` strictly inside a replaced range: `assoc < 0` → `posB`, else `posB + ins` | `mirrors/cm6/state/src/change.ts:134-135` | `change.rs:179-184` | test-change#21, #25 | test_change#023, #027 | implemented+tested |
| state.ChangeDesc.mapPos.B4 | `pos == posA` of a replacement → `posB` regardless of `assoc` | `mirrors/cm6/state/src/change.ts:135` | `change.rs:180` | test-change#25 (`[2,2,1]`) | test_change#027 | implemented+tested |
| state.ChangeDesc.mapPos.B5 | `pos == endA` of a replacement with `len > 0` → after the replacement regardless of `assoc` | `mirrors/cm6/state/src/change.ts:134` | `change.rs:179` | test-change#25 (`[4,4,-1]`) | test_change#027 | implemented+tested |
| state.ChangeDesc.mapPos.B6 | Between adjacent replacements stays between | `mirrors/cm6/state/src/change.ts:134-135` | `change.rs:179-184` | test-change#27 | test_change#029 | implemented+tested |
| state.ChangeDesc.mapPos.B7 | Multiple/mixed insertions and deletions accumulate | `mirrors/cm6/state/src/change.ts:124-139` | `change.rs:154-189` | test-change#22–#24 | test_change#024–#026 | implemented+tested |
| state.ChangeDesc.mapPos.B8 | Track modes return `null` before the position is returned (`TrackDel`: `posA < pos < endA`; `TrackBefore`: `posA < pos`; `TrackAfter`: `endA > pos`; only for change sections with `endA >= pos`) | `mirrors/cm6/state/src/change.ts:130-133` | `change.rs:167-177` (`None`) | test-change#21, #25 | test_change#023, #027 | implemented+tested |
| state.ChangeDesc.mapPos.B9 | Pure insertions never produce `null` | `mirrors/cm6/state/src/change.ts:130-133` | `change.rs:167-177` | test-change#20 (no mode cases) | none with mode on insertions | implemented, untested (oracle: modes × insertions, 0 mismatches) |
| state.ChangeDesc.mapPos.B10 | `pos == length` → final `posB` (trailing insertion: before if `assoc < 0`, after otherwise) | `mirrors/cm6/state/src/change.ts:134`, `:141` | `change.rs:179`, `:194` | test-change#20 (`[8,10]`), #22 (`[4,10,1]`) | test_change#022, #024 | implemented+tested |
| state.ChangeDesc.mapPos.B11 | `pos > length` → `RangeError("Position P is out of range for changeset of length L")` | `mirrors/cm6/state/src/change.ts:140` | `change.rs:191-193` panic, same text | none | none | implemented, untested (R2f, R2f2) |
| state.ChangeDesc.mapPos.B12 | Defaults `assoc = -1`, `mode = Simple`; 2-arg overload returns `number` | `mirrors/cm6/state/src/change.ts:120-122` | none: `map_pos(pos, assoc, mode) -> Option<usize>`; `ChangeSet::map_pos(pos, assoc) -> usize` covers only the Simple overload on sets | test-change#20–#27 (omit mode) | test_change#022–#029 (pass `Simple`) | differs (API shape) |
| state.ChangeDesc.mapPos.B13 | No validation of negative or fractional `pos` (R2n → `-1`, R7h → `6.5`) | `mirrors/cm6/state/src/change.ts:122-141` | unrepresentable (`usize`) | none | none | not applicable by type |
| state.ChangeDesc.mapPos.B14 | `assoc` is any number; only its sign matters | `mirrors/cm6/state/src/change.ts:134-135` | `assoc: i8` | test-change#20–#27 (±1) | test_change#022–#029 (±1) | differs: callers with `i32` values must cast; `as i8` truncates (R7j: `-100_000_000 as i8 == 0`) |
| state.ChangeDesc.mapPos.B15 | Position unit | `mirrors/cm6/state/src/change.ts:120` | `change.rs:149` | none non-ASCII | none non-ASCII | differs for astral chars (D1, R9) |
Proposed status: PORTED, DEVIATION because B12, B14 differ and B9, B11 are untested.

### state.cm6.158 `ChangeDesc.touchesRange` — cm6 `mirrors/cm6/state/src/change.ts:147`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.touchesRange.B1 | No change section with `pos <= to && end >= from` → `false` | `mirrors/cm6/state/src/change.ts:148-153` | none | none | none | missing (R5 `(5,5)`) |
| state.ChangeDesc.touchesRange.B2 | Touching change (bounds inclusive, adjacency counts) → `true` | `mirrors/cm6/state/src/change.ts:150` | none | none | none | missing (R5 `(2,2)`, `(4,4)`) |
| state.ChangeDesc.touchesRange.B3 | Change strictly containing the range (`pos < from && end > to`) → `"cover"` | `mirrors/cm6/state/src/change.ts:150` | none | none | none | missing (R5 `(3,3)`) |
| state.ChangeDesc.touchesRange.B4 | `to` defaults to `from` | `mirrors/cm6/state/src/change.ts:147` | none | none | none | missing |
| state.ChangeDesc.touchesRange.B5 | Stops once `pos > to` | `mirrors/cm6/state/src/change.ts:148` | none | none | none | missing |
| state.ChangeDesc.touchesRange.B6 | Insertion at the boundary touches (`true`, never `"cover"`) | `mirrors/cm6/state/src/change.ts:150` | none | none | none | missing (R5 `(8,8)`) |
| state.ChangeDesc.touchesRange.B7 | Empty set → `false` | `mirrors/cm6/state/src/change.ts:148` | none | none | none | missing |
Proposed status: MISSING (searched: see `docs/parity/state/raw/C2-api.md` header). Needed by `ferrum-view` (notes, Cross-crate facts).

### state.cm6.159 `ChangeDesc.toJSON` — cm6 `mirrors/cm6/state/src/change.ts:167`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.toJSON.B1 | Returns the flat sections array | `mirrors/cm6/state/src/change.ts:167` | none | none | none | missing (R6c) |
Proposed status: MISSING.

### state.cm6.160 `ChangeDesc^fromJSON` — cm6 `mirrors/cm6/state/src/change.ts:171`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.ChangeDesc.fromJSON.B1 | Not an array, odd length, or a non-number element → `RangeError("Invalid JSON representation of ChangeDesc")` | `mirrors/cm6/state/src/change.ts:172-173` | none | none | none | missing (R2l, R2m) |
| state.ChangeDesc.fromJSON.B2 | Otherwise wraps the numbers unvalidated | `mirrors/cm6/state/src/change.ts:174` | none (`ChangeDesc::create` is the unvalidated analogue for `Vec<i64>`) | none | none | missing (R6e) |
Proposed status: MISSING.

### state.cm6.161 `MapMode` — cm6 `mirrors/cm6/state/src/change.ts:6`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.MapMode.B1 | Four members Simple, TrackDel, TrackBefore, TrackAfter (values 0–3) | `mirrors/cm6/state/src/change.ts:6-16` | `change.rs:6-15` | test-change#21, #25 | test_change#023, #027 | implemented+tested |
Proposed status: PORTED.

### state.cm6.162 `MapMode.Simple` — cm6 `mirrors/cm6/state/src/change.ts:9`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.MapMode.Simple.B1 | Never reports deletions; always a position | `mirrors/cm6/state/src/change.ts:130` | `change.rs:167` | test-change#20–#27 | test_change#022–#029 | implemented+tested |
Proposed status: PORTED.

### state.cm6.163 `MapMode.TrackDel` — cm6 `mirrors/cm6/state/src/change.ts:11`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.MapMode.TrackDel.B1 | `null` when a change spans across `pos` (`posA < pos < endA`) | `mirrors/cm6/state/src/change.ts:131` | `change.rs:169` | test-change#21, #25 | test_change#023, #027 | implemented+tested |
Proposed status: PORTED.

### state.cm6.164 `MapMode.TrackBefore` — cm6 `mirrors/cm6/state/src/change.ts:13`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.MapMode.TrackBefore.B1 | `null` when the character before `pos` is changed (`posA < pos <= endA`) | `mirrors/cm6/state/src/change.ts:132` | `change.rs:170` | test-change#21, #25 | test_change#023, #027 | implemented+tested |
Proposed status: PORTED.

### state.cm6.165 `MapMode.TrackAfter` — cm6 `mirrors/cm6/state/src/change.ts:15`
| id | behavior | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.MapMode.TrackAfter.B1 | `null` when the character after `pos` is changed (`posA <= pos < endA`) | `mirrors/cm6/state/src/change.ts:133` | `change.rs:171` | test-change#21, #25 | test_change#023, #027 | implemented+tested |
Proposed status: PORTED.

## Unfinished

none
