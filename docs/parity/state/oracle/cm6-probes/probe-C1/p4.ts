import {Text} from "../src/index.ts"
try { Text.of(["ab","cd"]).lineAt(100) } catch (e: any) { console.log(e.constructor.name, e.message) }
try { Text.of(["ab","cd"]).line(101) } catch (e: any) { console.log(e.constructor.name, e.message) }
