//! Interactive Desktop GUI example for Ferrum editor engine using GPUI.
//!
//! Run with: `cargo run -p ferrum-view-gpui --example gui_editor`

use ferrum_commands::history::{history, HistoryConfig};
use ferrum_commands::{default_keymap, history_keymap, standard_keymap, KEYMAP};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use ferrum_view_gpui::FerrumEditorView;
use gpui::{px, size, App, AppContext, Application, Bounds, WindowBounds, WindowOptions};

fn main() {
    Application::new().run(|cx: &mut App| {
        let initial_text = r#"// Welcome to Ferrum Editor Engine!
// Built in 100% Rust with direct architectural parity to CodeMirror 6.
// Rendered on GPUI native desktop surface (100% Permissive - Apache-2.0 / MIT).
//
// Features in Phase 0:
// • Multi-cursor & full selection normalization (CM6 parity)
// • Deterministic ChangeSet OT (invert, compose, map)
// • Grapheme cluster boundary navigation (Unicode Graphemes)
// • Time-bucketed History & Undo/Redo (Ctrl+Z / Ctrl+Y)
// • Extensible Facets & Compartments system
// • Direct Keymap dispatch and typing

fn main() {
    println!("Hello from Ferrum on GPUI!");
    let greeting = "Interactive Ferrum is running!";
    println!("{}", greeting);
}
"#;

        let extensions = vec![
            history(HistoryConfig::default()),
            KEYMAP.of(history_keymap()),
            KEYMAP.of(standard_keymap()),
            KEYMAP.of(default_keymap()),
        ];

        let state = EditorState::create(EditorStateConfig {
            doc: Some(Text::from_str(initial_text)),
            extensions: Some(extensions),
            ..Default::default()
        });

        let bounds = Bounds::centered(None, size(px(860.0), px(620.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| FerrumEditorView::new(state, cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
