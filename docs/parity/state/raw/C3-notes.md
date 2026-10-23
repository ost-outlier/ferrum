# C3 notes (selection + char + charcategory + column)

> What: chunk C3 decision evidence, cross-crate facts, oracle, open questions. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `docs/parity/state/raw/C3-api.md` § Reproductions.

Scratch paths of the producing session were replaced by archived copies under `docs/parity/state/oracle/` (resolution rules: `docs/parity/README.md`). R-numbers refer to `docs/parity/state/raw/C3-api.md` `## Reproductions`.

## Decision evidence

### D1 (position unit)

Items affected (every C3 item that takes or returns a position or length): state.cm6.062 `ranges`, 064 `EditorSelection.map`, 070/071 `EditorSelection.toJSON/fromJSON`, 072 `single`, 073 `create`, 074 `cursor`, 075 `range`, 076 `undirectionalRange`, 078 `from`, 079 `to`, 081 `anchor`, 082 `head`, 087 `SelectionRange.map`, 088 `extend`, 090/091 `SelectionRange.toJSON/fromJSON`, 212 `findClusterBreak`, 213 `codePointAt`, 214 `fromCodePoint`, 215 `codePointSize`, 216 `countColumn` (`to`), 217 `findColumn` (result); Ferrum `selection.rs` (all `usize` fields/args), `char.rs:31` (`find_cluster_break`), `char.rs:70`, `char.rs:101`, `column.rs:8`, `column.rs:28`.

Current units: cm6 UTF-16 code units (`mirrors/cm6/state/src/char.ts:19-37`, `@marijn/find-cluster-break/src/index.js:48-50`); Ferrum `char` index (`crates/ferrum-state/src/char.rs:32`, `:38-42`, `crates/ferrum-state/src/column.rs:9`, `:29`).

Concrete values where the unit changes the result (all run, R2/R3):

| case | cm6 (UTF-16) | Ferrum (chars) | UTF-8 bytes would give |
|---|---|---|---|
| `findClusterBreak("😀", 0)` | 2 | 1 | 4 |
| `findClusterBreak("🇫🇷🇩🇪", 0)` | 4 | 2 | 8 |
| `findClusterBreak("🇫🇷🇩🇪", 8, false)` (end, backward) | 4 | 2 (from 4) | 8 (from 16) |
| `findClusterBreak("😀x", 1)` (inside surrogate pair) | 2 (snaps to pair start then advances) | position 1 means "after 😀" → 2 | byte 1 is inside the code point; would need a "snap to boundary" rule |
| `findColumn("😀b", 1, 4)` | 2 | Some(1) | 4 |
| `countColumn("😀b", 4, 1)` (`to` inside pair) | 1 | not expressible | `to` inside code point: must be defined |
| `codePointSize(0x1F600)` | 2 | (no function; would be 1) | 4 (`len_utf8`) |
| `codePointAt("a😀", 2)` | 56832 (low surrogate) | (no function) | (byte 2 is inside the code point) |
| `EditorSelection.toJSON()` of a cursor after `"é😀"` | `{"ranges":[{"anchor":3,"head":3}],"main":0}` (run: `docs/parity/state/oracle/cm6-probes/probe-C3/json.ts`) | (no JSON) | 6 |

What each option implies for C3:
- UTF-16: `codePointAt`/`fromCodePoint`/`codePointSize` port 1:1, all cm6 test and JSON values reused unchanged, lone-surrogate behavior (C09–C11) cannot occur with Rust `&str` (no lone surrogates) so B2/B3 of findClusterBreak become "snap" rules; Ferrum `find_cluster_break` would need UTF-16 ↔ byte conversion on every call.
- Code points (current): `codePointSize` would be constantly 1, `codePointAt` = `chars().nth`, `fromCodePoint` = `char::from_u32`; ASCII tests unchanged; JSON offsets differ from cm6 for any astral-plane character; per-call char→byte conversion is O(n) (`char.rs:38-42`) unless the rope is used.
- UTF-8 bytes: these three helpers become `len_utf8`/`str[pos..].chars().next()`; `unicode-segmentation` already works in bytes (`grapheme_indices` returns byte offsets, so `char.rs:49`, `:52`, `:63` conversions disappear); positions inside a code point need a defined behavior (cm6 has the analogous mid-surrogate rule, `index.js:48`).

None of the C3 cm6 test cases (Table 1 of `docs/parity/state/raw/C3-tests.md`) contains a non-ASCII position, so no C3 test expectation changes under any option.

### D2 (line breaks / Text model)

- cm6 applies `findClusterBreak`/`countColumn`/`findColumn` to **line text** (lines are stored without separators: e.g. `mirrors/cm6/view/src/cursor.ts:22-31`, `mirrors/cm6/commands/src/commands.ts:62`, `:581`, `:611`), so CRLF never reaches the cluster function in normal operation; when it does (a raw string), cm6 treats `\r` and `\n` as separate clusters (B8, R2 T02).
- Ferrum `find_cluster_break` is called on the **whole document string** in `crates/ferrum-commands/src/commands.rs:78/87`, `:104/113`, `:129/135`, `:150/156` (`state.doc().to_string()`), so separators stored raw by the ropey-backed `Text` are part of the input: a CRLF document gives one cluster for `\r\n` (UAX #29 GB3) and `\n` followed by a combining mark/ZWJ breaks (GB4/5), whereas in cm6 the same content would be on two lines and the separator would count as one position.
- `countColumn("a\r\nb", 4)`: cm6 4, Ferrum 3 (R2 T02) — relevant only if a string with an embedded CRLF is passed (Ferrum callers pass line text: `crates/ferrum-commands/src/commands.rs:1163`, `:1333`, `crates/ferrum-language/src/indent.rs:158`).
- Under the D2 recommendation (only `\n` as a line break, separators normalized on input), the CRLF mismatch class disappears for line-text callers; the whole-document callers in ferrum-commands would still see `\n`.

### Other evidence (no D-number; for the human)

- Grapheme segmentation policy: cm6 deliberately uses a simplified, table-driven approximation (`@marijn/find-cluster-break`, Extend + most SpacingMark below U+30000, ZWJ joins anything, no CR/LF/Control/Hangul/Prepend/GB9c rules). Ferrum uses full UAX #29 (`unicode-segmentation` 1.13.3). Choosing "cm6 parity" or "UAX #29" changes cursor motion, deletion, `countColumn` and `findColumn` for CRLF, Hangul jamo, Indic conjuncts, Prepend, tag-sequence flags, ZWJ between non-emoji, and control + mark (R2). Proposed `HUMAN DECISION` on state.cm6.212 (it is a deviation either way; the choice is which behavior is intended).

## Cross-crate facts

| capability | cm6 evidence | searched in Ferrum | what exists |
|---|---|---|---|
| `findClusterBreak(…, includeExtending = false)` for backward deletion of variation selectors / marks | `mirrors/cm6/commands/src/commands.ts:581`, `:585` (`deleteByChar`) | `find_cluster_break` signatures and callers in all crates | no parameter (`crates/ferrum-state/src/char.rs:31`); callers `crates/ferrum-commands/src/commands.rs:669`, `:693` use the 3-argument form |
| `EditorSelection.undirectionalRange` / `SelectionRange.undirectional` (double-click word selection, macOS-style selections) | `mirrors/cm6/view/src/cursor.ts:35` (`groupAt`), `mirrors/cm6/view/src/cursor.ts:167-168`, `mirrors/cm6/view/src/input.ts:591`, `:653` | `undirectional`, `undecided` in all crates | absent |
| `SelectionRange.bidiLevel` (caret bidi level, cursor motion in bidi text) | `mirrors/cm6/view/src/docview.ts:218-219`, `mirrors/cm6/view/src/bidi.ts:438`; set by `EditorSelection.cursor(pos, assoc, bidiLevel)` | `bidi` in all crates | absent (only a doc comment, `crates/ferrum-state/src/selection.rs:14`) |
| `SelectionRange.extend(from, to, assoc)` and `EditorSelection.replaceRange` / `addRange` (mouse selection) | `mirrors/cm6/view/src/input.ts:626`, `:630`; `mirrors/cm6/view/src/domchange.ts:60`, `:282` | `extend(` callers, `replace_range`, `add_range` in all crates | `extend` without assoc (`selection.rs:98`); `replace_range`/`add_range` for selections absent |
| `EditorSelection.fromJSON`/`toJSON` (history serialization) | `mirrors/cm6/commands/src/history.ts:211-212` | `from_json`, `to_json`, `serde` in all crates | absent |
| Group motion (`moveByGroup`/`byGroup`, `groupAt`) — cm6 in `view` | `mirrors/cm6/view/src/editorview.ts:694-695`, `mirrors/cm6/view/src/cursor.ts:15-36`, `:102-109`; `mirrors/cm6/commands/src/commands.ts:93-110` (`toGroupStart`) | `find_word_break`, `group` in all crates | `find_word_break` in **ferrum-state** `crates/ferrum-state/src/char.rs:70`, used by `crates/ferrum-commands/src/commands.rs:182`, `:204`, `:225`, `:246`, `crates/ferrum-view-gpui/src/editor_view.rs:235-236`; not in ferrum-view. Behaves differently at whitespace (R8 W01 → 4; cm6 `byGroup` code continues into the next word after leading whitespace, `mirrors/cm6/view/src/cursor.ts:105-107`; cm6 side not executed, needs a view) |
| Subword motion — cm6 in `commands` | `mirrors/cm6/commands/src/commands.ts:115-161` (`moveBySubword`, incl. `Intl.Segmenter` for CJK :112, :146-156) | `find_subword_break`, `subword` in all crates | `find_subword_break` in **ferrum-state** `crates/ferrum-state/src/char.rs:101`, used by `crates/ferrum-commands/src/commands.rs:268`, `:290`, `:311`, `:332`; per-char, no segmenter, panics for `pos > len` backward (R8 S05) |
| Character categorizer with language `wordChars` | `mirrors/cm6/state/src/state.ts:373-376` (C5), used by `mirrors/cm6/view/src/cursor.ts:16`, `:103`, `mirrors/cm6/commands/src/commands.ts:94`, `:116`, `:604` | `char_categorizer`, `word_chars`, `categorize_char` callers | `EditorState::char_categorizer` ignores position and language data (`crates/ferrum-state/src/state.rs:190-198`); ferrum-commands calls `categorize_char` directly (`crates/ferrum-commands/src/commands.rs:746`), bypassing any per-state categorizer |
| Linear-time cluster/column functions on long lines | cm6 `findClusterBreak` is O(1) local (`index.js:45-75`) | callers of `find_cluster_break`, `count_column` | Ferrum O(n)/O(n²) per call (R5), called on the whole document in `crates/ferrum-commands/src/commands.rs:87`, `:113`, `:135`, `:156`, `:669`, `:693`, and per line in `count_column` callers (`crates/ferrum-language/src/indent.rs:158`, `crates/ferrum-commands/src/commands.rs:1163`, `:1333`) |
| `asSingle` when `allowMultipleSelections` is off | `mirrors/cm6/state/src/state.ts:123`, `:251` (C5) | `as_single`, readers of `ALLOW_MULTIPLE_SELECTIONS` | facet defined `crates/ferrum-state/src/state.rs:13`, `:165`, never read; selection never collapsed (R6) |
| `checkSelection` on state creation and transactions | `mirrors/cm6/state/src/state.ts:250`, `mirrors/cm6/state/src/transaction.ts:185` (C5 callers) | "outside of document", `check_selection` | absent; out-of-document selections accepted (R1 S97, R6) |

## Oracle

Three differential harnesses (Node 22.23.3 + tsx on `mirrors/cm6/state/src` vs Ferrum release build of `docs/parity/state/oracle/ferrum-repro/repro-C3`), all with fixed seeds, PRNG mulberry32.

1. **Cluster/column oracle** — files: `docs/parity/state/oracle/cm6-probes/probe-C3/gen.ts` (case generation + cm6 results), `docs/parity/state/oracle/ferrum-repro/repro-C3/src/bin/charcol.rs` (Ferrum results), `docs/parity/state/oracle/cm6-probes/probe-C3/compare.ts` (diff). Data in `(oracle data, not archived: regenerate from the seed recorded in `docs/parity/state/oracle.md`)`, `cm6-results.txt`, `ferrum-results.txt`, `compare.out`, `ascii-mm.txt`, `nonascii-mm.txt`. Seed `0xC3C3`. Inputs: 34 hand-written table cases (R2), 2000 ASCII strings (length 0–12 over `a b Z space \t _ 1 . \r \n`), 500 non-ASCII strings (length 0–9 over `a \t space é U+0301 😀 👍 🏽 ZWJ 🇫 🇷 中 NBSP ᄀ ᅡ U+094D क U+FE0F \r \n`). Per string: forward and backward `findClusterBreak` from every code-point index, `countColumn(s,4,i)` for every i, `countColumn(s,4)`, `findColumn(s,col,4)` and strict for col 0..4n+2, all compared in code-point units (cm6 UTF-16 offsets converted). Results: ASCII 105/2000 strings mismatch, **every one contains `\r\n`** (checked: 0 mismatching ASCII strings without CRLF); minimal case `"1\r\na"` → cm6 fwd `1,2,3,4,4` | ferrum `1,3,3,4,4`. Non-ASCII 122/500 mismatch; every one falls into the classes CRLF, ZWJ next to a non-pictograph, Hangul jamo, Indic conjunct, or control char followed by an extending char (categorized automatically, 0 unexplained); minimal cases are the T-rows in R2 (e.g. `"a‍b"`, `"각"`, `"\ŕ"`).
2. **Selection oracle** — files `docs/parity/state/oracle/cm6-probes/probe-C3/selgen.ts`, `docs/parity/state/oracle/ferrum-repro/repro-C3/src/bin/selorc.rs`; data `(oracle data, not archived: regenerate from the seed recorded in `docs/parity/state/oracle.md`)`, `sel-cm6.txt`, `sel-ferrum.txt`. Seed `0x5E1C`, 5000 cases (1–6 ranges in 0..19, 40 % empty, random main, one random replacement on a 20-char doc, assoc ±1). `create` anchor/head/main: 1 mismatch / 5000; `map`: 1 / 5000 (same case); per-range assoc: 4380 / 5000 mismatch (cm6 ±1 on non-empty ranges, Ferrum 0). Minimal mismatch: `create([cursor(12), range(12,18), cursor(12)], 2)` → cm6 `main=1` | ferrum `main=0` (R4).
3. **Categorizer, exhaustive** — files `docs/parity/state/oracle/cm6-probes/probe-C3/catall.ts`, `docs/parity/state/oracle/ferrum-repro/repro-C3/src/bin/catall.rs`; data `(oracle data, not archived: regenerate from the seed recorded in `docs/parity/state/oracle.md`)`, `cat-ferrum.txt`, `cat-diff.txt`. All 1 111 998 scalar values as one-code-point strings. 2 mismatches: U+0085 (cm6 Other, ferrum Space), U+FEFF (cm6 Space, ferrum Other). Both sides use Unicode 17.0 (Node `process.versions.unicode` 17.0, Rust `char::UNICODE_VERSION` 17.0.0), so the `\p{Alphabetic}`/`\p{Number}` vs `is_alphanumeric` comparison is not confounded by version skew. Multi-code-point strings are not covered by this oracle; the hand cases in R7 show the differences.

## Open questions

1. Grapheme policy for `findClusterBreak`/`countColumn`/`findColumn`: reproduce cm6's simplified algorithm (bit-exact parity with cm6 cursor motion, deletion and columns) or keep UAX #29 (more correct for CRLF, Hangul jamo, Indic conjuncts, Prepend, tag flags)? Either way `includeExtending` is missing. (HUMAN DECISION candidate on state.cm6.212, 216, 217.)
2. `SelectionRange` representation: keep `pub anchor/head/goal_column/assoc` fields (mutable, any `i8` assoc, no bidi level, no undirectional flag) or move to cm6's private from/to/flags with constructors? Affects `assoc` defaults (cm6 auto ±1), `eq` semantics (cm6 ignores assoc by default; Ferrum `==` compares it) and the view's need for bidi level and undirectional ranges.
3. `EditorSelection::create(vec![])` returns a cursor at 0 instead of an error; out-of-range main index is clamped instead of preserved. Is a `Result`/panic intended to match cm6's RangeError? No verifiable reason found for the current behavior.
4. Ferrum `EditorSelection::cursor(pos)` returns an `EditorSelection` while cm6 `EditorSelection.cursor` returns a `SelectionRange`; the cm6 equivalent of the Ferrum function is `EditorSelection.single(pos)`. Keep the name (FERRUM-ONLY) or rename to avoid confusion when porting cm6 code that calls `EditorSelection.cursor(…)`?
5. `find_word_break` / `find_subword_break` / `categorize_char` live in ferrum-state while cm6 keeps group motion in view and subword motion in commands: FERRUM-ONLY or PLACEMENT DEVIATION (main session)?
6. Should `tabSize = 0` be rejected at facet level? cm6 returns NaN/odd values, Ferrum panics in `count_column`/`find_column`.
7. `goal_column` type: cm6 `number` (the view stores pixel offsets, possibly fractional) vs Ferrum `usize`; depends on D4 (layout unit).

## Unfinished

none
