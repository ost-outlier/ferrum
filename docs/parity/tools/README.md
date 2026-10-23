# Audit tools

> What: the shared tools of the parity audits and how to run them. Versions: Node 22, TypeScript 5.6.3 (installed in the harness), Python 3. Ids: none.

All commands run from the repository root, and paths in this file are relative to it. The tools find the root from their own location. Source mirrors: see the section Mirrors below.

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

## Mirrors

Read-only copies of outside sources live in `mirrors/` (gitignored). What is fetched, from where and at which commit is versioned in `docs/parity/tools/mirrors.json`; `docs/parity/tools/fetch_mirrors.py` does the work. A fetched tree has no `.git` (no hooks can run from it) and records its commit in `.mirror-commit`.

| Task | Command |
|---|---|
| Fetch everything pinned (fresh checkout, or after a manifest change) | `python3 docs/parity/tools/fetch_mirrors.py` |
| See what is missing, stale or unpinned | `python3 docs/parity/tools/fetch_mirrors.py --check` |
| Pin a mirror to the newest commit of its `ref`, then fetch it | `python3 docs/parity/tools/fetch_mirrors.py --pin zed` |
| Pin to a given tag or branch | `python3 docs/parity/tools/fetch_mirrors.py --pin cm6/view refs/tags/6.43.15` |
| Add a package to a set (for example a new cm6 package for a feature audit) | `python3 docs/parity/tools/fetch_mirrors.py --pin cm6/search refs/tags/<version>` |

Pinning writes the commit into the manifest; commit the manifest in the same commit as the documents that cite the new version, so that a document's Ferrum commit always fixes the sources it cites (`rules.md` §8.3).

**Adding a new mirror.** Add an entry under `mirrors` in `docs/parity/tools/mirrors.json`, with `"commit": null`, then pin it:

- one repository: `{"kind": "repo", "url": "<git url>", "ref": "refs/heads/main", "commit": null, "license": ["MIT"]}`; documents cite it as `mirrors/<name>/<path>`;
- a set of repositories under one URL pattern (as cm6, one repository per package): `{"kind": "set", "url": "https://host/org/{member}", "license": [...], "members": {"<member>": {"ref": "...", "commit": null}}}`; cited as `mirrors/<name>/<member>/<path>`;
- `"include": ["path", ...]` fetches only those paths (sparse checkout, plus the repository's own `LICENSE*` files);
- `"license_check": "cargo"` makes the fetch refuse any included path whose `Cargo.toml` `license` is not in `"license"`. Zed uses both: only its Apache-2.0 crates are ever on disk (`rules.md` §5.3).

Check the license of a new source before adding it (`rules.md` §5.1); record it in `"license"`. If a repository's server refuses to fetch a commit by id, the script fetches the `ref` and checks the commit out from it.
