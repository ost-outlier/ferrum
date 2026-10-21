import {ChangeSet, ChangeDesc, MapMode, Range, RangeSet, RangeSetBuilder, RangeValue, RangeComparator, SpanIterator} from "../src/index.ts"

class V extends RangeValue {
  constructor(readonly name: string, public startSide = 0, public endSide = 0, public point = false, public mapMode: MapMode = MapMode.TrackDel) { super() }
  eq(o: RangeValue) { return o instanceof V && o.name == this.name }
}
class Plain extends RangeValue { constructor(readonly name: string) { super() } } // default identity eq

const r = (from: number, to: number, v: RangeValue) => v.range(from, to)
const show = (set: RangeSet<any>) => { let out: string[] = []; for (let c = set.iter(); c.value; c.next()) out.push(`${c.from}-${c.to}:${c.value.name}`); return out.join(" ") }
const log = (id: string, v: any) => console.log(id, "=>", typeof v == "string" ? v : JSON.stringify(v))
const tryIt = (f: () => any) => { try { return f() } catch (e: any) { return "THROWS: " + e.message } }
const cs = (spec: any, len: number) => ChangeSet.of(spec, len)

class Comp implements RangeComparator<any> {
  calls: string[] = []
  compareRange(f: number, t: number, a: any[], b: any[]) { this.calls.push(`range(${f},${t},[${a.map(v => v.name)}],[${b.map(v => v.name)}])`) }
  comparePoint(f: number, t: number, a: any, b: any) { this.calls.push(`point(${f},${t},${a ? a.name : null},${b ? b.name : null})`) }
}
class CompB extends Comp { boundChange(p: number) { this.calls.push(`bound(${p})`) } }
class Spans implements SpanIterator<any> {
  calls: string[] = []
  span(f: number, t: number, a: readonly any[], o: number) { this.calls.push(`span(${f},${t},[${a.map(v => v.name)}],${o})`) }
  point(f: number, t: number, v: any, a: readonly any[], o: number, i: number) { this.calls.push(`point(${f},${t},${v.name},[${a.map(v => v.name)}],${o},${i})`) }
}

// R1 iter order of tie from/startSide
{ let s = RangeSet.of([r(0, 5, new V("A")), r(0, 3, new V("B"))]); log("R1 iter", show(s)) }
// R2 between with big sides
{ let s = RangeSet.of([r(5, 5, new V("L", -2e8, -2e8, true))]); let f: string[] = []; s.between(5, 10, (a, b, v) => { f.push(`${a}-${b}:${v.name}`) }); log("R2a between(5,10) endSide=-2e8 at 5", f.join(" ")) }
{ let s = RangeSet.of([r(5, 5, new V("W", 1e8 + 1, 1e8 + 1, true))]); let f: string[] = []; s.between(0, 5, (a, b, v) => { f.push(`${a}-${b}:${v.name}`) }); log("R2b between(0,5) startSide=1e8+1 at 5", f.join(" ")) }
// R3 map with big sides
{ let s = RangeSet.of([r(5, 5, new V("Wbefore", -1e8, -1e8, true))]); log("R3a widget side -1e8 at 5, insert at 5", show(s.map(cs({from: 5, insert: "x"}, 10)))) }
{ let s = RangeSet.of([r(4, 6, new V("Repl", 5e8 - 1, -6e8 + 1, true))]); log("R3b replace(4,6) sides 5e8-1/-6e8+1, insert at 4 and 6", show(s.map(cs([{from: 4, insert: "x"}, {from: 6, insert: "y"}], 10)))) }
{ let s = RangeSet.of([r(4, 6, new V("Mark", 5e8, -6e8, false))]); log("R3c mark(4,6) sides 5e8/-6e8, insert at 6", show(s.map(cs({from: 6, insert: "y"}, 10)))) }
{ let s = RangeSet.of([r(4, 6, new V("S128", 128, -128, false))]); log("R3d range(4,6) sides 128/-128, insert at 4 and 6", show(s.map(cs([{from: 4, insert: "x"}, {from: 6, insert: "y"}], 10)))) }
// R4 map: chunk covered by deletion drops Simple-mode points
{ let s = RangeSet.of([r(5, 5, new V("P", 0, 0, true, MapMode.Simple))]); log("R4a Simple point at 5, delete 2..8", show(s.map(cs({from: 2, to: 8}, 10)))) }
{ let s = RangeSet.of([r(4, 6, new V("I", -1, 1, false))]); log("R4b inclusive range(4,6), delete 2..8", show(s.map(cs({from: 2, to: 8}, 10)))) }
{ let s = RangeSet.of([r(0, 1, new V("X", 0, 0)), r(4, 6, new V("I", -1, 1, false))]); log("R4c same + (0,1) in chunk, delete 2..8 (touch not cover)", show(s.map(cs({from: 2, to: 8}, 10)))) }
// R5 empty range with endSide < startSide in a touched chunk
{ let s = RangeSet.of([r(0, 1, new V("X")), r(3, 3, new V("E", 1, -1, true))]); log("R5 empty(3,3) sides 1/-1, delete 1..2", show(s.map(cs({from: 1, to: 2}, 10)))) }
// R6 structural sharing in map
{ let rs: Range<V>[] = []; for (let i = 0; i < 1000; i++) rs.push(r(i, i + 1, new V("v" + i))); let s = RangeSet.of(rs)
  let m = s.map(cs({from: 1000, insert: "x"}, 1001)); log("R6 map change after all chunks: chunk[0] shared", m.chunk[0] === s.chunk[0])
  let u = s.update({add: [r(999, 999, new V("p", 1, 1, true))]}); log("R6b update add at end: chunk[0] shared", u.chunk[0] === s.chunk[0]) }
// R7 update no-op identity
{ let s = RangeSet.of([r(1, 2, new V("a"))]); log("R7 update({}) === this", s.update({}) === s); log("R7b filter all -> RangeSet.empty", s.update({filter: () => false}) === RangeSet.empty) }
// R8 update unsorted add, sort false/true
{ let s = RangeSet.of([r(5, 6, new V("a"))]); log("R8a update add unsorted sort=false", tryIt(() => show(s.update({add: [r(8, 9, new V("c")), r(1, 2, new V("b"))]}))))
  log("R8b update add unsorted sort=false (overlap)", tryIt(() => show(s.update({add: [r(8, 9, new V("c")), r(7, 10, new V("b"))]}))))
  log("R8c update add unsorted sort=true", tryIt(() => show(s.update({add: [r(8, 9, new V("c")), r(1, 2, new V("b"))], sort: true}))))
  log("R8d empty set update add unsorted sort=false", tryIt(() => show(RangeSet.empty.update({add: [r(8, 9, new V("c")), r(1, 2, new V("b"))]})))) }
// R9 builder strict edge: empty last range with endSide < startSide
{ log("R9 of([(0,0,s1/e-1),(0,0,s-1/e-1)])", tryIt(() => show(RangeSet.of([r(0, 0, new V("a", 1, -1, true)), r(0, 0, new V("b", -1, -1, true))])))) }
// R10 builder add misordered
{ log("R10 builder.add(8,9) then add(7,10)", tryIt(() => { let b = new RangeSetBuilder<V>(); b.add(8, 9, new V("a")); b.add(7, 10, new V("b")); return show(b.finish()) })) }
// R11 spans openStart, single set
{ let s = RangeSet.of([r(0, 6, new V("A")), r(5, 10, new V("B"))]); let it = new Spans; let ret = RangeSet.spans([s], 5, 10, it); log("R11 spans([A(0,6),B(5,10)],5,10)", it.calls.join(" ") + " ret=" + ret) }
// R12 openEnd multi-rank
{ let s0 = RangeSet.of([r(0, 10, new V("A"))]), s1 = RangeSet.of([r(0, 6, new V("B"))]); let it = new Spans; let ret = RangeSet.spans([s0, s1], 0, 10, it); log("R12 spans([[A(0,10)],[B(0,6)]],0,10)", it.calls.join(" ") + " ret=" + ret)
  let it2 = new Spans; let ret2 = RangeSet.spans([s0, s1], 0, 8, it2); log("R12b same to=8", it2.calls.join(" ") + " ret=" + ret2) }
// R13 compare shared chunks
{ let s1 = RangeSet.of([r(2, 4, new V("a"))]), s2 = RangeSet.of([r(0, 10, new V("b"))]); let c = new Comp; RangeSet.compare([s1], [s1, s2], ChangeSet.empty(10), c); log("R13 compare([s1],[s1,s2])", c.calls.join(" ")) }
// R14 compare in empty doc
{ let s = RangeSet.of([r(0, 0, new V("w", 1, 1, true))]); let c = new Comp; RangeSet.compare([RangeSet.empty], [s], ChangeSet.empty(0), c); log("R14 compare empty doc point", c.calls.join(" ")) }
// R15 compare changed tail
{ let s = RangeSet.of([r(6, 7, new V("a"))]); let c = new Comp; RangeSet.compare([s], [RangeSet.empty], cs({from: 5, to: 10, insert: "xyz"}, 10), c); log("R15 compare tail change", c.calls.join(" ")) }
// R16 compare gap start, side -2e8
{ let s = RangeSet.of([r(0, 0, new V("L", -2e8, -2e8, true))]); let c = new Comp; RangeSet.compare([s], [RangeSet.empty], ChangeSet.empty(10), c); log("R16 compare point at 0 sides -2e8", c.calls.join(" ")) }
// R17 boundChange
{ let a = RangeSet.of([r(0, 5, new V("a")), r(5, 10, new V("a"))]), b = RangeSet.of([r(0, 10, new V("a"))]); let c = new CompB; RangeSet.compare([a], [b], ChangeSet.empty(10), c); log("R17 boundChange", c.calls.join(" ")); let c2 = new Comp; RangeSet.compare([a], [b], ChangeSet.empty(10), c2); log("R17b no boundChange method", c2.calls.join(" ") || "(no calls)") }
// R18 identical ranges heap tie
{ let s = RangeSet.of([r(0, 5, new V("a")), r(0, 5, new V("b"))]); let it = new Spans; RangeSet.spans([s], 0, 5, it); log("R18 spans identical ranges", it.calls.join(" ")); log("R18 iter", show(s)) }
// R19 LayerCursor forward with chunk start > pos
{ let pts: Range<V>[] = []; for (let i = 0; i < 250; i++) pts.push(r(1, 1, new V("q", 0, 0, true))); pts.push(r(5, 5, new V("X", 0, 0, true))); pts.push(r(6, 6, new V("Y", 0, 0, true)))
  let b = RangeSet.of(pts), a = RangeSet.of([r(0, 2, new V("P", 1, 1, true))]); let it = new Spans; RangeSet.spans([a, b], 0, 10, it); log("R19 spans forward", it.calls.join(" ")) }
// R20 default eq identity
{ let s1 = RangeSet.of([r(0, 5, new Plain("p"))]), s2 = RangeSet.of([r(0, 5, new Plain("p"))]); let c = new Comp; RangeSet.compare([s1], [s2], ChangeSet.empty(10), c); log("R20 default eq (distinct instances same data)", c.calls.join(" ") || "(no calls)") }
// R21 iter(from)
{ let s = RangeSet.of([r(0, 2, new V("a")), r(3, 4, new V("b")), r(5, 9, new V("c"))]); let out: string[] = []; for (let c = s.iter(4); c.value; c.next()) out.push(c.value.name); log("R21 iter(4)", out.join(",")) }
// R22 cm6-only APIs
{ let s1 = RangeSet.of([r(0, 2, new V("a"))]), s2 = RangeSet.of([r(1, 3, new V("b"))])
  let j = RangeSet.join([s1, s2]); log("R22a join size/iter", j.size + " " + show(j))
  let out: string[] = []; for (let c = RangeSet.iter([s1, s2], 0); c.value; c.next()) out.push(`${c.from}-${c.to}:${c.value.name}@${c.rank}`); log("R22b static iter", out.join(" "))
  let c = RangeSet.iter([s1, s2]); c.goto(2); log("R22c goto(2)", `${c.from}-${c.to}:${c.value?.name}`)
  log("R22d eq same content", RangeSet.eq([s1], [RangeSet.of([r(0, 2, new V("a"))])])); log("R22e eq diff", RangeSet.eq([s1], [s2])); log("R22f eq range-limited", RangeSet.eq([RangeSet.of([r(0, 2, new V("a")), r(5, 6, new V("x"))])], [RangeSet.of([r(0, 2, new V("a")), r(5, 6, new V("y"))])], 0, 3))
  log("R22g value.range(3)", JSON.stringify(new V("z").range(3)))
  log("R22h RangeSet.of(single Range)", show(RangeSet.of(r(1, 2, new V("one"))))) }
// R23 compare minPointSize
{ let a = RangeSet.of([r(0, 2, new V("small", 0, 0, true)), r(3, 9, new V("m"))]), b = RangeSet.empty; let c = new Comp; RangeSet.compare([a], [b], ChangeSet.empty(10), c, 3); log("R23 compare minPointSize=3", c.calls.join(" ") || "(no calls)"); let c2 = new Comp; RangeSet.compare([a], [b], ChangeSet.empty(10), c2); log("R23b minPointSize=-1", c2.calls.join(" ")) }
// R24 spans minPointSize
{ let s = RangeSet.of([r(0, 2, new V("small", 0, 0, true)), r(3, 9, new V("m")), r(5, 8, new V("big", 0, 0, true))]); let it = new Spans; let ret = RangeSet.spans([s], 0, 10, it, 3); log("R24 spans minPointSize=3", it.calls.join(" ") + " ret=" + ret) }
// R25 size, length
{ let s = RangeSet.of([r(0, 5, new V("A")), r(0, 3, new V("B"))]); log("R25 size", s.size); log("R25 length(internal)", (s as any).length) }
// R26 point openStart, point starting before from
{ let s = RangeSet.of([r(0, 10, new V("A")), r(2, 6, new V("P", 0, 0, true))]); let it = new Spans; let ret = RangeSet.spans([s], 4, 8, it); log("R26 spans from inside point", it.calls.join(" ") + " ret=" + ret) }
// R27 between stops on false, ordering
{ let s = RangeSet.of([r(0, 5, new V("A")), r(0, 3, new V("B")), r(6, 7, new V("C"))]); let f: string[] = []; s.between(0, 10, (a, b, v) => { f.push(v.name); if (v.name == "A") return false }); log("R27 between stop", f.join(",")); let g: string[] = []; s.between(0, 10, (a, b, v) => { g.push(v.name) }); log("R27b between order", g.join(",")) }
// R28 map through empty / map identity
{ let s = RangeSet.of([r(1, 2, new V("a"))]); log("R28 map(empty changes) === this", s.map(ChangeSet.empty(10)) === s) }
// R29 filterFrom/filterTo touching boundaries
{ let s = RangeSet.of([r(0, 2, new V("a")), r(2, 4, new V("b")), r(4, 6, new V("c")), r(6, 8, new V("d"))]); let called: string[] = []; let u = s.update({filter: (f, t, v) => (called.push(v.name), false), filterFrom: 4, filterTo: 4}); log("R29 filter touching 4..4", called.join(",") + " -> " + show(u)) }
// R30 RangeSet.eq only considers points (minPoint 0)
{ let p1 = RangeSet.of([r(0, 2, new V("a", 0, 0, true))]), p2 = RangeSet.of([r(0, 2, new V("b", 0, 0, true))]); log("R30 eq points differ", RangeSet.eq([p1], [p2])); log("R30b eq marks differ", RangeSet.eq([RangeSet.of([r(0, 2, new V("a"))])], [RangeSet.of([r(0, 2, new V("b"))])])) }
