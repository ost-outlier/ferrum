// Random EditorSelection.create + EditorSelection.map cases. Seed 0x5E1C, mulberry32. Usage: tsx probe-C3/selgen.ts <dir>
import {EditorSelection, ChangeSet} from "../src/index.ts"
import {writeFileSync} from "fs"
function mulberry32(a: number) { return function () { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296 } }
const rnd = mulberry32(0x5E1C), int = (n: number) => Math.floor(rnd() * n)
let cases: string[] = [], res: string[] = []
const fmt = (s: EditorSelection) => s.ranges.map(r => r.anchor + "/" + r.head).join(",") + " main=" + s.mainIndex
const fmtA = (s: EditorSelection) => s.ranges.map(r => r.assoc).join(",")
for (let i = 0; i < 5000; i++) {
  let n = 1 + int(6), rs: [number, number][] = []
  for (let j = 0; j < n; j++) { let a = int(20), h = rnd() < 0.4 ? a : int(20); rs.push([a, h]) }
  let main = int(n)
  // change: one spec on doc length 20
  let from = int(21), to = Math.min(20, from + (rnd() < 0.5 ? 0 : int(5))), ins = rnd() < 0.3 && to > from ? 0 : int(4)
  let assoc = rnd() < 0.5 ? -1 : 1
  cases.push(`${i}\t${rs.map(r => r.join("/")).join(",")}\t${main}\t${from},${to},${ins}\t${assoc}`)
  let sel = EditorSelection.create(rs.map(([a, h]) => EditorSelection.range(a, h)), main)
  let cs = ChangeSet.of({from, to, insert: "x".repeat(ins)}, 20)
  let mapped = sel.map(cs.desc, assoc)
  res.push(`${i}\tcreate:${fmt(sel)}\tmap:${fmt(mapped)}\tassocCreate:${fmtA(sel)}\tassocMap:${fmtA(mapped)}`)
}
writeFileSync(process.argv[2] + "/sel-cases.txt", cases.join("\n") + "\n")
writeFileSync(process.argv[2] + "/sel-cm6.txt", res.join("\n") + "\n")
