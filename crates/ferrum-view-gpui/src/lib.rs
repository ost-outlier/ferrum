//! `ferrum-view-gpui` - GPUI rendering surface and native window bridge for Ferrum.
//!
//! Provides a hermetic, interactive desktop UI integration for Ferrum text editor engine
//! powered by GPUI (Apache-2.0).

pub mod editor_view;
pub mod keymap_bridge;
pub mod theme;

pub use editor_view::{EditorCanvasElement, FerrumEditorView};
pub use keymap_bridge::handle_keystroke;
pub use theme::EditorTheme;

#[cfg(test)]
mod tests;

