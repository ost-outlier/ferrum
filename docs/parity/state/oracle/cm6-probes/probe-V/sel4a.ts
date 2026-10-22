import {EditorSelection} from "../src/index.ts"
import {readFileSync, writeFileSync} from "fs"
const show = (s: EditorSelection) => s.ranges.map(r => r.anchor + "/" + r.head).join(",") + " main=" + s.mainIndex
// 4a exact case
console.log("4a case:", show(EditorSelection.create([EditorSelection.cursor(12), EditorSelection.range(12, 18), EditorSelection.cursor(12)], 2)))
// A4 generator check: how many cases have the main range with an equal twin (same anchor/head) elsewhere
let x = 0xA4A4A4A4 >>> 0
const rnd = (n: number) => { x ^= x << 13; x >>>= 0; x ^= x >>> 17; x ^= x << 5; x >>>= 0; return x % n }
let twins = 0, twinsBefore = 0
for (let i = 0; i < 5000; i++) {
  let n = 1 + rnd(5), ranges: number[][] = []
  for (let j = 0; j < n; j++) ranges.push([rnd(11), rnd(11)])
  let main = rnd(n)
  let m = ranges[main]
  let others = ranges.filter((r, k) => k != main && r[0] == m[0] && r[1] == m[1])
  if (others.length) twins++
}
console.log("A4 generator: cases where main has an equal twin:", twins)
// exhaustive: 3 ranges, anchors/heads in 0..4, every main index
let out: string[] = []
for (let a = 0; a < 25; a++) for (let b = 0; b < 25; b++) for (let c = 0; c < 25; c++) for (let m = 0; m < 3; m++) {
  let rs = [a, b, c].map(v => [Math.floor(v / 5), v % 5])
  let sel = EditorSelection.create(rs.map(([an, h]) => an == h ? EditorSelection.cursor(an) : EditorSelection.range(an, h)), m)
  out.push(rs.map(r => r.join(" ")).join(" ") + " " + m + " | " + sel.ranges.map(r => r.anchor + " " + r.head).join(" ") + " | " + sel.mainIndex)
}
writeFileSync(process.argv[2], out.join("\n") + "\n")
console.log("exhaustive cases:", out.length)
