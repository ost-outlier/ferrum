# A1 raw inventory: `@codemirror/state` 6.7.6 public API

> What: A1 raw inventory of the cm6 public API. Versions: cm6 `@codemirror/state` 6.7.6 (`mirrors/cm6/state/package.json`); Ferrum line numbers at commit `fb99ca1` (`crates/` unchanged through `ed70610`). Ids: id schemes and path rules in `docs/parity/README.md`.

Generated mechanically from `mirrors/cm6/state/src/index.ts` with the TypeScript compiler API (script: `../oracle/tools/cm6-exports.cjs`).
Included: every name re-exported by `index.ts`; every non-private, non-`@internal` member of exported classes, interfaces, enums and object types (instance members as `X.m`, static members as `X^m`, public constructor parameter properties); every field of the non-exported config object types accepted by `Facet.define`, `StateField.define`, `StateEffect.define` and `RangeSet.update` (their fields are public API through those parameters).
Excluded: `private` members, `@internal` members, module exports not re-exported by `index.ts` (`DefaultSplit`, `checkSelection`, `resolveTransaction`, `asArray`, `makeCategorizer`, `CompartmentInstance`, `Configuration`, `SlotStatus`, `ensureAddr`, `getAddr`, `languageData`/`allowMultipleSelections`/... module-level consts in `extension.ts` that are re-exposed as `EditorState^…`). Excluded items are still read as helpers of the items that use them.

**Total: 264 rows** (39 top-level exports).

| id | cm6 item | kind | cm6 file:line | chunk | proposed disposition |
|---|---|---|---|---|---|
| state.cm6.001 | `EditorStateConfig` | interface | `mirrors/cm6/state/src/state.ts:14` | C5 | CORE |
| state.cm6.002 | `EditorStateConfig.doc` | property | `mirrors/cm6/state/src/state.ts:21` | C5 | CORE |
| state.cm6.003 | `EditorStateConfig.selection` | property | `mirrors/cm6/state/src/state.ts:24` | C5 | CORE |
| state.cm6.004 | `EditorStateConfig.extensions` | property | `mirrors/cm6/state/src/state.ts:26` | C5 | CORE |
| state.cm6.005 | `EditorState` | class | `mirrors/cm6/state/src/state.ts:36` | C5 | CORE |
| state.cm6.006 | `EditorState.doc` | property (ctor param) | `mirrors/cm6/state/src/state.ts:46` | C5 | CORE |
| state.cm6.007 | `EditorState.selection` | property (ctor param) | `mirrors/cm6/state/src/state.ts:48` | C5 | CORE |
| state.cm6.008 | `EditorState.field` | method | `mirrors/cm6/state/src/state.ts:66` | C5 | CORE |
| state.cm6.009 | `EditorState.update` | method | `mirrors/cm6/state/src/state.ts:91` | C5 | CORE |
| state.cm6.010 | `EditorState.replaceSelection` | method | `mirrors/cm6/state/src/state.ts:129` | C5 | CORE |
| state.cm6.011 | `EditorState.changeByRange` | method | `mirrors/cm6/state/src/state.ts:144` | C5 | CORE |
| state.cm6.012 | `EditorState.changes` | method | `mirrors/cm6/state/src/state.ts:176` | C5 | CORE |
| state.cm6.013 | `EditorState.toText` | method | `mirrors/cm6/state/src/state.ts:184` | C5 | CORE |
| state.cm6.014 | `EditorState.sliceDoc` | method | `mirrors/cm6/state/src/state.ts:189` | C5 | CORE |
| state.cm6.015 | `EditorState.facet` | method | `mirrors/cm6/state/src/state.ts:194` | C5 | CORE |
| state.cm6.016 | `EditorState.toJSON` | method | `mirrors/cm6/state/src/state.ts:205` | C5 | CORE |
| state.cm6.017 | `EditorState^fromJSON` | static method | `mirrors/cm6/state/src/state.ts:222` | C5 | CORE |
| state.cm6.018 | `EditorState^create` | static method | `mirrors/cm6/state/src/state.ts:243` | C5 | CORE |
| state.cm6.019 | `EditorState^allowMultipleSelections` | static property | `mirrors/cm6/state/src/state.ts:262` | C5 | CORE |
| state.cm6.020 | `EditorState^tabSize` | static property | `mirrors/cm6/state/src/state.ts:267` | C5 | CORE |
| state.cm6.021 | `EditorState.tabSize` | getter | `mirrors/cm6/state/src/state.ts:273` | C5 | CORE |
| state.cm6.022 | `EditorState^lineSeparator` | static property | `mirrors/cm6/state/src/state.ts:282` | C5 | CORE |
| state.cm6.023 | `EditorState.lineBreak` | getter | `mirrors/cm6/state/src/state.ts:286` | C5 | CORE |
| state.cm6.024 | `EditorState^readOnly` | static property | `mirrors/cm6/state/src/state.ts:299` | C5 | CORE |
| state.cm6.025 | `EditorState.readOnly` | getter | `mirrors/cm6/state/src/state.ts:303` | C5 | CORE |
| state.cm6.026 | `EditorState^phrases` | static property | `mirrors/cm6/state/src/state.ts:309` | C5 | CORE |
| state.cm6.027 | `EditorState.phrase` | method | `mirrors/cm6/state/src/state.ts:324` | C5 | CORE |
| state.cm6.028 | `EditorState^languageData` | static property | `mirrors/cm6/state/src/state.ts:337` | C5 | CORE |
| state.cm6.029 | `EditorState.languageDataAt` | method | `mirrors/cm6/state/src/state.ts:353` | C5 | CORE |
| state.cm6.030 | `EditorState.charCategorizer` | method | `mirrors/cm6/state/src/state.ts:373` | C5 | CORE |
| state.cm6.031 | `EditorState.wordAt` | method | `mirrors/cm6/state/src/state.ts:382` | C5 | CORE |
| state.cm6.032 | `EditorState^changeFilter` | static property | `mirrors/cm6/state/src/state.ts:411` | C5 | CORE |
| state.cm6.033 | `EditorState^transactionFilter` | static property | `mirrors/cm6/state/src/state.ts:430` | C5 | CORE |
| state.cm6.034 | `EditorState^transactionExtender` | static property | `mirrors/cm6/state/src/state.ts:443` | C5 | CORE |
| state.cm6.035 | `StateCommand` | type | `mirrors/cm6/state/src/extension.ts:10` | C4 | CORE |
| state.cm6.036 | `Facet` | class | `mirrors/cm6/state/src/facet.ts:43` | C4 | CORE |
| state.cm6.037 | `Facet.reader` | getter | `mirrors/cm6/state/src/facet.ts:67` | C4 | CORE |
| state.cm6.038 | `Facet^define` | static method | `mirrors/cm6/state/src/facet.ts:70` | C4 | CORE |
| state.cm6.039 | `Facet.of` | method | `mirrors/cm6/state/src/facet.ts:79` | C4 | CORE |
| state.cm6.040 | `Facet.compute` | method | `mirrors/cm6/state/src/facet.ts:90` | C4 | CORE |
| state.cm6.041 | `Facet.computeN` | method | `mirrors/cm6/state/src/facet.ts:97` | C4 | CORE |
| state.cm6.042 | `Facet.from` | method | `mirrors/cm6/state/src/facet.ts:106` | C4 | CORE |
| state.cm6.043 | `Facet.tag` | property | `mirrors/cm6/state/src/facet.ts:113` | C4 | DROP |
| state.cm6.044 | `FacetReader` | type | `mirrors/cm6/state/src/facet.ts:120` | C4 | CORE |
| state.cm6.045 | `FacetReader.tag` | property | `mirrors/cm6/state/src/facet.ts:128` | C4 | DROP |
| state.cm6.046 | `StateField` | class | `mirrors/cm6/state/src/facet.ts:296` | C4 | CORE |
| state.cm6.047 | `StateField^define` | static method | `mirrors/cm6/state/src/facet.ts:311` | C4 | CORE |
| state.cm6.048 | `StateField.init` | method | `mirrors/cm6/state/src/facet.ts:356` | C4 | CORE |
| state.cm6.049 | `StateField.extension` | getter | `mirrors/cm6/state/src/facet.ts:363` | C4 | CORE |
| state.cm6.050 | `Extension` | type | `mirrors/cm6/state/src/facet.ts:374` | C4 | CORE |
| state.cm6.051 | `Prec` | const | `mirrors/cm6/state/src/facet.ts:390` | C4 | CORE |
| state.cm6.052 | `Prec.highest` | property | `mirrors/cm6/state/src/facet.ts:393` | C4 | CORE |
| state.cm6.053 | `Prec.high` | property | `mirrors/cm6/state/src/facet.ts:396` | C4 | CORE |
| state.cm6.054 | `Prec.default` | property | `mirrors/cm6/state/src/facet.ts:399` | C4 | CORE |
| state.cm6.055 | `Prec.low` | property | `mirrors/cm6/state/src/facet.ts:401` | C4 | CORE |
| state.cm6.056 | `Prec.lowest` | property | `mirrors/cm6/state/src/facet.ts:404` | C4 | CORE |
| state.cm6.057 | `Compartment` | class | `mirrors/cm6/state/src/facet.ts:418` | C4 | CORE |
| state.cm6.058 | `Compartment.of` | method | `mirrors/cm6/state/src/facet.ts:421` | C4 | CORE |
| state.cm6.059 | `Compartment.reconfigure` | method | `mirrors/cm6/state/src/facet.ts:425` | C4 | CORE |
| state.cm6.060 | `Compartment.get` | method | `mirrors/cm6/state/src/facet.ts:431` | C4 | CORE |
| state.cm6.061 | `EditorSelection` | class | `mirrors/cm6/state/src/selection.ts:115` | C3 | CORE |
| state.cm6.062 | `EditorSelection.ranges` | property (ctor param) | `mirrors/cm6/state/src/selection.ts:119` | C3 | CORE |
| state.cm6.063 | `EditorSelection.mainIndex` | property (ctor param) | `mirrors/cm6/state/src/selection.ts:122` | C3 | CORE |
| state.cm6.064 | `EditorSelection.map` | method | `mirrors/cm6/state/src/selection.ts:127` | C3 | CORE |
| state.cm6.065 | `EditorSelection.eq` | method | `mirrors/cm6/state/src/selection.ts:136` | C3 | CORE |
| state.cm6.066 | `EditorSelection.main` | getter | `mirrors/cm6/state/src/selection.ts:147` | C3 | CORE |
| state.cm6.067 | `EditorSelection.asSingle` | method | `mirrors/cm6/state/src/selection.ts:151` | C3 | CORE |
| state.cm6.068 | `EditorSelection.addRange` | method | `mirrors/cm6/state/src/selection.ts:156` | C3 | CORE |
| state.cm6.069 | `EditorSelection.replaceRange` | method | `mirrors/cm6/state/src/selection.ts:162` | C3 | CORE |
| state.cm6.070 | `EditorSelection.toJSON` | method | `mirrors/cm6/state/src/selection.ts:170` | C3 | CORE |
| state.cm6.071 | `EditorSelection^fromJSON` | static method | `mirrors/cm6/state/src/selection.ts:175` | C3 | CORE |
| state.cm6.072 | `EditorSelection^single` | static method | `mirrors/cm6/state/src/selection.ts:182` | C3 | CORE |
| state.cm6.073 | `EditorSelection^create` | static method | `mirrors/cm6/state/src/selection.ts:188` | C3 | CORE |
| state.cm6.074 | `EditorSelection^cursor` | static method | `mirrors/cm6/state/src/selection.ts:200` | C3 | CORE |
| state.cm6.075 | `EditorSelection^range` | static method | `mirrors/cm6/state/src/selection.ts:208` | C3 | CORE |
| state.cm6.076 | `EditorSelection^undirectionalRange` | static method | `mirrors/cm6/state/src/selection.ts:218` | C3 | CORE |
| state.cm6.077 | `SelectionRange` | class | `mirrors/cm6/state/src/selection.ts:24` | C3 | CORE |
| state.cm6.078 | `SelectionRange.from` | property (ctor param) | `mirrors/cm6/state/src/selection.ts:27` | C3 | CORE |
| state.cm6.079 | `SelectionRange.to` | property (ctor param) | `mirrors/cm6/state/src/selection.ts:29` | C3 | CORE |
| state.cm6.080 | `SelectionRange.goalColumn` | property (ctor param) | `mirrors/cm6/state/src/selection.ts:35` | C3 | CORE |
| state.cm6.081 | `SelectionRange.anchor` | getter | `mirrors/cm6/state/src/selection.ts:40` | C3 | CORE |
| state.cm6.082 | `SelectionRange.head` | getter | `mirrors/cm6/state/src/selection.ts:44` | C3 | CORE |
| state.cm6.083 | `SelectionRange.empty` | getter | `mirrors/cm6/state/src/selection.ts:47` | C3 | CORE |
| state.cm6.084 | `SelectionRange.assoc` | getter | `mirrors/cm6/state/src/selection.ts:53` | C3 | CORE |
| state.cm6.085 | `SelectionRange.undirectional` | getter | `mirrors/cm6/state/src/selection.ts:60` | C3 | CORE |
| state.cm6.086 | `SelectionRange.bidiLevel` | getter | `mirrors/cm6/state/src/selection.ts:66` | C3 | CORE |
| state.cm6.087 | `SelectionRange.map` | method | `mirrors/cm6/state/src/selection.ts:73` | C3 | CORE |
| state.cm6.088 | `SelectionRange.extend` | method | `mirrors/cm6/state/src/selection.ts:85` | C3 | CORE |
| state.cm6.089 | `SelectionRange.eq` | method | `mirrors/cm6/state/src/selection.ts:92` | C3 | CORE |
| state.cm6.090 | `SelectionRange.toJSON` | method | `mirrors/cm6/state/src/selection.ts:98` | C3 | CORE |
| state.cm6.091 | `SelectionRange^fromJSON` | static method | `mirrors/cm6/state/src/selection.ts:102` | C3 | CORE |
| state.cm6.092 | `Transaction` | class | `mirrors/cm6/state/src/transaction.ts:163` | C5 | CORE |
| state.cm6.093 | `Transaction.startState` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:171` | C5 | CORE |
| state.cm6.094 | `Transaction.changes` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:173` | C5 | CORE |
| state.cm6.095 | `Transaction.selection` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:176` | C5 | CORE |
| state.cm6.096 | `Transaction.effects` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:178` | C5 | CORE |
| state.cm6.097 | `Transaction.scrollIntoView` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:183` | C5 | CORE |
| state.cm6.098 | `Transaction.newDoc` | getter | `mirrors/cm6/state/src/transaction.ts:203` | C5 | CORE |
| state.cm6.099 | `Transaction.newSelection` | getter | `mirrors/cm6/state/src/transaction.ts:211` | C5 | CORE |
| state.cm6.100 | `Transaction.state` | getter | `mirrors/cm6/state/src/transaction.ts:219` | C5 | CORE |
| state.cm6.101 | `Transaction.annotation` | method | `mirrors/cm6/state/src/transaction.ts:225` | C5 | CORE |
| state.cm6.102 | `Transaction.docChanged` | getter | `mirrors/cm6/state/src/transaction.ts:231` | C5 | CORE |
| state.cm6.103 | `Transaction.reconfigured` | getter | `mirrors/cm6/state/src/transaction.ts:237` | C5 | CORE |
| state.cm6.104 | `Transaction.isUserEvent` | method | `mirrors/cm6/state/src/transaction.ts:244` | C5 | CORE |
| state.cm6.105 | `Transaction^time` | static property | `mirrors/cm6/state/src/transaction.ts:251` | C5 | CORE |
| state.cm6.106 | `Transaction^userEvent` | static property | `mirrors/cm6/state/src/transaction.ts:277` | C5 | CORE |
| state.cm6.107 | `Transaction^addToHistory` | static property | `mirrors/cm6/state/src/transaction.ts:281` | C5 | CORE |
| state.cm6.108 | `Transaction^remote` | static property | `mirrors/cm6/state/src/transaction.ts:287` | C5 | CORE |
| state.cm6.109 | `TransactionSpec` | interface | `mirrors/cm6/state/src/transaction.ts:121` | C5 | CORE |
| state.cm6.110 | `TransactionSpec.changes` | property | `mirrors/cm6/state/src/transaction.ts:123` | C5 | CORE |
| state.cm6.111 | `TransactionSpec.selection` | property | `mirrors/cm6/state/src/transaction.ts:127` | C5 | CORE |
| state.cm6.112 | `TransactionSpec.effects` | property | `mirrors/cm6/state/src/transaction.ts:132` | C5 | CORE |
| state.cm6.113 | `TransactionSpec.annotations` | property | `mirrors/cm6/state/src/transaction.ts:134` | C5 | CORE |
| state.cm6.114 | `TransactionSpec.userEvent` | property | `mirrors/cm6/state/src/transaction.ts:136` | C5 | CORE |
| state.cm6.115 | `TransactionSpec.scrollIntoView` | property | `mirrors/cm6/state/src/transaction.ts:139` | C5 | CORE |
| state.cm6.116 | `TransactionSpec.filter` | property | `mirrors/cm6/state/src/transaction.ts:146` | C5 | CORE |
| state.cm6.117 | `TransactionSpec.sequential` | property | `mirrors/cm6/state/src/transaction.ts:153` | C5 | CORE |
| state.cm6.118 | `Annotation` | class | `mirrors/cm6/state/src/transaction.ts:15` | C5 | CORE |
| state.cm6.119 | `Annotation.type` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:19` | C5 | CORE |
| state.cm6.120 | `Annotation.value` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:21` | C5 | CORE |
| state.cm6.121 | `Annotation^define` | static method | `mirrors/cm6/state/src/transaction.ts:25` | C5 | CORE |
| state.cm6.122 | `AnnotationType` | class | `mirrors/cm6/state/src/transaction.ts:33` | C5 | CORE |
| state.cm6.123 | `AnnotationType.of` | method | `mirrors/cm6/state/src/transaction.ts:35` | C5 | CORE |
| state.cm6.124 | `StateEffect` | class | `mirrors/cm6/state/src/transaction.ts:69` | C5 | CORE |
| state.cm6.125 | `StateEffect.value` | property (ctor param) | `mirrors/cm6/state/src/transaction.ts:75` | C5 | CORE |
| state.cm6.126 | `StateEffect.map` | method | `mirrors/cm6/state/src/transaction.ts:79` | C5 | CORE |
| state.cm6.127 | `StateEffect.is` | method | `mirrors/cm6/state/src/transaction.ts:86` | C5 | CORE |
| state.cm6.128 | `StateEffect^define` | static method | `mirrors/cm6/state/src/transaction.ts:93` | C5 | CORE |
| state.cm6.129 | `StateEffect^mapEffects` | static method | `mirrors/cm6/state/src/transaction.ts:98` | C5 | CORE |
| state.cm6.130 | `StateEffect^reconfigure` | static property | `mirrors/cm6/state/src/transaction.ts:113` | C5 | CORE |
| state.cm6.131 | `StateEffect^appendConfig` | static property | `mirrors/cm6/state/src/transaction.ts:116` | C5 | CORE |
| state.cm6.132 | `StateEffectType` | class | `mirrors/cm6/state/src/transaction.ts:48` | C5 | CORE |
| state.cm6.133 | `StateEffectType.of` | method | `mirrors/cm6/state/src/transaction.ts:61` | C5 | CORE |
| state.cm6.134 | `combineConfig` | function | `mirrors/cm6/state/src/config.ts:8` | C4 | CORE |
| state.cm6.135 | `ChangeSpec` | type | `mirrors/cm6/state/src/change.ts:188` | C2 | CORE |
| state.cm6.136 | `ChangeSet` | class | `mirrors/cm6/state/src/change.ts:196` | C2 | CORE |
| state.cm6.137 | `ChangeSet.apply` | method | `mirrors/cm6/state/src/change.ts:207` | C2 | CORE |
| state.cm6.138 | `ChangeSet.mapDesc` | method | `mirrors/cm6/state/src/change.ts:213` | C2 | CORE |
| state.cm6.139 | `ChangeSet.invert` | method | `mirrors/cm6/state/src/change.ts:219` | C2 | CORE |
| state.cm6.140 | `ChangeSet.compose` | method | `mirrors/cm6/state/src/change.ts:238` | C2 | CORE |
| state.cm6.141 | `ChangeSet.map` | method | `mirrors/cm6/state/src/change.ts:251` | C2 | CORE |
| state.cm6.142 | `ChangeSet.iterChanges` | method | `mirrors/cm6/state/src/change.ts:260` | C2 | CORE |
| state.cm6.143 | `ChangeSet.desc` | getter | `mirrors/cm6/state/src/change.ts:266` | C2 | CORE |
| state.cm6.144 | `ChangeSet.toJSON` | method | `mirrors/cm6/state/src/change.ts:299` | C2 | CORE |
| state.cm6.145 | `ChangeSet^of` | static method | `mirrors/cm6/state/src/change.ts:312` | C2 | CORE |
| state.cm6.146 | `ChangeSet^empty` | static method | `mirrors/cm6/state/src/change.ts:352` | C2 | CORE |
| state.cm6.147 | `ChangeSet^fromJSON` | static method | `mirrors/cm6/state/src/change.ts:358` | C2 | CORE |
| state.cm6.148 | `ChangeDesc` | class | `mirrors/cm6/state/src/change.ts:21` | C2 | CORE |
| state.cm6.149 | `ChangeDesc.length` | getter | `mirrors/cm6/state/src/change.ts:34` | C2 | CORE |
| state.cm6.150 | `ChangeDesc.newLength` | getter | `mirrors/cm6/state/src/change.ts:41` | C2 | CORE |
| state.cm6.151 | `ChangeDesc.empty` | getter | `mirrors/cm6/state/src/change.ts:51` | C2 | CORE |
| state.cm6.152 | `ChangeDesc.iterGaps` | method | `mirrors/cm6/state/src/change.ts:56` | C2 | CORE |
| state.cm6.153 | `ChangeDesc.iterChangedRanges` | method | `mirrors/cm6/state/src/change.ts:79` | C2 | CORE |
| state.cm6.154 | `ChangeDesc.invertedDesc` | getter | `mirrors/cm6/state/src/change.ts:84` | C2 | CORE |
| state.cm6.155 | `ChangeDesc.composeDesc` | method | `mirrors/cm6/state/src/change.ts:97` | C2 | CORE |
| state.cm6.156 | `ChangeDesc.mapDesc` | method | `mirrors/cm6/state/src/change.ts:103` | C2 | CORE |
| state.cm6.157 | `ChangeDesc.mapPos` | method | `mirrors/cm6/state/src/change.ts:120` | C2 | CORE |
| state.cm6.158 | `ChangeDesc.touchesRange` | method | `mirrors/cm6/state/src/change.ts:147` | C2 | CORE |
| state.cm6.159 | `ChangeDesc.toJSON` | method | `mirrors/cm6/state/src/change.ts:167` | C2 | CORE |
| state.cm6.160 | `ChangeDesc^fromJSON` | static method | `mirrors/cm6/state/src/change.ts:171` | C2 | CORE |
| state.cm6.161 | `MapMode` | enum | `mirrors/cm6/state/src/change.ts:6` | C2 | CORE |
| state.cm6.162 | `MapMode.Simple` | enum member | `mirrors/cm6/state/src/change.ts:9` | C2 | CORE |
| state.cm6.163 | `MapMode.TrackDel` | enum member | `mirrors/cm6/state/src/change.ts:11` | C2 | CORE |
| state.cm6.164 | `MapMode.TrackBefore` | enum member | `mirrors/cm6/state/src/change.ts:13` | C2 | CORE |
| state.cm6.165 | `MapMode.TrackAfter` | enum member | `mirrors/cm6/state/src/change.ts:15` | C2 | CORE |
| state.cm6.166 | `CharCategory` | enum | `mirrors/cm6/state/src/charcategory.ts:4` | C3 | CORE |
| state.cm6.167 | `CharCategory.Word` | enum member | `mirrors/cm6/state/src/charcategory.ts:6` | C3 | CORE |
| state.cm6.168 | `CharCategory.Space` | enum member | `mirrors/cm6/state/src/charcategory.ts:8` | C3 | CORE |
| state.cm6.169 | `CharCategory.Other` | enum member | `mirrors/cm6/state/src/charcategory.ts:10` | C3 | CORE |
| state.cm6.170 | `RangeValue` | class | `mirrors/cm6/state/src/rangeset.ts:5` | C6 | CORE |
| state.cm6.171 | `RangeValue.eq` | method | `mirrors/cm6/state/src/rangeset.ts:11` | C6 | CORE |
| state.cm6.172 | `RangeValue.startSide` | property | `mirrors/cm6/state/src/rangeset.ts:15` | C6 | CORE |
| state.cm6.173 | `RangeValue.endSide` | property | `mirrors/cm6/state/src/rangeset.ts:17` | C6 | CORE |
| state.cm6.174 | `RangeValue.mapMode` | property | `mirrors/cm6/state/src/rangeset.ts:22` | C6 | CORE |
| state.cm6.175 | `RangeValue.point` | property | `mirrors/cm6/state/src/rangeset.ts:28` | C6 | CORE |
| state.cm6.176 | `RangeValue.range` | method | `mirrors/cm6/state/src/rangeset.ts:31` | C6 | CORE |
| state.cm6.177 | `Range` | class | `mirrors/cm6/state/src/rangeset.ts:43` | C6 | CORE |
| state.cm6.178 | `Range.from` | property (ctor param) | `mirrors/cm6/state/src/rangeset.ts:46` | C6 | CORE |
| state.cm6.179 | `Range.to` | property (ctor param) | `mirrors/cm6/state/src/rangeset.ts:48` | C6 | CORE |
| state.cm6.180 | `Range.value` | property (ctor param) | `mirrors/cm6/state/src/rangeset.ts:50` | C6 | CORE |
| state.cm6.181 | `RangeSet` | class | `mirrors/cm6/state/src/rangeset.ts:226` | C6 | CORE |
| state.cm6.182 | `RangeSet.size` | getter | `mirrors/cm6/state/src/rangeset.ts:252` | C6 | CORE |
| state.cm6.183 | `RangeSet.update` | method | `mirrors/cm6/state/src/rangeset.ts:271` | C6 | CORE |
| state.cm6.184 | `RangeSet.map` | method | `mirrors/cm6/state/src/rangeset.ts:303` | C6 | CORE |
| state.cm6.185 | `RangeSet.between` | method | `mirrors/cm6/state/src/rangeset.ts:339` | C6 | CORE |
| state.cm6.186 | `RangeSet.iter` | method | `mirrors/cm6/state/src/rangeset.ts:351` | C6 | CORE |
| state.cm6.187 | `RangeSet^iter` | static method | `mirrors/cm6/state/src/rangeset.ts:360` | C6 | CORE |
| state.cm6.188 | `RangeSet^compare` | static method | `mirrors/cm6/state/src/rangeset.ts:366` | C6 | CORE |
| state.cm6.189 | `RangeSet^eq` | static method | `mirrors/cm6/state/src/rangeset.ts:389` | C6 | CORE |
| state.cm6.190 | `RangeSet^spans` | static method | `mirrors/cm6/state/src/rangeset.ts:415` | C6 | CORE |
| state.cm6.191 | `RangeSet^of` | static method | `mirrors/cm6/state/src/rangeset.ts:448` | C6 | CORE |
| state.cm6.192 | `RangeSet^join` | static method | `mirrors/cm6/state/src/rangeset.ts:456` | C6 | CORE |
| state.cm6.193 | `RangeSet^empty` | static property | `mirrors/cm6/state/src/rangeset.ts:467` | C6 | CORE |
| state.cm6.194 | `RangeCursor` | interface | `mirrors/cm6/state/src/rangeset.ts:185` | C6 | CORE |
| state.cm6.195 | `RangeCursor.next` | method | `mirrors/cm6/state/src/rangeset.ts:187` | C6 | CORE |
| state.cm6.196 | `RangeCursor.goto` | method | `mirrors/cm6/state/src/rangeset.ts:189` | C6 | CORE |
| state.cm6.197 | `RangeCursor.value` | property | `mirrors/cm6/state/src/rangeset.ts:192` | C6 | CORE |
| state.cm6.198 | `RangeCursor.from` | property | `mirrors/cm6/state/src/rangeset.ts:194` | C6 | CORE |
| state.cm6.199 | `RangeCursor.to` | property | `mirrors/cm6/state/src/rangeset.ts:196` | C6 | CORE |
| state.cm6.200 | `RangeCursor.rank` | property | `mirrors/cm6/state/src/rangeset.ts:199` | C6 | CORE |
| state.cm6.201 | `RangeSetBuilder` | class | `mirrors/cm6/state/src/rangeset.ts:487` | C6 | CORE |
| state.cm6.202 | `RangeSetBuilder.constructor` | constructor | `mirrors/cm6/state/src/rangeset.ts:511` | C6 | CORE |
| state.cm6.203 | `RangeSetBuilder.add` | method | `mirrors/cm6/state/src/rangeset.ts:515` | C6 | CORE |
| state.cm6.204 | `RangeSetBuilder.finish` | method | `mirrors/cm6/state/src/rangeset.ts:557` | C6 | CORE |
| state.cm6.205 | `RangeComparator` | interface | `mirrors/cm6/state/src/rangeset.ts:63` | C6 | CORE |
| state.cm6.206 | `RangeComparator.compareRange` | method | `mirrors/cm6/state/src/rangeset.ts:67` | C6 | CORE |
| state.cm6.207 | `RangeComparator.comparePoint` | method | `mirrors/cm6/state/src/rangeset.ts:69` | C6 | CORE |
| state.cm6.208 | `RangeComparator.boundChange` | method | `mirrors/cm6/state/src/rangeset.ts:74` | C6 | CORE |
| state.cm6.209 | `SpanIterator` | interface | `mirrors/cm6/state/src/rangeset.ts:80` | C6 | CORE |
| state.cm6.210 | `SpanIterator.span` | method | `mirrors/cm6/state/src/rangeset.ts:85` | C6 | CORE |
| state.cm6.211 | `SpanIterator.point` | method | `mirrors/cm6/state/src/rangeset.ts:92` | C6 | CORE |
| state.cm6.212 | `findClusterBreak` | function | `mirrors/cm6/state/src/char.ts:9` | C3 | CORE |
| state.cm6.213 | `codePointAt` | function | `mirrors/cm6/state/src/char.ts:19` | C3 | CORE |
| state.cm6.214 | `fromCodePoint` | function | `mirrors/cm6/state/src/char.ts:30` | C3 | CORE |
| state.cm6.215 | `codePointSize` | function | `mirrors/cm6/state/src/char.ts:37` | C3 | CORE |
| state.cm6.216 | `countColumn` | function | `mirrors/cm6/state/src/column.ts:5` | C3 | CORE |
| state.cm6.217 | `findColumn` | function | `mirrors/cm6/state/src/column.ts:24` | C3 | CORE |
| state.cm6.218 | `Line` | class | `mirrors/cm6/state/src/text.ts:562` | C1 | CORE |
| state.cm6.219 | `Line.from` | property (ctor param) | `mirrors/cm6/state/src/text.ts:566` | C1 | CORE |
| state.cm6.220 | `Line.to` | property (ctor param) | `mirrors/cm6/state/src/text.ts:569` | C1 | CORE |
| state.cm6.221 | `Line.number` | property (ctor param) | `mirrors/cm6/state/src/text.ts:571` | C1 | CORE |
| state.cm6.222 | `Line.text` | property (ctor param) | `mirrors/cm6/state/src/text.ts:573` | C1 | CORE |
| state.cm6.223 | `Line.length` | getter | `mirrors/cm6/state/src/text.ts:577` | C1 | CORE |
| state.cm6.224 | `TextIterator` | interface | `mirrors/cm6/state/src/text.ts:15` | C1 | CORE |
| state.cm6.225 | `TextIterator.next` | method | `mirrors/cm6/state/src/text.ts:19` | C1 | CORE |
| state.cm6.226 | `TextIterator.value` | property | `mirrors/cm6/state/src/text.ts:22` | C1 | CORE |
| state.cm6.227 | `TextIterator.done` | property | `mirrors/cm6/state/src/text.ts:25` | C1 | CORE |
| state.cm6.228 | `TextIterator.lineBreak` | property | `mirrors/cm6/state/src/text.ts:27` | C1 | CORE |
| state.cm6.229 | `Text` | class | `mirrors/cm6/state/src/text.ts:31` | C1 | CORE |
| state.cm6.230 | `Text.length` | property | `mirrors/cm6/state/src/text.ts:33` | C1 | CORE |
| state.cm6.231 | `Text.lines` | property | `mirrors/cm6/state/src/text.ts:35` | C1 | CORE |
| state.cm6.232 | `Text.lineAt` | method | `mirrors/cm6/state/src/text.ts:38` | C1 | CORE |
| state.cm6.233 | `Text.line` | method | `mirrors/cm6/state/src/text.ts:45` | C1 | CORE |
| state.cm6.234 | `Text.replace` | method | `mirrors/cm6/state/src/text.ts:54` | C1 | CORE |
| state.cm6.235 | `Text.append` | method | `mirrors/cm6/state/src/text.ts:64` | C1 | CORE |
| state.cm6.236 | `Text.slice` | method | `mirrors/cm6/state/src/text.ts:69` | C1 | CORE |
| state.cm6.237 | `Text.sliceString` | method | `mirrors/cm6/state/src/text.ts:77` | C1 | CORE |
| state.cm6.238 | `Text.eq` | method | `mirrors/cm6/state/src/text.ts:86` | C1 | CORE |
| state.cm6.239 | `Text.iter` | method | `mirrors/cm6/state/src/text.ts:104` | C1 | CORE |
| state.cm6.240 | `Text.iterRange` | method | `mirrors/cm6/state/src/text.ts:108` | C1 | CORE |
| state.cm6.241 | `Text.iterLines` | method | `mirrors/cm6/state/src/text.ts:115` | C1 | CORE |
| state.cm6.242 | `Text.toString` | method | `mirrors/cm6/state/src/text.ts:132` | C1 | CORE |
| state.cm6.243 | `Text.toJSON` | method | `mirrors/cm6/state/src/text.ts:136` | C1 | CORE |
| state.cm6.244 | `Text.children` | property | `mirrors/cm6/state/src/text.ts:147` | C1 | CORE |
| state.cm6.245 | `Text.[Symbol.iterator]` | property | `mirrors/cm6/state/src/text.ts:150` | C1 | CORE |
| state.cm6.246 | `Text^of` | static method | `mirrors/cm6/state/src/text.ts:153` | C1 | CORE |
| state.cm6.247 | `Text^empty` | static property | `mirrors/cm6/state/src/text.ts:160` | C1 | CORE |
| state.cm6.248 | `Facet.define config.combine` | property | `mirrors/cm6/state/src/facet.ts:12` | C4 | CORE |
| state.cm6.249 | `Facet.define config.compare` | property | `mirrors/cm6/state/src/facet.ts:17` | C4 | CORE |
| state.cm6.250 | `Facet.define config.compareInput` | property | `mirrors/cm6/state/src/facet.ts:20` | C4 | CORE |
| state.cm6.251 | `Facet.define config.static` | property | `mirrors/cm6/state/src/facet.ts:22` | C4 | CORE |
| state.cm6.252 | `Facet.define config.enables` | property | `mirrors/cm6/state/src/facet.ts:29` | C4 | CORE |
| state.cm6.253 | `StateField.define config.create` | property | `mirrors/cm6/state/src/facet.ts:262` | C4 | CORE |
| state.cm6.254 | `StateField.define config.update` | property | `mirrors/cm6/state/src/facet.ts:266` | C4 | CORE |
| state.cm6.255 | `StateField.define config.compare` | property | `mirrors/cm6/state/src/facet.ts:272` | C4 | CORE |
| state.cm6.256 | `StateField.define config.provide` | property | `mirrors/cm6/state/src/facet.ts:280` | C4 | CORE |
| state.cm6.257 | `StateField.define config.toJSON` | property | `mirrors/cm6/state/src/facet.ts:285` | C4 | CORE |
| state.cm6.258 | `StateField.define config.fromJSON` | property | `mirrors/cm6/state/src/facet.ts:289` | C4 | CORE |
| state.cm6.259 | `StateEffect.define spec.map` | property | `mirrors/cm6/state/src/transaction.ts:43` | C5 | CORE |
| state.cm6.260 | `RangeSet.update spec.add` | property | `mirrors/cm6/state/src/rangeset.ts:207` | C6 | CORE |
| state.cm6.261 | `RangeSet.update spec.sort` | property | `mirrors/cm6/state/src/rangeset.ts:210` | C6 | CORE |
| state.cm6.262 | `RangeSet.update spec.filter` | property | `mirrors/cm6/state/src/rangeset.ts:213` | C6 | CORE |
| state.cm6.263 | `RangeSet.update spec.filterFrom` | property | `mirrors/cm6/state/src/rangeset.ts:217` | C6 | CORE |
| state.cm6.264 | `RangeSet.update spec.filterTo` | property | `mirrors/cm6/state/src/rangeset.ts:219` | C6 | CORE |
