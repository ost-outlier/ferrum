import re, sys
def load(p):
    d = {}
    for line in open(p):
        line=line.rstrip("\n")
        if not line: continue
        cid, op, rest = (line.split(" ", 2)+[""])[:3]; d[(cid,op)] = rest
    return d
def merged(s):
    out=[]
    for m in re.finditer(r"[rp]\((\d+),(\d+)", s):
        f,t=int(m.group(1)),int(m.group(2))
        if out and out[-1]==f: out[-1]=t
        else: out+= [f,t]
    return out
a=load(sys.argv[1]); b=load(sys.argv[2]); cs=open(sys.argv[3]).read()
raw=mer=0; ex=[]
for k in a:
    if k[1]!="compareN" or a[k]==b[k]: continue
    raw+=1
    if merged(a[k])!=merged(b[k]):
        mer+=1
        if len(ex)<4: ex.append((k[0],merged(a[k]),merged(b[k])))
print("raw call-list mismatches",raw,"merged-range (view DecorationComparator) mismatches",mer)
for e in ex: print(e)
