import {EditorSelection} from "../src/index.ts"
import {writeFileSync} from "fs"
// xorshift32, seed 0xA4A4A4A4
let x = 0xA4A4A4A4 >>> 0
const rnd = (n: number) => { x ^= x << 13; x >>>= 0; x ^= x >>> 17; x ^= x << 5; x >>>= 0; return x % n }
let out: any[] = []
for (let i = 0; i < 5000; i++) {
  let n = 1 + rnd(5), ranges: number[][] = []
  for (let j = 0; j < n; j++) ranges.push([rnd(11), rnd(11)])
  let main = rnd(n)
  let sel = EditorSelection.create(ranges.map(([a, h]) => EditorSelection.range(a, h)), main)
  out.push({ranges, main, result: sel.ranges.map(r => [r.anchor, r.head]), resultMain: sel.mainIndex})
}
writeFileSync(process.argv[2], JSON.stringify(out))
