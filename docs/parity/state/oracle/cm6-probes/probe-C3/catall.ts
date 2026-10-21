import {EditorState, CharCategory} from "../src/index.ts"
import {writeFileSync} from "fs"
const cat = EditorState.create({}).charCategorizer(0)
let out = new Array(0x110000)
for (let c = 0; c < 0x110000; c++) out[c] = (c >= 0xD800 && c < 0xE000) ? "-" : "WSO"[cat(String.fromCodePoint(c))]
writeFileSync(process.argv[2], out.join(""))
