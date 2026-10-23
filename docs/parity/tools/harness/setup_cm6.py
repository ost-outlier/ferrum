"""Build the shared cm6 harness for one audit (run once, by the main session).

    python3 setup_cm6.py <package> [--dir DIR] [--npm PKG@VER ...]

<package> is a cm6 package fetched into mirrors/cm6/ (docs/parity/tools/fetch_mirrors.py): state, view, language, commands, ...
The harness gets: copies of that package's src/ and of every @codemirror/* package it depends on
(transitively), their third-party npm dependencies, typescript, and a tsconfig.json whose `paths`
map every @codemirror/<name> import to the copied sources. Probes and generators can then live
anywhere (normally docs/parity/<crate>/oracle/cm6/<chunk>/) and run with
    tsx --tsconfig <DIR>/tsconfig.json <probe>.ts
Extra npm packages (Lezer grammars, w3c-keyname, ...) go after --npm.
Default DIR: .harness/<package> at the repository root (gitignored; never referenced by deliverables).
An @codemirror/* dependency missing from mirrors/cm6/ is installed from npm like a third-party package: add it to
docs/parity/tools/mirrors.json first if its sources are part of the audit.
"""
import json, os, shutil, subprocess, sys, argparse
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.realpath(__file__)), *['..'] * 4))
CM6 = f'{REPO}/mirrors/cm6'
ap = argparse.ArgumentParser(); ap.add_argument('package'); ap.add_argument('--dir'); ap.add_argument('--npm', nargs='*', default=[])
a = ap.parse_args(); d = a.dir or f'{REPO}/.harness/{a.package}'
def pkgjson(name): return json.load(open(f'{CM6}/{name}/package.json'))
todo, seen, third = [a.package], set(), {}
while todo:
    p = todo.pop()
    if p in seen: continue
    seen.add(p)
    for dep, ver in pkgjson(p).get('dependencies', {}).items():
        if dep.startswith('@codemirror/') and os.path.isdir(f"{CM6}/{dep.split('/')[1]}"): todo.append(dep.split('/')[1])
        else: third[dep] = ver
os.makedirs(d, exist_ok=True)
paths = {}
for p in sorted(seen):
    dst = f'{d}/cm6/{p}/src'
    if os.path.isdir(dst): shutil.rmtree(dst)
    shutil.copytree(f'{CM6}/{p}/src', dst)
    paths[f'@codemirror/{p}'] = [f'{dst}/index.ts']
    paths[pkgjson(p)['name']] = [f'{dst}/index.ts']
paths['@harness/lib'] = [f'{REPO}/docs/parity/tools/harness/oracle-lib.ts']
json.dump({'name': f'harness-{a.package}', 'private': True}, open(f'{d}/package.json', 'w'))
json.dump({'compilerOptions': {'target': 'es2022', 'module': 'esnext', 'moduleResolution': 'bundler', 'baseUrl': d, 'paths': paths, 'skipLibCheck': True}}, open(f'{d}/tsconfig.json', 'w'), indent=1)
pkgs = [f'{k}@{v}' for k, v in sorted(third.items())] + ['typescript@5.6.3', '@types/node@22'] + a.npm
subprocess.run(['npm', 'install', '--silent', '--no-audit', '--no-fund', *pkgs], cwd=d, check=True)
versions = {p: pkgjson(p)['version'] for p in sorted(seen)}
json.dump({'cm6': versions, 'npm': pkgs}, open(f'{d}/harness.json', 'w'), indent=1)
print(f'harness ready: {d}\n  cm6 packages: {versions}\n  npm: {pkgs}\n  run: tsx --tsconfig {d}/tsconfig.json <probe.ts>')
