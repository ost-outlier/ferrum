# ferrum-view-gpui / src — API Reference

This crate provides the native GPU desktop window and UI adapter connecting Ferrum's abstract `EditorView` to GPUI (Apache-2.0).

---

### Component & View Entity

- `FerrumEditorView`: GPUI view component hosting the editor engine.
  - `FerrumEditorView::new(state: Arc<EditorState>, cx: &mut Context<Self>) -> Self`: Instantiates view with default Tokyo Night theme.
  - `editor.pos_at_coords(position: Point<Pixels>) -> Option<usize>`: Hit-testing translating mouse window coordinates to document character offsets.
  - `editor.on_key_down(event, window, cx)`: Keyboard dispatcher with clipboard support (`Mod-c`, `Mod-x`, `Mod-v`) and `KEYMAP` routing.
  - `editor.on_mouse_down(event, window, cx)`: Click handler (single-click cursor, double-click word selection, triple-click line selection, shift-click range extension).
  - `editor.on_mouse_move(event, window, cx)`: Interactive drag-selection handler.
  - `editor.on_mouse_up(event, window, cx)`: Concludes drag selection.

---

### Canvas Element & Rendering Pipeline

- `EditorCanvasElement`: Low-level GPUI element handling the rendering pipeline:
  - `request_layout`: Requests full bounds expansion in parent container.
  - `prepaint`: Shapes visible text lines with `window.text_system()`, computes gutter line numbers, and builds selection quads and caret paint quads.
  - `paint`: Paints background, gutter, selection highlights, text lines, and cursor on the GPU surface.

---

### Layout & Byte/Char Conversion Utilities

- `char_to_byte_offset(text: &str, char_offset: usize) -> usize`: Converts a character (codepoint) offset within a line to its corresponding UTF-8 byte offset for GPUI `ShapedLine::x_for_index`.
- `byte_to_char_offset(text: &str, byte_offset: usize) -> usize`: Converts a UTF-8 byte offset returned by GPUI `ShapedLine::closest_index_for_x` to the corresponding character offset for document operations.

---

### Keyboard Bridge

- `handle_keystroke(state: &EditorState, keystroke: &Keystroke) -> Option<TransactionSpec>`: Parses GPUI modifiers (`Mod`, `Shift`, `Alt`) and key names into Ferrum keymap queries via `KEYMAP` facet. Falls back to character insertion for printable keys.

---

### Themes & Styling

- `EditorTheme`: Theme configuration controlling `background`, `text_color`, `gutter_bg`, `gutter_fg`, `cursor_color`, and `selection_color`.
  - `EditorTheme::tokyo_night() -> Self`: Default dark theme preset.
  - `EditorTheme::one_dark() -> Self`: One Dark theme preset.
  - `EditorTheme::light() -> Self`: Clean light theme preset.
