import sys, collections
def load(p):
    d = {}
    for line in open(p):
        line=line.rstrip("\n")
        if not line: continue
        cid, op, rest = (line.split(" ", 2)+[""])[:3]; d[(cid,op)] = rest
    return d
def cases(p):
    res = {}; cur=None
    for line in open(p):
        t=line.split()
        if not t: continue
        if t[0]=="CASE": cur=t[1]; res[cur]={"vals":{}, "set":0}; continue
        if t[0]=="S": res[cur]["set"]+=1; continue
        if len(t)==7 and t[2].startswith("n"): res[cur]["vals"][t[2]]=dict(f=int(t[0]),to=int(t[1]),s=int(t[3]),e=int(t[4]),p=t[5]=="1",mm=int(t[6]),set=res[cur]["set"])
        if t[0]=="C": res[cur]["C"]=list(map(int,t[1:]))
    return res
a=load(sys.argv[1]); b=load(sys.argv[2]); cs=cases(sys.argv[3])
cls=collections.Counter(); ex={}
for k in a:
    if k[1]!="map" or a[k]==b[k] or sorted(a[k].split())==sorted(b[k].split()): continue
    sa=set(a[k].split()); sb=set(b[k].split())
    for tok in sorted(sb-sa)+sorted(sa-sb):
        side = "ferrum-only" if tok in sb else "cm6-only"
        rng,name=tok.split(":"); f,t=map(int,rng.split("-")); v=cs[k[0]]["vals"][name]
        if v["set"]!=1: cls["(not set0?)"]+=1
        if side=="ferrum-only" and f==t and v["e"]<v["s"]: c="R5 empty result, endSide<startSide (cm6 drops at rangeset.ts:151)"
        elif side=="ferrum-only" and f==t: c="ferrum keeps empty result (other: cover-drop R4 or side rule)"
        else: c=side+" other"
        cls[c]+=1; ex.setdefault(c,(k[0],tok,v))
for c,n in cls.most_common(): print(n, c, "example:", ex.get(c))
