// List every it() in cm6 test files with describe path, line, and whether it is generated in a loop/callback.
const ts = require(process.argv[2] + "/node_modules/typescript"); const fs = require("fs"), path = require("path");
const dir = process.argv[3];
for (const f of fs.readdirSync(dir).filter(f => /\.ts$/.test(f)).sort()) {
  const s = ts.createSourceFile(f, fs.readFileSync(path.join(dir, f), "utf8"), ts.ScriptTarget.ES2020, true);
  let n = 0;
  const visit = (node, desc, loop) => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && ["describe", "it"].includes(node.expression.text)) {
      const name = node.arguments[0] && ts.isStringLiteralLike(node.arguments[0]) ? node.arguments[0].text : node.arguments[0].getText(s);
      if (node.expression.text === "it") { n++; console.log([f, String(n).padStart(2, "0"), s.getLineAndCharacterOfPosition(node.getStart(s)).line + 1, desc.join(" > "), name, loop ? "LOOP-GENERATED" : ""].join("\t")); return; }
      return ts.forEachChild(node, c => visit(c, [...desc, name], loop));
    }
    const isLoop = ts.isForStatement(node) || ts.isForOfStatement(node) || ts.isForInStatement(node) || (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && ["forEach", "map"].includes(node.expression.name.text));
    ts.forEachChild(node, c => visit(c, desc, loop || isLoop));
  };
  visit(s, [], false);
}
