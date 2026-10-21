// Compare cm6-results.txt and ferrum-results.txt field by field. Usage: tsx probe-C3/compare.ts <dir> [filterPrefix]
import {readFileSync} from "fs"
const dir = process.argv[2], filter = process.argv[3] || ""
const parse = (f: string) => new Map(readFileSync(f, "utf8").trim().split("\n").map(l => {
  let [id, ...fields] = l.split("\t")
  return [id, Object.fromEntries(fields.map(x => [x.slice(0, x.indexOf(":")), x.slice(x.indexOf(":") + 1)]))] as [string, any]
}))
const a = parse(dir + "/cm6-results.txt"), b = parse(dir + "/ferrum-results.txt")
const cases = new Map(readFileSync(dir + "/cases.txt", "utf8").trim().split("\n").map(l => l.split("\t") as [string, string]))
let counts: Record<string, Record<string, number>> = {}
for (const [id, ra] of a) {
  if (filter && !id.startsWith(filter)) continue
  const rb = b.get(id)!
  const group = id[0] == "T" ? "table" : id[0] == "A" ? "ascii" : "nonascii"
  for (const k of Object.keys(ra)) {
    counts[group] ??= {}
    counts[group][k + "_total"] = (counts[group][k + "_total"] || 0) + 1
    if (ra[k] != rb[k]) {
      counts[group][k + "_mismatch"] = (counts[group][k + "_mismatch"] || 0) + 1
      if (group == "table" || process.env.VERBOSE) console.log(`${id} [${cases.get(id)}] ${k}: cm6=${ra[k]} ferrum=${rb[k]}`)
    }
  }
}
console.log(JSON.stringify(counts, null, 1))
