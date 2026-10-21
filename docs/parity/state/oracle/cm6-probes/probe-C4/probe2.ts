import {EditorState, Facet, Prec, StateField, StateEffect, Compartment, Extension} from "../src/index.ts"
function run(name: string, f: () => any) {
  try { console.log(name + ": " + f()) } catch (e: any) { console.log(name + ": THROWS " + (e.constructor?.name) + ": " + e.message) }
}
const mk = (...extensions: Extension[]) => EditorState.create({extensions})
const num = Facet.define<number>(), str = Facet.define<string>()
run("R12b combine count, empty tr", () => { let calls = 0; let f = Facet.define<number, number>({combine: v => { calls++; return v.length }}); let st = mk(f.of(1), f.compute([], () => 2)); let before = calls; st.update({}).state.facet(f); return "define+create=" + before + " afterEmptyTr=" + calls })
run("R57 compartment reconf same inner object", () => { let c = new Compartment; let inner = num.of(1); let st = mk(c.of(inner)); let st2 = st.update({effects: c.reconfigure(inner)}).state; return st2.facet(num).join() + " same=" + (st.facet(num) === st2.facet(num)) })
run("R58 field update returns new equal array", () => { let fld = StateField.define({create: () => [1], update: v => v.slice()}); let st = mk(fld); return st.field(fld) === st.update({}).state.field(fld) })
run("R60 field update reading dynamic facet of tr.state", () => { let fld = StateField.define({create: s => s.facet(num).join(), update: (v, tr) => tr.state.facet(num).join()}); let st = mk(fld, num.compute(["doc"], s => s.doc.length)); return st.update({changes: {insert: "ab", from: 0}}).state.field(fld) })
run("R61 provide inherits field prec", () => { let fld = StateField.define({create: () => 3, update: v => v, provide: f => num.from(f)}); return mk(num.of(1), Prec.highest(fld)).facet(num).join() })
run("R62 enables at default prec", () => { let other = Facet.define<string>(); let f = Facet.define<number>({enables: other.of("e")}); return mk(other.of("a"), Prec.lowest(f.of(1)), Prec.highest(other.of("h"))).facet(other).join() })
run("R63 enables dedup across providers", () => { let other = Facet.define<string>(); let f = Facet.define<number>({enables: other.of("e")}); return mk(f.of(1), f.of(2)).facet(other).join() })
run("R64 Facet.of distinct calls not deduped", () => mk(num.of(1), num.of(1)).facet(num).join())
run("R65 extension object with .extension", () => { let obj = {extension: [num.of(5), num.of(6)]}; return mk(obj, num.of(7)).facet(num).join() })
run("R66 init override on reconfigure re-inits", () => { let c = new Compartment; let fld = StateField.define({create: () => 0, update: v => v + 1}); let st = mk(fld, c.of([])); st = st.update({}).state; let a = st.field(fld); st = st.update({effects: c.reconfigure(fld.init(() => 100))}).state; return a + " -> " + st.field(fld) })
run("R67 dynamic facet slot when providers appear only via compartment", () => { let c = new Compartment; let st = mk(num.of(1), c.of(num.compute(["doc"], s => s.doc.length))); st = st.update({effects: c.reconfigure([])}).state; return st.facet(num).join() })
