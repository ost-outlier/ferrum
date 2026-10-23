"""Fetch the source mirrors pinned in docs/parity/tools/mirrors.json into mirrors/ (gitignored).

Usage (from the repository root): python3 docs/parity/tools/fetch_mirrors.py [package ...] [--check]
Each package is cloned at its pinned commit and copied without .git into mirrors/cm6/<package>/,
with the commit in mirrors/cm6/<package>/.mirror-commit. Packages already at their pin are skipped.
--check only reports which packages are missing or at another commit (exit 1 if any).
"""
import json, pathlib, shutil, subprocess, sys, tempfile
REPO = pathlib.Path(__file__).resolve().parents[3]
manifest = json.loads((REPO / 'docs/parity/tools/mirrors.json').read_text())['cm6']
want = [a for a in sys.argv[1:] if not a.startswith('--')] or list(manifest['packages'])
check = '--check' in sys.argv
bad = 0
for name in want:
    pin = manifest['packages'][name]
    dst = REPO / 'mirrors/cm6' / name
    stamp = dst / '.mirror-commit'
    have = stamp.read_text().strip() if stamp.exists() else None
    if have == pin['commit']:
        print(f'ok       {name} {pin["commit"][:7]}'); continue
    if check:
        print(f'{"missing" if have is None else "stale":8} {name} (want {pin["commit"][:7]}, have {have and have[:7]})'); bad += 1; continue
    with tempfile.TemporaryDirectory() as tmp:
        git = lambda *a: subprocess.run(['git', '-C', tmp, *a], check=True, capture_output=True)
        git('init', '-q')
        url = manifest['url'].format(package=name)
        try:
            git('fetch', '-q', '--depth', '1', url, pin['commit'])
        except subprocess.CalledProcessError:
            git('fetch', '-q', url, f'{pin["ref"]}:refs/mirror')  # servers that refuse fetching by commit
        git('-c', 'advice.detachedHead=false', 'checkout', '-q', pin['commit'])
        shutil.rmtree(pathlib.Path(tmp) / '.git')
        if dst.exists(): shutil.rmtree(dst)
        shutil.copytree(tmp, dst)
    stamp.write_text(pin['commit'] + '\n')
    print(f'fetched  {name} {pin["commit"][:7]}')
sys.exit(1 if bad else 0)
