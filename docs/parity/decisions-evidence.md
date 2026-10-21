# Evidence for foundational decisions

Collected by the parity audits (brief section 8). The audits do not decide; a human records decisions in `decisions.md`. Each section gives the decision id, the items affected, concrete examples (`file:line`, run reproductions) and what each option implies. Full tables live in the raw notes cited per section.

## From the `ferrum-state` audit (crate 1 of 4)

Decisions touched: D1, D2, D5, D6, D9, D10, plus a grapheme-policy question that has no D-number. D3, D4, D7 and D8 are not touched by `@codemirror/state` (no syntax, layout, style or input in that package).

### D1. Position unit

**Items affected:** every API that takes or returns a position or a length: `Text` (length, `line`, `lineAt`, `slice`, `sliceString`, iterators), `Line` fields, `ChangeDesc`/`ChangeSet` (section lengths, `mapPos`, `iterChanges`, `iterGaps`, `touchesRange`, JSON), `SelectionRange`/`EditorSelection` (all positions, JSON), `findClusterBreak`, `codePointAt`, `fromCodePoint`, `codePointSize`, `countColumn` (`to`), `findColumn` (result), `EditorState` (`sliceDoc`, `wordAt`, `languageDataAt`, `charCategorizer(at)`, `replaceSelection` cursor, `checkSelection` bound, JSON), `RangeSet` (integers supplied by callers). Ferrum-only conversion helpers `Text::len_chars`, `len_bytes`, `char_to_byte`, `byte_to_char`, `char_to_line`, `line_to_char`, `line_by_index` (`api-matrix.md`: `BLOCKED: D1`). Sources: `state/raw/C1-notes.md`, `C2-notes.md`, `C3-notes.md`, `C4-notes.md`, `C5-notes.md`, `C6-notes.md` (`## Decision evidence`).

**Current units:** cm6 UTF-16 code units (`state/src/text.ts:380`, `state/src/char.ts:19-37`); Ferrum `char` indices (`crates/ferrum-state/src/text.rs:37`, `char.rs:32-42`, `column.rs:9`). `ferrum-view-gpui` keeps its own conversions (`crates/ferrum-view-gpui/src/lib.rs:11`, `char_to_byte_offset`/`byte_to_char_offset`) instead of the `Text` ones.

**Concrete values (all reproduced on both sides):**

| case | cm6 (UTF-16) | Ferrum now (code points) | UTF-8 bytes | source |
|---|---|---|---|---|
| `Text.of(["a😀b"]).length` | 4 | 3 | 6 | C1 R9, R20d |
| `sliceString(1, 2)` of `"a😀b"` | lone high surrogate `"\ud83d"` | not expressible (code points cannot split the pair) | not expressible (inside a code point) | C1 R9 |
| insert `"😀"` at 1 into `"a😀b"`: section | `1 0:2 3` (newLength 6) | `1 0:1 2` (new_length 4) | `1 0:4 5` | C2 R9 |
| cm6 change `delete(1,3)` on `"a😀b"` replayed in Ferrum | valid | panic `Mismatched document length` | needs re-mapping | C2 R9 |
| `findClusterBreak("😀", 0)` | 2 | 1 | 4 | C3 R2/R3 |
| `findClusterBreak("🇫🇷🇩🇪", 0)` | 4 | 2 | 8 | C3 R2/R3 |
| `findColumn("😀b", 1, 4)` | 2 | `Some(1)` | 4 | C3 R3 |
| `codePointSize(0x1F600)` | 2 | (absent; would be 1) | 4 | C3 R3 |
| `EditorSelection.toJSON()` of a cursor after `"é😀"` | `anchor 3` | (no JSON) | 6 | C3 notes |
| `replaceSelection("😀")` in `"ab"` at 1: new cursor | 3 | 2 | 5 | C5 R16d |
| `wordAt(1)` in `"a𝐀b c"` | 0–4 | (absent; would be 0–3) | 0–6 | C5 R30c |
| a test-facet#12 provider over `"😀"` instead of `"hello"` | `doc.length` 2 → facet `1` | 1 → `[100, 10, 1]` | 4 → `[1]` | C4 R-D1 |

**Oracle evidence:** with ASCII input every chunk oracle shows 0 mismatches (text 81,600 checks; change 25,000 cases / ~2.6M `mapPos`; `changeByRange` 3,000 cases). BMP-only non-ASCII (é, 中) also matches, since BMP code points are one UTF-16 unit. Every astral-plane (emoji) sample mismatches in every position-bearing result. With positions converted to code points, `changeByRange`/`replaceSelection` agree on 500 non-ASCII cases, i.e. the algorithms are unit-independent and only the numbers differ (`oracle.md`).

**Tests affected:** no cm6 test in `@codemirror/state` uses a non-ASCII position (checked per chunk: C1, C2, C3, C4, C5 notes; C6 uses positions only). No expected value of the 172 cm6 cases changes under any option.

**What each option implies:**
- *UTF-16 code units:* every cm6 expected value and every cm6 JSON (history, collab peers) is reusable unchanged; `codePointAt`/`fromCodePoint`/`codePointSize` port directly; positions inside a surrogate pair become representable and need a rule (cm6 can create lone surrogates; Rust `String` cannot hold them), so cm6's mid-surrogate behaviors become "snap" rules. Requires a UTF-16 index over the UTF-8 rope (ropey 1.6.1 has `Rope::char_to_utf16_cu` `src/rope.rs:706` and `utf16_cu_to_char` `:727`), and conversion at every UTF-8 library boundary (regex, tree-sitter, `unicode-segmentation`, GPUI, `cosmic-text`).
- *Code points (current):* ASCII and BMP results equal cm6; JSON exchanged with cm6 peers carries different lengths for astral text; `codePointSize` is constantly 1; per-call char→byte conversion is O(n) in today's helpers (`char.rs:38-42`) unless the rope is used; still needs conversions to UTF-8 (libraries) and UTF-16 (LSP, IME).
- *UTF-8 bytes:* native for `unicode-segmentation` (`grapheme_indices` already returns bytes; `char.rs:49,52,63` conversions disappear), regex, tree-sitter and GPUI; lengths differ from cm6 for every non-ASCII character (é included); every API needs a defined behavior for offsets inside a code point (cm6 has the analogous mid-surrogate rule, `@marijn/find-cluster-break` `index.js:48`); `ChangeSet.of`/`apply`/`invert` need boundary validation.

### D2. Line breaks and the `Text` model

**Items affected:** `Text.of`, `Text` (length, `lines`, `line`, `lineAt`, `replace`, `append`, `slice`, `sliceString(lineSep)`, `eq`, `toString`, `toJSON`), `Line` (`to`, `text`, `length`), `ChangeSet.of` string inserts and its `lineSep` argument, `EditorState.create` string docs, `toText`, `sliceDoc`, `lineBreak`, `replaceSelection`, `changes()`, the `lineSeparator` facet; Ferrum-only `Text::from_str`, `From<&str>`, `From<String>` (`BLOCKED: D2`), `line_by_index`, `as_rope`. Sources: `state/raw/C1-notes.md`, `C2-notes.md`, `C3-notes.md`, `C5-notes.md`.

**Facts:**
- `ropey = "1.6"` with default features (`ferrum/Cargo.toml:20`, used only by `crates/ferrum-state/Cargo.toml:11`); locked 1.6.1. Default features enable `unicode_lines` → `cr_lines` and `simd`. Under `unicode_lines` the break set is LF, VT, FF, CR, NEL, U+2028, U+2029 and CRLF (`str_indices-0.4.4/src/lines.rs:1-12`, selected by `ropey-1.6.1/src/str_utils.rs:12-18`).
- cm6 splits on `DefaultSplit = /\r\n?|\n/` (`state/src/change.ts:3`) or on the `lineSeparator` facet, in `toText` (`state/src/state.ts:184-185`), `create` (`:246`), `replaceSelection` (`:130`) and `ChangeSet.of` (`change.ts:335`). Lines are stored without separators; a break counts as one position; output reinserts `state.lineBreak` (`sliceDoc`, `state.ts:189-191`). cm6 `Text.of` itself splits nothing and only throws for `[]` (`text.ts:153-157`).
- Ferrum stores separators raw (`text.rs:28-32`, `:218-228`), `Text::of` joins with `"\n"` and re-parses (`text.rs:86-93`), `line_by_index` strips only `\r\n`, `\n`, `\r` (`text.rs:114-120`), and the `lineSeparator` facet is stored but never read (`state.rs:25-27,185`).

**Concrete values:**

| case | cm6 | Ferrum | source |
|---|---|---|---|
| `Text.of(["a b","c"])` lines / line 1 text | 2 / `"a b"` | 3 / `"a "` (separator kept in `text` and `to`) | C1 R5, harness smoke test |
| `Text.of(["a\r\nb"])` lines | 1 (cm6 keeps it as content) | 2 | C1 R4 |
| `Text.of([])` | throws | empty document | C1 R1 |
| position inside a CRLF break | does not exist | `line_at(2)` of `"a\r\nb"` → line 1, `to = 1` | C1 R6c |
| edits that create or destroy breaks | impossible | append `"a\r"` + `"\nb"` merges; slicing a CRLF leaves a lone CR that is a break | C1 P1, P3, P4 |
| `create({doc: "a\r\nb\rc d"})` lines / length | 3 / 7 | 4 / 8 | C5 R21 |
| `create` with `lineSeparator: "\r\n"`, doc `"a\nb\r\nc"` | 2 lines / 5 | 3 lines / 6 (facet ignored) | C5 R21c |
| insert `"a\r\nb"`: section / text | `0:3` / `"a\nb"` | `0:4` / `"a\r\nb"` | C2 D2 table, C5 R18 |
| insert `"a b"` (U+2028) | 1 line | 2 lines | C2 D2 table |
| `sliceDoc` of doc `"a\r\nb\nc"` | `"a\nb\nc"` (len 5) | `slice_string` → `"a\r\nb\nc"` (len 6) | C5 R20 |
| `replaceSelection("a\r\nb")` into `"xy"` | `"a\nbxy"`, cursor 3 | `"a\r\nbxy"`, cursor 4 | C5 R16 |
| `countColumn("a\r\nb", 4)` | 4 | 3 | C3 R2 T02 |
| whole-document cluster breaking in ferrum-commands | n/a (cm6 works per line) | `\r\n` is one cluster (UAX #29 GB3) | C3 notes |

**Oracle:** with `\r` or U+2028 in line content, strings and lengths agree, but `lines`, `line`, `lineAt` and line texts disagree in almost every case (C1 oracle: lines 314/320, `line` 1,833/1,920).

**What each option implies:**
- *(a) cm6 semantics* (the `ferrum-foundation.md` recommendation): normalize on input with `\r\n?|\n` or `lineSeparator`, build ropey with `default-features = false` so only LF breaks lines, keep the separator in state for output (`lineBreak`, `sliceDoc`). All rows above then match cm6 for `toText`-style input. Sub-decision: what `Text::of` does with an element containing `"\n"` (cm6 silently builds a malformed document; mimic, reject or split). `ChangeSpec`/`Changes::to_changeset` need access to the separator (a string variant split with the state's separator). `as_rope` would expose a non-default ropey configuration.
- *(b) keep raw separators:* every line API needs the full separator table; positions inside CRLF exist; edits can merge or split breaks; cm6 tests and JSON that assume one position per break cannot be reused; output always echoes input separators.

### D5. Command type

**Items affected:** `StateCommand` (state.cm6.035, `PLACEMENT DEVIATION`, XC-001). cm6: `(target: {state, dispatch: (tr) => void}) => boolean`, defined in `@codemirror/state` (`state/src/extension.ts:10`). Ferrum: `pub type StateCommand = fn(&EditorState) -> Option<TransactionSpec>` only in `crates/ferrum-commands/src/commands.rs:20`, used by `KeyBinding` (`:28-44`) and `dispatch_key` (`:55`), re-exported by the facade (`crates/ferrum/src/lib.rs:25`). Source: `state/raw/C4-notes.md`.

**Options:** (a) cm6 shape in `ferrum-state` (state + dispatch callback, returns `bool`): supports several dispatches and "handled without change"; (b) current shape: pure and easy to test, but cannot report "handled, no change", cannot dispatch twice, and an `fn` pointer cannot capture configuration; (c) a closure type returning `Option<TransactionSpec>`, moved to `ferrum-state`: fixes placement and captures, still single-dispatch.

### D6. Ownership, threading and determinism

**Items affected:** facet/field/compartment ids, facet values, `Transaction::state`, `EditorState::create`. Sources: `state/raw/C4-notes.md`, `C5-notes.md`.

- **Ids:** three process-global `AtomicUsize` counters with `Ordering::Relaxed` (`crates/ferrum-state/src/facet.rs:5-7`); built-in facets in `LazyLock` get their id at first access (`state.rs:13-27`), so ids differ between programs and between test orders, and interleave across test threads (C4 R51). Current Ferrum outputs do not depend on id values (evaluation follows first appearance, `config.rs:61,107`). cm6 evaluates facet slots in ascending id order (`state/src/facet.ts:489`; C4 R28: cm6 `a,b`, Ferrum `b,a`): adopting that with global atomic ids would make getter side-effect order depend on thread scheduling.
- **Bounds:** every facet input/output, field value and closure must be `Send + Sync + Clone + 'static` (`facet.rs:36-60,168-172`); values are `Arc<dyn Any + Send + Sync>`; `combine_erased` clones every input on each call (`facet.rs:149-156`).
- **Identity:** cm6 packages above detect change by identity of facet/field values (10 sites, for example `view/src/editorview.ts:329`, `view/src/draw-selection.ts:69`, `view/src/gutter.ts:225,502`, `language/src/language.ts:557`, `language/src/fold.ts:411-412`). In Ferrum, pointer identity of `&Output` is kept only for static facets across non-reconfiguring transactions (C4 R10d) and for defaults (R40); never for dynamic facets (R10-R10c) or after a compartment reconfiguration (R11, R11b, R57). Under `Arc`, `Arc::ptr_eq` is the analogue of `===` and holds for `start_state` (C5 R40).
- **`Transaction::state`:** rebuilt on each call (`transaction.rs:176-178`; C5 R4: `ptr_eq` false, field updates run twice); a field update that reads `tr.state()` overflows the stack (C5 R33c), whereas cm6 sets `tr._state` before computing slots (`state/src/state.ts:58`) and test-state#16 depends on that. A cached `OnceLock<Arc<EditorState>>`-like design would be needed to keep cm6 semantics.

**Options to weigh:** keep `Arc` + `Send + Sync` everywhere (current; enables sharing across threads, costs clones and bounds) or single-threaded `Rc` states with explicit `Send` snapshots; per-state or per-configuration id allocation (deterministic) vs global counters; value-based change detection vs pointer identity (which requires `compare`/`compareInput` to keep old values).

### D9. Licensing

**Items affected:** workspace and crate manifests, READMEs, cm6 notice. Source: `state/claims.md` (state.claim.053 and related licensing rows).

- `ferrum/Cargo.toml` declares `license = "MIT"` for the workspace; `ferrum-state` inherits it. The crate README and the root README state dual MIT/Apache-2.0 and "clean-room". No `LICENSE*` file exists in the repository.
- The crate README says `RangeSet` is "ported directly" from cm6, and the code mirrors cm6 algorithms (for example `rangeset.rs` `SpanCursor`/`HeapCursor`). That makes the crate a derived work of MIT-licensed code; no cm6 copyright/permission notice exists in the repository (rules.md 5.2 requires one).
- Dependencies of `ferrum-state` (ropey, smallvec, parking_lot, thiserror, unicode-segmentation) are permissively licensed (A4 check).

**Options:** MIT only (matches the manifests; README must change) or MIT/Apache-2.0 (manifests and license files must change). Either way: add license files and a `NOTICE`/`THIRD-PARTY` file with cm6's MIT notice; remove "clean-room".

### D10. Short-term scope

**Items affected:** cm6 items that are `MISSING` today and whose need in the first foundation version is a scope question (`api-matrix.md`): JSON (`toJSON`/`fromJSON` on `ChangeSet`, `ChangeDesc`, `SelectionRange`, `EditorSelection`, `EditorState`, field `toJSON`/`fromJSON` hooks: 12 rows marked "JSON: could be `DEFER` under D10" in `dispositions.md`), `phrases`/`phrase` (XC-029), `Text.children` (no consumer in view/language/commands, C1 notes 4), `bidiLevel`/`undirectional` selections (XC-036). Evidence of consumers: JSON is used by cm6 history serialization (`commands/src/history.ts:197-212`); `phrase` by view panels and special chars (`view/src/special-chars.ts:157`, `view/src/dialog.ts:127`); bidi level by cursor motion (`view/src/docview.ts:218-219`).

### Grapheme cluster policy (no D-number; listed in `human-decisions.md`)

cm6 uses a deliberately simplified table-driven approximation (`@marijn/find-cluster-break`: Extend and most SpacingMark below U+30000, ZWJ joins anything, no CR/LF/Control/Hangul/Prepend/GB9c rules). Ferrum uses full UAX #29 (`unicode-segmentation` 1.13). They differ for CRLF, ZWJ next to ordinary letters (`"a‍b"`), Hangul jamo, Indic conjuncts, Prepend characters, tag-sequence flags and control + combining mark (C3 R2; oracle: every one of 122 non-ASCII mismatches falls into these classes; all 105 ASCII mismatches contain CRLF). The choice changes cursor motion, deletion, `countColumn` and `findColumn`. It is independent of D1 but interacts with D2 (whether CRLF can reach the function).
