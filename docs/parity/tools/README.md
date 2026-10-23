# Audit tools

> What: the shared tools of the parity audits and how to run them. Versions: Node 22, TypeScript 5.6.3 (installed in the harness), Python 3. Ids: none.

All commands run from the repository root, and paths in this file are relative to it. The tools find the root from their own location. Mirrors: `python3 docs/parity/tools/fetch_mirrors.py` fetches every package pinned in `docs/parity/tools/mirrors.json` into `mirrors/cm6/` (gitignored); `--check` reports missing or stale ones. A new audit adds its packages to the manifest first, with the commit it reads.

| Tool | Phase | Command | Output |
|---|---|---|---|
| `docs/parity/tools/harness/setup_cm6.py` | P0, once per audit, main session | `python3 docs/parity/tools/harness/setup_cm6.py <package> [--npm <pkg@ver> …]` | the cm6 harness in `.harness/<package>/` (sources of the package and of its `@codemirror/*` dependencies, npm dependencies, `tsconfig.json`, `harness.json` with versions) |
| `docs/parity/tools/harness/oracle-lib.ts` | P2, every probe and generator | `import {rng, capture, encode, fixtureWriter} from "@harness/lib"` | seeded PRNG, exception capture, Rust-safe string encoding, fixture writer |
| `docs/parity/tools/cm6_inventory.cjs` | P0 | `node docs/parity/tools/cm6_inventory.cjs --harness .harness/<package> --package <package> --prefix <crate>.cm6 > docs/parity/<crate>/raw/inventory.jsonl` | A1 rows (config-object fields detected automatically) |
| `docs/parity/tools/cm6_tests.cjs` | P0 | `node docs/parity/tools/cm6_tests.cjs --harness .harness/<package> --package <package> > docs/parity/<crate>/raw/tests.jsonl` | T1 rows; `generated: true` marks cases created in loops (expand by hand) |
| `docs/parity/tools/render.py` | any | `python3 docs/parity/tools/render.py <rows.jsonl> --cols id,item,… --title … --header … > <file>.md` | Markdown table from JSON Lines |
| `docs/parity/tools/ferrum_pub.py` | conformance G1 | `python3 docs/parity/tools/ferrum_pub.py <rustdoc json> crates/<crate> > pub.jsonl` (rustdoc JSON: `RUSTC_BOOTSTRAP=1 cargo rustdoc -p <crate> -- -Z unstable-options --output-format json`) | public items of a Ferrum crate |
| `docs/parity/tools/check_refs.py` | end of each phase, main session | `python3 docs/parity/tools/check_refs.py docs/parity docs/fundation --quiet` | count of unresolvable references: scratch paths, unstated roots, missing files, lines past the end, unknown evidence ids, sections that do not exist (`--quiet` prints one line) |

Run probes with `tsx --tsconfig .harness/<package>/tsconfig.json <probe>.ts`. Probes import cm6 by package name (`@codemirror/state`) and the helpers as `@harness/lib`; they live in `docs/parity/<crate>/oracle/cm6/<chunk>/`.
