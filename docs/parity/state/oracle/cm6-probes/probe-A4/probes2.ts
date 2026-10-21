import {EditorState, CharCategory, Text} from "../src/index.ts"
let cat = EditorState.create({}).charCategorizer(0)
console.log("Q1", CharCategory[cat("\u0301e")], CharCategory[cat("")])
console.log("Q2", Text.empty.lines)
