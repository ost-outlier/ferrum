import {ChangeSet, ChangeDesc, MapMode} from "../src/index.ts"
function changeSet(changes: [number, number, number][]) {
  let collect: any[] = []
  for (let [from, to, len] of changes) {
    if (len) collect.push({insert: "x".repeat(len), from, to})
    if (from < to) collect.push({from, to})
  }
  return ChangeSet.of(collect, 5100)
}
const cases: any = {t10: [[0,0,1],[2,3,0],[8,8,20]], t11: [[1,1,2],[2,2,2]], t12: [[1,1,2]], t13: [[1,1,2],[4,4,2]], t14: [[1,2,0],[1,1,1]], t15: [[0,4,0]], t16: [[3,7,0]], t17: [[2,4,6]], t18a: [[30,4500,0]], t18b: [[0,1000,0]], t19: [[1,2,0]], t20b: [[1,1,1]], t22: [[1,7,0]], t23: [[10,16,0]], t24: [[15,25,0]], t28: [[5,15,20]], t29: [[900,1000,0]], t30: [[300,500,100]], t33: [[0,0,50],[50,100,0],[150,200,0]], t34: [[2,4,0]], empty: []}
for (let k in cases) console.log(k, JSON.stringify(changeSet(cases[k]).desc.toJSON()))
// R31
import {RangeSet, RangeValue} from "../src/index.ts"
class V extends RangeValue { constructor(readonly name: string, public startSide = 0, public endSide = 0, public point = false) { super() } }
for (let side of [127, 128, 255, 256, -129, 100000001, -100000000, -200000000, 499999999, -599999999]) {
  let s = RangeSet.of([new V("p", side, side, true).range(5)]); let m = s.map(ChangeDesc.fromJSON([5, -1, 0, 1, 5, -1]))
  let it = m.iter(); console.log("R31 point side", side, "insert at 5 =>", it.value ? `${it.from}-${it.to}:p` : "")
}
