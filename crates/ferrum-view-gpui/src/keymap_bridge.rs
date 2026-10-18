use ferrum_commands::{dispatch_key, insert_text};
use ferrum_state::state::EditorState;
use ferrum_state::transaction::TransactionSpec;
use gpui::Keystroke;

/// Converts a GPUI keystroke into CodeMirror-compatible key name and shift flag.
pub fn parse_keystroke(keystroke: &Keystroke) -> (String, Option<String>, bool) {
    let is_mod = keystroke.modifiers.platform || keystroke.modifiers.control;
    let is_alt = keystroke.modifiers.alt;
    let is_shift = keystroke.modifiers.shift;

    let key_lower = keystroke.key.to_lowercase();
    let key_name: &str = match key_lower.as_str() {
        "left" | "arrowleft" => "ArrowLeft",
        "right" | "arrowright" => "ArrowRight",
        "up" | "arrowup" => "ArrowUp",
        "down" | "arrowdown" => "ArrowDown",
        "backspace" => "Backspace",
        "delete" => "Delete",
        "enter" | "return" => "Enter",
        "tab" => "Tab",
        "escape" => "Escape",
        "home" => "Home",
        "end" => "End",
        "pageup" => "PageUp",
        "pagedown" => "PageDown",
        _ => &keystroke.key,
    };

    let mut base_parts = Vec::new();
    if is_mod {
        base_parts.push("Mod");
    }
    if is_alt {
        base_parts.push("Alt");
    }
    base_parts.push(key_name);
    let base_name = base_parts.join("-");

    // Also build explicit Shift-prefixed name if shift is active (e.g. Shift-Mod-z)
    let explicit_shift_name = if is_shift {
        let mut shift_parts = vec!["Shift"];
        if is_mod {
            shift_parts.push("Mod");
        }
        if is_alt {
            shift_parts.push("Alt");
        }
        shift_parts.push(key_name);
        Some(shift_parts.join("-"))
    } else {
        None
    };

    (base_name, explicit_shift_name, is_shift)
}

/// Dispatches a keystroke against the editor state:
/// 1. First tries command keybindings from KEYMAP facet (including undo, cursor moves, selections).
/// 2. If no command matched and the key is printable text, generates an `insert_text` transaction.
pub fn handle_keystroke(state: &EditorState, keystroke: &Keystroke) -> Option<TransactionSpec> {
    let (base_name, explicit_shift_name, is_shift) = parse_keystroke(keystroke);

    // 1. Try explicit Shift-Mod keybinding if present (e.g. Shift-Mod-z for redo)
    if let Some(shift_name) = explicit_shift_name {
        if let Some(spec) = dispatch_key(state, &shift_name, false) {
            return Some(spec);
        }
    }

    // 2. Try standard keybinding with shift flag (e.g. ArrowLeft with is_shift for select_char_left)
    if let Some(spec) = dispatch_key(state, &base_name, is_shift) {
        return Some(spec);
    }

    // 3. If no command matched, check if it's printable character text
    let is_mod = keystroke.modifiers.platform || keystroke.modifiers.control;
    let is_alt = keystroke.modifiers.alt;

    if !is_mod && !is_alt {
        // Handle direct text characters
        if let Some(key_char) = &keystroke.key_char {
            return insert_text(state, key_char);
        } else if keystroke.key.chars().count() == 1 {
            return insert_text(state, &keystroke.key);
        } else if keystroke.key.eq_ignore_ascii_case("space") {
            return insert_text(state, " ");
        }
    }

    None
}
