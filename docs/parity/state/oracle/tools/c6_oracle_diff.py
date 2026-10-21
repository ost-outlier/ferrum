import sys, collections
def load(p):
    d = {}
    for line in open(p):
        line = line.rstrip("\n")
        if not line: continue
        cid, op, rest = (line.split(" ", 2) + [""])[:3]
        d[(cid, op)] = rest
    return d
a, b = load(sys.argv[1]), load(sys.argv[2])
stats = collections.Counter(); multi = collections.Counter(); first = {}
for k in a:
    op = k[1].rstrip("0123456789")
    stats[op + ":total"] += 1
    if k not in b: stats[op + ":missing"] += 1; continue
    if a[k] != b[k]:
        stats[op + ":mismatch"] += 1
        if op in ("iter", "map", "update") and sorted(a[k].split()) == sorted(b[k].split()):
            multi[op + ":order-only"] += 1
        first.setdefault(op, []).append(k)
for op in sorted(set(k.split(":")[0] for k in stats)):
    print(op, "total", stats[op+":total"], "mismatch", stats[op+":mismatch"], "order-only", multi[op+":order-only"])
for op, ks in first.items():
    print("== examples", op, [k[0] for k in ks[:8]])
