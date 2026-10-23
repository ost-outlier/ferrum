"""Fetch, check and pin the source mirrors of docs/parity/tools/mirrors.json (into mirrors/, gitignored).

Run from the repository root:
  python3 docs/parity/tools/fetch_mirrors.py [target ...]        fetch pinned targets (default: all pinned)
  python3 docs/parity/tools/fetch_mirrors.py --check [target ...] report missing or stale targets (exit 1 if any)
  python3 docs/parity/tools/fetch_mirrors.py --pin target [ref]  resolve ref (default: the entry's ref) to a commit
                                                                 and write it into the manifest; then fetch
A target is a mirror (`zed`, `cm6` = every member of the set) or one member of a set (`cm6/view`).
Pinning a member that a set does not list yet adds it. Fetched trees carry no .git; the commit is in
<dir>/.mirror-commit. A repo entry with "include" fetches only those paths (sparse checkout); with
"license_check": "cargo", every included crate must declare a license listed in "license".
"""
import json, pathlib, re, shutil, subprocess, sys, tempfile
REPO = pathlib.Path(__file__).resolve().parents[3]
MANIFEST = REPO / 'docs/parity/tools/mirrors.json'
data = json.loads(MANIFEST.read_text())
M = data['mirrors']

def targets(names):
    """Yield (label, entry, url, dst) for every target named (or all)."""
    for name in names or list(M):
        mirror, _, member = name.partition('/')
        e = M[mirror]
        if e['kind'] == 'set':
            for mem in ([member] if member else list(e['members'])):
                yield f'{mirror}/{mem}', e['members'][mem], e['url'].format(member=mem), REPO / 'mirrors' / mirror / mem, e
        else:
            yield mirror, e, e['url'], REPO / 'mirrors' / mirror, e

def git(cwd, *a):
    return subprocess.run(['git', '-C', str(cwd), *a], check=True, capture_output=True, text=True).stdout

def pin(name, ref=None):
    mirror, _, member = name.partition('/')
    e = M[mirror]
    if e['kind'] == 'set':
        if not member: sys.exit('pin one member of a set: cm6/<member>')
        entry = e['members'].setdefault(member, {'ref': ref or 'refs/heads/main'})
        url = e['url'].format(member=member)
    else:
        entry, url = e, e['url']
    ref = ref or entry['ref']
    out = subprocess.run(['git', 'ls-remote', url, ref], check=True, capture_output=True, text=True).stdout.split()
    if not out: sys.exit(f'{url}: no ref {ref}')
    peeled = subprocess.run(['git', 'ls-remote', url, ref + '^{}'], capture_output=True, text=True).stdout.split()
    entry['ref'], entry['commit'] = ref, (peeled or out)[0]
    MANIFEST.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n')
    print(f'pinned   {name} {ref} -> {entry["commit"][:7]} (commit docs/parity/tools/mirrors.json)')

def license_ok(tree, entry, mirror):
    if mirror.get('license_check') != 'cargo': return []
    bad = []
    for inc in entry.get('include', []):
        toml = tree / inc / 'Cargo.toml'
        m = re.search(r'^license\s*=\s*"([^"]+)"', toml.read_text(), re.M) if toml.exists() else None
        if not m or m.group(1) not in mirror['license']: bad.append(f'{inc}: {m.group(1) if m else "no license field"}')
    return bad

def fetch(label, entry, url, dst, mirror):
    with tempfile.TemporaryDirectory() as tmp:
        t = pathlib.Path(tmp)
        git(t, 'init', '-q')
        sparse = entry.get('include')
        filt = ['--filter=blob:none'] if sparse else []
        if sparse:
            # the repository's own license files too: included crates often link to them
            git(t, 'sparse-checkout', 'set', '--no-cone', '/LICENSE*', *[f'/{p}/' for p in sparse])
        try:
            git(t, 'fetch', '-q', '--depth', '1', *filt, url, entry['commit'])
        except subprocess.CalledProcessError:   # servers that refuse fetching a commit by id
            git(t, 'fetch', '-q', *filt, url, f'{entry["ref"]}:refs/mirror')
        git(t, '-c', 'advice.detachedHead=false', 'checkout', '-q', entry['commit'])
        bad = license_ok(t, entry, mirror)
        if bad: sys.exit(f'refused {label}: license not in {mirror["license"]}: ' + '; '.join(bad))
        shutil.rmtree(t / '.git')
        if dst.exists(): shutil.rmtree(dst)
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(t, dst, symlinks=True)
    (dst / '.mirror-commit').write_text(entry['commit'] + '\n')
    print(f'fetched  {label} {entry["commit"][:7]}')

args = [a for a in sys.argv[1:] if not a.startswith('--')]
if '--pin' in sys.argv:
    if not args: sys.exit('usage: --pin <target> [ref]')
    pin(args[0], args[1] if len(args) > 1 else None)
    args = args[:1]
check, problems = '--check' in sys.argv, 0
for label, entry, url, dst, mirror in targets(args):
    if not entry.get('commit'):
        print(f'unpinned {label} (pin it: fetch_mirrors.py --pin {label})'); continue
    stamp = dst / '.mirror-commit'
    have = stamp.read_text().strip() if stamp.exists() else None
    if have == entry['commit']: print(f'ok       {label} {have[:7]}'); continue
    if check:
        print(f'{"missing" if have is None else "stale":8} {label} (want {entry["commit"][:7]})'); problems += 1; continue
    fetch(label, entry, url, dst, mirror)
sys.exit(1 if problems else 0)
