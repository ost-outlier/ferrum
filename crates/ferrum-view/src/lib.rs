//! `ferrum-view` - View abstractions, decorations, viewport virtualization, and surface contracts.

pub mod decoration;
pub mod heightmap;
pub mod view;
pub mod viewport;

pub use decoration::{
    Decoration, DecorationKind, DecorationSet, ReplaceSpec, WidgetSpec, WidgetType, DECORATIONS,
};
pub use heightmap::{
    clear_height_change_flag, get_height_change_flag, set_height_change_flag, BlockInfo, BlockType,
    ChangedRange, HeightMap, HeightOracle, MeasuredHeights, QueryType,
};
pub use view::{DecoratedRange, EditorView, RenderedLine, ViewUpdate};
pub use viewport::Viewport;

#[cfg(test)]
mod tests;
