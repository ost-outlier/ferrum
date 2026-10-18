use crate::keymap_bridge::{handle_keystroke, parse_keystroke};
use crate::theme::EditorTheme;
use ferrum_commands::history::{history, HistoryConfig};
use ferrum_commands::{default_keymap, history_keymap, standard_keymap, KEYMAP};
use ferrum_state::selection::EditorSelection;
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use gpui::{Keystroke, Modifiers};

#[test]
fn test_parse_keystroke_basic() {
    let ks = Keystroke {
        modifiers: Modifiers::default(),
        key: "a".to_string(),
        key_char: Some("a".to_string()),
    };
    let (base, explicit_shift, is_shift) = parse_keystroke(&ks);
    assert_eq!(base, "a");
    assert_eq!(explicit_shift, None);
    assert!(!is_shift);
}

#[test]
fn test_parse_keystroke_arrows_and_modifiers() {
    let ks = Keystroke {
        modifiers: Modifiers {
            control: true,
            ..Default::default()
        },
        key: "z".to_string(),
        key_char: None,
    };
    let (base, explicit_shift, is_shift) = parse_keystroke(&ks);
    assert_eq!(base, "Mod-z");
    assert_eq!(explicit_shift, None);
    assert!(!is_shift);

    let ks_shift = Keystroke {
        modifiers: Modifiers {
            control: true,
            shift: true,
            ..Default::default()
        },
        key: "z".to_string(),
        key_char: None,
    };
    let (base, explicit_shift, is_shift) = parse_keystroke(&ks_shift);
    assert_eq!(base, "Mod-z");
    assert_eq!(explicit_shift, Some("Shift-Mod-z".to_string()));
    assert!(is_shift);
}

#[test]
fn test_handle_keystroke_typing() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("hello")),
        selection: Some(EditorSelection::cursor(5)),
        extensions: Some(vec![KEYMAP.of(default_keymap())]),
    });

    let ks = Keystroke {
        modifiers: Modifiers::default(),
        key: "!".to_string(),
        key_char: Some("!".to_string()),
    };

    let spec = handle_keystroke(&state, &ks).expect("keystroke handled");
    let tr = state.update(spec).expect("valid transaction");
    let next_state = state.apply(tr);
    assert_eq!(next_state.doc().to_string(), "hello!");
    assert_eq!(next_state.selection().main().head, 6);
}

#[test]
fn test_handle_keystroke_navigation_command() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("hello")),
        selection: Some(EditorSelection::cursor(5)),
        extensions: Some(vec![KEYMAP.of(standard_keymap()), KEYMAP.of(default_keymap())]),
    });

    let ks_left = Keystroke {
        modifiers: Modifiers::default(),
        key: "ArrowLeft".to_string(),
        key_char: None,
    };

    let spec = handle_keystroke(&state, &ks_left).expect("arrow handled");
    let tr = state.update(spec).expect("valid transaction");
    let next_state = state.apply(tr);
    assert_eq!(next_state.doc().to_string(), "hello");
    assert_eq!(next_state.selection().main().head, 4);
}

#[test]
fn test_handle_keystroke_undo_command() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("")),
        selection: Some(EditorSelection::cursor(0)),
        extensions: Some(vec![
            history(HistoryConfig::default()),
            KEYMAP.of(history_keymap()),
            KEYMAP.of(standard_keymap()),
            KEYMAP.of(default_keymap()),
        ]),
    });

    // Type 'A'
    let ks_a = Keystroke {
        modifiers: Modifiers::default(),
        key: "A".to_string(),
        key_char: Some("A".to_string()),
    };
    let spec1 = handle_keystroke(&state, &ks_a).expect("typing A handled");
    let tr1 = state.update(spec1).expect("valid transaction");
    let state2 = state.apply(tr1);
    assert_eq!(state2.doc().to_string(), "A");

    // Press Mod-z (undo)
    let ks_undo = Keystroke {
        modifiers: Modifiers {
            control: true,
            ..Default::default()
        },
        key: "z".to_string(),
        key_char: None,
    };
    let spec_undo = handle_keystroke(&state2, &ks_undo).expect("Mod-z matched undo keymap");
    let tr2 = state2.update(spec_undo).expect("valid undo transaction");
    let state3 = state2.apply(tr2);
    assert_eq!(state3.doc().to_string(), "");
}

#[test]
fn test_editor_theme_defaults() {
    let theme = EditorTheme::tokyo_night();
    assert_eq!(theme.background, EditorTheme::default().background);
}
