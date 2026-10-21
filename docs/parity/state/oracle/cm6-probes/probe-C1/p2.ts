import {Text} from "../src/index.ts"
let a = Text.of(["a\r"]), b = Text.of(["", "b"])
console.log("P1", a.lines, b.lines, a.append(b).lines, JSON.stringify(a.append(b).toString()))
let d = Text.of(["a\r", "b"]); console.log("P2", d.length, d.lines, JSON.stringify(d.line(1)), JSON.stringify(d.line(2)))
d = Text.of(["a\r", "x", "b"]); let r = d.replace(2, 4, Text.empty); console.log("P3", d.lines, JSON.stringify(r.toString()), r.lines)
d = Text.of(["a\r", "b"]); console.log("P4", JSON.stringify(d.slice(0, 2).toString()), d.slice(0,2).lines)
let big = "x".repeat(1_000_000); d = Text.of([big, "y"])
let t = performance.now(), n = 0; for (let i = 0; i < 100; i++) n += d.lineAt(5).from; console.log("P7 100x lineAt(5)", (performance.now() - t).toFixed(3), "ms", n)
