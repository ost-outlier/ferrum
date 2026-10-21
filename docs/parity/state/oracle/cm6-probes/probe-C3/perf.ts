import {countColumn, findClusterBreak} from "../src/index.ts"
for (const n of [10000, 20000, 40000]) {
  let s = "a".repeat(n), t = performance.now()
  let c = countColumn(s, 4), d1 = performance.now() - t
  t = performance.now(); let k = 0; for (let i = 0; i < 1000; i++) k += findClusterBreak(s, n / 2)
  console.log(`n=${n} countColumn=${c} in ${d1.toFixed(2)}ms; 1000x findClusterBreak(mid) in ${(performance.now() - t).toFixed(2)}ms`)
}
