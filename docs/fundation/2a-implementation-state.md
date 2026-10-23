# Implementation of `ferrum-state` (rebuild)

`ferrum-state` is rebuilt from cm6 `@codemirror/state` 6.7.6, in the existing crate directory, from an empty module tree. `2-parity-implementation.md` applies in full; this sheet adds what is specific to this crate:

1. **Phase 0** produces the crate's specification in the standard form of `1-parity-audit.md` §3, in `ferrum/docs/parity/state/spec/`. The crate was audited before that form existed; Phase 0 converts and completes that audit once. After Phase 0, the implementer reads only `ferrum/docs/parity/state/spec/`, never the older audit files.
2. The decisions and placement rules the crate must follow (§2).
3. The behaviors that the old implementation got wrong and that therefore need particular care (§3).

Paths are relative to `/workspace/`; instruction documents named alone live in `ferrum/docs/fundation/`.

---

## 1. Phase 0: the specification (audit work, no code)

Who: the main session, with one subagent per chunk (at most 4 in parallel), exactly as in `1-parity-audit.md` §4. Chunks: `text.ts`; `change.ts`; `selection.ts` + `char.ts` + `charcategory.ts` + `column.ts`; `facet.ts` + `extension.ts` + `config.ts`; `state.ts` + `transaction.ts`; `rangeset.ts`.

Read once, in Phase 0 only (old audit, folder `ferrum/docs/parity/state/`):

- `raw/a1-cm6-exports.md` and `dispositions.md`: the inventory ids `state.cm6.NNN` and the proposed dispositions;
- `behaviors.md`: only the columns *behavior*, *cm6 file:line* and *cm6 test(s)* (ids `state.<Item>.B<n>`);
- `raw/t1-cm6-tests.md` and the cm6 columns of `tests-side-by-side.md` Table 1: the 172 test cases;
- `oracle/cm6-probes/`: existing cm6 probes and generators, to adapt to the shared harness.

Everything else in that folder describes the old Ferrum code and is not used.

Steps:

1. **Harness and inventory** (main session): `setup_cm6.py state`; regenerate `spec/raw/inventory.jsonl` and `spec/raw/tests.jsonl` with the inventory tools (`ferrum/docs/parity/tools/README.md`). The tools produce the same 264 inventory ids as the old audit (only the labels of the 17 config-object fields differ) and the same 172 test ids; copy the dispositions onto them.
2. **Behaviors** (subagents): for each item, start from the old behavior list, re-read the cm6 source and every helper, and correct or extend the list (the old list is a starting point, not a verified specification). Keep existing ids; new behaviors get the next free number. Add `deps`, `hazards`, `consumer_values` and `cost` per `1-parity-audit.md` §6 and §9.
3. **Fixtures and generators** (subagents): fixtures for every behavior (`1-parity-audit.md` §6.2); generators covering the shape checklist (`1-parity-audit.md` §8). For this crate the checklist entries that matter most are equal and duplicate ranges, the same extension object reused, decoration side values ±1e8…±6e8 and their ±1 adjustments, astral characters, CRLF and U+2028.
4. **Tests** (subagents): every case is `LOGIC-ONLY` (`@codemirror/state` has no `webtest-*`); record the values that change under D1/D2 and the derived-test list.
5. **Synthesis** (main session): `hazards.md`, `human-decisions.md` (the questions in §2 below that are still open), `implementation-plan.md` (`1-parity-audit.md` §16, with the obvious decisions as one list), rendering, reference check.
6. **Verification** (`1-parity-audit.md` §12), then **Gate 2**.

---

## 2. Decisions and rules for this crate

Before code (in addition to `2-parity-implementation.md` §0), `ferrum/docs/parity/decisions.md` records:

- **D1** position unit and **D2** line breaks: they decide the types of `Text`, `ChangeSet`, `SelectionRange` and every position;
- **D5**: whether `StateCommand` lives in `ferrum-state` (as in cm6) and its signature;
- **D6**: ownership and threading, and how ids are allocated (per configuration or global);
- **D9**: license fields, license files, and the cm6 MIT notice in the repository;
- **D10**: first-version scope (for example JSON serialization, `phrases`, `Text.children`).

Questions to answer at Gate 2, each with its options in `spec/human-decisions.md`:

- how annotation and effect *types* are identified (typed objects as in cm6, or Rust types);
- the error contract per precondition (panic with `# Panics`, or `Result`), and whether cm6's error messages must match;
- grapheme clusters: cm6's simplified algorithm (exact parity) or full UAX #29;
- the shape of `ChangeSpec` (every cm6 form: nested lists, sets inside lists, string or `Text` inserts);
- whether structural sharing of `RangeSet` chunks is required (it is observable through `RangeSet.compare`), and whether `RangeSet.eq` reproduces cm6 6.7.6 exactly (it compares only point ranges);
- cm6 test-text#13 has a bug (`CodeMirror6/state/test/test-text.ts:96` overwrites its random range): port it as is or corrected.

Rules:

- `isolateHistory` is not a state item (it belongs to `ferrum-commands`); group and subword motion are not state items (they belong to `ferrum-view` and `ferrum-commands`); character categories come from `EditorState.charCategorizer(at)`, which reads the language's `wordChars`.
- cm6 `@internal` and non-exported mechanisms are `pub(crate)`.
- **Workspace while rebuilding** (the maintainer chooses at Gate 2): the other crates compile against the old `ferrum-state` API and will break. Recommended: remove `ferrum-view`, `ferrum-language`, `ferrum-commands`, `ferrum-view-gpui` and the facade `ferrum` from `[workspace] members` in `ferrum/Cargo.toml` until they are rebuilt, so that `cargo test --workspace` and `clippy` stay meaningful. They stay in the tree; the commands quarry runs the old code from a git worktree at commit `ed70610`. Alternatives: keep them compiling with adapters (work thrown away later), or delete them (git history keeps them).
- The old implementation (`ferrum/crates/ferrum-state/` at commit `ed70610`) is not a base. Its algorithms for `ChangeSet`/`ChangeDesc` (`of` for flat lists, `apply`, `compose`, `map`, `invert`, `mapPos`, `iterChanges`, `iterGaps`, `iterChangedRanges`) and for `changeByRange`/`replaceSelection` matched cm6 on every ASCII oracle case and may be consulted by the implementer (`2-parity-implementation.md` §4, step 3) as a reference; the item is still written against the new types, and its tests come from step 2 like every other test. The test subagent never reads the old code.

---

## 3. Behaviors that need particular care

The old implementation got each of these wrong. Each needs a test that would have caught it; the fixtures of Phase 0 give the expected values. The test card of each module (`2-parity-implementation.md` Appendix A) lists the bullets of that module.

**Text**
- Lines are split exactly as D2 decides; no position falls inside a line break; edits and slices never merge or split a break.
- `line` and `lineAt` cost is local (no copy of the document or of long lines beyond the returned text).
- `iter`, `iterRange`, `iterLines` (with `lineBreak`), and `sliceString(from, to, lineSep)` exist; consumers never need the whole document as a string.
- `Text.of([])` throws, like cm6 (or follows the decided error contract).

**Changes**
- `ChangeSet.of` splits string inserts on `lineSep` or `\r\n?|\n`, accepts every `ChangeSpec` form, and rejects a set of the wrong length.
- `mapPos` takes `assoc` as a full integer; `touchesRange` exists; JSON as D10 decides.

**Selection, characters, columns**
- `EditorSelection.create` keeps the main range by identity through sorting and merging (two equal ranges must not confuse it).
- Non-empty ranges get cm6's automatic `assoc`; `eq` ignores `assoc` unless asked; `bidiLevel` and undirectional ranges exist.
- Selections are checked against the document on creation and on every transaction.
- `findClusterBreak` is local in cost; `countColumn` and `findColumn` are linear in the line length.

**RangeSet**
- Sides travel as integers wide enough for ±6e8 through `map`, `between`, `compare` and `spans`; internal sentinels are larger than any side.
- `spans` reports open-start and open-end counts and its return value exactly as cm6, and never skips a point.
- `update` and `map` share untouched chunks; `compare` skips shared chunks and supports `minPointSize` and `boundChange`.
- Static `iter(sets, from)` with `goto` and `rank`, `eq` and `join` exist; misordered `RangeSetBuilder.add` is an error.

**Facets and configuration**
- Facet and field slots are computed on demand in dependency order; a cycle is reported as an error; a computed facet may read other computed facets and fields.
- Computed facets recompute only when a declared dependency changes, and `compare`/`compareInput` keep the previous value's identity when inputs are equal.
- The same extension object included twice counts once (highest precedence wins).
- Compartments: duplicate use is an error, reconfiguration drops removed content, a field added by reconfiguration is created from the new state; `Compartment.get` exists.
- `enables`, `provide`, `Facet.from`, `FacetReader`, `StateField.init`, `combineConfig` exist; a static facet rejects computed providers.

**State and transactions**
- `Transaction.state` is computed once, and field updates can read it.
- Pipeline order as cm6: change filters, then transaction filters (which may return replacement specs, applied in cm6's order), then extenders; `filter: false`, `sequential` and several specs per update are supported.
- `Transaction.selection` is only the explicitly set selection.
- `readOnly` takes the first value; `allowMultipleSelections` is enforced; `lineSeparator` is used by `toText`, `sliceDoc` and `lineBreak`.
- Effects are mapped by `changeByRange` and `StateEffect.mapEffects`; two effect or annotation types with the same value type stay distinct.
- `isUserEvent` uses prefix matching; `languageDataAt`, `wordAt`, `phrase`, `appendConfig` and `reconfigure` exist.

---

## 4. Order and hand-off

Modules for the workflow of `2-parity-implementation.md` §4, in this order: `text` (D1/D2 types first), `change`, `selection` with `char`/`charcategory`/`column`, `rangeset`, `facet`/`config`/`extension`, `state`/`transaction`. Inside each module, cm6 source order. Port all 172 cm6 cases and the derived tests; replay every fixture and run every generator before hand-off.

Hand-off as in `2-parity-implementation.md` §7, then `3-conformance-audit.md` with `ferrum/docs/parity/state/spec/` as the specification. After the conformance report is approved: rewrite `ferrum/crates/ferrum-state/README.md` and `ferrum/crates/ferrum-state/src/README.md`, and close in `ferrum/docs/parity/_cross-crate.md` every entry owned by `ferrum-state` that the crate now provides (citing item and test).
