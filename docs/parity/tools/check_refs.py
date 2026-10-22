"""Reference checker (ferrum/docs/fundation/1-parity-audit.md §11.1, rule 7).

Checks Markdown files for references an AI reader cannot follow:
  SCRATCH   path under /tmp, a scratchpad or SCR/
  ROOT      path relative to an unstated root (state/src/..., crates/ferrum-...), or an
            instruction document outside ferrum/docs/fundation/
  MISSING   path rooted at /workspace/ (CodeMirror6/..., ferrum/...) that does not exist
  LINE      file:line beyond the end of the file
  ID        "`<file>.md` R9"-style evidence id not found in that file
Usage: python3 check_refs.py <file-or-dir>... [--quiet]
Line numbers in ferrum/crates/ refer to the commit stated in each document's header; run at that commit.
"""
import re, sys, pathlib, collections
W = pathlib.Path('/workspace')
PATH = re.compile(r'(?<![\w/.@-])((?:CodeMirror6|ferrum)/[\w./-]*[\w])(?::L?(\d+))?')
SCR = re.compile(r'/tmp/|scratchpad|(?<![\w/])SCR/')
BARE = re.compile(r'(?<![\w/.@-])(?:(?:state|view|language|commands|streamparser|lang-[a-z]+)/(?:src|test)/|crates/ferrum|docs/(?:parity|fundation)/)')
OLDCOPY = re.compile(r'/workspace/(?:1-parity-audit|1[a-d]-scope-[\w-]+|2[a-z]?-[\w-]+|3-conformance-audit|ferrum-foundation|rules)\.md')
QID = re.compile(r'`((?:ferrum|CodeMirror6)/[\w./-]+\.md)`[ #]((?:R|P|S|T|W)[\w-]*\d[\w-]*)')
# Paths that are planned but not yet written; a line may opt out with <!-- refs:ignore --> (for quoted bad examples).
PLANNED = re.compile(r'ferrum/docs/parity/(?:decisions\.md|state/spec(?:/.*)?|(?!state/|tools/)[\w-]+/.*)$')  # decisions.md and future crate folders
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
