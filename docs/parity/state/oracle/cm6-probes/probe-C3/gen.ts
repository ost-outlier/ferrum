// Generates the shared case file (one string per line as space-separated hex code points, prefixed by an id)
// and computes cm6 results for findClusterBreak / countColumn / findColumn in CODE POINT units.
// Usage: tsx probe-C3/gen.ts <outdir>
import {findClusterBreak, countColumn, findColumn} from "../src/index.ts"
import {writeFileSync} from "fs"

const out = process.argv[2]

// mulberry32 PRNG, seed fixed
function mulberry32(a: number) {
  return function () {
    a |= 0; a = a + 0x6D2B79F5 | 0
    let t = Math.imul(a ^ a >>> 15, 1 | a)
    t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t
    return ((t ^ t >>> 14) >>> 0) / 4294967296
  }
}

const table: [string, string][] = [
  ["T01 plain", "ab"],
  ["T02 CRLF", "a\r\nb"],
  ["T03 CR CR LF", "\r\r\n"],
  ["T04 LF CR", "\n\r"],
  ["T05 combining acute", "éx"],
  ["T06 two combining", "é̂"],
  ["T07 leading combining", "̈a"],
  ["T08 ZWJ family", "\u{1F468}‍\u{1F469}‍\u{1F467}‍\u{1F466}"],
  ["T09 ZWJ between letters", "a‍b"],
  ["T10 trailing ZWJ", "x‍"],
  ["T11 double ZWJ", "‍‍"],
  ["T12 flags FR DE", "\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}"],
  ["T13 three RI", "\u{1F1EB}\u{1F1F7}\u{1F1E9}"],
  ["T14 skin tone", "\u{1F44D}\u{1F3FD}"],
  ["T15 ZWJ + modifier (woman technologist medium)", "\u{1F469}\u{1F3FD}‍\u{1F4BB}"],
  ["T16 Hangul precomposed", "한글"],
  ["T17 Hangul jamo L V T", "각"],
  ["T18 Hangul LV + T", "각"],
  ["T19 Devanagari ksha+i (conjunct + spacing mark)", "क्षि"],
  ["T20 Devanagari ki (spacing mark)", "कि"],
  ["T21 heart + VS16", "❤️"],
  ["T22 keycap 1", "1️⃣"],
  ["T23 Arabic prepend", "؀١"],
  ["T24 Thai sara am (spacing mark)", "กำ"],
  ["T25 England tag flag", "\u{1F3F4}\u{E0067}\u{E0062}\u{E0065}\u{E006E}\u{E0067}\u{E007F}"],
  ["T26 enclosing circle", "a⃝"],
  ["T27 emoji pair", "\u{1F600}\u{1F600}"],
  ["T28 CJK", "中文"],
  ["T29 tab and letters", "\tab\tc"],
  ["T30 ZWNJ (Extend)", "a‌b"],
  ["T31 Bengali ksha (conjunct)", "ক্ষ"],
  ["T32 emoji + VS15", "☺︎"],
  ["T33 CR + combining", "\ŕ"],
  ["T34 LF + ZWJ + emoji", "\n‍\u{1F600}"],
]

const asciiAlpha = ["a", "b", "Z", " ", "\t", "_", "1", ".", "\r", "\n"]
const nonAsciiAlpha = ["a", "\t", " ", "é", "́", "\u{1F600}", "\u{1F44D}", "\u{1F3FD}", "‍", "\u{1F1EB}", "\u{1F1F7}", "中", " ", "ᄀ", "ᅡ", "्", "क", "️", "\r", "\n"]
const rnd = mulberry32(0xC3C3)
const pick = (a: string[]) => a[Math.floor(rnd() * a.length)]
for (let i = 0; i < 2000; i++) {
  let n = Math.floor(rnd() * 13), s = ""
  for (let j = 0; j < n; j++) s += pick(asciiAlpha)
  table.push([`A${i}`, s])
}
for (let i = 0; i < 500; i++) {
  let n = Math.floor(rnd() * 10), s = ""
  for (let j = 0; j < n; j++) s += pick(nonAsciiAlpha)
  table.push([`N${i}`, s])
}

function cps(s: string) { return Array.from(s) }
function cpToU16(s: string, cp: number) { return cps(s).slice(0, cp).join("").length }
function u16ToCp(s: string, u: number) {
  let acc = 0, i = 0
  for (const c of cps(s)) { if (acc == u) return String(i); acc += c.length; i++; if (acc > u) return `mid${u}` }
  return acc == u ? String(i) : `beyond${u}`
}

let caseLines: string[] = [], resLines: string[] = []
for (const [id, s] of table) {
  const cp = cps(s)
  caseLines.push(id.replace(/ /g, "_") + "\t" + cp.map(c => c.codePointAt(0)!.toString(16)).join(" "))
  let fwd = [], bwd = [], cc = [], fc = [], fcs = []
  for (let i = 0; i <= cp.length; i++) {
    let u = cpToU16(s, i)
    fwd.push(u16ToCp(s, findClusterBreak(s, u, true)))
    bwd.push(u16ToCp(s, findClusterBreak(s, u, false)))
    cc.push(countColumn(s, 4, u))
  }
  for (let col = 0; col <= cp.length * 4 + 2; col++) {
    fc.push(u16ToCp(s, findColumn(s, col, 4)))
    let r = findColumn(s, col, 4, true)
    fcs.push(r < 0 ? "none" : u16ToCp(s, r))
  }
  resLines.push(`${id.replace(/ /g, "_")}\tfwd:${fwd.join(",")}\tbwd:${bwd.join(",")}\tcc:${cc.join(",")}\tccfull:${countColumn(s, 4)}\tfc:${fc.join(",")}\tfcs:${fcs.join(",")}`)
}
writeFileSync(out + "/cases.txt", caseLines.join("\n") + "\n")
writeFileSync(out + "/cm6-results.txt", resLines.join("\n") + "\n")
console.log("cases", table.length)
