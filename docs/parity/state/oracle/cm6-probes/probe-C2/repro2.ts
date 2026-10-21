import {Text, ChangeSet, ChangeDesc} from "../src/index.ts"
const mk = (s: number[]) => (ChangeDesc as any).create(s) as ChangeDesc
const t = (l: string, f: () => any) => { try { console.log(l, "=>", JSON.stringify(f())) } catch (e: any) { console.log(l, "=> THROWS", e.message) } }
t("R2p' map over empty shorter desc", () => String(ChangeSet.of({from: 1, insert: "x"}, 5).map(mk([3, -1]))))
t("R2q composeDesc with empty wrong-length", () => String(mk([2, -1, 0, 2]).composeDesc(mk([30, -1]))))
t("R2r compose empty-this wrong length", () => String(ChangeSet.empty(3).compose(ChangeSet.of({from: 0, insert: "q"}, 9))))
t("R3g of([],0) vs empty(0) json", () => [ChangeSet.of([], 0).toJSON(), ChangeSet.empty(0).toJSON()])
t("R1p spec with to omitted & insert Text empty", () => String(ChangeSet.of({from: 2, insert: Text.empty}, 5)))
t("R11c' createSet missing inserted apply", () => (ChangeSet as any).createSet([2, -1, 0, 2], []).apply(Text.of(["xy"])).toString())
t("R12 apply returns same doc for empty", () => { let d = Text.of(["ab"]); return ChangeSet.empty(2).apply(d) === d })
