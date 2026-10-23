# ferrum-state placement deviations

> What: items placed in a different crate than cm6. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`.

Items that exist, but in a different crate than cm6 (or the disposition) puts them (brief section 4, `PLACEMENT DEVIATION`; rules.md §2.3). Cross-crate entries: `docs/parity/_cross-crate.md`.

| id | item | where it is in Ferrum | where cm6 has it | owning Ferrum crate (dependency graph) | cross-crate entry | evidence |
|---|---|---|---|---|---|---|
| state.cm6.035 | `StateCommand` | `crates/ferrum-commands/src/commands.rs:20` (`fn(&EditorState) -> Option<TransactionSpec>`) | `@codemirror/state`, `mirrors/cm6/state/src/extension.ts:10` (`(target:{state,dispatch}) => boolean`) | ferrum-state (signature: D5) | XC-001 | `docs/parity/state/raw/C4-api.md` |
| state.fe.284 | `transaction::IsolateHistory` | `crates/ferrum-state/src/transaction.rs:143` | `@codemirror/commands`, `mirrors/cm6/commands/src/history.ts:14` (`isolateHistory` annotation, values "before"/"after"/"full") | ferrum-commands | XC-002 | `docs/parity/state/raw/C5-api.md` |
| state.fe.285 | `IsolateHistory::Before` | `crates/ferrum-state/src/transaction.rs:144` | `@codemirror/commands`, `mirrors/cm6/commands/src/history.ts:14` (`isolateHistory` annotation, values "before"/"after"/"full") | ferrum-commands | XC-002 | `docs/parity/state/raw/C5-api.md` |
| state.fe.286 | `IsolateHistory::After` | `crates/ferrum-state/src/transaction.rs:145` | `@codemirror/commands`, `mirrors/cm6/commands/src/history.ts:14` (`isolateHistory` annotation, values "before"/"after"/"full") | ferrum-commands | XC-002 | `docs/parity/state/raw/C5-api.md` |
| state.fe.287 | `IsolateHistory::Full` | `crates/ferrum-state/src/transaction.rs:146` | `@codemirror/commands`, `mirrors/cm6/commands/src/history.ts:14` (`isolateHistory` annotation, values "before"/"after"/"full") | ferrum-commands | XC-002 | `docs/parity/state/raw/C5-api.md` |
| state.fe.055 | `char::find_word_break` | `crates/ferrum-state/src/char.rs:70` | `@codemirror/view` group motion: `mirrors/cm6/view/src/cursor.ts:15-36` (`groupAt`), `:102-109` (`byGroup`), `mirrors/cm6/view/src/editorview.ts:694-695` (`moveByGroup`) | ferrum-view | XC-003 | `docs/parity/state/raw/C3-api.md` |
| state.fe.056 | `char::find_subword_break` | `crates/ferrum-state/src/char.rs:101` | `@codemirror/commands` subword motion: `mirrors/cm6/commands/src/commands.ts:115-161` (`moveBySubword`, `Intl.Segmenter` for CJK) | ferrum-commands | XC-003 | `docs/parity/state/raw/C3-api.md` |

Re-exports carrying these items: `ferrum_state::IsolateHistory` (state.fe.345), `ferrum_state::find_word_break` (state.fe.315), `ferrum_state::find_subword_break` (state.fe.314). The facade also re-exports ferrum-commands' `StateCommand` (`crates/ferrum/src/lib.rs:25`).

Behavior also differs for the motion functions (`docs/parity/state/raw/C3-api.md` R8: `find_word_break("foo bar",3,true)` → 4 where cm6 `byGroup` continues into the next word; `find_subword_break` is per `char`, has no CJK segmenter and panics for `pos > len` backward), so moving them is not enough to reach parity.
