// Differential oracle for RangeSet (C6). Usage: tsx probe-C6/oracle.ts <seed> <iterations> <sideMode:small|big> <casesOut> <cm6ResultsOut>
import {ChangeSet, MapMode, Range, RangeSet, RangeValue} from "../src/index.ts"
import {writeFileSync} from "fs"

const [seedArg, itArg, sideMode, casesOut, resOut] = process.argv.slice(2)
let seed = Number(seedArg) >>> 0
function rnd() { // mulberry32
  seed = (seed + 0x6D2B79F5) >>> 0
  let t = seed
  t = Math.imul(t ^ (t >>> 15), t | 1)
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296
}
const ri = (n: number) => Math.floor(rnd() * n)
const SMALL = [-2, -1, 0, 1, 2]
const BIG = [-600000000 + 1, -600000000, -400000000, -200000000, -100000000, -1, 0, 1, 100000001, 300000000, 500000000 - 1, 500000000]
const sides = sideMode == "big" ? BIG : SMALL

class V extends RangeValue {
  constructor(readonly name: string, public startSide: number, public endSide: number, public point: boolean, public mapMode: MapMode) { super() }
  eq(o: RangeValue) { return o instanceof V && o.name == this.name }
}
let nameCounter = 0
function rv(L: number): Range<V> {
  let from = ri(L + 1), empty = rnd() < 0.25, to = empty ? from : Math.min(L, from + 1 + ri(8))
  let point = empty ? rnd() < 0.85 : rnd() < 0.25
  let s = sides[ri(sides.length)], e = rnd() < 0.3 ? s : sides[ri(sides.length)]
  return new V("n" + (nameCounter++), s, e, point, ri(4) as MapMode).range(from, to)
}
const cmpRange = (a: Range<V>, b: Range<V>) => a.from - b.from || a.value.startSide - b.value.startSide
const fmtR = (r: {from: number, to: number, value: V}) => `${r.from} ${r.to} ${r.value.name} ${r.value.startSide} ${r.value.endSide} ${r.value.point ? 1 : 0} ${r.value.mapMode}`
const iterStr = (set: RangeSet<V>) => { let o: string[] = []; for (let c = set.iter(); c.value; c.next()) o.push(`${c.from}-${c.to}:${c.value.name}`); return o.join(" ") }
const names = (a: readonly V[]) => a.map(v => v.name).join(",")

let cases: string[] = [], results: string[] = []
let skipped = 0
const N = Number(itArg)
for (let id = 0; id < N; id++) {
  let L = 10 + ri(40)
  let nSets = 1 + ri(3)
  let setsRanges: Range<V>[][] = []
  for (let k = 0; k < nSets; k++) {
    let n = ri(k == 0 ? 30 : 10)
    let rs: Range<V>[] = []; for (let i = 0; i < n; i++) rs.push(rv(L))
    setsRanges.push(rs.slice().sort(cmpRange))
  }
  // changes
  let specs: any[] = [], nc = ri(4)
  for (let i = 0; i < nc; i++) { let a = ri(L + 1), b = Math.min(L, a + ri(6)); specs.push({from: a, to: rnd() < 0.5 ? a : b, insert: "x".repeat(ri(4))}) }
  let changes: ChangeSet
  try { changes = ChangeSet.of(specs, L) } catch (e) { changes = ChangeSet.empty(L) }
  // update spec
  let add: Range<V>[] = []; let na = ri(6); for (let i = 0; i < na; i++) add.push(rv(L)); add.sort(cmpRange)
  let fFrom = ri(L + 1), fTo = fFrom + ri(L + 1 - fFrom), fMod = ri(4) // 0 = no filter
  let bFrom = ri(L + 1), bTo = bFrom + ri(L + 1 - bFrom)
  let sFrom = ri(L + 1), sTo = sFrom + ri(L + 1 - sFrom), minPS = [-1, -1, 0, 2][ri(4)], cMinPS = -1

  let sets: RangeSet<V>[]
  try { sets = setsRanges.map(rs => RangeSet.of(rs)) } catch (e) { skipped++; continue }
  let lines = [`CASE ${id} ${L}`]
  for (let rs of setsRanges) { lines.push(`S ${rs.length}`); for (let r of rs) lines.push(fmtR(r)) }
  lines.push(`C ${changes.desc.toJSON().join(" ")}`)
  lines.push(`U ${fFrom} ${fTo} ${fMod} ${add.length}`); for (let r of add) lines.push(fmtR(r))
  lines.push(`B ${bFrom} ${bTo}`, `P ${sFrom} ${sTo} ${minPS}`, `Q ${cMinPS}`)

  const filter = fMod ? ((_f: number, _t: number, v: V) => Number(v.name.slice(1)) % fMod != 0) : undefined
  const out = (op: string, s: string) => results.push(`${id} ${op} ${s}`)
  sets.forEach((s, k) => out(`iter${k}`, iterStr(s)))
  sets.forEach((s, k) => out(`size${k}`, String(s.size)))
  { let f: string[] = []; sets[0].between(bFrom, bTo, (a, b, v) => { f.push(`${a}-${b}:${v.name}`) }); out("between", f.sort().join(" ")) }
  let mapped = sets.map(s => s.map(changes))
  out("map", iterStr(mapped[0]))
  let updated: RangeSet<V> | null = null
  try { updated = sets[0].update({add, filter, filterFrom: fFrom, filterTo: fTo}); out("update", iterStr(updated)) } catch (e: any) { out("update", "THROWS") }
  { let calls: string[] = []
    let ret = RangeSet.spans(sets, sFrom, sTo, {
      span(f, t, a, o) { calls.push(`s(${f},${t},[${names(a)}],${o})`) },
      point(f, t, v, a, o, i) { calls.push(`p(${f},${t},${v.name},[${names(a)}],${o},${i})`) }
    }, minPS)
    out("spans", calls.join(" ") + ` ret=${ret}`) }
  { let newSets = mapped.slice(); if (updated) newSets[0] = updated.map(changes)
    let calls: string[] = []
    RangeSet.compare(sets, newSets, changes, {
      compareRange(f, t, a, b) { calls.push(`r(${f},${t},[${names(a)}],[${names(b)}])`) },
      comparePoint(f, t, a, b) { calls.push(`p(${f},${t},${a ? a.name : "-"},${b ? b.name : "-"})`) }
    }, cMinPS)
    out("compare", calls.join(" "))
    for (let ns of newSets) { let rs: string[] = []; for (let c = ns.iter(); c.value; c.next()) rs.push(fmtR({from: c.from, to: c.to, value: c.value})); lines.push(`N ${rs.length}`, ...rs) }
    out("compareN", calls.join(" ")) }
  lines.push("END"); cases.push(lines.join("\n"))
}
writeFileSync(casesOut, cases.join("\n") + "\n")
writeFileSync(resOut, results.join("\n") + "\n")
console.log(`cases ${cases.length}, skipped (cm6 RangeSet.of threw) ${skipped}`)
