import {Text} from "../src/index.ts"
const J = (x: any) => JSON.stringify(x)
function t(name: string, f: () => any) { try { console.log(name, "=>", J(f())) } catch (e: any) { console.log(name, "=> THROWS", e.constructor.name, J(e.message)) } }
function depth(node: Text): number { return !node.children ? 1 : 1 + Math.max(...node.children.map(depth)) }
const line = "1234567890".repeat(10)
const lines = new Array(200).fill(line), text0 = lines.join("\n")
const doc0 = Text.of(lines)
const L = (l: any) => ({from: l.from, to: l.to, number: l.number, text: l.text, length: l.length})
// Text.of
t("R1 of([])", () => Text.of([]))
t("R2 of(['']) === empty", () => Text.of([""]) === Text.empty)
t("R3 of(['a\\nb']) len/lines/toString/toJSON", () => { let d = Text.of(["a\nb"]); return [d.length, d.lines, d.toString(), d.toJSON(), L(d.lineAt(2))] })
t("R4 of(['a\\r\\nb'])", () => { let d = Text.of(["a\r\nb"]); return [d.length, d.lines, L(d.line(1))] })
t("R5 of(['a\\u2028b','c'])", () => { let d = Text.of(["a b", "c"]); return [d.length, d.lines, L(d.line(1)), L(d.line(2))] })
t("R6 of(['a\\u0085b\\u000bc\\u000cd\\re'])", () => { let d = Text.of(["a\u0085b\u000bc\u000cd\re"]); return [d.length, d.lines] })
t("R7 of(['a','']) ", () => { let d = Text.of(["a", ""]); return [d.length, d.lines, L(d.line(2)), d.toJSON()] })
t("R8 empty", () => [Text.empty.length, Text.empty.lines, Text.empty.toJSON(), Text.empty.children])
// surrogate
t("R9 of(['a😀b'])", () => { let d = Text.of(["a😀b"]); return [d.length, L(d.lineAt(2)), d.sliceString(1, 3), J(d.sliceString(1, 2)), d.sliceString(3)] })
t("R9b of(['😀','x']) line2", () => { let d = Text.of(["😀", "x"]); return [d.length, L(d.line(2)), L(d.lineAt(3))] })
// range errors
t("R10 line(0)", () => doc0.line(0))
t("R11 line(lines+1)", () => doc0.line(201))
t("R12 lineAt(-1)", () => doc0.lineAt(-1))
t("R13 lineAt(len+1)", () => doc0.lineAt(doc0.length + 1))
t("R13b lineAt(len)", () => L(doc0.lineAt(doc0.length)))
t("R13c lineAt(1.5)", () => L(Text.of(["ab","cd"]).lineAt(1.5)))
t("R14 lineAt on break", () => { let d = Text.of(["ab", "cd"]); return [L(d.lineAt(2)), L(d.lineAt(3))] })
// sliceString
t("R15 sliceString lineSep", () => Text.of(["a", "b", "c"]).sliceString(0, 5, "\r\n"))
t("R15b sliceString default", () => Text.of(["a", "b", "c"]).sliceString(1))
t("R15c sliceString big lineSep", () => doc0.sliceString(99, 103, "<BR>"))
t("R16 sliceString clip", () => [Text.of(["abc"]).sliceString(-5, 2), Text.of(["abc"]).sliceString(2, 1), Text.of(["abc"]).sliceString(1, 100)])
t("R16b slice(5,0) len", () => doc0.slice(5, 0).length)
t("R16c slice(-5,0)/slice(0,-10)", () => [doc0.slice(-5, 0).length, doc0.slice(0, -10).length])
t("R16d slice default to", () => Text.of(["abc", "de"]).slice(2).toJSON())
t("R16e slice structure", () => { let s = Text.of(["abc", "de"]).slice(1, 5); return [s.length, s.lines, s.toJSON()] })
// replace
t("R17 replace from>to", () => Text.of(["abcdef"]).replace(4, 2, Text.of(["X"])).toString())
t("R17b replace out of range", () => Text.of(["abc"]).replace(10, 20, Text.of(["X"])).toString())
t("R17c replace negative", () => Text.of(["abc"]).replace(-3, 1, Text.of(["X"])).toString())
t("R17d replace with multi-line", () => { let d = Text.of(["abc"]).replace(1, 2, Text.of(["X", "Y"])); return [d.lines, d.toJSON()] })
t("R17e append empty", () => { let a = Text.of(["x"]); return [a.append(Text.empty) === a, Text.empty.append(a) === a] })
// eq
t("R18 eq", () => [Text.of(["a", "b"]).eq(Text.of(["a\nb"])), Text.of(["a\nb"]).toString() == Text.of(["a", "b"]).toString(), Text.of(["a", "b"]).eq(Text.of(["a", "b"]))])
// toString / toJSON
t("R19 toJSON", () => Text.of(["ab", "", "c"]).toJSON())
t("R19b toString lines", () => Text.of(["ab", "", "c"]).toString())
// children
t("R20 children", () => [doc0.children!.length, doc0.children!.map(c => c.length), doc0.children!.map(c => c.lines), depth(doc0)])
t("R20b small children", () => Text.of(["a"]).children)
t("R21 depth 2000", () => depth(Text.of(new Array(2000).fill(line))))
t("R21b depth after insert", () => { let doc = doc0; let insert = "abc".repeat(200), at = Math.floor(doc.length / 2); for (let i = 0; i < 10; i++) doc = doc.replace(at, at, Text.of([insert])); return [depth(doc), at] })
t("R21c depth after delete", () => { let d = doc0.replace(10, text0.length - 10, Text.empty); return [depth(d), d.children] })
t("R21d test10 pos", () => doc0.children![0].length)
// Line
t("R22 Line.length", () => { let l = Text.of(["abc", "de"]).line(2); return [l.length, l.to - l.from] })
// iter
const all = (it: any) => { let r: any[] = []; while (!it.next().done) r.push([it.value, it.lineBreak]); return r }
t("R23 iter", () => all(Text.of(["ab", "", "c"]).iter()))
t("R23b iter -1", () => all(Text.of(["ab", "", "c"]).iter(-1)))
t("R23c iter before next", () => { let it = Text.of(["ab"]).iter(); return [it.value, it.done, it.lineBreak] })
t("R23d iter after done", () => { let it = Text.of(["ab"]).iter(); it.next(); it.next(); let a = [it.value, it.done]; it.next(); return [a, it.value, it.done] })
t("R24 neg skip", () => { let it = Text.of(["one", "two", "three", "four"]).iter(); return [it.next(12).value, it.next(-12).value, it.next(12).value, it.next(-1000).value] })
t("R24b skip into break", () => { let it = Text.of(["one", "two"]).iter(); return [it.next(3).value, it.lineBreak] })
t("R24c skip past end", () => { let it = Text.of(["one", "two"]).iter(); let v = it.next(100); return [v.value, v.done] })
t("R25 iterRange", () => all(Text.of(["ab", "cd", "ef"]).iterRange(1, 7)))
t("R25b iterRange reverse", () => all(Text.of(["ab", "cd", "ef"]).iterRange(7, 1)))
t("R25c iterRange break at boundary", () => all(Text.of(["ab", "cd"]).iterRange(2, 3)))
t("R25d iterRange default to", () => all(Text.of(["ab", "cd"]).iterRange(3)))
t("R25e iterRange empty", () => all(Text.of(["ab"]).iterRange(1, 1)))
t("R25f iterRange out of range", () => all(Text.of(["ab"]).iterRange(0, 10)))
t("R25g iterRange skip", () => { let it = Text.of(["abcdef", "gh"]).iterRange(1, 8); return [it.next(2).value, it.next(-1).value, it.next(100).value, it.done] })
t("R25h iterRange skip neg clamp", () => { let it = Text.of(["abcdef"]).iterRange(2, 5); it.next(); return [it.value, it.next(-100).value] })
// iterLines
const lns = (it: any) => { let r: any[] = []; while (!it.next().done) r.push(it.value); return r }
const doc7 = Text.of(["ab", "cde", "", "", "f", "", "g"])
t("R26 iterLines()", () => lns(doc7.iterLines()))
t("R26b iterLines(2,3)", () => lns(doc7.iterLines(2, 3)))
t("R26c iterLines(2,1)", () => lns(doc7.iterLines(2, 1)))
t("R26d iterLines(3)", () => lns(doc7.iterLines(3)))
t("R26e iterLines(0)", () => lns(doc7.iterLines(0)))
t("R26f iterLines(1,100)", () => lns(doc7.iterLines(1, 100)))
t("R26g iterLines empty doc", () => lns(Text.empty.iterLines()))
t("R26h iterLines lineBreak", () => { let it = doc7.iterLines(); it.next(); return it.lineBreak })
t("R26i iterLines trailing empty", () => lns(Text.of(["a", ""]).iterLines()))
// Symbol.iterator
t("R27 spread", () => [...Text.of(["ab", "c"])])
t("R27b for-of iterRange", () => [...Text.of(["ab", "c"]).iterRange(1, 4)])
