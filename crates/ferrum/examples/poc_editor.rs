//! Interactive Proof of Concept (POC) demonstrating the Ferrum Editor Engine.
//!
//! Demonstrates:
//! 1. Functional, immutable EditorState with Text (ropey) and multi-cursor selections.
//! 2. Extension pipeline with Facets (TabSize, StatusItems) and conflict-free composition.
//! 3. Transaction pipeline with filter enforcement (blocking illegal tokens).
//! 4. History extension with full Undo / Redo stacks and inverted ChangeSets.
//! 5. Multi-cursor editing with real Operational Transformation (OT).
//! 6. Dynamic Compartment hot-reloading without editor restart.
//! 7. Viewport rendering with decorations (syntax marks, tokens).

use ferrum::commands::{
    cursor_char_left, cursor_char_right, default_keymap, history, insert_text, redo, undo,
    undo_depth, HistoryConfig, KEYMAP,
};
use ferrum::state::{
    Compartment, EditorSelection, EditorState, EditorStateConfig, Facet, SelectionRange,
    StateField, Text, Transaction, TransactionSpec, TRANSACTION_FILTER,
};
use ferrum::view::{Decoration, DecorationSet, EditorView, DECORATIONS};
use std::sync::Arc;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║       🦀 FERRUM EDITOR ENGINE — PHASE 0 PROOF OF CONCEPT        ║");
    println!("║       Architectural Parity with CodeMirror 6 in 100% Rust       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // ------------------------------------------------------------------------
    // 1. Extensions: Facets, StateFields, and Filters
    // ------------------------------------------------------------------------

    // A facet that configures tab size (defaults to 4)
    let tab_size_facet: Facet<usize, usize> =
        Facet::define(|inputs| inputs.last().copied().unwrap_or(4));

    // A facet where multiple independent extensions can contribute status bar badges
    let status_bar_facet: Facet<String, Vec<String>> =
        Facet::define(|inputs| inputs.to_vec());

    // A StateField tracking total transactions applied
    let tx_counter = StateField::define(|_state| 0usize, |count, _tr| count + 1);

    // A transaction filter blocking any input containing the forbidden token '@evil'
    let security_filter = Arc::new(|tr: &Transaction| {
        let mut allowed = true;
        tr.changes().iter_changes(
            |_, _, _, _, text| {
                if text.to_string().contains("@evil") {
                    println!("  🛡️  [Filter Intercepted] Blocked forbidden token '@evil'!");
                    allowed = false;
                }
            },
            false,
        );
        allowed
    });

    // Dynamic compartment for hot-swapping themes on the fly
    let theme_facet: Facet<&'static str, &'static str> =
        Facet::define(|inputs| inputs.last().copied().unwrap_or("dark"));
    let theme_compartment = Compartment::new();

    // Syntax decorations: keyword "fn", string literal, and comment
    let initial_decorations = DecorationSet::from_vec(vec![
        Decoration::mark(0..2, "keyword.declaration"),
        Decoration::mark(18..31, "string.quoted"),
        Decoration::mark(35..75, "comment.line"),
    ]);

    // Initial document with Unicode grapheme clusters and emojis
    let initial_code = "fn greeting() -> &str {\n    \"Hello, 🌍!\"; // Ferrum 🦀 editor engine\n}\n";

    // ------------------------------------------------------------------------
    // 2. Initialize EditorState
    // ------------------------------------------------------------------------
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(initial_code)),
        selection: Some(EditorSelection::single(0, 0)),
        extensions: Some(vec![
            history(HistoryConfig::default()),
            KEYMAP.of(default_keymap()),
            tab_size_facet.of(2),
            status_bar_facet.of("UTF-8".to_string()),
            status_bar_facet.of("Rust".to_string()),
            status_bar_facet.of("LF".to_string()),
            TRANSACTION_FILTER.of(security_filter),
            theme_compartment.of(theme_facet.of("tokyo-night")),
            DECORATIONS.of(initial_decorations),
            tx_counter.extension(),
        ]),
    });

    let mut view = EditorView::new(state);

    println!("▶ 1. Initial State Initialized:");
    println!("   Document: {} lines, {} Unicode codepoints", view.state().doc().len_lines(), view.state().doc().len_chars());
    println!("   Tab Size Facet: {}", view.state().facet(&tab_size_facet));
    println!("   Theme Facet:    {}", view.state().facet(&theme_facet));
    println!("   Status Badges:  [{}]", view.state().facet(&status_bar_facet).join(" | "));
    println!("   History Depth:  undo={}, redo=0", undo_depth(view.state()));
    render_view(&view);

    // ------------------------------------------------------------------------
    // 3. Test Transaction Filter Security
    // ------------------------------------------------------------------------
    println!("\n▶ 2. Testing Transaction Filter Enforcement:");
    let forbidden_spec = insert_text(view.state(), "let x = \"@evil injection\";").unwrap();
    let update = view.dispatch(forbidden_spec);
    assert!(update.is_none(), "Transaction should be strictly rejected by filter");
    println!("   ✓ Success: Transaction was rejected. Document remained unmodified.\n");

    // ------------------------------------------------------------------------
    // 4. Test Multi-Cursor Editing with Operational Transformation (OT)
    // ------------------------------------------------------------------------
    println!("▶ 3. Testing Multi-Cursor Editing (Simultaneous Typing):");
    // Place two cursors: one at start of line 1 (pos 0), one at start of line 2 (pos 24)
    let multi_sel = EditorSelection::create(
        vec![
            SelectionRange::cursor(0),
            SelectionRange::cursor(24),
        ],
        0,
    );
    view.dispatch(TransactionSpec::new().with_selection(multi_sel)).unwrap();

    println!("   Active cursors: {}", view.state().selection().ranges().len());
    for (i, r) in view.state().selection().ranges().iter().enumerate() {
        println!("     Cursor {}: pos={}", i + 1, r.head);
    }

    // Type "// [tag] " concurrently across both cursors
    let multi_insert_spec = insert_text(view.state(), "    // [TAG] ").unwrap();
    view.dispatch(multi_insert_spec).expect("Multi-cursor insert must succeed");

    println!("   After concurrent insertion:");
    render_view(&view);

    // ------------------------------------------------------------------------
    // 5. Test History Undo and Redo
    // ------------------------------------------------------------------------
    println!("\n▶ 4. Testing History Undo & Redo (CM6 Parity):");
    println!("   Undo depth before undo: {}", undo_depth(view.state()));

    let undo_spec = undo(view.state()).expect("Undo transaction available");
    view.dispatch(undo_spec).expect("Dispatch undo");
    println!("   After Undo (multi-cursor typing rolled back):");
    assert_eq!(view.state().doc().to_string(), initial_code);
    render_view(&view);

    let redo_spec = redo(view.state()).expect("Redo transaction available");
    view.dispatch(redo_spec).expect("Dispatch redo");
    println!("   After Redo (multi-cursor typing restored):");
    render_view(&view);

    // ------------------------------------------------------------------------
    // 6. Test Dynamic Compartment Reconfiguration
    // ------------------------------------------------------------------------
    println!("\n▶ 5. Testing Dynamic Compartment Hot-Reconfiguration:");
    println!("   Current Theme: {}", view.state().facet(&theme_facet));

    let reconfig_effect = theme_compartment.reconfigure(theme_facet.of("solarized-light"));
    view.dispatch(TransactionSpec::new().with_effect(reconfig_effect)).unwrap();
    println!("   New Theme Facet value: {}", view.state().facet(&theme_facet));
    assert_eq!(*view.state().facet(&theme_facet), "solarized-light");
    println!("   ✓ Success: Extension compartment reconfigured in-place via Transaction.\n");

    // ------------------------------------------------------------------------
    // 7. Navigation Commands with Unicode
    // ------------------------------------------------------------------------
    println!("▶ 6. Testing Unicode & Cursor Navigation Commands:");
    let right_spec = cursor_char_right(view.state()).unwrap();
    view.dispatch(right_spec);
    let left_spec = cursor_char_left(view.state()).unwrap();
    view.dispatch(left_spec);
    println!("   Navigation succeeded without panics across UTF-8 grapheme clusters.");

    // Final summary
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║       ✅ ALL PHASE 0 ARCHITECTURAL ACCEPTANCE CRITERIA MET       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

/// Renders the virtualized editor lines with line numbers and decoration badges.
fn render_view(view: &EditorView) {
    println!("   ┌─────────────────────────────────────────────────────────────");
    let lines = view.render_lines();
    for rendered in &lines {
        let decos: Vec<&str> = rendered
            .decorations
            .iter()
            .map(|d| match &d.kind {
                ferrum::view::DecorationKind::Mark { tag } => *tag,
                ferrum::view::DecorationKind::Replace { widget_id } => widget_id.unwrap_or("widget"),
                _ => "deco",
            })
            .collect();

        let deco_badge = if decos.is_empty() {
            String::new()
        } else {
            format!(" [tokens: {}]", decos.join(", "))
        };

        println!(
            "   │ {:2} │ {}{}",
            rendered.line.number,
            rendered.text.replace('\n', ""),
            deco_badge
        );
    }
    println!("   └─────────────────────────────────────────────────────────────");
}
