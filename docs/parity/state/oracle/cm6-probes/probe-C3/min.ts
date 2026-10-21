import {EditorSelection} from "../src/index.ts"
let s = EditorSelection.create([EditorSelection.cursor(12), EditorSelection.range(12, 18), EditorSelection.cursor(12)], 2)
console.log(s.ranges.map(r => r.anchor + "/" + r.head).join(","), "main=" + s.mainIndex)
