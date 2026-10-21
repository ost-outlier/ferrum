// Differential oracle generator for C1 (Text). Usage: tsx probe-C1/oracle.ts <seed> <iterations> <mode ascii|nonascii> <out.json>
import {Text} from "../src/index.ts"
import {writeFileSync} from "fs"
const [seedS, itersS, mode, out] = process.argv.slice(2)
let seed = Number(seedS) >>> 0
function rnd() { // mulberry32
  seed = (seed + 0x6D2B79F5) >>> 0
  let t = seed
  t = Math.imul(t ^ (t >>> 15), t | 1)
  t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296
}
const int = (n: number) => Math.floor(rnd() * n)
const alpha = ({ascii: "abcdefgh xyz", nonascii: "ab\u00e9\u4e2d\u{1F600}\u2028", bmp: "ab\u00e9\u4e2d", emoji: "ab\u{1F600}", u2028: "ab\u2028", cr: "ab\r"} as any)[mode]
const alphaArr = Array.from(alpha)
function str(max: number) { let s = ""; let n = int(max); for (let i = 0; i < n; i++) s += alphaArr[int(alphaArr.length)]; return s }
function linesArr(maxLines: number, maxLen: number) { let n = 1 + int(maxLines); let r: string[] = []; for (let i = 0; i < n; i++) r.push(str(maxLen)); return r }
let loneSurrogates = 0
// serde_json rejects lone surrogates; cm6 can produce them when a UTF-16 offset splits a pair. Replace with U+FFFD and count.
const W = (s: string) => { let w = (s as any).toWellFormed(); if (w != s) loneSurrogates++; return w }
const L = (l: any) => [l.from, l.to, l.number, W(l.text)]
const cases: any[] = []
const iters = Number(itersS)
for (let c = 0; c < iters; c++) {
  let init = linesArr(c % 10 == 0 ? 300 : 40, 30)
  let doc = Text.of(init)
  let ops: any[] = []
  for (let k = 0; k < 8; k++) {
    let len = doc.length
    let from = int(len + 3), to = int(len + 3)
    if (rnd() < 0.7 && from > to) [from, to] = [to, from]
    let ins = rnd() < 0.3 ? [""] : linesArr(4, 10)
    doc = doc.replace(from, to, Text.of(ins))
    let probes: any[] = []
    for (let p = 0; p < 6; p++) {
      let pos = int(doc.length + 1)
      let n = 1 + int(doc.lines)
      let a = int(doc.length + 3), b = int(doc.length + 3)
      probes.push({pos, lineAt: L(doc.lineAt(pos)), n, line: L(doc.line(n)), a, b, sliceString: W(doc.sliceString(a, b)), slice: W(doc.slice(a, b).toString()), sliceLines: doc.slice(a, b).lines})
    }
    ops.push({from, to, ins, length: doc.length, lines: doc.lines, text: W(doc.toString()), json: doc.toJSON().map(W), probes})
  }
  cases.push({init, ops})
}
writeFileSync(out, JSON.stringify(cases))
console.log("wrote", cases.length, "cases; strings with lone surrogates replaced:", loneSurrogates)
