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

#[test]
fn test_mouse_word_selection_double_click() {
    use ferrum_state::char::find_word_break;

    let doc = Text::from_str("fn hello_world() {}");
    let line = doc.line(0);
    let line_text = doc.slice_to_string(line.from..line.to);

    // Double click at position 5 (inside "hello_world")
    let pos = 5;
    let rel_pos = pos - line.from;
    let start_rel = find_word_break(&line_text, rel_pos, false);
    let end_rel = find_word_break(&line_text, rel_pos, true);
    let word_from = line.from + start_rel;
    let word_to = line.from + end_rel;

    assert_eq!(&line_text[start_rel..end_rel], "hello_world");
    let sel = EditorSelection::single(word_from, word_to);
    assert_eq!(sel.main().from(), 3);
    assert_eq!(sel.main().to(), 14);
}

#[test]
fn test_mouse_drag_selection_extension() {
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("Hello Ferrum World")),
        selection: Some(EditorSelection::cursor(0)),
        extensions: None,
    });

    // Simulate drag start at 0, move to 12
    let anchor = 0;
    let move_pos = 12;
    let sel = EditorSelection::single(anchor, move_pos);
    let tr = state.update(ferrum_state::transaction::TransactionSpec::new().with_selection(sel)).unwrap();
    let next_state = state.apply(tr);

    let main = next_state.selection().main();
    assert_eq!(main.anchor, 0);
    assert_eq!(main.head, 12);
    assert_eq!(next_state.doc().slice_to_string(main.from()..main.to()), "Hello Ferrum");
}

#[test]
fn test_clipboard_paste_and_cut_semantics() {
    use ferrum_commands::insert_text;

    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from_str("Hello World")),
        selection: Some(EditorSelection::single(0, 5)), // "Hello" selected
        extensions: None,
    });

    // Cut: copy text, then replace with ""
    let selected_text = state.doc().slice_to_string(state.selection().main().from()..state.selection().main().to());
    assert_eq!(selected_text, "Hello");

    let cut_spec = insert_text(&state, "").unwrap();
    let tr_cut = state.update(cut_spec).unwrap();
    let after_cut = state.apply(tr_cut);
    assert_eq!(after_cut.doc().to_string(), " World");

    // Paste: insert text
    let paste_spec = insert_text(&after_cut, "Greetings").unwrap();
    let tr_paste = after_cut.update(paste_spec).unwrap();
    let after_paste = after_cut.apply(tr_paste);
    assert_eq!(after_paste.doc().to_string(), "Greetings World");
}

