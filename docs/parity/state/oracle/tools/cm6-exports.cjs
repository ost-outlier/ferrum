// Enumerate the public API of a cm6 package: every name re-exported by src/index.ts,
// plus public (non-private, non-@internal) members of exported classes/interfaces/enums,
// plus fields of non-exported config object types used as parameters of define()/update().
const ts = require(process.argv[2] + "/node_modules/typescript");
const path = require("path"), fs = require("fs");
const srcDir = process.argv[3];
const files = fs.readdirSync(srcDir).filter(f => f.endsWith(".ts"));
const sf = {};
for (const f of files) sf[f] = ts.createSourceFile(f, fs.readFileSync(path.join(srcDir, f), "utf8"), ts.ScriptTarget.ES2020, true);
const line = (s, n) => s.getLineAndCharacterOfPosition(n.getStart(s)).line + 1;
const internal = (s, n) => (ts.getLeadingCommentRanges(s.text, n.getFullStart()) || []).some(r => /@internal/.test(s.text.slice(r.pos, r.end)));
const isPrivate = n => (ts.getCombinedModifierFlags(n) & (ts.ModifierFlags.Private | ts.ModifierFlags.Protected)) || (n.name && ts.isPrivateIdentifier(n.name));
const isStatic = n => !!(ts.getCombinedModifierFlags(n) & ts.ModifierFlags.Static);
// names exported from index
const index = sf["index.ts"]; const exported = [];
index.statements.forEach(st => { if (ts.isExportDeclaration(st)) { const mod = st.moduleSpecifier.text.replace("./", "") + ".ts"; st.exportClause.elements.forEach(e => exported.push([e.name.text, mod])); } });
const out = []; const configTypes = { FacetConfig: "Facet.define config", StateFieldSpec: "StateField.define config", StateEffectSpec: "StateEffect.define spec", RangeSetUpdate: "RangeSet.update spec" };
function findDecl(s, name) { for (const st of s.statements) { if (st.name && st.name.text === name) return st; if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) if (d.name.text === name) return d; } }
function members(s, owner, node) {
  const list = node.members || (node.type && node.type.members) || [];
  for (const m of list) {
    if (ts.isConstructorDeclaration(m)) {
      for (const prm of m.parameters) { const fl = ts.getCombinedModifierFlags(prm); if (prm.modifiers && prm.modifiers.length && !(fl & (ts.ModifierFlags.Private | ts.ModifierFlags.Protected)) && !internal(s, prm)) out.push([owner + "." + prm.name.text, "property (ctor param)", s.fileName, line(s, prm)]); } if (!isPrivate(m) && !internal(s, m) && owner === "RangeSetBuilder") out.push([owner + ".constructor", "constructor", s.fileName, line(s, m)]); continue; }
    if (!m.name || isPrivate(m) || internal(s, m) || ts.isIndexSignatureDeclaration(m)) continue;
    let nm = m.name.getText(s); if (ts.isComputedPropertyName(m.name)) nm = nm;
    const kind = ts.isEnumMember(m) ? "enum member" : ts.isGetAccessor(m) ? "getter" : (ts.isMethodDeclaration(m) || ts.isMethodSignature(m)) ? "method" : "property";
    const key = owner + (isStatic(m) ? "^" : ".") + nm;
    if (out.some(o => o[0] === key)) continue; // overloads
    out.push([key, (isStatic(m) ? "static " : "") + kind, s.fileName, line(s, m)]);
  }
}
for (const [name, file] of exported) {
  const s = sf[file]; const d = findDecl(s, name);
  if (!d) { out.push([name, "NOT FOUND", file, 0]); continue; }
  const kind = ts.isClassDeclaration(d) ? "class" : ts.isInterfaceDeclaration(d) ? "interface" : ts.isTypeAliasDeclaration(d) ? "type" : ts.isEnumDeclaration(d) ? "enum" : ts.isFunctionDeclaration(d) ? "function" : "const";
  out.push([name, kind, file, line(s, d)]);
  if (ts.isClassDeclaration(d) || ts.isInterfaceDeclaration(d) || ts.isEnumDeclaration(d)) members(s, name, d);
  if (ts.isTypeAliasDeclaration(d) && d.type.members) members(s, name, d);
  if (name === "Prec") d.initializer.properties.forEach(p => out.push(["Prec." + p.name.text, "property", file, line(s, p)]));
}
for (const [t, label] of Object.entries(configTypes)) for (const f of files) { const d = findDecl(sf[f], t); if (d) { const start = out.length; members(sf[f], label, d); } }
out.forEach((o, i) => console.log([String(i + 1).padStart(3, "0"), ...o].join("\t")));
