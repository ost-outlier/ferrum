import {Text, ChangeSet, ChangeDesc, MapMode} from "../src/index.ts"

function t(label: string, f: () => any) {
  try { let v = f(); console.log(label, "=>", typeof v == "string" ? JSON.stringify(v) : JSON.stringify(v)) }
  catch (e: any) { console.log(label, "=> THROWS", e.constructor.name + ":", e.message) }
}
const doc10 = Text.of(["0123456789"])
const mk = (s: number[]) => (ChangeDesc as any).create(s) as ChangeDesc

// R1 ChangeSet.of input forms
t("R1a nested arrays", () => String(ChangeSet.of([[{from: 1, insert: "a"}], [[{from: 3, to: 4}]]], 10)))
t("R1b ChangeSet inside array", () => {
  let inner = ChangeSet.of({from: 2, insert: "XY"}, 10)
  let cs = ChangeSet.of([{from: 5, insert: "a"}, inner, {from: 1, to: 2}], 10)
  return [String(cs), cs.apply(doc10).toString()]
})
t("R1c single object (not array)", () => String(ChangeSet.of({from: 3, to: 5, insert: "q"}, 10)))
t("R1d ChangeSet as whole spec", () => String(ChangeSet.of(ChangeSet.of({from: 3, insert: "q"}, 10), 10)))
t("R1e insert as Text", () => { let cs = ChangeSet.of({from: 0, insert: Text.of(["a", "b"])}, 10); return [String(cs), cs.apply(doc10).toString()] })
t("R1f insert string with \\r\\n default split", () => { let cs = ChangeSet.of({from: 0, insert: "a\r\nb"}, 10); return [String(cs), JSON.stringify(cs.apply(doc10).toString()), cs.newLength] })
t("R1g insert string with lone \\r", () => { let cs = ChangeSet.of({from: 0, insert: "a\rb"}, 10); return [String(cs), cs.apply(doc10).toString()] })
t("R1h lineSep='|'", () => { let cs = ChangeSet.of({from: 0, insert: "a|b\nc"}, 10, "|"); return [String(cs), JSON.stringify(cs.apply(doc10).toString()), cs.apply(doc10).lines] })
t("R1i insert u2028", () => { let cs = ChangeSet.of({from: 0, insert: "a b"}, 10); return [String(cs), cs.apply(doc10).lines] })
t("R1j insert undefined/empty", () => String(ChangeSet.of([{from: 3}, {from: 3, insert: ""}], 10)))
t("R1k empty array", () => [String(ChangeSet.of([], 10)), JSON.stringify(ChangeSet.of([], 10).toJSON())])
t("R1l empty array len 0", () => [String(ChangeSet.of([], 0)), JSON.stringify(ChangeSet.of([], 0).toJSON())])
t("R1m unsorted specs", () => String(ChangeSet.of([{from: 8, insert: "a"}, {from: 2, to: 4}], 10)))
t("R1n overlapping specs", () => { let cs = ChangeSet.of([{from: 2, to: 6, insert: "A"}, {from: 4, to: 8, insert: "B"}], 10); return [String(cs), cs.apply(doc10).toString()] })

// R2 errors
t("R2a invalid range from>to", () => ChangeSet.of({from: 5, to: 3}, 10))
t("R2b invalid range to>len", () => ChangeSet.of({from: 5, to: 11}, 10))
t("R2c invalid from<0", () => ChangeSet.of({from: -1, to: 2}, 10))
t("R2d mismatched set length in spec", () => ChangeSet.of([ChangeSet.empty(5)], 10))
t("R2e apply wrong length", () => ChangeSet.of({from: 1, insert: "x"}, 5).apply(doc10))
t("R2f mapPos past end", () => mk([4, -1, 0, 2]).mapPos(7))
t("R2f2 mapPos past end empty set", () => ChangeSet.empty(0).mapPos(1))
t("R2f3 mapPos pos==len", () => mk([4, -1, 0, 2]).mapPos(4, 1))
t("R2g compose mismatch", () => ChangeSet.of({from: 1, insert: "x"}, 5).compose(ChangeSet.of({from: 1, insert: "x"}, 5)))
t("R2h map mismatch", () => ChangeSet.of({from: 1, insert: "x"}, 5).map(ChangeSet.of({from: 1, insert: "x"}, 6)))
t("R2i composeDesc mismatch", () => mk([2, -1, 0, 2]).composeDesc(mk([1, -1, 0, 1])))
t("R2j fromJSON invalid", () => ChangeSet.fromJSON({}))
t("R2k fromJSON invalid part", () => ChangeSet.fromJSON([[1, 2]]))
t("R2l ChangeDesc.fromJSON odd", () => ChangeDesc.fromJSON([1]))
t("R2m ChangeDesc.fromJSON nonnum", () => ChangeDesc.fromJSON(["a", -1]))
t("R2n mapPos negative pos", () => mk([4, -1]).mapPos(-1))
t("R2o invert with short doc", () => { let cs = ChangeSet.of({from: 2, to: 8}, 10); return cs.invert(Text.of(["abc"])).toJSON() })
t("R2p mapPos into mismatch (map desc shorter)", () => ChangeSet.of({from: 1, insert: "x"}, 5).mapDesc(mk([3, -1])))

// R3 empty
t("R3a ChangeSet.empty(0)", () => [JSON.stringify(ChangeSet.empty(0).toJSON()), ChangeSet.empty(0).empty, ChangeSet.empty(0).length, String(ChangeSet.empty(0))])
t("R3b ChangeSet.empty(5)", () => [JSON.stringify(ChangeSet.empty(5).toJSON()), ChangeSet.empty(5).empty, String(ChangeSet.empty(5))])
t("R3c desc of empty", () => [ChangeSet.empty(5).desc.empty, JSON.stringify(ChangeSet.empty(5).desc.toJSON())])
t("R3d .empty on insertion", () => ChangeSet.of({from: 0, insert: "x"}, 0).empty)
t("R3e .empty on [3,-1,2,-1] unnormalized", () => mk([3, -1, 2, -1]).empty)
t("R3f length/newLength", () => { let c = ChangeSet.of([{from: 1, to: 3, insert: "abc"}], 10); return [c.length, c.newLength] })

// R4 iterChanges individual
{
  let cs = ChangeSet.of([{from: 2, to: 4, insert: "A"}, {from: 4, to: 6, insert: "B"}], 10)
  t("R4 sections", () => String(cs))
  let out: any[] = []
  cs.iterChanges((a, b, c, d, txt) => out.push([a, b, c, d, txt.toString()]))
  console.log("R4a iterChanges joined =>", JSON.stringify(out))
  out = []; cs.iterChanges((a, b, c, d, txt) => out.push([a, b, c, d, txt.toString()]), true)
  console.log("R4b iterChanges individual =>", JSON.stringify(out))
  out = []; cs.iterChangedRanges((a, b, c, d) => out.push([a, b, c, d]))
  console.log("R4c iterChangedRanges =>", JSON.stringify(out))
  out = []; cs.iterChangedRanges((a, b, c, d) => out.push([a, b, c, d]), true)
  console.log("R4d iterChangedRanges individual =>", JSON.stringify(out))
  out = []; cs.desc.iterChangedRanges((a, b, c, d) => out.push([a, b, c, d]), true)
  console.log("R4e desc.iterChangedRanges individual =>", JSON.stringify(out))
  out = []; cs.iterGaps((a, b, l) => out.push([a, b, l]))
  console.log("R4f iterGaps =>", JSON.stringify(out))
}

// R5 touchesRange
{
  let d = mk([2, -1, 2, 1, 4, -1, 0, 3, 2, -1])  // 2 2:1 4 0:3 2
  t("R5 desc", () => String(d))
  for (let [f, to] of [[0, 1], [0, 2], [1, 2], [2, 2], [3, 3], [2, 4], [3, 3], [4, 4], [5, 5], [5, 7], [8, 8], [9, 9], [8, 10], [0, 12], [10, 12], [12, 12]])
    t(`R5 touchesRange(${f},${to})`, () => d.touchesRange(f, to))
  t("R5 touchesRange(3) default to", () => d.touchesRange(3))
  t("R5 empty set touchesRange", () => ChangeSet.empty(4).touchesRange(0, 4))
  t("R5 pure deletion cover", () => mk([2, -1, 6, 0, 2, -1]).touchesRange(3, 5))
}

// R6 JSON
{
  let cs = ChangeSet.of([{from: 1, to: 2}, {from: 4, insert: "a\nb"}, {from: 6, to: 8, insert: "xy"}], 10)
  t("R6a toJSON", () => cs.toJSON())
  t("R6b fromJSON round trip", () => [String(ChangeSet.fromJSON(cs.toJSON())), ChangeSet.fromJSON(cs.toJSON()).apply(doc10).toString(), cs.apply(doc10).toString()])
  t("R6c desc toJSON", () => cs.desc.toJSON())
  t("R6d ChangeDesc.fromJSON", () => String(ChangeDesc.fromJSON(cs.desc.toJSON())))
  t("R6e ChangeDesc.fromJSON unvalidated negative", () => String(ChangeDesc.fromJSON([-5, -1])))
  t("R6f emoji toJSON", () => ChangeSet.of({from: 0, insert: "😀"}, 0).toJSON())
}

// R7 mapPos defaults / assoc values
{
  let d = mk([4, -1, 0, 2, 4, -1])
  t("R7a mapPos(4) default assoc", () => d.mapPos(4))
  t("R7b mapPos(4,0)", () => d.mapPos(4, 0))
  t("R7c mapPos(4,5)", () => d.mapPos(4, 5))
  t("R7d mapPos(4,-100)", () => d.mapPos(4, -100))
  t("R7e mapPos(4,1,Simple)", () => d.mapPos(4, 1, MapMode.Simple))
  t("R7f ChangeSet.mapPos with mode", () => ChangeSet.of({from: 2, to: 6}, 10).mapPos(4, 1, MapMode.TrackDel))
  t("R7g mapPos at insertion end of doc", () => d.mapPos(8, 1))
  t("R7h mapPos fractional pos", () => d.mapPos(4.5))
}

// R8 ChangeSet.mapDesc returns ChangeSet
{
  let a = ChangeSet.of({from: 1, insert: "zz"}, 5), b = ChangeSet.of({from: 0, to: 2}, 5)
  t("R8a ChangeSet.mapDesc type", () => [a.mapDesc(b.desc).constructor.name, String(a.mapDesc(b.desc))])
  t("R8b ChangeSet.mapDesc over empty", () => [a.mapDesc(ChangeSet.empty(5)).constructor.name, String(a.mapDesc(ChangeSet.empty(5)))])
  t("R8c desc.mapDesc type", () => [a.desc.mapDesc(b.desc).constructor.name])
  t("R8d map identity when other empty", () => a.map(ChangeSet.empty(5)) === a)
  t("R8e compose identity", () => [a.compose(ChangeSet.empty(7)) === a, ChangeSet.empty(5).compose(ChangeSet.of({from:0, insert:"q"},5)).toJSON()])
  t("R8f ChangeSet.desc identity", () => a.desc instanceof ChangeSet)
  t("R8g invertedDesc", () => String(mk([2, -1, 3, 1, 0, 4, 2, -1]).invertedDesc))
  t("R8h invert of set", () => { let c = ChangeSet.of([{from: 1, to: 3, insert: "XYZ"}, {from: 5, insert: "q"}], 10); return [c.invert(doc10).toJSON(), c.invert(doc10).apply(c.apply(doc10)).toString()] })
  t("R8i compose of desc with set (mixed)", () => String(mk([1, -1, 0, 2]).composeDesc(ChangeSet.of({from: 0, to: 1}, 3))))
}

// R9 non-ASCII
{
  let d = Text.of(["a😀b"])
  t("R9a length a😀b", () => d.length)
  t("R9b insert emoji length", () => { let c = ChangeSet.of({from: 1, insert: "😀"}, 4); return [String(c), c.newLength] })
  t("R9c delete emoji 1..3", () => ChangeSet.of({from: 1, to: 3}, 4).apply(d).toString())
  t("R9d delete half emoji 1..2", () => JSON.stringify(ChangeSet.of({from: 1, to: 2}, 4).apply(d).toString()))
  t("R9e mapPos(3) over insert at 0 of é", () => ChangeSet.of({from: 0, insert: "é"}, 4).mapPos(3))
  t("R9f CRLF in insert length", () => ChangeSet.of({from: 0, insert: "x\r\ny"}, 0).newLength)
}

// R10 Display / toString
t("R10 toString", () => String(ChangeSet.of([{from: 1, to: 2}, {from: 4, insert: "ab"}], 6)))
