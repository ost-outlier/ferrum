# C4 notes

## Decision evidence

### D1 (position unit)
- C4 APIs take no positions themselves. Positions enter through getter closures of `Facet.compute`/`computeN` and field `create`/`update`, which typically read `state.doc.length` (cm6, UTF-16) vs `doc().len_chars()` (Ferrum, code points).
- Affected tests: test-facet#10 (indirectly, doc changes), #12 (`s.doc.length % 2`), #14 (`s.doc.length` summed), #15, #16 (`s.doc.length`). All cm6 inputs are ASCII (`"hello"`), so expected values are identical under all three units.
- Concrete divergence (R-D1): test-facet#12's provider after inserting `"😀"` (U+1F600) instead of `"hello"`: cm6 `doc.length = 2` → facet `1`; Ferrum `len_chars = 1` → `[100, 10, 1]`. Under UTF-8 bytes the length would be 4 → `[1]` (same parity as cm6 here by accident); under code points 1 (current); under UTF-16 2 (cm6).
- Items affected: state.cm6.040, .041, .253, .254 (only through user getters). Files: `state/test/test-facet.ts:89,103,110,120`, `crates/ferrum-state/src/tests/test_facet.rs:116,127,162,180,271`.

### D5 (command type)
- cm6 `StateCommand = (target: {state, dispatch: (tr) => void}) => boolean` at `state/src/extension.ts:10`, i.e. in `@codemirror/state`.
- Ferrum `pub type StateCommand = fn(&EditorState) -> Option<TransactionSpec>` at `crates/ferrum-commands/src/commands.rs:20`, used in `KeyBinding` (`:28`, `:30`, `:35`, `:44`) and `dispatch_key` (`:55`), re-exported by `crates/ferrum/src/lib.rs:25`. No `StateCommand` in `ferrum-state`.
- Options: (a) cm6 shape in `ferrum-state` (closure taking state + dispatch callback, returning `bool`): supports multi-dispatch and "handled without change"; (b) current shape: pure, easy to test, but loses B2/B3 of state.StateCommand and cannot capture (`fn` pointer); (c) closure type returning `Option<TransactionSpec>` placed in `ferrum-state`: fixes placement and captures, still single-dispatch.

### D6 (ownership, threading, determinism)
- Ids come from three process-global `AtomicUsize` counters with `Ordering::Relaxed` (`crates/ferrum-state/src/facet.rs:5-7`, `:70`, `:182`, `:232`). cm6 uses one module counter `nextID` shared by facets, providers and fields (`state/src/facet.ts:4,45,140,312`).
- Ids depend on process-wide creation order: R51 printed `num=FacetId(1) a=FacetId(13) b=FacetId(14) tab_size(lazy)=FacetId(15) c=FacetId(16)`; built-in facets in `LazyLock` (`crates/ferrum-state/src/state.rs:13-27`) get their id at first access, so their id differs between programs and between test orders. With `cargo test` running tests on several threads, ids interleave non-deterministically.
- Current Ferrum outputs do not depend on id values: provider/facet evaluation order is first-appearance order (`crates/ferrum-state/src/config.rs:61,107`), and `HashMap`s keyed by id are only used for lookup. cm6, however, evaluates facet slots in ascending id order (`state/src/facet.ts:489`, `for…in` over integer keys; R28 `a,b` vs Ferrum `b,a`). If Ferrum adopted cm6's id-ordered evaluation with global atomic ids, side-effect order of getters would become dependent on thread scheduling of definitions.
- Every facet input/output, field value and closure must be `Send + Sync + Clone + 'static` (`facet.rs:36,44,53-60,168-172`); values are stored as `Arc<dyn Any + Send + Sync>`; `combine_erased` clones every input into a fresh `Vec<Input>` on each call (`facet.rs:149-156`).
- `Arc` and identity: cm6 upper packages detect changes by identity (`!=`) of facet/field values (10 sites, e.g. `view/src/editorview.ts:329`, `view/src/draw-selection.ts:69`, `view/src/gutter.ts:502`, `language/src/language.ts:557`, `language/src/fold.ts:411-412`). In Ferrum the observable equivalent is pointer identity of the `&Output` returned by `EditorState::facet`; it is preserved only for static facets across non-reconfiguring transactions (R10d) and for defaults (R40), never for dynamic facets (R10, R10b, R10c) nor after any compartment reconfigure (R11, R11b, R57). Field values are always re-wrapped in a new `Arc` (R58).

### D2
- No C4 item depends on line-break semantics.

## Cross-crate facts

| capability | cm6 evidence | searched in Ferrum | what exists |
|---|---|---|---|
| `combineConfig` (state) used by view/language/commands | `view/src/gutter.ts:462`, `view/src/draw-selection.ts:24`, `view/src/special-chars.ts:68`, `language/src/matchbrackets.ts:37`, `language/src/fold.ts:311`, `commands/src/history.ts:38` | `combine_config`, `combineconfig`, `merge conflict`, `fn combine` in `/workspace/ferrum/crates/*` | absent; `crates/ferrum-commands/src/history.rs:104-125` hand-rolls `HISTORY_CONFIG` combine (max/min/or) |
| `StateCommand` (state) needed by commands | `state/src/extension.ts:10`, `commands/src/history.ts:1` imports it from state | `StateCommand` in all crates | only `crates/ferrum-commands/src/commands.rs:20` (different signature) |
| `Facet.from` / field `provide` used by view/language | `language/src/fold.ts:128` (`provide: f => EditorView.decorations.from(f)`), `view/src/tooltip.ts:857`, `view/src/dialog.ts:86` (`showPanel.computeN([f], …)`), 6 `provide:` sites total | `provide`, `from(` on facets | absent in ferrum-state; `crates/ferrum-language/src/fold.rs:49` defines a `StateField` without any provide path; `crates/ferrum-view/src/decoration.rs:235` `DECORATIONS` is a plain list facet |
| `enables` used by view | `view/src/panel.ts:201`, `view/src/tooltip.ts:537` (4 sites) | `enables` | absent |
| `FacetReader` used by view | `view/src/tooltip.ts:31` | `FacetReader`, `reader` | absent |
| `StateField.init` used by view | `view/src/dialog.ts:53` (`appendConfig.of(dialogField.init(…))`) | `init_field`, `with_init` | absent |
| computed facets with `"selection"`/facet deps used by view/language | `view/src/gutter.ts:487,533`, `language/src/highlight.ts:112` (`computeN([EditorView.darkTheme], …)`) | `.compute(`, `compute_multi` in other crates | no `compute` call outside ferrum-state; a computed facet reading another computed facet panics (R5), so these cm6 patterns cannot be expressed today |
| identity-based change detection of facet/field values | 10 sites (see D6) | `ptr_eq`, `ptr::eq` | `crates/ferrum-view/src/decoration.rs:82,92`, `crates/ferrum-view/src/heightmap.rs:115` use `Arc::ptr_eq` on their own data, not on facet values |
| `StateEffect.reconfigure` / `appendConfig` (C5) needed by configuration tests | test-facet#15–#19, #21, #22; `state/src/state.ts:106-111` | `append_config`, `AppendConfig`, `Reconfigure` | only `CompartmentReconfigure` (`crates/ferrum-state/src/transaction.rs:50`); ferrum-state cannot reconfigure the root extensions at all |

## Oracle

Differential check of extension flattening (precedence buckets, nesting, identity dedup) — pure state.
- Files: generator `/tmp/claude-1000/-workspace/8fb87903-f955-477c-a67c-0747517f569e/scratchpad/cm6/probe-C4/oracle-gen.ts` (writes `<s-expression>\t<cm6 result>` lines), consumer `/tmp/claude-1000/-workspace/8fb87903-f955-477c-a67c-0747517f569e/scratchpad/repro-C4/src/bin/oracle.rs`, data `/tmp/claude-1000/-workspace/8fb87903-f955-477c-a67c-0747517f569e/scratchpad/oracle-C4/{noshare,share}.tsv`.
- PRNG: mulberry32. Trees: depth ≤ 4; leaf `vN` = `num.of(N)` (45 %), `(p P x)` = precedence P (25 %), `(g …)` = array of 1–4 children (30 %). With `share`, 30 % of nodes are registered `(d K x)` and 15 % of positions reuse a registered node `(r K)` — the same JS object in cm6, an `Extension` clone in Ferrum.
- Runs: `tsx probe-C4/oracle-gen.ts 196 2000 noshare …` and `… 197 2000 share …`; `./target/debug/oracle <file>`.
- Results: no sharing, seed 196, 2000 cases: **2000 match, 0 mismatch** (precedence ordering, nesting, inner override, `Prec.default`/`lowest` all agree). Sharing, seed 197, 2000 cases: **1657 match, 343 mismatch**. Shortest mismatch: `(g (d 0 v0) (r 0))` → cm6 `0`, Ferrum `0,0` (identity dedup missing, state.Extension.B5).
- Non-ASCII sample: not applicable (no positions in this oracle); D1 effect shown separately in R-D1.
- Not covered by the oracle (would need dynamic slots and transactions): dynamic recomputation and identity — covered by targeted repros R5–R12, R19, R28.

## Open questions

1. `Configuration`, `AnyFacet`, `AnyStateField`, the type-erased `Extension` variants, `FacetId`/`StateFieldId`/`CompartmentId`, `Facet::id`, `Facet::default_value`, `Compartment.id` are public while their cm6 counterparts are internal or absent. No verifiable reason found; candidate `HUMAN DECISION` (keep public, make `pub(crate)`, or seal the traits). Open construction already allows invariant breaks (R45, R46, R46b, R50).
2. `AnyFacet::is_static` and `AnyFacet::default_erased` are never called in the workspace (dead trait methods).
3. Ferrum's evaluation model (eager vectors filled in order, no on-demand computation) is the root of R5, R6, R7, R19, R39 and of the absence of cycle detection; cm6's model is on-demand slots with status flags (`state/src/facet.ts:559-579`). Recorded as a fact for the repair/rebuild decision; no fix proposed.
4. `Transaction::state()` re-applies the transaction on every call (`crates/ferrum-state/src/transaction.rs:176-178`); calling it from a field `update` recurses until stack overflow (bin `trstate`). Owned by C5 (`Transaction.state`), recorded here because it breaks state.FieldConfig.update.B4.
5. Observed while reading (C5-owned rows, not classified here): Ferrum `READ_ONLY` combine is `any` (`crates/ferrum-state/src/state.rs:21-23`) while cm6 `readOnly` takes `values[0]` (`state/src/extension.ts:29-31`), so `[readOnly.of(false), readOnly.of(true)]` would give `false` in cm6 and `true` in Ferrum (not run; C5 should reproduce). `TAB_SIZE` is a non-static `Facet::define` in both.
6. test_facet#011 sets a selection outside the document and passes (R-sel); cm6 throws `Selection points outside of document`. Selection bounds checking belongs to C3/C5.
7. The `ist` default comparator (`==`) could not be verified locally (package not installed); verification pass should confirm from the `ist` sources.

## Unfinished

none
