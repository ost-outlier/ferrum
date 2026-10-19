//! `ferrum-view` - View abstractions, decorations, viewport virtualization, and surface contracts.

pub mod decoration;
pub mod heightmap;
pub mod view;
pub mod viewport;

pub use decoration::{Decoration, DecorationKind, DecorationSet, DECORATIONS};
pub use heightmap::{BlockInfo, HeightMap, HeightOracle, QueryType};
pub use view::{DecoratedRange, EditorView, RenderedLine, ViewUpdate};
pub use viewport::Viewport;

#[cfg(test)]
mod tests;
