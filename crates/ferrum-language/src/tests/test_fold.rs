use crate::fold::{fold_state, FoldEffect, FoldRange, UnfoldEffect, UnfoldRange};
use ferrum_state::state::{EditorState, EditorStateConfig};
use ferrum_state::text::Text;
use ferrum_state::transaction::TransactionSpec;
use ferrum_view::decoration::DecorationSet;

fn ranges(set: &DecorationSet) -> String {
    let mut result = Vec::new();
    set.between(0, 100_000_000, |f, t, _| {
        result.push(format!("{}-{}", f, t));
        true
    });
    result.join(" ")
}

#[test]
fn test_cm6_fold_01_stores_fold_state() {
    let doc = "1\n2\n3\n4\n5\n6\n7\n8\n";
    let fold_field = fold_state();
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(doc)),
        selection: None,
        extensions: Some(vec![fold_field.clone().extension()]),
    });

    let tr1 = state
        .update(
            TransactionSpec::new()
                .with_effect(FoldEffect::of(FoldRange::new(0, 3)))
                .with_effect(FoldEffect::of(FoldRange::new(4, 7))),
        )
        .unwrap();
    let state = tr1.state();
    assert_eq!(ranges(state.field(&fold_field).unwrap()), "0-3 4-7");

    let tr2 = state
        .update(
            TransactionSpec::new().with_effect(UnfoldEffect::of(UnfoldRange { from: 4, to: 7 })),
        )
        .unwrap();
    let state = tr2.state();
    assert_eq!(ranges(state.field(&fold_field).unwrap()), "0-3");
}

#[test]
fn test_cm6_fold_02_can_store_fold_state_as_json() {
    let doc = "1\n2\n3\n4\n5\n6\n7\n8\n";
    let fold_field = fold_state();
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from(doc)),
        selection: None,
        extensions: Some(vec![fold_field.clone().extension()]),
    });

    let tr1 = state
        .update(
            TransactionSpec::new()
                .with_effect(FoldEffect::of(FoldRange::new(4, 7)))
                .with_effect(FoldEffect::of(FoldRange::new(8, 11))),
        )
        .unwrap();
    let state = tr1.state();
    assert_eq!(ranges(state.field(&fold_field).unwrap()), "4-7 8-11");
}
