# Public items of a Ferrum crate from rustdoc JSON, as JSON Lines (docs/fundation/3-conformance-audit.md G1).
#   RUSTC_BOOTSTRAP=1 cargo rustdoc -p <crate> -- -Z unstable-options --output-format json
#   python3 ferrum_pub.py target/doc/<crate_name>.json <crate dir, e.g. crates/ferrum-view> > pub.jsonl
# Enumerate every public item of a crate from rustdoc JSON (format 61):
# modules, structs (+pub fields), enums (+variants), traits (+methods), functions,
# consts, statics, type aliases, inherent impl methods/assoc items, crate-trait impls,
# and record std-trait impls (non-derived and derived) per type.
import json, sys
CRATE = sys.argv[2].rstrip('/')
d = json.load(open(sys.argv[1])); idx = d['index']; root = idx[str(d['root'])]
rows = []; seen = set(); impls_by_type = {}
def sp(it):
    s = it.get('span'); return f"{s['filename']}:{s['begin'][0]}" if s else "?"
def tname(t):
    if 'resolved_path' in t: return t['resolved_path']['path']
    if 'generic' in t: return t['generic']
    return '?'
def add(path, kind, it): rows.append((path, kind, sp(it)))
def walk_impl(owner, impl_id):
    imp = idx[str(impl_id)]['inner']['impl']
    tr = imp.get('trait')
    if imp.get('is_synthetic') or imp.get('blanket_impl'): return
    if tr:
        trn = tr['path']; tid = tr.get('id')
        local = tid is not None and str(tid) in idx and idx[str(tid)]['crate_id'] == 0
        derived = any('automatically_derived' in json.dumps(a) for a in idx[str(impl_id)].get('attrs', []))
        args = json.dumps(tr.get('args'))
        label = trn + ("" if trn not in ('From',) else "<" + args.split('"path": "')[1].split('"')[0] + ">" if '"path"' in args else trn)
        impls_by_type.setdefault(owner, []).append(("derive " if derived else "impl ") + label + ("" if not local else " (crate trait)"))
        if local or not derived:
            for i in imp['items']:
                it = idx[str(i)]
                if it['name'] in ('fmt','clone','eq','ne','default','hash','partial_cmp','cmp','from'): continue
                add(f"{owner} as {trn}::{it['name']}", "trait impl method", it)
        return
    for i in imp['items']:
        it = idx[str(i)]
        if it['visibility'] != 'public': continue
        k = list(it['inner'].keys())[0]
        add(f"{owner}::{it['name']}", {'function': 'method', 'assoc_const': 'assoc const', 'assoc_type': 'assoc type'}.get(k, k), it)
def walk(item_id, prefix):
    it = idx[str(item_id)]; k = list(it['inner'].keys())[0]; inner = it['inner'][k]
    if it['visibility'] != 'public' and k != 'module': return
    name = it.get('name'); path = f"{prefix}::{name}" if prefix else name
    if k == 'use':
        rows.append((f"{prefix}::{inner['name']}", "re-export of " + inner['source'], sp(it))); return
    if item_id in seen: return
    seen.add(item_id)
    if k == 'module':
        if prefix: add(path, 'module', it)
        for c in inner['items']: walk(c, path if prefix else root['name'])
        return
    add(path, k, it)
    if k == 'struct':
        kind = inner['kind']
        fields = kind.get('plain', {}).get('fields', []) if isinstance(kind, dict) else []
        if isinstance(kind, dict) and 'tuple' in kind: fields = [f for f in kind['tuple'] if f is not None]
        for f in fields:
            fi = idx[str(f)]
            if fi['visibility'] == 'public': add(f"{path}.{fi['name']}", 'field', fi)
        for i in inner['impls']: walk_impl(path, i)
    elif k == 'enum':
        for v in inner['variants']:
            vi = idx[str(v)]; add(f"{path}::{vi['name']}", 'variant', vi)
        for i in inner['impls']: walk_impl(path, i)
    elif k == 'trait':
        for i in inner['items']:
            ti = idx[str(i)]; add(f"{path}::{ti['name']}", 'trait method' + (' (provided)' if ti['inner'].get('function', {}).get('has_body') else ''), ti)
walk(d['root'], '')
for i, r in enumerate(rows, 1): print(json.dumps({"id": f"{CRATE.split('/')[-1]}.fe.{i:03d}", "item": r[0], "kind": r[1], "loc": r[2]}))
for t, l in impls_by_type.items(): print(json.dumps({"type": t, "impls": sorted(set(l))}), file=sys.stderr)
