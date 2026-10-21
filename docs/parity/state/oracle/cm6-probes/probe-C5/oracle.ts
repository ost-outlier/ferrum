// Differential oracle for EditorState.changeByRange / replaceSelection (chunk C5).
// Usage: tsx probe-C5/oracle.ts gen <n> <seed> <casesFile> <nonAscii 0|1>   -> writes cases + cm6 results (<casesFile>.cm6)
//        tsx probe-C5/oracle.ts cmp <casesFile>                              -> compares <casesFile>.cm6 with <casesFile>.ferrum
import {EditorState, EditorSelection} from "../src/index.ts"
import * as fs from "fs"

function mulberry32(a: number) { return () => { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296 } }
const esc = (s: string) => s.replace(/\\/g, "\\\\").replace(/\n/g, "\\n").replace(/\t/g, "\\t")

const cp = (s: string, pos: number) => Array.from(s.slice(0, pos)).length
function runCase(op: string, doc: string, ranges: [number, number][], main: number, text: string, conv = false) {
  let state = EditorState.create({doc, selection: EditorSelection.create(ranges.map(([a, h]) => EditorSelection.range(a, h)), main),
                                  extensions: EditorState.allowMultipleSelections.of(true)})
  let len = state.doc.length
  let spec: any
  if (op == "rs") spec = state.replaceSelection(text)
  else if (op == "cbr1") spec = state.changeByRange(r => { let n = (r.from + 2 * r.to) % 3; return {changes: {from: r.from, to: r.to, insert: "Z".repeat(n)}, range: EditorSelection.range(r.from, r.from + n)} })
  else if (op == "cbr2") spec = state.changeByRange(r => { let f = Math.max(0, r.from - 1); return {changes: {from: f, to: Math.min(len, r.to + 1), insert: "W"}, range: EditorSelection.cursor(f + 1)} })
  else if (op == "cbr3") spec = state.changeByRange(r => ({range: EditorSelection.range(r.head, r.anchor)}))
  else spec = state.changeByRange(r => ({changes: [{from: r.from, insert: "<"}, {from: r.to, insert: ">"}], range: EditorSelection.range(r.from + 1, r.to + 1)}))
  let next = state.update(spec).state
  if (conv) { let nd = next.doc.toString()
    return [esc(nd), next.selection.ranges.map(r => cp(nd, r.anchor) + ":" + cp(nd, r.head)).join(","), next.selection.mainIndex, "-",
            state.selection.ranges.map(r => cp(doc, r.anchor) + ":" + cp(doc, r.head)).join(",")].join("\t") }
  return [esc(next.doc.toString()), next.selection.ranges.map(r => r.anchor + ":" + r.head).join(","), next.selection.mainIndex, spec.changes.desc.toString(),
          state.selection.ranges.map(r => r.anchor + ":" + r.head).join(",")].join("\t")
}

let [mode, ...args] = process.argv.slice(2)
if (mode == "gen") {
  let [n, seed, file, nonAscii] = [+args[0], +args[1], args[2], args[3] == "1"]
  let rnd = mulberry32(seed), pick = (s: string) => { let a = Array.from(s); return a[Math.floor(rnd() * a.length)] }
  let alpha = nonAscii ? "ab \né😀" : "ab \n", talpha = nonAscii ? "x\n😀" : "xy\n"
  let ops = nonAscii ? ["rs", "cbr3"] : ["rs", "cbr1", "cbr2", "cbr3", "cbr4"]
  let cases: string[] = [], outs: string[] = []
  for (let i = 0; i < n; i++) {
    let docArr: string[] = []; let dl = Math.floor(rnd() * 20)
    for (let j = 0; j < dl; j++) docArr.push(pick(alpha))
    let doc = docArr.join("")
    // positions must be valid in both units: pick positions on code-point boundaries, emitted in both units
    let cpBoundsU16: number[] = [0], acc = 0
    for (let ch of docArr) { acc += ch.length; cpBoundsU16.push(acc) }
    let k = 1 + Math.floor(rnd() * 4), rangesCp: [number, number][] = [], rangesU16: [number, number][] = []
    for (let j = 0; j < k; j++) { let a = Math.floor(rnd() * (dl + 1)), h = Math.floor(rnd() * (dl + 1)); rangesCp.push([a, h]); rangesU16.push([cpBoundsU16[a], cpBoundsU16[h]]) }
    let main = Math.floor(rnd() * k), op = ops[Math.floor(rnd() * ops.length)]
    let tl = Math.floor(rnd() * 4), text = ""; for (let j = 0; j < tl; j++) text += pick(talpha)
    cases.push([op, esc(doc), rangesCp.map(r => r.join(":")).join(","), main, esc(text)].join("\t"))
    let out: string
    try { out = runCase(op, doc, rangesU16, main, text, nonAscii) } catch (e: any) { out = "THROWS " + e.message }
    outs.push(out)
  }
  fs.writeFileSync(file, cases.join("\n") + "\n"); fs.writeFileSync(file + ".cm6", outs.join("\n") + "\n")
} else {
  let file = args[0]
  let cases = fs.readFileSync(file, "utf8").trimEnd().split("\n"), a = fs.readFileSync(file + ".cm6", "utf8").trimEnd().split("\n"), b = fs.readFileSync(file + ".ferrum", "utf8").trimEnd().split("\n")
  let mism = 0, byOp: Record<string, number> = {}, firstByOp: Record<string, string> = {}
  for (let i = 0; i < cases.length; i++) if (a[i] != b[i]) {
    mism++; let op = cases[i].split("\t")[0]; byOp[op] = (byOp[op] || 0) + 1
    if (!firstByOp[op] || cases[i].length < firstByOp[op].split("\n")[0].length) firstByOp[op] = cases[i] + "\n   cm6:    " + a[i] + "\n   ferrum: " + b[i]
  }
  console.log(`cases=${cases.length} mismatches=${mism} byOp=${JSON.stringify(byOp)}`)
  for (let op in firstByOp) console.log(`shortest mismatch [${op}]:\n   case:   ${firstByOp[op]}`)
}
