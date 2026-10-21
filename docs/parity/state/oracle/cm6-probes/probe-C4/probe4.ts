import {EditorState, Facet} from "../src/index.ts"
const num = Facet.define<number>()
let st = EditorState.create({extensions: [num.computeN(["doc"], s => s.doc.length % 2 ? [100, 10] : []), num.of(1), Facet.define<number>().compute(["doc"], s => 0)]})
st = st.update({changes: {insert: "😀", from: 0}}).state
console.log("D1 computeN parity after inserting U+1F600: " + st.facet(num).join() + " doc.length=" + st.doc.length)
