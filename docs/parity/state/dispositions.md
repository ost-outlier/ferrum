# ferrum-state dispositions (proposed)

Proposals for human approval (brief section 3). One row per cm6 export row of `raw/a1-cm6-exports.md` (264) and per cm6 source file (13).

Common ground for every `CORE` row: `@codemirror/state` has no DOM, CSS or browser dependency (its only dependency is `@marijn/find-cluster-break`, `state/package.json`), and nothing in `state/src` touches `document`, `window` or layout. Every item is therefore platform-independent logic; the reason column says what the item does for a native editor and flags decision dependencies.

JSON items (`toJSON`/`fromJSON`) are proposed `CORE` because the scope sheet asks for them and they are needed for persistence and for `collab`; decision D10 could move them to `DEFER`.

## Per export

| id | cm6 item | cm6 file:line | disposition | reason |
|---|---|---|---|---|
| state.cm6.001 | `EditorStateConfig` | `state/src/state.ts:14` | CORE | cm6: “Options passed when creating an editor state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.002 | `EditorStateConfig.doc` | `state/src/state.ts:21` | CORE | cm6: “The initial document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.003 | `EditorStateConfig.selection` | `state/src/state.ts:24` | CORE | cm6: “The starting selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.004 | `EditorStateConfig.extensions` | `state/src/state.ts:26` | CORE | cm6: “Extension(s) to associate with this state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.005 | `EditorState` | `state/src/state.ts:36` | CORE | cm6: “The editor state class is a persistent (immutable) data structure.” Platform-independent; needed by a native editor as-is. |
| state.cm6.006 | `EditorState.doc` | `state/src/state.ts:46` | CORE | cm6: “The current document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.007 | `EditorState.selection` | `state/src/state.ts:48` | CORE | cm6: “The current selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.008 | `EditorState.field` | `state/src/state.ts:66` | CORE | cm6: “Retrieve the value of a state field.” Platform-independent; needed by a native editor as-is. |
| state.cm6.009 | `EditorState.update` | `state/src/state.ts:91` | CORE | cm6: “Create a transaction that updates this state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.010 | `EditorState.replaceSelection` | `state/src/state.ts:129` | CORE | cm6: “Create a transaction spec that replaces every selection range with the given content.” Platform-independent; needed by a native editor as-is. |
| state.cm6.011 | `EditorState.changeByRange` | `state/src/state.ts:144` | CORE | cm6: “Create a set of changes and a new selection by running the given function for each range in the active selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.012 | `EditorState.changes` | `state/src/state.ts:176` | CORE | cm6: “Create a change set from the given change description, taking the state's document length and line separator into account.” Platform-independent; needed by a native editor as-is. |
| state.cm6.013 | `EditorState.toText` | `state/src/state.ts:184` | CORE | cm6: “Using the state's line separator, create a `Text` instance from the given string.” Platform-independent; needed by a native editor as-is. |
| state.cm6.014 | `EditorState.sliceDoc` | `state/src/state.ts:189` | CORE | cm6: “Return the given range of the document as a string.” Platform-independent; needed by a native editor as-is. |
| state.cm6.015 | `EditorState.facet` | `state/src/state.ts:194` | CORE | cm6: “Get the value of a state facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.016 | `EditorState.toJSON` | `state/src/state.ts:205` | CORE | cm6: “Convert this state to a JSON-serializable object.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.017 | `EditorState^fromJSON` | `state/src/state.ts:222` | CORE | cm6: “Deserialize a state from its JSON representation.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.018 | `EditorState^create` | `state/src/state.ts:243` | CORE | cm6: “Create a new state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.019 | `EditorState^allowMultipleSelections` | `state/src/state.ts:262` | CORE | cm6: “A facet that, when enabled, causes the editor to allow multiple ranges to be selected.” Platform-independent; needed by a native editor as-is. |
| state.cm6.020 | `EditorState^tabSize` | `state/src/state.ts:267` | CORE | cm6: “Configures the tab size to use in this state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.021 | `EditorState.tabSize` | `state/src/state.ts:273` | CORE | cm6: “The size (in columns) of a tab in the document, determined by the `tabSize` facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.022 | `EditorState^lineSeparator` | `state/src/state.ts:282` | CORE | cm6: “The line separator to use.” Platform-independent; needed by a native editor as-is. |
| state.cm6.023 | `EditorState.lineBreak` | `state/src/state.ts:286` | CORE | cm6: “Get the proper line-break string for this state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.024 | `EditorState^readOnly` | `state/src/state.ts:299` | CORE | cm6: “This facet controls the value of the `readOnly` getter, which is consulted by commands and extensions that implement editing functionality to determine whether they should apply.” Platform-independent; needed by a native editor as-is. |
| state.cm6.025 | `EditorState.readOnly` | `state/src/state.ts:303` | CORE | cm6: “Returns true when the editor is configured to be read-only.” Platform-independent; needed by a native editor as-is. |
| state.cm6.026 | `EditorState^phrases` | `state/src/state.ts:309` | CORE | cm6: “Registers translation phrases.” Platform-independent; needed by a native editor as-is. |
| state.cm6.027 | `EditorState.phrase` | `state/src/state.ts:324` | CORE | cm6: “Look up a translation for the given phrase (via the `phrases` facet), or return the original string if no translation is found.” Platform-independent; needed by a native editor as-is. |
| state.cm6.028 | `EditorState^languageData` | `state/src/state.ts:337` | CORE | cm6: “A facet used to register language data providers.” Platform-independent; needed by a native editor as-is. |
| state.cm6.029 | `EditorState.languageDataAt` | `state/src/state.ts:353` | CORE | cm6: “Find the values for a given language data field, provided by the the `languageData` facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.030 | `EditorState.charCategorizer` | `state/src/state.ts:373` | CORE | cm6: “Return a function that can categorize strings (expected to represent a single grapheme cluster) into one of: - Word (contains an alphanumeric character or a character explicitly listed in the local language's `"wordChars"` language data, which should be a string) - Space (contains only whitespace) - Other (anything else)” Platform-independent; needed by a native editor as-is. |
| state.cm6.031 | `EditorState.wordAt` | `state/src/state.ts:382` | CORE | cm6: “Find the word at the given position, meaning the range containing all word characters around it.” Platform-independent; needed by a native editor as-is. |
| state.cm6.032 | `EditorState^changeFilter` | `state/src/state.ts:411` | CORE | cm6: “Facet used to register change filters, which are called for each transaction (unless explicitly disabled), and can suppress part of the transaction's changes.” Platform-independent; needed by a native editor as-is. |
| state.cm6.033 | `EditorState^transactionFilter` | `state/src/state.ts:430` | CORE | cm6: “Facet used to register a hook that gets a chance to update or replace transaction specs before they are applied.” Platform-independent; needed by a native editor as-is. |
| state.cm6.034 | `EditorState^transactionExtender` | `state/src/state.ts:443` | CORE | cm6: “This is a more limited form of `transactionFilter`, which can only add annotations and effects.” Platform-independent; needed by a native editor as-is. |
| state.cm6.035 | `StateCommand` | `state/src/extension.ts:10` | CORE | Type of commands that act on state only; cm6 defines it in `state` (`extension.ts:10`) so `commands` and other packages can depend on it. Signature is decision D5. |
| state.cm6.036 | `Facet` | `state/src/facet.ts:43` | CORE | cm6: “A facet is a labeled value that is associated with an editor state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.037 | `Facet.reader` | `state/src/facet.ts:67` | CORE | cm6: “Returns a facet reader for this facet, which can be used to read it but not to define values for it.” Platform-independent; needed by a native editor as-is. |
| state.cm6.038 | `Facet^define` | `state/src/facet.ts:70` | CORE | cm6: “Define a new facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.039 | `Facet.of` | `state/src/facet.ts:79` | CORE | cm6: “Returns an extension that adds the given value to this facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.040 | `Facet.compute` | `state/src/facet.ts:90` | CORE | cm6: “Create an extension that computes a value for the facet from a state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.041 | `Facet.computeN` | `state/src/facet.ts:97` | CORE | cm6: “Create an extension that computes zero or more values for this facet from a state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.042 | `Facet.from` | `state/src/facet.ts:106` | CORE | cm6: “Shorthand method for registering a facet source with a state field as input.” Platform-independent; needed by a native editor as-is. |
| state.cm6.043 | `Facet.tag` | `state/src/facet.ts:113` | DROP | TypeScript phantom member (`facet.ts:113`): declared only so TypeScript infers `Output`; never assigned at runtime. Rust expresses the same with the generic parameter of `Facet<Input, Output>`. |
| state.cm6.044 | `FacetReader` | `state/src/facet.ts:120` | CORE | cm6: “A facet reader can be used to fetch the value of a facet, through `EditorState.facet` or as a dependency in `Facet.compute`, but not to define new values for the facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.045 | `FacetReader.tag` | `state/src/facet.ts:128` | DROP | TypeScript phantom member (`facet.ts:128`, doc: "Not actually present on the object"). No runtime value; Rust generics carry the output type. |
| state.cm6.046 | `StateField` | `state/src/facet.ts:296` | CORE | cm6: “Fields can store additional information in an editor state, and keep it in sync with the rest of the state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.047 | `StateField^define` | `state/src/facet.ts:311` | CORE | cm6: “Define a state field.” Platform-independent; needed by a native editor as-is. |
| state.cm6.048 | `StateField.init` | `state/src/facet.ts:356` | CORE | cm6: “Returns an extension that enables this field and overrides the way it is initialized.” Platform-independent; needed by a native editor as-is. |
| state.cm6.049 | `StateField.extension` | `state/src/facet.ts:363` | CORE | cm6: “State field instances can be used as `Extension` values to enable the field in a given state.” Platform-independent; needed by a native editor as-is. |
| state.cm6.050 | `Extension` | `state/src/facet.ts:374` | CORE | cm6: “Extension values can be provided when creating a state to attach various kinds of configuration and behavior information.” Platform-independent; needed by a native editor as-is. |
| state.cm6.051 | `Prec` | `state/src/facet.ts:390` | CORE | cm6: “By default extensions are registered in the order they are found in the flattened form of nested array that was provided.” Platform-independent; needed by a native editor as-is. |
| state.cm6.052 | `Prec.highest` | `state/src/facet.ts:393` | CORE | cm6: “The highest precedence level, for extensions that should end up near the start of the precedence ordering.” Platform-independent; needed by a native editor as-is. |
| state.cm6.053 | `Prec.high` | `state/src/facet.ts:396` | CORE | cm6: “A higher-than-default precedence, for extensions that should come before those with default precedence.” Platform-independent; needed by a native editor as-is. |
| state.cm6.054 | `Prec.default` | `state/src/facet.ts:399` | CORE | cm6: “The default precedence, which is also used for extensions without an explicit precedence.” Platform-independent; needed by a native editor as-is. |
| state.cm6.055 | `Prec.low` | `state/src/facet.ts:401` | CORE | cm6: “A lower-than-default precedence.” Platform-independent; needed by a native editor as-is. |
| state.cm6.056 | `Prec.lowest` | `state/src/facet.ts:404` | CORE | cm6: “The lowest precedence level.” Platform-independent; needed by a native editor as-is. |
| state.cm6.057 | `Compartment` | `state/src/facet.ts:418` | CORE | cm6: “Extension compartments can be used to make a configuration dynamic.” Platform-independent; needed by a native editor as-is. |
| state.cm6.058 | `Compartment.of` | `state/src/facet.ts:421` | CORE | cm6: “Create an instance of this compartment to add to your state configuration.” Platform-independent; needed by a native editor as-is. |
| state.cm6.059 | `Compartment.reconfigure` | `state/src/facet.ts:425` | CORE | cm6: “Create an effect that reconfigures this compartment.” Platform-independent; needed by a native editor as-is. |
| state.cm6.060 | `Compartment.get` | `state/src/facet.ts:431` | CORE | cm6: “Get the current content of the compartment in the state, or `undefined` if it isn't present.” Platform-independent; needed by a native editor as-is. |
| state.cm6.061 | `EditorSelection` | `state/src/selection.ts:115` | CORE | cm6: “An editor selection holds one or more selection ranges.” Platform-independent; needed by a native editor as-is. |
| state.cm6.062 | `EditorSelection.ranges` | `state/src/selection.ts:119` | CORE | cm6: “The ranges in the selection, sorted by position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.063 | `EditorSelection.mainIndex` | `state/src/selection.ts:122` | CORE | cm6: “The index of the _main_ range in the selection (which is usually the range that was added last).” Platform-independent; needed by a native editor as-is. |
| state.cm6.064 | `EditorSelection.map` | `state/src/selection.ts:127` | CORE | cm6: “Map a selection through a change.” Platform-independent; needed by a native editor as-is. |
| state.cm6.065 | `EditorSelection.eq` | `state/src/selection.ts:136` | CORE | cm6: “Compare this selection to another selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.066 | `EditorSelection.main` | `state/src/selection.ts:147` | CORE | cm6: “Get the primary selection range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.067 | `EditorSelection.asSingle` | `state/src/selection.ts:151` | CORE | cm6: “Make sure the selection only has one range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.068 | `EditorSelection.addRange` | `state/src/selection.ts:156` | CORE | cm6: “Extend this selection with an extra range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.069 | `EditorSelection.replaceRange` | `state/src/selection.ts:162` | CORE | cm6: “Replace a given range with another range, and then normalize the selection to merge and sort ranges if necessary.” Platform-independent; needed by a native editor as-is. |
| state.cm6.070 | `EditorSelection.toJSON` | `state/src/selection.ts:170` | CORE | cm6: “Convert this selection to an object that can be serialized to JSON.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.071 | `EditorSelection^fromJSON` | `state/src/selection.ts:175` | CORE | cm6: “Create a selection from a JSON representation.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.072 | `EditorSelection^single` | `state/src/selection.ts:182` | CORE | cm6: “Create a selection holding a single range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.073 | `EditorSelection^create` | `state/src/selection.ts:188` | CORE | cm6: “Sort and merge the given set of ranges, creating a valid selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.074 | `EditorSelection^cursor` | `state/src/selection.ts:200` | CORE | cm6: “Create a cursor selection range at the given position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.075 | `EditorSelection^range` | `state/src/selection.ts:208` | CORE | cm6: “Create a selection range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.076 | `EditorSelection^undirectionalRange` | `state/src/selection.ts:218` | CORE | cm6: “Create an undirectional selection range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.077 | `SelectionRange` | `state/src/selection.ts:24` | CORE | cm6: “A single selection range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.078 | `SelectionRange.from` | `state/src/selection.ts:27` | CORE | cm6: “The lower boundary of the range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.079 | `SelectionRange.to` | `state/src/selection.ts:29` | CORE | cm6: “The upper boundary of the range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.080 | `SelectionRange.goalColumn` | `state/src/selection.ts:35` | CORE | cm6: “The goal column (stored vertical offset) associated with a cursor.” Platform-independent; needed by a native editor as-is. |
| state.cm6.081 | `SelectionRange.anchor` | `state/src/selection.ts:40` | CORE | cm6: “The anchor of the range—the side that doesn't move when you extend it.” Platform-independent; needed by a native editor as-is. |
| state.cm6.082 | `SelectionRange.head` | `state/src/selection.ts:44` | CORE | cm6: “The head of the range, which is moved when the range is extended.” Platform-independent; needed by a native editor as-is. |
| state.cm6.083 | `SelectionRange.empty` | `state/src/selection.ts:47` | CORE | cm6: “True when `anchor` and `head` are at the same position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.084 | `SelectionRange.assoc` | `state/src/selection.ts:53` | CORE | cm6: “If this is a cursor that is explicitly associated with the character on one of its sides, this returns the side.” Platform-independent; needed by a native editor as-is. |
| state.cm6.085 | `SelectionRange.undirectional` | `state/src/selection.ts:60` | CORE | cm6: “A flag that, when set, makes some selection-extending commands treat the range's head and anchor as exchangeable, so that for example Shift-ArrowUp will make the lower side of the selection the anchor, even if that was the head before.” Platform-independent; needed by a native editor as-is. |
| state.cm6.086 | `SelectionRange.bidiLevel` | `state/src/selection.ts:66` | CORE | cm6: “The bidirectional text level associated with this cursor, if any.” Platform-independent; needed by a native editor as-is. |
| state.cm6.087 | `SelectionRange.map` | `state/src/selection.ts:73` | CORE | cm6: “Map this range through a change, producing a valid range in the updated document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.088 | `SelectionRange.extend` | `state/src/selection.ts:85` | CORE | cm6: “Extend this range to cover at least `from` to `to`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.089 | `SelectionRange.eq` | `state/src/selection.ts:92` | CORE | cm6: “Compare this range to another range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.090 | `SelectionRange.toJSON` | `state/src/selection.ts:98` | CORE | cm6: “Return a JSON-serializable object representing the range.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.091 | `SelectionRange^fromJSON` | `state/src/selection.ts:102` | CORE | cm6: “Convert a JSON representation of a range to a `SelectionRange` instance.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.092 | `Transaction` | `state/src/transaction.ts:163` | CORE | cm6: “Changes to the editor state are grouped into transactions.” Platform-independent; needed by a native editor as-is. |
| state.cm6.093 | `Transaction.startState` | `state/src/transaction.ts:171` | CORE | cm6: “The state from which the transaction starts.” Platform-independent; needed by a native editor as-is. |
| state.cm6.094 | `Transaction.changes` | `state/src/transaction.ts:173` | CORE | cm6: “The document changes made by this transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.095 | `Transaction.selection` | `state/src/transaction.ts:176` | CORE | cm6: “The selection set by this transaction, or undefined if it doesn't explicitly set a selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.096 | `Transaction.effects` | `state/src/transaction.ts:178` | CORE | cm6: “The effects added to the transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.097 | `Transaction.scrollIntoView` | `state/src/transaction.ts:183` | CORE | cm6: “Whether the selection should be scrolled into view after this transaction is dispatched.” Platform-independent; needed by a native editor as-is. |
| state.cm6.098 | `Transaction.newDoc` | `state/src/transaction.ts:203` | CORE | cm6: “The new document produced by the transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.099 | `Transaction.newSelection` | `state/src/transaction.ts:211` | CORE | cm6: “The new selection produced by the transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.100 | `Transaction.state` | `state/src/transaction.ts:219` | CORE | cm6: “The new state created by the transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.101 | `Transaction.annotation` | `state/src/transaction.ts:225` | CORE | cm6: “Get the value of the given annotation type, if any.” Platform-independent; needed by a native editor as-is. |
| state.cm6.102 | `Transaction.docChanged` | `state/src/transaction.ts:231` | CORE | cm6: “Indicates whether the transaction changed the document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.103 | `Transaction.reconfigured` | `state/src/transaction.ts:237` | CORE | cm6: “Indicates whether this transaction reconfigures the state (through a configuration compartment or with a top-level configuration effect.” Platform-independent; needed by a native editor as-is. |
| state.cm6.104 | `Transaction.isUserEvent` | `state/src/transaction.ts:244` | CORE | cm6: “Returns true if the transaction has a user event annotation that is equal to or more specific than `event`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.105 | `Transaction^time` | `state/src/transaction.ts:251` | CORE | cm6: “Annotation used to store transaction timestamps.” Platform-independent; needed by a native editor as-is. |
| state.cm6.106 | `Transaction^userEvent` | `state/src/transaction.ts:277` | CORE | cm6: “Annotation used to associate a transaction with a user interface event.” Platform-independent; needed by a native editor as-is. |
| state.cm6.107 | `Transaction^addToHistory` | `state/src/transaction.ts:281` | CORE | cm6: “Annotation indicating whether a transaction should be added to the undo history or not.” Platform-independent; needed by a native editor as-is. |
| state.cm6.108 | `Transaction^remote` | `state/src/transaction.ts:287` | CORE | cm6: “Annotation indicating (when present and true) that a transaction represents a change made by some other actor, not the user.” Platform-independent; needed by a native editor as-is. |
| state.cm6.109 | `TransactionSpec` | `state/src/transaction.ts:121` | CORE | cm6: “Describes a transaction when calling the `EditorState.update` method.” Platform-independent; needed by a native editor as-is. |
| state.cm6.110 | `TransactionSpec.changes` | `state/src/transaction.ts:123` | CORE | cm6: “The changes to the document made by this transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.111 | `TransactionSpec.selection` | `state/src/transaction.ts:127` | CORE | cm6: “When set, this transaction explicitly updates the selection.” Platform-independent; needed by a native editor as-is. |
| state.cm6.112 | `TransactionSpec.effects` | `state/src/transaction.ts:132` | CORE | cm6: “Attach state effects to this transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.113 | `TransactionSpec.annotations` | `state/src/transaction.ts:134` | CORE | cm6: “Set annotations for this transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.114 | `TransactionSpec.userEvent` | `state/src/transaction.ts:136` | CORE | cm6: “Shorthand for `annotations:` `Transaction.userEvent``.of(...)`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.115 | `TransactionSpec.scrollIntoView` | `state/src/transaction.ts:139` | CORE | cm6: “When set to `true`, the transaction is marked as needing to scroll the current selection into view.” Platform-independent; needed by a native editor as-is. |
| state.cm6.116 | `TransactionSpec.filter` | `state/src/transaction.ts:146` | CORE | cm6: “By default, transactions can be modified by change filters and transaction filters.” Platform-independent; needed by a native editor as-is. |
| state.cm6.117 | `TransactionSpec.sequential` | `state/src/transaction.ts:153` | CORE | cm6: “Normally, when multiple specs are combined (for example by `EditorState.update`), the positions in `changes` are taken to refer to the document positions in the initial document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.118 | `Annotation` | `state/src/transaction.ts:15` | CORE | cm6: “Annotations are tagged values that are used to add metadata to transactions in an extensible way.” Platform-independent; needed by a native editor as-is. |
| state.cm6.119 | `Annotation.type` | `state/src/transaction.ts:19` | CORE | cm6: “The annotation type.” Platform-independent; needed by a native editor as-is. |
| state.cm6.120 | `Annotation.value` | `state/src/transaction.ts:21` | CORE | cm6: “The value of this annotation.” Platform-independent; needed by a native editor as-is. |
| state.cm6.121 | `Annotation^define` | `state/src/transaction.ts:25` | CORE | cm6: “Define a new type of annotation.” Platform-independent; needed by a native editor as-is. |
| state.cm6.122 | `AnnotationType` | `state/src/transaction.ts:33` | CORE | cm6: “Marker that identifies a type of annotation.” Platform-independent; needed by a native editor as-is. |
| state.cm6.123 | `AnnotationType.of` | `state/src/transaction.ts:35` | CORE | cm6: “Create an instance of this annotation.” Platform-independent; needed by a native editor as-is. |
| state.cm6.124 | `StateEffect` | `state/src/transaction.ts:69` | CORE | cm6: “State effects can be used to represent additional effects associated with a transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.125 | `StateEffect.value` | `state/src/transaction.ts:75` | CORE | cm6: “The value of this effect.” Platform-independent; needed by a native editor as-is. |
| state.cm6.126 | `StateEffect.map` | `state/src/transaction.ts:79` | CORE | cm6: “Map this effect through a position mapping.” Platform-independent; needed by a native editor as-is. |
| state.cm6.127 | `StateEffect.is` | `state/src/transaction.ts:86` | CORE | cm6: “Tells you whether this effect object is of a given type.” Platform-independent; needed by a native editor as-is. |
| state.cm6.128 | `StateEffect^define` | `state/src/transaction.ts:93` | CORE | cm6: “Define a new effect type.” Platform-independent; needed by a native editor as-is. |
| state.cm6.129 | `StateEffect^mapEffects` | `state/src/transaction.ts:98` | CORE | cm6: “Map an array of effects through a change set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.130 | `StateEffect^reconfigure` | `state/src/transaction.ts:113` | CORE | cm6: “This effect can be used to reconfigure the root extensions of the editor.” Platform-independent; needed by a native editor as-is. |
| state.cm6.131 | `StateEffect^appendConfig` | `state/src/transaction.ts:116` | CORE | cm6: “Append extensions to the top-level configuration of the editor.” Platform-independent; needed by a native editor as-is. |
| state.cm6.132 | `StateEffectType` | `state/src/transaction.ts:48` | CORE | cm6: “Representation of a type of state effect.” Platform-independent; needed by a native editor as-is. |
| state.cm6.133 | `StateEffectType.of` | `state/src/transaction.ts:61` | CORE | cm6: “Create a state effect instance of this type.” Platform-independent; needed by a native editor as-is. |
| state.cm6.134 | `combineConfig` | `state/src/config.ts:8` | CORE | Utility used by many cm6 packages (view, commands, search, autocomplete) to merge configuration objects from facet inputs; platform-independent. Rust may express it differently (struct merge); behavior (conflict error "Config merge conflict for field") is observable. |
| state.cm6.135 | `ChangeSpec` | `state/src/change.ts:188` | CORE | cm6: “This type is used as argument to `EditorState.changes` and in the `changes` field of transaction specs to succinctly describe document changes.” Platform-independent; needed by a native editor as-is. |
| state.cm6.136 | `ChangeSet` | `state/src/change.ts:196` | CORE | cm6: “A change set represents a group of modifications to a document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.137 | `ChangeSet.apply` | `state/src/change.ts:207` | CORE | cm6: “Apply the changes to a document, returning the modified document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.138 | `ChangeSet.mapDesc` | `state/src/change.ts:213` | CORE | cm6: “Override of `ChangeDesc.mapDesc` (same semantics; ChangeSet instances map their description).” Platform-independent; needed by a native editor as-is. |
| state.cm6.139 | `ChangeSet.invert` | `state/src/change.ts:219` | CORE | cm6: “Given the document as it existed _before_ the changes, return a change set that represents the inverse of this set, which could be used to go from the document created by the changes back to the document as it existed before the changes.” Platform-independent; needed by a native editor as-is. |
| state.cm6.140 | `ChangeSet.compose` | `state/src/change.ts:238` | CORE | cm6: “Combine two subsequent change sets into a single set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.141 | `ChangeSet.map` | `state/src/change.ts:251` | CORE | cm6: “Given another change set starting in the same document, maps this change set over the other, producing a new change set that can be applied to the document produced by applying `other`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.142 | `ChangeSet.iterChanges` | `state/src/change.ts:260` | CORE | cm6: “Iterate over the changed ranges in the document, calling `f` for each, with the range in the original document (`fromA`-`toA`) and the range that replaces it in the new document (`fromB`-`toB`).” Platform-independent; needed by a native editor as-is. |
| state.cm6.143 | `ChangeSet.desc` | `state/src/change.ts:266` | CORE | cm6: “Get a change description for this change set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.144 | `ChangeSet.toJSON` | `state/src/change.ts:299` | CORE | cm6: “Serialize this change set to a JSON-representable value.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.145 | `ChangeSet^of` | `state/src/change.ts:312` | CORE | cm6: “Create a change set for the given changes, for a document of the given length, using `lineSep` as line separator.” Platform-independent; needed by a native editor as-is. |
| state.cm6.146 | `ChangeSet^empty` | `state/src/change.ts:352` | CORE | cm6: “Create an empty changeset of the given length.” Platform-independent; needed by a native editor as-is. |
| state.cm6.147 | `ChangeSet^fromJSON` | `state/src/change.ts:358` | CORE | cm6: “Create a changeset from its JSON representation (as produced by `toJSON`.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.148 | `ChangeDesc` | `state/src/change.ts:21` | CORE | cm6: “A change description is a variant of change set that doesn't store the inserted text.” Platform-independent; needed by a native editor as-is. |
| state.cm6.149 | `ChangeDesc.length` | `state/src/change.ts:34` | CORE | cm6: “The length of the document before the change.” Platform-independent; needed by a native editor as-is. |
| state.cm6.150 | `ChangeDesc.newLength` | `state/src/change.ts:41` | CORE | cm6: “The length of the document after the change.” Platform-independent; needed by a native editor as-is. |
| state.cm6.151 | `ChangeDesc.empty` | `state/src/change.ts:51` | CORE | cm6: “False when there are actual changes in this set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.152 | `ChangeDesc.iterGaps` | `state/src/change.ts:56` | CORE | cm6: “Iterate over the unchanged parts left by these changes.” Platform-independent; needed by a native editor as-is. |
| state.cm6.153 | `ChangeDesc.iterChangedRanges` | `state/src/change.ts:79` | CORE | cm6: “Iterate over the ranges changed by these changes.” Platform-independent; needed by a native editor as-is. |
| state.cm6.154 | `ChangeDesc.invertedDesc` | `state/src/change.ts:84` | CORE | cm6: “Get a description of the inverted form of these changes.” Platform-independent; needed by a native editor as-is. |
| state.cm6.155 | `ChangeDesc.composeDesc` | `state/src/change.ts:97` | CORE | cm6: “Compute the combined effect of applying another set of changes after this one.” Platform-independent; needed by a native editor as-is. |
| state.cm6.156 | `ChangeDesc.mapDesc` | `state/src/change.ts:103` | CORE | cm6: “Map this description, which should start with the same document as `other`, over another set of changes, so that it can be applied after it.” Platform-independent; needed by a native editor as-is. |
| state.cm6.157 | `ChangeDesc.mapPos` | `state/src/change.ts:120` | CORE | cm6: “`mode` determines whether deletions should be reported.” Platform-independent; needed by a native editor as-is. |
| state.cm6.158 | `ChangeDesc.touchesRange` | `state/src/change.ts:147` | CORE | cm6: “Check whether these changes touch a given range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.159 | `ChangeDesc.toJSON` | `state/src/change.ts:167` | CORE | cm6: “Serialize this change desc to a JSON-representable value.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.160 | `ChangeDesc^fromJSON` | `state/src/change.ts:171` | CORE | cm6: “Create a change desc from its JSON representation (as produced by `toJSON`.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.161 | `MapMode` | `state/src/change.ts:6` | CORE | cm6: “Distinguishes different ways in which positions can be mapped.” Platform-independent; needed by a native editor as-is. |
| state.cm6.162 | `MapMode.Simple` | `state/src/change.ts:9` | CORE | cm6: “Map a position to a valid new position, even when its context was deleted.” Platform-independent; needed by a native editor as-is. |
| state.cm6.163 | `MapMode.TrackDel` | `state/src/change.ts:11` | CORE | cm6: “Return null if deletion happens across the position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.164 | `MapMode.TrackBefore` | `state/src/change.ts:13` | CORE | cm6: “Return null if the character _before_ the position is deleted.” Platform-independent; needed by a native editor as-is. |
| state.cm6.165 | `MapMode.TrackAfter` | `state/src/change.ts:15` | CORE | cm6: “Return null if the character _after_ the position is deleted.” Platform-independent; needed by a native editor as-is. |
| state.cm6.166 | `CharCategory` | `state/src/charcategory.ts:4` | CORE | cm6: “The categories produced by a character categorizer.” Platform-independent; needed by a native editor as-is. |
| state.cm6.167 | `CharCategory.Word` | `state/src/charcategory.ts:6` | CORE | cm6: “Word characters.” Platform-independent; needed by a native editor as-is. |
| state.cm6.168 | `CharCategory.Space` | `state/src/charcategory.ts:8` | CORE | cm6: “Whitespace.” Platform-independent; needed by a native editor as-is. |
| state.cm6.169 | `CharCategory.Other` | `state/src/charcategory.ts:10` | CORE | cm6: “Anything else.” Platform-independent; needed by a native editor as-is. |
| state.cm6.170 | `RangeValue` | `state/src/rangeset.ts:5` | CORE | cm6: “Each range is associated with a value, which must inherit from this class.” Platform-independent; needed by a native editor as-is. |
| state.cm6.171 | `RangeValue.eq` | `state/src/rangeset.ts:11` | CORE | cm6: “Compare this value with another value.” Platform-independent; needed by a native editor as-is. |
| state.cm6.172 | `RangeValue.startSide` | `state/src/rangeset.ts:15` | CORE | cm6: “The bias value at the start of the range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.173 | `RangeValue.endSide` | `state/src/rangeset.ts:17` | CORE | cm6: “The bias value at the end of the range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.174 | `RangeValue.mapMode` | `state/src/rangeset.ts:22` | CORE | cm6: “The mode with which the location of the range should be mapped when its `from` and `to` are the same, to decide whether a change deletes the range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.175 | `RangeValue.point` | `state/src/rangeset.ts:28` | CORE | cm6: “Determines whether this value marks a point range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.176 | `RangeValue.range` | `state/src/rangeset.ts:31` | CORE | cm6: “Create a range with this value.” Platform-independent; needed by a native editor as-is. |
| state.cm6.177 | `Range` | `state/src/rangeset.ts:43` | CORE | cm6: “A range associates a value with a range of positions.” Platform-independent; needed by a native editor as-is. |
| state.cm6.178 | `Range.from` | `state/src/rangeset.ts:46` | CORE | cm6: “The range's start position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.179 | `Range.to` | `state/src/rangeset.ts:48` | CORE | cm6: “Its end position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.180 | `Range.value` | `state/src/rangeset.ts:50` | CORE | cm6: “The value associated with this range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.181 | `RangeSet` | `state/src/rangeset.ts:226` | CORE | cm6: “A range set stores a collection of ranges in a way that makes them efficient to map and update.” Platform-independent; needed by a native editor as-is. |
| state.cm6.182 | `RangeSet.size` | `state/src/rangeset.ts:252` | CORE | cm6: “The number of ranges in the set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.183 | `RangeSet.update` | `state/src/rangeset.ts:271` | CORE | cm6: “Update the range set, optionally adding new ranges or filtering out existing ones.” Platform-independent; needed by a native editor as-is. |
| state.cm6.184 | `RangeSet.map` | `state/src/rangeset.ts:303` | CORE | cm6: “Map this range set through a set of changes, return the new set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.185 | `RangeSet.between` | `state/src/rangeset.ts:339` | CORE | cm6: “Iterate over the ranges that touch the region `from` to `to`, calling `f` for each.” Platform-independent; needed by a native editor as-is. |
| state.cm6.186 | `RangeSet.iter` | `state/src/rangeset.ts:351` | CORE | cm6: “Iterate over the ranges in this set, in order, including all ranges that end at or after `from`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.187 | `RangeSet^iter` | `state/src/rangeset.ts:360` | CORE | cm6: “Iterate over the ranges in a collection of sets, in order, starting from `from`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.188 | `RangeSet^compare` | `state/src/rangeset.ts:366` | CORE | cm6: “Iterate over two groups of sets, calling methods on `comparator` to notify it of possible differences.” Platform-independent; needed by a native editor as-is. |
| state.cm6.189 | `RangeSet^eq` | `state/src/rangeset.ts:389` | CORE | cm6: “Compare the contents of two groups of range sets, returning true if they are equivalent in the given range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.190 | `RangeSet^spans` | `state/src/rangeset.ts:415` | CORE | cm6: “Iterate over a group of range sets at the same time, notifying the iterator about the ranges covering every given piece of content.” Platform-independent; needed by a native editor as-is. |
| state.cm6.191 | `RangeSet^of` | `state/src/rangeset.ts:448` | CORE | cm6: “Create a range set for the given range or array of ranges.” Platform-independent; needed by a native editor as-is. |
| state.cm6.192 | `RangeSet^join` | `state/src/rangeset.ts:456` | CORE | cm6: “Join an array of range sets into a single set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.193 | `RangeSet^empty` | `state/src/rangeset.ts:467` | CORE | cm6: “The empty set of ranges.” Platform-independent; needed by a native editor as-is. |
| state.cm6.194 | `RangeCursor` | `state/src/rangeset.ts:185` | CORE | cm6: “A range cursor is an object that moves to the next range every time you call `next` on it.” Platform-independent; needed by a native editor as-is. |
| state.cm6.195 | `RangeCursor.next` | `state/src/rangeset.ts:187` | CORE | cm6: “Move the iterator forward.” Platform-independent; needed by a native editor as-is. |
| state.cm6.196 | `RangeCursor.goto` | `state/src/rangeset.ts:189` | CORE | cm6: “Jump the cursor to the given position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.197 | `RangeCursor.value` | `state/src/rangeset.ts:192` | CORE | cm6: “The next range's value.” Platform-independent; needed by a native editor as-is. |
| state.cm6.198 | `RangeCursor.from` | `state/src/rangeset.ts:194` | CORE | cm6: “The next range's start position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.199 | `RangeCursor.to` | `state/src/rangeset.ts:196` | CORE | cm6: “The next end position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.200 | `RangeCursor.rank` | `state/src/rangeset.ts:199` | CORE | cm6: “The position of the set that this range comes from in the array of sets being iterated over.” Platform-independent; needed by a native editor as-is. |
| state.cm6.201 | `RangeSetBuilder` | `state/src/rangeset.ts:487` | CORE | cm6: “A range set builder is a data structure that helps build up a range set directly, without first allocating an array of `Range` objects.” Platform-independent; needed by a native editor as-is. |
| state.cm6.202 | `RangeSetBuilder.constructor` | `state/src/rangeset.ts:511` | CORE | cm6: “Create an empty builder.” Platform-independent; needed by a native editor as-is. |
| state.cm6.203 | `RangeSetBuilder.add` | `state/src/rangeset.ts:515` | CORE | cm6: “Add a range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.204 | `RangeSetBuilder.finish` | `state/src/rangeset.ts:557` | CORE | cm6: “Finish the range set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.205 | `RangeComparator` | `state/src/rangeset.ts:63` | CORE | cm6: “Collection of methods used when comparing range sets.” Platform-independent; needed by a native editor as-is. |
| state.cm6.206 | `RangeComparator.compareRange` | `state/src/rangeset.ts:67` | CORE | cm6: “Notifies the comparator that a range (in positions in the new document) has the given sets of values associated with it, which are different in the old (A) and new (B) sets.” Platform-independent; needed by a native editor as-is. |
| state.cm6.207 | `RangeComparator.comparePoint` | `state/src/rangeset.ts:69` | CORE | cm6: “Notification for a changed (or inserted, or deleted) point range.” Platform-independent; needed by a native editor as-is. |
| state.cm6.208 | `RangeComparator.boundChange` | `state/src/rangeset.ts:74` | CORE | cm6: “Notification for a changed boundary between ranges.” Platform-independent; needed by a native editor as-is. |
| state.cm6.209 | `SpanIterator` | `state/src/rangeset.ts:80` | CORE | cm6: “Methods used when iterating over the spans created by a set of ranges.” Platform-independent; needed by a native editor as-is. |
| state.cm6.210 | `SpanIterator.span` | `state/src/rangeset.ts:85` | CORE | cm6: “Called for any ranges not covered by point decorations.” Platform-independent; needed by a native editor as-is. |
| state.cm6.211 | `SpanIterator.point` | `state/src/rangeset.ts:92` | CORE | cm6: “Called when going over a point decoration.” Platform-independent; needed by a native editor as-is. |
| state.cm6.212 | `findClusterBreak` | `state/src/char.ts:9` | CORE | cm6: “Returns a next grapheme cluster break _after_ (not equal to) `pos`, if `forward` is true, or before otherwise.” Platform-independent; needed by a native editor as-is. |
| state.cm6.213 | `codePointAt` | `state/src/char.ts:19` | CORE | Exists in cm6 as a portability shim for JS runtimes without `String.prototype.codePointAt` (README "Code Points and Characters"), and to read a code point at a UTF-16 offset. Native need depends on D1: with UTF-16 positions Ferrum needs it; with code-point or UTF-8 positions `str::chars`/`char` cover it. Kept CORE pending D1 (alternative: DROP if D1 ≠ UTF-16). |
| state.cm6.214 | `fromCodePoint` | `state/src/char.ts:30` | CORE | Portability shim for `String.fromCodePoint` (README). Rust `char::from_u32` is native. Kept CORE pending D1 (alternative: DROP; it has no position semantics). |
| state.cm6.215 | `codePointSize` | `state/src/char.ts:37` | CORE | Returns 1 or 2 = size of a code point in UTF-16 code units; used by cm6 view/commands to step over surrogate pairs. Its meaning is the position unit itself: D1 decides whether it is `char::len_utf16`, `char::len_utf8`, or always 1. |
| state.cm6.216 | `countColumn` | `state/src/column.ts:5` | CORE | cm6: “Count the column position at the given offset into the string, taking extending characters and tab size into account.” Platform-independent; needed by a native editor as-is. |
| state.cm6.217 | `findColumn` | `state/src/column.ts:24` | CORE | cm6: “Find the offset that corresponds to the given column position in a string, taking extending characters and tab size into account.” Platform-independent; needed by a native editor as-is. |
| state.cm6.218 | `Line` | `state/src/text.ts:562` | CORE | cm6: “This type describes a line in the document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.219 | `Line.from` | `state/src/text.ts:566` | CORE | cm6: “The position of the start of the line.” Platform-independent; needed by a native editor as-is. |
| state.cm6.220 | `Line.to` | `state/src/text.ts:569` | CORE | cm6: “The position at the end of the line (_before_ the line break, or at the end of document for the last line).” Platform-independent; needed by a native editor as-is. |
| state.cm6.221 | `Line.number` | `state/src/text.ts:571` | CORE | cm6: “This line's line number (1-based).” Platform-independent; needed by a native editor as-is. |
| state.cm6.222 | `Line.text` | `state/src/text.ts:573` | CORE | cm6: “The line's content.” Platform-independent; needed by a native editor as-is. |
| state.cm6.223 | `Line.length` | `state/src/text.ts:577` | CORE | cm6: “The length of the line (not including any line break after it).” Platform-independent; needed by a native editor as-is. |
| state.cm6.224 | `TextIterator` | `state/src/text.ts:15` | CORE | cm6: “A text iterator iterates over a sequence of strings.” Platform-independent; needed by a native editor as-is. |
| state.cm6.225 | `TextIterator.next` | `state/src/text.ts:19` | CORE | cm6: “Retrieve the next string.” Platform-independent; needed by a native editor as-is. |
| state.cm6.226 | `TextIterator.value` | `state/src/text.ts:22` | CORE | cm6: “The current string.” Platform-independent; needed by a native editor as-is. |
| state.cm6.227 | `TextIterator.done` | `state/src/text.ts:25` | CORE | cm6: “Whether the end of the iteration has been reached.” Platform-independent; needed by a native editor as-is. |
| state.cm6.228 | `TextIterator.lineBreak` | `state/src/text.ts:27` | CORE | cm6: “Whether the current string represents a line break.” Platform-independent; needed by a native editor as-is. |
| state.cm6.229 | `Text` | `state/src/text.ts:31` | CORE | cm6: “The data structure for documents.” Platform-independent; needed by a native editor as-is. |
| state.cm6.230 | `Text.length` | `state/src/text.ts:33` | CORE | cm6: “The length of the string.” Platform-independent; needed by a native editor as-is. |
| state.cm6.231 | `Text.lines` | `state/src/text.ts:35` | CORE | cm6: “The number of lines in the string (always >= 1).” Platform-independent; needed by a native editor as-is. |
| state.cm6.232 | `Text.lineAt` | `state/src/text.ts:38` | CORE | cm6: “Get the line description around the given position.” Platform-independent; needed by a native editor as-is. |
| state.cm6.233 | `Text.line` | `state/src/text.ts:45` | CORE | cm6: “Get the description for the given (1-based) line number.” Platform-independent; needed by a native editor as-is. |
| state.cm6.234 | `Text.replace` | `state/src/text.ts:54` | CORE | cm6: “Replace a range of the text with the given content.” Platform-independent; needed by a native editor as-is. |
| state.cm6.235 | `Text.append` | `state/src/text.ts:64` | CORE | cm6: “Append another document to this one.” Platform-independent; needed by a native editor as-is. |
| state.cm6.236 | `Text.slice` | `state/src/text.ts:69` | CORE | cm6: “Retrieve the text between the given points.” Platform-independent; needed by a native editor as-is. |
| state.cm6.237 | `Text.sliceString` | `state/src/text.ts:77` | CORE | cm6: “Retrieve a part of the document as a string” Platform-independent; needed by a native editor as-is. |
| state.cm6.238 | `Text.eq` | `state/src/text.ts:86` | CORE | cm6: “Test whether this text is equal to another instance.” Platform-independent; needed by a native editor as-is. |
| state.cm6.239 | `Text.iter` | `state/src/text.ts:104` | CORE | cm6: “Iterate over the text.” Platform-independent; needed by a native editor as-is. |
| state.cm6.240 | `Text.iterRange` | `state/src/text.ts:108` | CORE | cm6: “Iterate over a range of the text.” Platform-independent; needed by a native editor as-is. |
| state.cm6.241 | `Text.iterLines` | `state/src/text.ts:115` | CORE | cm6: “Return a cursor that iterates over the given range of lines, _without_ returning the line breaks between, and yielding empty strings for empty lines.” Platform-independent; needed by a native editor as-is. |
| state.cm6.242 | `Text.toString` | `state/src/text.ts:132` | CORE | cm6: “Return the document as a string, using newline characters to separate lines.” Platform-independent; needed by a native editor as-is. |
| state.cm6.243 | `Text.toJSON` | `state/src/text.ts:136` | CORE | Serialization (array of lines) used by `EditorState.toJSON` and collab/persistence; Rust idiom would be serde, but the format is observable. |
| state.cm6.244 | `Text.children` | `state/src/text.ts:147` | CORE | Public accessor of the tree structure (`text.ts:147`). Proposed CORE because it is public API, but it exposes a representation detail (leaf/node split, 32-line leaves): whether a native editor needs it is a human question (see `human-decisions.md`). |
| state.cm6.245 | `Text.[Symbol.iterator]` | `state/src/text.ts:150` | CORE | JS iteration protocol over the document chunks; native equivalent is `IntoIterator`/`Iterator` over chunks. |
| state.cm6.246 | `Text^of` | `state/src/text.ts:153` | CORE | cm6: “Create a `Text` instance for the given array of lines.” Platform-independent; needed by a native editor as-is. |
| state.cm6.247 | `Text^empty` | `state/src/text.ts:160` | CORE | cm6: “The empty document.” Platform-independent; needed by a native editor as-is. |
| state.cm6.248 | `Facet.define config.combine` | `state/src/facet.ts:12` | CORE | cm6: “How to combine the input values into a single output value.” Platform-independent; needed by a native editor as-is. |
| state.cm6.249 | `Facet.define config.compare` | `state/src/facet.ts:17` | CORE | cm6: “How to compare output values to determine whether the value of the facet changed.” Platform-independent; needed by a native editor as-is. |
| state.cm6.250 | `Facet.define config.compareInput` | `state/src/facet.ts:20` | CORE | cm6: “How to compare input values to avoid recomputing the output value when no inputs changed.” Platform-independent; needed by a native editor as-is. |
| state.cm6.251 | `Facet.define config.static` | `state/src/facet.ts:22` | CORE | cm6: “Forbids dynamic inputs to this facet.” Platform-independent; needed by a native editor as-is. |
| state.cm6.252 | `Facet.define config.enables` | `state/src/facet.ts:29` | CORE | cm6: “If given, these extension(s) (or the result of calling the given function with the facet) will be added to any state where this facet is provided.” Platform-independent; needed by a native editor as-is. |
| state.cm6.253 | `StateField.define config.create` | `state/src/facet.ts:262` | CORE | cm6: “Creates the initial value for the field when a state is created.” Platform-independent; needed by a native editor as-is. |
| state.cm6.254 | `StateField.define config.update` | `state/src/facet.ts:266` | CORE | cm6: “Compute a new value from the field's previous value and a transaction.” Platform-independent; needed by a native editor as-is. |
| state.cm6.255 | `StateField.define config.compare` | `state/src/facet.ts:272` | CORE | cm6: “Compare two values of the field, returning `true` when they are the same.” Platform-independent; needed by a native editor as-is. |
| state.cm6.256 | `StateField.define config.provide` | `state/src/facet.ts:280` | CORE | cm6: “Provide extensions based on this field.” Platform-independent; needed by a native editor as-is. |
| state.cm6.257 | `StateField.define config.toJSON` | `state/src/facet.ts:285` | CORE | cm6: “A function used to serialize this field's content to JSON.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.258 | `StateField.define config.fromJSON` | `state/src/facet.ts:289` | CORE | cm6: “A function that deserializes the JSON representation of this field's content.” Platform-independent; needed by a native editor as-is. JSON: could be `DEFER` under D10. |
| state.cm6.259 | `StateEffect.define spec.map` | `state/src/transaction.ts:43` | CORE | cm6: “Provides a way to map an effect like this through a position mapping.” Platform-independent; needed by a native editor as-is. |
| state.cm6.260 | `RangeSet.update spec.add` | `state/src/rangeset.ts:207` | CORE | cm6: “An array of ranges to add.” Platform-independent; needed by a native editor as-is. |
| state.cm6.261 | `RangeSet.update spec.sort` | `state/src/rangeset.ts:210` | CORE | cm6: “Indicates whether the library should sort the ranges in `add`.” Platform-independent; needed by a native editor as-is. |
| state.cm6.262 | `RangeSet.update spec.filter` | `state/src/rangeset.ts:213` | CORE | cm6: “Filter the ranges already in the set.” Platform-independent; needed by a native editor as-is. |
| state.cm6.263 | `RangeSet.update spec.filterFrom` | `state/src/rangeset.ts:217` | CORE | cm6: “Can be used to limit the range on which the filter is applied.” Platform-independent; needed by a native editor as-is. |
| state.cm6.264 | `RangeSet.update spec.filterTo` | `state/src/rangeset.ts:219` | CORE | cm6: “The end position to apply the filter to.” Platform-independent; needed by a native editor as-is. |

## Per source file

| cm6 file | disposition | reason |
|---|---|---|
| `state/src/text.ts` | CORE | Document data structure (`Text`, `Line`, iterators). |
| `state/src/change.ts` | CORE | Change representation, mapping, composition, inversion, JSON. |
| `state/src/selection.ts` | CORE | Selection model and normalization. |
| `state/src/state.ts` | CORE | `EditorState`, configuration-driven queries, built-in facets as statics. |
| `state/src/transaction.ts` | CORE | Transactions, annotations, effects, filter/extender pipeline. |
| `state/src/facet.ts` | CORE | Extension system: facets, fields, precedence, compartments, configuration resolution. |
| `state/src/extension.ts` | CORE | Built-in facet definitions and the `StateCommand` type. |
| `state/src/config.ts` | CORE | `combineConfig` utility. |
| `state/src/rangeset.ts` | CORE | `RangeSet` data structure used by view (decorations) and language (folding, highlighting). |
| `state/src/char.ts` | CORE | Cluster breaking and code point helpers; position-unit dependent (D1). |
| `state/src/charcategory.ts` | CORE | Character categories for word selection and motion. |
| `state/src/column.ts` | CORE | Column counting with tabs and clusters (indentation, vertical motion). |
| `state/src/index.ts` | CORE | Public export surface; defines what A1 contains. |

## Counts

Exports: 264 rows: `CORE` 262, `DROP` 2. Source files: 13 `CORE`.

Items whose disposition depends on a decision: `codePointAt`, `fromCodePoint`, `codePointSize` (D1); `StateCommand` (D5); 12 JSON rows (D10); `Text.children` (human question).
