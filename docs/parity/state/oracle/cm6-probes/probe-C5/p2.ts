import {EditorState} from "../src/index.ts"
let s = EditorState.create({doc: "a\nb\r\nc", extensions: EditorState.lineSeparator.of("\r\n")})
console.log("R21c =>", s.doc.lines, s.doc.length)
