import {EditorState} from "../src/index.ts"
const s = "é😀"; console.log(JSON.stringify(EditorState.create({doc: s, selection: {anchor: s.length}}).selection.toJSON()))
