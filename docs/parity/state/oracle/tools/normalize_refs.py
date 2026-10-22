"""One-off normalizer for the ferrum-state audit documents (ferrum/docs/parity/**/*.md).

Rewrites references written for the producing session into references rooted at
/workspace/ (ferrum/docs/fundation/1-parity-audit.md §11.1):
  - scratch paths (/tmp/..., SCR/, .../scratchpad/) -> archived copies under
    ferrum/docs/parity/state/oracle/ or an explicit "not archived" note;
  - cm6 package-relative paths (state/src/..., view/src/...) -> CodeMirror6/...;
  - Ferrum workspace-relative paths (crates/ferrum-...) -> ferrum/crates/...;
  - parity-folder-relative paths (raw/..., ../_cross-crate.md, sibling deliverables)
    -> ferrum/docs/parity/...;
  - chunk-qualified evidence ids in prose ("C1 R9") -> "`ferrum/docs/parity/state/raw/C1-api.md` R9".
Usage: python3 normalize_refs.py [--dry-run]
"""
import re, sys, pathlib
ROOT = pathlib.Path('/workspace/ferrum/docs/parity')
ST = 'ferrum/docs/parity/state'
PRE = r'(?:/tmp/claude-1000/-workspace/8fb87903-f955-477c-a67c-0747517f569e/scratchpad/|(?<![\w/])SCR/|…/scratchpad/|\.\.\./scratchpad/|(?<![\w/])scratchpad/)'
PKG = r'(?:state|view|language|commands|autocomplete|search|lint|collab|merge|lsp-client|streamparser|lang-[a-z]+)'
DELIV = r'(?:api-matrix|behaviors|tests-side-by-side|claims|ferrum-only|placement|human-decisions|oracle|verification|progress|dispositions|subagent-card)'
NA = '(oracle data, not archived: regenerate from the seed recorded in `' + ST + '/oracle.md`)'
rules = [
    (PRE + r'cm6/probe-(\w+)', ST + r'/oracle/cm6-probes/probe-\1'),
    (PRE + r'cm6/node_modules/@marijn/find-cluster-break/?', '`@marijn/find-cluster-break` 1.0.4 (npm) '),
    (PRE + r'cm6/src', 'CodeMirror6/state/src'),
    (PRE + r'cm6/?(?![\w-])', ST + '/oracle.md'),
    (PRE + r'repro-(\w+)', ST + r'/oracle/ferrum-repro/repro-\1'),
    (PRE + r'oracle-C2/classify\.cjs', ST + '/oracle/tools/classify-C2.cjs'),
    (PRE + r'oracle-\w+/?[\w.{},*/-]*', NA),
    (PRE + r'verif/ist-dl[\w./-]*', '`ist` 1.1.7 package source from npm (not archived)'),
    (PRE + r'verif/target/?', '(build directory, not archived)'),
    (PRE + r'verif/', ST + '/oracle/verification/'),
    (PRE + r'prompts/sample\.txt', ST + '/verification-sample.txt'),
    (PRE + r'prompts/common\.md', ST + '/subagent-card.md'),
    (PRE + r'tools/', ST + '/oracle/tools/'),
    (PRE + r'a4gen/', ST + '/oracle/a4gen/'),
    (r'(?<![\w/.@-])(' + PKG + r')/(src|test)/', r'CodeMirror6/\1/\2/'),
    (r'(?<![\w/.@-])(' + PKG + r')/package\.json', r'CodeMirror6/\1/package.json'),
    (r'(?<![\w/.-])crates/(ferrum[\w-]*)', r'ferrum/crates/\1'),
    (r'/workspace/ferrum/', 'ferrum/'),
    (r'/workspace/CodeMirror6/', 'CodeMirror6/'),
    (r'(?<![\w/.-])\.\./_cross-crate\.md', 'ferrum/docs/parity/_cross-crate.md'),
    (r'(?<![\w/.-])\.\./decisions-evidence\.md', 'ferrum/docs/parity/decisions-evidence.md'),
    (r'(?<![\w/.-])(?:state/)?raw/', ST + '/raw/'),
    (r'(?<![\w/.-])oracle/(cm6-probes|ferrum-repro|tools|verification|a4gen)', ST + r'/oracle/\1'),
    (r'(?<![\w/.-])(C\d-(?:api|notes|tests|behaviors)\.md|A4-claims\.md|a1-cm6-exports\.md|a2-ferrum-pub\.md|t1-cm6-tests\.md|t2-ferrum-tests\.md)', ST + r'/raw/\1'),
    (r'`(' + DELIV + r')\.md`', '`' + ST + r'/\1.md`'),
    (r'(?<![\w/.-])`_cross-crate\.md', '`ferrum/docs/parity/_cross-crate.md'),
    (r'(?<![\w/.-])`decisions-evidence\.md', '`ferrum/docs/parity/decisions-evidence.md'),
    (r'(?<![\w/.-])`decisions\.md`', '`ferrum/docs/parity/decisions.md`'),
]
# "C1 R9", "C3 R2/R3", "C1 P1, P3", "C4 R-D1": qualify the chunk with its reproduction file (prose files only)
CHUNK_ID = re.compile(r'(?<![\w/.`-])C([1-6]) (?=(?:R|P|S|T|W)[\w-]*\b)')
SRC_COL = re.compile(r'(?<![\w/.`-])C([1-6])-(api|notes|tests|behaviors)(?![\w.])')
def fix(text, prose):
    for pat, rep in rules: text = re.sub(pat, rep, text)
    text = re.sub(r'(ferrum/docs/parity/state/)+', 'ferrum/docs/parity/state/', text)
    text = re.sub(r'(ferrum/)+(crates|docs)', r'ferrum/\2', text)
    text = re.sub(r'(CodeMirror6/)+', 'CodeMirror6/', text)
    text = SRC_COL.sub(lambda m: f'`{ST}/raw/C{m.group(1)}-{m.group(2)}.md`', text)
    if prose: text = CHUNK_ID.sub(lambda m: f'`{ST}/raw/C{m.group(1)}-api.md` ', text)
    return text
dry = '--dry-run' in sys.argv; changed = 0
for p in sorted(ROOT.rglob('*.md')):
    if '/oracle/' in str(p) or p.name == 'README.md' and p.parent == ROOT: continue
    defining = re.fullmatch(r'C\d-api\.md', p.name) is not None
    old = p.read_text(); new = fix(old, prose=not defining)
    if new != old:
        changed += 1
        if not dry: p.write_text(new)
print('files changed:', changed, '(dry run)' if dry else '')
