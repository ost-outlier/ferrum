import {countColumn} from "../src/index.ts"
for (let rep = 0; rep < 3; rep++) for (const n of [10000, 20000, 40000]) {
  let s = "a".repeat(n), t = performance.now()
  let c = countColumn(s, 4)
  console.log(`rep ${rep} n=${n} countColumn=${c} in ${(performance.now() - t).toFixed(3)}ms`)
}
