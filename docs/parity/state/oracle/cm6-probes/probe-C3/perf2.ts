import {findClusterBreak} from "../src/index.ts"
for (const n of [10000, 20000, 40000]) { let s = "a".repeat(n), t = performance.now(); let r = findClusterBreak(s, n, false); console.log(`n=${n} one backward findClusterBreak(from end) = ${r} in ${(performance.now() - t).toFixed(3)}ms`) }
