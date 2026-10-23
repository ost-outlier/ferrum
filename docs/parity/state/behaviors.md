# ferrum-state behaviors

> What: behaviors B1…Bn of every in-scope cm6 item, with Ferrum code/test verdicts. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`.

Merged from `docs/parity/state/raw/C1-behaviors.md` … `docs/parity/state/raw/C6-behaviors.md` (one section per non-`DROP` cm6 export, 262, plus the internal helper `makeCategorizer` read by C3). Behavior ids `state.<item>.B<n>` are stable. Reproduction ids inside a section refer to the same chunk's `docs/parity/state/raw/C<n>-api.md` `## Reproductions` (or the behaviors file's own section where present). No item in this crate is `CONTRACT`: `@codemirror/state` asks nothing of the DOM or of Lezer, so there is no contract question to record.

Totals: 948 behavior rows. Verdicts (first words of the verdict column): missing 311, differs 234, implemented+tested 213, implemented, untested 137, other 53.

## Chunk C1 (source: `docs/parity/state/raw/C1-behaviors.md`)


30 cm6 items owned (state.cm6.218–247), none DROP: 30 sections below. Repro ids (R*, P*, S-p3) refer to `docs/parity/state/raw/C1-api.md` `## Reproductions`. Test ids: cm6 `test-text#NN` (T1), Ferrum `test_text#NNN` (T2). Mapping of Ferrum test ids to cm6 cases: #001–#024 ↔ cm6 #01–#24 by number; #025/#026/#027 ↔ cm6 #25; #028/#029 ↔ cm6 #26; #030/#031/#032 ↔ cm6 #27/#28/#29. "tested" in a verdict means a Ferrum test asserts the behavior with the cm6 scenario's values or stronger (see `docs/parity/state/raw/C1-tests.md` for the per-test verdicts).

Shared helpers read (cm6, `mirrors/cm6/state/src/text.ts`): `Tree` consts `:1-7`; `Open` flags `:10`; `TextLeaf` `:166-245` (`lines` `:171`, `children` `:173`, `lineInner` `:175`, `decompose` `:185`, `replace` `:202`, `sliceString` `:211`, `flatten` `:225`, `scanIdentical` `:229`, `split` `:231`); `TextNode` `:251-374` (`lines` ctor `:252-257`, `lineInner` `:259`, `decompose` `:269`, `replace` `:281`, `sliceString` `:304`, `flatten` `:316`, `scanIdentical` `:320`, `from` `:333`); `Text.empty` `:376`; `textLength` `:378`; `appendText` `:384`; `sliceText` `:398`; `RawTextCursor` `:402-473`; `PartialTextCursor` `:475-517`; `LineCursor` `:519-552`; iterator protocol `:554-558`; `Line` `:562-578`; `clip` `:580-583`.
Ferrum code read: `crates/ferrum-state/src/text.rs:1-243` (whole file); ropey 1.6.1 `src/str_utils.rs:1-100` (line-break selection by feature), `src/rope.rs:821` (`line`), `:1851` (`From<RopeSlice>`), `:1997` (`PartialEq`), `src/slice.rs:1794` (`RopeSlice` eq); str_indices 0.4.4 `src/lines.rs:1-12` (break set: LF, VT, FF, CR, NEL, U+2028, U+2029, CRLF).

### state.cm6.218 `Line` — cm6 `mirrors/cm6/state/src/text.ts:562`
Helpers read: `Line` constructor `text.ts:564-574`; producers `TextLeaf.lineInner` `text.ts:175-183`.
| id | behavior (branch/guard/edge case/side effect) | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.B1 | `Line` is an immutable value (all fields `readonly`), constructor `@internal`: only `Text.line`/`lineAt` create one | `text.ts:563-574` | `crates/ferrum-state/src/text.rs:231-243` (`pub struct`, all `pub` fields, constructible, mutable) | none | none | differs: Ferrum `Line` can be built and mutated by any caller (R22) |
| state.Line.B2 | invariant `length == to - from` always holds (getter) | `text.ts:577` | `text.rs:240` stored field | none | none | differs: invariant can be broken by assignment (R22 `(99, 2)`); untested |
Proposed status: PORTED, DEVIATION because the value type exists with the same fields but is mutable/constructible and stores `length` instead of computing it; field-level deviations in 219–223.

### state.cm6.219 `Line.from` — cm6 `mirrors/cm6/state/src/text.ts:566`
Helpers read: `TextLeaf.lineInner` `text.ts:175-183` (`offset`), `TextNode.lineInner` `text.ts:259-267` (`offset = end + 1`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.from.B1 | start position of the line, in UTF-16 code units from document start | `text.ts:177-179` | `text.rs:107` (`rope.line_to_char`, code points) | test-text#25, #26 | test_text#025, #028 | implemented+tested for ASCII; differs for astral chars (D1): R9b cm6 3 \| ferrum 2 |
| state.Line.from.B2 | for line n > 1, `from = previous.to + 1` (every break is exactly one position, never part of a line) | `text.ts:180`, `text.ts:264` | `text.rs:107` | test-text#25 (`(i-1)*101`) | test_text#025 | differs (D2): CRLF gives `prev.to + 2` (R6c `(0,1)`→`(3,4)`), U+2028/U+2029/NEL/VT/FF give `prev.to + 0` because the separator is inside the previous line (R5, R6); tested only for `\n` |
Proposed status: PORTED, DEVIATION because the unit is code points and the inter-line gap depends on the raw separator.

### state.cm6.220 `Line.to` — cm6 `mirrors/cm6/state/src/text.ts:569`
Helpers read: `TextLeaf.lineInner` `text.ts:177` (`end = offset + string.length`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.to.B1 | position at the end of the line content, before the break (or document end for the last line) | `text.ts:177-179` | `text.rs:109-125` (`from + len_chars` minus stripped `\r\n`/`\n`/`\r`) | test-text#25, #26 | test_text#025, #028 | implemented+tested for `\n`; D1 unit |
| state.Line.to.B2 | `to` never includes a separator character | `text.ts:177` (lines stored without separators) | `text.rs:114-120` strips only `\r\n`, `\n`, `\r` | none | none | differs (D2): U+2028, U+2029, U+0085, VT, FF are ropey breaks but are not stripped, so `to` points after the separator (R5 `(0,2,"a\u{2028}")`, R6) |
Proposed status: PORTED, DEVIATION because of D1 and the incomplete separator stripping.

### state.cm6.221 `Line.number` — cm6 `mirrors/cm6/state/src/text.ts:571`
Helpers read: `TextLeaf.lineInner` `text.ts:178-181`, `TextNode.lineInner` `text.ts:261-265`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.number.B1 | 1-based line number | `text.ts:178-181` | `text.rs:123` (`line_idx + 1`) | test-text#25, #26 | test_text#025, #028 | implemented+tested |
| state.Line.number.B2 | numbering counts only the line structure of the `Text` (separators inside a `Text.of` string never start a line) | `text.ts:171`, `text.ts:256` | `text.rs:137` → ropey `char_to_line` (unicode_lines) | none | none | differs (D2): R6 `line_at(8).number` → cm6 1 \| ferrum 5 |
Proposed status: PORTED, DEVIATION because line numbering follows ropey's break set.

### state.cm6.222 `Line.text` — cm6 `mirrors/cm6/state/src/text.ts:573`
Helpers read: `TextLeaf.lineInner` `text.ts:177-179` (`string = this.text[i]`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.text.B1 | the line's content without break | `text.ts:177-179` | `text.rs:113-120` | test-text#25, #26 | test_text#023, #025, #028 | implemented+tested for `\n`; differs (D2) for U+2028/U+2029/NEL/VT/FF (kept in `text`, R5, R6) |
| state.Line.text.B2 | no copy: the stored leaf string is returned (allocation-free apart from the `Line` object) | `text.ts:177-179` | `text.rs:113` `line_slice.to_string()` per call | none | none | differs (allocation): O(line length) `String` allocation per `line`/`line_at`/`line_by_index` call; measured P7: 84 µs/call on a 1,000,000-char line vs 0.9 µs in cm6 |
Proposed status: PORTED, DEVIATION because of separator handling and per-call allocation.

### state.cm6.223 `Line.length` — cm6 `mirrors/cm6/state/src/text.ts:577`
Helpers read: none beyond `Line`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.length.B1 | `to - from`, UTF-16 units, excluding the break | `text.ts:577` | `text.rs:126` (`text_len`, code points) | none | none | implemented, untested; D1 (R9 cm6 4 \| ferrum 3) |
| state.Line.length.B2 | computed on access, cannot diverge from `to - from` | `text.ts:577` | `text.rs:240` stored `pub` field | none | none | differs (R22) |
Proposed status: PORTED, DEVIATION (untested: B1; differs: B2).

### state.cm6.224 `TextIterator` — cm6 `mirrors/cm6/state/src/text.ts:15`
Helpers read: `RawTextCursor` `text.ts:402-473`, `PartialTextCursor` `text.ts:475-517`, `LineCursor` `text.ts:519-552`, iterator protocol `text.ts:554-558`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.B1 | one interface for the three cursor kinds (raw, range, lines) with `next(skip)`, `value`, `done`, `lineBreak` | `text.ts:15-28` | none | test-text#17–#23 | none | missing |
| state.TextIterator.B2 | the iterator is itself `Iterable` (returns `this` from `[Symbol.iterator]`), so `for…of` works on cursors | `text.ts:556-557` | none | none | none | missing |
| state.TextIterator.B3 | `next()` mutates and returns the same object (no allocation per step) | `text.ts:19`, `text.ts:463-469` | none | test-text#19 | none | missing |
Proposed status: MISSING (searched: see docs/parity/state/raw/C1-api.md state.cm6.224).

### state.cm6.225 `TextIterator.next` — cm6 `mirrors/cm6/state/src/text.ts:19`
Helpers read: `RawTextCursor.nextInner` `text.ts:417-461`, `RawTextCursor.next` `text.ts:463-469`, `PartialTextCursor.next/nextInner` `text.ts:490-511`, `LineCursor.next` `text.ts:526-546`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.next.B1 | `next()` advances to the next line string or line break | `text.ts:417-461` | none | test-text#17 | none | missing |
| state.TextIterator.next.B2 | positive `skip` skips that many positions; may return a line suffix (forward) or prefix (reverse) | `text.ts:440-448` | none | test-text#19 | none | missing |
| state.TextIterator.next.B3 | a skipped line break counts as one position | `text.ts:432-439` | none | test-text#19 | none | missing |
| state.TextIterator.next.B4 | negative `skip`: moves back `-skip` positions, then re-reads the current string (R24 `next(-12)` → `"ne"`, `next(-1000)` → `"one"`) | `text.ts:463-468` | none | test-text#19 | none | missing |
| state.TextIterator.next.B5 | skipping past the end yields `done = true`, `value = ""` (R24c) | `text.ts:423-428` | none | none | none | missing |
| state.TextIterator.next.B6 | skip units are UTF-16 code units (D1) | `text.ts:444-448` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.226 `TextIterator.value` — cm6 `mirrors/cm6/state/src/text.ts:22`
Helpers read: `RawTextCursor` fields `text.ts:403-405`, `PartialTextCursor.nextInner` `text.ts:502`, `LineCursor.next` `text.ts:526-546`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.value.B1 | `""` before the first `next()` (R23c) | `text.ts:405` | none | none | none | missing |
| state.TextIterator.value.B2 | `""` at the end; stays `""` after further `next()` (R23d) | `text.ts:425-427` | none | test-text#17 (loop end) | none | missing |
| state.TextIterator.value.B3 | a line break is reported as the literal `"\n"`, independent of `lineSeparator` | `text.ts:436` | none | test-text#17 | none | missing |
Proposed status: MISSING.

### state.cm6.227 `TextIterator.done` — cm6 `mirrors/cm6/state/src/text.ts:25`
Helpers read: `RawTextCursor.nextInner` `text.ts:418-428`, `PartialTextCursor.nextInner` `text.ts:491-503`, `LineCursor.next` `text.ts:528-533`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.done.B1 | `false` until the end is reached, then `true` | `text.ts:418`, `text.ts:425` | none | test-text#17, #18, #20, #21, #23 | none | missing |
| state.TextIterator.done.B2 | for a range cursor, `done` becomes true when the produced value is empty (`done = !this.value`) | `text.ts:503` | none | test-text#20, #21 | none | missing |
Proposed status: MISSING.

### state.cm6.228 `TextIterator.lineBreak` — cm6 `mirrors/cm6/state/src/text.ts:27`
Helpers read: `RawTextCursor.nextInner` `text.ts:432-438`, `PartialTextCursor.lineBreak` `text.ts:513`, `LineCursor.lineBreak` `text.ts:548`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.lineBreak.B1 | raw cursor: `true` exactly when the current string is a break | `text.ts:435` | none | test-text#17 | none | missing |
| state.TextIterator.lineBreak.B2 | range cursor: `true` only if the underlying cursor is on a break and the clipped value is non-empty | `text.ts:513` | none | none | none | missing |
| state.TextIterator.lineBreak.B3 | line cursor: always `false` (R26h) | `text.ts:548` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.229 `Text` — cm6 `mirrors/cm6/state/src/text.ts:31`
Helpers read: all shared helpers listed at the top.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.B1 | persistent immutable value; every operation returns a new `Text` and leaves the receiver unchanged | `text.ts:54-74` | `text.rs:154-192` (clone rope, mutate copy) | test-text#14 (reuse of `doc` after replaces) | test_text#014 | implemented+tested |
| state.Text.B2 | B-tree of leaves holding ≤ `Tree.Branch` (32) lines each, with structural sharing between versions | `text.ts:1-7`, `text.ts:166-374` | ropey `Rope` (`Arc` nodes, byte-sized chunks; not line-based) | test-text#04, #05, #06, #10 | none (shape not observable) | differs: shape is not exposed (see state.cm6.244) |
| state.Text.B3 | stores lines without separators; a break is one position | `text.ts:163-165`, `text.ts:378-382` | `text.rs:9-11` raw rope with separators | none directly | none | differs (D2): R4, R5, R6, R6c |
| state.Text.B4 | iterable (`Symbol.iterator` → `iter()`) | `text.ts:555` | none | none | none | missing (state.cm6.245) |
Proposed status: PORTED, DEVIATION because the type and its core edit/query operations exist, but the text model (D2), unit (D1), tree API, iterators and JSON differ or are missing.

### state.cm6.230 `Text.length` — cm6 `mirrors/cm6/state/src/text.ts:33`
Helpers read: `textLength` `text.ts:378-382`, `TextLeaf` ctor `text.ts:167`, `TextNode.from` length default `text.ts:333`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.length.B1 | length = sum of line lengths + (lines − 1) | `text.ts:378-382` | `text.rs:36-38` (`len_chars`) | test-text#03 | test_text#003 | implemented+tested (ASCII) |
| state.Text.length.B2 | unit: UTF-16 code units (astral char = 2) | `text.ts:380` (`line.length`) | `text.rs:37` code points | none | none | differs (D1): R9 cm6 4 \| ferrum 3 |
| state.Text.length.B3 | a line break counts 1 regardless of the original separator (after `toText`) | `mirrors/cm6/state/src/state.ts:184-185` + `text.ts:380` | `text.rs:37` counts raw separator chars | none | none | differs (D2): `"a\r\nb"` → cm6 3 (S-p3) \| ferrum 4 (R6c) |
Proposed status: PORTED, DEVIATION (D1, D2).

### state.cm6.231 `Text.lines` — cm6 `mirrors/cm6/state/src/text.ts:35`
Helpers read: `TextLeaf.lines` `text.ts:171`, `TextNode` ctor `text.ts:252-257`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.lines.B1 | always ≥ 1 (empty doc has 1 line) | `text.ts:34`, `text.ts:376` | `text.rs:52-54` (ropey `len_lines` ≥ 1) | none | none (test_text#004 asserts 2000, not the empty case) | implemented, untested (R8 both 1) |
| state.Text.lines.B2 | equals the number of stored lines (`Text.of(a).lines == a.length`) | `text.ts:171`, `text.ts:256` | `text.rs:53` | test-text#23, #25 (via `doc.lines + 1`) | test_text#004, #023, #025 | implemented+tested for `\n`-free line arrays |
| state.Text.lines.B3 | characters inside a line (`\r`, U+2028, U+2029, NEL, VT, FF) never add lines | `text.ts:153-157` (no splitting) | ropey `unicode_lines` (str_indices `src/lines.rs:1-12`) | none | none | differs (D2): R5 2 vs 3, R6 1 vs 5, R4 1 vs 2 |
Proposed status: PORTED, DEVIATION (D2; B1 untested).

### state.cm6.232 `Text.lineAt` — cm6 `mirrors/cm6/state/src/text.ts:38`
Helpers read: `TextLeaf.lineInner` `text.ts:175-183`, `TextNode.lineInner` `text.ts:259-267`, `Line` `text.ts:562-578`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.lineAt.B1 | `pos < 0` → `RangeError "Invalid position <pos> in document of length <len>"` | `text.ts:39-40` | none (`usize` makes it unrepresentable) | test-text#26 (`lineAt(-10)`) | none | not applicable in Ferrum's type (R12) |
| state.Text.lineAt.B2 | `pos > length` → `RangeError` with the message above | `text.ts:39-40` | `text.rs:134-136` panic `"Invalid position: <pos>"` | test-text#26 | test_text#029 | differs: panic instead of exception, different message (matches `/Invalid position/`), no `# Panics` doc, no non-panicking variant (R13) |
| state.Text.lineAt.B3 | a position at a line's end (just before the break) belongs to that line; the next position belongs to the next line | `text.ts:178` (`end >= target`) | `text.rs:137` (ropey `char_to_line`) | test-text#26 | test_text#028 | implemented+tested (R14 equal) |
| state.Text.lineAt.B4 | `pos == length` → last line | `text.ts:178` | `text.rs:134` (allowed), `text.rs:63` | test-text#26 (loop stops before length) | none | implemented, untested (R13b equal) |
| state.Text.lineAt.B5 | returned `Line` has from/to/number/text of that line | `text.ts:179` | `text.rs:105-129` | test-text#26 | test_text#028 | implemented+tested (ASCII) |
| state.Text.lineAt.B6 | cost: one tree descent, no copy of the line content | `text.ts:259-267`, `text.ts:175-183` | `text.rs:137-138` → `line_by_index` allocates the line (`text.rs:113`) | none | none | differs (allocation, P7) |
| state.Text.lineAt.B7 | `pos` is a UTF-16 offset (D1) | `text.ts:177` | code points | none | none | differs (R9, R9b) |
| state.Text.lineAt.B8 | there is no position "inside" a line break (each break is one position) | `text.ts:163-165` | CRLF has an inner position: `from_str("a\r\nb").line_at(2)` → line 1 `(0,1)` with `pos > to` (R6c) | none | none | differs (D2) |
Proposed status: PORTED, DEVIATION because of the panic contract (B2), D1 (B7), D2 (B8), allocation (B6), and B4 untested.

### state.cm6.233 `Text.line` — cm6 `mirrors/cm6/state/src/text.ts:45`
Helpers read: `TextLeaf.lineInner` `text.ts:175-183`, `TextNode.lineInner` `text.ts:259-267`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.line.B1 | `n < 1` → `RangeError "Invalid line number <n> in <lines>-line document"` | `text.ts:46` | `text.rs:98-100` panic `"Invalid line: <n>"` | test-text#25 | test_text#026 | differs: panic, different message (matches `/Invalid line/`) (R10) |
| state.Text.line.B2 | `n > lines` → same `RangeError` | `text.ts:46` | `text.rs:98-100` | test-text#25 | test_text#027 | differs as B1 (R11) |
| state.Text.line.B3 | returns line n (from/to/number/text) | `text.ts:47` | `text.rs:101` → `text.rs:105-129` | test-text#25 | test_text#025 | implemented+tested (ASCII) |
| state.Text.line.B4 | which line is "n" depends only on the `Text` line structure | `text.ts:178`, `text.ts:262` | ropey line index | none | none | differs (D2): R5 `line(2)` → cm6 `"c"` \| ferrum `"b"` |
| state.Text.line.B5 | cost: one tree descent, no copy | `text.ts:259-267` | `text.rs:113` allocation | none | none | differs (allocation, P7) |
| state.Text.line.B6 | positions in UTF-16 | `text.ts:177` | code points | none | none | differs (D1, R9b) |
Proposed status: PORTED, DEVIATION (panic contract, D1, D2, allocation).

### state.cm6.234 `Text.replace` — cm6 `mirrors/cm6/state/src/text.ts:54`
Helpers read: `clip` `text.ts:580-583`, `Text.replace` `text.ts:54-61`, `TextLeaf.replace` `text.ts:202-209`, `TextNode.replace` `text.ts:281-302`, `decompose` `text.ts:185-200`, `text.ts:269-279`, `appendText` `text.ts:384-396`, `sliceText` `text.ts:398-400`, `TextLeaf.split` `text.ts:231-244`, `TextNode.from` `text.ts:333-373`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.replace.B1 | clip: `from` clamped to `[0, length]`, `to` to `[from, length]` | `text.ts:55`, `text.ts:580-583` | `text.rs:156-157` | test-text#08 (`to = length`) | test_text#008 (`to = length + 1`) | implemented+tested for `to > length`; `from > to` (R17) and `from > length` (R17b) implemented, untested; negative unrepresentable (R17c) |
| state.Text.replace.B2 | inserted `Text`'s first line joins the text before `from`, its last line joins the text after `to` | `text.ts:57-59`, `text.ts:205` | `text.rs:159-164` (string insert) | test-text#01 | test_text#001 | implemented+tested |
| state.Text.replace.B3 | deletion (empty insert) at start, end, whole doc, child-node start | `text.ts:57-60` | `text.rs:159-161` | test-text#06, #07, #08, #09, #27 | test_text#006, #007, #008, #009, #030 | implemented+tested |
| state.Text.replace.B4 | content preserved under many random edits | `text.ts:281-302` | `text.rs:154-167` | test-text#12 (200 random edits, `Math.random`) | test_text#012 (50 edits, LCG seed 42) | implemented+tested (weaker iteration count); ASCII oracle 0 mismatches (2,400 replaces) |
| state.Text.replace.B5 | tree stays balanced: depth ≤ 2 after repeated inserts, collapses to a leaf after deleting nearly everything | `text.ts:291-297`, `text.ts:333-373` | ropey internal balancing (not observable) | test-text#05, #06 | none (depth assertions dropped) | not observable in Ferrum (no tree API) |
| state.Text.replace.B6 | structural sharing: a change inside one child only rebuilds that child (fast path) | `text.ts:283-298` | ropey `Rope::clone` + `insert/remove` (shares untouched nodes) | none | none | implemented differently, not observable, untested |
| state.Text.replace.B7 | insertion argument is a `Text`, so the inserted lines are already split; nothing is re-parsed | `text.ts:54` | `text.rs:154` takes `&str`, parsed with ropey semantics | none | none | differs: inserting a `Text` needs `to_string()` + re-parse; D2 parsing applies (P6: `"\u{2028}y"` adds a line) |
| state.Text.replace.B8 | an edit never merges two breaks or a content `\r` with a following break | `text.ts:384-396` (line arrays) | ropey raw string | none | none | differs (D2): deleting between `\r` and `\n` or appending `\n` after `\r` forms one CRLF break (P1, P3) |
| state.Text.replace.B9 | positions in UTF-16 | `text.ts:580-583` | code points | none | none | differs (D1); BMP-only oracle: 0 mismatches; emoji sample (seed 11): 317 of 320 replace results differ |
Proposed status: PORTED, DEVIATION because the argument type differs (B7), D1/D2 (B8, B9), tree properties are not observable (B5), and B1 sub-cases are untested.

### state.cm6.235 `Text.append` — cm6 `mirrors/cm6/state/src/text.ts:64`
Helpers read: `Text.replace` and its helpers (state.cm6.234).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.append.B1 | `replace(length, length, other)`: last line of `this` joins the first line of `other`; `lines = a.lines + b.lines − 1` | `text.ts:64-66` | `text.rs:189-191` (`Rope::append`) | test-text#02 | test_text#002 | implemented+tested |
| state.Text.append.B2 | empty operand yields a value equal to the other operand | `text.ts:58` | `text.rs:183-188` (early returns) | none | none | implemented, untested |
| state.Text.append.B3 | line count is exactly `a.lines + b.lines − 1` for every input | `text.ts:384-396` | ropey | none | none | differs (D2): P1 `["a\r"]` + `["","b"]` → cm6 1 + 2 − 1 = 2 \| ferrum 2 + 2 − 2 = 2 (CR/LF merged) |
Proposed status: PORTED, DEVIATION (B2 untested, B3 D2).

### state.cm6.236 `Text.slice` — cm6 `mirrors/cm6/state/src/text.ts:69`
Helpers read: `clip` `text.ts:580-583`, `decompose` `text.ts:185-200`, `text.ts:269-279`, `sliceText` `text.ts:398`, `TextNode.from` `text.ts:333`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.slice.B1 | clip boundaries: negative, beyond length, `from > to` → empty | `text.ts:70`, `text.ts:580-583` | `text.rs:171-175` | test-text#29 (5 cases) | test_text#032 (3 cases: `0..0`, empty `0..10`, empty `1000..1100`) | implemented; `slice(5,0)` untested in Ferrum (R16b equal), negatives unrepresentable (R16c) |
| state.Text.slice.B2 | `to` defaults to `length` | `text.ts:69` | none (`Range` required) | none | none | differs (R16d) |
| state.Text.slice.B3 | returned `Text` content equals `toString().slice(from,to)` | `text.ts:71-73` | `text.rs:176-178` | test-text#13, #28, #10 | none (Ferrum #010/#013/#031 call `slice_string`) | implemented, untested; ASCII oracle 0 mismatches (14,400 slices) |
| state.Text.slice.B4 | returned `Text` keeps the line structure (`lines` of the slice) | `text.ts:72-73` | `text.rs:177` | none | none | implemented, untested (R16e equal); oracle `slice.lines` 0 mismatches ASCII |
| state.Text.slice.B5 | a slice can never cut a line break in half | `text.ts:163-165` | ropey char slice | none | none | differs (D2): P4 `from_str("a\r\nb").slice(0..2)` → `"a\r"` with 2 lines |
| state.Text.slice.B6 | positions in UTF-16; a slice may cut a surrogate pair | `text.ts:580-583`, `text.ts:398` | code points | none | none | differs (D1) |
Proposed status: PORTED, DEVIATION (B3/B4 untested, B2 missing default, B5 D2, B6 D1).

### state.cm6.237 `Text.sliceString` — cm6 `mirrors/cm6/state/src/text.ts:77`
Helpers read: `TextLeaf.sliceString` `text.ts:211-223`, `TextNode.sliceString` `text.ts:304-314`, `clip` `text.ts:580-583`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.sliceString.B1 | clip boundaries | `text.ts:212`, `text.ts:305` | `text.rs:143-145` | test-text#28 (random in-range) | test_text#031 (fixed in-range) | implemented+tested in range; out-of-range implemented, untested (R16 equal) |
| state.Text.sliceString.B2 | `to` defaults to `length` | `text.ts:211`, `text.ts:304` | none | none | none | differs (R15b) |
| state.Text.sliceString.B3 | `lineSep` argument (default `"\n"`) is inserted for every break in range | `text.ts:216-217`, `text.ts:309` | none (no parameter) | none | none | missing (R15, R15c); used by `EditorState.sliceDoc` (`mirrors/cm6/state/src/state.ts:189-190`) |
| state.Text.sliceString.B4 | substring content | `text.ts:218-219` | `text.rs:145` | test-text#28 | test_text#010, #013, #019, #020, #021, #022, #031 | implemented+tested (ASCII) |
| state.Text.sliceString.B5 | UTF-16 offsets; can return a lone surrogate | `text.ts:219` | code points (cannot split a code point) | none | none | differs (D1): R9 `(1,2)` → `"\ud83d"` vs `"😀"` |
| state.Text.sliceString.B6 | breaks are always output as `lineSep`, never as the original separator | `text.ts:217` | `text.rs:145` raw rope content | none | none | differs (D2): R15d raw `"\r\n"` returned |
Proposed status: PORTED, DEVIATION (B2/B3 missing, B5 D1, B6 D2).

### state.cm6.238 `Text.eq` — cm6 `mirrors/cm6/state/src/text.ts:86`
Helpers read: `TextNode.scanIdentical` `text.ts:320-331`, `TextLeaf.scanIdentical` `text.ts:229`, `RawTextCursor` `text.ts:402-469`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.eq.B1 | identical object → `true` | `text.ts:87` | ropey slice eq (`slice.rs:1794`) compares content | test-text#14 (`doc.eq(doc)`) | test_text#014 | implemented+tested (value) |
| state.Text.eq.B2 | different `length` or `lines` → `false` | `text.ts:88` | byte-length check `slice.rs:1796` | test-text#14 | test_text#014 | implemented+tested |
| state.Text.eq.B3 | content compared line by line, breaks included | `text.ts:89-98` | chunk byte compare `slice.rs:1800-1835` | test-text#14, #16 | test_text#014, #016 | implemented+tested |
| state.Text.eq.B4 | equal content with different tree shape → `true` | `text.ts:89-98` | content compare | test-text#15 | test_text#015 | implemented+tested |
| state.Text.eq.B5 | cost: skips shared subtrees at both ends (`scanIdentical`), so comparing two versions that share structure is proportional to the differing region | `text.ts:89`, `text.ts:320-331` | always full O(n) comparison after the length check | none | none | differs (complexity, verified in code; not measured) |
| state.Text.eq.B6 | `lines` participates: `Text.of(["a","b"])` ≠ `Text.of(["a\nb"])` | `text.ts:88` | content only | none | none | differs (D2): R18 cm6 `false` \| ferrum `true` |
Proposed status: PORTED, DEVIATION (B5 complexity, B6 D2); the derived-`PartialEq` form itself would be RUST-IDIOMATIC.

### state.cm6.239 `Text.iter` — cm6 `mirrors/cm6/state/src/text.ts:104`
Helpers read: `RawTextCursor` `text.ts:402-473`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.iter.B1 | `dir = 1` (default): yields lines and breaks in order | `text.ts:104`, `text.ts:412-414` | none | test-text#17 | none | missing |
| state.Text.iter.B2 | `dir = -1`: yields them in reverse | `text.ts:414`, `text.ts:442` | none | test-text#18 | none | missing |
| state.Text.iter.B3 | breaks are separate `"\n"` items with `lineBreak = true`; line items never contain `"\n"` | `text.ts:432-438` | none | test-text#17 | none | missing |
| state.Text.iter.B4 | empty lines yield no item (two consecutive breaks) (R23) | `text.ts:440-448` (`next.length > max(0, skip)`) | none | none | none | missing |
| state.Text.iter.B5 | supports skip/negative skip (state.cm6.225) | `text.ts:463-469` | none | test-text#19 | none | missing |
Proposed status: MISSING.

### state.cm6.240 `Text.iterRange` — cm6 `mirrors/cm6/state/src/text.ts:108`
Helpers read: `PartialTextCursor` `text.ts:475-517`, `RawTextCursor` `text.ts:402-473`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.iterRange.B1 | `from < to`: forward over `[from, to)` only, clipping the first/last string | `text.ts:483-503` | none | test-text#20, #22 | none | missing |
| state.Text.iterRange.B2 | `from > to`: reverse iteration over `[to, from)` | `text.ts:484-487` | none | test-text#21 | none | missing |
| state.Text.iterRange.B3 | `to` defaults to `length` (R25d) | `text.ts:108` | none | none | none | missing |
| state.Text.iterRange.B4 | `to > length` is effectively clipped (R25f) | `text.ts:491`, `text.ts:500-503` | none | none | none | missing |
| state.Text.iterRange.B5 | empty range → done on first `next()` (R25e) | `text.ts:491-495` | none | none | none | missing |
| state.Text.iterRange.B6 | a break inside the range is reported with `lineBreak = true` (R25c) | `text.ts:513` | none | none | none | missing |
| state.Text.iterRange.B7 | positive skip clamped to the range end; negative skip clamped to the range start | `text.ts:507-510` | none | none | none | missing (R25g, R25h) |
| state.Text.iterRange.B8 | range positions in UTF-16 (D1) | `text.ts:496-502` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.241 `Text.iterLines` — cm6 `mirrors/cm6/state/src/text.ts:115`
Helpers read: `LineCursor` `text.ts:519-552`, `Text.line` `text.ts:45`, `iterRange` `text.ts:108`, `iter` `text.ts:104`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.iterLines.B1 | no arguments: every line, without breaks | `text.ts:117-118` | none | test-text#23 (`get()`) | none (test_text#023 reads `line(n).text`) | missing |
| state.Text.iterLines.B2 | `from` only: from that line to the end | `text.ts:120` | none | test-text#23 (`get(3)`) | none | missing |
| state.Text.iterLines.B3 | `from`, `to` are 1-based, `to` exclusive; `to == lines + 1` means to the end | `text.ts:121-122` | none | test-text#23 (`get(1, lines+1)`, `get(2,3)`, `get(1,5)`) | none | missing |
| state.Text.iterLines.B4 | empty lines yield `""` (R26) | `text.ts:534-537` | none | test-text#23 | none | missing |
| state.Text.iterLines.B5 | `to <= from` yields a single `""` (R26c) | `text.ts:122` (`Math.max(start, …)`), `text.ts:528-530` | none | test-text#23 (`get(2,1)` → `""`) | none | missing |
| state.Text.iterLines.B6 | invalid `from` or `to − 1` → `RangeError` from `line()` (R26e, R26f) | `text.ts:121-122` | none | none | none | missing |
| state.Text.iterLines.B7 | `lineBreak` is always `false` (R26h) | `text.ts:548` | none | none | none | missing |
| state.Text.iterLines.B8 | empty document yields one `""` (R26g); trailing empty line is yielded (R26i) | `text.ts:528-530` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.242 `Text.toString` — cm6 `mirrors/cm6/state/src/text.ts:132`
Helpers read: `sliceString` `text.ts:211-223`, `text.ts:304-314`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.toString.B1 | whole document with lines joined by `"\n"` | `text.ts:132` | `text.rs:200-204` (`write!(f, "{}", rope)`) | test-text#01, #02, #05–#09, #11, #12, #27 | test_text#001, #002, #005–#012, #017, #018, #030 | implemented+tested for `\n` documents |
| state.Text.toString.B2 | always `"\n"`, never the document's original separator | `text.ts:132` → `text.ts:211` default `lineSep` | raw rope | none | none | differs (D2): R19 `"ab\r\n\r\nc"` returned raw |
Proposed status: PORTED, DEVIATION (B2 D2).

### state.cm6.243 `Text.toJSON` — cm6 `mirrors/cm6/state/src/text.ts:136`
Helpers read: `TextLeaf.flatten` `text.ts:225-227`, `TextNode.flatten` `text.ts:316-318`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.toJSON.B1 | returns the array of lines (no separators) | `text.ts:136-140` | none | test-text#24 | none | missing (R19) |
| state.Text.toJSON.B2 | `Text.of(t.toJSON()).eq(t)` round-trips | `text.ts:135` (doc), `text.ts:153` | none | test-text#24 | none (test_text#024 compares `Text::of(&lines)` with a second `Text::of(&lines)`) | missing |
| state.Text.toJSON.B3 | empty document → `[""]` (R8) | `text.ts:376` | none | none | none | missing |
Proposed status: MISSING (searched: `to_json`, `toJSON`, `serde`, `Serialize` in all crates).

### state.cm6.244 `Text.children` — cm6 `mirrors/cm6/state/src/text.ts:147`
Helpers read: `TextLeaf.children` `text.ts:173`, `TextNode` ctor `text.ts:254`, `TextNode.from` `text.ts:333-373`, `TextLeaf.split` `text.ts:231-244`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.children.B1 | leaf → `null` (R20b) | `text.ts:173` | none | test-text#06 (depth 1) | none | missing |
| state.Text.children.B2 | branch → array of child `Text`s with implied breaks between them | `text.ts:247-254` | none | test-text#10 (`children[0].length`) | none | missing |
| state.Text.children.B3 | loading 2000 lines gives depth ≤ 2 | `text.ts:156`, `text.ts:333-373` | none | test-text#04 | none (test_text#004 asserts `len_lines() == 2000`) | missing |
| state.Text.children.B4 | after edits depth stays ≤ 2; after near-total deletion collapses to a leaf | `text.ts:291-297`, `text.ts:336-340` | none | test-text#05, #06 | none | missing |
Proposed status: MISSING. The tree shape is observable in cm6 only through this property; ropey's tree is private, so none of B1–B4 can be asserted in Ferrum.

### state.cm6.245 `Text.[Symbol.iterator]` — cm6 `mirrors/cm6/state/src/text.ts:150`
Helpers read: `text.ts:554-558`, `RawTextCursor` `text.ts:402`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.Symbol.iterator.B1 | `for (s of text)` / spread yields `iter()` items (R27) | `text.ts:555` | none (no `IntoIterator for &Text`) | none | none | missing |
| state.Text.Symbol.iterator.B2 | cursors returned by `iter`/`iterRange`/`iterLines` are themselves iterable | `text.ts:556-557` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.246 `Text^of` — cm6 `mirrors/cm6/state/src/text.ts:153`
Helpers read: `TextLeaf` ctor `text.ts:167`, `textLength` `text.ts:378-382`, `TextLeaf.split` `text.ts:231-244`, `TextNode.from` `text.ts:333-373`, `Text.empty` `text.ts:376`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.of.B1 | empty array → `RangeError "A document must have at least one line"` | `text.ts:154` | `text.rs:86-93` returns an empty doc | none | none | differs (R1) |
| state.Text.of.B2 | `[""]` → the shared `Text.empty` | `text.ts:155` | `text.rs:92` new empty rope | none | test_text#011 (`Text::of(&[""])` as start doc) | implemented (value-equal, R2); identity not applicable to Rust values |
| state.Text.of.B3 | ≤ 32 lines → single leaf; more → balanced tree from 32-line leaves | `text.ts:156` | ropey `from_str` | test-text#04 | none | not observable in Ferrum |
| state.Text.of.B4 | each array element becomes exactly one line; elements are not split or validated (a `"\n"`, `"\r"`, U+2028 … inside an element stays content) | `text.ts:153-157` | `text.rs:87-92` joins with `"\n"` and re-parses with ropey | none | none | differs (D2): R3, R4, R5, R6, P2 |
| state.Text.of.B5 | resulting length = Σ element lengths + (n − 1) in UTF-16 | `text.ts:378-382` | `text.rs:92` code points | test-text#03 | test_text#003 | implemented+tested (ASCII); D1 |
| state.Text.of.B6 | cost: O(n) leaf construction, no intermediate joined string | `text.ts:231-244` | `text.rs:87-91` collects a `Vec<&str>` and a joined `String` of the whole document before building the rope | none | none | differs (allocation of a full copy; verified in code, not measured) |
Proposed status: PORTED, DEVIATION (B1, B4 differ; B6 allocation; B3 not observable).

### state.cm6.247 `Text^empty` — cm6 `mirrors/cm6/state/src/text.ts:160`
Helpers read: `text.ts:376`, `TextLeaf` `text.ts:166-173`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.empty.B1 | `length 0`, `lines 1`, `toJSON() [""]`, `children null` | `text.ts:376` | `text.rs:21-25` (`len 0`, `len_lines 1`) | test-text#29 (`Text.empty.slice(…)`) | test_text#032 (`Text::empty().slice(…)`) | implemented; only `slice` on it is tested; `lines == 1` untested (R8) |
| state.Text.empty.B2 | a single shared instance (`Text.of([""]) === Text.empty`), used as the "no text" value by `replace`/`ChangeSet` | `text.ts:155`, `text.ts:376` | function returning a new value each call | none | none | not applicable to Rust value semantics (R2 value-equal) |
| state.Text.empty.B3 | replacing with `Text.empty` deletes | `text.ts:58` (`if (text.length)`) | `text.rs:162` (`if !replacement.is_empty()`) | test-text#06, #07, #08, #09 | test_text#006, #007, #008, #009 | implemented+tested |
Proposed status: PORTED, DEVIATION (B1 partly untested; B2 not applicable).

### Unfinished

none

## Chunk C2 (source: `docs/parity/state/raw/C2-behaviors.md`)


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

### Unfinished

none

## Chunk C3 (source: `docs/parity/state/raw/C3-behaviors.md`)


41 cm6 items, none DROP; one section per item (state.cm6.061–091, 166–169, 212–217), plus one helper section (`makeCategorizer`), which has no export of its own but is needed to classify `CharCategory` results, Ferrum `categorize_char` and the test-charcategory cases.

Reproduction labels (S.., C.., P.., F.., Z.., K.., J.., T.., G.., W..) refer to `docs/parity/state/raw/C3-api.md` `## Reproductions` R1–R8. "untested" means that no Ferrum test exercises the behavior (the only C3 Ferrum tests are test_selection#001–#004 and test_charcategory#001–#003; a grep of every `tests` directory in the workspace found no other test calling `find_cluster_break`, `count_column`, `find_column`, `SelectionRange::map/extend`, `EditorSelection::map` or `categorize_char`).

Shared helpers read for the selection items: `RangeFlag` `mirrors/cm6/state/src/selection.ts:12-18`, `SelectionRange.create` `mirrors/cm6/state/src/selection.ts:109`, `EditorSelection.normalized` `mirrors/cm6/state/src/selection.ts:223-236`, `checkSelection` `mirrors/cm6/state/src/selection.ts:239-242`, `ChangeDesc.mapPos` / `ChangeDesc.empty` `mirrors/cm6/state/src/change.ts` (C2; read only for the mapping semantics). Ferrum: `ChangeDesc::map_pos` `crates/ferrum-state/src/change.rs:149`, `ChangeDesc::is_empty` `crates/ferrum-state/src/change.rs:73`.

---

### state.cm6.061 `EditorSelection` — cm6 `mirrors/cm6/state/src/selection.ts:115`
Helpers read: constructor `mirrors/cm6/state/src/selection.ts:116-123`, `checkSelection` `mirrors/cm6/state/src/selection.ts:239`, callers `mirrors/cm6/state/src/state.ts:250`, `mirrors/cm6/state/src/transaction.ts:185`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.B1 | Private constructor: instances only from `create`, `single`, `fromJSON`, `map`, `asSingle`, `addRange`, `replaceRange` | `mirrors/cm6/state/src/selection.ts:116` | private fields `crates/ferrum-state/src/selection.rs:118-119`; factories `create` :147, `single` :138, `cursor` :130, `Default` :122, `map` :230 | test-selection#01–#04 | test_selection#001–#004 | implemented+tested |
| state.EditorSelection.B2 | At least one range: `create([])` throws `RangeError("A selection needs at least one range")`; only `fromJSON({ranges:[],main:-1})` can build an empty selection (S6B) | `mirrors/cm6/state/src/selection.ts:189`, `:176` | `crates/ferrum-state/src/selection.rs:148-150` returns `cursor(0)` | none | none | differs: no error, silently a cursor at 0 (S73) |
| state.EditorSelection.B3 | `checkSelection`: throws `RangeError("Selection points outside of document")` when any `range.to > docLength`; called by `EditorState.create` and the `Transaction` constructor; no lower-bound check (S99) | `mirrors/cm6/state/src/selection.ts:239-242`; `mirrors/cm6/state/src/state.ts:250`; `mirrors/cm6/state/src/transaction.ts:185` | none (searched `check_selection`, "outside of document" in all crates; `crates/ferrum-state/src/state.rs:107`, `:269` accept any selection) | none in C3 files | none | missing: ferrum accepts head 5 on a 3-char doc (S97) and head 9 via update (S98) |

Proposed status: PORTED, DEVIATION because B2 differs and B3 is missing.

### state.cm6.062 `EditorSelection.ranges` — cm6 `mirrors/cm6/state/src/selection.ts:119`
Helpers read: `EditorSelection.create`, `normalized`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.ranges.B1 | Ranges sorted by `from`, non-overlapping; non-empty ranges may touch; an empty range at the end of a non-empty one is merged, at the start it is kept (S76–S78) | `mirrors/cm6/state/src/selection.ts:117-119`, `:229` | `crates/ferrum-state/src/selection.rs:219`, invariant from `create` :147-203 | test-selection#02, #03 | test_selection#002, #003 | implemented+tested |
| state.EditorSelection.ranges.B2 | Readonly array; elements immutable | `mirrors/cm6/state/src/selection.ts:119` | `&[SelectionRange]` shared slice `crates/ferrum-state/src/selection.rs:219` | test-selection#01 | test_selection#001 | implemented+tested (Rust borrow instead of readonly) |

Proposed status: PORTED, RUST-IDIOMATIC because the contents are identical (R4) and only the accessor form changes (field → method). The difference in range `assoc` is recorded on state.cm6.084.

### state.cm6.063 `EditorSelection.mainIndex` — cm6 `mirrors/cm6/state/src/selection.ts:122`
Helpers read: `create` :188, `normalized` :223, `fromJSON` :175.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.mainIndex.B1 | Index of the main range | `mirrors/cm6/state/src/selection.ts:122` | `crates/ferrum-state/src/selection.rs:213` | test-selection#01 | test_selection#001 (through `main()`) | implemented+tested |
| state.EditorSelection.mainIndex.B2 | Not validated: may be ≥ length (create fast path, S74), −1 (normalize path, S75), negative or fractional (fromJSON, S6B/S6C) | `mirrors/cm6/state/src/selection.ts:195`, `:226`, `:178` | clamped to `len-1` `crates/ferrum-state/src/selection.rs:164`, `:172`, `:198` | none | none | differs (clamping), untested |

Proposed status: PORTED, DEVIATION because B2 differs.

### state.cm6.064 `EditorSelection.map` — cm6 `mirrors/cm6/state/src/selection.ts:127`
Helpers read: `SelectionRange.map` :73, `create` :188, `ChangeDesc.empty` (C2).

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.map.B1 | Empty change → returns `this` (S90) | `mirrors/cm6/state/src/selection.ts:128` | `crates/ferrum-state/src/selection.rs:231-233` returns `self.clone()` | none | none | implemented (equal value; identity is not observable in Rust), untested |
| state.EditorSelection.map.B2 | `assoc` defaults to −1 | `mirrors/cm6/state/src/selection.ts:127` | mandatory `assoc: i8` `crates/ferrum-state/src/selection.rs:230` | none | none | implemented (no default), untested |
| state.EditorSelection.map.B3 | Every range mapped with `range.map(change, assoc)` | `mirrors/cm6/state/src/selection.ts:129` | `crates/ferrum-state/src/selection.rs:234` | none | none | implemented, untested (range-level differences in state.cm6.087) |
| state.EditorSelection.map.B4 | Result re-normalized with `create(mapped, this.mainIndex)` (merging collapsed ranges, S91) | `mirrors/cm6/state/src/selection.ts:129` | `crates/ferrum-state/src/selection.rs:235` | none | none | implemented, untested; inherits main-index divergence state.EditorSelection.create.B5 (R4 map: 4999/5000) |
| state.EditorSelection.map.B5 | Position beyond change length → `RangeError` from `mapPos` (S34) | `mirrors/cm6/state/src/change.ts` (mapPos, C2) | panic in `crates/ferrum-state/src/change.rs:192` | none | none | implemented (panic instead of exception), untested |

Proposed status: PORTED, DEVIATION because B1–B5 are untested and B4 inherits the create main-index divergence.

### state.cm6.065 `EditorSelection.eq` — cm6 `mirrors/cm6/state/src/selection.ts:136`
Helpers read: `SelectionRange.eq` :92.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.eq.B1 | Different range count → false (S5A) | `mirrors/cm6/state/src/selection.ts:137` | derived `PartialEq` `crates/ferrum-state/src/selection.rs:116` (SmallVec equality) | none | none | implemented, untested |
| state.EditorSelection.eq.B2 | Different `mainIndex` → false (S57) | `mirrors/cm6/state/src/selection.ts:138` | derived, `main_index` field | none | none | implemented, untested |
| state.EditorSelection.eq.B3 | Range-by-range `SelectionRange.eq(other, includeAssoc)` | `mirrors/cm6/state/src/selection.ts:139-140` | derived `SelectionRange: PartialEq` `selection.rs:6` | none | none | differs (see state.SelectionRange.eq.B4) |
| state.EditorSelection.eq.B4 | Default `includeAssoc = false`: cursors equal regardless of assoc (S58 → true) | `mirrors/cm6/state/src/selection.ts:136` | always compares `assoc` | none | none | differs: ferrum false |
| state.EditorSelection.eq.B5 | `includeAssoc = true`: cursor assoc compared (S59) | `mirrors/cm6/state/src/selection.ts:136` | no parameter | none | none | missing (ferrum's `==` behaves like includeAssoc=true for cursors but also compares assoc of non-empty ranges) |

Proposed status: PORTED, DEVIATION because B3–B5 differ and nothing is tested.

### state.cm6.066 `EditorSelection.main` — cm6 `mirrors/cm6/state/src/selection.ts:147`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.main.B1 | Returns `ranges[mainIndex]` | `mirrors/cm6/state/src/selection.ts:147` | `crates/ferrum-state/src/selection.rs:207-209` | test-selection#01 | test_selection#001 | implemented+tested |

Proposed status: PORTED because its single behavior is implemented and tested (out-of-range indices are a property of `mainIndex`/`create`, recorded there).

### state.cm6.067 `EditorSelection.asSingle` — cm6 `mirrors/cm6/state/src/selection.ts:151`
Helpers read: `main` :147; callers `mirrors/cm6/state/src/state.ts:123`, `:251`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.asSingle.B1 | One range → returns `this` (S82) | `mirrors/cm6/state/src/selection.ts:152` | none | none | none | missing |
| state.EditorSelection.asSingle.B2 | Otherwise `[main]` with mainIndex 0 (S81); used by `EditorState.create` and `applyTransaction` when `allowMultipleSelections` is false | `mirrors/cm6/state/src/selection.ts:152`; `mirrors/cm6/state/src/state.ts:123`, `:251` | none (R6: ferrum keeps 2 ranges) | none in C3 files (test-state "blocks multiple selections", C5) | none | missing |

Proposed status: MISSING (searched `as_single`, `asSingle`, `collapse`, uses of `allow_multiple_selections` in all crates).

### state.cm6.068 `EditorSelection.addRange` — cm6 `mirrors/cm6/state/src/selection.ts:156`
Helpers read: `create` :188, `normalized` :223.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.addRange.B1 | New range prepended, then `create` normalizes (merging, S85) | `mirrors/cm6/state/src/selection.ts:157` | none | none | none | missing |
| state.EditorSelection.addRange.B2 | `main = true` (default): new range becomes main (index 0 before sort, tracked through sort, S83 → main=2) | `mirrors/cm6/state/src/selection.ts:157` | none | none | none | missing |
| state.EditorSelection.addRange.B3 | `main = false`: previous main stays main (`mainIndex + 1`, S84) | `mirrors/cm6/state/src/selection.ts:157` | none | none | none | missing |

Proposed status: MISSING (searched `add_range`; only `RangeSetBuilder::add_range` `crates/ferrum-state/src/rangeset.rs:362`).

### state.cm6.069 `EditorSelection.replaceRange` — cm6 `mirrors/cm6/state/src/selection.ts:162`
Helpers read: `create` :188.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.replaceRange.B1 | Copies ranges, replaces index `which` (default `mainIndex`) (S87, S88) | `mirrors/cm6/state/src/selection.ts:163-164` | none | none | none | missing |
| state.EditorSelection.replaceRange.B2 | Re-normalizes with the old `mainIndex` (S89: merge into one range) | `mirrors/cm6/state/src/selection.ts:165` | none | none | none | missing |
| state.EditorSelection.replaceRange.B3 | `which` beyond length → sparse array → `TypeError` in create (S8A) | `mirrors/cm6/state/src/selection.ts:164` | none | none | none | missing |

Proposed status: MISSING (searched `replace_range`, `replace_main`, `with_range`).

### state.cm6.070 `EditorSelection.toJSON` — cm6 `mirrors/cm6/state/src/selection.ts:170`
Helpers read: `SelectionRange.toJSON` :98.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.toJSON.B1 | `{ranges: [{anchor, head}…], main: mainIndex}` (S66); offsets in UTF-16 | `mirrors/cm6/state/src/selection.ts:171` | none | none in C3 files (test-state "can be serialized to JSON", C5) | none | missing |

Proposed status: MISSING (searched `to_json`, `Serialize`, `serde`).

### state.cm6.071 `EditorSelection^fromJSON` — cm6 `mirrors/cm6/state/src/selection.ts:175`
Helpers read: `SelectionRange.fromJSON` :102; callers `mirrors/cm6/state/src/state.ts:235`, `mirrors/cm6/commands/src/history.ts:211-212`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.fromJSON.B1 | Validation: falsy json, non-array `ranges`, non-number `main`, `main >= ranges.length` → `RangeError("Invalid JSON representation for EditorSelection")` (S68, S69) | `mirrors/cm6/state/src/selection.ts:176-177` | none | none | none | missing |
| state.EditorSelection.fromJSON.B2 | Each range through `SelectionRange.fromJSON` (S67) | `mirrors/cm6/state/src/selection.ts:178` | none | none | none | missing |
| state.EditorSelection.fromJSON.B3 | No normalization: unsorted/overlapping ranges kept (S6A) | `mirrors/cm6/state/src/selection.ts:178` (private constructor, not `create`) | none | none | none | missing |
| state.EditorSelection.fromJSON.B4 | Accepts `main` negative or fractional and empty `ranges` with `main < 0` (S6B, S6C) | `mirrors/cm6/state/src/selection.ts:176` | none | none | none | missing |
| state.EditorSelection.fromJSON.B5 | Errors of a range propagate (`SelectionRange` RangeError, S6D) | `mirrors/cm6/state/src/selection.ts:178` | none | none | none | missing |

Proposed status: MISSING (searched `from_json`, `Deserialize`).

### state.cm6.072 `EditorSelection^single` — cm6 `mirrors/cm6/state/src/selection.ts:182`
Helpers read: `EditorSelection.range` :208.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.single.B1 | `head` defaults to `anchor` (S70) | `mirrors/cm6/state/src/selection.ts:182` | `head` required `crates/ferrum-state/src/selection.rs:138` (`EditorSelection::cursor(pos)` :130 covers the one-argument form) | none | none | implemented (split into two functions), untested |
| state.EditorSelection.single.B2 | Range built with `EditorSelection.range` → automatic assoc for non-empty (S71 assoc 1) | `mirrors/cm6/state/src/selection.ts:183` | `SelectionRange::new` → assoc 0 `selection.rs:140` | none | none | differs |
| state.EditorSelection.single.B3 | mainIndex 0, no normalization, no bounds check | `mirrors/cm6/state/src/selection.ts:183` | `crates/ferrum-state/src/selection.rs:139-142` | none | none | implemented, untested |

Proposed status: PORTED, DEVIATION because B2 differs and nothing is tested.

### state.cm6.073 `EditorSelection^create` — cm6 `mirrors/cm6/state/src/selection.ts:188`
Helpers read: `normalized` :223-236, `EditorSelection.range` :208, `SelectionRange.empty` :47.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.create.B1 | Empty input → `RangeError` | `mirrors/cm6/state/src/selection.ts:189` | `crates/ferrum-state/src/selection.rs:148-150` → `cursor(0)` | none | none | differs (S73) |
| state.EditorSelection.create.B2 | Fast-path check from `pos = 0` including index 0: empty range needs `from > pos`, non-empty `from >= pos` (an initial cursor at 0 takes the normalize path, same result) | `mirrors/cm6/state/src/selection.ts:190-194` | `crates/ferrum-state/src/selection.rs:153-161` (skips index 0) | test-selection#01 | test_selection#001 | implemented+tested (index-0 difference not observable: S79 equal) |
| state.EditorSelection.create.B3 | Fast path returns the given ranges and mainIndex unchanged (even if out of range, S74) | `mirrors/cm6/state/src/selection.ts:195` | `crates/ferrum-state/src/selection.rs:163-169` clamps main | test-selection#01 | test_selection#001 (in-range only) | differs for out-of-range main, untested |
| state.EditorSelection.create.B4 | Normalize: copy, remember `main = ranges[mainIndex]`, stable sort by `from` | `mirrors/cm6/state/src/selection.ts:192`, `:224-225` | `crates/ferrum-state/src/selection.rs:172-173` (`sort_by_key`, stable) | test-selection#02 | test_selection#002 | implemented+tested |
| state.EditorSelection.create.B5 | Main located after sort by object identity (`indexOf`) | `mirrors/cm6/state/src/selection.ts:226` | value equality `crates/ferrum-state/src/selection.rs:174` | none | none | differs: `create([cursor(12), range(12,18), cursor(12)], 2)` → cm6 main=1 \| ferrum main=0 (R4) |
| state.EditorSelection.create.B6 | Merge condition: empty range merges when `from <= prev.to`, non-empty when `from < prev.to` | `mirrors/cm6/state/src/selection.ts:229` | `crates/ferrum-state/src/selection.rs:180` | test-selection#02, #03 | test_selection#002, #003 | implemented+tested |
| state.EditorSelection.create.B7 | Merged range spans `prev.from` … `max(range.to, prev.to)` | `mirrors/cm6/state/src/selection.ts:230` | `crates/ferrum-state/src/selection.rs:181-182` | test-selection#02, #03 | test_selection#002, #003 | implemented+tested |
| state.EditorSelection.create.B8 | `mainIndex--` when `i <= mainIndex` (main tracking through merges, S7E/S7F) | `mirrors/cm6/state/src/selection.ts:231` | `crates/ferrum-state/src/selection.rs:183-185` | none (cm6 merge tests use main 0) | none | implemented, untested |
| state.EditorSelection.create.B9 | Merged range takes the direction of the later range and is built by `EditorSelection.range` → automatic assoc ±1; goalColumn, bidiLevel, undirectional dropped (S7A–S7D) | `mirrors/cm6/state/src/selection.ts:232` | `crates/ferrum-state/src/selection.rs:186-190` (`SelectionRange::new`: assoc 0, goal None) | test-selection#04 (direction) | test_selection#004 (direction) | direction implemented+tested; assoc differs (S7A: −1 vs 0, S7B: 1 vs 0) |
| state.EditorSelection.create.B10 | Out-of-range main on the normalize path → `main` undefined → mainIndex −1 (S75) | `mirrors/cm6/state/src/selection.ts:224-226` | clamped `crates/ferrum-state/src/selection.rs:172`, `:198` | none | none | differs, untested |

Proposed status: PORTED, DEVIATION because B1, B3, B5, B9 (assoc), B10 differ and B8 is untested.

### state.cm6.074 `EditorSelection^cursor` — cm6 `mirrors/cm6/state/src/selection.ts:200`
Helpers read: `SelectionRange.create` :109, `RangeFlag` :12.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.cursor.B1 | Empty range at `pos` | `mirrors/cm6/state/src/selection.ts:201-202` | `SelectionRange::cursor` `crates/ferrum-state/src/selection.rs:20-27` | none | none (used in test_state setups, C5) | implemented, untested in C3 |
| state.EditorSelection.cursor.B2 | `assoc`: 0 → none, negative → AssocBefore (−1), positive → AssocAfter (1, any magnitude: S10) | `mirrors/cm6/state/src/selection.ts:202` | no parameter; field `assoc` writable | none | none | missing parameter |
| state.EditorSelection.cursor.B3 | `bidiLevel`: null → unset (7), else `min(6, level)` (S09) | `mirrors/cm6/state/src/selection.ts:203` | none | none | none | missing |
| state.EditorSelection.cursor.B4 | `goalColumn` stored | `mirrors/cm6/state/src/selection.ts:204` | via `with_goal_column` `selection.rs:64` or field | none | none | implemented differently, untested |

Proposed status: PORTED, DEVIATION because B2–B3 are missing and nothing is tested. Note the name clash: Ferrum `EditorSelection::cursor` (`selection.rs:130`) returns an `EditorSelection`.

### state.cm6.075 `EditorSelection^range` — cm6 `mirrors/cm6/state/src/selection.ts:208`
Helpers read: `SelectionRange.create` :109, `RangeFlag` :12.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.range.B1 | `bidiLevel`: null → 7 (unset), else `min(6, level)` (S05) | `mirrors/cm6/state/src/selection.ts:209` | none | none | none | missing |
| state.EditorSelection.range.B2 | No explicit assoc and `anchor != head` → assoc = `head < anchor ? 1 : -1` (S01, S02) | `mirrors/cm6/state/src/selection.ts:210` | `SelectionRange::new` sets 0 `crates/ferrum-state/src/selection.rs:30-37` | none | none | differs |
| state.EditorSelection.range.B3 | Explicit `assoc` argument (also on empty ranges, S06) | `mirrors/cm6/state/src/selection.ts:211` | none (field write) | none | none | missing |
| state.EditorSelection.range.B4 | `head < anchor` → stored `from = head, to = anchor` + Inverted flag; anchor/head reported back unchanged | `mirrors/cm6/state/src/selection.ts:212-213` | stores anchor/head directly, `from()`/`to()` :41-49 | test-selection#01, #04 | test_selection#001, #004 | implemented+tested |
| state.EditorSelection.range.B5 | `goalColumn` stored | `mirrors/cm6/state/src/selection.ts:212-213` | `with_goal_column` :64 | none | none | implemented differently, untested |

Proposed status: PORTED, DEVIATION because B1–B3 are missing/different.

### state.cm6.076 `EditorSelection^undirectionalRange` — cm6 `mirrors/cm6/state/src/selection.ts:218`
Helpers read: `SelectionRange.create` :109.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.EditorSelection.undirectionalRange.B1 | Flags = `Undirectional` only: assoc 0, bidiLevel **0** (not null), not inverted (S11, S12) | `mirrors/cm6/state/src/selection.ts:219` | none | none | none | missing |
| state.EditorSelection.undirectionalRange.B2 | `from`/`to` stored as given, no ordering: `(5,2)` gives from 5 > to 2 (S11) | `mirrors/cm6/state/src/selection.ts:219` | none | none | none | missing |
| state.EditorSelection.undirectionalRange.B3 | No goalColumn | `mirrors/cm6/state/src/selection.ts:219` | none | none | none | missing |

Proposed status: MISSING (searched `undirectional`, `undecided`, `directionless` in all crates).

### state.cm6.077 `SelectionRange` — cm6 `mirrors/cm6/state/src/selection.ts:24`
Helpers read: `RangeFlag` :12-18, constructor :25-36, `SelectionRange.create` :109.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.B1 | Stores `from`, `to`, `flags`, `goalColumn`; anchor/head derived | `mirrors/cm6/state/src/selection.ts:25-36` | stores `anchor`, `head`, `goal_column`, `assoc` `crates/ferrum-state/src/selection.rs:7-16` | test-selection#01 | test_selection#001 | implemented differently (observable values equal for anchor/head/from/to) |
| state.SelectionRange.B2 | Flags: bidi level (3 bits, 7 = unset), assoc before/after, inverted, undirectional | `mirrors/cm6/state/src/selection.ts:3-18` | only `assoc: i8`; inversion implicit (`head < anchor`) | none | none | differs: no bidi, no undirectional |
| state.SelectionRange.B3 | Private constructor and readonly fields: ranges only from factories, immutable | `mirrors/cm6/state/src/selection.ts:25`, `:27-35` | all fields `pub`, `Copy`, mutable | none | none | differs (any `i8` assoc, any anchor/head) |
| state.SelectionRange.B4 | Identity-based reuse: `map` returns `this` when unchanged (S31) | `mirrors/cm6/state/src/selection.ts:81` | value type | none | none | not observable in Rust (no identity) |

Proposed status: PORTED, DEVIATION because B2–B3 differ.

### state.cm6.078 `SelectionRange.from` — cm6 `mirrors/cm6/state/src/selection.ts:27`
Helpers read: `EditorSelection.range` :212, `undirectionalRange` :219.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.from.B1 | Lower boundary | `mirrors/cm6/state/src/selection.ts:27` | `min(anchor, head)` `crates/ferrum-state/src/selection.rs:41-43` | test-selection#01 | test_selection#001 | implemented+tested |
| state.SelectionRange.from.B2 | `undirectionalRange(5,2)` stores from 5 (> to) | `mirrors/cm6/state/src/selection.ts:219` | not constructible | none | none | not applicable without undirectionalRange (recorded on state.cm6.076) |

Proposed status: PORTED, RUST-IDIOMATIC (field → method, same value; B2 belongs to a missing constructor).

### state.cm6.079 `SelectionRange.to` — cm6 `mirrors/cm6/state/src/selection.ts:29`
Helpers read: as `from`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.to.B1 | Upper boundary | `mirrors/cm6/state/src/selection.ts:29` | `max(anchor, head)` `crates/ferrum-state/src/selection.rs:47-49` | test-selection#01 | test_selection#001 | implemented+tested |
| state.SelectionRange.to.B2 | `undirectionalRange(5,2)` stores to 2 (< from) | `mirrors/cm6/state/src/selection.ts:219` | not constructible | none | none | not applicable (state.cm6.076) |

Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.080 `SelectionRange.goalColumn` — cm6 `mirrors/cm6/state/src/selection.ts:35`
Helpers read: `cursor` :200, `range` :208, `map` :81, `extend` :85, `normalized` :232.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.goalColumn.B1 | Set only by `cursor`/`range` arguments; readonly | `mirrors/cm6/state/src/selection.ts:35`, `:204`, `:212` | `pub goal_column` + `with_goal_column` `crates/ferrum-state/src/selection.rs:13`, `:64` | none | none | implemented differently (mutable), untested |
| state.SelectionRange.goalColumn.B2 | Kept by `map` (S29) | `mirrors/cm6/state/src/selection.ts:81` | `crates/ferrum-state/src/selection.rs:92` | none | none | implemented, untested |
| state.SelectionRange.goalColumn.B3 | Dropped by `extend` (S46) | `mirrors/cm6/state/src/selection.ts:86-88` | `crates/ferrum-state/src/selection.rs:100`, `:109` | none | none | implemented, untested |
| state.SelectionRange.goalColumn.B4 | Dropped when ranges merge in `create` (S7C) | `mirrors/cm6/state/src/selection.ts:232` | `crates/ferrum-state/src/selection.rs:186-190` | none | none | implemented, untested |

Proposed status: PORTED, DEVIATION because B1–B4 are untested and the field is mutable (`usize` instead of `number`).

### state.cm6.081 `SelectionRange.anchor` — cm6 `mirrors/cm6/state/src/selection.ts:40`
Helpers read: `RangeFlag.Inverted` :16.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.anchor.B1 | `Inverted ? to : from` | `mirrors/cm6/state/src/selection.ts:40` | stored field `crates/ferrum-state/src/selection.rs:9` | test-selection#01, #04 | test_selection#001, #004 | implemented+tested |

Proposed status: PORTED, RUST-IDIOMATIC (stored instead of derived; same values).

### state.cm6.082 `SelectionRange.head` — cm6 `mirrors/cm6/state/src/selection.ts:44`
Helpers read: `RangeFlag.Inverted` :16.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.head.B1 | `Inverted ? from : to` | `mirrors/cm6/state/src/selection.ts:44` | stored field `crates/ferrum-state/src/selection.rs:11` | test-selection#01, #04 | test_selection#001, #004 | implemented+tested |

Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.083 `SelectionRange.empty` — cm6 `mirrors/cm6/state/src/selection.ts:47`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.empty.B1 | `from == to` | `mirrors/cm6/state/src/selection.ts:47` | `anchor == head` `crates/ferrum-state/src/selection.rs:53-55` | test-selection#03 (merge branch) | test_selection#003 | implemented+tested |

Proposed status: PORTED, RUST-IDIOMATIC (getter → `is_empty()`).

### state.cm6.084 `SelectionRange.assoc` — cm6 `mirrors/cm6/state/src/selection.ts:53`
Helpers read: `RangeFlag` :14-15, `cursor` :202, `range` :210-211, `map` :81, `extend` :86-88, `normalized` :232; consumers `mirrors/cm6/view/src/cursor.ts` (`moveVertically` passes `start.assoc`), `mirrors/cm6/view/src/bidi.ts:438`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.assoc.B1 | Decoded from flags: AssocBefore → −1, AssocAfter → 1, else 0 (only −1/0/1) | `mirrors/cm6/state/src/selection.ts:53` | `pub assoc: i8` `crates/ferrum-state/src/selection.rs:15` (any i8) | none | none | differs (type range), untested |
| state.SelectionRange.assoc.B2 | Automatically ±1 for non-empty ranges built by `range()` (forward −1, inverted 1) | `mirrors/cm6/state/src/selection.ts:210` | constructors give 0 `selection.rs:25`, `:35` | none | none | differs (S01, S02) |
| state.SelectionRange.assoc.B3 | Preserved through `map` (S22, S29) | `mirrors/cm6/state/src/selection.ts:81` | `crates/ferrum-state/src/selection.rs:93` | none | none | implemented, untested |
| state.SelectionRange.assoc.B4 | Recomputed by `extend` and by merges (via `range()`) | `mirrors/cm6/state/src/selection.ts:86-88`, `:232` | reset to 0 `selection.rs:100`, `:109`, `:186-190` | none | none | differs (S45, S49, S7A) |

Proposed status: PORTED, DEVIATION because B1, B2, B4 differ and nothing is tested.

### state.cm6.085 `SelectionRange.undirectional` — cm6 `mirrors/cm6/state/src/selection.ts:60`
Helpers read: `RangeFlag.Undirectional` :17, `undirectionalRange` :218, consumer `mirrors/cm6/view/src/cursor.ts:167`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.undirectional.B1 | True only for ranges from `undirectionalRange` | `mirrors/cm6/state/src/selection.ts:60-62` | none | none | none | missing |
| state.SelectionRange.undirectional.B2 | Kept by `map` (S30), dropped by `extend`/merge (S7C) | `mirrors/cm6/state/src/selection.ts:81`, `:86-88`, `:232` | none | none | none | missing |

Proposed status: MISSING.

### state.cm6.086 `SelectionRange.bidiLevel` — cm6 `mirrors/cm6/state/src/selection.ts:66`
Helpers read: `RangeFlag.BidiLevelMask` :13, `cursor` :203, `range` :209, `undirectionalRange` :219.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.bidiLevel.B1 | `null` when the 3 bits are 7 (unset) | `mirrors/cm6/state/src/selection.ts:66-69` | none | none | none | missing |
| state.SelectionRange.bidiLevel.B2 | Levels clamped to 0–6 at construction (S05, S09) | `mirrors/cm6/state/src/selection.ts:203`, `:209` | none | none | none | missing |
| state.SelectionRange.bidiLevel.B3 | Kept by `map` (S29), dropped by `extend`/merge; `undirectionalRange` yields 0 (S12) | `mirrors/cm6/state/src/selection.ts:81`, `:219` | none | none | none | missing |

Proposed status: MISSING (searched `bidi`, `bidi_level` in all crates).

### state.cm6.087 `SelectionRange.map` — cm6 `mirrors/cm6/state/src/selection.ts:73`
Helpers read: `ChangeDesc.mapPos` (C2, default `MapMode.Simple`), constructor :25.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.map.B1 | Empty range: both ends `mapPos(from, assoc)` (S20, S21, S33) | `mirrors/cm6/state/src/selection.ts:75-76` | `crates/ferrum-state/src/selection.rs:71-75` | none | none | implemented, untested |
| state.SelectionRange.map.B2 | Non-empty: `from` mapped with +1, `to` with −1 — insertions at the edges stay outside, the `assoc` argument is ignored (S23–S25, S32, S35) | `mirrors/cm6/state/src/selection.ts:78-79` | `crates/ferrum-state/src/selection.rs:77-82` | none | none | implemented, untested |
| state.SelectionRange.map.B3 | Unchanged → returns `this` (S31) | `mirrors/cm6/state/src/selection.ts:81` | always a new value | none | none | not observable in Rust |
| state.SelectionRange.map.B4 | Keeps flags (inverted, assoc, bidi, undirectional) and goalColumn | `mirrors/cm6/state/src/selection.ts:81` | keeps direction (:83-87), `assoc`, `goal_column` (:89-94) | none | none | implemented for existing fields, untested |
| state.SelectionRange.map.B5 | Non-empty range collapsing to a point keeps its flags (S27/S28: cm6 assoc −1/1, ferrum 0/0 because built by `new`) | `mirrors/cm6/state/src/selection.ts:81` | `crates/ferrum-state/src/selection.rs:83-94` | none | none | implemented; assoc values differ only through construction (state.cm6.084) |
| state.SelectionRange.map.B6 | `assoc` default −1; out-of-range position throws (S34) | `mirrors/cm6/state/src/selection.ts:73` | mandatory `assoc`; panic `crates/ferrum-state/src/change.rs:192` | none | none | implemented (no default; panic), untested |

Proposed status: PORTED, DEVIATION because every behavior is untested (positions match in all probes and in the oracle).

### state.cm6.088 `SelectionRange.extend` — cm6 `mirrors/cm6/state/src/selection.ts:85`
Helpers read: `EditorSelection.range` :208.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.extend.B1 | `from <= anchor <= to` → new range `from`→`to` (anchor replaced, S41, S43) | `mirrors/cm6/state/src/selection.ts:86` | `crates/ferrum-state/src/selection.rs:99-100` | none | none | implemented, untested |
| state.SelectionRange.extend.B2 | Otherwise keep anchor, head = whichever of from/to is farther from anchor, ties → `to` (S40, S42, S44, S47, S48) | `mirrors/cm6/state/src/selection.ts:87-88` | `crates/ferrum-state/src/selection.rs:102-109` | none | none | implemented, untested |
| state.SelectionRange.extend.B3 | `to` defaults to `from` | `mirrors/cm6/state/src/selection.ts:85` | required | none | none | implemented (no default) |
| state.SelectionRange.extend.B4 | Result via `range(…, assoc)`: automatic ±1 assoc or explicit `assoc` argument (S45, S49) | `mirrors/cm6/state/src/selection.ts:86`, `:88` | `SelectionRange::new` → assoc 0, no argument | none | none | differs |
| state.SelectionRange.extend.B5 | goalColumn/bidi dropped (S46) | `mirrors/cm6/state/src/selection.ts:86-88` | `crates/ferrum-state/src/selection.rs:100`, `:109` | none | none | implemented, untested |

Proposed status: PORTED, DEVIATION because B4 differs and nothing is tested. cm6 view uses the assoc argument (`mirrors/cm6/view/src/input.ts:626`).

### state.cm6.089 `SelectionRange.eq` — cm6 `mirrors/cm6/state/src/selection.ts:92`
Helpers read: `assoc` :53, `empty` :47.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.eq.B1 | Compares anchor (S54) | `mirrors/cm6/state/src/selection.ts:93` | derived `PartialEq` `crates/ferrum-state/src/selection.rs:6` | none | none | implemented, untested |
| state.SelectionRange.eq.B2 | Compares head | `mirrors/cm6/state/src/selection.ts:93` | derived | none | none | implemented, untested |
| state.SelectionRange.eq.B3 | Compares goalColumn (S53) | `mirrors/cm6/state/src/selection.ts:93` | derived | none | none | implemented, untested |
| state.SelectionRange.eq.B4 | Assoc compared only with `includeAssoc` and only for empty ranges; bidi/undirectional never compared (S50, S51, S52, S55, S56) | `mirrors/cm6/state/src/selection.ts:94` | assoc always compared | none | none | differs |

Proposed status: PORTED, DEVIATION because B4 differs and nothing is tested.

### state.cm6.090 `SelectionRange.toJSON` — cm6 `mirrors/cm6/state/src/selection.ts:98`

Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.toJSON.B1 | `{anchor, head}` (S60) | `mirrors/cm6/state/src/selection.ts:98` | none | none | none | missing |
| state.SelectionRange.toJSON.B2 | assoc, bidiLevel, goalColumn, undirectional not serialized (S61) | `mirrors/cm6/state/src/selection.ts:98` | none | none | none | missing |

Proposed status: MISSING.

### state.cm6.091 `SelectionRange^fromJSON` — cm6 `mirrors/cm6/state/src/selection.ts:102`
Helpers read: `EditorSelection.range` :208.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SelectionRange.fromJSON.B1 | Falsy or non-number anchor/head → `RangeError("Invalid JSON representation for SelectionRange")` (S63, S64) | `mirrors/cm6/state/src/selection.ts:103-104` | none | none | none | missing |
| state.SelectionRange.fromJSON.B2 | Built with `range(anchor, head)` → automatic assoc (S62 assoc 1) | `mirrors/cm6/state/src/selection.ts:105` | none | none | none | missing |
| state.SelectionRange.fromJSON.B3 | No integer, sign or document-bounds check (S65) | `mirrors/cm6/state/src/selection.ts:103` | none | none | none | missing |

Proposed status: MISSING.

### state.cm6.166 `CharCategory` — cm6 `mirrors/cm6/state/src/charcategory.ts:4`
Helpers read: `makeCategorizer` `mirrors/cm6/state/src/charcategory.ts:28` (see helper section).

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.B1 | Three members Word (0), Space (1), Other (2) | `mirrors/cm6/state/src/charcategory.ts:4-11` | `crates/ferrum-state/src/char.rs:5-9` (implicit discriminants 0, 1, 2) | test-charcategory#01–#03 | test_charcategory#001–#003 | implemented+tested |

Proposed status: PORTED.

### state.cm6.167 `CharCategory.Word` — cm6 `mirrors/cm6/state/src/charcategory.ts:6`
Helpers read: as 166.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.Word.B1 | Member value 0, produced for word characters | `mirrors/cm6/state/src/charcategory.ts:6` | `crates/ferrum-state/src/char.rs:6` | test-charcategory#01 | test_charcategory#001 | implemented+tested |

Proposed status: PORTED.

### state.cm6.168 `CharCategory.Space` — cm6 `mirrors/cm6/state/src/charcategory.ts:8`
Helpers read: as 166.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.Space.B1 | Member value 1, produced for whitespace | `mirrors/cm6/state/src/charcategory.ts:8` | `crates/ferrum-state/src/char.rs:7` | test-charcategory#02 | test_charcategory#002 | implemented+tested |

Proposed status: PORTED.

### state.cm6.169 `CharCategory.Other` — cm6 `mirrors/cm6/state/src/charcategory.ts:10`
Helpers read: as 166.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.CharCategory.Other.B1 | Member value 2, produced for everything else | `mirrors/cm6/state/src/charcategory.ts:10` | `crates/ferrum-state/src/char.rs:8` | test-charcategory#03 | test_charcategory#003 | implemented+tested |

Proposed status: PORTED.

### Helper (no export): `makeCategorizer` — cm6 `mirrors/cm6/state/src/charcategory.ts:28`
Read for state.fe.053 `categorize_char` and `EditorState::char_categorizer` (`crates/ferrum-state/src/state.rs:190-198`); the exported `EditorState.charCategorizer` (`mirrors/cm6/state/src/state.ts:373`) is chunk C5.
Helpers read: `wordChar` `mirrors/cm6/state/src/charcategory.ts:15-16`, `hasWordChar` :18-26, `nonASCIISingleCaseWordChar` :13, `EditorState.languageDataAt` `mirrors/cm6/state/src/state.ts:353`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.categorizer.B1 | `!/\S/.test(str)` → Space: the whole string (a cluster) must be JS whitespace; empty string → Space. JS `\s` includes U+FEFF and excludes U+0085 | `mirrors/cm6/state/src/charcategory.ts:30` | first char only, `char::is_whitespace` `crates/ferrum-state/src/char.rs:13`, `state.rs:192-196` (empty → Other) | test-charcategory#02 | test_charcategory#002 | differs: `""` Space vs Other, `" ́"` Other vs Space, U+0085, U+FEFF (R7) |
| state.categorizer.B2 | Word if **any** code point of the string matches `[\p{Alphabetic}\p{Number}_]` | `mirrors/cm6/state/src/charcategory.ts:16`, `:19`, `:31` | first char only, `is_alphanumeric() \|\| '_'` `crates/ferrum-state/src/char.rs:15` | test-charcategory#01 | test_charcategory#001 | differs for multi-code-point strings (`" a"`, `"/a"`); identical for all single code points except U+0085/U+FEFF (exhaustive, R7) |
| state.categorizer.B3 | Fallback when the `u`-flag property regex is unsupported: `/\w/`, or non-ASCII with case distinction, or `nonASCIISingleCaseWordChar` | `mirrors/cm6/state/src/charcategory.ts:13`, `:20-25` | none | none | none | not reached on modern engines (Node 22 uses B2); no Ferrum equivalent needed unless parity with old engines is wanted |
| state.categorizer.B4 | Extra `wordChars` from language data at `at`: Word if the string contains any of them | `mirrors/cm6/state/src/charcategory.ts:32`; `mirrors/cm6/state/src/state.ts:374-375` | none (`_at` ignored, `crates/ferrum-state/src/state.rs:190`) | none | none | missing (G `languageData wordChars "$"` → cm6 Word, ferrum Other) |
| state.categorizer.B5 | Otherwise Other | `mirrors/cm6/state/src/charcategory.ts:33` | `crates/ferrum-state/src/char.rs:17` | test-charcategory#03 | test_charcategory#003 | implemented+tested |
| state.categorizer.B6 | No CJK special case in the categorizer (CJK letters are `\p{Alphabetic}` → Word); CJK word segmentation exists only in `moveBySubword` (`mirrors/cm6/commands/src/commands.ts:112`, `:146-156`) | `mirrors/cm6/state/src/charcategory.ts:16` | `is_alphanumeric` → Word | none | none | implemented, untested (CJK, Hiragana, Hangul → Word on both sides, R7) |

Proposed status (for the helper, informational): partially equivalent; differences B1, B2 (multi-char), B4.

### state.cm6.212 `findClusterBreak` — cm6 `mirrors/cm6/state/src/char.ts:9`
Helpers read: `@marijn/find-cluster-break/src/index.js` `findClusterBreak` :41, `nextClusterBreak` :45-66, `prevClusterBreak` :68-75, `isExtendingChar` :17-26, `isRegionalIndicator` :28, `codePointAt` :77, `codePointSize` :87, extender table :5-15 and its generator `bin/build-extenders.js` (Extend + SpacingMark minus 31 exceptions, code points < U+30000). Ferrum: `unicode-segmentation` 1.13.3 `grapheme_indices(true)`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.findClusterBreak.B1 | Forward at `pos == length` → `pos`; backward at `pos <= 1` → 0 (C14–C17) | `index.js:46`, `:69`, `:74` | `crates/ferrum-state/src/char.rs:32-35` | none | none | implemented, untested |
| state.findClusterBreak.B2 | Forward from inside a surrogate pair moves to the pair start first (C06, C07) | `index.js:48` | not representable in char units | none | none | not applicable under char positions (D1) |
| state.findClusterBreak.B3 | Steps by code points (surrogate pair = 2 units); lone surrogates count as one unit (C08, C09–C11) | `index.js:49-50`, `:77-83` | char units; `&str` cannot contain lone surrogates | none | none | differs by unit (D1): `"😀"` → 2 vs 1 |
| state.findClusterBreak.B4 | Joins following extending characters (GCB Extend and SpacingMark, only < U+30000) when `includeExtending` | `index.js:53`, `:17-26` | UAX #29 GB9/GB9a on all code points `char.rs:44` | none | none | differs for tag characters / VS supplement ≥ U+E0000 (T25); equal for marks, VS16, skin tones, SpacingMarks (T05, T06, T14, T20, T21, T24) |
| state.findClusterBreak.B5 | `includeExtending = false`: extending characters not joined, ZWJ and RI rules still apply (C01, C03, C05, C19, C20) | `index.js:41`, `:53` | no parameter `char.rs:31` | none | none | missing (cm6 commands use it: `mirrors/cm6/commands/src/commands.ts:581`, `:585`) |
| state.findClusterBreak.B6 | ZWJ joins the next code point unconditionally, and ZWJ itself joins the previous one | `index.js:53` | GB9 (ZWJ is Extend) + GB11 (only ExtPict ZWJ × ExtPict) | none | none | differs: `"a‍b"` one cluster vs two (T09); `"\n‍😀"` (T34); equal for emoji ZWJ sequences (T08, T15) |
| state.findClusterBreak.B7 | Regional indicators paired by counting preceding RIs (odd count joins) | `index.js:56-60` | GB12/GB13 | none | none | implemented (T12, T13, C21–C24 equal after unit conversion), untested |
| state.findClusterBreak.B8 | No CR×LF rule: `"\r\n"` is two clusters | `index.js:45-66` (no rule) | GB3: one cluster | none | none | differs (T02, T03; 105/2000 ASCII oracle strings, all containing CRLF) |
| state.findClusterBreak.B9 | No control rules: CR/LF/control followed by an extending char or ZWJ joins (`"\ŕ"`, `"\t́"`) | `index.js:53` | GB4/GB5: break | none | none | differs (T33, T34) |
| state.findClusterBreak.B10 | No Hangul syllable rules: L, V, T jamo and LV+T are separate clusters | (no rule) | GB6–GB8 join | none | none | differs (T17, T18) |
| state.findClusterBreak.B11 | No Prepend (GB9b) and no Indic conjunct rule (GB9c) | (no rule) | GB9b, GB9c implemented | none | none | differs (T19, T23, T31) |
| state.findClusterBreak.B12 | Backward: `prevClusterBreak` probes `nextClusterBreak(pos-2)` while `pos > 1`, returns the first result `< pos`; from inside a cluster returns that cluster's start | `index.js:68-75` | start of grapheme before `pos` `char.rs:57-66` | none | none | implemented (equal on all non-B8–B11 table cases), untested |
| state.findClusterBreak.B13 | `pos` beyond length: forward returns garbage (`"ab"`,5 → 7), backward 2; negative pos accepted (C12, C13, C18) | `index.js:46-50` | forward returns `pos` (5), backward 1 | none | none | differs (out-of-contract input) |
| state.findClusterBreak.B14 | Cost: inspects only code units near `pos` | `index.js:45-75` | whole-string scan + `Vec` of all graphemes per call; backward re-counts prefixes: O(n) forward, O(n²) backward (R5) | none | none | differs (complexity) |

Proposed status: PORTED, DEVIATION because B3–B6, B8–B11, B13, B14 differ, B5 is missing and nothing is tested.

### state.cm6.213 `codePointAt` — cm6 `mirrors/cm6/state/src/char.ts:19`
Helpers read: `surrogateLow`/`surrogateHigh` `mirrors/cm6/state/src/char.ts:13-14`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.codePointAt.B1 | Non-high-surrogate unit → that unit (P06, P02 low half) | `mirrors/cm6/state/src/char.ts:20-21` | none | none | none | missing |
| state.codePointAt.B2 | High surrogate at the last position → the unit (P03) | `mirrors/cm6/state/src/char.ts:21` | none | none | none | missing |
| state.codePointAt.B3 | High surrogate not followed by low → the unit (P04) | `mirrors/cm6/state/src/char.ts:22-23` | none | none | none | missing |
| state.codePointAt.B4 | Valid pair → combined code point (P01); out of range → NaN (P05) | `mirrors/cm6/state/src/char.ts:24`, `:20` | none | none | none | missing |

Proposed status: MISSING (searched `code_point_at`, `codepoint`, `char_code_at`; meaning depends on D1).

### state.cm6.214 `fromCodePoint` — cm6 `mirrors/cm6/state/src/char.ts:30`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.fromCodePoint.B1 | `code <= 0xFFFF` → one unit, incl. lone surrogates and negative values wrapping (`-1` → U+FFFF) (F01, F03, F05, F06) | `mirrors/cm6/state/src/char.ts:31` | none | none | none | missing |
| state.fromCodePoint.B2 | Otherwise a surrogate pair (F02, F07) | `mirrors/cm6/state/src/char.ts:32-33` | none | none | none | missing |
| state.fromCodePoint.B3 | No range validation (0x110000 → `\udc00\udc00`, F04) | `mirrors/cm6/state/src/char.ts:30-33` | none | none | none | missing |

Proposed status: MISSING (searched `from_code_point`; Rust `char::from_u32` returns `Option` and cannot produce lone surrogates).

### state.cm6.215 `codePointSize` — cm6 `mirrors/cm6/state/src/char.ts:37`
Helpers read: none.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.codePointSize.B1 | `< 0x10000` → 1, else 2 (Z01–Z03) | `mirrors/cm6/state/src/char.ts:37` | none (`char::len_utf16` exists in std, unused) | none | none | missing |

Proposed status: MISSING (searched `code_point_size`, `len_utf16`).

### state.cm6.216 `countColumn` — cm6 `mirrors/cm6/state/src/column.ts:5`
Helpers read: `findClusterBreak` `mirrors/cm6/state/src/char.ts:9` (and its dependency).

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.countColumn.B1 | Loop while `i < to && i < length`; `to` defaults to length; `to > length` clamps (K04, K05); negative `to` → 0 (K09) | `mirrors/cm6/state/src/column.ts:5`, `:7` | `crates/ferrum-state/src/column.rs:9`, `:14` | none | none | implemented, untested |
| state.countColumn.B2 | Tab: `n += tabSize - n % tabSize`, `i++` (K01–K03) | `mirrors/cm6/state/src/column.ts:8-10` | `crates/ferrum-state/src/column.rs:15-17` | none | none | implemented, untested |
| state.countColumn.B3 | Other: `n++`, advance to next cluster break (one column per cluster; no East Asian width, K10) | `mirrors/cm6/state/src/column.ts:12-13` | `crates/ferrum-state/src/column.rs:19-21` (with an `i+1` fallback cm6 does not need) | none | none | implemented; results differ wherever findClusterBreak differs (R2: CRLF 4 vs 3, Hangul jamo 3 vs 1, `"a‍b"` 1 vs 2, …) |
| state.countColumn.B4 | `to` inside a cluster counts the whole cluster (K06) | `mirrors/cm6/state/src/column.ts:7`, `:13` | same structure | none | none | implemented, untested (UTF-16 case not representable) |
| state.countColumn.B5 | `tabSize` 0 → `NaN` (K08) | `mirrors/cm6/state/src/column.ts:9` | panic "attempt to calculate the remainder with a divisor of zero" | none | none | differs |
| state.countColumn.B6 | `to` in UTF-16 units | `mirrors/cm6/state/src/column.ts:5` | `to: Option<usize>` in chars | none | none | differs by unit (D1) |
| state.countColumn.B7 | O(n) | `mirrors/cm6/state/src/column.ts:7-15` | O(n²) via `find_cluster_break` + an extra `Vec<char>` (R5: 40 000 chars → 63 s) | none | none | differs (complexity) |

Proposed status: PORTED, DEVIATION because B3, B5–B7 differ and nothing is tested.

### state.cm6.217 `findColumn` — cm6 `mirrors/cm6/state/src/column.ts:24`
Helpers read: `findClusterBreak` `mirrors/cm6/state/src/char.ts:9`.

| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.findColumn.B1 | Returns `i` as soon as `n >= col` (col ≤ 0 → 0, J08, J09) | `mirrors/cm6/state/src/column.ts:26` | `crates/ferrum-state/src/column.rs:34-36` | none | none | implemented, untested |
| state.findColumn.B2 | Stops at end of string | `mirrors/cm6/state/src/column.ts:27` | `crates/ferrum-state/src/column.rs:37-39` | none | none | implemented, untested |
| state.findColumn.B3 | Tab adds `tabSize - n % tabSize`, others 1; **every** character, tabs included, is advanced with `findClusterBreak` (so `"\t́"` is one step in cm6) | `mirrors/cm6/state/src/column.ts:28-29` | tab advances `i += 1` only `crates/ferrum-state/src/column.rs:40-42`, others by cluster `:43-46` | none | none | differs where clusters differ (R2) |
| state.findColumn.B4 | Column inside a tab → offset before the tab (J02) | `mirrors/cm6/state/src/column.ts:26-28` | same | none | none | implemented, untested |
| state.findColumn.B5 | Too short: `strict === true` → −1, else `string.length` (J03, J04, J12) | `mirrors/cm6/state/src/column.ts:31` | `None` / `Some(chars.len())` `crates/ferrum-state/src/column.rs:50-54` | none | none | implemented (Option instead of −1), untested |
| state.findColumn.B6 | `tabSize` 0 → returns a value (J10: 1) | `mirrors/cm6/state/src/column.ts:28` | panic (divisor of zero) | none | none | differs |
| state.findColumn.B7 | Result in UTF-16 units (J06: 2 vs 1); O(n) | `mirrors/cm6/state/src/column.ts:24-31` | chars; O(n²) | none | none | differs (unit D1, complexity) |

Proposed status: PORTED, DEVIATION because B3, B6, B7 differ and nothing is tested.

### Unfinished

none

## Chunk C4 (source: `docs/parity/state/raw/C4-behaviors.md`)


36 cm6 items (38 owned minus 2 DROP: state.cm6.043 `Facet.tag`, state.cm6.045 `FacetReader.tag`). Repro ids (Rn) refer to `docs/parity/state/raw/C4-api.md` § Reproductions. Ferrum test ids from `docs/parity/state/raw/t2-ferrum-tests.md`, cm6 ids from `docs/parity/state/raw/t1-cm6-tests.md`. "test_state#…"/"test-state#…" rows are owned by C5 and cited here only as evidence.

Shared helpers read for every item below (cm6): `nextID` `mirrors/cm6/state/src/facet.ts:4`; `sameArray` `:131-133`; `Slot` `:135`; `Provider` `:137`; `FacetProvider` + `dynamicSlot` `:139-194`; `compareArray` `:196-200`; `ensureAll` `:202-207`; `dynamicFacetSlot` `:209-258`; `initField` `:292`; `StateField.create`/`slot` `:317-351`; `PrecExtension` `:407-410`; `CompartmentInstance` `:440-443`; `DynamicSlot` `:445-449`; `Configuration` + `staticFacet` + `resolve` `:451-518`; `flatten` `:520-557`; `SlotStatus` `:559-564`; `ensureAddr` `:566-575`; `getAddr` `:577-579`; `EditorState` constructor `mirrors/cm6/state/src/state.ts:42-61`; `field` `:66-76`; `applyTransaction` `:96-125`; `facet` `:194-199`; `toJSON` `:205-216`; `fromJSON` `:222-238`; `create` `:243-253`; `Compartment.reconfigure` effect type `:446`.
Ferrum code read: `crates/ferrum-state/src/facet.rs:1-278`, `crates/ferrum-state/src/config.rs:1-255`, `crates/ferrum-state/src/state.rs:105-162, 308-365`, `crates/ferrum-state/src/transaction.rs:48-63, 163-226`.

Order of evaluation established from the code (needed by many rows):
- cm6 `EditorState.create` / constructor: resolve configuration, then compute every dynamic slot through `ensureAddr` in slot order (fields first, then facets in ascending facet id with their providers before them, `facet.ts:483-512`); any read of a not-yet-computed slot computes it on demand (`ensureAddr` `:566-575`), and re-entering a slot marked `Computing` throws "Cyclic dependency" (`:570`).
- Ferrum `EditorState::create` (`state.rs:105-135`): resolve, then initialize **all fields** with `field_values` and `dynamic_values` both empty (`:121-125`), then compute **all dynamic facets** in first-appearance order with `dynamic_values` still empty (`:128-132`). Reading another field during field init returns `None` (`config.rs:198`); reading any dynamic facet during field init or dynamic facet computation indexes an empty vector and panics (`config.rs:175`). Static facets and (during facet computation) fields read fine.
- Ferrum `apply` (`state.rs:308-365`): only `CompartmentReconfigure` effects are considered; if any, `Configuration::resolve(&base, Some(&merged))` without old state; every field is updated (existing) or initialized from the **old** state and then updated (new); every dynamic facet is recomputed unconditionally with `dynamic_values` empty.

### state.cm6.035 `StateCommand` — cm6 `mirrors/cm6/state/src/extension.ts:10`
Helpers read: none (type alias). Ferrum: `crates/ferrum-commands/src/commands.rs:20`, uses at `:28-44`, `:55`.
| id | behavior (branch/guard/edge case/side effect) | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateCommand.B1 | Receives `{state, dispatch}` and returns `boolean` (true = command applied) | `mirrors/cm6/state/src/extension.ts:10` | `crates/ferrum-commands/src/commands.rs:20` returns `Option<TransactionSpec>` | none | none | differs: return type and no `dispatch` |
| state.StateCommand.B2 | May dispatch zero, one or several transactions via the callback | `mirrors/cm6/state/src/extension.ts:10` | none (at most one spec returned) | none | none | missing |
| state.StateCommand.B3 | May return `true` without dispatching (handled, no change) | `mirrors/cm6/state/src/extension.ts:10` | none (`Some(empty spec)` is the only approximation) | none | none | differs |
| state.StateCommand.B4 | Any function value, may capture environment | `mirrors/cm6/state/src/extension.ts:10` | `fn` pointer type, no captures, `commands.rs:20` | none | none | differs |
| state.StateCommand.B5 | Defined in `@codemirror/state` | `mirrors/cm6/state/src/extension.ts:10` | defined in `ferrum-commands` | none | none | differs: placement |
Proposed status: candidate `PLACEMENT DEVIATION` with signature deviation, because the type lives in `ferrum-commands` with a different contract; the signature is decision D5.

### state.cm6.036 `Facet` — cm6 `mirrors/cm6/state/src/facet.ts:43`
Helpers read: constructor `:51-63`, `getAddr` `:577`, `EditorState.facet` `mirrors/cm6/state/src/state.ts:194-199`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.B1 | Every facet has a unique id from a module counter shared with providers and fields | `facet.ts:4,45` | `facet.rs:5,11,70` (own global atomic) | implicit in all | implicit in all | implemented, untested (ids not asserted) |
| state.Facet.B2 | Reading a facet absent from the state returns its default | `state.ts:196` | `config.rs:188` | test-facet#04 | test_facet#004 | implemented+tested |
| state.Facet.B3 | Default computed eagerly at construction (`combine([])` called once) | `facet.ts:61` | `facet.rs:68` | none | none | implemented, untested (R22 both `1`) |
| state.Facet.B4 | `enables` extensions computed at construction (function receives the facet) | `facet.ts:62` | none | none | none | missing |
| state.Facet.B5 | A facet is a `FacetReader`: readable and usable as a compute dependency | `facet.ts:43,135` | reading only: `state.rs:149-154`; no dependency concept | test-facet#08,#09,#21 | none | differs: no dependency role |
| state.Facet.B6 | Reading returns the typed `Output` | `state.ts:194-199` | `config.rs:168-189` (returns `&Output`) | test-facet#01 | test_facet#001 | implemented+tested |
| state.Facet.B7 | Value identity preserved across states when the value did not change (static: config reuse `state.ts:121`, `facet.ts:494-498`; dynamic: `facet.ts:166-169,238,245-252`) | as listed | static across non-reconfiguring tr: shared `Arc<Configuration>` `state.rs:327-328`; dynamic: always new `config.rs:143`; static after reconfigure: recombined `config.rs:121` | test-facet#08,#15,#16,#17,#21,#22 | none | differs: R10d `true`/`true`; R10, R10b, R10c, R11, R11b, R57 cm6 `true`, ferrum `false` |
Proposed status: `PORTED, DEVIATION` because B4 is missing, B5 and B7 differ, B1 and B3 are untested.

### state.cm6.037 `Facet.reader` — cm6 `mirrors/cm6/state/src/facet.ts:67`
Helpers read: `FacetReader` `:120-129`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.reader.B1 | Returns the facet itself typed as `FacetReader` (read-only handle; `reader === facet`) | `facet.ts:67` | none | none | none | missing (R36) |
| state.Facet.reader.B2 | Handle accepted by `state.facet` and as `compute` dependency | `facet.ts:135`, `state.ts:194` | none | none | none | missing (R37) |
Proposed status: `MISSING` (searched `reader`, `FacetReader` in all crates).

### state.cm6.038 `Facet^define` — cm6 `mirrors/cm6/state/src/facet.ts:70`
Helpers read: constructor `:51-63`, `sameArray` `:131`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.define.B1 | `combine` optional; default is identity (input array is the output) | `facet.ts:71` | `facet.rs:64` requires combine; `facet.rs:132` `define_list` supplies identity | test-facet#01 | test_facet#001 | differs in API shape (required argument); identity case implemented+tested |
| state.Facet.define.B2 | `compareInput` default `===` | `facet.ts:72` | none | none | none | missing |
| state.Facet.define.B3 | `compare` default: `sameArray` without combine, `===` with combine | `facet.ts:73,131-133` | none | test-facet#08 | none | missing |
| state.Facet.define.B4 | `static` flag stored | `facet.ts:74` | `facet.rs:78-85` | none | test_facet#013 (flag set, not exercised) | implemented, untested (see state.FacetConfig.static) |
| state.Facet.define.B5 | `enables` stored/evaluated | `facet.ts:75,62` | none | none | none | missing |
| state.Facet.define.B6 | `combine([])` called once at definition | `facet.ts:61` | `facet.rs:68` | none | none | implemented, untested (R22) |
| state.Facet.define.B7 | Each call yields a distinct facet | `facet.ts:45` | `facet.rs:70` | implicit | implicit (test_facet#001 two facets) | implemented+tested |
| state.Facet.define.B8 | `Facet.define()` with no config is valid | `facet.ts:70` (`= {}`) | `define_list()` `facet.rs:132` | test-facet#01 | test_facet#001 | implemented+tested |
Proposed status: `PORTED, DEVIATION` because B2, B3, B5 are missing and B1 differs in shape.

### state.cm6.039 `Facet.of` — cm6 `mirrors/cm6/state/src/facet.ts:79`
Helpers read: `FacetProvider` `:139-145`, `flatten` `:524-547`, `resolve` `:489-499`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.of.B1 | Returns a static provider extension carrying the value | `facet.ts:80` | `facet.rs:98-100` | test-facet#01 | test_facet#001 | implemented+tested |
| state.Facet.of.B2 | Each call is a new provider identity; two calls with equal values both count | `facet.ts:140` | `facet.rs:99` (no identity at all) | none | none | implemented, untested (R64 both `1,1`) |
| state.Facet.of.B3 | The same provider object included twice counts once | `facet.ts:524-531` | none | test-facet#03 | test_facet#003 (asserts only `contains(&50)`) | differs (R1 `1,50,4` vs `1,50,4,50`) |
| state.Facet.of.B4 | Providers ordered by flattened precedence order | `facet.ts:476,556` | `config.rs:65-101` | test-facet#01,#02,#05 | test_facet#001,#002,#005 | implemented+tested |
| state.Facet.of.B5 | All-static facet combined once at configuration time | `facet.ts:492-499` | `config.rs:111-122` | test-facet#13 | test_facet#013 | implemented+tested |
| state.Facet.of.B6 | Providing a facet adds its `enables` at default precedence | `facet.ts:547` | none | none | none | missing |
| state.Facet.of.B7 | On reconfigure, static value reused if provider array identical, or if new combined value `compare`s equal to the old | `facet.ts:494-498` | none (`config.rs:121` always recombines) | test-facet#17 | none | differs (R11, R12, R57) |
Proposed status: `PORTED, DEVIATION` because B3, B7 differ, B6 is missing, B2 untested.

### state.cm6.040 `Facet.compute` — cm6 `mirrors/cm6/state/src/facet.ts:90`
Helpers read: `FacetProvider.dynamicSlot` `:147-191`, `ensureAll` `:202-207`, `dynamicFacetSlot` `:209-258`, `ensureAddr` `:566-575`, `resolve` `:500-512`, `state.ts:42-61,96-125`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.compute.B1 | Throws "Can't compute a static facet" for static facets | `facet.ts:91` | `facet.rs:107` (panic, other message) | none | none | implemented (panic, different message), untested (R4) |
| state.Facet.compute.B2 | Takes a dependency list: `"doc"`, `"selection"`, facets/readers, fields | `facet.ts:90,135,152-156` | none (signature has no deps) | test-facet#08–#11,#21 | none | missing |
| state.Facet.compute.B3 | Value computed when the state is created | `facet.ts:159-162` | `state.rs:127-132` | test-facet#07 | test_facet#007 | implemented+tested |
| state.Facet.compute.B4 | On a transaction, recomputed only if (doc dep ∧ docChanged) ∨ (selection dep ∧ (docChanged ∨ tr.selection)) ∨ some facet/field dep changed | `facet.ts:164` | `state.rs:356-362` recomputes always | test-facet#08,#10,#11 | test_facet#010,#011 (do not count calls after the first) | differs (R9, R9b, R35/R56) |
| state.Facet.compute.B5 | New input kept only if `!compareInput(new, old)`; else old kept and slot unchanged | `facet.ts:166-169` | none | none | none | missing (R33) |
| state.Facet.compute.B6 | On reconfigure, an existing provider keeps its old value if all facet/field deps are identical or the recomputed value compares equal | `facet.ts:173-183` | none (recomputed, `config.rs:128-144`) | test-facet#15,#22 | none | differs (R11b) |
| state.Facet.compute.B7 | A provider new in the configuration is computed on reconfigure | `facet.ts:184-188` | `state.rs:356-362` | none | none | implemented, untested (R44 both `2`) |
| state.Facet.compute.B8 | Getter may read other facets/fields; dependencies computed on demand regardless of order | `facet.ts:202-207,566-575` | none: `dynamic_values` empty during computation (`state.rs:128-132,357-361`), read panics at `config.rs:175`; fields are readable (initialized first) | test-facet#08,#09,#21; test-state#16 | none | differs: R5, R5b, R39 panic `index out of bounds`; R8 field read works |
| state.Facet.compute.B9 | Cycle between slots throws "Cyclic dependency between fields and/or facets" | `facet.ts:570` | none | test-facet#20 | none (test_facet#020 tests no cycle) | missing (R19 panics with `index out of bounds` instead) |
| state.Facet.compute.B10 | Getter receives the new state (new doc/selection) | `state.ts:59,123-124` | `state.rs:348-360` | test-facet#10,#11 | test_facet#011 | implemented+tested |
| state.Facet.compute.B11 | Facet output = combine over providers in order, static values interleaved | `facet.ts:219-227,500-509` | `config.rs:128-144` | test-facet#07,#14 | test_facet#007,#014 | implemented+tested |
| state.Facet.compute.B12 | Facet-level `compare` decides whether the output changed; old output kept if equal | `facet.ts:238,250-252` | none | test-facet#16 | none | missing (R34) |
| state.Facet.compute.B13 | Facet slot skips recomputation when no dynamic provider changed | `facet.ts:236` | none (`state.rs:359-361`) | test-facet#08 | none | differs (R12b combine called again) |
| state.Facet.compute.B14 | Evaluation order of slots: fields, then facets by ascending facet id (`for…in` over integer keys) | `facet.ts:483-512`, `state.ts:59` | fields, then facets by first appearance in the flattened list `config.rs:61-107` | none | none | differs (R28 `a,b` vs `b,a`; only side-effect order is observable in cm6) |
| state.Facet.compute.B15 | Getters usually read positions/lengths (`doc.length`, UTF-16) | test-facet#10,#12,#14 | `len_chars()` (code points) | test-facet#12,#14 | test_facet#012,#014 | differs for non-ASCII (R-D1), D1 |
Proposed status: `PORTED, DEVIATION` because B2, B5, B9, B12 are missing, B4, B6, B8, B13, B14 differ, B1, B7 untested.

### state.cm6.041 `Facet.computeN` — cm6 `mirrors/cm6/state/src/facet.ts:97`
Helpers read: same as state.cm6.040, plus `compareArray` `:196-200`, multi flattening `:223`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.computeN.B1 | Throws "Can't compute a static facet" for static facets | `facet.ts:98` | `facet.rs:119` (panic, other message) | none | none | implemented (different message), untested (R4b) |
| state.Facet.computeN.B2 | Values flattened into the facet inputs in provider order | `facet.ts:223` | `config.rs:138-140` | test-facet#12 | test_facet#012 | implemented+tested |
| state.Facet.computeN.B3 | Recompute only when deps change; new array kept only if `!compareArray(new, old, compareInput)` | `facet.ts:164-169,196-200` | none (always recomputed) | test-facet#12 | none | differs |
| state.Facet.computeN.B4 | After a doc change the new values appear | `facet.ts:164-168` | `state.rs:356-362` | test-facet#12 | none (test_facet#012 stops before the update) | implemented, untested (R20 both `100,10,1`) |
| state.Facet.computeN.B5 | Dependency list parameter | `facet.ts:97` | none | test-facet#12 | none | missing |
Proposed status: `PORTED, DEVIATION` because B5 is missing, B3 differs, B1 and B4 untested; also renamed to `compute_multi`.

### state.cm6.042 `Facet.from` — cm6 `mirrors/cm6/state/src/facet.ts:106`
Helpers read: `compute` `:90-93`, `state.field` `state.ts:66-76`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.from.B1 | Equals `compute([field], s => get(s.field(field)))` | `facet.ts:110` | none (manual `compute` reading the field works, R8) | none | none | missing (R26) |
| state.Facet.from.B2 | `get` omitted → identity | `facet.ts:109` | none | none | none | missing |
| state.Facet.from.B3 | Recomputed only when the field slot changed (`compareF`) | `facet.ts:164,333` | none | test-state#16 | none | missing |
| state.Facet.from.B4 | Throws on static facets (through `compute`) | `facet.ts:91,110` | none | none | none | missing (R4c) |
Proposed status: `MISSING`.

### state.cm6.044 `FacetReader` — cm6 `mirrors/cm6/state/src/facet.ts:120`
Helpers read: `Slot` `:135`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetReader.B1 | Type that can be read via `state.facet` and used as a compute dependency, but cannot provide values | `facet.ts:116-129,135` | none | none | none | missing (R37) |
Proposed status: `MISSING`.

### state.cm6.046 `StateField` — cm6 `mirrors/cm6/state/src/facet.ts:296`
Helpers read: `StateField.create`/`slot` `:317-351`, `state.field` `state.ts:66-76`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.B1 | Unique id (shared counter) | `facet.ts:302,312` | `facet.rs:6,182` (separate counter) | implicit | implicit | implemented, untested |
| state.StateField.B2 | Value lives in a dynamic slot computed on demand | `facet.ts:323-351`, `state.ts:74` | eager vector `state.rs:121-125` | test-state#08 | test_state#008 | differs: B2 of FieldConfig.create (R7) |
| state.StateField.B3 | Field is itself the extension | `facet.ts:363` | `facet.rs:194` | test-facet#18 | test_facet#018 | implemented+tested |
| state.StateField.B4 | Field included several times is created once | `facet.ts:524-526` | `config.rs:92-98` (dedup by id) | none | none | implemented, untested (R41) |
| state.StateField.B5 | `provides` computed once at definition | `facet.ts:298,313` | none | none | none | missing |
| state.StateField.B6 | Spec retained for JSON hooks | `facet.ts:307` | none | test-state#10 | none | missing |
Proposed status: `PORTED, DEVIATION` because B5, B6 missing, B2 differs, B1, B4 untested.

### state.cm6.047 `StateField^define` — cm6 `mirrors/cm6/state/src/facet.ts:311`
Helpers read: constructor `:300-308`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.define.B1 | `create` and `update` required | `facet.ts:262,266,312` | `facet.rs:176-186` | test-state#08 | test_state#008 | implemented+tested |
| state.StateField.define.B2 | `compare` defaults to `===` | `facet.ts:312` | none | none | none | missing (R24b) |
| state.StateField.define.B3 | `provide(field)` called once at definition | `facet.ts:313` | none | none | none | missing (R25) |
| state.StateField.define.B4 | `toJSON`/`fromJSON` kept in `spec` | `facet.ts:307,312` | none | test-state#10 | none | missing (R27) |
| state.StateField.define.B5 | Id from the counter shared with facets | `facet.ts:312` | `facet.rs:182` separate counter | none | none | differs (unobservable in cm6 except ordering; D6) |
Proposed status: `PORTED, DEVIATION` because B2–B4 are missing.

### state.cm6.048 `StateField.init` — cm6 `mirrors/cm6/state/src/facet.ts:356`
Helpers read: `initField` `:292`, `StateField.create` `:317-320`, slot `reconfigure` `:337-349`, `fromJSON` `state.ts:226-231`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.init.B1 | Returns `[field, initField.of({field, create})]` | `facet.ts:357` | none | test-state#09 | none (test_state#009 uses a plain initializer) | missing (R23) |
| state.StateField.init.B2 | At create, the override is used instead of the spec's `create` | `facet.ts:318-319` | none | test-state#09 | none | missing |
| state.StateField.init.B3 | On reconfigure, a new/different override re-initializes the field | `facet.ts:338-342` | none | none | none | missing (R66) |
| state.StateField.init.B4 | Used by `EditorState.fromJSON` to restore fields | `state.ts:231` | none | test-state#10 | none | missing |
| state.StateField.init.B5 | `initField` is a static facet | `facet.ts:292` | none | none | none | missing |
Proposed status: `MISSING`.

### state.cm6.049 `StateField.extension` — cm6 `mirrors/cm6/state/src/facet.ts:363`
Helpers read: `flatten` `:542-544`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.extension.B1 | Returns an extension enabling the field | `facet.ts:363` | `facet.rs:194-196` | test-facet#18,#19 | test_facet#018,#019; test_state#008 | implemented+tested |
| state.StateField.extension.B2 | Repeated inclusion → one field, created once | `facet.ts:524-526` | `config.rs:92-98` | none | none | implemented, untested (R41) |
Proposed status: `PORTED, DEVIATION` because B2 is untested.

### state.cm6.050 `Extension` — cm6 `mirrors/cm6/state/src/facet.ts:374`
Helpers read: `flatten` `:520-557`, `resolve` `:474-477`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Extension.B1 | Nested arrays flattened in order | `facet.ts:532-533` | `config.rs:249-253` | test-facet#02 | test_facet#002 | implemented+tested |
| state.Extension.B2 | Any object with an `extension` property is unwrapped recursively | `facet.ts:548-552` | none (closed enum; only `From<Vec<Extension>>` `facet.rs:274`) | none | none | differs (R65) |
| state.Extension.B3 | Unrecognized value throws "Unrecognized extension value in extension set" | `facet.ts:550` | not representable (closed enum) | none | none | not applicable in Rust (type system); R30 |
| state.Extension.B4 | Self-referential `extension` throws the "multiple instances of @codemirror/state" error | `facet.ts:551` | not representable | none | none | not applicable in Rust |
| state.Extension.B5 | Same extension object (provider, array, prec wrapper, compartment instance, field) included once, at the highest precedence where it occurs; lower-precedence repeats ignored, higher-precedence repeat moves it | `facet.ts:522-531` | none (fields only, by id, `config.rs:94`) | test-facet#03 | test_facet#003 (weakened) | differs (R1, R2, R2b, R2c; oracle 343/2000 mismatches) |
| state.Extension.B6 | Precedence buckets concatenated highest→lowest | `facet.ts:521,556` | `config.rs:45-52,65` | test-facet#05 | test_facet#005 | implemented+tested (oracle 2000/2000) |
| state.Extension.B7 | Fields and providers resolved from one ordered list | `facet.ts:474-477` | `config.rs:65-101` | test-facet#18 | test_facet#018 | implemented+tested |
| state.Extension.B8 | (Ferrum-only failure mode) public type-erased variants accept values of the wrong type | n/a | `facet.rs:246-258`, `facet.rs:153` | n/a | none | Ferrum-only: R45, R45b, R46, R50 panic at create/read |
Proposed status: `PORTED, DEVIATION` because B5 is missing, B2 differs; B3/B4 are replaced by static typing (RUST-IDIOMATIC part), B8 is a new failure mode.

### state.cm6.051 `Prec` — cm6 `mirrors/cm6/state/src/facet.ts:390`
Helpers read: `Prec_` `:376`, `prec` `:378-380`, `PrecExtension` `:407`, `flatten` `:523,540-547,555`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.B1 | Five levels, highest=0 … lowest=4 | `facet.ts:376` | `facet.rs:23-29` | test-facet#05 | test_facet#005 | implemented+tested |
| state.Prec.B2 | No explicit precedence → nearest parent's, else default | `facet.ts:523,533,555` | `config.rs:55,249-252` | test-facet#06 | test_facet#006 | implemented+tested |
| state.Prec.B3 | Inner explicit precedence overrides the outer | `facet.ts:541` | `config.rs:237-238` | none | none | implemented, untested (R3; oracle) |
| state.Prec.B4 | Final order: by precedence, then by order within a precedence | `facet.ts:556` | `config.rs:65` | test-facet#05 | test_facet#005 | implemented+tested |
| state.Prec.B5 | `Prec.x(ext)` wraps in a new `PrecExtension` | `facet.ts:379` | `facet.rs:269-271` (`with_prec(self, prec)`) | test-facet#05 | test_facet#005 | implemented+tested (RUST-IDIOMATIC shape) |
| state.Prec.B6 | `enables` extensions always get default precedence | `facet.ts:547` | none | none | none | missing (R62) |
| state.Prec.B7 | Field `provides` inherit the field's precedence | `facet.ts:544` | none | none | none | missing (R61) |
Proposed status: `PORTED, DEVIATION` because B3 is untested and B6, B7 depend on missing features (`enables`, `provide`).

### state.cm6.052 `Prec.highest` — cm6 `mirrors/cm6/state/src/facet.ts:393`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.highest.B1 | Places the extension in bucket 0 (before everything else) | `facet.ts:376,393` | `facet.rs:24` + `config.rs:237-238` | test-facet#05,#06 | test_facet#005,#006 | implemented+tested (R29) |
Proposed status: `PORTED, RUST-IDIOMATIC` (`ext.with_prec(Prec::Highest)` instead of `Prec.highest(ext)`, identical ordering, tested).

### state.cm6.053 `Prec.high` — cm6 `mirrors/cm6/state/src/facet.ts:396`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.high.B1 | Bucket 1 | `facet.ts:376,396` | `facet.rs:25` | test-facet#05 | test_facet#005 | implemented+tested |
Proposed status: `PORTED, RUST-IDIOMATIC`.

### state.cm6.054 `Prec.default` — cm6 `mirrors/cm6/state/src/facet.ts:399`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.default.B1 | Bucket 2 | `facet.ts:376,399` | `facet.rs:26` | none | none | implemented, untested (oracle) |
| state.Prec.default.B2 | Explicit `default` inside a higher/lower parent resets the precedence | `facet.ts:541` | `config.rs:237-238` | none | none | implemented, untested (oracle `(p 2 …)` nodes nested in other `p` nodes) |
Proposed status: `PORTED, DEVIATION` (untested: B1, B2).

### state.cm6.055 `Prec.low` — cm6 `mirrors/cm6/state/src/facet.ts:401`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.low.B1 | Bucket 3 | `facet.ts:376,401` | `facet.rs:27` | test-facet#05 | test_facet#005 | implemented+tested |
Proposed status: `PORTED, RUST-IDIOMATIC`.

### state.cm6.056 `Prec.lowest` — cm6 `mirrors/cm6/state/src/facet.ts:404`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.lowest.B1 | Bucket 4 (after everything else) | `facet.ts:376,404` | `facet.rs:28` | none | none | implemented, untested (R3, oracle) |
Proposed status: `PORTED, DEVIATION` (untested: B1).

### state.cm6.057 `Compartment` — cm6 `mirrors/cm6/state/src/facet.ts:418`
Helpers read: `Configuration.compartments` `:455`, `flatten` `:534-539`, `applyTransaction` `state.ts:97-105`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Compartment.B1 | Each `new Compartment` is a distinct compartment (object identity) | `facet.ts:418` | `facet.rs:218-234` (`CompartmentId` counter; `Copy` values compare by id) | test-state#12 | test_state#012 | implemented+tested |
| state.Compartment.B2 | The state records, per present compartment, its current content; dropped compartments are forgotten | `facet.ts:455,472,538`, `state.ts:100-104` | `config.rs:26,40-43`, `state.rs:319-322` (stale entries kept) | test-state#14,#15 | test_state#014 (first half only), test_state#015 (no reconfigure) | differs (R14, R14b) |
| state.Compartment.B3 | `get(state)` returns current content | `facet.ts:431-433` | none | test-state#12,#15 | none | missing |
Proposed status: `PORTED, DEVIATION` because B2 differs and B3 is missing.

### state.cm6.058 `Compartment.of` — cm6 `mirrors/cm6/state/src/facet.ts:421`
Helpers read: `CompartmentInstance` `:440-443`, `flatten` `:524-539`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Compartment.of.B1 | Wraps an extension in a compartment instance | `facet.ts:421` | `facet.rs:237-239` | test-state#12 | test_state#012, test_facet#015 | implemented+tested |
| state.Compartment.of.B2 | Content = current content from the compartments map if present, else the wrapped extension | `facet.ts:537` | `config.rs:241-246` | test-state#12,#14 | test_state#012,#014 | implemented+tested |
| state.Compartment.of.B3 | Only compartments present in the new configuration are recorded | `facet.ts:472,538` | none (`config.rs:40-43` copies all overrides) | test-state#15 | none | differs (R14 `10,2` vs `0,2`) |
| state.Compartment.of.B4 | Two instances of one compartment, or nesting a compartment in itself, throws `RangeError("Duplicate use of compartment in extensions")` | `facet.ts:535-536` | none | test-state#13 | test_state#013 (asserts `contains(1) \|\| contains(2)`) | differs: R13 `1,1`; nested → stack overflow abort (R13b) |
| state.Compartment.of.B5 | The same instance object twice is deduplicated, not an error | `facet.ts:524-529` | none | none | none | differs (R13c `1` vs `1,1`) |
| state.Compartment.of.B6 | When a higher-precedence repeat of an instance is found, its map entry is dropped and re-added | `facet.ts:529` | none | none | none | missing |
| state.Compartment.of.B7 | Content inherits the instance's precedence | `facet.ts:539` | `config.rs:247` | none | none | implemented, untested |
Proposed status: `PORTED, DEVIATION` because B3–B6 differ or are missing and B7 is untested.

### state.cm6.059 `Compartment.reconfigure` — cm6 `mirrors/cm6/state/src/facet.ts:425`
Helpers read: `state.ts:96-125` (`applyTransaction`), `:446`; `resolve` `:469-517`; slot `reconfigure` functions `:173-189`, `:242-256`, `:337-349`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Compartment.reconfigure.B1 | Returns a `StateEffect` of type `Compartment.reconfigure` carrying `{compartment, extension}` | `facet.ts:426`, `state.ts:446` | `transaction.rs:55-63` (`StateEffect::of(CompartmentReconfigure)`) | test-state#12 | none calls the method; test_state#012,#014, test_facet#015,#018 build the effect by hand | implemented, untested (method) |
| state.Compartment.reconfigure.B2 | Applying copies the compartments map, sets the new content, resolves a new configuration with the old state | `state.ts:99-105,116` | `state.rs:310-329` (no old state) | test-state#12,#14 | test_state#012,#014 | implemented+tested (content); value preservation differs (B5) |
| state.Compartment.reconfigure.B3 | Several effects for one compartment: last wins | `state.ts:104` | `state.rs:313,321` | none | none | implemented, untested (R42) |
| state.Compartment.reconfigure.B4 | Reconfiguring a compartment absent from the configuration has no lasting effect | `facet.ts:472,538` | none | none | none | differs (R14b `1 \| 9` vs `1 \| 5`) |
| state.Compartment.reconfigure.B5 | Values preserved across the reconfigure: static facets (`:494-498`), dynamic facets (`:173-183`, `:242-256`), fields (`:343-345`) | as listed | fields: `state.rs:334-336`; facets: none | test-facet#15,#16,#17,#21,#22; test-state#11 | test_facet#018 (field value only) | fields implemented+tested (R16); facets differ (R11, R11b, R57) |
| state.Compartment.reconfigure.B6 | New fields are created with an intermediate state having the new configuration and old doc/selection, then updated by the transaction | `state.ts:117-118,123-124`, `facet.ts:347` | `state.rs:337-340` uses the old state | test-facet#18,#19 | test_facet#018 | differs: R17 `15` vs `Some(5)`; doc same (R17b); create-then-update order same (R18) |
| state.Compartment.reconfigure.B7 | A field removed by the reconfigure disappears | `facet.ts:483-486` | `state.rs:332-341` | test-state#11 | none | implemented, untested (R16b) |
| state.Compartment.reconfigure.B8 | `tr.reconfigured` becomes true (C5 item) | `transaction.ts:237` | none (C5) | none | none | missing (recorded for C5) |
Proposed status: `PORTED, DEVIATION` because B4, B5 (facets), B6 differ, B1, B3, B7 untested.

### state.cm6.060 `Compartment.get` — cm6 `mirrors/cm6/state/src/facet.ts:431`
Helpers read: `Configuration.compartments` `:455`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Compartment.get.B1 | Returns the current content (after reconfigures) | `facet.ts:432` | none | test-state#12 | none | missing (R15) |
| state.Compartment.get.B2 | Returns `undefined` when the compartment is not in the state | `facet.ts:432` | none | test-state#15 | none | missing (R15) |
Proposed status: `MISSING`.

### state.cm6.134 `combineConfig` — cm6 `mirrors/cm6/state/src/config.ts:8`
Helpers read: none.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.combineConfig.B1 | Iterates configs in order and their own keys; unset key takes the value | `config.ts:14-16` | none | none | none | missing |
| state.combineConfig.B2 | `===`-equal value or `undefined` value: no conflict | `config.ts:17` | none | none | none | missing (R31b, R31c) |
| state.combineConfig.B3 | Conflict with a combine function for the key: `combine(current, value)` | `config.ts:18` | none | none | none | missing (R31b) |
| state.combineConfig.B4 | Conflict without combine: throws "Config merge conflict for field <key>" | `config.ts:19` | none | none | none | missing (R31) |
| state.combineConfig.B5 | Defaults fill only keys still `undefined` | `config.ts:21` | none | none | none | missing (R31d) |
| state.combineConfig.B6 | Returns a fresh object | `config.ts:13,22` | none | none | none | missing |
Proposed status: `MISSING` (searched `combine_config`, `combineconfig`, `merge conflict`, `fn combine` in all crates). `crates/ferrum-commands/src/history.rs:104-125` hand-rolls one specific merge.

### state.cm6.248 `Facet.define config.combine` — cm6 `mirrors/cm6/state/src/facet.ts:12`
Helpers read: constructor `:61`, `resolve` `:492-499`, `dynamicFacetSlot.get/update/reconfigure` `:219-256`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.combine.B1 | Optional; absent → output is the input array | `facet.ts:71` | required (`facet.rs:64`); `define_list` `facet.rs:132` | test-facet#01 | test_facet#001 | differs in shape; identity case tested |
| state.FacetConfig.combine.B2 | Called with `[]` at definition for the default | `facet.ts:61` | `facet.rs:68` | none | none | implemented, untested (R22) |
| state.FacetConfig.combine.B3 | All-static facet: called once per configuration, skipped when providers are the same array as the old configuration's | `facet.ts:494-497` | `config.rs:121` (always) | test-facet#17 | none | differs (R12 `2` vs `3`) |
| state.FacetConfig.combine.B4 | Dynamic facet: called only when a dynamic input changed | `facet.ts:236-237` | `config.rs:143` every transaction | test-facet#08 | none | differs (R12b `2` vs `3`) |
| state.FacetConfig.combine.B5 | Combined output compared with `compare`; old output kept when equal | `facet.ts:238,250-252,498` | none | test-facet#16 | none | missing |
| state.FacetConfig.combine.B6 | Inputs in precedence order, multi providers flattened | `facet.ts:219-226` | `config.rs:129-142` | test-facet#13,#14 | test_facet#013,#014 | implemented+tested |
Proposed status: `PORTED, DEVIATION` because B3, B4 differ, B5 missing, B2 untested.

### state.cm6.249 `Facet.define config.compare` — cm6 `mirrors/cm6/state/src/facet.ts:17`
Helpers read: `:73`, `:131-133`, `:238`, `:245-252`, `:498`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.compare.B1 | Custom output comparison keeps the old output when equal (transaction, reconfigure, static recombine) | `facet.ts:238,250,498` | none | test-facet#16 | none | missing (R32, R34) |
| state.FacetConfig.compare.B2 | Default: `===` with combine, `sameArray` without | `facet.ts:73` | none | test-facet#08,#17 | none | missing (R10, R11) |
Proposed status: `MISSING`.

### state.cm6.250 `Facet.define config.compareInput` — cm6 `mirrors/cm6/state/src/facet.ts:20`
Helpers read: `:72`, `:149`, `:166`, `:180`, `:196-200`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.compareInput.B1 | Compares a single provider's new and old input; equal → slot unchanged | `facet.ts:149,166` | none | none | none | missing (R33) |
| state.FacetConfig.compareInput.B2 | Multi providers compared element-wise via `compareArray` | `facet.ts:166,196-200` | none | none | none | missing |
| state.FacetConfig.compareInput.B3 | Default `===` | `facet.ts:72` | none | test-facet#10 (count stays 1) | none | missing |
Proposed status: `MISSING`.

### state.cm6.251 `Facet.define config.static` — cm6 `mirrors/cm6/state/src/facet.ts:22`
Helpers read: `:58`, `:74`, `:91`, `:98`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.static.B1 | `compute`/`computeN`/`from` throw "Can't compute a static facet" | `facet.ts:91,98` | `facet.rs:107,119` (panic "Cannot attach dynamic provider to static facet") | none | none | implemented (different message), untested (R4, R4b) |
| state.FacetConfig.static.B2 | Static facets only ever get static providers → value stored in `staticValues` | `facet.ts:492-499` | `config.rs:111-122` | test-facet#13 | test_facet#013 | implemented+tested |
| state.FacetConfig.static.B3 | (Ferrum-only) check bypassed through public `Extension::FacetDynamic`; `AnyFacet::is_static` never consulted | n/a | `facet.rs:38,248`, `config.rs` (no call) | n/a | none | Ferrum-only gap (R46b `9`) |
Proposed status: `PORTED, DEVIATION` because B1 is untested with a different message and B3 lets the invariant be broken.

### state.cm6.252 `Facet.define config.enables` — cm6 `mirrors/cm6/state/src/facet.ts:29`
Helpers read: `:49`, `:62`, `:547`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.enables.B1 | Extension, or function of the facet, evaluated at definition | `facet.ts:62` | none | none | none | missing |
| state.FacetConfig.enables.B2 | Added when the facet has at least one provider, at default precedence, deduplicated | `facet.ts:547,524` | none | none | none | missing (R62 `h,a,e`, R63 `e`) |
| state.FacetConfig.enables.B3 | Not added when the facet is absent (default read) | `facet.ts:547` | none | none | none | missing (R21) |
Proposed status: `MISSING`.

### state.cm6.253 `StateField.define config.create` — cm6 `mirrors/cm6/state/src/facet.ts:262`
Helpers read: `StateField.create` `:317-320`, slot `create` `:326-329`, slot `reconfigure` `:337-349`, `ensureAddr` `:566-575`, `state.ts:117-118,243-253`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.create.B1 | Called once when the state is created | `facet.ts:326-328` | `state.rs:121-125` | test-state#08 | test_state#008 | implemented+tested |
| state.FieldConfig.create.B2 | Receives the state under construction; can read other fields and any facet (computed on demand) | `facet.ts:327`, `state.ts:66-76,194-199` | other fields → `None` (`config.rs:198`); dynamic facets → panic (`config.rs:175`); static facets OK | test-state#08; test-facet#19 | test_state#008 (passes only because field1's init is 0 and the test uses `unwrap_or(0)`) | differs (R6 panic, R7/R7b `None` vs `15`, R6b OK) |
| state.FieldConfig.create.B3 | Replaced by a `StateField.init` override when present | `facet.ts:318-319` | none | test-state#09 | none | missing |
| state.FieldConfig.create.B4 | Fields new after reconfiguration are created from a state with the new configuration (old doc/selection) | `state.ts:117-118`, `facet.ts:347` | `state.rs:338` (`field.init(self)`, old configuration) | test-facet#19 | none (test_facet#019 does not reconfigure) | differs (R17) |
| state.FieldConfig.create.B5 | A field reading itself throws the cyclic error | `facet.ts:570` | none | none | none | differs (R19b `Some(0)`) |
Proposed status: `PORTED, DEVIATION` because B2, B4, B5 differ and B3 is missing.

### state.cm6.254 `StateField.define config.update` — cm6 `mirrors/cm6/state/src/facet.ts:266`
Helpers read: slot `update` `:330-336`, `applyTransaction` `state.ts:117-124`, `Transaction.state` `transaction.ts` (C5).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.update.B1 | Called on every transaction with the old value and the transaction | `facet.ts:331-332` | `state.rs:332-336` | test-state#08,#09 | test_state#008,#009, test_facet#019 | implemented+tested |
| state.FieldConfig.update.B2 | If `compare(old, new)` the old value is kept and dependents see no change | `facet.ts:333` | none (always a new `Arc`, `facet.rs:212`) | none | none | missing (R24, R58) |
| state.FieldConfig.update.B3 | Fields added by a reconfiguration are created, then updated by the same transaction | `state.ts:117-124` | `state.rs:337-340` | test-facet#18 | test_facet#018 (value only) | implemented+tested (R18 event order matches) |
| state.FieldConfig.update.B4 | `update` may read `tr.state` (the state being built; slots computed on demand) | `state.ts:57-59` | `transaction.rs:176-178` re-runs `apply` → infinite recursion | test-state#16 (`tr.state.doc`) | none | differs: stack overflow (bin `trstate`) |
| state.FieldConfig.update.B5 | Updated values of other fields are visible through `tr.state.field` | `state.ts:66-76` | none (same root as B4) | none | none | differs |
Proposed status: `PORTED, DEVIATION` because B2 is missing and B4, B5 differ.

### state.cm6.255 `StateField.define config.compare` — cm6 `mirrors/cm6/state/src/facet.ts:272`
Helpers read: `:305`, `:312`, `:333`, dependent `compute` `:164`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.compare.B1 | Equal → old value kept, slot not marked changed, dependent facets not recomputed | `facet.ts:333,164` | none | none | none | missing (R24) |
| state.FieldConfig.compare.B2 | Default `===` | `facet.ts:312` | none | test-state#16 (identity of facet computed from field) | none | missing (R24b) |
Proposed status: `MISSING`.

### state.cm6.256 `StateField.define config.provide` — cm6 `mirrors/cm6/state/src/facet.ts:280`
Helpers read: `:298`, `:313`, `:544`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.provide.B1 | Called once with the field at definition | `facet.ts:313` | none | none | none | missing |
| state.FieldConfig.provide.B2 | Provided extensions included wherever the field is, at the field's precedence | `facet.ts:544` | none | none | none | missing (R61 `3,1`) |
| state.FieldConfig.provide.B3 | Typical use `facet.from(field)` | `facet.ts:280` doc | none | none | none | missing (R25 `3`) |
Proposed status: `MISSING`.

### state.cm6.257 `StateField.define config.toJSON` — cm6 `mirrors/cm6/state/src/facet.ts:285`
Helpers read: `EditorState.toJSON` `state.ts:205-216`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.toJSON.B1 | `toJSON(value, state)` called by `state.toJSON(fields)` for fields present in the state | `state.ts:211-213` | none | test-state#10 | none | missing (R27) |
Proposed status: `MISSING`.

### state.cm6.258 `StateField.define config.fromJSON` — cm6 `mirrors/cm6/state/src/facet.ts:289`
Helpers read: `EditorState.fromJSON` `state.ts:222-238`, `StateField.init` `facet.ts:356-358`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.fromJSON.B1 | `fromJSON(json, state)` called through a `field.init` override when the JSON has the property | `state.ts:226-231` | none | test-state#10 | none | missing (R27 `200`) |
Proposed status: `MISSING`.

### Unfinished

none

## Chunk C5 (source: `docs/parity/state/raw/C5-behaviors.md`)


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

### Unfinished
none

## Chunk C6 (source: `docs/parity/state/raw/C6-behaviors.md`)


Owned cm6 items: 47 (none DROP); one section each, in A1 id order. cm6 paths are relative to `mirrors/cm6/`, Ferrum paths to `ferrum/`. Test ids: cm6 `test-rangeset#NN` (T1), Ferrum `test_rangeset#NNN` (T2). "R<n>" = reproduction in `docs/parity/state/raw/C6-api.md` `## Reproductions`. "Replay" = R33 (cm6 cases re-run against Ferrum with cm6 inputs). That is evidence collected during this audit, not a Ferrum test. Where the Ferrum test column says "none", no test in `crates/ferrum-state/src/tests/test_rangeset.rs` asserts the behavior.

Shared cm6 helpers, read for all sections below: `cmpVal` `mirrors/cm6/state/src/rangeset.ts:38`, `cmpRange` `:58`, `C` `:95`, `Chunk` `:102-179` (`findIndex` `:116`, `between` `:128`, `map` `:133`), `last` `:470`, `lazySort` `:472`, `RangeSetBuilder` `:487-568`, `findSharedChunks` `:570`, `LayerCursor` `:584-668`, `HeapCursor` `:670-723`, `heapBubble` `:725`, `SpanCursor` `:741-866`, `compare` `:868`, `sameValues` `:898`, `remove` `:904`, `insert` `:909`, `findMinIndex` `:914`, and from `mirrors/cm6/state/src/change.ts`: `ChangeDesc.mapPos` `:122`, `touchesRange` `:147`, `iterGaps` `:56`, `empty`/`length`.
Ferrum counterparts read: `crates/ferrum-state/src/rangeset.rs:1-962` in full, `crates/ferrum-state/src/change.rs:26-195` (`ChangeDesc::create`, `empty`, `length`, `new_length`, `is_empty`, `iter_gaps`, `map_pos`), `:318-376` (`ChangeSet::create_set`, `desc`).

### state.cm6.170 `RangeValue` — cm6 `mirrors/cm6/state/src/rangeset.ts:5`
Helpers read: prototype defaults `mirrors/cm6/state/src/rangeset.ts:34-36`, `cmpVal` `:38`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.B1 | Abstract base class; values subclass it and may override `eq`, `startSide`, `endSide`, `mapMode`, `point`, `range` | `mirrors/cm6/state/src/rangeset.ts:5-32` | trait with provided methods `crates/ferrum-state/src/rangeset.rs:14-25` (no `range`) | test-rangeset#01–#47 (`Value`) | test_rangeset#001–#047 (`TestVal`) | differs: there is no `range` hook (see state.RangeValue.range) |
| state.RangeValue.B2 | Defaults on the prototype: `startSide = endSide = 0`, `point = false`, `mapMode = TrackDel` | `mirrors/cm6/state/src/rangeset.ts:34-36` | `crates/ferrum-state/src/rangeset.rs:16-22` | none (the test `Value` overrides every default) | none (`TestVal` overrides sides and point) | implemented, untested |
| state.RangeValue.B3 | Values of different classes can live in one set (`RangeSet<RangeValue>`); `cmpVal` treats different constructors as unequal before calling `eq` | `mirrors/cm6/state/src/rangeset.ts:39` | a set is generic over one `T`; there is no constructor check (`rangeset.rs:840`, `:870`) | none | none | differs: the generic `T` replaces the constructor check (an enum `T` must handle it inside `eq_val`) |
| state.RangeValue.B4 | Any `number` is allowed for sides (the view uses values up to ±6e8) | `mirrors/cm6/state/src/rangeset.ts:15-17`; `mirrors/cm6/view/src/decoration.ts:171-185` | `i32` sides; truncated to `i8` in `map` (`rangeset.rs:247`, `:250`, `:259`, `:260`) | none | none | differs: R3, R31 |
Proposed status: PORTED, DEVIATION because the default `eq` differs (B3/eq row), sides are truncated during mapping (B4), and there is no `range` hook.

### state.cm6.171 `RangeValue.eq` — cm6 `mirrors/cm6/state/src/rangeset.ts:11`
Helpers read: `cmpVal` `mirrors/cm6/state/src/rangeset.ts:38`, `sameValues` `:898`, `compare` `:868`, `RangeSet.eq` `:389`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.eq.B1 | Default implementation compares by identity (`this == other`) | `mirrors/cm6/state/src/rangeset.ts:11` | default `self == other` (`PartialEq`) `crates/ferrum-state/src/rangeset.rs:24` | none | none | differs: structural, not identity (R20) |
| state.RangeValue.eq.B2 | `cmpVal`: identical objects are equal without calling `eq`; otherwise the constructors must match, then `eq` decides | `mirrors/cm6/state/src/rangeset.ts:39` | `eq_val` called directly `crates/ferrum-state/src/rangeset.rs:840`, `:870` | test-rangeset#27 (identical names), #30 | none (test_rangeset#027, #030 assert nothing) | differs: no identity shortcut and no constructor check; untested |
| state.RangeValue.eq.B3 | `sameValues` compares element-wise in order (`a[i] != b[i] && !cmpVal`) | `mirrors/cm6/state/src/rangeset.ts:898-902` | `same_values` `crates/ferrum-state/src/rangeset.rs:835-845` | test-rangeset#25–#34 | none | implemented, untested |
| state.RangeValue.eq.B4 | Used by `compare` (ranges and points) and `RangeSet.eq` | `mirrors/cm6/state/src/rangeset.ts:882-888`, `:401-403` | `compare_cursors` `rangeset.rs:868-876`; no `RangeSet::eq` | test-rangeset#25–#34 | none | implemented for compare, untested; missing for `eq` |
Proposed status: PORTED, DEVIATION because the default semantics differ (B1, R20) and nothing is tested.

### state.cm6.172 `RangeValue.startSide` — cm6 `mirrors/cm6/state/src/rangeset.ts:15`
Helpers read: `cmpRange` `:58`, `RangeSetBuilder.addInner` `:524`, `Chunk.findIndex` `:116`, `Chunk.map` `:133`, `LayerCursor.compare` `:664`, `RangeSet.spans` `:428-430`, `ChangeDesc.mapPos` `change.ts:122`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.startSide.B1 | Default 0 | `mirrors/cm6/state/src/rangeset.ts:34` | `rangeset.rs:16` | none | none | implemented, untested |
| state.RangeValue.startSide.B2 | Orders ranges starting at the same position (`cmpRange`, builder strict check, `of` sort) | `mirrors/cm6/state/src/rangeset.ts:58-60`, `:526` | `rangeset.rs:155-158`, `:372-375` | test-rangeset#02 (second assertion) | none (test_rangeset#002 checks only the from-order case) | implemented, untested in Ferrum (Replay #02 passes) |
| state.RangeValue.startSide.B3 | Used as `assoc` when mapping the start (empty ranges: with `mapMode`; non-empty: `Simple`); full `number` precision | `mirrors/cm6/state/src/rangeset.ts:139`, `:147` | `rangeset.rs:247`, `:259` (`as i8`) | test-rangeset#11, #12, #19, #22–#24 | test_rangeset#011, #012, #019 | differs: truncated to `i8` (R3, R31); small sides tested |
| state.RangeValue.startSide.B4 | Heap/iteration order: `from`, then `startSide` | `mirrors/cm6/state/src/rangeset.ts:665` | `rangeset.rs:533-538` (cursor), `:194-197` (`iter`) | test-rangeset#19, #22 | test_rangeset#019 | implemented+tested |
| state.RangeValue.startSide.B5 | In spans, a point with `startSide < 0` reports `openCount = active.length` | `mirrors/cm6/state/src/rangeset.ts:429` | `rangeset.rs:943-944` | none | none | implemented, untested |
Proposed status: PORTED, DEVIATION because of B3 (i8 truncation).

### state.cm6.173 `RangeValue.endSide` — cm6 `mirrors/cm6/state/src/rangeset.ts:17`
Helpers read: as state.cm6.172 plus `SpanCursor.next` `:803`, `findMinIndex` `:914`, `activeForPoint` `:850`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.endSide.B1 | Default 0 | `mirrors/cm6/state/src/rangeset.ts:34` | `rangeset.rs:18` | none | none | implemented, untested |
| state.RangeValue.endSide.B2 | Builder: a range overlaps the previous one in its layer when `from < lastTo` or `from == lastTo && startSide < last.endSide`; it then goes to the next layer | `mirrors/cm6/state/src/rangeset.ts:525`, `:528` | `rangeset.rs:371`, `:378-380` | test-rangeset#01, #20 | test_rangeset#001 | implemented+tested |
| state.RangeValue.endSide.B3 | Used as `assoc` when mapping the end (always `Simple` mode) | `mirrors/cm6/state/src/rangeset.ts:143`, `:148` | `rangeset.rs:250` (uses `map_mode`), `:260` (`as i8`) | test-rangeset#11, #13, #16 | test_rangeset#011, #013, #016 | differs: R3 (truncation), R32 (mode) |
| state.RangeValue.endSide.B4 | A non-empty range that collapses with `startSide > 0 && endSide <= 0` is dropped | `mirrors/cm6/state/src/rangeset.ts:149` | `rangeset.rs:262` | test-rangeset#14 | test_rangeset#014 | implemented+tested |
| state.RangeValue.endSide.B5 | A mapped range is dropped when `(newTo - newFrom \|\| endSide - startSide) < 0` (an empty result with `endSide < startSide`) | `mirrors/cm6/state/src/rangeset.ts:151` | none (`rangeset.rs:254` keeps `new_to >= new_pos`) | none | none | missing: R5 |
| state.RangeValue.endSide.B6 | Cursor/active end ordering: `findIndex(end)`, `forward`, `findMinIndex`, and activeForPoint's `endSide >= point.endSide` | `mirrors/cm6/state/src/rangeset.ts:121`, `:623`, `:917`, `:855` | `rangeset.rs:67-68`, `:494`, `:709`, `:816` | test-rangeset#40 | test_rangeset#040 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B3 and B5.

### state.cm6.174 `RangeValue.mapMode` — cm6 `mirrors/cm6/state/src/rangeset.ts:22`
Helpers read: `Chunk.map` `:133-151`, `ChangeDesc.mapPos` `change.ts:122-142`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.mapMode.B1 | Default `MapMode.TrackDel` | `mirrors/cm6/state/src/rangeset.ts:36` | `rangeset.rs:22` | test-rangeset#15 (point at 3 inside deletion 0..4 dropped) | test_rangeset#015 | implemented+tested |
| state.RangeValue.mapMode.B2 | Applied only to the start mapping of empty ranges (`curFrom == curTo`); `null` drops the range | `mirrors/cm6/state/src/rangeset.ts:139-140` | `rangeset.rs:247` | test-rangeset#15 | test_rangeset#015 | implemented+tested |
| state.RangeValue.mapMode.B3 | NOT applied to the end-side mapping of an empty range with `startSide != endSide` (`Simple`) | `mirrors/cm6/state/src/rangeset.ts:143` | differs `rangeset.rs:250` (passes `map_mode()`; `None` → keeps `new_to = new_pos`) | none | none | differs: R32 |
| state.RangeValue.mapMode.B4 | NOT applied to non-empty ranges (both ends `Simple`) | `mirrors/cm6/state/src/rangeset.ts:147-148` | `rangeset.rs:259-260` (`MapMode::Simple`) | test-rangeset#16 | test_rangeset#016 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B3.

### state.cm6.175 `RangeValue.point` — cm6 `mirrors/cm6/state/src/rangeset.ts:28`
Helpers read: `addInner` `:537`, `Chunk.map` `:153`, `LayerCursor.next` `:640`, `HeapCursor.from` `:686`, `SpanCursor.next` `:826-841`, `RangeSet.compare` `:376-377`, `findSharedChunks` `:573`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.point.B1 | Default false | `mirrors/cm6/state/src/rangeset.ts:35` | `rangeset.rs:20` | none | none | implemented, untested |
| state.RangeValue.point.B2 | Points contribute `to - from` to the chunk/set `maxPoint` (−1 = no points) | `mirrors/cm6/state/src/rangeset.ts:537`, `:153`, `:505` | `rangeset.rs:397-399`, `:348`; map recomputes through the builder | none directly | none | implemented, untested |
| state.RangeValue.point.B3 | In spans/compare, a point shadows the ranges and points it covers; a non-empty point ending exactly at the previous point's end is skipped | `mirrors/cm6/state/src/rangeset.ts:826-841` | `rangeset.rs:778-792` | test-rangeset#39, #40, #42 | test_rangeset#039, #040, #042 | implemented+tested |
| state.RangeValue.point.B4 | `minPointSize`: layers/chunks whose `maxPoint` is below it are skipped, and only points of at least that size are yielded | `mirrors/cm6/state/src/rangeset.ts:611`, `:640`, `:686` | `rangeset.rs:475`, `:526`, `:556` | test-rangeset#41 | test_rangeset#041 | implemented+tested (spans only; compare has no minPointSize, see state.cm6.188) |
Proposed status: PORTED, DEVIATION because B1/B2 are untested and point filtering is missing from `compare`.

### state.cm6.176 `RangeValue.range` — cm6 `mirrors/cm6/state/src/rangeset.ts:31`
Helpers read: `Range.create` `mirrors/cm6/state/src/rangeset.ts:53`; overrides `mirrors/cm6/view/src/decoration.ts:297-300`, `:314-317`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeValue.range.B1 | `value.range(from, to)` creates `Range(from, to, this)` | `mirrors/cm6/state/src/rangeset.ts:31` | `Range::new(from, to, value)` `rangeset.rs:37` | all (`mk`) | all (`mk`) | implemented+tested (different API shape) |
| state.RangeValue.range.B2 | `to` defaults to `from` | `mirrors/cm6/state/src/rangeset.ts:31` | none (`to` is required) | test-rangeset#10, #12, #17 (`mk(1)`) | none | missing (R22g) |
| state.RangeValue.range.B3 | Overridable per value type, e.g. to validate (the view's mark/line decorations throw) | `mirrors/cm6/view/src/decoration.ts:297-300` | none (no method on the trait) | none | none | missing |
Proposed status: PORTED, DEVIATION because B2 and B3 are missing (`Range::new` is the only constructor).

### state.cm6.177 `Range` — cm6 `mirrors/cm6/state/src/rangeset.ts:43`
Helpers read: `Range.create` `:53`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.B1 | Private constructor: ranges are obtained only via `RangeValue.range` (or @internal `Range.create`) | `mirrors/cm6/state/src/rangeset.ts:44`, `:53` | `pub fn new` `rangeset.rs:37` and `pub` fields | all | all | differs: freely constructible |
| state.Range.B2 | Immutable (`readonly` fields) | `mirrors/cm6/state/src/rangeset.ts:46-50` | `pub` mutable fields `rangeset.rs:30-32` | none | none | differs (Rust ownership: only the owner's copy changes) |
Proposed status: PORTED, DEVIATION because B1 and B2 differ (encapsulation).

### state.cm6.178 `Range.from` — cm6 `mirrors/cm6/state/src/rangeset.ts:46`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.from.B1 | Start position, read-only, in the caller's document unit | `mirrors/cm6/state/src/rangeset.ts:46` | `rangeset.rs:30` (`pub usize`) | test-rangeset#19, #22, #44 | test_rangeset#010, #012, #017, #019 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (public field on an owned value; observable behavior is the same).

### state.cm6.179 `Range.to` — cm6 `mirrors/cm6/state/src/rangeset.ts:48`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.to.B1 | End position, read-only | `mirrors/cm6/state/src/rangeset.ts:48` | `rangeset.rs:31` | test-rangeset#10–#24 | test_rangeset#011, #013, #016 | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.180 `Range.value` — cm6 `mirrors/cm6/state/src/rangeset.ts:50`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Range.value.B1 | The associated value (shared object reference) | `mirrors/cm6/state/src/rangeset.ts:50` | `rangeset.rs:32` (owned clone; `iter` clones, `:208`) | test-rangeset#19, #22 | test_rangeset#019 | implemented+tested (clone instead of reference; identity is not observable in Ferrum) |
Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.181 `RangeSet` — cm6 `mirrors/cm6/state/src/rangeset.ts:226`
Helpers read: constructor/`create` `:227-243`, `chunkEnd` `:260`, `isEmpty` `:356`, `empty` `:467`, `:482`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.B1 | Ranges are stored in chunks of at most 250 (`C.ChunkSize`) with chunk-relative offsets | `mirrors/cm6/state/src/rangeset.ts:97`, `:529-531` | `rangeset.rs:10`, `:382-392` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSet.B2 | Overlapping ranges spill into `nextLayer` sets | `mirrors/cm6/state/src/rangeset.ts:519-520` | `rangeset.rs:363-366` | test-rangeset#01, #20 | test_rangeset#001 (`next_layer.is_some()`) | implemented+tested |
| state.RangeSet.B3 | Immutable; operations return new sets and share untouched chunks (by identity) | `mirrors/cm6/state/src/rangeset.ts:274`, `:287`, `:315-318` | `update`/`map` rebuild every chunk (`rangeset.rs:218-270`) | test-rangeset#03 | none | differs: R6 |
| state.RangeSet.B4 | Internals are `@internal`; construction only through `of`/builder/`empty` | `mirrors/cm6/state/src/rangeset.ts:227-243` | `pub` fields `rangeset.rs:93-96`, struct literal possible | test-rangeset#01, #03, #05, #07, #18 read `chunk`/`nextLayer` | test_rangeset#001, #007, #018 read `chunk`/`next_layer` | differs (encapsulation) |
| state.RangeSet.B5 | The empty set is a unique object whose `nextLayer` is itself | `mirrors/cm6/state/src/rangeset.ts:467`, `:482` | `next_layer: None` `rangeset.rs:111` | test-rangeset#09, #18 | test_rangeset#009, #018 (`is_empty`) | differs: there is no singleton |
Proposed status: PORTED, DEVIATION because of B3–B5.

### state.cm6.182 `RangeSet.size` — cm6 `mirrors/cm6/state/src/rangeset.ts:252`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.size.B1 | 0 for the empty set | `mirrors/cm6/state/src/rangeset.ts:253` | `rangeset.rs:122-131` (no chunks, no layer) | test-rangeset#09 (indirect) | test_rangeset#009, #014, #015 | implemented+tested |
| state.RangeSet.size.B2 | The sum of chunk value counts plus `nextLayer.size` | `mirrors/cm6/state/src/rangeset.ts:254-256` | `rangeset.rs:124-129` | test-rangeset#01, #03–#07 | test_rangeset#001, #003–#007 | implemented+tested |
Proposed status: PORTED (every behavior is implemented and tested; the oracle shows 0/9 003 mismatches).

### state.cm6.183 `RangeSet.update` — cm6 `mirrors/cm6/state/src/rangeset.ts:271`
Helpers read: `LayerCursor` `:584-668` (incl. `goto`, `next`, `nextChunk`), `RangeSetBuilder.addInner` `:524`, `addChunk` `:542`, `finishInner` `:560`, `RangeSet.of` `:448`, `cmpRange` `:58`, `chunkEnd` `:260`, `length` `:246`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.update.B1 | With no `add` and no `filter`, returns `this` unchanged (identity) | `mirrors/cm6/state/src/rangeset.ts:274` | none (always rebuilds, `rangeset.rs:219-235`) | none | none | differs: R7a |
| state.RangeSet.update.B2 | `sort: true` sorts a copy of `add` by `cmpRange`; otherwise `add` must already be sorted | `mirrors/cm6/state/src/rangeset.ts:275` | `sort` ignored; always sorts everything `rangeset.rs:235` | none | none | differs: R8 |
| state.RangeSet.update.B3 | On an empty set: `RangeSet.of(add)` (strict, throws on unsorted `add`) or `this` | `mirrors/cm6/state/src/rangeset.ts:276` | no special case | none | none | differs: R8d |
| state.RangeSet.update.B4 | Merges existing ranges and `add` in `(from, startSide)` order; on ties the added range goes first | `mirrors/cm6/state/src/rangeset.ts:280-283` | full re-sort, stable: existing ranges first on ties (`rangeset.rs:219`, `:232`, `:235`) | test-rangeset#03, #04 | test_rangeset#003, #004 | differs in layer placement and order on ties (R1 class; the oracle shows 1 236 order-only differences, 0 content) |
| state.RangeSet.update.B5 | An added range that does not fit in the current layer spills into `nextLayer.update({add: spill, …})` | `mirrors/cm6/state/src/rangeset.ts:283`, `:298-299` | `RangeSetBuilder::add_range` spill `rangeset.rs:363-366` | test-rangeset#04 | test_rangeset#004 | implemented+tested (different structure, same content) |
| state.RangeSet.update.B6 | Whole chunks before the next added range and outside the filter range are reused (`addChunk`, structural sharing) | `mirrors/cm6/state/src/rangeset.ts:284-288`, `:542-553` | none | test-rangeset#03 | none | missing: R6b |
| state.RangeSet.update.B7 | `filter` is called only for ranges touching `[filterFrom, filterTo]` (`filterFrom > to \|\| filterTo < from` skips); a range is kept when it returns true | `mirrors/cm6/state/src/rangeset.ts:290` | `rangeset.rs:221-229` | test-rangeset#05–#09 | test_rangeset#005–#009 (#008 weak) | implemented+tested (exact call list only in Replay #08 and R29) |
| state.RangeSet.update.B8 | Defaults: `filterFrom = 0`, `filterTo = this.length` | `mirrors/cm6/state/src/rangeset.ts:272` | `rangeset.rs:288-289` (`usize::MAX`) | test-rangeset#05 | test_rangeset#005 | implemented+tested (equivalent) |
| state.RangeSet.update.B9 | The filter is applied to every layer with the same `filterFrom`/`filterTo` | `mirrors/cm6/state/src/rangeset.ts:299` | `iter()` collects all layers `rangeset.rs:201-215` | test-rangeset#05 (set0 is single-layer) | none (multi-layer) | implemented, untested on multiple layers (oracle: 0 content mismatches) |
| state.RangeSet.update.B10 | Returns `RangeSet.empty` when everything is filtered out | `mirrors/cm6/state/src/rangeset.ts:298`, `:562` | returns an empty set (not a singleton) | test-rangeset#09 | test_rangeset#009 (size 0) | differs: no identity (R7b) |
| state.RangeSet.update.B11 | On a non-empty set, added ranges go through non-strict `addInner`: an unsorted `add` does not throw | `mirrors/cm6/state/src/rangeset.ts:283` | `rangeset.rs:235` (sorted anyway) | none | none | implemented (R8a, R8b match) |
| state.RangeSet.update.B12 | Cost: proportional to the touched chunks plus `add` (chunks reused) | `mirrors/cm6/state/src/rangeset.ts:284-288` | O(n log n) and n value clones on every call (`iter` + sort + rebuild) | none | none | differs (complexity) |
Proposed status: PORTED, DEVIATION because B1–B4, B6, B10 and B12 differ. Content parity is confirmed by the oracle (4 500 cases).

### state.cm6.184 `RangeSet.map` — cm6 `mirrors/cm6/state/src/rangeset.ts:303`
Helpers read: `Chunk.map` `:133-178`, `RangeSetBuilder.addRange` `:518`, `finishInner` `:560`, `ChangeDesc.touchesRange` `change.ts:147`, `mapPos` `change.ts:122`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.map.B1 | Empty changes or an empty set → returns `this` | `mirrors/cm6/state/src/rangeset.ts:304` | `rangeset.rs:240-242` (`self.clone()`, Arc-shared chunks) | none | none | implemented (R28), untested |
| state.RangeSet.map.B2 | A chunk not touched by the changes is kept by identity at `mapPos(start)` | `mirrors/cm6/state/src/rangeset.ts:315-318` | none (each range is re-mapped and rebuilt) | none | none | differs: R6a (content is the same) |
| state.RangeSet.map.B3 | A chunk entirely covered by one change (`touchesRange == "cover"`) is dropped with all its ranges | `mirrors/cm6/state/src/rangeset.ts:314-328` | none | none | none | missing: R4 |
| state.RangeSet.map.B4 | Empty range: start mapped with `(startSide, mapMode)`; `null` → dropped | `mirrors/cm6/state/src/rangeset.ts:138-141` | `rangeset.rs:246-247` | test-rangeset#15, #17 | test_rangeset#015, #017 | implemented+tested |
| state.RangeSet.map.B5 | Empty range with `startSide != endSide`: end mapped with `(endSide, Simple)`; dropped if `newTo < newFrom` | `mirrors/cm6/state/src/rangeset.ts:142-145` | `rangeset.rs:249-256` (uses `map_mode`) | none | none | differs: R32 |
| state.RangeSet.map.B6 | Non-empty range: `from`/`to` mapped `Simple` with the sides; dropped if `newFrom > newTo`, or if it collapsed with `startSide > 0 && endSide <= 0` | `mirrors/cm6/state/src/rangeset.ts:146-150` | `rangeset.rs:258-265` | test-rangeset#11, #13, #14, #16 | test_rangeset#011, #013, #014, #016 | implemented+tested |
| state.RangeSet.map.B7 | Any result with `(newTo - newFrom \|\| endSide - startSide) < 0` is dropped | `mirrors/cm6/state/src/rangeset.ts:151` | none | none | none | missing: R5 |
| state.RangeSet.map.B8 | Ranges that move out of order: points are reinserted at the right place in the chunk, others spill into a new layer | `mirrors/cm6/state/src/rangeset.ts:155-175` | global re-sort + builder (`rangeset.rs:269`) | test-rangeset#19–#24 | test_rangeset#019 (names), #020–#024 (size only) | implemented+tested (different structure; Replay #19–#24 pass) |
| state.RangeSet.map.B9 | Spilled ranges are built on top of the mapped `nextLayer` | `mirrors/cm6/state/src/rangeset.ts:330-331` | single rebuild | test-rangeset#20 | test_rangeset#020 (weak) | implemented (different structure) |
| state.RangeSet.map.B10 | `maxPoint` recomputed from mapped points | `mirrors/cm6/state/src/rangeset.ts:153`, `:316`, `:324` | through the builder `rangeset.rs:397-399` | none | none | implemented, untested |
| state.RangeSet.map.B11 | Result is `next` (possibly `RangeSet.empty`) when no chunk survives | `mirrors/cm6/state/src/rangeset.ts:332` | returns an empty set | test-rangeset#18 | test_rangeset#018 (`is_empty`) | implemented+tested (no singleton identity) |
| state.RangeSet.map.B12 | Sides are used as `assoc` at full precision | `mirrors/cm6/state/src/rangeset.ts:139`, `:143`, `:147-148` | `as i8` `rangeset.rs:247`, `:250`, `:259`, `:260` | none | none | differs: R3, R31 |
| state.RangeSet.map.B13 | Never fails (mapping cannot produce misordered layers) | `mirrors/cm6/state/src/rangeset.ts:303-333` | `unwrap_or_else(|_| Self::empty())` `rangeset.rs:269`, unreachable after sorting | none | none | implemented (the silent fallback would hide a bug) |
| state.RangeSet.map.B14 | Cost: only touched chunks are re-mapped | `mirrors/cm6/state/src/rangeset.ts:314-318` | O(n log n), clones every value | none | none | differs (complexity) |
Proposed status: PORTED, DEVIATION because B2, B3, B5, B7, B12 and B14 differ.

### state.cm6.185 `RangeSet.between` — cm6 `mirrors/cm6/state/src/rangeset.ts:339`
Helpers read: `Chunk.between` `:128`, `Chunk.findIndex` `:116`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.between.B1 | Empty set → no calls | `mirrors/cm6/state/src/rangeset.ts:340` | loop over zero chunks `rangeset.rs:173` | none | none | implemented, untested |
| state.RangeSet.between.B2 | Only chunks with `to >= start && from <= start + length` are visited | `mirrors/cm6/state/src/rangeset.ts:343` | `rangeset.rs:176` | test-rangeset#46 | test_rangeset#046 | implemented+tested |
| state.RangeSet.between.B3 | Within a chunk, reports ranges with `to >= from` and `from <= to` (inclusive touch), using sentinel sides `±C.Far` | `mirrors/cm6/state/src/rangeset.ts:129` | `rangeset.rs:79-80` with `±1_000_000` and `saturating_sub` | test-rangeset#46, #47 | test_rangeset#046 (weak), #047 | differs: R2 (sides beyond ±1e6) |
| state.RangeSet.between.B4 | A callback returning `false` stops the whole iteration, including later layers | `mirrors/cm6/state/src/rangeset.ts:130`, `:344` | `rangeset.rs:82-84`, `:177-178`, `:183-184` | none | none | implemented (R27), untested |
| state.RangeSet.between.B5 | No order guarantee; actual order is layer by layer, chunk order | `mirrors/cm6/state/src/rangeset.ts:335-338` | same traversal | none | none | implemented (R27b: both `A,C,B`) |
| state.RangeSet.between.B6 | Zero-length set/range at 0 is reported for `between(0,0)` | `mirrors/cm6/state/src/rangeset.ts:129` | same | test-rangeset#47 | test_rangeset#047 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B3 (R2).

### state.cm6.186 `RangeSet.iter` — cm6 `mirrors/cm6/state/src/rangeset.ts:351`
Helpers read: `HeapCursor.from` `:678`, `goto` `:695`, `next` `:708`, `heapBubble` `:725`, `LayerCursor.goto` `:600`, `compare` `:664`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet.iter.B1 | Returns a lazy `RangeCursor` (no materialization) | `mirrors/cm6/state/src/rangeset.ts:351-353` | returns a `Vec<Range<T>>` with cloned values `rangeset.rs:191-199` | test-rangeset#44 | test_rangeset#044 | differs (API shape, allocation O(n)) |
| state.RangeSet.iter.B2 | `iter(from)` starts at the first range ending at or after `from` (default 0) | `mirrors/cm6/state/src/rangeset.ts:351`, `:600-602`, `:616` | none (no parameter) | test-rangeset#45 | test_rangeset#045 (emulated by filtering) | missing: R21 |
| state.RangeSet.iter.B3 | Order: `from`, `startSide`, `rank`, `to`, `endSide` | `mirrors/cm6/state/src/rangeset.ts:664-667` | sort by `from`, `startSide` only (stable by layer) `rangeset.rs:194-197` | test-rangeset#19, #22, #44 | test_rangeset#019, #022 (size only), #044 (count only) | differs: R1 |
| state.RangeSet.iter.B4 | Ties with all keys equal follow the heap's swap-on-equal behavior | `mirrors/cm6/state/src/rangeset.ts:730`, `:734` | n/a for `iter` (sort) | none | none | differs: R18 |
| state.RangeSet.iter.B5 | Iterates every layer | `mirrors/cm6/state/src/rangeset.ts:685` | `collect_ranges` recursion `rangeset.rs:212-214` | test-rangeset#44 | test_rangeset#044 | implemented+tested |
Proposed status: PORTED, DEVIATION because of B1–B4.

### state.cm6.187 `RangeSet^iter` — cm6 `mirrors/cm6/state/src/rangeset.ts:360`
Helpers read: `HeapCursor.from` `:678-691`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^iter.B1 | Iterates several sets at once in heap order, starting at `from` | `mirrors/cm6/state/src/rangeset.ts:360-362` | none (private `HeapCursor` `rangeset.rs:542`, not exposed) | none | none | missing |
| state.RangeSet^iter.B2 | `rank` = index of the set in `sets` | `mirrors/cm6/state/src/rangeset.ts:687` | none (private) | none | none | missing |
| state.RangeSet^iter.B3 | All layers of one set share its rank | `mirrors/cm6/state/src/rangeset.ts:685-687` | none | none | none | missing |
| state.RangeSet^iter.B4 | Returns the `LayerCursor` directly when only one layer exists | `mirrors/cm6/state/src/rangeset.ts:690` | none | none | none | missing |
Proposed status: MISSING (searched: see `docs/parity/state/raw/C6-api.md` state.cm6.187). R22b.

### state.cm6.188 `RangeSet^compare` — cm6 `mirrors/cm6/state/src/rangeset.ts:366`
Helpers read: `findSharedChunks` `:570`, `SpanCursor` `:741-866`, `compare` `:868-896`, `sameValues` `:898`, `cmpVal` `:38`, `ChangeDesc.iterGaps` `change.ts:56`, `touchesRange` `change.ts:147`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^compare.B1 | `textDiff` is a `ChangeDesc` | `mirrors/cm6/state/src/rangeset.ts:370` | takes `&ChangeSet`, uses `.desc()` `rangeset.rs:899`, `:908` | all compare tests | test_rangeset#025–#034 | differs (callers holding only a `ChangeDesc` cannot call it) |
| state.RangeSet^compare.B2 | Sets are filtered: keep `maxPoint > 0 \|\| !isEmpty && maxPoint >= minPointSize` | `mirrors/cm6/state/src/rangeset.ts:376-377` | none (all sets used; empty layers skipped in `HeapCursor::from_sets` `rangeset.rs:556`) | test-rangeset#30 | none | differs only with minPointSize (B9) |
| state.RangeSet^compare.B3 | Chunks without non-empty points that are identical (same object) at the mapped position and untouched by the changes are skipped on both sides | `mirrors/cm6/state/src/rangeset.ts:378`, `:570-582`, `:609`, `:648-650` | none | test-rangeset#29 | none | missing: R13 (changes the reported active arrays and split points) |
| state.RangeSet^compare.B4 | Only unchanged gaps (`iterGaps`) are compared; changed regions are never reported | `mirrors/cm6/state/src/rangeset.ts:383` | gaps with `length > 0` `rangeset.rs:908-914`, plus an extra tail comparison `:915-922` | test-rangeset#28, #33 | none (test_rangeset#028, #033 assert nothing) | differs: R15 (tail region compared) |
| state.RangeSet^compare.B5 | Empty document (`textDiff.empty && length == 0`): compares at position 0 | `mirrors/cm6/state/src/rangeset.ts:384` | none | none | none | missing: R14 |
| state.RangeSet^compare.B6 | Each gap starts with `goto(start)` side `-C.Far` | `mirrors/cm6/state/src/rangeset.ts:872-873`, `:766` | `goto(start, -1_000_000)` `rangeset.rs:855-856` | none | none | differs: R16 |
| state.RangeSet^compare.B7 | `compareRange(pos, clipEnd, a.active, b.active)` when the active sets differ (`sameValues`, order-sensitive) and the piece is non-empty | `mirrors/cm6/state/src/rangeset.ts:888` | `rangeset.rs:876-878` | test-rangeset#25–#27, #33 | none | implemented, untested in Ferrum (Replay #25–#27, #33 pass) |
| state.RangeSet^compare.B8 | `comparePoint` when the points differ, or when the active ranges covering the point (`activeForPoint`) differ | `mirrors/cm6/state/src/rangeset.ts:880-885` | `rangeset.rs:867-874` | test-rangeset#30–#32, #34 | none | implemented, untested in Ferrum (Replay passes) |
| state.RangeSet^compare.B9 | `minPointSize` (default −1): ignore non-point ranges and points smaller than it | `mirrors/cm6/state/src/rangeset.ts:374`, `:380-381` | none (`min_point = -1` hard-coded `rangeset.rs:902`) | none | none | missing: R23 |
| state.RangeSet^compare.B10 | `boundChange(pos)` when the boundary between ranges changes (`dEnd` or a different `openEnd`), not for points, only if the comparator defines it | `mirrors/cm6/state/src/rangeset.ts:876-889` | none | none | none | missing: R17 |
| state.RangeSet^compare.B11 | Reported positions are in new-document coordinates, clipped to the gap end | `mirrors/cm6/state/src/rangeset.ts:874-879` | `rangeset.rs:857-865` | test-rangeset#28 | none | implemented, untested in Ferrum (Replay #28 passes) |
| state.RangeSet^compare.B12 | Steps both cursors in lockstep (`diff <= 0` → a.next, `diff >= 0` → b.next), ending after the gap end | `mirrors/cm6/state/src/rangeset.ts:891-894` | `rangeset.rs:881-890` | test-rangeset#33 | none | implemented, untested |
Proposed status: PORTED, DEVIATION because of B1, B3–B6, B9 and B10; Ferrum tests assert nothing. On identical new sets the oracle gives 994/4 500 raw call-list mismatches (`compareN`).

### state.cm6.189 `RangeSet^eq` — cm6 `mirrors/cm6/state/src/rangeset.ts:389`
Helpers read: `findSharedChunks` `:570`, `SpanCursor` `:741`, `sameValues` `:898`, `cmpVal` `:38`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^eq.B1 | `to` defaults to `C.Far - 1`, `from` to 0 | `mirrors/cm6/state/src/rangeset.ts:391-393` | none | none | none | missing |
| state.RangeSet^eq.B2 | Sets present (by identity) in both lists are ignored; empty sets ignored | `mirrors/cm6/state/src/rangeset.ts:394-395` | none | none | none | missing |
| state.RangeSet^eq.B3 | A different number of remaining sets → false; none remaining → true | `mirrors/cm6/state/src/rangeset.ts:396-397` | none | none | none | missing |
| state.RangeSet^eq.B4 | Walks both sides with `SpanCursor(…, minPoint 0)`: only point ranges are considered | `mirrors/cm6/state/src/rangeset.ts:399` | none | none | none | missing (R30: marks differing only in value compare equal) |
| state.RangeSet^eq.B5 | Unequal when span ends, active sets or points differ | `mirrors/cm6/state/src/rangeset.ts:401-404` | none | none | none | missing |
| state.RangeSet^eq.B6 | Returns true once past `to` | `mirrors/cm6/state/src/rangeset.ts:405` | none | none | none | missing |
| state.RangeSet^eq.B7 | Shared chunks are skipped | `mirrors/cm6/state/src/rangeset.ts:398` | none | none | none | missing |
Proposed status: MISSING (searched: see `docs/parity/state/raw/C6-api.md` state.cm6.189).

### state.cm6.190 `RangeSet^spans` — cm6 `mirrors/cm6/state/src/rangeset.ts:415`
Helpers read: `SpanCursor` `:741-866` (`goto`, `forward`, `next`, `addActive`, `removeActive`, `activeForPoint`, `openEnd`), `HeapCursor` `:670-723`, `LayerCursor` `:584-668`, `insert`/`remove`/`findMinIndex` `:904-921`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^spans.B1 | The whole `[from, to]` is covered by `span`/`point` calls in order; empty non-point pieces are skipped | `mirrors/cm6/state/src/rangeset.ts:424-440` | `rangeset.rs:937-960` | test-rangeset#35, #36 | test_rangeset#035, #036 | implemented+tested |
| state.RangeSet^spans.B2 | `active` holds covering non-point ranges ordered by rank, then by `to` (bigger outside) | `mirrors/cm6/state/src/rangeset.ts:790-799` | `rangeset.rs:719-740` | test-rangeset#37, #38, #43 | test_rangeset#037, #038, #043 | implemented+tested |
| state.RangeSet^spans.B3 | Ties in the heap (all keys equal) are resolved by swap-on-equal | `mirrors/cm6/state/src/rangeset.ts:730`, `:734` | `rangeset.rs:623`, `:626` (no swap on equal) | none | none | differs: R18 (active order `a,b` vs `b,a`) |
| state.RangeSet^spans.B4 | `openStart` of a span = the number of trailing `active` entries that started before the span start | `mirrors/cm6/state/src/rangeset.ts:844-847`, `:423`, `:435` | `rangeset.rs:796-803` counts every entry started before | none | none | differs: R11 |
| state.RangeSet^spans.B5 | `openEnd(to)` = the number of trailing `active` entries ending after `to` | `mirrors/cm6/state/src/rangeset.ts:861-865` | `rangeset.rs:824-832` counts every entry | none | none | differs: R12 |
| state.RangeSet^spans.B6 | Point `openCount`: `pointFrom < from` → `active.length + 1`; `startSide < 0` → `active.length`; else `min(active.length, openRanges)` | `mirrors/cm6/state/src/rangeset.ts:428-430` | `rangeset.rs:941-947` | none | none | implemented (R26 matches), untested |
| state.RangeSet^spans.B7 | Point `active` = ranges with `rank >= pointRank` that cover the point end (`activeForPoint`) | `mirrors/cm6/state/src/rangeset.ts:850-859` | `rangeset.rs:806-822` | test-rangeset#40 | test_rangeset#040 | implemented+tested (oracle case 277 differs through B5/B11 effects) |
| state.RangeSet^spans.B8 | Point `index` = rank of the point's set | `mirrors/cm6/state/src/rangeset.ts:431` | `rangeset.rs:948` | none | none | implemented, untested |
| state.RangeSet^spans.B9 | A non-empty point ending at the previous point's end is omitted; zero-length points at the same place are all reported | `mirrors/cm6/state/src/rangeset.ts:829-831` | `rangeset.rs:781-782` | test-rangeset#39, #42 | test_rangeset#039, #042 | implemented+tested |
| state.RangeSet^spans.B10 | `minPointSize >= 0` ignores non-point ranges and smaller points | `mirrors/cm6/state/src/rangeset.ts:420`, `:640` | `rangeset.rs:526`, `:931` | test-rangeset#41 | test_rangeset#041 | implemented+tested (R24 matches) |
| state.RangeSet^spans.B11 | After a point, cursors are forwarded past it (`forward`), which may jump a layer to a later chunk | `mirrors/cm6/state/src/rangeset.ts:839`, `:622-625`, `:606-620` | `rangeset.rs:790`, `:493-498`, `:470-491` (`saturating_sub`) | test-rangeset#40 | test_rangeset#040 | differs: R19 (a point at a later chunk start is lost) |
| state.RangeSet^spans.B12 | Return value: open count at the end, +1 if a point extends past `to` | `mirrors/cm6/state/src/rangeset.ts:437` | `rangeset.rs:954-957` | none | none | implemented (values differ through B5: R12b), untested |
Proposed status: PORTED, DEVIATION because of B3, B4, B5 and B11; open counts are untested on both sides.

### state.cm6.191 `RangeSet^of` — cm6 `mirrors/cm6/state/src/rangeset.ts:448`
Helpers read: `lazySort` `:472`, `cmpRange` `:58`, `RangeSetBuilder.add` `:515`, `finish` `:557`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^of.B1 | Accepts a single `Range` or an array | `mirrors/cm6/state/src/rangeset.ts:450` | `Vec` only `rangeset.rs:148` | none | none | differs: R22h |
| state.RangeSet^of.B2 | `sort = true` → `lazySort` (sorts a copy only if out of order; stable) | `mirrors/cm6/state/src/rangeset.ts:450`, `:472-479` | `of_sorted(…, true)` always sorts (stable) `rangeset.rs:154-159` | test-rangeset#01, #44 (pre-sorted with `cmp`) | test_rangeset#001, #015, #016, #035–#044 | implemented+tested (same result) |
| state.RangeSet^of.B3 | Strict add: misordered input throws "Ranges must be added sorted by `from` position and `startSide`" | `mirrors/cm6/state/src/rangeset.ts:451`, `:526-527` | `add_strict` → `Err(…)` `rangeset.rs:162`, `:374-376` | test-rangeset#02 | test_rangeset#002 (first assertion only) | differs: `Result` instead of a throw; misses R9 |
| state.RangeSet^of.B4 | Overlapping ranges are layered | `mirrors/cm6/state/src/rangeset.ts:519-520` | `rangeset.rs:363-366` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSet^of.B5 | Empty input → `RangeSet.empty` | `mirrors/cm6/state/src/rangeset.ts:562` | empty set (not a singleton) | none | none | implemented (no identity), untested |
Proposed status: PORTED, DEVIATION because of B1, B3 and B5.

### state.cm6.192 `RangeSet^join` — cm6 `mirrors/cm6/state/src/rangeset.ts:456`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^join.B1 | Empty array → `RangeSet.empty` | `mirrors/cm6/state/src/rangeset.ts:457` | none | none | none | missing |
| state.RangeSet^join.B2 | Layers of `sets[0..n-2]` are stacked on top of the last set, in order; `maxPoint` is the max | `mirrors/cm6/state/src/rangeset.ts:458-462` | none | none | none | missing |
| state.RangeSet^join.B3 | No chunk is copied (layers reused by reference) | `mirrors/cm6/state/src/rangeset.ts:461` | none | none | none | missing |
Proposed status: MISSING (R22a).

### state.cm6.193 `RangeSet^empty` — cm6 `mirrors/cm6/state/src/rangeset.ts:467`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSet^empty.B1 | A singleton with no chunks, `maxPoint = -1`, `nextLayer = itself` | `mirrors/cm6/state/src/rangeset.ts:467`, `:482` | `RangeSet::empty()` `rangeset.rs:107-114` (fresh value, `next_layer: None`) | test-rangeset#09, #18 | test_rangeset#009, #018 | differs: no singleton |
| state.RangeSet^empty.B2 | Operations return this exact object when the result is empty, so callers test `set == RangeSet.empty` | `mirrors/cm6/state/src/rangeset.ts:298`, `:332`, `:562` | `is_empty()` `rangeset.rs:117` | test-rangeset#09, #18 | test_rangeset#018 | differs (identity replaced by `is_empty`; R7b) |
Proposed status: PORTED, DEVIATION.

### state.cm6.194 `RangeCursor` — cm6 `mirrors/cm6/state/src/rangeset.ts:185`
Helpers read: `HeapCursor` `:670`, `LayerCursor` `:584`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.B1 | Starts pointing at the first range (read before calling `next`) | `mirrors/cm6/state/src/rangeset.ts:181-184` | none (`Vec`) | test-rangeset#19, #44 | none | missing |
| state.RangeCursor.B2 | At the end `value` is `null` and `from = to = C.Far` | `mirrors/cm6/state/src/rangeset.ts:630-631`, `:710-712` | none | test-rangeset#19 | none | missing |
Proposed status: MISSING.

### state.cm6.195 `RangeCursor.next` — cm6 `mirrors/cm6/state/src/rangeset.ts:187`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.next.B1 | Advances lazily to the next range in heap order | `mirrors/cm6/state/src/rangeset.ts:708-722`, `:627-643` | none (public); private `HeapCursor::next` `rangeset.rs:600` | test-rangeset#19, #44, #45 | none | missing |
Proposed status: MISSING.

### state.cm6.196 `RangeCursor.goto` — cm6 `mirrors/cm6/state/src/rangeset.ts:189`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.goto.B1 | Re-positions at the first range ending at or after `pos` (side `-C.Far`) | `mirrors/cm6/state/src/rangeset.ts:695-700`, `:600-604` | none (public); private `rangeset.rs:575` | none | none | missing (R22c) |
| state.RangeCursor.goto.B2 | Can move backwards (resets chunk/range index) | `mirrors/cm6/state/src/rangeset.ts:601` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.197 `RangeCursor.value` — cm6 `mirrors/cm6/state/src/rangeset.ts:192`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.value.B1 | The current value, `null` at the end | `mirrors/cm6/state/src/rangeset.ts:192`, `:631`, `:711` | none (each `Range.value` in the `Vec`) | test-rangeset#19, #22 | none | missing |
Proposed status: MISSING.

### state.cm6.198 `RangeCursor.from` — cm6 `mirrors/cm6/state/src/rangeset.ts:194`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.from.B1 | The current start; `C.Far` at the end | `mirrors/cm6/state/src/rangeset.ts:194`, `:630` | none | test-rangeset#19, #22, #44, #45 | none | missing |
Proposed status: MISSING.

### state.cm6.199 `RangeCursor.to` — cm6 `mirrors/cm6/state/src/rangeset.ts:196`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.to.B1 | The current end; `C.Far` at the end | `mirrors/cm6/state/src/rangeset.ts:196`, `:630` | none | test-rangeset#22, #45 | none | missing |
Proposed status: MISSING.

### state.cm6.200 `RangeCursor.rank` — cm6 `mirrors/cm6/state/src/rangeset.ts:199`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeCursor.rank.B1 | Index of the origin set; −1 at the end of a HeapCursor | `mirrors/cm6/state/src/rangeset.ts:199`, `:712`, `:718` | none (private field `rangeset.rs:547`, `usize::MAX` at the end `:605`) | none | none | missing |
Proposed status: MISSING.

### state.cm6.201 `RangeSetBuilder` — cm6 `mirrors/cm6/state/src/rangeset.ts:487`
Helpers read: `finishChunk` `:501`, `addRange` `:518`, `addInner` `:524`, `addChunk` `:542`, `finishInner` `:560`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.B1 | Builds a set without an intermediate `Range` array | `mirrors/cm6/state/src/rangeset.ts:484-486` | `rangeset.rs:295-415` | indirect (all) | indirect (all via `of`) | implemented+tested |
| state.RangeSetBuilder.B2 | Chunks closed at 250 ranges | `mirrors/cm6/state/src/rangeset.ts:529` | `rangeset.rs:382-384` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.B3 | Layering through a nested builder | `mirrors/cm6/state/src/rangeset.ts:519-520`, `:564` | `rangeset.rs:363-366`, `:407` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.B4 | `addChunk`/`finishInner` let `update`/`map` reuse chunks and stack on an existing next layer | `mirrors/cm6/state/src/rangeset.ts:542-567` | none | test-rangeset#03 | none | missing (internal; it is what makes R6 possible) |
Proposed status: PORTED, DEVIATION because of B4 and `add` (state.cm6.203).

### state.cm6.202 `RangeSetBuilder.constructor` — cm6 `mirrors/cm6/state/src/rangeset.ts:511`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.constructor.B1 | Creates an empty builder (`lastFrom = lastTo = -C.Far`, `maxPoint = -1`) | `mirrors/cm6/state/src/rangeset.ts:488-511` | `rangeset.rs:319-335` | indirect | indirect (`of_sorted` `rangeset.rs:160`) | implemented+tested |
Proposed status: PORTED, RUST-IDIOMATIC (`new()` + `Default`).

### state.cm6.203 `RangeSetBuilder.add` — cm6 `mirrors/cm6/state/src/rangeset.ts:515`
Helpers read: `addRange` `:518`, `addInner` `:524-539`, `finishChunk` `:501`.
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.add.B1 | Strict: throws when the range is before the previous one by `(from, startSide)` and overlaps it (`diff <= 0`) | `mirrors/cm6/state/src/rangeset.ts:515`, `:526-527` | `add` is non-strict `rangeset.rs:353-355`; the strict variant is `add_strict` `:358` | test-rangeset#02 | test_rangeset#002 (via `of`) | differs: R10 |
| state.RangeSetBuilder.add.B2 | The strict check also fires when `from == lastTo && startSide == last.endSide` and `(from, startSide)` precede the previous range's | `mirrors/cm6/state/src/rangeset.ts:525-526` | `diff` never 0 (`rangeset.rs:371`) | none | none | differs: R9 |
| state.RangeSetBuilder.add.B3 | A range overlapping the previous one (`diff < 0`) goes to the next-layer builder | `mirrors/cm6/state/src/rangeset.ts:528`, `:519-520` | `rangeset.rs:378-380`, `:363-366` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.add.B4 | Positions stored relative to the chunk start | `mirrors/cm6/state/src/rangeset.ts:530-532` | `rangeset.rs:386-392` | test-rangeset#01 (`checkSet`) | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.add.B5 | Points update `maxPoint` | `mirrors/cm6/state/src/rangeset.ts:537` | `rangeset.rs:397-399` | none | none | implemented, untested |
| state.RangeSetBuilder.add.B6 | `add` after `finish` throws (`from = null`) | `mirrors/cm6/state/src/rangeset.ts:565` | `finish(self)` consumes the builder (compile time) | none | none | implemented (RUST-IDIOMATIC) |
Proposed status: PORTED, DEVIATION because of B1 and B2.

### state.cm6.204 `RangeSetBuilder.finish` — cm6 `mirrors/cm6/state/src/rangeset.ts:557`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetBuilder.finish.B1 | Closes the pending chunk and returns the set; with no ranges returns `RangeSet.empty` | `mirrors/cm6/state/src/rangeset.ts:557-562` | `rangeset.rs:405-414` (an empty set) | indirect | indirect | implemented+tested (no singleton) |
| state.RangeSetBuilder.finish.B2 | Next-layer builders are finished recursively | `mirrors/cm6/state/src/rangeset.ts:563-564` | `rangeset.rs:407` | test-rangeset#01 | test_rangeset#001 | implemented+tested |
| state.RangeSetBuilder.finish.B3 | The builder is unusable afterwards | `mirrors/cm6/state/src/rangeset.ts:565` | consumes `self` | none | none | implemented (compile time) |
Proposed status: PORTED, RUST-IDIOMATIC.

### state.cm6.205 `RangeComparator` — cm6 `mirrors/cm6/state/src/rangeset.ts:63`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.B1 | Interface with `compareRange`, `comparePoint` and an optional `boundChange` | `mirrors/cm6/state/src/rangeset.ts:63-75` | trait with two required methods `rangeset.rs:418-421` | test-rangeset#25–#34 | test_rangeset#025–#034 | differs: no `boundChange` |
| state.RangeComparator.B2 | Notified with new-document positions | `mirrors/cm6/state/src/rangeset.ts:64-66` | same | test-rangeset#28 | none | implemented, untested |
Proposed status: PORTED, DEVIATION.

### state.cm6.206 `RangeComparator.compareRange` — cm6 `mirrors/cm6/state/src/rangeset.ts:67`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.compareRange.B1 | Called for each non-empty piece where the active sets differ | `mirrors/cm6/state/src/rangeset.ts:888` | `rangeset.rs:876-878` | test-rangeset#25–#27, #33 | none (no assertions) | implemented, untested |
| state.RangeComparator.compareRange.B2 | Ranges inside shared chunks do not appear in `activeA`/`activeB` | `mirrors/cm6/state/src/rangeset.ts:378` | none | test-rangeset#29 | none | differs: R13 |
| state.RangeComparator.compareRange.B3 | Never called inside changed regions | `mirrors/cm6/state/src/rangeset.ts:383` | tail region compared `rangeset.rs:915-922` | test-rangeset#28 | none | differs: R15 |
Proposed status: PORTED, DEVIATION.

### state.cm6.207 `RangeComparator.comparePoint` — cm6 `mirrors/cm6/state/src/rangeset.ts:69`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.comparePoint.B1 | Called for changed, inserted or deleted points (`null` on the absent side) | `mirrors/cm6/state/src/rangeset.ts:880-885` | `rangeset.rs:867-874` (`Option<&T>`) | test-rangeset#31, #32 | none | implemented, untested |
| state.RangeComparator.comparePoint.B2 | Called when the same point has a different covering range set ("different cover") | `mirrors/cm6/state/src/rangeset.ts:883` | `rangeset.rs:871` | test-rangeset#34 | none | implemented, untested (Replay #34 passes) |
| state.RangeComparator.comparePoint.B3 | Points at a gap start with sides below `-1e6`, and points in an empty document, are reported | `mirrors/cm6/state/src/rangeset.ts:384`, `:872` | none | none | none | differs: R14, R16 |
Proposed status: PORTED, DEVIATION.

### state.cm6.208 `RangeComparator.boundChange` — cm6 `mirrors/cm6/state/src/rangeset.ts:74`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeComparator.boundChange.B1 | Optional; if absent, no bound tracking happens | `mirrors/cm6/state/src/rangeset.ts:876` | none | none | none | missing |
| state.RangeComparator.boundChange.B2 | Called at the position where ranges used to be split (or are now split) although the covered values are equal | `mirrors/cm6/state/src/rangeset.ts:887`, `:889` | none | none | none | missing (R17) |
| state.RangeComparator.boundChange.B3 | Not reported for point segments, nor at the gap end | `mirrors/cm6/state/src/rangeset.ts:885`, `:889` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.209 `SpanIterator` — cm6 `mirrors/cm6/state/src/rangeset.ts:80`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SpanIterator.B1 | The iterated range is fully covered by `span`/`point` calls | `mirrors/cm6/state/src/rangeset.ts:77-79` | `rangeset.rs:424-427` | test-rangeset#35, #41 | test_rangeset#035, #041 | implemented+tested |
Proposed status: PORTED, DEVIATION through its members (rows 210, 211).

### state.cm6.210 `SpanIterator.span` — cm6 `mirrors/cm6/state/src/rangeset.ts:85`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SpanIterator.span.B1 | Receives `(from, to, active)` for every non-point piece, `active` possibly empty | `mirrors/cm6/state/src/rangeset.ts:434` | `rangeset.rs:951` | test-rangeset#35–#38, #43 | test_rangeset#035–#038, #043 | implemented+tested |
| state.SpanIterator.span.B2 | `openStart` = the number of active ranges continued from before the span | `mirrors/cm6/state/src/rangeset.ts:423`, `:435` | `rangeset.rs:936`, `:952` | none | none | differs: R11, R12 |
| state.SpanIterator.span.B3 | Not called for empty pieces | `mirrors/cm6/state/src/rangeset.ts:433` | `rangeset.rs:950` | test-rangeset#39 | test_rangeset#039 | implemented+tested |
Proposed status: PORTED, DEVIATION.

### state.cm6.211 `SpanIterator.point` — cm6 `mirrors/cm6/state/src/rangeset.ts:92`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.SpanIterator.point.B1 | Receives `(from, to, value)` clipped to the iterated range (from = current position) | `mirrors/cm6/state/src/rangeset.ts:431` | `rangeset.rs:948` | test-rangeset#39, #40 | test_rangeset#039, #040 | implemented+tested |
| state.SpanIterator.point.B2 | `active` = higher-or-equal-precedence ranges covering the point | `mirrors/cm6/state/src/rangeset.ts:427`, `:850-859` | `rangeset.rs:940`, `:806-822` | none (names not checked for points) | none | implemented, untested |
| state.SpanIterator.point.B3 | `openStart` rule (`active.length + 1` when the point started before `from`) | `mirrors/cm6/state/src/rangeset.ts:428-430` | `rangeset.rs:941-947` | none | none | implemented (R26), untested |
| state.SpanIterator.point.B4 | Every point not shadowed is reported, including one at the start of a later chunk after a forward jump | `mirrors/cm6/state/src/rangeset.ts:839`, `:616` | `rangeset.rs:790`, `:484` | none | none | differs: R19 |
Proposed status: PORTED, DEVIATION.

### state.cm6.260 `RangeSet.update spec.add` — cm6 `mirrors/cm6/state/src/rangeset.ts:207`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.add.B1 | Default `[]` | `mirrors/cm6/state/src/rangeset.ts:272` | `rangeset.rs:285` | test-rangeset#05 | test_rangeset#005 | implemented+tested |
| state.RangeSetUpdate.add.B2 | Must be sorted by `(from, startSide)` unless `sort` is true | `mirrors/cm6/state/src/rangeset.ts:203-207` | always sorted with the existing ranges | none | none | differs: R8d |
| state.RangeSetUpdate.add.B3 | Added ranges become part of the set | `mirrors/cm6/state/src/rangeset.ts:280-283` | `rangeset.rs:231-233` | test-rangeset#03, #04 | test_rangeset#003, #004 | implemented+tested |
Proposed status: PORTED, DEVIATION.

### state.cm6.261 `RangeSet.update spec.sort` — cm6 `mirrors/cm6/state/src/rangeset.ts:210`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.sort.B1 | Default false | `mirrors/cm6/state/src/rangeset.ts:272` | `rangeset.rs:286` | none | none | implemented (field), untested |
| state.RangeSetUpdate.sort.B2 | true → `add` is sorted with `cmpRange` before merging | `mirrors/cm6/state/src/rangeset.ts:275` | field never read in `update` (`rangeset.rs:218-236`) | none | none | differs: ignored (R8) |
Proposed status: PORTED, DEVIATION.

### state.cm6.262 `RangeSet.update spec.filter` — cm6 `mirrors/cm6/state/src/rangeset.ts:213`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.filter.B1 | Keeps the ranges for which it returns true | `mirrors/cm6/state/src/rangeset.ts:290` | `rangeset.rs:222-228` | test-rangeset#05–#07, #09 | test_rangeset#005–#007, #009 | implemented+tested |
| state.RangeSetUpdate.filter.B2 | Called exactly once per range touching the filter range, in position order | `mirrors/cm6/state/src/rangeset.ts:290` | `rangeset.rs:222` (sorted `Vec`) | test-rangeset#08 | test_rangeset#008 (count between 200 and 400) | implemented, untested exactly (R29, Replay #08 match) |
| state.RangeSetUpdate.filter.B3 | Not called for ranges inside reused chunks outside the filter range | `mirrors/cm6/state/src/rangeset.ts:286` | the equivalent condition per range `rangeset.rs:223` | test-rangeset#08 | test_rangeset#008 (weak) | implemented |
Proposed status: PORTED, DEVIATION because B2 is untested exactly in Ferrum.

### state.cm6.263 `RangeSet.update spec.filterFrom` — cm6 `mirrors/cm6/state/src/rangeset.ts:217`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.filterFrom.B1 | Default 0 | `mirrors/cm6/state/src/rangeset.ts:272` | `rangeset.rs:288` | test-rangeset#05 | test_rangeset#005 | implemented+tested |
| state.RangeSetUpdate.filterFrom.B2 | Ranges with `to < filterFrom` are kept without calling the filter | `mirrors/cm6/state/src/rangeset.ts:290` | `rangeset.rs:223` | test-rangeset#08 | test_rangeset#008 (weak) | implemented, untested exactly |
Proposed status: PORTED, DEVIATION.

### state.cm6.264 `RangeSet.update spec.filterTo` — cm6 `mirrors/cm6/state/src/rangeset.ts:219`
| id | behavior | cm6 file:line | Ferrum code file:line | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.RangeSetUpdate.filterTo.B1 | Default `this.length` | `mirrors/cm6/state/src/rangeset.ts:272` | `usize::MAX` `rangeset.rs:289` | test-rangeset#05 | test_rangeset#005 | implemented+tested (equivalent) |
| state.RangeSetUpdate.filterTo.B2 | Ranges with `from > filterTo` are kept without calling the filter | `mirrors/cm6/state/src/rangeset.ts:290` | `rangeset.rs:223` | test-rangeset#08 | test_rangeset#008 (weak) | implemented, untested exactly |
Proposed status: PORTED, DEVIATION.

### Unfinished

none
