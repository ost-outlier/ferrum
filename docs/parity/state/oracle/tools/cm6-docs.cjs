// For each A1 row (file:line), print the first sentence of the preceding /// doc comment.
const fs = require("fs");
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => l.split("\t"));
const cache = {};
for (const [n, name, kind, file, line] of rows) {
  const src = cache[file] || (cache[file] = fs.readFileSync(process.argv[3] + "/" + file, "utf8").split("\n"));
  let i = +line - 2, doc = [];
  while (i >= 0 && /^\s*\/\/\//.test(src[i])) { doc.unshift(src[i].replace(/^\s*\/\/\/\s?/, "")); i--; }
  if (!doc.length) { // ctor param / member with comment directly above? try same line trailing
    doc = [""];
  }
  let text = doc.join(" ").replace(/\s+/g, " ").trim();
  let first = (text.match(/^.*?[.!?](\s|$)/) || [text])[0].trim();
  first = first.replace(/\[([^\]]*)\]\([^)]*\)/g, "$1").replace(/\|/g, "\\|");
  console.log([n, first].join("\t"));
}
