import {EditorState} from "../src/index.ts"
try { EditorState.create({}).update({selection: {anchor: 5}}); console.log("no throw") } catch (e: any) { console.log("THROWS " + e.message) }
