# C4 behaviors

36 cm6 items (38 owned minus 2 DROP: state.cm6.043 `Facet.tag`, state.cm6.045 `FacetReader.tag`). Repro ids (Rn) refer to `C4-api.md` § Reproductions. Ferrum test ids from `t2-ferrum-tests.md`, cm6 ids from `t1-cm6-tests.md`. "test_state#…"/"test-state#…" rows are owned by C5 and cited here only as evidence.

Shared helpers read for every item below (cm6): `nextID` `state/src/facet.ts:4`; `sameArray` `:131-133`; `Slot` `:135`; `Provider` `:137`; `FacetProvider` + `dynamicSlot` `:139-194`; `compareArray` `:196-200`; `ensureAll` `:202-207`; `dynamicFacetSlot` `:209-258`; `initField` `:292`; `StateField.create`/`slot` `:317-351`; `PrecExtension` `:407-410`; `CompartmentInstance` `:440-443`; `DynamicSlot` `:445-449`; `Configuration` + `staticFacet` + `resolve` `:451-518`; `flatten` `:520-557`; `SlotStatus` `:559-564`; `ensureAddr` `:566-575`; `getAddr` `:577-579`; `EditorState` constructor `state/src/state.ts:42-61`; `field` `:66-76`; `applyTransaction` `:96-125`; `facet` `:194-199`; `toJSON` `:205-216`; `fromJSON` `:222-238`; `create` `:243-253`; `Compartment.reconfigure` effect type `:446`.
Ferrum code read: `crates/ferrum-state/src/facet.rs:1-278`, `crates/ferrum-state/src/config.rs:1-255`, `crates/ferrum-state/src/state.rs:105-162, 308-365`, `crates/ferrum-state/src/transaction.rs:48-63, 163-226`.

Order of evaluation established from the code (needed by many rows):
- cm6 `EditorState.create` / constructor: resolve configuration, then compute every dynamic slot through `ensureAddr` in slot order (fields first, then facets in ascending facet id with their providers before them, `facet.ts:483-512`); any read of a not-yet-computed slot computes it on demand (`ensureAddr` `:566-575`), and re-entering a slot marked `Computing` throws "Cyclic dependency" (`:570`).
- Ferrum `EditorState::create` (`state.rs:105-135`): resolve, then initialize **all fields** with `field_values` and `dynamic_values` both empty (`:121-125`), then compute **all dynamic facets** in first-appearance order with `dynamic_values` still empty (`:128-132`). Reading another field during field init returns `None` (`config.rs:198`); reading any dynamic facet during field init or dynamic facet computation indexes an empty vector and panics (`config.rs:175`). Static facets and (during facet computation) fields read fine.
- Ferrum `apply` (`state.rs:308-365`): only `CompartmentReconfigure` effects are considered; if any, `Configuration::resolve(&base, Some(&merged))` without old state; every field is updated (existing) or initialized from the **old** state and then updated (new); every dynamic facet is recomputed unconditionally with `dynamic_values` empty.

### state.cm6.035 `StateCommand` — cm6 `state/src/extension.ts:10`
Helpers read: none (type alias). Ferrum: `crates/ferrum-commands/src/commands.rs:20`, uses at `:28-44`, `:55`.
| id | behavior (branch/guard/edge case/side effect) | cm6 file:line | Ferrum code file:line or "none" | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateCommand.B1 | Receives `{state, dispatch}` and returns `boolean` (true = command applied) | `state/src/extension.ts:10` | `crates/ferrum-commands/src/commands.rs:20` returns `Option<TransactionSpec>` | none | none | differs: return type and no `dispatch` |
| state.StateCommand.B2 | May dispatch zero, one or several transactions via the callback | `state/src/extension.ts:10` | none (at most one spec returned) | none | none | missing |
| state.StateCommand.B3 | May return `true` without dispatching (handled, no change) | `state/src/extension.ts:10` | none (`Some(empty spec)` is the only approximation) | none | none | differs |
| state.StateCommand.B4 | Any function value, may capture environment | `state/src/extension.ts:10` | `fn` pointer type, no captures, `commands.rs:20` | none | none | differs |
| state.StateCommand.B5 | Defined in `@codemirror/state` | `state/src/extension.ts:10` | defined in `ferrum-commands` | none | none | differs: placement |
Proposed status: candidate `PLACEMENT DEVIATION` with signature deviation, because the type lives in `ferrum-commands` with a different contract; the signature is decision D5.

### state.cm6.036 `Facet` — cm6 `state/src/facet.ts:43`
Helpers read: constructor `:51-63`, `getAddr` `:577`, `EditorState.facet` `state/src/state.ts:194-199`.
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

### state.cm6.037 `Facet.reader` — cm6 `state/src/facet.ts:67`
Helpers read: `FacetReader` `:120-129`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.reader.B1 | Returns the facet itself typed as `FacetReader` (read-only handle; `reader === facet`) | `facet.ts:67` | none | none | none | missing (R36) |
| state.Facet.reader.B2 | Handle accepted by `state.facet` and as `compute` dependency | `facet.ts:135`, `state.ts:194` | none | none | none | missing (R37) |
Proposed status: `MISSING` (searched `reader`, `FacetReader` in all crates).

### state.cm6.038 `Facet^define` — cm6 `state/src/facet.ts:70`
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

### state.cm6.039 `Facet.of` — cm6 `state/src/facet.ts:79`
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

### state.cm6.040 `Facet.compute` — cm6 `state/src/facet.ts:90`
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

### state.cm6.041 `Facet.computeN` — cm6 `state/src/facet.ts:97`
Helpers read: same as state.cm6.040, plus `compareArray` `:196-200`, multi flattening `:223`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.computeN.B1 | Throws "Can't compute a static facet" for static facets | `facet.ts:98` | `facet.rs:119` (panic, other message) | none | none | implemented (different message), untested (R4b) |
| state.Facet.computeN.B2 | Values flattened into the facet inputs in provider order | `facet.ts:223` | `config.rs:138-140` | test-facet#12 | test_facet#012 | implemented+tested |
| state.Facet.computeN.B3 | Recompute only when deps change; new array kept only if `!compareArray(new, old, compareInput)` | `facet.ts:164-169,196-200` | none (always recomputed) | test-facet#12 | none | differs |
| state.Facet.computeN.B4 | After a doc change the new values appear | `facet.ts:164-168` | `state.rs:356-362` | test-facet#12 | none (test_facet#012 stops before the update) | implemented, untested (R20 both `100,10,1`) |
| state.Facet.computeN.B5 | Dependency list parameter | `facet.ts:97` | none | test-facet#12 | none | missing |
Proposed status: `PORTED, DEVIATION` because B5 is missing, B3 differs, B1 and B4 untested; also renamed to `compute_multi`.

### state.cm6.042 `Facet.from` — cm6 `state/src/facet.ts:106`
Helpers read: `compute` `:90-93`, `state.field` `state.ts:66-76`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Facet.from.B1 | Equals `compute([field], s => get(s.field(field)))` | `facet.ts:110` | none (manual `compute` reading the field works, R8) | none | none | missing (R26) |
| state.Facet.from.B2 | `get` omitted → identity | `facet.ts:109` | none | none | none | missing |
| state.Facet.from.B3 | Recomputed only when the field slot changed (`compareF`) | `facet.ts:164,333` | none | test-state#16 | none | missing |
| state.Facet.from.B4 | Throws on static facets (through `compute`) | `facet.ts:91,110` | none | none | none | missing (R4c) |
Proposed status: `MISSING`.

### state.cm6.044 `FacetReader` — cm6 `state/src/facet.ts:120`
Helpers read: `Slot` `:135`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetReader.B1 | Type that can be read via `state.facet` and used as a compute dependency, but cannot provide values | `facet.ts:116-129,135` | none | none | none | missing (R37) |
Proposed status: `MISSING`.

### state.cm6.046 `StateField` — cm6 `state/src/facet.ts:296`
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

### state.cm6.047 `StateField^define` — cm6 `state/src/facet.ts:311`
Helpers read: constructor `:300-308`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.define.B1 | `create` and `update` required | `facet.ts:262,266,312` | `facet.rs:176-186` | test-state#08 | test_state#008 | implemented+tested |
| state.StateField.define.B2 | `compare` defaults to `===` | `facet.ts:312` | none | none | none | missing (R24b) |
| state.StateField.define.B3 | `provide(field)` called once at definition | `facet.ts:313` | none | none | none | missing (R25) |
| state.StateField.define.B4 | `toJSON`/`fromJSON` kept in `spec` | `facet.ts:307,312` | none | test-state#10 | none | missing (R27) |
| state.StateField.define.B5 | Id from the counter shared with facets | `facet.ts:312` | `facet.rs:182` separate counter | none | none | differs (unobservable in cm6 except ordering; D6) |
Proposed status: `PORTED, DEVIATION` because B2–B4 are missing.

### state.cm6.048 `StateField.init` — cm6 `state/src/facet.ts:356`
Helpers read: `initField` `:292`, `StateField.create` `:317-320`, slot `reconfigure` `:337-349`, `fromJSON` `state.ts:226-231`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.init.B1 | Returns `[field, initField.of({field, create})]` | `facet.ts:357` | none | test-state#09 | none (test_state#009 uses a plain initializer) | missing (R23) |
| state.StateField.init.B2 | At create, the override is used instead of the spec's `create` | `facet.ts:318-319` | none | test-state#09 | none | missing |
| state.StateField.init.B3 | On reconfigure, a new/different override re-initializes the field | `facet.ts:338-342` | none | none | none | missing (R66) |
| state.StateField.init.B4 | Used by `EditorState.fromJSON` to restore fields | `state.ts:231` | none | test-state#10 | none | missing |
| state.StateField.init.B5 | `initField` is a static facet | `facet.ts:292` | none | none | none | missing |
Proposed status: `MISSING`.

### state.cm6.049 `StateField.extension` — cm6 `state/src/facet.ts:363`
Helpers read: `flatten` `:542-544`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.StateField.extension.B1 | Returns an extension enabling the field | `facet.ts:363` | `facet.rs:194-196` | test-facet#18,#19 | test_facet#018,#019; test_state#008 | implemented+tested |
| state.StateField.extension.B2 | Repeated inclusion → one field, created once | `facet.ts:524-526` | `config.rs:92-98` | none | none | implemented, untested (R41) |
Proposed status: `PORTED, DEVIATION` because B2 is untested.

### state.cm6.050 `Extension` — cm6 `state/src/facet.ts:374`
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

### state.cm6.051 `Prec` — cm6 `state/src/facet.ts:390`
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

### state.cm6.052 `Prec.highest` — cm6 `state/src/facet.ts:393`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.highest.B1 | Places the extension in bucket 0 (before everything else) | `facet.ts:376,393` | `facet.rs:24` + `config.rs:237-238` | test-facet#05,#06 | test_facet#005,#006 | implemented+tested (R29) |
Proposed status: `PORTED, RUST-IDIOMATIC` (`ext.with_prec(Prec::Highest)` instead of `Prec.highest(ext)`, identical ordering, tested).

### state.cm6.053 `Prec.high` — cm6 `state/src/facet.ts:396`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.high.B1 | Bucket 1 | `facet.ts:376,396` | `facet.rs:25` | test-facet#05 | test_facet#005 | implemented+tested |
Proposed status: `PORTED, RUST-IDIOMATIC`.

### state.cm6.054 `Prec.default` — cm6 `state/src/facet.ts:399`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.default.B1 | Bucket 2 | `facet.ts:376,399` | `facet.rs:26` | none | none | implemented, untested (oracle) |
| state.Prec.default.B2 | Explicit `default` inside a higher/lower parent resets the precedence | `facet.ts:541` | `config.rs:237-238` | none | none | implemented, untested (oracle `(p 2 …)` nodes nested in other `p` nodes) |
Proposed status: `PORTED, DEVIATION` (untested: B1, B2).

### state.cm6.055 `Prec.low` — cm6 `state/src/facet.ts:401`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.low.B1 | Bucket 3 | `facet.ts:376,401` | `facet.rs:27` | test-facet#05 | test_facet#005 | implemented+tested |
Proposed status: `PORTED, RUST-IDIOMATIC`.

### state.cm6.056 `Prec.lowest` — cm6 `state/src/facet.ts:404`
Helpers read: as state.cm6.051.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Prec.lowest.B1 | Bucket 4 (after everything else) | `facet.ts:376,404` | `facet.rs:28` | none | none | implemented, untested (R3, oracle) |
Proposed status: `PORTED, DEVIATION` (untested: B1).

### state.cm6.057 `Compartment` — cm6 `state/src/facet.ts:418`
Helpers read: `Configuration.compartments` `:455`, `flatten` `:534-539`, `applyTransaction` `state.ts:97-105`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Compartment.B1 | Each `new Compartment` is a distinct compartment (object identity) | `facet.ts:418` | `facet.rs:218-234` (`CompartmentId` counter; `Copy` values compare by id) | test-state#12 | test_state#012 | implemented+tested |
| state.Compartment.B2 | The state records, per present compartment, its current content; dropped compartments are forgotten | `facet.ts:455,472,538`, `state.ts:100-104` | `config.rs:26,40-43`, `state.rs:319-322` (stale entries kept) | test-state#14,#15 | test_state#014 (first half only), test_state#015 (no reconfigure) | differs (R14, R14b) |
| state.Compartment.B3 | `get(state)` returns current content | `facet.ts:431-433` | none | test-state#12,#15 | none | missing |
Proposed status: `PORTED, DEVIATION` because B2 differs and B3 is missing.

### state.cm6.058 `Compartment.of` — cm6 `state/src/facet.ts:421`
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

### state.cm6.059 `Compartment.reconfigure` — cm6 `state/src/facet.ts:425`
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

### state.cm6.060 `Compartment.get` — cm6 `state/src/facet.ts:431`
Helpers read: `Configuration.compartments` `:455`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.Compartment.get.B1 | Returns the current content (after reconfigures) | `facet.ts:432` | none | test-state#12 | none | missing (R15) |
| state.Compartment.get.B2 | Returns `undefined` when the compartment is not in the state | `facet.ts:432` | none | test-state#15 | none | missing (R15) |
Proposed status: `MISSING`.

### state.cm6.134 `combineConfig` — cm6 `state/src/config.ts:8`
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

### state.cm6.248 `Facet.define config.combine` — cm6 `state/src/facet.ts:12`
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

### state.cm6.249 `Facet.define config.compare` — cm6 `state/src/facet.ts:17`
Helpers read: `:73`, `:131-133`, `:238`, `:245-252`, `:498`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.compare.B1 | Custom output comparison keeps the old output when equal (transaction, reconfigure, static recombine) | `facet.ts:238,250,498` | none | test-facet#16 | none | missing (R32, R34) |
| state.FacetConfig.compare.B2 | Default: `===` with combine, `sameArray` without | `facet.ts:73` | none | test-facet#08,#17 | none | missing (R10, R11) |
Proposed status: `MISSING`.

### state.cm6.250 `Facet.define config.compareInput` — cm6 `state/src/facet.ts:20`
Helpers read: `:72`, `:149`, `:166`, `:180`, `:196-200`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.compareInput.B1 | Compares a single provider's new and old input; equal → slot unchanged | `facet.ts:149,166` | none | none | none | missing (R33) |
| state.FacetConfig.compareInput.B2 | Multi providers compared element-wise via `compareArray` | `facet.ts:166,196-200` | none | none | none | missing |
| state.FacetConfig.compareInput.B3 | Default `===` | `facet.ts:72` | none | test-facet#10 (count stays 1) | none | missing |
Proposed status: `MISSING`.

### state.cm6.251 `Facet.define config.static` — cm6 `state/src/facet.ts:22`
Helpers read: `:58`, `:74`, `:91`, `:98`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.static.B1 | `compute`/`computeN`/`from` throw "Can't compute a static facet" | `facet.ts:91,98` | `facet.rs:107,119` (panic "Cannot attach dynamic provider to static facet") | none | none | implemented (different message), untested (R4, R4b) |
| state.FacetConfig.static.B2 | Static facets only ever get static providers → value stored in `staticValues` | `facet.ts:492-499` | `config.rs:111-122` | test-facet#13 | test_facet#013 | implemented+tested |
| state.FacetConfig.static.B3 | (Ferrum-only) check bypassed through public `Extension::FacetDynamic`; `AnyFacet::is_static` never consulted | n/a | `facet.rs:38,248`, `config.rs` (no call) | n/a | none | Ferrum-only gap (R46b `9`) |
Proposed status: `PORTED, DEVIATION` because B1 is untested with a different message and B3 lets the invariant be broken.

### state.cm6.252 `Facet.define config.enables` — cm6 `state/src/facet.ts:29`
Helpers read: `:49`, `:62`, `:547`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FacetConfig.enables.B1 | Extension, or function of the facet, evaluated at definition | `facet.ts:62` | none | none | none | missing |
| state.FacetConfig.enables.B2 | Added when the facet has at least one provider, at default precedence, deduplicated | `facet.ts:547,524` | none | none | none | missing (R62 `h,a,e`, R63 `e`) |
| state.FacetConfig.enables.B3 | Not added when the facet is absent (default read) | `facet.ts:547` | none | none | none | missing (R21) |
Proposed status: `MISSING`.

### state.cm6.253 `StateField.define config.create` — cm6 `state/src/facet.ts:262`
Helpers read: `StateField.create` `:317-320`, slot `create` `:326-329`, slot `reconfigure` `:337-349`, `ensureAddr` `:566-575`, `state.ts:117-118,243-253`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.create.B1 | Called once when the state is created | `facet.ts:326-328` | `state.rs:121-125` | test-state#08 | test_state#008 | implemented+tested |
| state.FieldConfig.create.B2 | Receives the state under construction; can read other fields and any facet (computed on demand) | `facet.ts:327`, `state.ts:66-76,194-199` | other fields → `None` (`config.rs:198`); dynamic facets → panic (`config.rs:175`); static facets OK | test-state#08; test-facet#19 | test_state#008 (passes only because field1's init is 0 and the test uses `unwrap_or(0)`) | differs (R6 panic, R7/R7b `None` vs `15`, R6b OK) |
| state.FieldConfig.create.B3 | Replaced by a `StateField.init` override when present | `facet.ts:318-319` | none | test-state#09 | none | missing |
| state.FieldConfig.create.B4 | Fields new after reconfiguration are created from a state with the new configuration (old doc/selection) | `state.ts:117-118`, `facet.ts:347` | `state.rs:338` (`field.init(self)`, old configuration) | test-facet#19 | none (test_facet#019 does not reconfigure) | differs (R17) |
| state.FieldConfig.create.B5 | A field reading itself throws the cyclic error | `facet.ts:570` | none | none | none | differs (R19b `Some(0)`) |
Proposed status: `PORTED, DEVIATION` because B2, B4, B5 differ and B3 is missing.

### state.cm6.254 `StateField.define config.update` — cm6 `state/src/facet.ts:266`
Helpers read: slot `update` `:330-336`, `applyTransaction` `state.ts:117-124`, `Transaction.state` `transaction.ts` (C5).
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.update.B1 | Called on every transaction with the old value and the transaction | `facet.ts:331-332` | `state.rs:332-336` | test-state#08,#09 | test_state#008,#009, test_facet#019 | implemented+tested |
| state.FieldConfig.update.B2 | If `compare(old, new)` the old value is kept and dependents see no change | `facet.ts:333` | none (always a new `Arc`, `facet.rs:212`) | none | none | missing (R24, R58) |
| state.FieldConfig.update.B3 | Fields added by a reconfiguration are created, then updated by the same transaction | `state.ts:117-124` | `state.rs:337-340` | test-facet#18 | test_facet#018 (value only) | implemented+tested (R18 event order matches) |
| state.FieldConfig.update.B4 | `update` may read `tr.state` (the state being built; slots computed on demand) | `state.ts:57-59` | `transaction.rs:176-178` re-runs `apply` → infinite recursion | test-state#16 (`tr.state.doc`) | none | differs: stack overflow (bin `trstate`) |
| state.FieldConfig.update.B5 | Updated values of other fields are visible through `tr.state.field` | `state.ts:66-76` | none (same root as B4) | none | none | differs |
Proposed status: `PORTED, DEVIATION` because B2 is missing and B4, B5 differ.

### state.cm6.255 `StateField.define config.compare` — cm6 `state/src/facet.ts:272`
Helpers read: `:305`, `:312`, `:333`, dependent `compute` `:164`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.compare.B1 | Equal → old value kept, slot not marked changed, dependent facets not recomputed | `facet.ts:333,164` | none | none | none | missing (R24) |
| state.FieldConfig.compare.B2 | Default `===` | `facet.ts:312` | none | test-state#16 (identity of facet computed from field) | none | missing (R24b) |
Proposed status: `MISSING`.

### state.cm6.256 `StateField.define config.provide` — cm6 `state/src/facet.ts:280`
Helpers read: `:298`, `:313`, `:544`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.provide.B1 | Called once with the field at definition | `facet.ts:313` | none | none | none | missing |
| state.FieldConfig.provide.B2 | Provided extensions included wherever the field is, at the field's precedence | `facet.ts:544` | none | none | none | missing (R61 `3,1`) |
| state.FieldConfig.provide.B3 | Typical use `facet.from(field)` | `facet.ts:280` doc | none | none | none | missing (R25 `3`) |
Proposed status: `MISSING`.

### state.cm6.257 `StateField.define config.toJSON` — cm6 `state/src/facet.ts:285`
Helpers read: `EditorState.toJSON` `state.ts:205-216`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.toJSON.B1 | `toJSON(value, state)` called by `state.toJSON(fields)` for fields present in the state | `state.ts:211-213` | none | test-state#10 | none | missing (R27) |
Proposed status: `MISSING`.

### state.cm6.258 `StateField.define config.fromJSON` — cm6 `state/src/facet.ts:289`
Helpers read: `EditorState.fromJSON` `state.ts:222-238`, `StateField.init` `facet.ts:356-358`.
| id | behavior | cm6 file:line | Ferrum code | cm6 test(s) | Ferrum test(s) | verdict |
|---|---|---|---|---|---|---|
| state.FieldConfig.fromJSON.B1 | `fromJSON(json, state)` called through a `field.init` override when the JSON has the property | `state.ts:226-231` | none | test-state#10 | none | missing (R27 `200`) |
Proposed status: `MISSING`.

## Unfinished

none
