import {ChangeDesc, MapMode, RangeSet, RangeValue} from "../src/index.ts"
import {readFileSync} from "fs"
class V extends RangeValue { constructor(readonly name: string, public startSide: number, public endSide: number, public point: boolean, public mapMode: MapMode) { super() } eq(o: any) { return o.name == this.name } }
let lines = readFileSync(process.argv[2], "utf8").split("\n"), rs: any[] = [], i = 2, n = +lines[1].split(" ")[1]
for (let k = 0; k < n; k++) { let t = lines[i++].split(" "); rs.push(new V(t[2], +t[3], +t[4], t[5] == "1", +t[6]).range(+t[0], +t[1])) }
let ch = ChangeDesc.fromJSON([7, -1, 5, 1, 1, -1])
function survivors(rs: any[]) { // per-range cm6 Chunk.map rules (no cover)
  let out: string[] = []
  for (let r of rs) { let v = r.value, nf, nt
    if (r.from == r.to) { let m = ch.mapPos(r.from, v.startSide, v.mapMode); if (m == null) continue; nf = nt = m; if (v.startSide != v.endSide) { nt = ch.mapPos(r.from, v.endSide); if (nt < nf) continue } }
    else { nf = ch.mapPos(r.from, v.startSide); nt = ch.mapPos(r.to, v.endSide); if (nf > nt || nf == nt && v.startSide > 0 && v.endSide <= 0) continue }
    if ((nt - nf || v.endSide - v.startSide) < 0) continue
    out.push(v.name) }
  return out
}
function lost(rs: any[]) { let m = RangeSet.of(rs).map(ch); let got = new Set<string>(); m.between(0, 1000, (f, t, v: any) => { got.add(v.name) }); return survivors(rs).filter(x => !got.has(x)) }
let cur = rs.slice()
for (let changed = true; changed;) { changed = false
  for (let j = 0; j < cur.length; j++) { let tryR = cur.slice(0, j).concat(cur.slice(j + 1)); if (lost(tryR).length) { cur = tryR; changed = true; break } } }
console.log("minimal input:"); for (let r of cur) console.log(`  ${r.from}-${r.to} ${r.value.name} s=${r.value.startSide} e=${r.value.endSide} point=${r.value.point} mm=${r.value.mapMode}`)
console.log("lost:", lost(cur)); let m = RangeSet.of(cur).map(ch); let it: string[] = []; for (let c = m.iter(); c.value; c.next()) it.push(`${c.from}-${c.to}:${(c.value as any).name}`); console.log("cm6 mapped:", it.join(" "), "size", m.size)
