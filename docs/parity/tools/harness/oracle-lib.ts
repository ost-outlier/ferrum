// Shared helpers for cm6 probes and generators (ferrum/docs/fundation/1-parity-audit.md §7–§8).
// Imported as "@harness/lib" (mapped by setup_cm6.py). Keep it small and stable: every audit uses it.
import {writeFileSync, appendFileSync} from "node:fs"

/** Seeded PRNG (mulberry32). Record the seed in the generator header and in every fixture. */
export function rng(seed: number) {
  let a = seed >>> 0
  const next = () => { a = (a + 0x6D2B79F5) >>> 0; let t = a; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296 }
  return {next, int: (n: number) => Math.floor(next() * n), pick: <T>(xs: readonly T[]) => xs[Math.floor(next() * xs.length)], chance: (p: number) => next() < p}
}

/** Strings that Rust can hold: lone surrogates become {utf16: [code units]} (§6.2). */
export function encodeStr(s: string): string | {utf16: number[]} {
  for (let i = 0; i < s.length; i++) {
    const c = s.charCodeAt(i)
    if (c >= 0xD800 && c <= 0xDBFF) { const d = s.charCodeAt(i + 1); if (d >= 0xDC00 && d <= 0xDFFF) { i++; continue } }
    if (c >= 0xD800 && c <= 0xDFFF) return {utf16: Array.from(s, (_, j) => s.charCodeAt(j)).slice(0, s.length)}
  }
  return s
}

/** Deep-encode every string of a value with encodeStr. */
export function encode(v: any): any {
  if (typeof v === "string") return encodeStr(v)
  if (Array.isArray(v)) return v.map(encode)
  if (v && typeof v === "object") return Object.fromEntries(Object.entries(v).map(([k, x]) => [k, encode(x)]))
  return v
}

/** Run f and record either its result or the thrown error as the output (§6.2: exceptions are outputs). */
export function capture<T>(f: () => T): {ok: any} | {error: {name: string, message: string}} {
  try { return {ok: encode(f())} } catch (e: any) { return {error: {name: e?.name ?? "Error", message: String(e?.message ?? e)}} }
}

/** Append fixtures as JSON Lines: {id, item_id, behaviors, input, output, encoding}. */
export function fixtureWriter(path: string, fresh = true) {
  if (fresh) writeFileSync(path, "")
  let n = 0
  return (item_id: string, behaviors: string[], input: any, output: any, id?: string) => {
    n++; appendFileSync(path, JSON.stringify({id: id ?? `F${String(n).padStart(4, "0")}`, item_id, behaviors, input: encode(input), output, encoding: "utf16"}) + "\n")
  }
}
