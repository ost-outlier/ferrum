import collections
exec(open('rows1.py').read()); exec(open('rows2.py').read()); exec(open('rows3.py').read())
SP="/tmp/claude-1000/-workspace/8fb87903-f955-477c-a67c-0747517f569e/scratchpad"
cnt=collections.Counter(v for *_,v in rows)
per=collections.OrderedDict()
for c,s,e,v in rows:
    key=s.split(":")[0] if not s.startswith("git") else "git log (ferrum)"
    per.setdefault(key,collections.Counter())[v]+=1
def esc(x): return x.replace("|","\\|").replace("\n"," ")
out=[]
out.append("# A4 claims (ferrum-state)\n")
out.append("Totals: %d claims (true %d, false %d, unverifiable %d); per source file counts:\n"%(len(rows),cnt["true"],cnt["false"],cnt["unverifiable"]))
for k,c in per.items():
    out.append("- `%s`: %d (true %d, false %d, unverifiable %d)"%(k,sum(c.values()),c["true"],c["false"],c["unverifiable"]))
out.append("""
Conventions. Source paths are relative to `/workspace/`. Evidence paths: Ferrum relative to `/workspace/ferrum/`, cm6 relative to `/workspace/CodeMirror6/`. Quotes from `ferrum_strategic_architecture_vision.md` (Portuguese) are translated to English and marked "(translated)". "≡" means the Ferrum code was compared line by line with the cited cm6 code and does the same thing. A row marked `true` for a doc comment means the comment correctly describes the Ferrum code; parity with cm6 is judged only where the text claims it ("matches CM6", "1:1", "canonical", "parity"), and such a claim is `false` as soon as one counterexample exists. Complexity claims are judged by code inspection only: no benchmark exists in the repository (no `benches/`, no criterion dependency), and no complexity was measured. Test names (`test_cm6_*`) are not used as evidence of behavior.

Scope notes. Root README quickstart 3 (`ferrum/README.md:179-197`, decorations/viewport) uses no state API and belongs to the view audit; the root README total test counts other than the ferrum-state line, and the commits that touch only view/commands/gpui, are out of scope. The facade test `test_ferrum_facade_integration` is a test and was not extracted.

| id | claim (quoted, shortened only with […]) | source file:line | evidence (code file:line, cm6 file:line, repro) | verdict |
|---|---|---|---|---|""")
for i,(c,s,e,v) in enumerate(rows,1):
    out.append("| state.claim.%03d | %s | `%s` | %s | %s |"%(i,esc(c.replace("[...]","[…]")),s,esc(e),v))
out.append(r"""
## Reproductions

All Ferrum reproductions live in the scratch crate `%(SP)s/repro-A4/` (copied from `repro-template`; `Cargo.toml` depends on `ferrum-state` and, for R3, the `ferrum` facade, both by path). Run with `cargo run -q --offline --bin <name>`. cm6 scripts live in `%(SP)s/cm6/probe-A4/` and run with `cd %(SP)s/cm6 && npx tsx probe-A4/<file>.ts`. Workspace test runs used an external `CARGO_TARGET_DIR` and `--locked --offline`, so nothing under `/workspace/ferrum` was written.

### R1 Crate README quickstart (`src/bin/crate_readme_quickstart.rs`)
Verbatim copy of `ferrum/crates/ferrum-state/README.md:57-80` inside `fn main`. Ferrum: compiles without warnings, both assertions pass, prints `quickstart OK; doc len = 13, main head = 14`. cm6 equivalent (`probe-A4/probes.ts` P0): `EditorState.create({doc: "Hello, world!", selection: EditorSelection.cursor(5)}).update({changes: {from: 5, to: 12, insert: " Ferrum"}, selection: EditorSelection.cursor(14)})` → `RangeError: Selection points outside of document`.

### R2 src/README signature check (`src/bin/src_readme_api.rs`)
One function per symbol listed in `ferrum/crates/ferrum-state/src/README.md`, written with the signature the README gives. `cargo build --bin src_readme_api` reports exactly these errors (everything else compiles):
- line for README:13 `state.field(field) -> &u8`: E0308 mismatched types (returns `Option<&Value>`).
- README:56 `desc.map_pos(pos, assoc)`: E0061 "this method takes 3 arguments but 2 arguments were supplied" + E0308 (returns `Option<usize>`).
- README:57 `desc.compose(other)`: E0599 no method named `compose` for `&ChangeDesc`.
- README:58 `desc.map(other, before)`: E0599 (`&ChangeDesc` is not an iterator; no inherent `map`).
- README:80 `rs.between(from, to, |a, b, v| {})`: E0308 (callback must return `bool`).

### R3 Root README quickstarts (`src/bin/root_readme_ex1.rs`, `src/bin/root_readme_ex2.rs`)
Example 1 (`ferrum/README.md:116-151`) verbatim: compiles with 2 warnings (unused imports `TransactionSpec`, `redo`); output: the document with `// Edited with Ferrum` prepended, then the original document after undo. Example 2 (`ferrum/README.md:155-175`) is a fragment that needs `view`; with the state/view setup of example 1 prepended it compiles and prints doc `"    // [TAG] fn main() {\n    println!    // [TAG] (\"Hello, Ferrum!\");\n}\n"` and cursors `[(13, 13), (50, 50)]`.

### R4 Behavior probes (Ferrum `src/bin/probes.rs`, `src/bin/probes2.rs`; cm6 `probe-A4/probes.ts`, `probe-A4/probes2.ts`)
| probe | input | cm6 | ferrum |
|---|---|---|---|
| P0 | crate README quickstart (R1) | `RangeError: Selection points outside of document` | accepted, head 14 in 13-char doc |
| P1 | doc `"a\u2028b"` | length 3, 1 line, line1.text `"a\u2028b"`, to 3 | length 3, 2 lines, line1.text `"a\u{2028}"`, to 2 |
| P1 | doc `"a\u0085b"` | 1 line | 2 lines, line1.text `"a\u{85}"`, to 2 |
| P1 | doc `"a\u000bb"` (VT) | 1 line | 2 lines, line1.to 2 |
| P1 | doc `"a\u000cb"` (FF) | 1 line | 2 lines, line1.to 2 |
| P1 | doc `"a\rb"` | 2 lines, line1 `"a"`, length 3 | 2 lines, line1 `"a"`, length 3 |
| P1 | doc `"a\r\nb"` | length 3, 2 lines | length 4, 2 lines |
| P1 | doc `"a\nb"` | length 3, 2 lines | length 3, 2 lines |
| P2 | doc `"😀a"` length | 3 | 2 |
| P3 | `line(0)` | `RangeError: Invalid line number 0 in 2-line document` | panic `Invalid line: 0` (`line(1)` is the first line) |
| P4 | `findClusterBreak(s, 0)` / `find_cluster_break(s, 0, true)`; cm6 in UTF-16 units, Ferrum in chars | `"\r\n"` 1; `"e\u0301x"` 2; family+`x` 11 (=7 chars); `🇩🇪🇫🇷` 4 (=2 chars); `👍🏽x` 4 (=2 chars); `"\u1100\u1161\u11a8x"` 1; `"क्षa"` 2; `"\u0600a"` 1; `"a\u200db"` 3 | `"\r\n"` 2; 2; 7; 2; 2; Hangul 3; `"क्षa"` 3; `"\u0600a"` 2; `"a\u200db"` 2 |
| P4b | backward from end of family emoji | 0 | 0 |
| P5 | categorizer on U+FEFF, U+00A0, U+200B, U+1680, U+180E, `_`, `é`, `٣`, U+0301, `$`, `中` | Space, Space, Other, Space, Other, Word, Word, Word, Other, Other, Word | Other, Space, Other, Space, Other, Word, Word, Word, Other, Other, Word |
| P6 | `find_subword_break("ae\u{301}b", 0, true)` | (no cm6 counterpart in state) | 2 (cluster of `e\u0301` is 1..3) |
| P6b | `find_word_break("ae\u{301} b", 0, true)` | (none) | 3 |
| P6c | `find_subword_break("fooBar",0)`, `("foo_bar",0)` | (none) | 3, 3 |
| P7 | `countColumn("😀\tx",4)`, `findColumn("😀a",1,4)`, `findColumn("ab",5,4,true)`, `countColumn("\tx",0)` | 5, 2, -1, NaN | 5, `Some(1)`, `None`, panic "attempt to calculate the remainder with a divisor of zero" |
| P8 | `EditorSelection.create([], 0)` | `RangeError: A selection needs at least one range` | `[(0,0)]` |
| P8b | create `[(6,7),(7,8),c3,c3,(10,12),(13,11)]`, main 5 | `[[3,3],[6,7],[7,8],[13,10]]`, main 3 | identical |
| P8c | create `[cursor(1)]`, main 7 | mainIndex 7 | 0 |
| P9 | `[readOnly.of(false), readOnly.of(true)]` | `readOnly` false | `is_read_only()` true |
| P10 | doc `"abcdef"`, two cursors, no `allowMultipleSelections` | 1 range | 2 ranges |
| P11 | doc `"a\|b"`, `lineSeparator.of("\|")` | 2 lines | 1 line |
| P12 | same extension value used twice (`let e = f.of(1); [e, e]` / `vec![e.clone(), e]`) | `[1]` | `[1, 1]` |
| P13 | `a.compute(s => s.facet(b).length)`, `b.compute(() => 5)` | `[1]` | panic `index out of bounds: the len is 0 but the index is 1` (`config.rs:175`) |
| P14 | two `userEvent` annotations `"a"`, `"b"`; read back | `"a"` | `UserEvent("b")` |
| P15 | `tr.state === tr.state` / `Arc::ptr_eq(&tr.state(), &tr.state())` | true | false |
| P16 | set with range 5..6, startSide 2,000,000; `between(0, 5)` | `[[5,6]]` | `[]` |
| P17 | point at 2, startSide=endSide=-256, map over insert `"xx"` at 2 (doc length 4) | `[[2,2]]` | `[(4,4)]` |
| P17b | same with side -1 | `[[2,2]]` | `[(2,2)]` |
| P18 | builder `add(5,6)` then `add(1,2)` | `Error: Ranges must be added sorted by \`from\` position and \`startSide\`` | no error, size 2 |
| P19 | `fn send_sync<T: Send + Sync>()` for `EditorState`, `Text` | — | compiles |
| P20 | property check, xorshift64 seed `0x9E3779B97F4A7C15`, 20000 cases, ASCII docs of length 0..19, 0..3 random replace specs each: `a.compose(b.map(a,false)).apply(d) == b.compose(a.map(b,true)).apply(d)` and `a.invert(d).apply(a.apply(d)) == d` | — | 0 confluence failures, 0 inversion failures |
| P21 | `changeByRange` over cursors 1 and 4, each inserting `"XX"` with a position effect (`StateEffect.define({map})`) | effects `[1,6]` (first effect mapped) | no counterpart: `StateEffect` has no mapping, effects are concatenated unmapped (`state.rs:234`) |
| P22 | `replaceSelection("x").selection.main.assoc` | -1 | `replace_selection` builds `SelectionRange::cursor` with assoc 0 (`state.rs:249`) |
| Q1 | categorizer(`"\u0301e"`), categorizer(`""`) | Word, Space | Other, Other |
| Q2 | empty doc line count | 1 | 1 |
| Q3 | `line_at(3)` on `"ab"` | (cm6 throws RangeError too) | panic |

### R5 Selection normalization oracle
cm6 `probe-A4/selnorm.ts` (xorshift32, seed `0xA4A4A4A4`) generates 5000 cases of 1-5 ranges with anchor/head in 0..10 and a random valid main index, writes cm6's `EditorSelection.create` result to `%(SP)s/repro-A4/selnorm.json`; Ferrum `src/bin/selnorm.rs` replays them through `EditorSelection::create`. Result: 5000 cases, 0 mismatches (ranges and main index). ASCII positions only; no non-ASCII sample needed (no text involved).

### R6 Historical commits
`git archive <commit>` of the workspace into `%(SP)s/repro-A4/hist/<commit>/`, then `cargo test --offline -p ferrum-state` with a separate target dir per commit (a shared target dir gave stale artifacts because path packages with identical relative paths share metadata hashes). 423cb68: `177 passed; 0 failed`. a8d9f00: `177 passed; 0 failed`. Test counts per file at both commits: change 42, charcategory 3, facet 22, rangeset 47, selection 4, state 27, text 32.

### R7 Current test counts and warnings
`cargo test --locked --offline -p ferrum-state -p ferrum-commands -p ferrum-language` (external target dir): ferrum-state 177 passed, ferrum-commands 142 passed, ferrum-language 15 passed, 0 doctests. After `cargo clean -p ferrum-state`, `cargo test --no-run -p ferrum-state` printed no warnings. `cargo clippy` is not installed (`'cargo-clippy' is not installed for the toolchain`).

### R8 Workspace searches
`grep -rn -i "change_filter|transaction_extender|language_data|to_json|from_json|word_at|slice_doc|line_break|is_user_event|append_config|utf16|utf_16|touches_range" crates/*/src --include=*.rs` (excluding tests): only unrelated hits in `crates/ferrum-view` (`line_breaks` of widgets, `widget_line_breaks`). `grep allow_multiple_selections|ALLOW_MULTIPLE` and `line_separator|LINE_SEPARATOR` outside tests: only the definitions in `crates/ferrum-state/src/state.rs`. `grep -rni copyright` over `*.md|*.rs|*.toml`: no hits. `ls /workspace/ferrum`: no `LICENSE*`, `NOTICE` or `THIRD-PARTY` file.

## Unfinished

none
"""%{"SP":SP})
open('/workspace/ferrum/docs/parity/state/raw/A4-claims.md','w').write("\n".join(out))
print(len(rows),cnt)
for k,c in per.items(): print(k,dict(c))
