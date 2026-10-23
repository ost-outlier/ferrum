"""Render JSON Lines rows as a Markdown table (docs/fundation/1-parity-audit.md §11).

    python3 render.py rows.jsonl --cols id,item,file,line [--title "Inventory"] [--header "> What: ..."] > out.md

Arrays are joined with ", "; "|" and newlines inside cells are escaped. Prints the row count under the title.
"""
import json, sys, argparse
ap = argparse.ArgumentParser(); ap.add_argument('jsonl'); ap.add_argument('--cols', required=True)
ap.add_argument('--title'); ap.add_argument('--header')
a = ap.parse_args(); cols = a.cols.split(',')
rows = [json.loads(l) for l in open(a.jsonl) if l.strip()]
def cell(v):
    if v is None: return ''
    if isinstance(v, dict): v = json.dumps(v, ensure_ascii=False)
    elif isinstance(v, list): v = ', '.join(json.dumps(x, ensure_ascii=False) if isinstance(x, (dict, list)) else str(x) for x in v)
    return str(v).replace('|', '\\|').replace('\n', ' ')
if a.title: print(f'# {a.title}\n')
if a.header: print(a.header + '\n')
print(f'Rows: {len(rows)}.\n')
print('| ' + ' | '.join(cols) + ' |'); print('|' + '---|' * len(cols))
for r in rows: print('| ' + ' | '.join(cell(r.get(c)) for c in cols) + ' |')
