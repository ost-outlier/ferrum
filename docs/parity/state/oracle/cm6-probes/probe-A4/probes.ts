import {Text, EditorState, EditorSelection, Facet, StateField, StateEffect, Transaction, RangeSet, RangeValue, RangeSetBuilder,
        ChangeSet, CharCategory, findClusterBreak, countColumn, findColumn, Annotation} from "../src/index.ts"
const tryit = (f: () => any) => { try { return JSON.stringify(f()) } catch (e: any) { return "THROWS " + e.constructor.name + ": " + e.message } }

// P0 crate README quickstart equivalent
console.log("P0", tryit(() => {
  let s = EditorState.create({doc: "Hello, world!", selection: EditorSelection.cursor(5)})
  let tr = s.update({changes: {from: 5, to: 12, insert: " Ferrum"}, selection: EditorSelection.cursor(14)})
  return [tr.state.doc.toString(), tr.state.selection.main.head]
}))
// P1 line breaks
for (let s of ["a b", "a\u0085b", "a\u000bb", "a\u000cb", "a\rb", "a\r\nb", "a\nb"]) {
  let st = EditorState.create({doc: s})
  console.log("P1", JSON.stringify(s), "len=" + st.doc.length, "lines=" + st.doc.lines, "line1.text=" + JSON.stringify(st.doc.line(1).text), "line1.to=" + st.doc.line(1).to)
}
// P2
let d2 = EditorState.create({doc: "😀a"}).doc
console.log("P2 len=" + d2.length)
// P3
console.log("P3 line(0)", tryit(() => Text.of(["a", "b"]).line(0)))
// P4 cluster
const cases: [string, number][] = [["\r\n", 0], ["éx", 0], ["👨‍👩‍👧‍👦x", 0], ["🇩🇪🇫🇷", 0], ["👍🏽x", 0], ["각x", 0], ["क्षa", 0], ["؀a", 0], ["a‍b", 0]]
for (let [s, p] of cases) console.log("P4", JSON.stringify(s), "fcb fwd=" + findClusterBreak(s, p), "utf16len=" + s.length)
console.log("P4b fam back", findClusterBreak("👨‍👩‍👧‍👦", "👨‍👩‍👧‍👦".length, false))
// P5 categorizer
let cat = EditorState.create({}).charCategorizer(0)
for (let c of ["﻿", " ", "​", " ", "᠎", "_", "é", "٣", "́", "$", "中"]) console.log("P5", JSON.stringify(c), CharCategory[cat(c)])
// P7 columns
console.log("P7", countColumn("😀\tx", 4), findColumn("😀a", 1, 4), findColumn("ab", 5, 4, true), countColumn("\tx", 0))
// P8 selection
console.log("P8", tryit(() => EditorSelection.create([], 0)))
let s8 = EditorSelection.create([EditorSelection.range(6,7), EditorSelection.range(7,8), EditorSelection.cursor(3), EditorSelection.cursor(3), EditorSelection.range(10,12), EditorSelection.range(13,11)], 5)
console.log("P8b", JSON.stringify(s8.ranges.map(r => [r.anchor, r.head])), "main=" + s8.mainIndex)
console.log("P8c", tryit(() => EditorSelection.create([EditorSelection.cursor(1)], 7).mainIndex))
// P9 readOnly
console.log("P9", EditorState.create({extensions: [EditorState.readOnly.of(false), EditorState.readOnly.of(true)]}).readOnly)
// P10 allowMultipleSelections
console.log("P10", EditorState.create({doc: "abcdef", selection: EditorSelection.create([EditorSelection.cursor(1), EditorSelection.cursor(4)])}).selection.ranges.length)
// P11 lineSeparator
console.log("P11", EditorState.create({doc: "a|b", extensions: EditorState.lineSeparator.of("|")}).doc.lines)
// P12 duplicate extension
let f = Facet.define<number>(); let e = f.of(1)
console.log("P12", JSON.stringify(EditorState.create({extensions: [e, e]}).facet(f)))
// P13 dynamic facet depending on another
let a = Facet.define<number>(), b = Facet.define<number>()
console.log("P13", tryit(() => EditorState.create({extensions: [a.compute([b], s => s.facet(b).length), b.compute([], () => 5)]}).facet(a)))
// P14 annotation order
let st = EditorState.create({})
console.log("P14", st.update({annotations: [Transaction.userEvent.of("a"), Transaction.userEvent.of("b")]}).annotation(Transaction.userEvent))
// P15
let tr15 = st.update({}); console.log("P15", tr15.state === tr15.state)
// P16 between with big side
class Side extends RangeValue { constructor(public s: number, public e: number, public p: boolean) { super(); this.startSide = s; this.endSide = e; this.point = p } eq(o: Side) { return o.s == this.s && o.e == this.e && o.p == this.p } }
let set16 = RangeSet.of([new Side(2000000, 0, false).range(5, 6)])
let hits: number[][] = []; set16.between(0, 5, (f, t) => { hits.push([f, t]) }); console.log("P16", JSON.stringify(hits))
// P17 map side -256
let cs = ChangeSet.of({from: 2, insert: "xx"}, 4)
for (let side of [-256, -1]) {
  let set17 = RangeSet.of([new Side(side, side, true).range(2, 2)]).map(cs)
  let out: number[][] = []; set17.between(0, 10, (f, t) => { out.push([f, t]) }); console.log("P17 side", side, JSON.stringify(out))
}
// P18 builder unsorted
console.log("P18", tryit(() => { let bld = new RangeSetBuilder<Side>(); bld.add(5, 6, new Side(0, 0, false)); bld.add(1, 2, new Side(0, 0, false)); return bld.finish().size }))
// P21 changeByRange effects mapping
let posEffect = StateEffect.define<number>({map: (v, m) => m.mapPos(v)})
let st21 = EditorState.create({doc: "abcdef", selection: EditorSelection.create([EditorSelection.cursor(1), EditorSelection.cursor(4)]), extensions: EditorState.allowMultipleSelections.of(true)})
let r21 = st21.changeByRange(r => ({range: EditorSelection.cursor(r.from + 2), changes: {from: r.from, insert: "XX"}, effects: posEffect.of(r.from)}))
console.log("P21 effects", JSON.stringify(r21.effects.map(e => e.value)))
// P22 replaceSelection assoc
let st22 = EditorState.create({doc: "abc"}); console.log("P22 assoc", (st22.replaceSelection("x").selection as any).main.assoc)
