# C1 behaviors

30 cm6 items owned (state.cm6.218–247), none DROP: 30 sections below. Repro ids (R*, P*, S-p3) refer to `C1-api.md` `## Reproductions`. Test ids: cm6 `test-text#NN` (T1), Ferrum `test_text#NNN` (T2). Mapping of Ferrum test ids to cm6 cases: #001–#024 ↔ cm6 #01–#24 by number; #025/#026/#027 ↔ cm6 #25; #028/#029 ↔ cm6 #26; #030/#031/#032 ↔ cm6 #27/#28/#29. "tested" in a verdict means a Ferrum test asserts the behavior with the cm6 scenario's values or stronger (see `C1-tests.md` for the per-test verdicts).

Shared helpers read (cm6, `state/src/text.ts`): `Tree` consts `:1-7`; `Open` flags `:10`; `TextLeaf` `:166-245` (`lines` `:171`, `children` `:173`, `lineInner` `:175`, `decompose` `:185`, `replace` `:202`, `sliceString` `:211`, `flatten` `:225`, `scanIdentical` `:229`, `split` `:231`); `TextNode` `:251-374` (`lines` ctor `:252-257`, `lineInner` `:259`, `decompose` `:269`, `replace` `:281`, `sliceString` `:304`, `flatten` `:316`, `scanIdentical` `:320`, `from` `:333`); `Text.empty` `:376`; `textLength` `:378`; `appendText` `:384`; `sliceText` `:398`; `RawTextCursor` `:402-473`; `PartialTextCursor` `:475-517`; `LineCursor` `:519-552`; iterator protocol `:554-558`; `Line` `:562-578`; `clip` `:580-583`.
Ferrum code read: `crates/ferrum-state/src/text.rs:1-243` (whole file); ropey 1.6.1 `src/str_utils.rs:1-100` (line-break selection by feature), `src/rope.rs:821` (`line`), `:1851` (`From<RopeSlice>`), `:1997` (`PartialEq`), `src/slice.rs:1794` (`RopeSlice` eq); str_indices 0.4.4 `src/lines.rs:1-12` (break set: LF, VT, FF, CR, NEL, U+2028, U+2029, CRLF).

### state.cm6.218 `Line` — cm6 `state/src/text.ts:562`
Helpers read: `Line` constructor `text.ts:564-574`; producers `TextLeaf.lineInner` `text.ts:175-183`.
| id | behavior (branch/guard/edge case/side effect) | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.B1 | `Line` is an immutable value (all fields `readonly`), constructor `@internal`: only `Text.line`/`lineAt` create one | `text.ts:563-574` | `crates/ferrum-state/src/text.rs:231-243` (`pub struct`, all `pub` fields, constructible, mutable) | none | none | differs: Ferrum `Line` can be built and mutated by any caller (R22) |
| state.Line.B2 | invariant `length == to - from` always holds (getter) | `text.ts:577` | `text.rs:240` stored field | none | none | differs: invariant can be broken by assignment (R22 `(99, 2)`); untested |
Proposed status: PORTED, DEVIATION because the value type exists with the same fields but is mutable/constructible and stores `length` instead of computing it; field-level deviations in 219–223.

### state.cm6.219 `Line.from` — cm6 `state/src/text.ts:566`
Helpers read: `TextLeaf.lineInner` `text.ts:175-183` (`offset`), `TextNode.lineInner` `text.ts:259-267` (`offset = end + 1`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.from.B1 | start position of the line, in UTF-16 code units from document start | `text.ts:177-179` | `text.rs:107` (`rope.line_to_char`, code points) | test-text#25, #26 | test_text#025, #028 | implemented+tested for ASCII; differs for astral chars (D1): R9b cm6 3 \| ferrum 2 |
| state.Line.from.B2 | for line n > 1, `from = previous.to + 1` (every break is exactly one position, never part of a line) | `text.ts:180`, `text.ts:264` | `text.rs:107` | test-text#25 (`(i-1)*101`) | test_text#025 | differs (D2): CRLF gives `prev.to + 2` (R6c `(0,1)`→`(3,4)`), U+2028/U+2029/NEL/VT/FF give `prev.to + 0` because the separator is inside the previous line (R5, R6); tested only for `\n` |
Proposed status: PORTED, DEVIATION because the unit is code points and the inter-line gap depends on the raw separator.

### state.cm6.220 `Line.to` — cm6 `state/src/text.ts:569`
Helpers read: `TextLeaf.lineInner` `text.ts:177` (`end = offset + string.length`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.to.B1 | position at the end of the line content, before the break (or document end for the last line) | `text.ts:177-179` | `text.rs:109-125` (`from + len_chars` minus stripped `\r\n`/`\n`/`\r`) | test-text#25, #26 | test_text#025, #028 | implemented+tested for `\n`; D1 unit |
| state.Line.to.B2 | `to` never includes a separator character | `text.ts:177` (lines stored without separators) | `text.rs:114-120` strips only `\r\n`, `\n`, `\r` | none | none | differs (D2): U+2028, U+2029, U+0085, VT, FF are ropey breaks but are not stripped, so `to` points after the separator (R5 `(0,2,"a\u{2028}")`, R6) |
Proposed status: PORTED, DEVIATION because of D1 and the incomplete separator stripping.

### state.cm6.221 `Line.number` — cm6 `state/src/text.ts:571`
Helpers read: `TextLeaf.lineInner` `text.ts:178-181`, `TextNode.lineInner` `text.ts:261-265`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.number.B1 | 1-based line number | `text.ts:178-181` | `text.rs:123` (`line_idx + 1`) | test-text#25, #26 | test_text#025, #028 | implemented+tested |
| state.Line.number.B2 | numbering counts only the line structure of the `Text` (separators inside a `Text.of` string never start a line) | `text.ts:171`, `text.ts:256` | `text.rs:137` → ropey `char_to_line` (unicode_lines) | none | none | differs (D2): R6 `line_at(8).number` → cm6 1 \| ferrum 5 |
Proposed status: PORTED, DEVIATION because line numbering follows ropey's break set.

### state.cm6.222 `Line.text` — cm6 `state/src/text.ts:573`
Helpers read: `TextLeaf.lineInner` `text.ts:177-179` (`string = this.text[i]`).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.text.B1 | the line's content without break | `text.ts:177-179` | `text.rs:113-120` | test-text#25, #26 | test_text#023, #025, #028 | implemented+tested for `\n`; differs (D2) for U+2028/U+2029/NEL/VT/FF (kept in `text`, R5, R6) |
| state.Line.text.B2 | no copy: the stored leaf string is returned (allocation-free apart from the `Line` object) | `text.ts:177-179` | `text.rs:113` `line_slice.to_string()` per call | none | none | differs (allocation): O(line length) `String` allocation per `line`/`line_at`/`line_by_index` call; measured P7: 84 µs/call on a 1,000,000-char line vs 0.9 µs in cm6 |
Proposed status: PORTED, DEVIATION because of separator handling and per-call allocation.

### state.cm6.223 `Line.length` — cm6 `state/src/text.ts:577`
Helpers read: none beyond `Line`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Line.length.B1 | `to - from`, UTF-16 units, excluding the break | `text.ts:577` | `text.rs:126` (`text_len`, code points) | none | none | implemented, untested; D1 (R9 cm6 4 \| ferrum 3) |
| state.Line.length.B2 | computed on access, cannot diverge from `to - from` | `text.ts:577` | `text.rs:240` stored `pub` field | none | none | differs (R22) |
Proposed status: PORTED, DEVIATION (untested: B1; differs: B2).

### state.cm6.224 `TextIterator` — cm6 `state/src/text.ts:15`
Helpers read: `RawTextCursor` `text.ts:402-473`, `PartialTextCursor` `text.ts:475-517`, `LineCursor` `text.ts:519-552`, iterator protocol `text.ts:554-558`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.B1 | one interface for the three cursor kinds (raw, range, lines) with `next(skip)`, `value`, `done`, `lineBreak` | `text.ts:15-28` | none | test-text#17–#23 | none | missing |
| state.TextIterator.B2 | the iterator is itself `Iterable` (returns `this` from `[Symbol.iterator]`), so `for…of` works on cursors | `text.ts:556-557` | none | none | none | missing |
| state.TextIterator.B3 | `next()` mutates and returns the same object (no allocation per step) | `text.ts:19`, `text.ts:463-469` | none | test-text#19 | none | missing |
Proposed status: MISSING (searched: see C1-api.md state.cm6.224).

### state.cm6.225 `TextIterator.next` — cm6 `state/src/text.ts:19`
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

### state.cm6.226 `TextIterator.value` — cm6 `state/src/text.ts:22`
Helpers read: `RawTextCursor` fields `text.ts:403-405`, `PartialTextCursor.nextInner` `text.ts:502`, `LineCursor.next` `text.ts:526-546`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.value.B1 | `""` before the first `next()` (R23c) | `text.ts:405` | none | none | none | missing |
| state.TextIterator.value.B2 | `""` at the end; stays `""` after further `next()` (R23d) | `text.ts:425-427` | none | test-text#17 (loop end) | none | missing |
| state.TextIterator.value.B3 | a line break is reported as the literal `"\n"`, independent of `lineSeparator` | `text.ts:436` | none | test-text#17 | none | missing |
Proposed status: MISSING.

### state.cm6.227 `TextIterator.done` — cm6 `state/src/text.ts:25`
Helpers read: `RawTextCursor.nextInner` `text.ts:418-428`, `PartialTextCursor.nextInner` `text.ts:491-503`, `LineCursor.next` `text.ts:528-533`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.done.B1 | `false` until the end is reached, then `true` | `text.ts:418`, `text.ts:425` | none | test-text#17, #18, #20, #21, #23 | none | missing |
| state.TextIterator.done.B2 | for a range cursor, `done` becomes true when the produced value is empty (`done = !this.value`) | `text.ts:503` | none | test-text#20, #21 | none | missing |
Proposed status: MISSING.

### state.cm6.228 `TextIterator.lineBreak` — cm6 `state/src/text.ts:27`
Helpers read: `RawTextCursor.nextInner` `text.ts:432-438`, `PartialTextCursor.lineBreak` `text.ts:513`, `LineCursor.lineBreak` `text.ts:548`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.TextIterator.lineBreak.B1 | raw cursor: `true` exactly when the current string is a break | `text.ts:435` | none | test-text#17 | none | missing |
| state.TextIterator.lineBreak.B2 | range cursor: `true` only if the underlying cursor is on a break and the clipped value is non-empty | `text.ts:513` | none | none | none | missing |
| state.TextIterator.lineBreak.B3 | line cursor: always `false` (R26h) | `text.ts:548` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.229 `Text` — cm6 `state/src/text.ts:31`
Helpers read: all shared helpers listed at the top.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.B1 | persistent immutable value; every operation returns a new `Text` and leaves the receiver unchanged | `text.ts:54-74` | `text.rs:154-192` (clone rope, mutate copy) | test-text#14 (reuse of `doc` after replaces) | test_text#014 | implemented+tested |
| state.Text.B2 | B-tree of leaves holding ≤ `Tree.Branch` (32) lines each, with structural sharing between versions | `text.ts:1-7`, `text.ts:166-374` | ropey `Rope` (`Arc` nodes, byte-sized chunks; not line-based) | test-text#04, #05, #06, #10 | none (shape not observable) | differs: shape is not exposed (see state.cm6.244) |
| state.Text.B3 | stores lines without separators; a break is one position | `text.ts:163-165`, `text.ts:378-382` | `text.rs:9-11` raw rope with separators | none directly | none | differs (D2): R4, R5, R6, R6c |
| state.Text.B4 | iterable (`Symbol.iterator` → `iter()`) | `text.ts:555` | none | none | none | missing (state.cm6.245) |
Proposed status: PORTED, DEVIATION because the type and its core edit/query operations exist, but the text model (D2), unit (D1), tree API, iterators and JSON differ or are missing.

### state.cm6.230 `Text.length` — cm6 `state/src/text.ts:33`
Helpers read: `textLength` `text.ts:378-382`, `TextLeaf` ctor `text.ts:167`, `TextNode.from` length default `text.ts:333`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.length.B1 | length = sum of line lengths + (lines − 1) | `text.ts:378-382` | `text.rs:36-38` (`len_chars`) | test-text#03 | test_text#003 | implemented+tested (ASCII) |
| state.Text.length.B2 | unit: UTF-16 code units (astral char = 2) | `text.ts:380` (`line.length`) | `text.rs:37` code points | none | none | differs (D1): R9 cm6 4 \| ferrum 3 |
| state.Text.length.B3 | a line break counts 1 regardless of the original separator (after `toText`) | `state/src/state.ts:184-185` + `text.ts:380` | `text.rs:37` counts raw separator chars | none | none | differs (D2): `"a\r\nb"` → cm6 3 (S-p3) \| ferrum 4 (R6c) |
Proposed status: PORTED, DEVIATION (D1, D2).

### state.cm6.231 `Text.lines` — cm6 `state/src/text.ts:35`
Helpers read: `TextLeaf.lines` `text.ts:171`, `TextNode` ctor `text.ts:252-257`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.lines.B1 | always ≥ 1 (empty doc has 1 line) | `text.ts:34`, `text.ts:376` | `text.rs:52-54` (ropey `len_lines` ≥ 1) | none | none (test_text#004 asserts 2000, not the empty case) | implemented, untested (R8 both 1) |
| state.Text.lines.B2 | equals the number of stored lines (`Text.of(a).lines == a.length`) | `text.ts:171`, `text.ts:256` | `text.rs:53` | test-text#23, #25 (via `doc.lines + 1`) | test_text#004, #023, #025 | implemented+tested for `\n`-free line arrays |
| state.Text.lines.B3 | characters inside a line (`\r`, U+2028, U+2029, NEL, VT, FF) never add lines | `text.ts:153-157` (no splitting) | ropey `unicode_lines` (str_indices `src/lines.rs:1-12`) | none | none | differs (D2): R5 2 vs 3, R6 1 vs 5, R4 1 vs 2 |
Proposed status: PORTED, DEVIATION (D2; B1 untested).

### state.cm6.232 `Text.lineAt` — cm6 `state/src/text.ts:38`
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

### state.cm6.233 `Text.line` — cm6 `state/src/text.ts:45`
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

### state.cm6.234 `Text.replace` — cm6 `state/src/text.ts:54`
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

### state.cm6.235 `Text.append` — cm6 `state/src/text.ts:64`
Helpers read: `Text.replace` and its helpers (state.cm6.234).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.append.B1 | `replace(length, length, other)`: last line of `this` joins the first line of `other`; `lines = a.lines + b.lines − 1` | `text.ts:64-66` | `text.rs:189-191` (`Rope::append`) | test-text#02 | test_text#002 | implemented+tested |
| state.Text.append.B2 | empty operand yields a value equal to the other operand | `text.ts:58` | `text.rs:183-188` (early returns) | none | none | implemented, untested |
| state.Text.append.B3 | line count is exactly `a.lines + b.lines − 1` for every input | `text.ts:384-396` | ropey | none | none | differs (D2): P1 `["a\r"]` + `["","b"]` → cm6 1 + 2 − 1 = 2 \| ferrum 2 + 2 − 2 = 2 (CR/LF merged) |
Proposed status: PORTED, DEVIATION (B2 untested, B3 D2).

### state.cm6.236 `Text.slice` — cm6 `state/src/text.ts:69`
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

### state.cm6.237 `Text.sliceString` — cm6 `state/src/text.ts:77`
Helpers read: `TextLeaf.sliceString` `text.ts:211-223`, `TextNode.sliceString` `text.ts:304-314`, `clip` `text.ts:580-583`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.sliceString.B1 | clip boundaries | `text.ts:212`, `text.ts:305` | `text.rs:143-145` | test-text#28 (random in-range) | test_text#031 (fixed in-range) | implemented+tested in range; out-of-range implemented, untested (R16 equal) |
| state.Text.sliceString.B2 | `to` defaults to `length` | `text.ts:211`, `text.ts:304` | none | none | none | differs (R15b) |
| state.Text.sliceString.B3 | `lineSep` argument (default `"\n"`) is inserted for every break in range | `text.ts:216-217`, `text.ts:309` | none (no parameter) | none | none | missing (R15, R15c); used by `EditorState.sliceDoc` (`state/src/state.ts:189-190`) |
| state.Text.sliceString.B4 | substring content | `text.ts:218-219` | `text.rs:145` | test-text#28 | test_text#010, #013, #019, #020, #021, #022, #031 | implemented+tested (ASCII) |
| state.Text.sliceString.B5 | UTF-16 offsets; can return a lone surrogate | `text.ts:219` | code points (cannot split a code point) | none | none | differs (D1): R9 `(1,2)` → `"\ud83d"` vs `"😀"` |
| state.Text.sliceString.B6 | breaks are always output as `lineSep`, never as the original separator | `text.ts:217` | `text.rs:145` raw rope content | none | none | differs (D2): R15d raw `"\r\n"` returned |
Proposed status: PORTED, DEVIATION (B2/B3 missing, B5 D1, B6 D2).

### state.cm6.238 `Text.eq` — cm6 `state/src/text.ts:86`
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

### state.cm6.239 `Text.iter` — cm6 `state/src/text.ts:104`
Helpers read: `RawTextCursor` `text.ts:402-473`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.iter.B1 | `dir = 1` (default): yields lines and breaks in order | `text.ts:104`, `text.ts:412-414` | none | test-text#17 | none | missing |
| state.Text.iter.B2 | `dir = -1`: yields them in reverse | `text.ts:414`, `text.ts:442` | none | test-text#18 | none | missing |
| state.Text.iter.B3 | breaks are separate `"\n"` items with `lineBreak = true`; line items never contain `"\n"` | `text.ts:432-438` | none | test-text#17 | none | missing |
| state.Text.iter.B4 | empty lines yield no item (two consecutive breaks) (R23) | `text.ts:440-448` (`next.length > max(0, skip)`) | none | none | none | missing |
| state.Text.iter.B5 | supports skip/negative skip (state.cm6.225) | `text.ts:463-469` | none | test-text#19 | none | missing |
Proposed status: MISSING.

### state.cm6.240 `Text.iterRange` — cm6 `state/src/text.ts:108`
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

### state.cm6.241 `Text.iterLines` — cm6 `state/src/text.ts:115`
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

### state.cm6.242 `Text.toString` — cm6 `state/src/text.ts:132`
Helpers read: `sliceString` `text.ts:211-223`, `text.ts:304-314`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.toString.B1 | whole document with lines joined by `"\n"` | `text.ts:132` | `text.rs:200-204` (`write!(f, "{}", rope)`) | test-text#01, #02, #05–#09, #11, #12, #27 | test_text#001, #002, #005–#012, #017, #018, #030 | implemented+tested for `\n` documents |
| state.Text.toString.B2 | always `"\n"`, never the document's original separator | `text.ts:132` → `text.ts:211` default `lineSep` | raw rope | none | none | differs (D2): R19 `"ab\r\n\r\nc"` returned raw |
Proposed status: PORTED, DEVIATION (B2 D2).

### state.cm6.243 `Text.toJSON` — cm6 `state/src/text.ts:136`
Helpers read: `TextLeaf.flatten` `text.ts:225-227`, `TextNode.flatten` `text.ts:316-318`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.toJSON.B1 | returns the array of lines (no separators) | `text.ts:136-140` | none | test-text#24 | none | missing (R19) |
| state.Text.toJSON.B2 | `Text.of(t.toJSON()).eq(t)` round-trips | `text.ts:135` (doc), `text.ts:153` | none | test-text#24 | none (test_text#024 compares `Text::of(&lines)` with a second `Text::of(&lines)`) | missing |
| state.Text.toJSON.B3 | empty document → `[""]` (R8) | `text.ts:376` | none | none | none | missing |
Proposed status: MISSING (searched: `to_json`, `toJSON`, `serde`, `Serialize` in all crates).

### state.cm6.244 `Text.children` — cm6 `state/src/text.ts:147`
Helpers read: `TextLeaf.children` `text.ts:173`, `TextNode` ctor `text.ts:254`, `TextNode.from` `text.ts:333-373`, `TextLeaf.split` `text.ts:231-244`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.children.B1 | leaf → `null` (R20b) | `text.ts:173` | none | test-text#06 (depth 1) | none | missing |
| state.Text.children.B2 | branch → array of child `Text`s with implied breaks between them | `text.ts:247-254` | none | test-text#10 (`children[0].length`) | none | missing |
| state.Text.children.B3 | loading 2000 lines gives depth ≤ 2 | `text.ts:156`, `text.ts:333-373` | none | test-text#04 | none (test_text#004 asserts `len_lines() == 2000`) | missing |
| state.Text.children.B4 | after edits depth stays ≤ 2; after near-total deletion collapses to a leaf | `text.ts:291-297`, `text.ts:336-340` | none | test-text#05, #06 | none | missing |
Proposed status: MISSING. The tree shape is observable in cm6 only through this property; ropey's tree is private, so none of B1–B4 can be asserted in Ferrum.

### state.cm6.245 `Text.[Symbol.iterator]` — cm6 `state/src/text.ts:150`
Helpers read: `text.ts:554-558`, `RawTextCursor` `text.ts:402`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.Symbol.iterator.B1 | `for (s of text)` / spread yields `iter()` items (R27) | `text.ts:555` | none (no `IntoIterator for &Text`) | none | none | missing |
| state.Text.Symbol.iterator.B2 | cursors returned by `iter`/`iterRange`/`iterLines` are themselves iterable | `text.ts:556-557` | none | none | none | missing |
Proposed status: MISSING.

### state.cm6.246 `Text^of` — cm6 `state/src/text.ts:153`
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

### state.cm6.247 `Text^empty` — cm6 `state/src/text.ts:160`
Helpers read: `text.ts:376`, `TextLeaf` `text.ts:166-173`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Text.empty.B1 | `length 0`, `lines 1`, `toJSON() [""]`, `children null` | `text.ts:376` | `text.rs:21-25` (`len 0`, `len_lines 1`) | test-text#29 (`Text.empty.slice(…)`) | test_text#032 (`Text::empty().slice(…)`) | implemented; only `slice` on it is tested; `lines == 1` untested (R8) |
| state.Text.empty.B2 | a single shared instance (`Text.of([""]) === Text.empty`), used as the "no text" value by `replace`/`ChangeSet` | `text.ts:155`, `text.ts:376` | function returning a new value each call | none | none | not applicable to Rust value semantics (R2 value-equal) |
| state.Text.empty.B3 | replacing with `Text.empty` deletes | `text.ts:58` (`if (text.length)`) | `text.rs:162` (`if !replacement.is_empty()`) | test-text#06, #07, #08, #09 | test_text#006, #007, #008, #009 | implemented+tested |
Proposed status: PORTED, DEVIATION (B1 partly untested; B2 not applicable).

## Unfinished

none
