import {EditorState, Text} from "../src/index.ts"
for (let s of ["a\r\nb", "a\rb", "a b", "a\u0085b\u000bc\u000cd"]) {
  let st = EditorState.create({doc: s})
  console.log(JSON.stringify(s), "-> doc length", st.doc.length, "lines", st.doc.lines, "toJSON", JSON.stringify(st.doc.toJSON()), "sliceDoc", JSON.stringify(st.sliceDoc()))
}
let crlf = EditorState.create({doc: "a\r\nb", extensions: EditorState.lineSeparator.of("\r\n")})
console.log("lineSeparator=\\r\\n: length", crlf.doc.length, "lines", crlf.doc.lines, "sliceDoc", JSON.stringify(crlf.sliceDoc()), "toString", JSON.stringify(crlf.doc.toString()))
