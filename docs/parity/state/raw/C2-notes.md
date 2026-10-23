# C2 notes (`change.ts` / `change.rs`)

> What: chunk C2 decision evidence, cross-crate facts, oracle, open questions. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `docs/parity/state/raw/C2-api.md` § Reproductions.

## Decision evidence

### D1 (position unit)

Items affected: every length and position in this chunk: `ChangeDesc.length`/`newLength` (`mirrors/cm6/state/src/change.ts:34`, `:41` ↔ `crates/ferrum-state/src/change.rs:46`, `:57`), section lengths (`ChangeSet.of` insert length `mirrors/cm6/state/src/change.ts:336` ↔ `change.rs:519` `len_chars()`), `apply` length guard (`:208` ↔ `change.rs:380`), `invert` slices (`:227` ↔ `change.rs:429`), `mapPos` (`:120` ↔ `change.rs:149`), `iterChanges`/`iterGaps`/`iterChangedRanges` positions, `touchesRange` (missing), JSON lengths (`toJSON` `:299`, missing in Ferrum).

Concrete values (R9 in `docs/parity/state/raw/C2-api.md`; oracle unicode sample below):

| case | cm6 (UTF-16) | Ferrum now (code points) | UTF-8 bytes (option) |
|---|---|---|---|
| doc `"a😀b"` length | 4 | 3 | 6 |
| insert `"😀"` at 1: section | `1 0:2 3`, newLength 6 | `1 0:1 2`, new_length 4 | `1 0:4 …` |
| delete the emoji | `{from:1,to:3}` | `delete(1,2)` | `1..5` |
| cm6 numbers `delete(1,3)` replayed in Ferrum | — | panic `Mismatched document length: changeset expected 4, doc is 3` | would cut inside `b`'s neighbour / need re-mapping |
| `{from:1,to:2}` (half a surrogate pair) | allowed, result `"a\ude00b"` (lone surrogate) | not expressible | would have to be rejected (not a char boundary) |
| insert `"é"` at 0, `mapPos(3)` | 4 | 4 | 5 |
| `toJSON` of insert `"😀"` into empty doc | `[[0,"😀"]]` with section `0:2` | no JSON | section `0:4` |

What each option implies for C2: UTF-16 keeps every cm6 expected value and JSON compatibility (`ChangeSet.toJSON` from cm6 collab/history peers could be read unchanged); code points (current) make JSON exchanged with cm6 peers carry different lengths for astral text; UTF-8 bytes would make lengths differ for every non-ASCII char (é too) and require boundary validation in `ChangeSet.of`/`apply`/`invert`. All 40 cm6 cases in `test-change.ts` are ASCII-only, so none of their expected values change under any option. `mapPos`'s `assoc: i8` parameter (`change.rs:149`) is unrelated to the unit but is a type choice that loses information (see Cross-crate facts).

### D2 (line breaks / Text model)

Items affected: `ChangeSet.of` string inserts (`mirrors/cm6/state/src/change.ts:335`: `Text.of(insert.split(lineSep || DefaultSplit))`, `DefaultSplit = /\r\n?|\n/` `:3`) ↔ Ferrum `ChangeSpec.insert: Text` built with `Text::from(&str)` → `Rope::from_str` (`crates/ferrum-state/src/text.rs:28`, `:218`), no split, no `lineSep` parameter; `EditorState.changes` / transaction resolution (`mirrors/cm6/state/src/state.ts:178`, `mirrors/cm6/state/src/transaction.ts:335` pass `state.facet(lineSeparator)`) ↔ `EditorState::changes` `crates/ferrum-state/src/state.rs:201` and `update` `state.rs:257-260` ignore `EditorState::line_separator()` (`state.rs:185`).

| insert string | cm6 section / resulting text | Ferrum section / resulting text |
|---|---|---|
| `"a\r\nb"` | `0:3`, `"a\nb"` | `0:4`, `"a\r\nb"` |
| `"a\rb"` | `0:3`, `"a\nb"` | `0:3`, `"a\rb"` (ropey counts 2 lines) |
| `"a b"` | `0:3`, 1 line | `0:3`, 2 lines (ropey `unicode_lines`) |
| `"a\|b\nc"` with `lineSep = "\|"` | `0:5`, `"a\nb\nc"`, 2 lines | no `lineSep` |
| `"x\r\ny"` into empty doc | newLength 3 | new_length 4 |

Under the D2 recommendation (normalize on input, only `\n` breaks) Ferrum's `ChangeSet::of_specs` would need the split (or `ChangeSpec` would need a string variant that is split with the state's separator), and `Changes::to_changeset` would need the line separator.

## Cross-crate facts

1. **`touchesRange` (missing in Ferrum).** cm6 uses: `mirrors/cm6/view/src/editorview.ts:1254` (bidi order cache invalidation), `mirrors/cm6/view/src/viewstate.ts:440` (line-gap mapping), `mirrors/cm6/view/src/domobserver.ts:688-689` (DOM selection restore). Searched: `touches_range`, `touchesRange`, `cover` in `crates/*` → none (only unrelated doc comments). `ferrum-view` does not use `ChangeDesc`/`ChangeSet` at all (`grep -n "map_pos\|ChangeDesc\|ChangeSet" crates/ferrum-view/src/*.rs` → no hits; `heightmap.rs:50` defines its own `ChangedRange`). Owner per graph: ferrum-state.
2. **`iterChangedRanges`.** cm6 uses: `mirrors/cm6/view/src/extension.ts:447` (`ViewUpdate.changedRanges`), `mirrors/cm6/language/src/fold.ts:104` (clearing touched folds), `mirrors/cm6/language/src/language.ts:422` (`ParseContext.changes`). Ferrum has it (`crates/ferrum-state/src/change.rs:99`), used only by `crates/ferrum-commands/src/history.rs:419-421`; `crates/ferrum-language/src/tree.rs:176` `changes(&self, _changes, ..)` ignores its changes argument; untested in ferrum-state.
3. **`ChangeSet.toJSON`/`fromJSON`, `ChangeDesc.toJSON`/`fromJSON` (missing).** cm6 uses: `mirrors/cm6/commands/src/history.ts:197-210` (history event serialization, `json.changes && ChangeSet.fromJSON`, `json.mapped && ChangeDesc.fromJSON`). Ferrum `crates/ferrum-commands/src/tests/test_history.rs:750` `test_cm6_hist_46_survives_serialization` does not serialize anything (asserts only `state.doc().to_string() == "abcd"`).
4. **`invertedDesc`.** cm6 uses `mirrors/cm6/view/src/docview.ts:621`; Ferrum uses `inverted_desc` in `crates/ferrum-commands/src/history.rs:278`; no test in ferrum-state.
5. **`mapPos` `assoc: i8` and `i32` sides.** `crates/ferrum-state/src/rangeset.rs:247`, `:250`, `:259`, `:260` call `changes.map_pos(pos, r.value.start_side() as i8, …)` / `end_side() as i8`; `RangeValue::start_side` returns `i32` and ferrum-view decorations use sides like `SIDE_INLINE_BEFORE = -100_000_000`, `SIDE_BLOCK_INC_START = -300_000_000`, `SIDE_NON_INC_END = -600_000_000`, `SIDE_INLINE_AFTER = 100_000_000` (`crates/ferrum-view/src/decoration.rs:22-31`). Measured (R7): `-100_000_000i32 as i8 == 0`, `-300_000_000 as i8 == 0`, `-600_000_000 as i8 == 0`, `100_000_000 as i8 == 0`, `-100_000_001 as i8 == -1`. cm6 passes the number unchanged (`mirrors/cm6/state/src/rangeset.ts:139`, `:143`, `:147-148`: `changes.mapPos(curFrom, val.startSide, val.mapMode)`), where only the sign matters (`mirrors/cm6/state/src/change.ts:134-135`). Consequence: a negative side such as `SIDE_INLINE_BEFORE` becomes assoc 0 (forward) in Ferrum. Owner: C6 (`rangeset.rs`) / ferrum-view, root cause is the `i8` parameter of `ChangeDesc::map_pos` (C2).
6. **`ChangeSet.filter` (cm6 `@internal`, `mirrors/cm6/state/src/change.ts:269-296`)**, used by `changeFilter` in `mirrors/cm6/state/src/transaction.ts`. No `filter` method in `crates/ferrum-state/src/change.rs`; owner of the audit is C5 (`changeFilter`).
7. **`ChangeSet::map_pos`** (Simple-only shortcut, `change.rs:363`) is used by `crates/ferrum-commands/src/commands.rs:1124-1125`.
8. Observed while reading ferrum-state tests for indirect coverage (C6 owns them): `crates/ferrum-state/src/tests/test_rangeset.rs:413-444` (test_rangeset#025–#027) build a `ChangeSet::empty` and call `RangeSet::compare` without any assertion (comment "In RED phase stub, compare doesn't fill comp yet").

## Oracle

Files (scratch, outside the repositories):
- cm6 side and generator: `docs/parity/state/oracle/cm6-probes/probe-C2/oracle.ts` (`gen <seed> <n> <ascii|unicode>`, `run <cases>`, `cmp <cm6-out> <ferrum-out> <cases>`), run with `tsx` from `docs/parity/state/oracle.md`.
- Ferrum side: `docs/parity/state/oracle/ferrum-repro/repro-C2/src/bin/oracle.rs` (`cargo build --release --offline --bin oracle`; reads the cases JSON, writes results JSON; uses `serde_json` in the scratch crate only).
- Unicode classifier: `docs/parity/state/oracle/tools/classify-C2.cjs`.
- Data: `(oracle data, not archived: regenerate from the seed recorded in `docs/parity/state/oracle.md`)` (`cases-ascii.json`, `cases-ascii2.json`, `cases-uni.json`, `cm6-*.json`, `ferrum-*.json`, `cmp-uni.json`).

PRNG: mulberry32 (seeded, in `oracle.ts`). Case shape: doc of `r(25)` characters from `a–h` and `\n`; spec lists A and B (0–6 specs each) for the doc length, spec list C (0–6 specs) for `A.newLength`; each spec is an insertion (`from ∈ [0,len]`, 1–4 chars), a deletion, or a replacement (`from ∈ [0,len]`, `to ≤ from+5`, 0–3 chars incl. `\n`), unsorted and possibly overlapping. Specs are passed to cm6 as `{from,to,insert}` and to Ferrum as `ChangeSpec::replace(from,to,insert)`.

Compared per case (JSON-equal): `A` and `B` section strings; `[length, newLength, empty]`; `iterChanges` joined and individual (with text); `iterGaps`; `apply(doc)`; `A.map(B)` and `A.map(B, true)` (sections and individual changes); `A.desc.mapDesc(B.desc, false/true)`; convergence pair `A∘B.map(A)` / `B∘A.map(B,true)` applied; `invert` (sections, texts, apply back); `invertedDesc`; `A.compose(C)` (sections, texts, apply); `composeDesc`; `mapPos(pos, assoc ∈ {−1, 1}, all four modes)` for every `pos ∈ [0, size]`; `iterChangedRanges(individual = true)`. cm6 `toJSON` is recorded in the cm6 output (`json` key) but has no Ferrum counterpart and is excluded from comparison.

Runs:
| run | seed | cases | mapPos values | `null` results | changes iterated | empty A sets | mismatching cases |
|---|---|---|---|---|---|---|---|
| ASCII 1 | 3266 | 5,000 | 520,664 | 83,190 | 11,831 | 815 | 0 |
| ASCII 2 | 777 | 20,000 | 2,095,136 | 330,540 | 46,616 | 3,345 | 0 |
| non-ASCII sample | 4242 | 500 | — | — | — | — | 383 |

The non-ASCII sample keeps the doc ASCII and draws inserts from `a–h`, `\n`, `é`, `😀`, `\r\n`, `\r`, U+2028 (no C list). Mismatches by insert content (`classify.cjs`): ASCII-only 0/62; `é` only 0/19; U+2028 only 0/22; `é`+U+2028 0/14; every class containing `😀` or `\r`: 100% mismatching (e.g. `CR` 73/73, `astral` 18/18). Minimal mismatching cases: (astral) doc `"dacg"`, A = `[{from:1,to:1,insert:"😀bc"}]` → cm6 `"1 0:4 3"`, Ferrum `"1 0:3 3"`, and every `mapPos` after position 1 is one lower in Ferrum (D1); (CR) doc `"hc\ngfcad"`, A = `[{from:8,to:8,insert:"h \r\n"}]` → cm6 apply `"hc\ngfcadh \n"`, Ferrum `"hc\ngfcadh \r\n"` (D2). These are the documented D1/D2 differences, not algorithm bugs.

Not covered by the oracle: error paths (covered by repros R2), JSON (no Ferrum counterpart), `touchesRange` (missing), `ChangeSpec` forms Ferrum cannot express (nested arrays, `ChangeSet` in lists, `lineSep`).

Conclusion: for ASCII input the Ferrum implementation of `ChangeSet.of` (flat lists), `apply`, `compose`, `composeDesc`, `map`, `mapDesc`, `invert`, `invertedDesc`, `mapPos` (all modes, assoc ±1), `iterChanges`, `iterGaps`, `iterChangedRanges`, `length`, `newLength`, `empty` matched cm6 on all 25,000 cases.

## Open questions

1. Should the oracle (or a ported version of it as a Ferrum test with fixed cases exported from cm6) count as test evidence? With it, state.cm6.149–154, the untested-only deviations of 139/141/142/146/155/156 and 157 B9 would be proven; without it they stay `PORTED, DEVIATION (untested …)`.
2. Panic/error text: cm6 messages are `RangeError`/`Error` texts; Ferrum panics use different texts for `apply` (`Mismatched document length: …`), `of` (`Invalid change range F..T for document of length L`), `compose`/`map` (`… in compose`/`… in map`); `mapPos` uses the same text. HUMAN DECISION: whether exact message parity is required, and whether these preconditions should be panics (rules.md §6.4 allows documented panics; none of these is documented under `# Panics`) or `Result`s.
3. `ChangeSpec`/`Changes` shape: cm6's recursive union (`ChangeSpec[]` nesting, `ChangeSet` inside lists) has no Ferrum counterpart. Options: a recursive `Changes` enum (`Multiple(Vec<Changes>)`), or keep flat lists and document. HUMAN DECISION.
4. Public raw constructors (`ChangeDesc::create`, `ChangeSet::create_set`, `ChangeDesc::from_string`) are `@internal` or test-only in cm6 and unvalidated in Ferrum (R2s, R11c). No verifiable reason found for making them `pub`. HUMAN DECISION (keep, hide behind `#[doc(hidden)]`/`cfg(test)`, or validate).
5. `ChangeDesc::empty(length)` reuses cm6's name `empty` for a constructor while the boolean is `is_empty`. Keep or rename? HUMAN DECISION.
6. `assoc: i8` in `map_pos`: change to `i32` (or a sign type) given the truncation seen in rangeset callers (Cross-crate fact 5)? HUMAN DECISION, evidence in R7.
7. `ChangeSet::map_pos` (Simple only, `unwrap_or(pos)` unreachable): keep as the counterpart of cm6's 2-argument overload, or add a mode parameter like cm6 (`ChangeSet.mapPos(pos, assoc, mode)`)?
8. JSON (`toJSON`/`fromJSON`): needed for history serialization and collab; format choice depends on D1 (lengths) and D2 (lines). Not decided here.

## Unfinished

none
