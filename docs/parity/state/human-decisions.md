# ferrum-state: items left for a human decision

> What: questions left to a human, grouped (HD-…). Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`.

Everything the audit could not decide alone (brief sections 2.2, 4, 12). Grouped by question; every `HUMAN DECISION` row of `docs/parity/state/api-matrix.md` (4 cm6 rows, 59 Ferrum rows) belongs to exactly one group below, and the open questions raised by the chunk audits (`docs/parity/state/raw/C<n>-notes.md` `## Open questions`) are folded in with their source. Foundational decisions D1–D10 are not repeated here; their evidence is in `docs/parity/decisions-evidence.md`.

Also for the human gate (brief section 12): approve the dispositions in `docs/parity/state/dispositions.md` (262 `CORE`, 2 `DROP`), the `FERRUM-ONLY` proposals in `docs/parity/state/ferrum-only.md`, the `PLACEMENT DEVIATION` rows in `docs/parity/state/placement.md`, and every `PORTED, RUST-IDIOMATIC` row of `docs/parity/state/api-matrix.md` (22 cm6, 31 Ferrum, plus inherited re-exports).

## HD-TYPED-IDS. How annotation and effect types are identified (4 cm6 rows)

Rows: state.cm6.121 `Annotation^define`, state.cm6.122 `AnnotationType`, state.cm6.128 `StateEffect^define`, state.cm6.132 `StateEffectType`. Source: C5 Q1.

Ferrum identifies annotations and effects only by the Rust `TypeId` of the value (`crates/ferrum-state/src/transaction.rs:11-44,117-138`). Two cm6 types with the same value type are distinct objects in cm6 and indistinguishable in Ferrum (`docs/parity/state/raw/C5-api.md` R9, R9b); there is no place for `StateEffect.define({map})` (effect mapping, XC-031).
Options: (a) typed `AnnotationType<T>` / `StateEffectType<T>` objects with ids, like cm6; (b) keep `TypeId` and require a newtype per type, documenting it; (c) hybrid (newtype convention plus an optional `map` registered per type).

## HD-INTERNALS. cm6 internals exposed as public API (55 Ferrum rows)

Rows: state.fe.008 `ChangeDesc::create`, .033 `ChangeSet::create_set`, .061–.064 `Configuration` and its `resolve`/`facet`/`field`, .066–.068 `FacetId`/`StateFieldId`/`CompartmentId`, .075–.083 `AnyFacet`/`AnyStateField` and their methods, .087 `Facet::id`, .088 `Facet::default_value`, .093–.096 and .101–.103 the crate-trait impls, .099 `StateField::id`, .105 `Compartment.id`, .110–.115 the six type-erased `Extension` variants, .119 `CHUNK_SIZE`, .131–.139 `Chunk` and its fields/methods, .141–.144 `RangeSet` fields, .148 `RangeSet::length`, .167 `RangeSetBuilder::add_range`, .232 `EditorState::apply`, .267–.269 `CompartmentReconfigure` and its fields. Sources: C2 Q4, C4 Q1–Q2, C5 Q6, Q8, C6 Q2.

Facts: in cm6 these are `@internal`, private, or module exports not re-exported by `index.ts`. In Ferrum they can be used to break invariants from outside: a wrong value type in `Extension::FacetStatic` panics with "Facet input type mismatch" (`docs/parity/state/raw/C4-api.md` R45); reusing another facet's id panics with "Static facet type mismatch" (R46); a dynamic provider can be attached to a static facet because `AnyFacet::is_static` is never called (R46b; `is_static` and `default_erased` are dead code); `ChangeSet::create_set` with no inserted texts applies as a no-op while `new_length` reports 4 (`docs/parity/state/raw/C2-api.md` R11c); `EditorState::apply` accepts a transaction built on another state (`docs/parity/state/raw/C5-api.md` R45); mutating `Chunk`/`RangeSet` fields breaks ordering (not reproduced row by row); setting one `Compartment.id` to another's makes them equal, and reconfiguring one replaces the other's content (`docs/parity/state/verification.md` finding 3). Read-only accessors in this group (e.g. `default_erased`, `AnyStateField::init`) cannot break invariants by themselves. Tests use some of them (`set.chunk.len()`, `CompartmentReconfigure{…}` built by hand instead of `comp.reconfigure(ext)`).
Options per row: (a) `pub(crate)`; (b) public behind `#[doc(hidden)]` or an `internals` feature; (c) public, documented, with validation where invariants exist.

## HD-NAMES. Naming and helpers that mislead porters (3 Ferrum rows)

- state.fe.191 `EditorSelection::cursor(pos)` returns a selection equal to cm6 `EditorSelection.single(pos)`, while cm6 `EditorSelection.cursor` returns a `SelectionRange` (31 call sites in ferrum-commands). Options: rename/remove in favor of `single`, or keep and document. Source: C3 Q4.
- state.fe.019 `ChangeDesc::from_string`: test helper (cm6 test `mk`) in the public API; accepts input cm6 rejects, panics with `expect` (`docs/parity/state/raw/C2-api.md` R2q/R2r); used by Ferrum tests (e.g. `test_change.rs:603`). Options: `#[cfg(test)]`/test-utils feature, or public returning `Result`. Source: C2 Q4.
- state.fe.053 `categorize_char`: categorizer without language `wordChars` and without position; 2 code points differ from cm6 (U+0085, U+FEFF). Options: keep as building block, or make private once `charCategorizer(at)` reads language data. Source: C3 Q5.

## HD-ROPE. `Text::as_rope` (1 Ferrum row)

state.fe.255: exposes `ropey::Rope` (third-party type with `unicode_lines` semantics) in the public API; no caller in the workspace. Tied to D2. Options: remove, feature-gate, keep. Source: C1 Q6.

## HD-API-PATHS. Two public paths per item

Every item is public both as `ferrum_state::<module>::X` (10 public modules) and, for 48 items, as `ferrum_state::X`; cm6 has one flat namespace (`index.ts`). Duplicate API surface also exists inside `Text`: `len`/`len_chars`, `slice_string`/`slice_to_string`, inherent `from_str` vs `From<&str>` (C1 Q6). Options: private modules with root re-exports only; keep both; keep modules and drop duplicates.

## HD-ERRORS. Precondition violations: panic, `Result` or clamp

Sources: C1 Q2, Q4; C2 Q2; C3 Q3, Q6; C6 Q5.
- cm6 throws `RangeError` for out-of-range `line`/`lineAt`, invalid change ranges, mismatched lengths, selections outside the document, `EditorSelection.create([])`, misordered `RangeSetBuilder.add`. Ferrum: `line`/`line_at` panic (no `# Panics` docs, no non-panicking variant, contrary to rules.md §6.4); `line_by_index`, `char_to_line`, `line_to_char`, `char_to_byte`, `byte_to_char`, `slice`, `slice_string`, `replace` clamp silently; `Text::of(&[])` returns an empty document; `EditorSelection::create(vec![])` returns `cursor(0)` and clamps the main index; selections outside the document are accepted; `RangeSetBuilder::add` silently layers misordered input; panic texts differ from cm6's error texts (only `mapPos` matches).
- `tabSize = 0`: cm6 yields NaN-like results, Ferrum panics in `count_column`/`find_column`.
Question: per precondition, panic (documented, rules.md §6.4), `Result`, or clamp; and whether error-message text parity is required.

## HD-GRAPHEME. Grapheme cluster policy

Rows affected: state.cm6.212 `findClusterBreak`, .216 `countColumn`, .217 `findColumn` (status `PORTED, DEVIATION` either way). cm6's simplified algorithm vs full UAX #29; evidence in `docs/parity/decisions-evidence.md` (grapheme section). `includeExtending` is missing in both cases. Source: C3 Q1.

## HD-SELECTION-MODEL. `SelectionRange` representation

Public mutable `anchor`/`head`/`goal_column`/`assoc` fields vs cm6's private from/to/flags. Affects automatic `assoc` (cm6 ±1 on non-empty ranges, Ferrum always 0: 4,380 of 5,000 oracle cases), `eq` (cm6 ignores assoc by default, Ferrum `==` compares it), bidi level and undirectional ranges (XC-036), and `goal_column` type (`usize` vs cm6 `number`, which the view fills with pixel offsets; depends on D4). Source: C3 Q2, Q7.

## HD-CHANGESPEC. Shape of `ChangeSpec`/`Changes`

cm6's recursive union (nested arrays, a `ChangeSet` inside a list, string vs `Text` inserts, `lineSep`) vs Ferrum's flat `Changes` enum; `Changes::Set` skips the length check. Options: recursive `Changes` (`Multiple(Vec<Changes>)`) with a string variant split by the state's separator (ties to D2), or keep flat and document. Source: C2 Q3.

## HD-ASSOC-TYPE. `assoc: i8` in `ChangeDesc::map_pos`

The `i8` parameter makes `RangeSet::map` truncate `i32` sides (`-100_000_000 as i8 == 0`), which affects every ferrum-view decoration (XC-010). Options: `i32`, a sign enum, or keep `i8` and stop casting sides. Also: `ChangeSet::map_pos` is Simple-only (cm6 has a 3-argument overload). Source: C2 Q6, Q7.

## HD-TX-PIPELINE. Transaction pipeline contract

- Filters: cm6 `transactionFilter` rewrites transactions (returns specs; no rejection, an empty spec instead); Ferrum `TRANSACTION_FILTER` vetoes (`bool`, `update → None`) and runs first-to-last (cm6 last-to-first). Keep a veto at all? (C5 Q2)
- `EditorState::update` returning `Option<Transaction>` vs always a `Transaction`; one spec vs `...specs` merged (C5 Q3).
- `Transaction::state()` caching and reentrancy (C5 Q4; D6).
- `Transaction::selection()` returning the mapped selection when none was set (C5 Q5; XC-026).
- `readOnly` combine `any` vs cm6 "first value": no verifiable reason found (C5 Q7; XC-034).
- cm6 quirk: transaction extenders applied twice when a transaction filter returns specs (state.EditorState.update.B22). Reproduce or not? (C5 Q10)

## HD-FACET-MODEL. Evaluation model of facets and fields

Ferrum fills fields, then dynamic facets, eagerly in order, with no dependency list and no on-demand computation (`state.rs:120-132,331-362`, `config.rs`). This is the root of: computed facet reading a computed facet → `index out of bounds` panic; field init reading another field → `None`; no cycle detection; recomputation on every transaction; loss of value identity (`docs/parity/state/raw/C4-api.md` R5–R12, R19, R39). cm6 uses on-demand slots with status flags (`mirrors/cm6/state/src/facet.ts:559-579`). Recorded as a fact for the repair-or-rebuild decision (C4 Q3); no fix proposed.

## HD-RANGESET. RangeSet semantics to keep

- `RangeSet.eq` in cm6 6.7.6 compares only point ranges (`mirrors/cm6/state/src/rangeset.ts:399`; `docs/parity/state/raw/C6-api.md` R30): reproduce exactly when porting? (C6 Q1)
- Structural sharing (`update`/`map` reuse chunks, `compare` skips shared chunks) is observable through `compare` results (`docs/parity/state/raw/C6-api.md` R13), not only performance: required? (C6 Q3)
- `RangeValue::eq_val` default: cm6 object identity vs Ferrum `PartialEq`; options: keep `PartialEq`, require explicit `eq_val`, or `Arc` + pointer comparison (C6 Q4).

## HD-TEXT. `Text` model details

- `Text::of` with an element containing `"\n"`: mimic cm6 (one malformed line), split, or reject (C1 Q1; D2 sub-question).
- `Text.children` (state.cm6.244, status `MISSING`): its only users are cm6's own tree-shape tests (test-text#04, #05, #06, #10). Require a tree API, or revisit the disposition (C1 Q3).
- `Line` with public fields and a stored `length` vs a computed `length()` (C1 Q5).

## HD-TESTS. Test evidence and cm6 test quirks

- Should the differential oracle (or a fixed-case export of it as Ferrum tests) count as test evidence? With it, the "implemented, untested" behaviors that the oracle exercised would be proven (C2 Q1).
- cm6 test-text#13 has a bug (`start = 4150; end = 4160` overwrites the random range each iteration, `mirrors/cm6/state/test/test-text.ts:96`): port it faithfully or corrected? (C1 Q7)
