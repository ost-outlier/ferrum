import {ChangeDesc, MapMode, RangeSet, RangeValue} from "../src/index.ts"
import {readFileSync} from "fs"
class V extends RangeValue { constructor(readonly name: string, public startSide: number, public endSide: number, public point: boolean, public mapMode: MapMode) { super() } eq(o: any) { return o.name == this.name } }
let lines = readFileSync(process.argv[2], "utf8").split("\n"), rs: any[] = [], i = 2, n = +lines[1].split(" ")[1]
for (let k = 0; k < n; k++) { let t = lines[i++].split(" "); rs.push(new V(t[2], +t[3], +t[4], t[5] == "1", +t[6]).range(+t[0], +t[1])) }
let set = RangeSet.of(rs), ch = ChangeDesc.fromJSON([7, -1, 5, 1, 1, -1])
let m = set.map(ch)
console.log("orig size", set.size, "mapped size", m.size)
let b: string[] = []; m.between(0, 100, (f, t, v: any) => { b.push(`${f}-${t}:${v.name}`) }); console.log("between", b.join(" "))
let it: string[] = []; for (let c = m.iter(); c.value; c.next()) it.push(`${c.from}-${c.to}:${(c.value as any).name}`); console.log("iter   ", it.join(" "))
for (let l = m, d = 0; l.size; l = (l as any).nextLayer, d++) { let s: string[] = []; for (let c = 0; c < l.chunk.length; c++) { let ch = l.chunk[c]; for (let j = 0; j < ch.value.length; j++) s.push(`${l.chunkPos[c] + ch.from[j]}-${l.chunkPos[c] + ch.to[j]}:${(ch.value[j] as any).name}(${(ch.value[j] as any).startSide},${(ch.value[j] as any).endSide})`) } console.log("layer", d, s.join(" ")) }
