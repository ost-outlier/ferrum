use gpui::{rgba, Rgba};

/// Theme configuration for styling editor elements and text tokens.
#[derive(Debug, Clone)]
pub struct EditorTheme {
    pub background: Rgba,
    pub gutter_bg: Rgba,
    pub gutter_fg: Rgba,
    pub text_color: Rgba,
    pub cursor_color: Rgba,
    pub selection_color: Rgba,
    pub keyword_color: Rgba,
    pub string_color: Rgba,
    pub comment_color: Rgba,
    pub type_color: Rgba,
    pub function_color: Rgba,
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self::tokyo_night()
    }
}

impl EditorTheme {
    /// Tokyo Night dark theme.
    pub fn tokyo_night() -> Self {
        Self {
            background: rgba(0x1a1b26ff),
            gutter_bg: rgba(0x16161eff),
            gutter_fg: rgba(0x565f89ff),
            text_color: rgba(0xa9b1d6ff),
            cursor_color: rgba(0xc0caf5ff),
            selection_color: rgba(0x33467c66),
            keyword_color: rgba(0xbb9af7ff),
            string_color: rgba(0x9ece6aff),
            comment_color: rgba(0x565f89ff),
            type_color: rgba(0x2ac3deff),
            function_color: rgba(0x7aa2f7ff),
        }
    }

    /// Resolves token tag to a theme color.
    pub fn color_for_tag(&self, tag: &str) -> Rgba {
        if tag.starts_with("keyword") {
            self.keyword_color
        } else if tag.starts_with("string") {
            self.string_color
        } else if tag.starts_with("comment") {
            self.comment_color
        } else if tag.starts_with("type") {
            self.type_color
        } else if tag.starts_with("function") {
            self.function_color
        } else {
            self.text_color
        }
    }
}
