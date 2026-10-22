// A1 inventory of a cm6 package's public API, as JSON Lines (ferrum/docs/fundation/1-parity-audit.md §5).
//   node cm6_inventory.cjs --harness <harness dir> --package <name> --prefix <crate>.cm6 > inventory.jsonl
// Rows: every name re-exported by src/index.ts; every public (non-private, non-@internal) member of
// exported classes, interfaces, enums and object types (static members as X^m, constructor parameter
// properties); every field of non-exported object types that exported functions/methods take as a
// parameter (config objects: detected automatically, reported with kind "config field").
const args = Object.fromEntries(process.argv.slice(2).join(" ").split("--").filter(Boolean).map(s => { const [k, ...v] = s.trim().split(/\s+/); return [k, v]; }));
const ts = require(args.harness[0] + "/node_modules/typescript"), fs = require("fs"), path = require("path");
const pkg = args.package[0], prefix = args.prefix[0], src = `${args.harness[0]}/cm6/${pkg}/src`;
const files = fs.readdirSync(src).filter(f => f.endsWith(".ts") && !f.endsWith(".d.ts"));
const sf = Object.fromEntries(files.map(f => [f, ts.createSourceFile(f, fs.readFileSync(path.join(src, f), "utf8"), ts.ScriptTarget.ES2022, true)]));
const line = (s, n) => s.getLineAndCharacterOfPosition(n.getStart(s)).line + 1;
const internal = (s, n) => (ts.getLeadingCommentRanges(s.text, n.getFullStart()) || []).some(r => /@internal/.test(s.text.slice(r.pos, r.end)));
const flags = n => ts.getCombinedModifierFlags(n);
const hidden = n => (flags(n) & (ts.ModifierFlags.Private | ts.ModifierFlags.Protected)) || (n.name && ts.isPrivateIdentifier(n.name));
const isStatic = n => !!(flags(n) & ts.ModifierFlags.Static);
const out = [], seen = new Set();
const add = (item, kind, s, n) => { if (seen.has(item)) return; seen.add(item); out.push({item, kind, file: `CodeMirror6/${pkg}/src/${s.fileName}`, line: line(s, n)}); };
function findDecl(s, name) { for (const st of s.statements) { if (st.name && st.name.text === name) return st; if (ts.isVariableStatement(st)) for (const d of st.declarationList.declarations) if (d.name.text === name) return d; } }
const exported = [];
sf["index.ts"].statements.forEach(st => { if (ts.isExportDeclaration(st) && st.exportClause) { const mod = st.moduleSpecifier.text.replace("./", "") + ".ts"; st.exportClause.elements.forEach(e => exported.push([(e.propertyName || e.name).text, e.name.text, mod])); } });
const exportedNames = new Set(exported.map(e => e[0]));
const localObjTypes = {}; // name -> [file, decl]
for (const [f, s] of Object.entries(sf)) for (const st of s.statements) if ((ts.isInterfaceDeclaration(st) || (ts.isTypeAliasDeclaration(st) && ts.isTypeLiteralNode(st.type))) && !exportedNames.has(st.name.text)) localObjTypes[st.name.text] = [f, st];
const configUses = {}; // local type name -> first owner label
function refs(t, acc) { if (!t) return; if (ts.isTypeReferenceNode(t) && ts.isIdentifier(t.typeName)) acc.push(t.typeName.text); ts.forEachChild(t, c => refs(c, acc)); }
function noteParams(owner, fn) { for (const p of fn.parameters || []) { const acc = []; refs(p.type, acc); for (const r of acc) if (localObjTypes[r] && !configUses[r]) configUses[r] = owner; } }
function members(s, owner, list) {
  for (const m of list) {
    if (ts.isConstructorDeclaration(m)) {
      for (const p of m.parameters) if (p.modifiers && p.modifiers.length && !hidden(p) && !internal(s, p)) add(`${owner}.${p.name.text}`, "property (ctor param)", s, p);
      if (!hidden(m) && !internal(s, m)) { add(`${owner}.constructor`, "constructor", s, m); noteParams(`new ${owner}`, m); }
      continue;
    }
    if (!m.name || hidden(m) || internal(s, m) || ts.isIndexSignatureDeclaration(m)) continue;
    const kind = ts.isEnumMember(m) ? "enum member" : ts.isGetAccessor(m) ? "getter" : (ts.isMethodDeclaration(m) || ts.isMethodSignature(m)) ? "method" : "property";
    const key = `${owner}${isStatic(m) ? "^" : "."}${m.name.getText(s)}`;
    add(key, (isStatic(m) ? "static " : "") + kind, s, m);
    if (kind === "method") noteParams(key, m);
  }
}
for (const [name, alias, file] of exported) {
  const s = sf[file], d = findDecl(s, name);
  if (!d) { out.push({item: alias, kind: "NOT FOUND", file: `CodeMirror6/${pkg}/src/${file}`, line: 0}); continue; }
  const kind = ts.isClassDeclaration(d) ? "class" : ts.isInterfaceDeclaration(d) ? "interface" : ts.isTypeAliasDeclaration(d) ? "type" : ts.isEnumDeclaration(d) ? "enum" : ts.isFunctionDeclaration(d) ? "function" : "const";
  add(alias, kind, s, d);
  if (ts.isFunctionDeclaration(d)) noteParams(alias, d);
  if (ts.isClassDeclaration(d) || ts.isInterfaceDeclaration(d) || ts.isEnumDeclaration(d)) members(s, alias, d.members);
  else if (ts.isTypeAliasDeclaration(d) && d.type.members) members(s, alias, d.type.members);
  else if (ts.isVariableDeclaration(d) && d.initializer && ts.isObjectLiteralExpression(d.initializer)) d.initializer.properties.forEach(p => p.name && add(`${alias}.${p.name.getText(s)}`, "property", s, p));
}
for (const [t, owner] of Object.entries(configUses)) { const [f, d] = localObjTypes[t]; const list = d.members || d.type.members; for (const m of list) if (m.name && !internal(sf[f], m)) add(`${owner} config ${t}.${m.name.getText(sf[f])}`, "config field", sf[f], m); }
out.forEach((o, i) => console.log(JSON.stringify({id: `${prefix}.${String(i + 1).padStart(3, "0")}`, ...o, chunk: null, disposition: null, reason: null})));
