// Differential oracle for C2 (change.ts). Usage:
//   tsx probe-C2/oracle.ts gen <seed> <n> <alphabet-mode> > cases.json
//   tsx probe-C2/oracle.ts run cases.json > cm6-out.json
//   tsx probe-C2/oracle.ts cmp cm6-out.json ferrum-out.json
// PRNG: mulberry32(seed). alphabet-mode: "ascii" (a-h plus \n) or "unicode" (adds é, 😀, \r\n, \r, U+2028).
import {Text, ChangeSet, MapMode} from "../src/index.ts"
import {readFileSync} from "fs"

function mulberry32(a: number) {
  return function() {
    a |= 0; a = a + 0x6D2B79F5 | 0
    let t = Math.imul(a ^ a >>> 15, 1 | a)
    t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t
    return ((t ^ t >>> 14) >>> 0) / 4294967296
  }
}

type Spec = {from: number, to: number, insert: string}

function gen(seed: number, n: number, mode: string) {
  let rnd = mulberry32(seed)
  let r = (k: number) => Math.floor(rnd() * k)
  let ascii = ["a", "b", "c", "d", "e", "f", "g", "h", "\n"]
  let uni = ascii.concat(["é", "😀", "\r\n", "\r", " "])
  let alpha = mode == "unicode" ? uni : ascii
  let rStr = (l: number) => { let s = ""; for (let i = 0; i < l; i++) s += alpha[r(alpha.length)]; return s }
  // Lengths are computed in UTF-16 (cm6 unit). For ASCII they equal code points.
  let rSpec = (len: number): Spec => {
    let kind = r(4)
    if (kind == 0) { let from = r(len + 1); return {from, to: from, insert: rStr(r(4) + 1)} }
    let from = r(len + 1), to = Math.min(len, from + r(6))
    return {from, to, insert: kind == 1 ? "" : rStr(r(4))}
  }
  let rSpecs = (len: number) => { let out: Spec[] = []; for (let i = r(7); i > 0; i--) out.push(rSpec(len)); return out }
  let cases = []
  for (let i = 0; i < n; i++) {
    // ASCII mode: doc lengths in UTF-16 == code points. Unicode mode keeps docs ASCII-only (inserts carry the non-ASCII)
    let size = r(25)
    let doc = ""; for (let k = 0; k < size; k++) doc += ascii[r(ascii.length)]
    let a = rSpecs(size), b = rSpecs(size)
    let setA = ChangeSet.of(a, size)
    let cSize = setA.newLength
    let c = mode == "unicode" ? [] : rSpecs(cSize)
    cases.push({doc, size, a, b, c, cSize})
  }
  return cases
}

function changes(set: ChangeSet, individual: boolean) {
  let out: any[] = []
  set.iterChanges((fa, ta, fb, tb, t) => out.push([fa, ta, fb, tb, t.toString()]), individual)
  return out
}

function run(cases: any[]) {
  let modes = [MapMode.Simple, MapMode.TrackDel, MapMode.TrackBefore, MapMode.TrackAfter]
  return cases.map(cs => {
    let res: any = {}
    try {
      let doc = Text.of(cs.doc.split(/\r\n?|\n/))
      let A = ChangeSet.of(cs.a, cs.size), B = ChangeSet.of(cs.b, cs.size)
      res.A = String(A); res.Alen = [A.length, A.newLength, A.empty]
      res.Ach = changes(A, false); res.AchI = changes(A, true)
      let gaps: any[] = []; A.iterGaps((a, b, l) => gaps.push([a, b, l])); res.Agaps = gaps
      res.Aapply = A.apply(doc).toString()
      res.B = String(B)
      let mF = A.map(B), mT = A.map(B, true)
      res.mapF = String(mF); res.mapT = String(mT)
      res.mapFch = changes(mF, true); res.mapTch = changes(mT, true)
      res.mapDescF = String(A.desc.mapDesc(B.desc)); res.mapDescT = String(A.desc.mapDesc(B.desc, true))
      res.conv = [A.compose(B.map(A)).apply(doc).toString(), B.compose(A.map(B, true)).apply(doc).toString()]
      let inv = A.invert(doc)
      res.inv = String(inv); res.invch = changes(inv, true); res.invApply = inv.apply(A.apply(doc)).toString()
      res.invDesc = String(A.desc.invertedDesc)
      if (cs.c.length || cs.cSize >= 0) {
        let C = ChangeSet.of(cs.c, cs.cSize)
        let AC = A.compose(C)
        res.comp = String(AC); res.compch = changes(AC, true); res.compApply = AC.apply(doc).toString()
        res.compDesc = String(A.desc.composeDesc(C.desc))
      }
      let mp: any[] = []
      for (let pos = 0; pos <= cs.size; pos++) for (let assoc of [-1, 1]) for (let m of modes) mp.push(A.mapPos(pos, assoc, m))
      res.mapPos = mp
      let cr: any[] = []; A.desc.iterChangedRanges((a, b, c, d) => cr.push([a, b, c, d]), true); res.ranges = cr
      res.json = A.toJSON()
    } catch (e: any) { res.error = String(e.message) }
    return res
  })
}

let [cmd, x, y, z] = process.argv.slice(2)
if (cmd == "gen") console.log(JSON.stringify(gen(+x, +y, z || "ascii")))
else if (cmd == "run") console.log(JSON.stringify(run(JSON.parse(readFileSync(x, "utf8")))))
else if (cmd == "cmp") {
  let a = JSON.parse(readFileSync(x, "utf8")), b = JSON.parse(readFileSync(y, "utf8"))
  let cases = z ? JSON.parse(readFileSync(z, "utf8")) : null
  let perKey: Record<string, number> = {}, first: Record<string, any> = {}, bad = 0
  for (let i = 0; i < a.length; i++) {
    let caseBad = false
    for (let k of new Set([...Object.keys(a[i]), ...Object.keys(b[i])])) {
      if (k == "json") continue // no Ferrum counterpart; compared separately
      if (JSON.stringify(a[i][k]) != JSON.stringify(b[i][k])) {
        perKey[k] = (perKey[k] || 0) + 1; caseBad = true
        if (!first[k] || JSON.stringify(cases?.[i] ?? "").length < JSON.stringify(first[k].input).length)
          first[k] = {i, input: cases?.[i], cm6: a[i][k], ferrum: b[i][k]}
      }
    }
    if (caseBad) bad++
  }
  console.log(JSON.stringify({cases: a.length, mismatchingCases: bad, perKey, smallestExample: first}, null, 1))
}
