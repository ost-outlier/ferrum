# C1 notes (text)

> What: chunk C1 decision evidence, cross-crate facts, oracle, open questions. Versions: cm6 `@codemirror/state` 6.7.6 (`CodeMirror6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`ferrum/crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `ferrum/docs/parity/README.md`; bare reproduction ids (R…, P…, S…) refer to `ferrum/docs/parity/state/raw/C1-api.md` § Reproductions.

Repro ids refer to `ferrum/docs/parity/state/raw/C1-api.md` `## Reproductions`.

## Decision evidence

### D1 (position unit)

Affected items: state.cm6.219 `Line.from`, .220 `Line.to`, .223 `Line.length`, .230 `Text.length`, .232 `lineAt`, .233 `line`, .234 `replace`, .236 `slice`, .237 `sliceString`, .225 `TextIterator.next` (skip), .240 `iterRange`; Ferrum-only conversion APIs state.fe.238 `len_chars`, .239 `len_bytes`, .242–.245 (`char_to_line`, `line_to_char`, `char_to_byte`, `byte_to_char`).

- cm6 counts UTF-16 code units everywhere in `text.ts` (`string.length` in `textLength` `CodeMirror6/state/src/text.ts:380`, `lineInner` `:177`, `sliceString` `:219`, cursor skips `:444-448`). Ferrum counts code points (`rope.len_chars()` `ferrum/crates/ferrum-state/src/text.rs:37`, `line_to_char` `:107`, `slice` `:145`, `:177`).
- Concrete values, `Text.of(["a😀b"])` (R9): length cm6 4 / code points 3 / UTF-8 bytes 6 (R20d `len_bytes`). `lineAt(2)`: cm6 `{0,4}` vs Ferrum `{0,3}`. `sliceString(1,3)`: cm6 `"😀"` vs Ferrum `slice_string(1,3)` `"😀b"`. `sliceString(1,2)`: cm6 returns a lone high surrogate `"\ud83d"`; code points cannot split the pair (Ferrum `"😀"`). `Text.of(["😀","x"]).line(2).from`: UTF-16 3 / code points 2 / UTF-8 5 (R9b; bytes from `char_to_byte(1)` style arithmetic: 4 bytes + 1 newline).
- Oracle (see below): BMP-only non-ASCII documents (é, 中) give 0 mismatches over 10,880 checks, because BMP code points are one UTF-16 unit each. Documents with U+1F600 give mismatches in every position-bearing check (length 314/320, replace 317/320, lineAt 1,904/1,920, line 1,861/1,920, sliceString/slice 902/1,920).
- cm6 can produce strings with lone surrogates (1,414 such strings in the 40-case mixed sample; 2,269 in the emoji sample), which Rust `String` cannot hold. Under the UTF-16 option, Ferrum would need a rule for positions inside a surrogate pair (clamp or reject); under code points or UTF-8 bytes, such cm6 test values have no representation.
- Ferrum's existing byte conversions: `byte_to_char` silently rounds a byte offset inside a code point down (R20d: byte 2 of `"a😀b"` → char 1); `char_to_byte`/`byte_to_char` clamp out-of-range input. Under the UTF-8-bytes option, every `Text` API would need a defined behavior for non-boundary offsets (cm6 has none because every UTF-16 offset is "valid").
- Options: UTF-16 — every cm6 test value in this file is reusable unchanged, including the surrogate-splitting behavior; requires a UTF-16 index over a UTF-8 rope (ropey 1.6.1 offers `Rope::char_to_utf16_cu` (`src/rope.rs:706`) and `Rope::utf16_cu_to_char` (`src/rope.rs:727`)). Code points (current) — all `test-text.ts` cases are ASCII, so no expected value in C1 tests changes; only the non-ASCII behaviors above differ. UTF-8 bytes — same for the ASCII tests; `Line.length`/`Text.length` become byte counts (R9: 6), and splitting inside a code point needs a rule.

### D2 (line breaks / Text model)

Affected items: state.cm6.218–222 (`Line` and fields), .229 `Text`, .230 `length`, .231 `lines`, .232 `lineAt`, .233 `line`, .234 `replace`, .235 `append`, .236 `slice`, .237 `sliceString`, .238 `eq`, .242 `toString`, .243 `toJSON`, .246 `Text.of`; Ferrum-only state.fe.236 `from_str`, .248 `line_by_index`, .356/.357 `From<&str>/From<String>`.

- ropey configuration: `ropey = "1.6"` (`ferrum/Cargo.toml:20`), used only by `ferrum/crates/ferrum-state/Cargo.toml:11` with default features; `cargo tree -e features -i ropey` shows `default` → `unicode_lines` → `cr_lines`, plus `simd`. Locked version 1.6.1 (`Cargo.lock:4432-4433`). ropey selects `str_indices::lines` under `unicode_lines` (`ropey-1.6.1/src/str_utils.rs:12-18`), whose break set is LF, VT, FF, CR, NEL, U+2028, U+2029 and CRLF (`str_indices-0.4.4/src/lines.rs:1-12`).
- cm6 `Text.of` does not split or validate its input lines (`CodeMirror6/state/src/text.ts:153-157`): `Text.of(["a\nb"])` is a 1-line document whose only line contains `"\n"` (R3); `Text.of(["a\r\nb"])` is 1 line of length 4 (R4). It throws only for an empty array (R1). Splitting with `DefaultSplit = /\r\n?|\n/` (`CodeMirror6/state/src/change.ts:3`) or the `lineSeparator` facet happens in `EditorState.toText` (`CodeMirror6/state/src/state.ts:184-185`), `EditorState.create` (`state.ts:246`), `replaceSelection` (`state.ts:130`) and `ChangeSet.of` (`change.ts:335`). Output reinserts `state.lineBreak` via `sliceDoc` (`state.ts:189-190`, `:286`). S-p3: `"a\r\nb"` and `"a\rb"` → length 3, 2 lines; `"a b"` and `"a\u0085b\u000bc\u000cd"` → 1 line (cm6 does not break on them).
- Ferrum `Text::of` joins with `"\n"` and re-parses (`ferrum/crates/ferrum-state/src/text.rs:86-93`); `from_str`/`From` keep the input raw (`text.rs:28-32`, `:218-228`). There is no normalization step anywhere in `text.rs`.
- Consequences measured:
  - Line count: R5 `["a b","c"]` cm6 2 / Ferrum 3; R6 one cm6 line → 5 Ferrum lines; R4 `["a\r\nb"]` 1 / 2.
  - `line_by_index` strips only `\r\n`, `\n`, `\r` (`text.rs:114-120`) although ropey also breaks on VT, FF, NEL, U+2028, U+2029: those lines keep the separator in `text` and `to` (R5 `(0,2,"a\u{2028}")`, R6), so `next.from == prev.to` (no gap) for them, `prev.to + 1` for LF and lone CR, `prev.to + 2` for CRLF (R6c).
  - Position inside a CRLF break exists in Ferrum (R6c: `line_at(2)` of `"a\r\nb"` → line 1 with `to = 1`); cm6 has no such position.
  - Edits can merge breaks: P1 (append of `"a\r"` and `"\nb"`), P3 (delete between `\r` and `\n`), P4 (slice cutting a CRLF yields a text ending in a lone CR that counts as a line break).
  - Output: `to_string`/`slice_string` return raw separators (R15d, R19); cm6 `toString` always uses `"\n"` and `sliceString` accepts `lineSep` (R15). Ferrum has no `lineSep` parameter, so `sliceDoc`/`lineBreak` (C5) cannot be built on `Text` without a post-pass.
  - `eq` differs for documents that cm6 would consider differently structured (R18).
- Oracle: with `\r` or U+2028 in line content (cm6 `Text.of` keeps them as content), Ferrum agrees on every string/length result but disagrees on `lines`, `line`, `lineAt`, `slice.lines` and line texts (cr sample: lines 314/320, line 1,833/1,920, lineAt 1,870/1,920; u2028 sample similar). Note this compares against cm6's `Text.of` (no splitting); against `toText`-normalized cm6 documents the length would also differ for CRLF/CR.
- Options (as framed in `ferrum-foundation.md` D2): (a) cm6 semantics — normalize on input (`\r\n?|\n` or `lineSeparator`), disable `unicode_lines`/`cr_lines` (`default-features = false`) so only LF breaks lines; then `line_by_index` stripping becomes a single `\n`, R3–R6/P1–P4 match cm6 for `toText`-style input, `Text::of` with `"\n"` inside an element still differs from cm6 (cm6 silently builds a malformed doc; whether to mimic, reject or split is a sub-decision). (b) Keep raw separators — every line API needs the full separator table, positions inside CRLF exist, and cm6 tests that use `\r\n` documents (C5 `lineSeparator`) need new expected values.

### Complexity / allocation (feeds rules.md §6.3 and D1/D2 implementation)

- `Text::line_by_index` (`text.rs:105-129`), reached from `line` and `line_at`, allocates the full line as a `String` on every call (`text.rs:113`) and then truncates it. cm6 `lineInner` (`text.ts:175-183`, `:259-267`) returns the stored leaf string. Measured (P7, release build, this machine): 100 × `line_at(5)` on a 1,000,000-char line = 8.40 ms in Ferrum vs 0.089 ms in cm6 (Node 22.23.3); on a 1-char line Ferrum 0.10 ms. `ferrum-commands` calls `line_by_index`/`line_at` per selection range (`ferrum/crates/ferrum-commands/src/commands.rs:547-642`, `:849-851`, `:1020`).
- `Text::of` builds a joined `String` of the entire document before creating the rope (`text.rs:87-92`); cm6 builds leaves directly (`text.ts:231-244`). Not measured.
- `Text` equality (derived) is always a full content comparison (`ropey-1.6.1/src/slice.rs:1794-1838`); cm6 `eq` skips identical subtrees (`text.ts:89`, `:320-331`). Not measured.
- `Text::replace` takes `&str`, so inserting an existing `Text` (e.g. from `ChangeSet` inverse/composition) requires materializing it; cm6 splices subtrees (`text.ts:57-60`). Verified in code (`text.rs:154`); C2 owns the call sites.

## Cross-crate facts

1. **Text iteration (`iter`, `iterRange`, `iterLines`, `TextIterator`).** cm6 consumers outside state: `CodeMirror6/commands/src/commands.ts:669` (`deleteTrailingWhitespace` uses `state.doc.iter()` and `lineBreak`), `CodeMirror6/language/src/isolate.ts:11` and `:100` (`text.iter()`, `doc.iter()`), `CodeMirror6/language/src/matchbrackets.ts:196` (`state.doc.iterRange`), `CodeMirror6/view/src/matchdecorator.ts:8` (`doc.iterRange`), `CodeMirror6/view/src/buildtile.ts:251` (`doc.iter()`, DOM-tile builder, likely not ported). Searched all crates (S1, S2 in ferrum/docs/parity/state/raw/C1-api.md): no iterator on `Text`; only route is `Text::as_rope()` (`ferrum/crates/ferrum-state/src/text.rs:195`), which no crate calls. What exists instead: Ferrum commands materialize the whole document with `state.doc().to_string()` (`ferrum/crates/ferrum-commands/src/commands.rs:78, 104, 129, 150, 175, 197, 219, 240, 261, 283, 305, 326, 663, 686`; `ferrum/crates/ferrum-commands/src/comment.rs:292`), and ferrum-language parses `state.doc().to_string()` (`ferrum/crates/ferrum-language/src/tree.rs:160`, `:177`; `ferrum/crates/ferrum-language/src/language.rs:44`). Capability owner per the graph: ferrum-state.
2. **`sliceString` with `lineSep` / `sliceDoc` / `lineBreak`.** cm6 `CodeMirror6/state/src/state.ts:189-190` (`sliceDoc` → `sliceString(from, to, this.lineBreak)`), used by commands for clipboard-like output. Ferrum `Text::slice_string` has no separator parameter (`text.rs:149`); searched `slice_doc`, `line_break`, `lineBreak` in all crates: absent (S2). C5 owns `sliceDoc`/`lineBreak`; this is the missing primitive under them.
3. **String-to-Text ingest.** cm6 splits inserted strings with `DefaultSplit`/`lineSeparator` (`change.ts:335`, `state.ts:130`, `state.ts:185`, `state.ts:246`); Ferrum `ChangeSpec::insert/replace` and `EditorState::replace_selection` convert with `impl Into<Text>` → raw `Text::from_str` (`ferrum/crates/ferrum-state/src/change.rs:261-273`, `ferrum/crates/ferrum-state/src/state.rs:245`). Every crate that inserts text (commands, view-gpui input) inherits D2 behavior from this. Owner: ferrum-state (C2/C5 rows).
4. **Tree shape (`Text.children`).** No cm6 consumer in view/language/commands uses `Text.children` (grep `\.children` in those `src/` dirs only hits view tiles, `CodeMirror6/view/src/cursor.ts:371-384`, `CodeMirror6/view/src/buildtile.ts:59, 602`, which are DOM tiles, not `Text`). Relevant only to state tests.
5. **`Line` allocation cost** is inherited by every Ferrum consumer of `line`/`line_at`/`line_by_index` (fact 1 lists commands call sites).

## Oracle

- Files: generator `ferrum/docs/parity/state/oracle/cm6-probes/probe-C1/oracle.ts` (cm6, run with `tsx probe-C1/oracle.ts <seed> <cases> <mode> <out.json>`); replayer `ferrum/docs/parity/state/oracle/ferrum-repro/repro-C1/src/bin/oracle.rs` (`cargo build --offline --bin oracle`, `./target/debug/oracle <file.json>`). Data in `(oracle data, not archived: regenerate from the seed recorded in `ferrum/docs/parity/state/oracle.md`)`.
- PRNG: mulberry32 in the generator; the Rust side only replays recorded inputs and compares against recorded cm6 outputs.
- Per case: initial doc `Text.of(lines)` (1–40 lines, every 10th case 1–300 lines; lines 0–29 chars), then 8 operations `replace(from, to, Text.of(ins))` with `from`, `to` ∈ [0, length+2] (30 % of cases with `from > to` kept), `ins` = `[""]` (30 %) or 1–4 lines of 0–9 chars. After each operation: `length`, `lines`, `toString`, line texts (`toJSON` vs Ferrum `line(n).text` for all n), and 6 probes each of `lineAt(pos)`, `line(n)`, `sliceString(a,b)`, `slice(a,b).toString()`, `slice(a,b).lines` with `a`, `b` ∈ [0, length+2] in either order. Ferrum receives `ins.join("\n")` as `&str`.
- Runs and results:

| mode (alphabet) | seed | cases | checks | mismatching categories |
|---|---|---|---|---|
| ascii (`abcdefgh xyz`) | 20261021 | 300 | 81,600 | 0 |
| bmp (`ab`, U+00E9, U+4E2D) | 11 | 40 | 10,880 | 0 |
| emoji (`ab`, U+1F600) | 11 | 40 | 10,880 | 9 (all categories; D1) |
| u2028 (`ab`, U+2028) | 11 | 40 | 10,880 | 5: line 1,816, lineAt 1,868, lines 314, slice.lines 849, line texts 314 (D2) |
| cr (`ab`, `\r`) | 11 | 40 | 10,880 | 5: line 1,833, lineAt 1,870, lines 314, slice.lines 843, line texts 314 (D2) |
| nonascii (all of the above) | 11 | 40 | 10,880 | 9 |

- Minimal mismatching cases (hand-reduced, verified by R-probes rather than the oracle files): emoji → R9 (`Text.of(["a😀b"]).length` 4 vs 3); u2028 → R5 (`lines` 2 vs 3); cr → P2/R6b (`Text.of(["a\r","b"]).line(1)` `{0,2,"a\r"}` vs `(0,1,"a")`).
- Lone surrogates produced by cm6 are replaced by U+FFFD in the JSON (serde_json rejects them); counts are printed by the generator (0 for ascii/bmp/u2028/cr, 2,269 for emoji, 1,414 for nonascii). This affects only the emoji/nonascii rows, which mismatch anyway.
- Not covered by the oracle: iterators, `toJSON`, `children` (no Ferrum counterpart), `append` (covered indirectly by replace at end), out-of-range `line`/`lineAt` (covered by R10–R13).

## Open questions

- Q1 (D2 sub-question): should Ferrum `Text::of` reproduce cm6's lack of validation (an element containing `"\n"` becomes one malformed line, R3), split such elements, or reject them? cm6 behavior is unspecified by its docs ("Create a Text instance for the given array of lines") and untested.
- Q2: `Text::of(&[])` returns an empty document where cm6 throws (R1). No verifiable reason found for the difference. HUMAN DECISION: panic like cm6, return `Result`, or keep.
- Q3 (disposition): `Text.children` (state.cm6.244) is public cm6 API whose only users are cm6's own tests (tree-shape assertions in test-text#04/#05/#06/#10). Is it required in Ferrum (then a tree API over ropey or a custom tree is needed), or should its disposition be revisited? Not decided here; status proposed `MISSING`.
- Q4: out-of-range contract. cm6 throws `RangeError` for `line`/`lineAt`; Ferrum `line`/`line_at` panic (no `# Panics` docs, no `Option` variant, contrary to `rules.md` §6.4), while `line_by_index`, `char_to_line`, `line_to_char`, `char_to_byte`, `byte_to_char`, `slice`, `slice_string`, `replace` clamp silently. `line_by_index`/`line_to_char` clamping where cm6's equivalent (`line`) throws has no verifiable reason (no comment) → HUMAN DECISION.
- Q5: `Line` mutability (pub fields, stored `length`): keep as a plain data struct (Rust-idiomatic) or make fields private with a computed `length()`? No verifiable reason found for the stored field.
- Q6: duplicate API surface — `len`/`len_chars`, `slice_string`/`slice_to_string`, inherent `from_str` vs `From<&str>`; and `as_rope` leaking `ropey::Rope` (third-party type, with `unicode_lines` semantics) into the public API with no caller. HUMAN DECISION on which to keep.
- Q7: cm6 test-text#13 has a bug (`start = 4150; end = 4160` overwrites the random range on every iteration, `CodeMirror6/state/test/test-text.ts:96`), so it tests one range 400 times. A faithful port would reproduce it; a corrected port would differ from cm6. Flag for the maintainer.

## Unfinished

none
