CR="ferrum/crates/ferrum-state/README.md"
SR="ferrum/crates/ferrum-state/src/README.md"
RR="ferrum/README.md"
FL="ferrum/crates/ferrum/src/lib.rs"
FC="ferrum/crates/ferrum/Cargo.toml"
WC="ferrum/Cargo.toml"
SC="ferrum/crates/ferrum-state/Cargo.toml"
V="ferrum_strategic_architecture_vision.md"
G="git log (ferrum)"
P="crates/ferrum-state/src/"
T=True
PAR_CE=("counterexamples: R4 P0 (selection outside doc accepted; cm6 throws `state/src/selection.ts:241`), P1 (U+2028/U+0085/VT/FF split lines; cm6 `state/src/state.ts:246` splits only on `\\r\\n?|\\n`), "
        "P2 (\"😀a\" length 2 vs cm6 3, D1), P4 (cluster breaks differ for CRLF, Hangul, Devanagari conjunct, prepend, letter+ZWJ+letter), P9 (readOnly combine), P10 (allowMultipleSelections not enforced), "
        "P11 (lineSeparator ignored), P12 (no extension dedup, cm6 `state/src/facet.ts:522`), P13 (dynamic facet reading a dynamic facet panics; cm6 `state/src/facet.ts:570`), P14 (annotation order), P16/P17/P18 (RangeSet); "
        "no counterpart anywhere in `crates/*/src` for changeFilter, transactionExtender, languageData, toJSON/fromJSON, wordAt, sliceDoc, lineBreak, isUserEvent, appendConfig (grep of snake_case names, R8)")
rows=[]
def r(claim, src, ev, v): rows.append((claim, src, ev, v))
# ---------------- crate README ----------------
r("crates.io badge \"v0.1.0\"", CR+":3", "Version 0.1.0 matches `Cargo.toml:13`; publication on crates.io not checked (offline audit, no network access used)", "unverifiable")
r("docs.rs badge for `ferrum-state`", CR+":4", "No network access used; no docs.rs build can be checked offline", "unverifiable")
r("\"License: MIT\" (badge)", CR+":5", "`Cargo.toml:16` `license = \"MIT\"`, inherited by `crates/ferrum-state/Cargo.toml:5`", "true")
r("badge links to `LICENSE-MIT`", CR+":5", "No `LICENSE*` file in `/workspace/ferrum/` or `crates/ferrum-state/` (`ls`), so the link target does not exist", "false")
r("\"Core functional data structures and transaction engine for Ferrum.\"", CR+":7", "Text/ChangeSet/EditorSelection/EditorState/Transaction exist (`"+P+"lib.rs:16-33`)", "true")
r("\"Direct 1:1 mathematical parity with `@codemirror/state`\"", CR+":8", PAR_CE, "false")
r("\"implements the core data structures that govern state management, transactional document updates, operational transformation (OT), multi-cursor selections, and the facet configuration system\"", CR+":14", "`"+P+"state.rs:95`, `"+P+"transaction.rs:164`, `"+P+"change.rs:320`, `"+P+"selection.rs:117`, `"+P+"facet.rs:53`, `"+P+"config.rs:24`", "true")
r("\"It has **zero UI dependencies**\"", CR+":14", "Dependencies `crates/ferrum-state/Cargo.toml:10-15`: ropey, smallvec, parking_lot, thiserror, unicode-segmentation; `cargo tree -p ferrum-state` shows no UI/windowing/shaping/GPU crate", "true")
r("\"compiles to native targets\"", CR+":14", "Builds and tests on x86_64-unknown-linux-gnu (R7)", "true")
r("\"[compiles to] WebAssembly\"", CR+":14", "Only `x86_64-unknown-linux-gnu` is installed (`rustup target list --installed`), so no wasm build could be attempted. Code note: `EditorState::update` calls `std::time::SystemTime::now()` (`"+P+"state.rs:279`), which panics at run time on `wasm32-unknown-unknown` per std docs", "unverifiable")
r("\"Backed by balanced B-tree ropes ([`ropey`])\"", CR+":21", "`"+P+"text.rs:10` `rope: Rope`", "true")
r("\"$O(\\log N)$ inserts, deletions, slicing\"", CR+":22", "By code inspection only (no benchmark exists; `find crates -name benches` empty): `Text::replace` = Rope clone + `remove` + `insert` (`"+P+"text.rs:154-167`), `Text::slice` = `RopeSlice` into `Rope` (`"+P+"text.rs:170-179`); relies on ropey's documented complexity. Note `slice_to_string` and `line_by_index` (`"+P+"text.rs:113`) allocate O(length) Strings", "true")
r("\"index translation (byte $\\leftrightarrow$ UTF-16 $\\leftrightarrow$ Unicode char)\"", CR+":22", "`Text` exposes only char<->byte (`"+P+"text.rs:73-83`) and char<->line; grep for `utf16`/`utf_16` in `crates/*/src` finds nothing; UTF-16 is reachable only through the raw `as_rope()` (`"+P+"text.rs:195`)", "false")
r("\"Immutable, cheap snapshots using copy-on-write (`Arc`)\"", CR+":23", "`Text` derives `Clone` over `ropey::Rope` (`"+P+"text.rs:8-11`); ropey nodes are `Arc`-shared; all mutators return a new `Text` (`"+P+"text.rs:154-192`)", "true")
r("\"Section-based delta representation (`SectionIter`)\"", CR+":26", "`ChangeDesc.sections: Vec<i64>` (`"+P+"change.rs:22-24`); `SectionIter` exists but is private (`"+P+"change.rs:644`)", "true")
r("\"**Composition**: $A \\circ B$ combines two consecutive changes into a single atomic changeset.\"", CR+":27", "`ChangeSet::compose` (`"+P+"change.rs:446`), algorithm mirrors cm6 `composeSets` (`state/src/change.ts:238`); R4 P20 uses it on 20000 random cases", "true")
r("\"**Mapping (Rebasing)**: $A \\circ B' = B \\circ A'$ ensures confluence under concurrent transactions.\"", CR+":28", "Empirical only: R4 P20, `a.compose(b.map(a,false)).apply(d) == b.compose(a.map(b,true)).apply(d)` held in 20000/20000 random ASCII cases (xorshift64, seed 0x9E3779B97F4A7C15); no proof", "true")
r("\"**Inversion**: `ChangeSet::invert(&doc)` produces the exact inverse changeset needed for undo/redo stacks.\"", CR+":29", "`"+P+"change.rs:412-443` equals cm6 `state/src/change.ts:219-236` step by step; R4 P20: `invert(doc).apply(apply(doc)) == doc` in 20000/20000 cases", "true")
r("\"Strict canonical normalization invariant mirroring CodeMirror 6\"", CR+":32", "R5 differential oracle: 5000 random selections, 0 mismatches against cm6 `EditorSelection.create` (`state/src/selection.ts:188-236`). Outside normalization, `create` differs on empty input / invalid main index (R4 P8, P8c), counted under `state.claim` rows for `selection.rs:146`", "true")
r("\"Adjacent empty cursors at the same position merge.\"", CR+":33", "R4 P8b: two `cursor(3)` → one; same in cm6", "true")
r("\"Touching non-empty ranges (e.g. `6..7` and `7..8`) **coexist** without unintended merging.\"", CR+":34", "R4 P8b: `(6,7),(7,8)` kept, cm6 identical; `"+P+"selection.rs:180`", "true")
r("\"Overlapping ranges merge, inheriting the direction (`inverted`) of the latest overlapping range.\"", CR+":35", "R4 P8b: `(10,12)+(13,11)` → `(13,10)` in both; `"+P+"selection.rs:186-190` ≡ `state/src/selection.ts:230-233`", "true")
r("\"First-class multi-selection support for arbitrary cursor counts.\"", CR+":36", "`SmallVec<[SelectionRange;1]>` with no limit (`"+P+"selection.rs:118`). Note: the `allowMultipleSelections` facet is never consulted (R4 P10)", "true")
r("\"Ported directly from `@codemirror/state/src/rangeset.ts`.\"", CR+":39", "`RangeSet::map` collects, sorts and rebuilds all ranges (`"+P+"rangeset.rs:239-270`) instead of cm6's per-chunk reuse (`state/src/rangeset.ts:303-333`); side values truncated `as i8` (`"+P+"rangeset.rs:247,250,259,260`, R4 P17); `between` uses ±1e6 side bounds instead of ±1e9 (`"+P+"rangeset.rs:79-80` vs `state/src/rangeset.ts:129`, R4 P16); `add` accepts unsorted input (R4 P18; cm6 throws `state/src/rangeset.ts:527`); `compare` lacks `minPointSize`, `boundChange`, shared chunks", "false")
r("\"B-tree chunk partitioning\"", CR+":40", "`RangeSet` is a flat `Vec` of chunks plus a linked `next_layer` (`"+P+"rangeset.rs:92-97`); there is no tree", "false")
r("\"(chunk size 250)\"", CR+":40", "`"+P+"rangeset.rs:10` `CHUNK_SIZE = 250`; cm6 `state/src/rangeset.ts:97`", "true")
r("\"for $O(\\log N)$ range lookups\"", CR+":40", "`between` visits every chunk linearly (`"+P+"rangeset.rs:173-181`), O(N/250 + k) per layer, plus every layer; no benchmark exists", "false")
r("\"[for $O(\\log N)$ range] updates\"", CR+":40", "`update` calls `iter()` (collect all + sort) and rebuilds with `of_sorted(.., true)` (`"+P+"rangeset.rs:218-236`): O(N log N)", "false")
r("\"Layer spilling preserves identical interval spans across overlapping attributes.\"", CR+":41", "Layer spilling exists (`"+P+"rangeset.rs:362-367`), but the sentence has no testable meaning (\"identical interval spans\" undefined)", "unverifiable")
r("\"Full `spans` iterator [...] with point/mark resolution\"", CR+":42", "`"+P+"rangeset.rs:926-961` matches cm6 `RangeSet.spans` line by line (code comparison; not differentially tested)", "true")
r("\"Full [...] `compare` algorithm[...]\"", CR+":42", "`"+P+"rangeset.rs:896-920` has no `minPointSize` parameter and no set filtering (cm6 `state/src/rangeset.ts:366-377`), no `boundChange` (cm6 `rangeset.ts:876-889`), no shared-chunk skipping (`rangeset.ts:378`), no empty-doc call (`rangeset.ts:384`), and adds a trailing-region comparison cm6 does not do (`"+P+"rangeset.rs:915-923`)", "false")
r("\"Maps cleanly through `ChangeDesc` deltas via `RangeSet::map`.\"", CR+":43", "R4 P17: point at 2 with startSide -256 over insert at 2 → Ferrum `(4,4)`, cm6 `(2,2)` (`start_side() as i8` truncates -256 to 0, `"+P+"rangeset.rs:247`)", "false")
r("\"**Facet System**: Strongly-typed dependency-injection mechanism. Extensions provide inputs to facets without mutating editor internals.\"", CR+":46", "`Facet<Input, Output>` generic (`"+P+"facet.rs:53-58`), `of`/`compute` return `Extension` values (`"+P+"facet.rs:98-127`)", "true")
r("\"**Compartments**: [...] swapped at runtime via `compartment.reconfigure(...)`.\"", CR+":47", "`Compartment::reconfigure` (`"+P+"transaction.rs:57`), applied in `EditorState::apply` (`"+P+"state.rs:310-329`)", "true")
r("\"**State Fields**: Persistent, transaction-aware data fields [...] that update deterministically with every transaction.\"", CR+":48", "`"+P+"state.rs:332-341` calls `update` for every field on every `apply`", "true")
r("\"Cursor stepping [...] strictly respect[s] Unicode Extended Grapheme Clusters (`unicode-segmentation`)\"", CR+":51", "`find_cluster_break` uses `grapheme_indices(true)` (`"+P+"char.rs:44`); R4 P4. (Differs from cm6's `@marijn/find-cluster-break`, see R4 P4.)", "true")
r("\"word boundaries [...] strictly respect Unicode Extended Grapheme Clusters\"", CR+":51", "`find_word_break` steps by `find_cluster_break` (`"+P+"char.rs:81-96`); R4 P6b `\"ae\\u0301 b\"` → 3 (cluster end)", "true")
r("\"subword boundaries (`camelCase` / `snake_case`) [...] strictly respect Unicode Extended Grapheme Clusters\"", CR+":51", "`find_subword_break` steps one `char` at a time (`"+P+"char.rs:113-147`); R4 P6: `find_subword_break(\"ae\\u{301}b\",0,true)` = 2, inside the cluster 1..3", "false")
r("\"character categorization strictly respect[s] Unicode Extended Grapheme Clusters\"", CR+":51", "`categorize_char` takes one `char` (`"+P+"char.rs:12`); `char_categorizer` looks only at the first char (`"+P+"state.rs:191-196`). R4 Q1: `categorizer(\"\\u{301}e\")` Ferrum `Other`, cm6 `Word`", "false")
r("Quickstart example (lines 57-80) compiles", CR+":57", "R1: compiles unchanged inside `fn main`", "true")
r("Quickstart asserts `new_state.doc().to_string() == \"Hello Ferrum!\"`", CR+":78", "R1: assertion passes", "true")
r("Quickstart asserts `new_state.selection().main().head == 14`", CR+":79", "R1: passes in Ferrum, but 14 > doc length 13; the cm6 equivalent throws `RangeError: Selection points outside of document` (R4 P0, `state/src/selection.ts:241`). The example works only because Ferrum does not validate selections", "true")
r("\"All test suites are ported directly from CodeMirror 6\"", CR+":86", "cm6 has 172 cases, Ferrum 177 tests (raw T1/T2). `test_cm6_facet_20_errors_on_cyclic_dependencies` (`"+P+"tests/test_facet.rs:251-255`) asserts a plain computed value, no cycle; `test_cm6_state_13_raises_an_error_on_duplicate_extension_groups` (`"+P+"tests/test_state.rs:215-223`) asserts that no error happens", "false")
r("\"[...] pass [...] (177/177 tests)\"", CR+":86", "R7: `cargo test -p ferrum-state` → 177 passed, 0 failed", "true")
r("\"[...] with 100% behavioral parity\"", CR+":86", PAR_CE, "false")
for i,(ts,rs) in enumerate([("test-change","test_change"),("test-selection","test_selection"),("test-rangeset","test_rangeset"),("test-facet","test_facet"),("test-state","test_state"),("test-text","test_text")]):
    r("`@codemirror/state/test/%s.ts` → `tests::%s`"%(ts,rs), CR+":%d"%(87+i), "Module exists (`"+P+"tests/%s.rs`). Name mapping only; per-test equivalence is audited in the T tables. `test-charcategory.ts` → `tests::test_charcategory` exists but is not listed"%rs, "true")
r("Run test suite: `cargo test -p ferrum-state --lib`", CR+":96", "R7: runs 177 tests", "true")
r("\"Dual-licensed under either of: MIT License [...] Apache License, Version 2.0\"", CR+":103", "`Cargo.toml:16` `license = \"MIT\"` only; no Apache license file", "false")
r("License links `https://github.com/your-org/ferrum/blob/main/LICENSE-MIT` / `LICENSE-APACHE`", CR+":104", "Placeholder org `your-org`; `Cargo.toml:17` says `github.com/ferrum-editor/ferrum`; neither file exists in the repository", "false")
r("\"Strict Clean-Room implementation\"", CR+":107", "The same README says RangeSet is \"Ported directly from `@codemirror/state/src/rangeset.ts`\" (line 39); code follows cm6 algorithms step by step (e.g. `"+P+"change.rs:737-808` vs `state/src/change.ts` `composeSets`). A port is a derived work, not clean room; no cm6 MIT notice exists in the repo (grep `copyright` → none)", "false")
r("\"contains zero copyleft or GPL code\"", CR+":107", "All dependencies of ferrum-state are MIT or MIT/Apache-2.0 (ropey MIT; smallvec, parking_lot(_core), lock_api, thiserror(-impl), unicode-segmentation, str_indices, scopeguard, cfg-if, libc, proc-macro2, quote, syn: MIT OR Apache-2.0; unicode-ident adds Unicode-3.0). Provenance of the crate's own code cannot be checked", "unverifiable")
