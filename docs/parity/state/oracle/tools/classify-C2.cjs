// Classify unicode-mode oracle mismatches by the kind of non-ASCII content in the inserts.
const [cases, a, b] = process.argv.slice(2).map(f => JSON.parse(require("fs").readFileSync(f, "utf8")))
const LS = String.fromCharCode(0x2028)
const cat = ch => ch.codePointAt(0) > 0xffff ? "astral" : ch == "\r" ? "CR" : ch == LS ? "U+2028" : ch.codePointAt(0) > 127 ? "BMP-nonASCII" : "ascii"
const stats = {}
cases.forEach((c, i) => {
  const ins = [...c.a, ...c.b].map(s => s.insert).join("")
  const cats = new Set([...ins].map(cat))
  const k = [...cats].filter(x => x != "ascii").sort().join("+") || "ascii-only"
  const keys = new Set([...Object.keys(a[i]), ...Object.keys(b[i])])
  const bad = [...keys].some(key => key != "json" && JSON.stringify(a[i][key]) != JSON.stringify(b[i][key]))
  stats[k] = stats[k] || {cases: 0, mismatching: 0}; stats[k].cases++; if (bad) stats[k].mismatching++
})
console.log(JSON.stringify(stats, null, 1))
