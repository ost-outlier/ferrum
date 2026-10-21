// Differential oracle for C4: extension flattening (precedence, nesting, dedup of shared objects).
// PRNG: mulberry32, seed given on argv (default 0xC4). Output: one line per case "<sexp>\t<cm6 result>".
import {EditorState, Facet, Prec, Extension} from "../src/index.ts"
import {writeFileSync} from "fs"
function mulberry32(a: number) { return function() { a |= 0; a = a + 0x6D2B79F5 | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296 } }
const seed = Number(process.argv[2] ?? 0xC4), N = Number(process.argv[3] ?? 2000), share = process.argv[4] == "share"
const rnd = mulberry32(seed)
const num = Facet.define<number>()
const precs = [Prec.highest, Prec.high, Prec.default, Prec.low, Prec.lowest]
let lines: string[] = []
for (let c = 0; c < N; c++) {
  let counter = 0, defs: {s: string, e: Extension}[] = []
  function gen(depth: number): {s: string, e: Extension} {
    let r = rnd()
    if (share && defs.length && r < 0.15) { let k = Math.floor(rnd() * defs.length); return {s: `(r ${k})`, e: defs[k].e} }
    let node: {s: string, e: Extension}
    if (depth > 3 || r < 0.45) { let v = counter++; node = {s: `v${v}`, e: num.of(v)} }
    else if (r < 0.7) { let p = Math.floor(rnd() * 5); let inner = gen(depth + 1); node = {s: `(p ${p} ${inner.s})`, e: precs[p](inner.e)} }
    else { let n = 1 + Math.floor(rnd() * 4); let kids = Array.from({length: n}, () => gen(depth + 1)); node = {s: `(g ${kids.map(k => k.s).join(" ")})`, e: kids.map(k => k.e)} }
    if (share && rnd() < 0.3) { defs.push(node); return {s: `(d ${defs.length - 1} ${node.s})`, e: node.e} }
    return node
  }
  let root = gen(0)
  let out: string
  try { out = EditorState.create({extensions: root.e}).facet(num).join(",") } catch (e: any) { out = "ERR " + e.message }
  lines.push(root.s + "\t" + out)
}
writeFileSync(process.argv[5], lines.join("\n") + "\n")
