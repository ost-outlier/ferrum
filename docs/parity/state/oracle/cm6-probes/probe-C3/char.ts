import {findClusterBreak, codePointAt, fromCodePoint, codePointSize, countColumn, findColumn, EditorState, CharCategory} from "../src/index.ts"
import {makeCategorizer} from "../src/charcategory.ts"
function t(label: string, f: () => any) {
  try { console.log(label, "=>", JSON.stringify(f())) } catch (e: any) { console.log(label, "=> THROWS", e.constructor.name + ": " + e.message) }
}
const hex = (s: string) => Array.from({length: s.length}, (_, i) => s.charCodeAt(i).toString(16)).join(" ")
// findClusterBreak
t("C01 fcb('e\\u0301x',0,true,false)", () => findClusterBreak("éx", 0, true, false))
t("C02 fcb('e\\u0301x',0,true,true)", () => findClusterBreak("éx", 0, true, true))
t("C03 fcb('e\\u0301x',2,false,false)", () => findClusterBreak("éx", 2, false, false))
t("C04 fcb('e\\u0301x',2,false,true)", () => findClusterBreak("éx", 2, false, true))
t("C05 fcb('a\\u200db',0,true,false) ZWJ still joins", () => findClusterBreak("a‍b", 0, true, false))
t("C06 fcb('\\u{1F600}x',1) mid-surrogate fwd", () => findClusterBreak("\u{1F600}x", 1))
t("C07 fcb('\\u{1F600}x',1,false) mid-surrogate bwd", () => findClusterBreak("\u{1F600}x", 1, false))
t("C08 fcb('\\u{1F600}',0) emoji fwd (UTF-16)", () => findClusterBreak("\u{1F600}", 0))
t("C09 fcb('a\\ud83db',1) lone high surrogate", () => findClusterBreak("a\ud83db", 1))
t("C10 fcb('a\\ude00b',1) lone low surrogate", () => findClusterBreak("a\ude00b", 1))
t("C11 fcb('\\ude00',0)", () => findClusterBreak("\ude00", 0))
t("C12 fcb('ab',5) pos>len fwd", () => findClusterBreak("ab", 5))
t("C13 fcb('ab',5,false) pos>len bwd", () => findClusterBreak("ab", 5, false))
t("C14 fcb('ab',2) at end fwd", () => findClusterBreak("ab", 2))
t("C15 fcb('ab',0,false) at start bwd", () => findClusterBreak("ab", 0, false))
t("C16 fcb('',0)", () => findClusterBreak("", 0))
t("C17 fcb('',0,false)", () => findClusterBreak("", 0, false))
t("C18 fcb('ab',-1)", () => findClusterBreak("ab", -1))
t("C19 fcb('a\\ufe0f',0,true,false) VS with includeExtending false", () => findClusterBreak("❤️", 0, true, false))
t("C20 fcb('\\u{1F44D}\\u{1F3FD}',0,true,false) skin tone, includeExtending false", () => findClusterBreak("\u{1F44D}\u{1F3FD}", 0, true, false))
t("C21 fcb flags pair (UTF-16) 0", () => findClusterBreak("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 0))
t("C22 fcb flags bwd from 8", () => findClusterBreak("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 8, false))
t("C23 fcb flags from 2 (mid flag)", () => findClusterBreak("\u{1F1EB}\u{1F1F7}\u{1F1E9}\u{1F1EA}", 2))
// codePointAt
t("P01 codePointAt('a\\u{1F600}',1)", () => codePointAt("a\u{1F600}", 1))
t("P02 codePointAt('a\\u{1F600}',2) low half", () => codePointAt("a\u{1F600}", 2))
t("P03 codePointAt('\\ud83d',0) lone high at end", () => codePointAt("\ud83d", 0))
t("P04 codePointAt('\\ud83dx',0) high + non-low", () => codePointAt("\ud83dx", 0))
t("P05 codePointAt('ab',5) out of range", () => codePointAt("ab", 5))
t("P06 codePointAt('ab',0)", () => codePointAt("ab", 0))
// fromCodePoint
t("F01 fromCodePoint(0x61)", () => hex(fromCodePoint(0x61)))
t("F02 fromCodePoint(0x1F600)", () => hex(fromCodePoint(0x1F600)))
t("F03 fromCodePoint(0xD800) lone surrogate", () => hex(fromCodePoint(0xD800)))
t("F04 fromCodePoint(0x110000) out of range", () => hex(fromCodePoint(0x110000)))
t("F05 fromCodePoint(-1)", () => hex(fromCodePoint(-1)))
t("F06 fromCodePoint(0xFFFF)", () => hex(fromCodePoint(0xFFFF)))
t("F07 fromCodePoint(0x10000)", () => hex(fromCodePoint(0x10000)))
// codePointSize
t("Z01 codePointSize(0xFFFF)", () => codePointSize(0xFFFF))
t("Z02 codePointSize(0x10000)", () => codePointSize(0x10000))
t("Z03 codePointSize(0x61)", () => codePointSize(0x61))
// countColumn / findColumn extras
t("K01 countColumn('\\tab',4)", () => countColumn("\tab", 4))
t("K02 countColumn('ab\\tc',4)", () => countColumn("ab\tc", 4))
t("K03 countColumn('a\\t\\t',2)", () => countColumn("a\t\t", 2))
t("K04 countColumn('abcd',4,2)", () => countColumn("abcd", 4, 2))
t("K05 countColumn('abcd',4,10) to>len", () => countColumn("abcd", 4, 10))
t("K06 countColumn('\\u{1F600}b',4,1) to inside surrogate", () => countColumn("\u{1F600}b", 4, 1))
t("K07 countColumn('\\u{1F600}b',4) emoji", () => countColumn("\u{1F600}b", 4))
t("K08 countColumn('\\t',0) tabSize 0", () => countColumn("\t", 0))
t("K09 countColumn('ab',4,-1)", () => countColumn("ab", 4, -1))
t("K10 countColumn('\\u4E2Dx',4) CJK width", () => countColumn("中x", 4))
t("J01 findColumn('\\tab',5,4)", () => findColumn("\tab", 5, 4))
t("J02 findColumn('\\tab',2,4) inside tab", () => findColumn("\tab", 2, 4))
t("J03 findColumn('ab',5,4)", () => findColumn("ab", 5, 4))
t("J04 findColumn('ab',5,4,true)", () => findColumn("ab", 5, 4, true))
t("J05 findColumn('ab',2,4,true) exact end", () => findColumn("ab", 2, 4, true))
t("J06 findColumn('\\u{1F600}b',1,4)", () => findColumn("\u{1F600}b", 1, 4))
t("J07 findColumn('e\\u0301b',1,4)", () => findColumn("éb", 1, 4))
t("J08 findColumn('ab',0,4,true) col 0", () => findColumn("ab", 0, 4, true))
t("J09 findColumn('ab',-3,4)", () => findColumn("ab", -3, 4))
t("J10 findColumn('\\t',1,0) tabSize 0", () => findColumn("\t", 1, 0))
t("J11 findColumn('',0,4,true)", () => findColumn("", 0, 4, true))
t("J12 findColumn('',1,4,true)", () => findColumn("", 1, 4, true))
// categorizer
const cat = EditorState.create({}).charCategorizer(0)
const name = (c: CharCategory) => CharCategory[c]
for (const [l, s] of [["empty", ""], ["space", " "], ["tab", "\t"], ["NBSP", " "], ["NEL U+0085", "\u0085"], ["BOM U+FEFF", "﻿"], ["ZWSP U+200B", "​"], ["MVS U+180E", "᠎"], ["ideographic space", "　"], ["LS U+2028", " "],
  ["a", "a"], ["1", "1"], ["_", "_"], ["$", "$"], ["-", "-"], ["Arabic-Indic 3 U+0663", "٣"], ["Devanagari 3 U+0969", "३"], ["Roman numeral U+2163", "Ⅳ"], ["superscript 2", "²"], ["fraction 1/2", "½"],
  ["combining acute alone", "́"], ["e+acute", "é"], ["space+acute", " ́"], ["acute+space", "́ "], ["Devanagari virama", "्"], ["Devanagari vowel sign i", "ि"], ["CJK 中", "中"], ["Hiragana あ", "あ"], ["Hangul 한", "한"],
  ["emoji 😀", "\u{1F600}"], ["emoji ZWJ seq", "\u{1F468}‍\u{1F469}"], ["flag FR", "\u{1F1EB}\u{1F1F7}"], ["ZWJ", "‍"], ["a+space (two chars)", "a "], [" a (space first)", " a"], ["/a", "/a"], ["ß", "ß"], ["ª ordinal", "ª"], ["Ⓐ circled A", "Ⓐ"], ["math bold A U+1D400", "\u{1D400}"], ["lone surrogate", "\ud83d"]] as [string, string][])
  t(`G ${l}`, () => name(cat(s)))
t("G wordChars '$' via makeCategorizer('$')", () => name(makeCategorizer("$")("$")))
t("G wordChars '-' match within string", () => name(makeCategorizer("-")("/-")))
t("G languageData wordChars", () => name(EditorState.create({extensions: EditorState.languageData.of(() => [{wordChars: "$"}])}).charCategorizer(0)("$")))
