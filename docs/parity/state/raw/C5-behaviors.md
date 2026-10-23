# C5 behaviors (state.ts + transaction.ts + built-in facets of extension.ts)

> What: chunk C5 behavior sections. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `docs/parity/state/raw/C5-api.md` § Reproductions.

Scope: 77 cm6 items owned by C5 (`docs/parity/state/raw/a1-cm6-exports.md` rows state.cm6.001–034, 092–133, 259). No item is DROP, so every item has a section below.

Conventions. cm6 paths are relative to `mirrors/cm6/`, Ferrum paths to `ferrum/`. Repro ids (R…) are defined in `docs/parity/state/raw/C5-api.md` `## Reproductions`. cm6 test ids `test-state#NN`, Ferrum test ids `test_state#0NN` (from `docs/parity/state/raw/t1-cm6-tests.md` / `docs/parity/state/raw/t2-ferrum-tests.md`). Other-chunk test ids are cited where they exercise a C5 behavior (`test-facet#NN` / `test_facet#0NN`, `test-charcategory#NN` / `test_charcategory#00N`, and ferrum-commands tests by file:line).

Common helpers read for this chunk (cited once here, referenced below as "pipeline helpers"):
- `mirrors/cm6/state/src/transaction.ts:290` `joinRanges`; `:310` `mergeTransaction`; `:330` `resolveTransactionInner`; `:343` `resolveTransaction`; `:356` `filterTransaction`; `:392` `extendTransaction`; `:405` `asArray`; `:169-188` `Transaction` constructor; `:191` `Transaction.create`.
- `mirrors/cm6/state/src/selection.ts:239` `checkSelection`; `mirrors/cm6/state/src/selection.ts:151` `asSingle`; `mirrors/cm6/state/src/selection.ts:73` `SelectionRange.map`.
- `mirrors/cm6/state/src/change.ts:3` `DefaultSplit`; `mirrors/cm6/state/src/change.ts:335` (string inserts split on `lineSep || DefaultSplit` inside `ChangeSet.of`).
- `mirrors/cm6/state/src/state.ts:42-61` `EditorState` constructor (sets `tr._state` before computing slots); `mirrors/cm6/state/src/state.ts:96-125` `applyTransaction`.
- `mirrors/cm6/state/src/facet.ts` `ensureAddr`/`getAddr` (lazy slot evaluation), `StateField` slot `reconfigure` (new field `create(state)` on the intermediate state) — C4 owns these; read here only as helpers of `create`/`applyTransaction`.
- `mirrors/cm6/state/src/extension.ts:5-31` built-in facet definitions; `mirrors/cm6/state/src/charcategory.ts:28` `makeCategorizer`.

Ferrum code read: `crates/ferrum-state/src/state.rs` (whole), `crates/ferrum-state/src/transaction.rs` (whole), `crates/ferrum-state/src/config.rs` (whole, as helper), `crates/ferrum-state/src/facet.rs:40-278`, `crates/ferrum-state/src/change.rs:244-540`, `crates/ferrum-state/src/selection.rs:1-238`, `crates/ferrum-state/src/char.rs:1-20`, `crates/ferrum-state/src/text.rs:1-230`.

---

### state.cm6.001 `EditorStateConfig` — cm6 `mirrors/cm6/state/src/state.ts:14`
Helpers read: `mirrors/cm6/state/src/state.ts:243-254` (only consumer, `create`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorStateConfig.B1 | all fields optional; `{}` is a valid config | `mirrors/cm6/state/src/state.ts:14-27`, `:243` default `= {}` | `crates/ferrum-state/src/state.rs:82-90` (`#[derive(Default)]`, all `Option`) | test-state#06 | test_state#006 | implemented+tested |
| state.EditorStateConfig.B2 | `doc` accepts a string or a `Text` (see 002) | `mirrors/cm6/state/src/state.ts:21` | `crates/ferrum-state/src/state.rs:85` `Option<Text>` only | test-state#01 | test_state#001 | differs: no string form (see state.EditorStateConfig.doc.B1) |
| state.EditorStateConfig.B3 | `selection` accepts `EditorSelection` or `{anchor, head?}` (see 003) | `mirrors/cm6/state/src/state.ts:24` | `crates/ferrum-state/src/state.rs:87` `Option<EditorSelection>` | none | none | differs: no `{anchor, head}` form |
| state.EditorStateConfig.B4 | `extensions` accepts any `Extension` (nested arrays) | `mirrors/cm6/state/src/state.ts:26` | `crates/ferrum-state/src/state.rs:89` `Option<Vec<Extension>>` | test-state#03 | test_state#003 | implemented+tested (Vec of `Extension`, `Extension::Group` nests) |
Proposed status: PORTED, DEVIATION because B2 and B3 differ.

### state.cm6.002 `EditorStateConfig.doc` — cm6 `mirrors/cm6/state/src/state.ts:21`
Helpers read: `mirrors/cm6/state/src/state.ts:245-246`, `mirrors/cm6/state/src/change.ts:3`, `mirrors/cm6/state/src/text.ts` `Text.of` (C1).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorStateConfig.doc.B1 | a string is split into lines on the static `lineSeparator` facet value, else on `/\r\n?|\n/` (R21, R21c) | `mirrors/cm6/state/src/state.ts:246` | none: `crates/ferrum-state/src/state.rs:85,106` take a `Text`; `Text::from` (`crates/ferrum-state/src/text.rs:218`) uses ropey line breaks and keeps separators | none | none | differs (D2): `"a\r\nb\rc\u2028d"` → cm6 3 lines / length 7, Ferrum 4 lines / length 8 (R21) |
| state.EditorStateConfig.doc.B2 | a `Text` instance is used as is (same object, R21b) | `mirrors/cm6/state/src/state.ts:245` | `crates/ferrum-state/src/state.rs:106` (moved in) | test-state#01 | test_state#001 | implemented+tested |
| state.EditorStateConfig.doc.B3 | absent → empty document | `mirrors/cm6/state/src/state.ts:246` (`config.doc || ""`) | `crates/ferrum-state/src/state.rs:106` (`unwrap_or_default`) | test-state#06 | test_state#006 | implemented+tested |
Proposed status: PORTED, DEVIATION because B1 is missing (D2-dependent).

### state.cm6.003 `EditorStateConfig.selection` — cm6 `mirrors/cm6/state/src/state.ts:24`
Helpers read: `mirrors/cm6/state/src/state.ts:247-251`, `mirrors/cm6/state/src/selection.ts:239`, `mirrors/cm6/state/src/selection.ts:151`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorStateConfig.selection.B1 | absent → cursor at 0 | `mirrors/cm6/state/src/state.ts:247` | `crates/ferrum-state/src/state.rs:107`, `crates/ferrum-state/src/selection.rs:122-126` | test-state#01 | test_state#001 | implemented+tested |
| state.EditorStateConfig.selection.B2 | `{anchor, head?}` → `EditorSelection.single(anchor, head)` (R1b) | `mirrors/cm6/state/src/state.ts:249` | none | none | none | missing |
| state.EditorStateConfig.selection.B3 | validated against doc length: RangeError "Selection points outside of document" (R1) | `mirrors/cm6/state/src/state.ts:250` | none | none | none | missing: Ferrum keeps `10:10` on a 3-char doc; a later `doc().line_at(head)` panics "Invalid position: 10" (R1x) |
| state.EditorStateConfig.selection.B4 | collapsed to the main range when `allowMultipleSelections` is false (R2) | `mirrors/cm6/state/src/state.ts:251` | none | test-state#17 | test_state#017 (asserts `<= 2`) | missing |
Proposed status: PORTED, DEVIATION because B2–B4 are missing.

### state.cm6.004 `EditorStateConfig.extensions` — cm6 `mirrors/cm6/state/src/state.ts:26`
Helpers read: `mirrors/cm6/state/src/state.ts:244`; `Configuration.resolve` (C4).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorStateConfig.extensions.B1 | resolved with an empty compartment map; absent → `[]` | `mirrors/cm6/state/src/state.ts:244` | `crates/ferrum-state/src/state.rs:108-110` (`Configuration::resolve(&exts, None)`) | test-state#03, test-state#12 | test_state#003, test_state#012 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (candidate: `Vec<Extension>` instead of a recursive `Extension` value; same observable result, benefit = static typing; equivalence evidence test_state#003/#012).

### state.cm6.005 `EditorState` — cm6 `mirrors/cm6/state/src/state.ts:36`
Helpers read: `mirrors/cm6/state/src/state.ts:42-61`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.B1 | immutable: constructor private, `doc`/`selection` readonly | `mirrors/cm6/state/src/state.ts:42-53` | `crates/ferrum-state/src/state.rs:95-101` private fields; only `create`/`apply` construct | test-state#01 | test_state#001 | implemented+tested |
| state.EditorState.B2 | a state is a plain object; identity (`===`) of states and of their facet/field values is what consumers compare | `mirrors/cm6/state/src/state.ts:36`; consumers `mirrors/cm6/view/src/draw-selection.ts:69`, `mirrors/cm6/view/src/gutter.ts:225` | `crates/ferrum-state/src/state.rs:105,134,308,364` return `Arc<EditorState>`; each `apply`/`Transaction::state()` call builds a new `Arc` (R4, R44) | test-state#16 (`ist(state2.facet(facet), state.facet(facet))`) | test_state#016 (does not assert identity) | differs: see state.Transaction.state.B1, state.EditorState.facet.B4 |
| state.EditorState.B3 | full member set: `field`, `update`, `replaceSelection`, `changeByRange`, `changes`, `toText`, `sliceDoc`, `facet`, `toJSON`, `fromJSON`, `create`, built-in facets, getters `tabSize`/`lineBreak`/`readOnly`, `phrase`, `languageDataAt`, `charCategorizer`, `wordAt` | `mirrors/cm6/state/src/state.ts:36-444` | present: `field`, `update`, `replace_selection`, `change_by_range`, `changes`, `facet`, `create`, `char_categorizer`, `is_read_only`, facet accessors (`crates/ferrum-state/src/state.rs:103-366`) | — | — | differs: missing `toText`, `sliceDoc`, `toJSON`, `fromJSON`, `tabSize` getter, `lineBreak`, `phrase`, `phrases`, `languageData`, `languageDataAt`, `wordAt`, `changeFilter`, `transactionExtender` (searched, see docs/parity/state/raw/C5-api.md) |
Proposed status: PORTED, DEVIATION because B2 and B3 differ.

### state.cm6.006 `EditorState.doc` — cm6 `mirrors/cm6/state/src/state.ts:46`
Helpers read: none.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.doc.B1 | returns the current document | `mirrors/cm6/state/src/state.ts:46` | `crates/ferrum-state/src/state.rs:138-140` | test-state#01, test-state#02 | test_state#001, test_state#002 | implemented+tested |
Proposed status: PORTED (accessor only; the document's line model is a C1/D2 question).

### state.cm6.007 `EditorState.selection` — cm6 `mirrors/cm6/state/src/state.ts:48`
Helpers read: `mirrors/cm6/state/src/state.ts:123,250-251`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.selection.B1 | returns the current selection | `mirrors/cm6/state/src/state.ts:48` | `crates/ferrum-state/src/state.rs:143-145` | test-state#01, test-state#03 | test_state#001, test_state#003 | implemented+tested |
| state.EditorState.selection.B2 | invariant: every range lies inside the document (enforced by `checkSelection` in `create` and in the `Transaction` constructor) | `mirrors/cm6/state/src/state.ts:250`, `mirrors/cm6/state/src/transaction.ts:185` | none | none | none | differs: Ferrum state can hold `10:10` on a 3-char doc (R1, R32) |
| state.EditorState.selection.B3 | invariant: a single range unless `allowMultipleSelections` | `mirrors/cm6/state/src/state.ts:123,251` | none | test-state#17 | test_state#017 (asserts `<= 2`) | differs (R2, R3) |
Proposed status: PORTED, DEVIATION because B2, B3 differ.

### state.cm6.008 `EditorState.field` — cm6 `mirrors/cm6/state/src/state.ts:66`
Helpers read: `mirrors/cm6/state/src/facet.ts` `ensureAddr`, `getAddr`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.field.B1 | present field → its value | `mirrors/cm6/state/src/state.ts:69-75` | `crates/ferrum-state/src/state.rs:157-162`, `crates/ferrum-state/src/config.rs:192-202` | test-state#08, test-state#09 | test_state#008, test_state#009 | implemented+tested |
| state.EditorState.field.B2 | absent field with `require` (default) → RangeError "Field is not present in this state" (R23) | `mirrors/cm6/state/src/state.ts:71` | none: returns `None` | test-state#11 (third assertion uses `false`) | none | differs: no panicking variant; `None` matches only the `require: false` form |
| state.EditorState.field.B3 | absent field with `require: false` → `undefined` (R23) | `mirrors/cm6/state/src/state.ts:72` | `crates/ferrum-state/src/config.rs:197` (`?` → `None`) | test-state#11 | none | implemented, untested |
| state.EditorState.field.B4 | value computed lazily via `ensureAddr`, so `field()` called inside another field's `create` returns that field's value regardless of order (R33, R33b) | `mirrors/cm6/state/src/state.ts:74` | none: `crates/ferrum-state/src/state.rs:121-125` initializes fields while `state.field_values` is still empty, so `field()` inside an init returns `None` | test-state#08 | test_state#008 (masks with `unwrap_or(0)`) | differs: f1=5, f2 = f1+10 → cm6 15, Ferrum `None` (R33) |
Proposed status: PORTED, DEVIATION because B2 and B4 differ, B3 untested.

### state.cm6.009 `EditorState.update` — cm6 `mirrors/cm6/state/src/state.ts:91`
Helpers read: pipeline helpers (all of `mirrors/cm6/state/src/transaction.ts:290-407`), `mirrors/cm6/state/src/change.ts` `ChangeSet.of`/`filter`/`map`/`mapDesc`/`compose`/`invertedDesc` (C2), `mirrors/cm6/state/src/selection.ts:239`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.update.B1 | takes any number of specs; zero specs → an empty transaction (R14e) | `mirrors/cm6/state/src/state.ts:91`, `mirrors/cm6/state/src/transaction.ts:344` | `crates/ferrum-state/src/state.rs:257` takes exactly one `TransactionSpec` | none | none | differs |
| state.EditorState.update.B2 | first spec's `changes` resolved against the current doc length; string inserts split on `lineSeparator`/`DefaultSplit` | `mirrors/cm6/state/src/transaction.ts:334-335,344` | `crates/ferrum-state/src/state.rs:258-261` (`to_changeset(len)`; inserts are `Text`, no split) | test-state#02 | test_state#002 (builds the `ChangeSet` itself) | differs for `\r`-containing inserts (D2, see state.EditorState.changes.B4); ASCII/`\n` implemented+tested |
| state.EditorState.update.B3 | `selection` as `{anchor, head?}` → `EditorSelection.single` | `mirrors/cm6/state/src/transaction.ts:336` | none | test-state#20, test-state#24 | none | missing |
| state.EditorState.update.B4 | `userEvent` → `Transaction.userEvent` annotation appended after the spec's annotations (R10b) | `mirrors/cm6/state/src/transaction.ts:332` | none | none | none | missing |
| state.EditorState.update.B5 | `scrollIntoView` → `!!spec.scrollIntoView` | `mirrors/cm6/state/src/transaction.ts:339` | none | test-state#27 | none | missing |
| state.EditorState.update.B6 | non-sequential merge: later spec's changes refer to the start doc; `mapForA = b.changes.map(a.changes)`, `changes = a.compose(mapForA)` (R14 → "axbyz") | `mirrors/cm6/state/src/transaction.ts:316-319,346-349` | none | none | none | missing |
| state.EditorState.update.B7 | `sequential: true`: spec resolved against `s.changes.newLength` and composed (R14b → "abxyz") | `mirrors/cm6/state/src/transaction.ts:312-315,348-349` | none | test-state#25 | none | missing |
| state.EditorState.update.B8 | merged selection: later spec's selection wins (mapped by `mapForB`), else earlier one mapped through `mapForA` (R14c, R46 → `3:3`) | `mirrors/cm6/state/src/transaction.ts:323` | none | none | none | missing |
| state.EditorState.update.B9 | merged effects: `mapEffects(a.effects, mapForA)` ++ `mapEffects(b.effects, mapForB)` | `mirrors/cm6/state/src/transaction.ts:324` | none | none | none | missing |
| state.EditorState.update.B10 | merged annotations concatenated (a then b, R45 → first wins), `scrollIntoView` OR-ed (R14d) | `mirrors/cm6/state/src/transaction.ts:325-326` | none | none | none | missing |
| state.EditorState.update.B11 | `filter: false` on any spec disables change/transaction filters (R12h) | `mirrors/cm6/state/src/transaction.ts:345-347` | none | test-state#23, test-state#26 | none | missing |
| state.EditorState.update.B12 | `Transaction` constructor runs `checkSelection(selection, changes.newLength)` → RangeError (R32) | `mirrors/cm6/state/src/transaction.ts:185` | none | none | none | missing: Ferrum accepts `cursor(10)` on "abc" (R32) |
| state.EditorState.update.B13 | `Transaction.time` annotation with `Date.now()` appended unless a time annotation is present (R7b, R8) | `mirrors/cm6/state/src/transaction.ts:186-187` | `crates/ferrum-state/src/state.rs:277-284` | none | none in ferrum-state (ferrum-commands `crates/ferrum-commands/src/tests/test_history.rs:34` supplies an explicit time) | implemented, untested |
| state.EditorState.update.B14 | order: change filters → transaction filters → extenders (R12e "cf,tf,ext") | `mirrors/cm6/state/src/transaction.ts:352,356-401` | `crates/ferrum-state/src/state.rs:296-302` boolean transaction filters only | test-state#20..#27 | none equivalent | differs |
| state.EditorState.update.B15 | change filters iterated in facet order; `false` stops with result false; arrays are unioned with `joinRanges` (R11, R11c) | `mirrors/cm6/state/src/transaction.ts:290-300,360-365` | none | test-state#20, test-state#22 | test_state#020, test_state#022 (test a boolean transaction filter instead) | missing |
| state.EditorState.update.B16 | change-filter result `false` → changes emptied, explicit selection and effects mapped back through `invertedDesc` (R11 head 7, R11e) | `mirrors/cm6/state/src/transaction.ts:366-370,376-378` | none | test-state#20 | none | missing |
| state.EditorState.update.B17 | change-filter ranges → `tr.changes.filter(ranges)`; selection/effects mapped back through the filtered part (R11b "et") | `mirrors/cm6/state/src/transaction.ts:371-375` | none | test-state#21 | none | missing |
| state.EditorState.update.B18 | transaction filters run from last to first (lowest precedence first) (R12d "BA") | `mirrors/cm6/state/src/transaction.ts:383` | `crates/ferrum-state/src/state.rs:298` iterates first to last (R10b "AB") | none | none | differs |
| state.EditorState.update.B19 | transaction filter returns a `Transaction` / `[Transaction]` (used directly) or spec(s) resolved with `filter=false` (R12, R12b, R12c, R43) | `mirrors/cm6/state/src/transaction.ts:384-387` | `crates/ferrum-state/src/state.rs:299-301`: `bool`; `false` → whole `update` returns `None` | test-state#24, test-state#25 | test_state#024, test_state#025 (no filter) | differs |
| state.EditorState.update.B20 | extenders run from last to first, also when `filter: false` (R12f, R13, R13b "2,1") | `mirrors/cm6/state/src/transaction.ts:352,393-395` | none | test-state#26, test-state#27 | none | missing |
| state.EditorState.update.B21 | extender result null/empty skipped; otherwise only its effects/annotations are merged sequentially (changes/selection kept from `tr`) | `mirrors/cm6/state/src/transaction.ts:396-400` | none | test-state#26 | none | missing |
| state.EditorState.update.B22 | when a transaction filter returns specs, the recursive `resolveTransaction` also extends, so extenders apply twice (R12g → 2 effects) | `mirrors/cm6/state/src/transaction.ts:387 → :352` | none | none | none | missing (cm6 quirk; record only) |
| state.EditorState.update.B23 | "did nothing": always returns a `Transaction`; a filter can only reduce it (R12c: `{}` → empty transaction) | `mirrors/cm6/state/src/state.ts:91` | `crates/ferrum-state/src/state.rs:257` returns `Option<Transaction>`; `None` when a filter rejects (R10) | none | test_state#020..#024 assert `None` | differs |
| state.EditorState.update.B24 | no explicit selection → `tr.selection` stays `undefined` even when the doc changes (R5) | `mirrors/cm6/state/src/transaction.ts:176,351` | `crates/ferrum-state/src/state.rs:269-275` stores the mapped selection in `selection` | none | none | differs |
| state.EditorState.update.B25 | new doc not computed by `update` (lazy `newDoc`) | `mirrors/cm6/state/src/transaction.ts:203-205` | `crates/ferrum-state/src/state.rs:262-267` applies changes eagerly, also for transactions a filter then rejects | none | none | differs (cost only) |
| state.EditorState.update.B26 | invalid change bounds throw RangeError "Invalid change range F to T (in doc of length N)" (R31) | `mirrors/cm6/state/src/change.ts` `ChangeSet.of` (C2) | `crates/ferrum-state/src/change.rs:513-518` panics "Invalid change range 2..1 for document of length 4"; negative `from` unrepresentable (`usize`) | test-state#05 (3 cases) | test_state#005 (1 case, no message check) | implemented (panic, undocumented `# Panics`), partially tested |
Proposed status: PORTED, DEVIATION because B1, B3–B12, B14–B25 are missing or differ; B13 untested.

### state.cm6.010 `EditorState.replaceSelection` — cm6 `mirrors/cm6/state/src/state.ts:129`
Helpers read: `mirrors/cm6/state/src/state.ts:130-132,144-186`, `mirrors/cm6/state/src/selection.ts` `EditorSelection.cursor`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.replaceSelection.B1 | a string is converted with `toText` (lineSeparator/DefaultSplit) (R16, R16b) | `mirrors/cm6/state/src/state.ts:130` | `crates/ferrum-state/src/state.rs:245-246` `impl Into<Text>` (ropey, raw separators) | none | none | differs (D2): "a\r\nb" into "xy" → cm6 doc "a\nbxy" len 5 cursor 3; Ferrum "a\r\nbxy" len 6 cursor 4 |
| state.EditorState.replaceSelection.B2 | a `Text` is used directly | `mirrors/cm6/state/src/state.ts:129` | `crates/ferrum-state/src/state.rs:246` | none | none | implemented, untested |
| state.EditorState.replaceSelection.B3 | every range `from..to` replaced by the text (R16c) | `mirrors/cm6/state/src/state.ts:131` | `crates/ferrum-state/src/state.rs:248-251` | test-state#03 | none (test_state#003 uses `change_by_range` directly) | implemented, untested in ferrum-state (oracle: 0 mismatches on 3000 ASCII cases incl. `rs`) |
| state.EditorState.replaceSelection.B4 | resulting cursor at `range.from + text.length` with `assoc = -1` (R16, R42) | `mirrors/cm6/state/src/state.ts:132` | `crates/ferrum-state/src/state.rs:249` `SelectionRange::cursor` → `assoc = 0` | none | none | differs: assoc −1 vs 0 (R42) |
| state.EditorState.replaceSelection.B5 | text length counted in UTF-16 units (R16d: "😀" → cursor 3) | `mirrors/cm6/state/src/state.ts:132` | `crates/ferrum-state/src/state.rs:247` `len_chars` (code points → cursor 2) | none | none | differs (D1) |
| state.EditorState.replaceSelection.B6 | returns a spec with changes, selection, effects (no annotations, no userEvent) | `mirrors/cm6/state/src/state.ts:131` | `crates/ferrum-state/src/state.rs:252` `into_spec` | test-state#03 | none | implemented, untested |
Proposed status: PORTED, DEVIATION because B1, B4, B5 differ and B2, B3, B6 are untested in ferrum-state.

### state.cm6.011 `EditorState.changeByRange` — cm6 `mirrors/cm6/state/src/state.ts:144`
Helpers read: `mirrors/cm6/state/src/state.ts:176-179`, `mirrors/cm6/state/src/transaction.ts:98-106,405`, `mirrors/cm6/state/src/change.ts` `map`, `mapDesc`, `compose` (C2), `mirrors/cm6/state/src/selection.ts:73` (`SelectionRange.map`, default assoc −1), `mirrors/cm6/state/src/selection.ts:188` (`EditorSelection.create`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.changeByRange.B1 | `f` called once per range, in range order, starting with range 0 | `mirrors/cm6/state/src/state.ts:154,157-158` | `crates/ferrum-state/src/state.rs:213,221-222` | test-state#18, test-state#19 | test_state#018, test_state#019 | implemented+tested |
| state.EditorState.changeByRange.B2 | each result's `changes` goes through `this.changes()` (any ChangeSpec form, lineSeparator) | `mirrors/cm6/state/src/state.ts:155,159` | `crates/ferrum-state/src/state.rs:214-217,223-226` (`Changes` enum, `to_changeset`) | test-state#18 | test_state#018 | differs: no nested arrays, no lineSeparator (see state.EditorState.changes.B3/B4) |
| state.EditorState.changeByRange.B3 | later result: `newMapped = newChanges.map(changes)`; earlier ranges mapped through `newMapped`; new range mapped through `changes.mapDesc(newChanges, true)`; `changes = changes.compose(newMapped)` (R17c, R17d) | `mirrors/cm6/state/src/state.ts:159-163` | `crates/ferrum-state/src/state.rs:227-233` | test-state#19 | test_state#019 (doc only) | implemented+tested (doc); selection result untested in Ferrum but matches (R17c) and oracle 0/3000 mismatches |
| state.EditorState.changeByRange.B4 | effects: accumulated effects mapped through `newMapped`, new effects mapped through `mapBy` (`StateEffect.mapEffects`) (R17: cm6 [2,6]) | `mirrors/cm6/state/src/state.ts:164` | `crates/ferrum-state/src/state.rs:234` concatenates without mapping (Ferrum [2,4]) | none | none | differs (no `StateEffect.map` exists) |
| state.EditorState.changeByRange.B5 | selection = `EditorSelection.create(ranges, sel.mainIndex)` (R17c main 2) | `mirrors/cm6/state/src/state.ts:168` | `crates/ferrum-state/src/state.rs:239` | test-state#18 | test_state#018 | implemented+tested (main index preservation untested; R17c matches) |
| state.EditorState.changeByRange.B6 | `effects` may be a single effect or an array | `mirrors/cm6/state/src/state.ts:147,156` | `crates/ferrum-state/src/state.rs:34` `Vec<StateEffect>` | none | none | implemented (Vec), untested |
| state.EditorState.changeByRange.B7 | positions in callbacks are UTF-16 offsets | `mirrors/cm6/state/src/state.ts:144` | code points | none | none | differs (D1); unit-converted non-ASCII oracle (rs, cbr3) 0/500 mismatches |
Proposed status: PORTED, DEVIATION because B2, B4 differ and B7 depends on D1.

### state.cm6.012 `EditorState.changes` — cm6 `mirrors/cm6/state/src/state.ts:176`
Helpers read: `mirrors/cm6/state/src/change.ts` `ChangeSet.of` (C2), `mirrors/cm6/state/src/change.ts:335`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.changes.B1 | a `ChangeSet` is returned unchanged (same object, R18c) | `mirrors/cm6/state/src/state.ts:177` | none: `crates/ferrum-state/src/state.rs:201` takes a `ChangeSpec` | none | none | missing (only `Changes::Set` in `update`/`RangeResult`) |
| state.EditorState.changes.B2 | default `[]` → empty set of doc length (R18 `[3]`) | `mirrors/cm6/state/src/state.ts:176` | none (argument required) | none | none | missing |
| state.EditorState.changes.B3 | arrays and nested arrays of specs (R18) | `mirrors/cm6/state/src/state.ts:178` | none for `changes()`; `ChangeSet::of_specs` (`crates/ferrum-state/src/change.rs:484`) is flat | none | none | missing |
| state.EditorState.changes.B4 | string inserts split on `lineSeparator`/DefaultSplit (R18: "a\r\nb" → length 3; R18b) | `mirrors/cm6/state/src/state.ts:178`, `mirrors/cm6/state/src/change.ts:335` | `crates/ferrum-state/src/state.rs:202` (insert is `Text`; "a\r\nb" → length 4, R18) | none | none | differs (D2) |
| state.EditorState.changes.B5 | validated against `doc.length` (R31) | `mirrors/cm6/state/src/state.ts:178` | `crates/ferrum-state/src/state.rs:202`, `crates/ferrum-state/src/change.rs:513-518` | test-state#05 | test_state#005 | implemented (panic), partially tested |
Proposed status: PORTED, DEVIATION because B1–B4 are missing or differ.

### state.cm6.013 `EditorState.toText` — cm6 `mirrors/cm6/state/src/state.ts:184`
Helpers read: `mirrors/cm6/state/src/change.ts:3`, `Text.of` (C1).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.toText.B1 | splits on the configured `lineSeparator` (R19b: only `\r\n`, 2 lines) | `mirrors/cm6/state/src/state.ts:185` | none | test-state#07 | none | missing |
| state.EditorState.toText.B2 | without separator splits on `\r\n?|\n` (R19: 4 lines, length 7) | `mirrors/cm6/state/src/state.ts:185` | none | test-state#07 | none | missing |
Proposed status: MISSING (searched `to_text`, `toText` in all crates; nearest is `Text::from`, which ignores the facet).

### state.cm6.014 `EditorState.sliceDoc` — cm6 `mirrors/cm6/state/src/state.ts:189`
Helpers read: `mirrors/cm6/state/src/text.ts` `sliceString(from, to, lineSep)` (C1), `mirrors/cm6/state/src/state.ts:286`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.sliceDoc.B1 | defaults `from = 0`, `to = doc.length` | `mirrors/cm6/state/src/state.ts:189` | none | none | none | missing |
| state.EditorState.sliceDoc.B2 | lines joined with `lineBreak` (R20 "a\nb\nc"; R20b "a\r\nb\nc") | `mirrors/cm6/state/src/state.ts:190` | none (`Text::slice_string` returns raw stored separators, R20) | none | none | missing |
Proposed status: MISSING (searched `slice_doc`, `sliceDoc`).

### state.cm6.015 `EditorState.facet` — cm6 `mirrors/cm6/state/src/state.ts:194`
Helpers read: `mirrors/cm6/state/src/facet.ts` `ensureAddr`, `getAddr`, `Facet.default`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.facet.B1 | facet not in config → `facet.default` | `mirrors/cm6/state/src/state.ts:196` | `crates/ferrum-state/src/config.rs:187-188` | test-facet#04, test-state#07 | test_facet#004, test_state#007 | implemented+tested |
| state.EditorState.facet.B2 | facet present → combined value | `mirrors/cm6/state/src/state.ts:197-198` | `crates/ferrum-state/src/config.rs:173-185` | test-state#06, test-state#12 | test_state#006, test_state#012 | implemented+tested |
| state.EditorState.facet.B3 | dynamic values computed on demand, so a dynamic facet (or a field init) may read another dynamic facet in any order (R33d, R33e) | `mirrors/cm6/state/src/state.ts:197` | none: `crates/ferrum-state/src/state.rs:128-132` computes slots with `state.dynamic_values` empty → `config.rs:175` index panic | test-facet#09 (C4) | none | differs: panic "index out of bounds: the len is 0 but the index is 1" (R33d) |
| state.EditorState.facet.B4 | unchanged inputs keep the same output object across transactions (R34 `true`) | `mirrors/cm6/state/src/state.ts:197` + C4 `compare` | `crates/ferrum-state/src/state.rs:357-361` recomputes every dynamic slot per state (R34 `ptr_eq` false) | test-state#16 | test_state#016 (no identity check) | differs (C4 owns `compare`) |
Proposed status: PORTED, DEVIATION because B3, B4 differ.

### state.cm6.016 `EditorState.toJSON` — cm6 `mirrors/cm6/state/src/state.ts:205`
Helpers read: `mirrors/cm6/state/src/state.ts:189-191`, `mirrors/cm6/state/src/selection.ts:170` (`EditorSelection.toJSON`), `StateField.spec.toJSON` (C4).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.toJSON.B1 | `doc` = `sliceDoc()` (joined with `lineBreak`) | `mirrors/cm6/state/src/state.ts:207` | none | test-state#10 | none | missing |
| state.EditorState.toJSON.B2 | `selection` = `selection.toJSON()` (`{ranges:[{anchor,head}], main}`) | `mirrors/cm6/state/src/state.ts:208` | none | test-state#10 | none | missing |
| state.EditorState.toJSON.B3 | for each `prop → field`, only `StateField`s present in the state are serialized with `spec.toJSON(value, state)` (R22) | `mirrors/cm6/state/src/state.ts:210-214` | none | test-state#10 | test_state#010 (asserts only `doc()`) | missing |
Proposed status: MISSING (searched `to_json`, `toJSON`, `serde` in all crates; only test names match).

### state.cm6.017 `EditorState^fromJSON` — cm6 `mirrors/cm6/state/src/state.ts:222`
Helpers read: `mirrors/cm6/state/src/state.ts:243-254`, `mirrors/cm6/state/src/selection.ts:175` (`EditorSelection.fromJSON`), `StateField.init` (C4).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.fromJSON.B1 | non-object or non-string `doc` → RangeError "Invalid JSON representation for EditorState" (R22b) | `mirrors/cm6/state/src/state.ts:223-224` | none | none | none | missing |
| state.EditorState.fromJSON.B2 | fields whose prop is an own property of `json` get `field.init(state => spec.fromJSON(value, state))`, placed before `config.extensions` | `mirrors/cm6/state/src/state.ts:225-236` | none | test-state#10 | none | missing |
| state.EditorState.fromJSON.B3 | selection via `EditorSelection.fromJSON`; doc string split on the configured lineSeparator (R22d) | `mirrors/cm6/state/src/state.ts:234-235` | none | test-state#10 | none | missing |
Proposed status: MISSING (searched `from_json`, `fromJSON`).

### state.cm6.018 `EditorState^create` — cm6 `mirrors/cm6/state/src/state.ts:243`
Helpers read: `mirrors/cm6/state/src/state.ts:42-61`, `mirrors/cm6/state/src/selection.ts:151,182,239`, `mirrors/cm6/state/src/change.ts:3`, `Configuration.resolve`, `ensureAddr`, `StateField` slot `create` (C4).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.create.B1 | `{}` → empty doc, cursor 0, no extensions | `mirrors/cm6/state/src/state.ts:243-247` | `crates/ferrum-state/src/state.rs:106-110` | test-state#01, test-state#06 | test_state#001, test_state#006 | implemented+tested |
| state.EditorState.create.B2 | string doc split on static lineSeparator / DefaultSplit | `mirrors/cm6/state/src/state.ts:246` | none (see state.EditorStateConfig.doc.B1) | none | none | differs (D2, R21, R21c) |
| state.EditorState.create.B3 | `{anchor, head}` selection form | `mirrors/cm6/state/src/state.ts:249` | none | none | none | missing (R1b) |
| state.EditorState.create.B4 | `checkSelection` → RangeError (R1) | `mirrors/cm6/state/src/state.ts:250` | none | none | none | missing |
| state.EditorState.create.B5 | `asSingle()` unless static `allowMultipleSelections` (R2: cm6 `1:1 main=0`, Ferrum `0:0,1:1 main=1`) | `mirrors/cm6/state/src/state.ts:251` | none | test-state#17 | test_state#017 (asserts `<= 2`) | missing |
| state.EditorState.create.B6 | extensions resolved with empty compartment map | `mirrors/cm6/state/src/state.ts:244` | `crates/ferrum-state/src/state.rs:110` | test-state#12 | test_state#012 | implemented+tested |
| state.EditorState.create.B7 | every slot (field `create`, dynamic facet) evaluated through `ensureAddr` in dependency order (R33, R33b, R33d, R33e) | `mirrors/cm6/state/src/state.ts:59`, `mirrors/cm6/state/src/facet.ts` `ensureAddr` | `crates/ferrum-state/src/state.rs:120-132`: fields first with empty `field_values`, then dynamic facets with empty `dynamic_values` | test-state#08, test-state#16 | test_state#008 (masked), test_state#016 (no field-derived facet) | differs: R33 `None` vs 15; R33d/R33e panic |
| state.EditorState.create.B8 | returns an `EditorState` | `mirrors/cm6/state/src/state.ts:252` | returns `Arc<EditorState>` (`crates/ferrum-state/src/state.rs:105,134`) | all | all | implemented (Arc wrapper; see D6 evidence) |
Proposed status: PORTED, DEVIATION because B2–B5, B7 are missing or differ.

### state.cm6.019 `EditorState^allowMultipleSelections` — cm6 `mirrors/cm6/state/src/state.ts:262` (definition `mirrors/cm6/state/src/extension.ts:12`)
Helpers read: `mirrors/cm6/state/src/extension.ts:12-15`, `mirrors/cm6/state/src/state.ts:123,251`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^allowMultipleSelections.B1 | combine: `values.some(v => v)`; default false (R25 true) | `mirrors/cm6/state/src/extension.ts:13` | `crates/ferrum-state/src/state.rs:13-15` | test-state#03 (single `true`) | test_state#003 (single `true`) | implemented; combine of several values untested (R25 matches) |
| state.EditorState^allowMultipleSelections.B2 | static facet | `mirrors/cm6/state/src/extension.ts:14` | `crates/ferrum-state/src/state.rs:14` (`define_static`) | none | none | implemented, untested |
| state.EditorState^allowMultipleSelections.B3 | consulted by `create` (`staticFacet`) and `applyTransaction` (start state's value) to collapse selections | `mirrors/cm6/state/src/state.ts:123,251` | none (grep: `ALLOW_MULTIPLE_SELECTIONS` only read by its accessor) | test-state#17 | test_state#017 (weakened) | missing (R2, R3) |
Proposed status: PORTED, DEVIATION because B3 is missing, B2 untested.

### state.cm6.020 `EditorState^tabSize` — cm6 `mirrors/cm6/state/src/state.ts:267`
Helpers read: `mirrors/cm6/state/src/state.ts:267-269`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^tabSize.B1 | default 4 | `mirrors/cm6/state/src/state.ts:268` | `crates/ferrum-state/src/state.rs:18` | test-state#06 | test_state#006 | implemented+tested |
| state.EditorState^tabSize.B2 | first (highest-precedence) value wins (R26 → 2) | `mirrors/cm6/state/src/state.ts:268` | `crates/ferrum-state/src/state.rs:18` (`first()`) | test-state#06 (single value) | test_state#006 (single value) | implemented; multi-value untested (R26 matches) |
| state.EditorState^tabSize.B3 | non-static (may be computed / reconfigured; R15 reconfigure → 8) | `mirrors/cm6/state/src/state.ts:267` | `crates/ferrum-state/src/state.rs:17` (`define`, non-static) | test-state#06 | none (reconfigure part dropped) | implemented, untested; reconfigure via `StateEffect.reconfigure` missing |
Proposed status: PORTED, DEVIATION because B2, B3 are untested (value type `usize` vs `number`).

### state.cm6.021 `EditorState.tabSize` (getter) — cm6 `mirrors/cm6/state/src/state.ts:273`
Helpers read: `mirrors/cm6/state/src/state.ts:194-199`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.tabSize.B1 | returns `facet(EditorState.tabSize)` | `mirrors/cm6/state/src/state.ts:273` | none as a getter; same value via `*state.facet(EditorState::tab_size())` (`crates/ferrum-state/src/state.rs:170`, note: Ferrum's `tab_size()` is the facet accessor, not the getter) | test-state#06 | test_state#006 (uses the facet expression) | missing (getter) |
Proposed status: MISSING (searched `fn tab_size(&self` and `tab_size` in all crates; `ferrum-commands/src/commands.rs`, `ferrum-language/src/indent.rs` read the facet directly).

### state.cm6.022 `EditorState^lineSeparator` — cm6 `mirrors/cm6/state/src/state.ts:282` (definition `mirrors/cm6/state/src/extension.ts:17`)
Helpers read: `mirrors/cm6/state/src/extension.ts:17-20`; consumers `mirrors/cm6/state/src/state.ts:178,185,246,286`, `mirrors/cm6/state/src/transaction.ts:335`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^lineSeparator.B1 | combine: first value or `undefined` | `mirrors/cm6/state/src/extension.ts:18` | `crates/ferrum-state/src/state.rs:25-27` (`Option<String>`) | test-state#07 | test_state#007 | implemented+tested |
| state.EditorState^lineSeparator.B2 | static facet | `mirrors/cm6/state/src/extension.ts:19` | `crates/ferrum-state/src/state.rs:26` (`define_static`) | none | none | implemented, untested |
| state.EditorState^lineSeparator.B3 | consulted by `create` (doc split), `toText`, `changes`, `resolveTransactionInner`, `lineBreak`/`sliceDoc` | `mirrors/cm6/state/src/state.ts:178,185,246,286`, `mirrors/cm6/state/src/transaction.ts:335` | none: `LINE_SEPARATOR` is read nowhere except its accessor (`crates/ferrum-state/src/state.rs:185-187`; grep in all crates: only tests) | test-state#07 (toText part) | none | missing (facet has no effect) |
| state.EditorState^lineSeparator.B4 | changeable by `StateEffect.reconfigure` (test-state#07 last assertion) | `mirrors/cm6/state/src/state.ts:106-108` | none | test-state#07 | none | missing |
Proposed status: PORTED, DEVIATION because B3, B4 missing, B2 untested (D2-dependent).

### state.cm6.023 `EditorState.lineBreak` — cm6 `mirrors/cm6/state/src/state.ts:286`
Helpers read: `mirrors/cm6/state/src/state.ts:194-199`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.lineBreak.B1 | facet value, or `"\n"` when unset (R20c) | `mirrors/cm6/state/src/state.ts:286` | none | none | none | missing |
Proposed status: MISSING (searched `line_break`, `lineBreak`; only `ferrum-view` heightmap `widget_line_breaks`, unrelated).

### state.cm6.024 `EditorState^readOnly` — cm6 `mirrors/cm6/state/src/state.ts:299` (definition `mirrors/cm6/state/src/extension.ts:29`)
Helpers read: `mirrors/cm6/state/src/extension.ts:29-31`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^readOnly.B1 | combine: first (highest-precedence) value, default false (R24: `[of(false), of(true)]` → false) | `mirrors/cm6/state/src/extension.ts:30` | `crates/ferrum-state/src/state.rs:21-23` `any()` → true | none | none | differs |
| state.EditorState^readOnly.B2 | non-static | `mirrors/cm6/state/src/extension.ts:29` | `crates/ferrum-state/src/state.rs:22` (`define`) | none | none | implemented, untested |
Proposed status: PORTED, DEVIATION because B1 differs, B2 untested.

### state.cm6.025 `EditorState.readOnly` (getter) — cm6 `mirrors/cm6/state/src/state.ts:303`
Helpers read: `mirrors/cm6/state/src/extension.ts:29-31`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.readOnly.B1 | returns `facet(readOnly)` | `mirrors/cm6/state/src/state.ts:303` | `crates/ferrum-state/src/state.rs:180-182` (`is_read_only`) | none | none in ferrum-state (consumed in `crates/ferrum-commands/src/commands.rs`) | implemented, untested; inherits state.EditorState^readOnly.B1 |
Proposed status: PORTED, DEVIATION because the combined value differs (R24) and it is untested.

### state.cm6.026 `EditorState^phrases` — cm6 `mirrors/cm6/state/src/state.ts:309`
Helpers read: `mirrors/cm6/state/src/state.ts:309-314`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^phrases.B1 | facet of `{[key]: string}` objects, all values kept | `mirrors/cm6/state/src/state.ts:309` | none | none | none | missing |
| state.EditorState^phrases.B2 | `compare`: same key count and equal values (shallow), so recomputed equal objects do not count as changes | `mirrors/cm6/state/src/state.ts:310-313` | none | none | none | missing |
Proposed status: MISSING (searched `phrase`, `phrases`).

### state.cm6.027 `EditorState.phrase` — cm6 `mirrors/cm6/state/src/state.ts:324`
Helpers read: `mirrors/cm6/state/src/state.ts:309-333`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.phrase.B1 | first map (facet order) with an own property for the phrase wins (R27 "Hallo …", not "second") | `mirrors/cm6/state/src/state.ts:325-326` | none | none | none | missing |
| state.EditorState.phrase.B2 | no translation → original string (R27 "Missing") | `mirrors/cm6/state/src/state.ts:332` | none | none | none | missing |
| state.EditorState.phrase.B3 | with insert args: `$n` → `insert[n-1]`, `$` → `$1`, `$$` → `$`, `$0`/out of range left literally (R27 "Hallo x y $ $0 $3", "a Z b") | `mirrors/cm6/state/src/state.ts:327-331` | none | none | none | missing |
| state.EditorState.phrase.B4 | without insert args no substitution (R27 "Hallo $1 $2 $$ $0 $3") | `mirrors/cm6/state/src/state.ts:327` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.028 `EditorState^languageData` — cm6 `mirrors/cm6/state/src/state.ts:337` (definition `mirrors/cm6/state/src/extension.ts:5`)
Helpers read: `mirrors/cm6/state/src/extension.ts:5`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^languageData.B1 | facet of providers `(state, pos, side) => object[]` | `mirrors/cm6/state/src/extension.ts:5` | none | none | none | missing |
Proposed status: MISSING (searched `language_data`, `languageData`, `LanguageData` in all crates).

### state.cm6.029 `EditorState.languageDataAt` — cm6 `mirrors/cm6/state/src/state.ts:353`
Helpers read: `mirrors/cm6/state/src/extension.ts:5`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.languageDataAt.B1 | every provider (facet order) is called; each result object with an own property `name` contributes its value, in order (R28 `["-","$"]`) | `mirrors/cm6/state/src/state.ts:355-359` | none | none | none | missing |
| state.EditorState.languageDataAt.B2 | `side` defaults to −1 and is passed to providers (R28 `[-1]`, `[1]`) | `mirrors/cm6/state/src/state.ts:353,356` | none | none | none | missing |
| state.EditorState.languageDataAt.B3 | no match → `[]` | `mirrors/cm6/state/src/state.ts:361` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.030 `EditorState.charCategorizer` — cm6 `mirrors/cm6/state/src/state.ts:373`
Helpers read: `mirrors/cm6/state/src/charcategory.ts:13-35`, `mirrors/cm6/state/src/state.ts:353-362`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.charCategorizer.B1 | word chars = first `"wordChars"` language data at `at`, else "" (R29: `"-"` with wordChars "-" → Word) | `mirrors/cm6/state/src/state.ts:374-375` | `crates/ferrum-state/src/state.rs:190` ignores `_at` | none | none | differs: Ferrum `"-"` → Other |
| state.EditorState.charCategorizer.B2 | Space iff the string has no non-whitespace (`/\S/`), so `""` → Space (R29) | `mirrors/cm6/state/src/charcategory.ts:30` | `crates/ferrum-state/src/state.rs:194-196` `""` → Other | none | none | differs |
| state.EditorState.charCategorizer.B3 | Word iff any character of the string is `\p{Alphabetic}`/`\p{Number}`/`_` (R29 `"!a"` → Word) | `mirrors/cm6/state/src/charcategory.ts:18-19,31` | `crates/ferrum-state/src/state.rs:192-193` first char only → Other | test-charcategory#01 | test_charcategory#001 | differs for multi-char strings; single ASCII chars implemented+tested |
| state.EditorState.charCategorizer.B4 | Word iff the string contains a char of `wordChars` | `mirrors/cm6/state/src/charcategory.ts:32` | none | none | none | missing |
| state.EditorState.charCategorizer.B5 | otherwise Other | `mirrors/cm6/state/src/charcategory.ts:33` | `crates/ferrum-state/src/char.rs:17-18` | test-charcategory#03 | test_charcategory#003 | implemented+tested |
| state.EditorState.charCategorizer.B6 | JS `\s` whitespace class: U+0085 is not space (Other), U+FEFF is space (R29) | `mirrors/cm6/state/src/charcategory.ts:30` | `crates/ferrum-state/src/char.rs:13` `char::is_whitespace`: U+0085 Space, U+FEFF Other | test-charcategory#02 | test_charcategory#002 (ASCII space only) | differs (C7 owns `categorize_char`) |
Proposed status: PORTED, DEVIATION because B1, B2, B3 (multi-char), B4, B6 differ.

### state.cm6.031 `EditorState.wordAt` — cm6 `mirrors/cm6/state/src/state.ts:382`
Helpers read: `mirrors/cm6/state/src/char.ts` `findClusterBreak` (C7), `mirrors/cm6/state/src/state.ts:373-376`, `mirrors/cm6/state/src/text.ts` `lineAt` (C1).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState.wordAt.B1 | works within the line containing `pos` (R30: pos 15/16 on line 2) | `mirrors/cm6/state/src/state.ts:383` | none | none | none | missing |
| state.EditorState.wordAt.B2 | extends backward by grapheme clusters while the cluster is Word | `mirrors/cm6/state/src/state.ts:386-390` | none | none | none | missing |
| state.EditorState.wordAt.B3 | extends forward likewise | `mirrors/cm6/state/src/state.ts:391-395` | none | none | none | missing |
| state.EditorState.wordAt.B4 | no word char adjacent → `null` (R30 pos 8) | `mirrors/cm6/state/src/state.ts:396` | none | none | none | missing |
| state.EditorState.wordAt.B5 | uses `charCategorizer(pos)` (wordChars; R30b "baz-q" → 0-5) | `mirrors/cm6/state/src/state.ts:384` | none | none | none | missing |
| state.EditorState.wordAt.B6 | positions in UTF-16 (R30c "a𝐀b c" → 0-4) | `mirrors/cm6/state/src/state.ts:385-396` | none | none | none | missing (D1) |
Proposed status: MISSING (searched `word_at`, `wordAt`; `find_word_break` in `crates/ferrum-state/src/char.rs` is a different function).

### state.cm6.032 `EditorState^changeFilter` — cm6 `mirrors/cm6/state/src/state.ts:411` (definition `mirrors/cm6/state/src/extension.ts:22`)
Helpers read: `mirrors/cm6/state/src/transaction.ts:290-300,356-379`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^changeFilter.B1 | facet of `(tr) => boolean | number[]` | `mirrors/cm6/state/src/extension.ts:22` | none | test-state#20..#23 | none (test_state#020..#023 use `TRANSACTION_FILTER`) | missing |
| state.EditorState^changeFilter.B2 | `true` → no-op; `false` → drop all changes; ranges → suppress changes in ranges (see state.EditorState.update.B15–B17) | `mirrors/cm6/state/src/transaction.ts:360-379` | none | test-state#20, #21, #22 | none | missing |
| state.EditorState^changeFilter.B3 | skipped when `filter: false` | `mirrors/cm6/state/src/transaction.ts:352` | none | test-state#23 | none | missing |
Proposed status: MISSING (searched `change_filter`, `changeFilter`: only Ferrum test names).

### state.cm6.033 `EditorState^transactionFilter` — cm6 `mirrors/cm6/state/src/state.ts:430` (definition `mirrors/cm6/state/src/extension.ts:24`)
Helpers read: `mirrors/cm6/state/src/transaction.ts:381-389`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^transactionFilter.B1 | facet of `(tr) => TransactionSpec | TransactionSpec[]` | `mirrors/cm6/state/src/extension.ts:24` | `crates/ferrum-state/src/transaction.rs:222-226` `Fn(&Transaction) -> bool` | test-state#24, #25 | test_state#020..#024 | differs |
| state.EditorState^transactionFilter.B2 | returned `Transaction`/`[Transaction]` replaces `tr`; other specs are resolved (filter disabled) and can add changes/selection (R12, R12b, R43) | `mirrors/cm6/state/src/transaction.ts:385-387` | none (`false` → `None`) | test-state#24, #25 | none | differs |
| state.EditorState^transactionFilter.B3 | filters run from last to first (R12d) | `mirrors/cm6/state/src/transaction.ts:383` | `crates/ferrum-state/src/state.rs:298` first to last (R10b) | none | none | differs |
| state.EditorState^transactionFilter.B4 | skipped when `filter: false` | `mirrors/cm6/state/src/transaction.ts:352` | none (always run) | none | none | differs |
| state.EditorState^transactionFilter.B5 | run after change filters | `mirrors/cm6/state/src/transaction.ts:381` | n/a | none | none | missing |
Proposed status: PORTED, DEVIATION because B1–B5 differ (contract is a boolean veto, not a spec rewriter).

### state.cm6.034 `EditorState^transactionExtender` — cm6 `mirrors/cm6/state/src/state.ts:443` (definition `mirrors/cm6/state/src/extension.ts:26`)
Helpers read: `mirrors/cm6/state/src/transaction.ts:392-401`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorState^transactionExtender.B1 | facet of `(tr) => {effects?, annotations?} | null` | `mirrors/cm6/state/src/extension.ts:26-27` | none | test-state#26, #27 | none (test_state#026/#027 put the values in the spec) | missing |
| state.EditorState^transactionExtender.B2 | runs even with `filter: false`, after filters, last to first (R13, R13b, R12e, R12f) | `mirrors/cm6/state/src/transaction.ts:352,394` | none | test-state#26, #27 | none | missing |
| state.EditorState^transactionExtender.B3 | effects/annotations appended (sequential merge); changes and selection unchanged | `mirrors/cm6/state/src/transaction.ts:397-400` | none | test-state#27 | none | missing |
Proposed status: MISSING (searched `transaction_extender`, `transactionExtender`, `extender`).

### state.cm6.092 `Transaction` — cm6 `mirrors/cm6/state/src/transaction.ts:163`
Helpers read: `mirrors/cm6/state/src/transaction.ts:163-288`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.B1 | created only through `update`/filters (private constructor, internal `create`) | `mirrors/cm6/state/src/transaction.ts:169,191` | `crates/ferrum-state/src/transaction.rs:164-172` `pub(crate)` fields; created in `crates/ferrum-state/src/state.rs:286-294` | all | all | implemented+tested |
| state.Transaction.B2 | caches `_doc` and `_state` | `mirrors/cm6/state/src/transaction.ts:165-167` | `doc` stored eagerly; no state cache | test-state#16 (`tr.state` inside update) | none | differs (see 098, 100) |
| state.Transaction.B3 | member set: `startState`, `changes`, `selection`, `effects`, `scrollIntoView`, `newDoc`, `newSelection`, `state`, `annotation`, `docChanged`, `reconfigured`, `isUserEvent`, statics `time`/`userEvent`/`addToHistory`/`remote` | `mirrors/cm6/state/src/transaction.ts:171-287` | `crates/ferrum-state/src/transaction.rs:174-219` | — | — | differs: missing `scrollIntoView`, `newSelection`, `reconfigured`, `isUserEvent`, `remote` |
| state.Transaction.B4 | not clonable (one object per transaction) | `mirrors/cm6/state/src/transaction.ts:163` | `#[derive(Clone)]` `crates/ferrum-state/src/transaction.rs:163`; every clone/`state()` call recomputes a new state | none | none | differs (observable through state.Transaction.state.B1/B2) |
Proposed status: PORTED, DEVIATION because B2–B4 differ.

### state.cm6.093 `Transaction.startState` — cm6 `mirrors/cm6/state/src/transaction.ts:171`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.startState.B1 | the state `update` was called on (same object, R40) | `mirrors/cm6/state/src/transaction.ts:171` | `crates/ferrum-state/src/transaction.rs:181-183`, set at `crates/ferrum-state/src/state.rs:287` (R40 `ptr_eq` true) | test-state#21 (filter reads it) | none in ferrum-state (used by `crates/ferrum-commands/src/history.rs:494`) | implemented, untested |
Proposed status: PORTED, DEVIATION (untested: B1).

### state.cm6.094 `Transaction.changes` — cm6 `mirrors/cm6/state/src/transaction.ts:173`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.changes.B1 | the merged, filtered change set | `mirrors/cm6/state/src/transaction.ts:173,351,376` | `crates/ferrum-state/src/transaction.rs:186-188` (unfiltered, single spec) | test-state#20, #25 | none in ferrum-state | implemented for one unfiltered spec, untested |
Proposed status: PORTED, DEVIATION (untested: B1; filtering and merging missing upstream).

### state.cm6.095 `Transaction.selection` — cm6 `mirrors/cm6/state/src/transaction.ts:176`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.selection.B1 | the explicitly set selection, `undefined` otherwise, even when the doc changed (R5, R6) | `mirrors/cm6/state/src/transaction.ts:174-176` | `crates/ferrum-state/src/transaction.rs:191-193` + `crates/ferrum-state/src/state.rs:269-275` (mapped selection when doc changed) | test-state#24 | test_state#024 | differs: R5 cm6 `undefined`, Ferrum `Some("2:2")` |
| state.Transaction.selection.B2 | validated by `checkSelection` (R32) | `mirrors/cm6/state/src/transaction.ts:185` | none | none | none | missing |
| state.Transaction.selection.B3 | after change-filter suppression it is the explicit selection mapped back (R11e) | `mirrors/cm6/state/src/transaction.ts:376` | none | test-state#20 | none | missing |
Proposed status: PORTED, DEVIATION because B1 differs, B2, B3 missing.

### state.cm6.096 `Transaction.effects` — cm6 `mirrors/cm6/state/src/transaction.ts:178`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.effects.B1 | spec effects in order, then extender effects (last extender first, R13b "2,1") | `mirrors/cm6/state/src/transaction.ts:178,324,397` | `crates/ferrum-state/src/transaction.rs:196-198` (spec effects only) | test-state#27 | test_state#027 (asserts length 2 only) | implemented for spec effects, order untested; extender part missing |
| state.Transaction.effects.B2 | mapped back when a change filter suppresses changes (R11e eff [2]) | `mirrors/cm6/state/src/transaction.ts:377` | none | none | none | missing |
Proposed status: PORTED, DEVIATION (untested: order; B2 missing).

### state.cm6.097 `Transaction.scrollIntoView` — cm6 `mirrors/cm6/state/src/transaction.ts:183`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.scrollIntoView.B1 | true when any merged spec set it (R13b, R14d) | `mirrors/cm6/state/src/transaction.ts:183,326,339` | none | test-state#27 (sets it) | none | missing |
Proposed status: MISSING (searched `scroll_into_view`: only `ferrum-view/src/viewstate.rs:177` `ViewState::scroll_into_view(pos)`, a view method, not a transaction flag).

### state.cm6.098 `Transaction.newDoc` — cm6 `mirrors/cm6/state/src/transaction.ts:203`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.newDoc.B1 | `changes.apply(startState.doc)` | `mirrors/cm6/state/src/transaction.ts:204` | `crates/ferrum-state/src/transaction.rs:211-213` (`doc()`), computed at `crates/ferrum-state/src/state.rs:263-267` | test-state#02 (via state) | test_state#016, test_state#021, test_state#022 (filters/fields read `tr.doc()`) | implemented+tested |
| state.Transaction.newDoc.B2 | cached; equals `tr.state.doc` (R39) | `mirrors/cm6/state/src/transaction.ts:204` | stored once; moved into the new state (`crates/ferrum-state/src/state.rs:346`) | none | none | implemented, untested |
| state.Transaction.newDoc.B3 | lazy: not computed unless accessed | `mirrors/cm6/state/src/transaction.ts:197-205` | eager in `update` (also for rejected transactions) | none | none | differs (cost) |
Proposed status: PORTED, DEVIATION because B3 differs (no verifiable reason found for eager computation) and B2 untested.

### state.cm6.099 `Transaction.newSelection` — cm6 `mirrors/cm6/state/src/transaction.ts:211`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.newSelection.B1 | explicit selection when set | `mirrors/cm6/state/src/transaction.ts:212` | none as getter | none | none | missing |
| state.Transaction.newSelection.B2 | else start selection mapped through changes; same object when no changes (R6 `sameObj` true) | `mirrors/cm6/state/src/transaction.ts:212` | folded into `update` (`crates/ferrum-state/src/state.rs:269-275`) and `apply` (`:343-345`) | none | none | missing as API |
Proposed status: MISSING (searched `new_selection`: only `ferrum-commands`/`ferrum-view` local variables).

### state.cm6.100 `Transaction.state` — cm6 `mirrors/cm6/state/src/transaction.ts:219`
Helpers read: `mirrors/cm6/state/src/state.ts:42-61,96-125`; `StateField` slot `update`/`reconfigure`, `Configuration.resolve` (C4).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.state.B1 | computed on first access and retained; repeated access returns the same object (R4 `same: true`) | `mirrors/cm6/state/src/transaction.ts:219-222` | `crates/ferrum-state/src/transaction.rs:176-178` `start_state.apply(self.clone())` each call (R4 `ptr_eq` false) | test-state#02 | none (Ferrum tests call `state.apply(tr)`) | differs |
| state.Transaction.state.B2 | field update functions run once per transaction (R4 updates 1) | `mirrors/cm6/state/src/state.ts:124` | each call re-runs every field update (R4 → 2; R10c filter calling `tr.state()` + `apply` → 2) | none | none | differs |
| state.Transaction.state.B3 | re-entrant: `tr._state` is set before slots are computed, so a field update reading `tr.state` sees the new state (R33c `[2]`) | `mirrors/cm6/state/src/state.ts:58` | none: field update calling `tr.state()` recurses into `apply` → "thread 'main' has overflowed its stack" (repro `src/bin/reentrant.rs`) | test-state#16 | test_state#016 (uses `tr.doc()` instead) | differs (abort) |
| state.Transaction.state.B4 | `Compartment.reconfigure` effects replace compartment contents in a copied map | `mirrors/cm6/state/src/state.ts:99-105` | `crates/ferrum-state/src/state.rs:310-326` | test-state#12, #14 | test_state#012, test_state#014, test_facet#015, test_facet#018 | implemented+tested |
| state.Transaction.state.B5 | `StateEffect.reconfigure` replaces the base extensions (appended config discarded, compartments kept) | `mirrors/cm6/state/src/state.ts:106-108` | none | test-state#06, #07, #11, #14, #15 | none | missing |
| state.Transaction.state.B6 | `StateEffect.appendConfig` appends to the base (R15b) | `mirrors/cm6/state/src/state.ts:109-111` | none | test-facet#16, #18, #19, #21, #22 (C4) | none | missing |
| state.Transaction.state.B7 | on reconfiguration slots are reconfigured on an intermediate state (new config, old doc/selection) with access to the old state: new fields created against the new config (R35 "2|1+u") | `mirrors/cm6/state/src/state.ts:115-119` | `crates/ferrum-state/src/state.rs:323-326,338`: new field `init(self)` on the OLD state (R35 "1+u") | test-facet#18, #19 (C4) | test_facet#018 (init ignores state) | differs |
| state.Transaction.state.B8 | without reconfiguration values are copied and updated; unchanged dynamic facets keep identity (R34) | `mirrors/cm6/state/src/state.ts:121,124` | `crates/ferrum-state/src/state.rs:332-341,357-361` recompute every dynamic facet | test-state#16 | test_state#016 | differs (identity) |
| state.Transaction.state.B9 | selection collapsed with `asSingle` unless the START state allows multiple selections (R3) | `mirrors/cm6/state/src/state.ts:123` | `crates/ferrum-state/src/state.rs:343-345` no collapse | test-state#17 | test_state#017 (weakened) | differs |
| state.Transaction.state.B10 | new doc = `tr.newDoc` | `mirrors/cm6/state/src/state.ts:124` | `crates/ferrum-state/src/state.rs:346` | test-state#02 | test_state#002 | implemented+tested |
| state.Transaction.state.B11 | fields still present keep their value through reconfiguration; dropped fields disappear (R15e `[1,2,undefined]`) | `mirrors/cm6/state/src/facet.ts` slot `reconfigure` | `crates/ferrum-state/src/state.rs:333-336` keeps by id; removal only via compartments | test-state#11 | test_state#011 (no reconfiguration) | implemented for compartments, untested |
Proposed status: PORTED, DEVIATION because B1–B3, B5–B9 differ or are missing; B11 untested.

### state.cm6.101 `Transaction.annotation` — cm6 `mirrors/cm6/state/src/transaction.ts:225`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.annotation.B1 | returns the value of the FIRST annotation of the type (R7 → 1, R45 → 1) | `mirrors/cm6/state/src/transaction.ts:226` | `crates/ferrum-state/src/transaction.rs:201-208` iterates `.rev()` → LAST (R7 → 2) | none | none | differs |
| state.Transaction.annotation.B2 | annotation types compared by identity: two `Annotation.define<number>()` are distinct (R9b) | `mirrors/cm6/state/src/transaction.ts:226` | keyed by `TypeId` of the value type (`crates/ferrum-state/src/transaction.rs:118,133`): any `usize` annotation answers `annotation::<usize>()` | test-state#04 | test_state#004 | differs (design; see state.cm6.121) |
| state.Transaction.annotation.B3 | absent → `undefined` | `mirrors/cm6/state/src/transaction.ts:227` | `None` (`:207`) | none | none in ferrum-state | implemented, untested |
| state.Transaction.annotation.B4 | present → value | `mirrors/cm6/state/src/transaction.ts:226` | `:202-205` | test-state#04 | test_state#004 | implemented+tested |
Proposed status: PORTED, DEVIATION because B1, B2 differ, B3 untested.

### state.cm6.102 `Transaction.docChanged` — cm6 `mirrors/cm6/state/src/transaction.ts:231`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.docChanged.B1 | `!changes.empty`; empty replace `{0,0}` → false (R38 `[false,true,false]`) | `mirrors/cm6/state/src/transaction.ts:231` | `crates/ferrum-state/src/state.rs:262`, `crates/ferrum-state/src/transaction.rs:216-218` (R38 same) | test-state#16 | test_state#023 (true case via filter), test_state#016 (field reads it, no assertion on false) | implemented; false case untested |
Proposed status: PORTED, DEVIATION (untested: false case).

### state.cm6.103 `Transaction.reconfigured` — cm6 `mirrors/cm6/state/src/transaction.ts:237`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.reconfigured.B1 | `startState.config != state.config` (forces state; R15c `[false,true,true]`) | `mirrors/cm6/state/src/transaction.ts:237` | none | none | none | missing |
Proposed status: MISSING (searched `reconfigured`: only an example `println!` in `crates/ferrum/examples/poc_editor.rs:166`).

### state.cm6.104 `Transaction.isUserEvent` — cm6 `mirrors/cm6/state/src/transaction.ts:244`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction.isUserEvent.B1 | exact match → true | `mirrors/cm6/state/src/transaction.ts:246` | none | none | none | missing |
| state.Transaction.isUserEvent.B2 | annotation more specific with a `.` boundary → true ("select.pointer" matches "select", not "sel"; "select.pointer.x" no) (R10) | `mirrors/cm6/state/src/transaction.ts:246` | none | none | none | missing |
| state.Transaction.isUserEvent.B3 | no user event → false | `mirrors/cm6/state/src/transaction.ts:246` | none | none | none | missing |
Proposed status: MISSING (searched `is_user_event`, `isUserEvent`, `user_event` in all crates; `crates/ferrum-commands/src/history.rs:489` reads the raw `UserEvent` string only).

### state.cm6.105 `Transaction^time` — cm6 `mirrors/cm6/state/src/transaction.ts:251`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction^time.B1 | annotation type holding a number (ms) | `mirrors/cm6/state/src/transaction.ts:251` | `crates/ferrum-state/src/transaction.rs:157-159` `TransactionTime(pub u64)` | none | none | implemented, untested in ferrum-state |
| state.Transaction^time.B2 | added automatically with `Date.now()` (R8) | `mirrors/cm6/state/src/transaction.ts:186-187` | `crates/ferrum-state/src/state.rs:278-284` (`SystemTime` ms; 0 if before epoch) | none | none | implemented, untested |
| state.Transaction^time.B3 | explicit time annotation preserved (R7b 42) | `mirrors/cm6/state/src/transaction.ts:186` | `crates/ferrum-state/src/state.rs:278` | none | `crates/ferrum-commands/src/tests/test_history.rs:34` (`timed_type`, indirect) | implemented, tested only indirectly in ferrum-commands |
Proposed status: PORTED, DEVIATION (untested in ferrum-state: B1, B2).

### state.cm6.106 `Transaction^userEvent` — cm6 `mirrors/cm6/state/src/transaction.ts:277`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction^userEvent.B1 | string annotation type | `mirrors/cm6/state/src/transaction.ts:277` | `crates/ferrum-state/src/transaction.rs:153-155` `UserEvent(pub String)` | none | `crates/ferrum-commands/src/tests/test_history.rs` (UserEvent annotations, 10 uses) | implemented, tested indirectly |
| state.Transaction^userEvent.B2 | set by `TransactionSpec.userEvent` shorthand | `mirrors/cm6/state/src/transaction.ts:332` | none | none | none | missing |
| state.Transaction^userEvent.B3 | matched with `isUserEvent` prefix semantics | `mirrors/cm6/state/src/transaction.ts:244-247` | none (R9c: only exact string available) | none | none | missing |
Proposed status: PORTED, DEVIATION because B2, B3 are missing.

### state.cm6.107 `Transaction^addToHistory` — cm6 `mirrors/cm6/state/src/transaction.ts:281`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction^addToHistory.B1 | boolean annotation type; history skips transactions where it is `false` (`mirrors/cm6/commands/src/history.ts:71`) (R41) | `mirrors/cm6/state/src/transaction.ts:281` | `crates/ferrum-state/src/transaction.rs:149-151` `AddToHistory(pub bool)`; consumer `crates/ferrum-commands/src/history.rs:477` | none in state (commands history tests) | `crates/ferrum-commands/src/tests/test_history.rs:46` (`receive`) | implemented, tested in ferrum-commands |
Proposed status: PORTED, RUST-IDIOMATIC (candidate; newtype keyed by `TypeId`, depends on the annotation-typing HUMAN DECISION at state.cm6.121).

### state.cm6.108 `Transaction^remote` — cm6 `mirrors/cm6/state/src/transaction.ts:287`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Transaction^remote.B1 | boolean annotation marking changes from another actor (R41) | `mirrors/cm6/state/src/transaction.ts:287` | none | none | none | missing |
Proposed status: MISSING (searched `remote`, `Remote` in all crates: only a test name in `ferrum-commands`).

### state.cm6.109 `TransactionSpec` — cm6 `mirrors/cm6/state/src/transaction.ts:121`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.B1 | plain object with optional `changes`, `selection`, `effects`, `annotations`, `userEvent`, `scrollIntoView`, `filter`, `sequential` | `mirrors/cm6/state/src/transaction.ts:121-154` | `crates/ferrum-state/src/transaction.rs:65-76` (`changes`, `selection`, `effects`, `annotations` only) | test-state#02..#27 | test_state#002..#027 | differs: four fields missing |
| state.TransactionSpec.B2 | a `Transaction` can be used as a spec (filters return `[tr, spec]`) | `mirrors/cm6/state/src/transaction.ts:387` | none | test-state#24, #25 | none | missing |
Proposed status: PORTED, DEVIATION because B1 is incomplete and B2 missing.

### state.cm6.110 `TransactionSpec.changes` — cm6 `mirrors/cm6/state/src/transaction.ts:123`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.changes.B1 | any `ChangeSpec` (object, `ChangeSet`, nested arrays) | `mirrors/cm6/state/src/transaction.ts:123,334-335` | `crates/ferrum-state/src/transaction.rs:69`, `crates/ferrum-state/src/change.rs:280-298` (`Set`/`Single`/`Multiple`, no nesting) | test-state#02 | test_state#002 | implemented for flat forms; nesting missing |
| state.TransactionSpec.changes.B2 | string inserts split on lineSeparator | `mirrors/cm6/state/src/transaction.ts:335` | insert is `Text` | none | none | differs (D2) |
| state.TransactionSpec.changes.B3 | positions refer to the start doc unless `sequential` | `mirrors/cm6/state/src/transaction.ts:146-153` | single spec only | test-state#25 | none | missing (no multi-spec) |
Proposed status: PORTED, DEVIATION because B1 partial, B2 differs, B3 missing.

### state.cm6.111 `TransactionSpec.selection` — cm6 `mirrors/cm6/state/src/transaction.ts:127`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.selection.B1 | `EditorSelection` or `{anchor, head?}` in post-change coordinates | `mirrors/cm6/state/src/transaction.ts:127,336` | `crates/ferrum-state/src/transaction.rs:71` `Option<EditorSelection>` | test-state#17, #24 | test_state#017, test_state#024 | implemented for `EditorSelection`; object form missing |
| state.TransactionSpec.selection.B2 | validated (RangeError, R32) | `mirrors/cm6/state/src/transaction.ts:185` | none | none | none | missing |
Proposed status: PORTED, DEVIATION because B2 is missing and the object form absent.

### state.cm6.112 `TransactionSpec.effects` — cm6 `mirrors/cm6/state/src/transaction.ts:132`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.effects.B1 | single effect or array (R44) | `mirrors/cm6/state/src/transaction.ts:132,337` | `crates/ferrum-state/src/transaction.rs:73` `Vec<StateEffect>` | test-state#06, #12 | test_state#012, test_state#027 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (candidate; `Vec` for single-or-array).

### state.cm6.113 `TransactionSpec.annotations` — cm6 `mirrors/cm6/state/src/transaction.ts:134`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.annotations.B1 | single annotation or array | `mirrors/cm6/state/src/transaction.ts:134,331` | `crates/ferrum-state/src/transaction.rs:75` `Vec<Annotation>` | test-state#04 | test_state#004 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (candidate).

### state.cm6.114 `TransactionSpec.userEvent` — cm6 `mirrors/cm6/state/src/transaction.ts:136`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.userEvent.B1 | shorthand for `annotations: Transaction.userEvent.of(…)`, appended after other annotations (R10b) | `mirrors/cm6/state/src/transaction.ts:136,332` | none (callers push `UserEvent(..)` annotations manually) | none | none | missing |
Proposed status: MISSING.

### state.cm6.115 `TransactionSpec.scrollIntoView` — cm6 `mirrors/cm6/state/src/transaction.ts:139`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.scrollIntoView.B1 | marks the transaction as needing to scroll the selection into view | `mirrors/cm6/state/src/transaction.ts:139,339` | none | test-state#27 | none | missing |
Proposed status: MISSING.

### state.cm6.116 `TransactionSpec.filter` — cm6 `mirrors/cm6/state/src/transaction.ts:146`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.filter.B1 | `false` disables change filters and transaction filters, not extenders (R11d, R12f) | `mirrors/cm6/state/src/transaction.ts:146,345-352` | none | test-state#23, #26 | none | missing |
Proposed status: MISSING.

### state.cm6.117 `TransactionSpec.sequential` — cm6 `mirrors/cm6/state/src/transaction.ts:153`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TransactionSpec.sequential.B1 | positions of this spec refer to the doc produced by previous specs (R14b, R12b) | `mirrors/cm6/state/src/transaction.ts:153,348-349` | none | test-state#25 | none | missing |
Proposed status: MISSING.

### state.cm6.118 `Annotation` — cm6 `mirrors/cm6/state/src/transaction.ts:15`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Annotation.B1 | holds `type` and `value` | `mirrors/cm6/state/src/transaction.ts:17-22` | `crates/ferrum-state/src/transaction.rs:115-120` (`type_id: TypeId`, `value: Arc<dyn Any>`, both `pub(crate)`) | test-state#04 | test_state#004 | implemented (type erased) |
| state.Annotation.B2 | annotations of distinct types never alias, even with the same value type (R9b) | `mirrors/cm6/state/src/transaction.ts:226` | aliasing by value type | none | none | differs |
Proposed status: PORTED, DEVIATION because B2 differs.

### state.cm6.119 `Annotation.type` — cm6 `mirrors/cm6/state/src/transaction.ts:19`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Annotation.type.B1 | public `type` property (R37 `a.type === t`) | `mirrors/cm6/state/src/transaction.ts:19` | none public (`type_id` is `pub(crate)`) | none | none | missing |
Proposed status: MISSING.

### state.cm6.120 `Annotation.value` — cm6 `mirrors/cm6/state/src/transaction.ts:21`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Annotation.value.B1 | the value (R37 "v") | `mirrors/cm6/state/src/transaction.ts:21` | `crates/ferrum-state/src/transaction.rs:131-138` `downcast_ref::<T>()` | test-state#04 | test_state#004 (through `Transaction::annotation`) | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (candidate; typed downcast instead of a typed field).

### state.cm6.121 `Annotation^define` — cm6 `mirrors/cm6/state/src/transaction.ts:25`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Annotation.define.B1 | each call returns a new, distinct `AnnotationType` (R9b) | `mirrors/cm6/state/src/transaction.ts:25` | none: the Rust value type acts as the annotation type (`Annotation::of::<T>`, `crates/ferrum-state/src/transaction.rs:124-129`) | test-state#04, #26 | test_state#004 | differs: two types sharing a value type are indistinguishable |
Proposed status: HUMAN DECISION. Options: (a) port `AnnotationType<T>` as a value with a unique id (like `Facet`/`StateField` already do); (b) keep `TypeId` keying and require a newtype per annotation, documenting that `annotation::<usize>()` answers for every `usize` annotation; (c) hybrid. No verifiable reason for the current choice found.

### state.cm6.122 `AnnotationType` — cm6 `mirrors/cm6/state/src/transaction.ts:33`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.AnnotationType.B1 | identity marker for a kind of annotation | `mirrors/cm6/state/src/transaction.ts:33-36` | none (Rust type parameter) | test-state#04 | test_state#004 | differs |
Proposed status: HUMAN DECISION (same options as state.cm6.121).

### state.cm6.123 `AnnotationType.of` — cm6 `mirrors/cm6/state/src/transaction.ts:35`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.AnnotationType.of.B1 | creates an `Annotation` of this type with the value | `mirrors/cm6/state/src/transaction.ts:35` | `crates/ferrum-state/src/transaction.rs:124-129` `Annotation::of::<T>(value)`, `TransactionSpec::with_annotation` `:109-112` | test-state#04 | test_state#004 | implemented+tested (type = `T`) |
Proposed status: PORTED, DEVIATION (type identity = value type, see state.Annotation.B2).

### state.cm6.124 `StateEffect` — cm6 `mirrors/cm6/state/src/transaction.ts:69`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.B1 | holds `type` (internal) and `value` | `mirrors/cm6/state/src/transaction.ts:71-75` | `crates/ferrum-state/src/transaction.rs:9-14` | test-state#12 | test_state#012 | implemented+tested |
| state.StateEffect.B2 | mappable through position changes (`map`) | `mirrors/cm6/state/src/transaction.ts:79-82` | none | none | none | missing |
| state.StateEffect.B3 | type identity by object (R9) | `mirrors/cm6/state/src/transaction.ts:86` | `TypeId` of value type | none | none | differs |
Proposed status: PORTED, DEVIATION because B2 missing, B3 differs.

### state.cm6.125 `StateEffect.value` — cm6 `mirrors/cm6/state/src/transaction.ts:75`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.value.B1 | the value | `mirrors/cm6/state/src/transaction.ts:75` | `crates/ferrum-state/src/transaction.rs:38-45` `downcast_ref::<T>()`; used by `apply` (`crates/ferrum-state/src/state.rs:312`) | test-state#12, #27 | test_state#012 (via `apply`) | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (candidate).

### state.cm6.126 `StateEffect.map` — cm6 `mirrors/cm6/state/src/transaction.ts:79`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.map.B1 | applies the type's `map(value, mapping)` (R36 a → 6) | `mirrors/cm6/state/src/transaction.ts:80` | none | none | none | missing |
| state.StateEffect.map.B2 | `undefined` → effect deleted (R36 b) | `mirrors/cm6/state/src/transaction.ts:81` | none | none | none | missing |
| state.StateEffect.map.B3 | unchanged value → the same effect object (R36 `cSame`) | `mirrors/cm6/state/src/transaction.ts:81` | none | none | none | missing |
Proposed status: MISSING (searched `map_effects`, `fn map` in transaction.rs, `mapEffects`; ferrum-commands `test_cm6_hist_44_can_map_effects` at `crates/ferrum-commands/src/tests/test_history.rs:738` asserts only `undo_depth == 0`).

### state.cm6.127 `StateEffect.is` — cm6 `mirrors/cm6/state/src/transaction.ts:86`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.is.B1 | true iff the effect's type is the given type object (R9: e1 true, e2 false) | `mirrors/cm6/state/src/transaction.ts:86` | `crates/ferrum-state/src/transaction.rs:33-36` `is::<T>()` compares `TypeId` of the value type (R9: any `usize` effect is `is::<usize>()`) | test-state#27 | none (test_state#027 never calls `is`) | differs |
Proposed status: PORTED, DEVIATION because B1 differs.

### state.cm6.128 `StateEffect^define` — cm6 `mirrors/cm6/state/src/transaction.ts:93`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.define.B1 | creates a new distinct `StateEffectType` | `mirrors/cm6/state/src/transaction.ts:94` | none (value type acts as effect type) | test-state#27 | none | differs |
| state.StateEffect.define.B2 | optional `map` spec, identity mapping by default | `mirrors/cm6/state/src/transaction.ts:94` | none | none | none | missing |
Proposed status: HUMAN DECISION (same options as state.cm6.121, plus where a `map` function would live).

### state.cm6.129 `StateEffect^mapEffects` — cm6 `mirrors/cm6/state/src/transaction.ts:98`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.mapEffects.B1 | empty input → same array (R36 `emptySame`) | `mirrors/cm6/state/src/transaction.ts:99` | none | none | none | missing |
| state.StateEffect.mapEffects.B2 | each effect mapped, deleted ones dropped, order kept (R36 list `[6,3]`) | `mirrors/cm6/state/src/transaction.ts:100-105` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.130 `StateEffect^reconfigure` — cm6 `mirrors/cm6/state/src/transaction.ts:113`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.reconfigure.B1 | replaces the root extensions; appended config discarded; compartment contents kept (test-state#14 "0,2"; R15b) | `mirrors/cm6/state/src/transaction.ts:113`, `mirrors/cm6/state/src/state.ts:106-108` | none | test-state#06, #07, #11, #14, #15 | none | missing |
| state.StateEffect.reconfigure.B2 | fields still configured keep their values (R15e) | `mirrors/cm6/state/src/state.ts:116-119` | none | test-state#11 | none | missing |
Proposed status: MISSING (searched `reconfigure`: only `Compartment::reconfigure` and `CompartmentReconfigure`).

### state.cm6.131 `StateEffect^appendConfig` — cm6 `mirrors/cm6/state/src/transaction.ts:116`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffect.appendConfig.B1 | appends extensions to the root config (R15b `[2,3,5]`) | `mirrors/cm6/state/src/transaction.ts:116`, `mirrors/cm6/state/src/state.ts:109-111` | none | test-facet#16, #18, #19, #21, #22 (C4) | none | missing |
Proposed status: MISSING (searched `append_config`, `appendConfig`).

### state.cm6.132 `StateEffectType` — cm6 `mirrors/cm6/state/src/transaction.ts:48`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffectType.B1 | identity object holding the `map` function | `mirrors/cm6/state/src/transaction.ts:48-57` | none | test-state#27 | none | differs |
Proposed status: HUMAN DECISION (see state.cm6.121/128).

### state.cm6.133 `StateEffectType.of` — cm6 `mirrors/cm6/state/src/transaction.ts:61`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffectType.of.B1 | creates a `StateEffect` with this type and the value | `mirrors/cm6/state/src/transaction.ts:61` | `crates/ferrum-state/src/transaction.rs:25-31` `StateEffect::of::<T>(value)` | test-state#27 | test_state#027 | implemented+tested (type = `T`) |
Proposed status: PORTED, DEVIATION (type identity = value type, see state.StateEffect.is.B1).

### state.cm6.259 `StateEffect.define spec.map` — cm6 `mirrors/cm6/state/src/transaction.ts:43`
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateEffectSpec.map.B1 | `(value, mapping) => value | undefined`; used by `map`, `mapEffects`, `changeByRange`, `mergeTransaction`, change filters | `mirrors/cm6/state/src/transaction.ts:43`, `:80`, `:324`, `:377`, `mirrors/cm6/state/src/state.ts:164` | none | none | none | missing |
Proposed status: MISSING.

## Unfinished
none
