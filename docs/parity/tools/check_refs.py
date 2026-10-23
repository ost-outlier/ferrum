"""Reference checker (ferrum/docs/fundation/1-parity-audit.md §11.1, rule 7).

Checks Markdown files for references an AI reader cannot follow:
  SCRATCH   path under /tmp, a scratchpad or SCR/
  ROOT      path relative to an unstated root (state/src/..., crates/ferrum-...), or an
            instruction document outside ferrum/docs/fundation/
  MISSING   path rooted at /workspace/ (CodeMirror6/..., ferrum/...) that does not exist
  LINE      file:line beyond the end of the file
  ID        "`<file>.md` R9"-style evidence id not found in that file
  SECTION   "`<file>.md` §N.M" with no such section in that file: a heading "## N." / "### N.M", or item
            M of a numbered list under heading N. A bare §N inherits the file of a qualified §
            earlier on the same line; otherwise, in an instruction document it means that document
            (rules.md §8.5), and elsewhere it is not checked.
Usage: python3 check_refs.py <file-or-dir>... [--quiet]
Line numbers in ferrum/crates/ refer to the commit stated in each document's header; run at that commit.
"""
import os, re, sys, pathlib, collections
# Workspace root, written /workspace/ in the documents (rules.md §8.1): $FERRUM_WORKSPACE, or the directory containing ferrum/.
W = pathlib.Path(os.environ.get('FERRUM_WORKSPACE') or pathlib.Path(__file__).resolve().parents[4])
PATH = re.compile(r'(?<![\w/.@-])((?:CodeMirror6|ferrum)/[\w./-]*[\w])(?::L?(\d+))?')
SCR = re.compile(r'/tmp/|scratchpad|(?<![\w/])SCR/')
BARE = re.compile(r'(?<![\w/.@-])(?:(?:state|view|language|commands|streamparser|lang-[a-z]+)/(?:src|test)/|crates/ferrum|docs/(?:parity|fundation)/)')
OLDCOPY = re.compile(r'/workspace/(?:1-parity-audit|1[a-d]-scope-[\w-]+|2[a-z]?-[\w-]+|3-conformance-audit|ferrum-foundation|rules)\.md')
QID = re.compile(r'`((?:ferrum|CodeMirror6)/[\w./-]+\.md)`[ #]((?:R|P|S|T|W)[\w-]*\d[\w-]*)')
# Paths that are planned but not yet written; a line may opt out with <!-- refs:ignore --> (for quoted bad examples).
PLANNED = re.compile(r'ferrum/docs/parity/(?:decisions\.md|state/spec(?:/.*)?|(?!state/|tools/)[\w-]+/.*)$')  # decisions.md and future crate folders
FUND = W / 'ferrum/docs/fundation'
SEC = re.compile(r'(?:`?((?:ferrum/)?[\w./-]*[\w-]+\.md)`?\s+)?§\s?(\d+(?:\.\d+)?)(?:\s?[–-]\s?(\d+(?:\.\d+)?))?')
# Historical documents describe sections of earlier versions; their bare § are not checked.
SEC_SKIP = {'LEARNINGS.md', '1a-scope-state.md'}
sections_cache = {}
def sections(p):
    if p not in sections_cache:
        keys, top, in_code = set(), None, False
        for line in p.read_text(errors='replace').split('\n'):
            if line.startswith('```'): in_code = not in_code
            if in_code: continue
            m = re.match(r'#{2,4} (\d+(?:\.\d+)*)\.?\s', line)
            if m:
                keys.add(m.group(1)); top = m.group(1).split('.')[0]; continue
            m = re.match(r'(\d+)\. ', line)
            if m and top is not None: keys.add(f'{top}.{m.group(1)}')
        sections_cache[p] = keys
    return sections_cache[p]
def resolve(name):
    p = W / name if name.startswith('ferrum/') else FUND / pathlib.Path(name).name
    return p if p.is_file() else None
lines_cache = {}
def nlines(p):
    if p not in lines_cache: lines_cache[p] = p.read_text(errors='replace').count('\n') + 1
    return lines_cache[p]
def check(md, out):
    text = md.read_text(); in_code = False
    for i, line in enumerate(text.split('\n'), 1):
        if line.startswith('```'): in_code = not in_code
        if 'refs:ignore' in line: continue
        for m in SCR.finditer(line): out.append(('SCRATCH', md, i, line[max(0, m.start()-30):m.end()+40]))
        for m in OLDCOPY.finditer(line): out.append(('ROOT', md, i, m.group(0) + ' (instruction docs live only in ferrum/docs/fundation/)'))
        for m in ([] if in_code else BARE.finditer(line)): out.append(('ROOT', md, i, line[max(0, m.start()-10):m.end()+40]))
        for m in PATH.finditer(line):
            rel, ln = m.group(1), m.group(2)
            nxt = line[m.end(1):m.end(1)+2]
            if any(c in rel for c in '<>*{}…') or any(c in nxt for c in '<*{…'): continue
            p = W / rel
            if PLANNED.match(rel): continue
            if not p.exists():
                # tolerate generated/runtime paths only when explicitly marked in the line
                if re.search(r'not archived|does not exist|not yet written|will be|(?i:\bat (?:commit )?[0-9a-f]{7}\b)', line): continue
                out.append(('MISSING', md, i, rel)); continue
            if ln and p.is_file() and int(ln) > nlines(p): out.append(('LINE', md, i, f'{rel}:{ln} > {nlines(p)} lines'))
        last = None
        for m in ([] if in_code else SEC.finditer(line)):
            name = m.group(1)
            if name: target = resolve(name); last = target
            elif last: target = last
            elif md.resolve().parent == FUND and md.name not in SEC_SKIP: target = md.resolve()
            else: continue
            if target is None: continue
            for key in filter(None, (m.group(2), m.group(3))):
                if key not in sections(target): out.append(('SECTION', md, i, f'§{key} not in {target.relative_to(W)}'))
        for m in QID.finditer(line):
            f = W / m.group(1)
            for rid in re.split(r'[-–/]', m.group(2)):
                if not re.fullmatch(r'[RPSTW][\w]*\d[\w]*', rid): continue
                if f.exists() and not re.search(r'(?<![\w-])' + re.escape(rid) + r'(?![\w])', f.read_text()):
                    out.append(('ID', md, i, f'{rid} not in {m.group(1)}'))
args = [a for a in sys.argv[1:] if not a.startswith('--')]
files = []
for a in args:
    p = pathlib.Path(a)
    files += sorted(x for x in p.rglob('*.md') if '/oracle/' not in str(x)) if p.is_dir() else [p]
out = []
for f in files: check(f, out)
c = collections.Counter(k for k, *_ in out)
if '--quiet' not in sys.argv:
    for k, f, i, s in out: print(f'{k:8} {f}:{i}  {s}')
print('files', len(files), 'findings', dict(c) if c else 0)
sys.exit(1 if out else 0)
