// T1 inventory of a cm6 package's test cases, as JSON Lines (ferrum/docs/fundation/1-parity-audit.md §10).
//   node cm6_tests.cjs --harness <harness dir> --package <name> > tests.jsonl
// One row per it(), webtest files included; "generated": true marks cases created inside a loop or callback
// (expand them by hand: one row per generated case).
const args = Object.fromEntries(process.argv.slice(2).join(" ").split("--").filter(Boolean).map(s => { const [k, ...v] = s.trim().split(/\s+/); return [k, v]; }));
const ts = require(args.harness[0] + "/node_modules/typescript"), fs = require("fs"), path = require("path");
const pkg = args.package[0], dir = `/workspace/CodeMirror6/${pkg}/test`;
for (const f of fs.readdirSync(dir).filter(f => /\.ts$/.test(f)).sort()) {
  const s = ts.createSourceFile(f, fs.readFileSync(path.join(dir, f), "utf8"), ts.ScriptTarget.ES2022, true); let n = 0;
  const visit = (node, desc, loop) => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && ["describe", "it"].includes(node.expression.text)) {
      const a0 = node.arguments[0], name = a0 && ts.isStringLiteralLike(a0) ? a0.text : a0 ? a0.getText(s) : "";
      if (node.expression.text === "it") { n++; console.log(JSON.stringify({id: `${f.replace(/\.ts$/, "")}#${String(n).padStart(2, "0")}`, file: `CodeMirror6/${pkg}/test/${f}`, line: s.getLineAndCharacterOfPosition(node.getStart(s)).line + 1, describe: desc.join(" > "), name, generated: loop, scenario: null, expected: null, behaviors: [], class: null, class_reason: null, port_notes: null})); return; }
      return ts.forEachChild(node, c => visit(c, [...desc, name], loop));
    }
    const isLoop = ts.isForStatement(node) || ts.isForOfStatement(node) || ts.isForInStatement(node) || (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && ["forEach", "map"].includes(node.expression.name.text));
    ts.forEachChild(node, c => visit(c, desc, loop || isLoop));
  };
  visit(s, [], false);
}
