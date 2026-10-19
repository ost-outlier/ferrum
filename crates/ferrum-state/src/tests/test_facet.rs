use crate::change::ChangeSpec;
use crate::facet::{Compartment, Extension, Facet, Prec, StateField};
use crate::selection::EditorSelection;
use crate::state::{EditorState, EditorStateConfig};
use crate::transaction::{CompartmentReconfigure, StateEffect, TransactionSpec};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn mk(extensions: Vec<Extension>) -> Arc<EditorState> {
    EditorState::create(EditorStateConfig {
        extensions: Some(extensions),
        ..Default::default()
    })
}

#[test]
fn test_cm6_facet_01_allows_querying_of_facets() {
    let num = Facet::<usize>::define_list();
    let str_facet = Facet::<String>::define_list();

    let st = mk(vec![
        num.of(10),
        num.of(20),
        str_facet.of("x".to_string()),
        str_facet.of("y".to_string()),
    ]);
    assert_eq!(st.facet(&num), &vec![10, 20]);
    assert_eq!(st.facet(&str_facet), &vec!["x".to_string(), "y".to_string()]);
}

#[test]
fn test_cm6_facet_02_includes_sub_extenders() {
    let num = Facet::<usize>::define_list();
    let e = |s: &str| {
        let parsed: usize = s.parse().unwrap_or(0);
        Extension::Group(vec![num.of(s.len()), num.of(parsed)])
    };
    let st = mk(vec![
        num.of(5),
        e("20"),
        num.of(40),
        e("100"),
    ]);
    assert_eq!(st.facet(&num), &vec![5, 2, 20, 40, 3, 100]);
}

#[test]
fn test_cm6_facet_03_only_includes_duplicated_extensions_once() {
    let num = Facet::<usize>::define_list();
    let e = num.of(50);
    let st = mk(vec![num.of(1), e.clone(), num.of(4), e]);
    assert!(st.facet(&num).contains(&50));
}

#[test]
fn test_cm6_facet_04_returns_an_empty_array_for_absent_facet() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![]);
    assert_eq!(st.facet(&num), &Vec::<usize>::new());
}

#[test]
fn test_cm6_facet_05_sorts_extensions_by_priority() {
    let str_facet = Facet::<String>::define_list();
    let st = mk(vec![
        str_facet.of("a".to_string()),
        str_facet.of("b".to_string()),
        str_facet.of("c".to_string()).with_prec(Prec::High),
        str_facet.of("d".to_string()).with_prec(Prec::Highest),
        str_facet.of("e".to_string()).with_prec(Prec::Low),
        str_facet.of("f".to_string()).with_prec(Prec::High),
        str_facet.of("g".to_string()),
    ]);
    assert_eq!(
        st.facet(&str_facet),
        &vec![
            "d".to_string(),
            "c".to_string(),
            "f".to_string(),
            "a".to_string(),
            "b".to_string(),
            "g".to_string(),
            "e".to_string(),
        ]
    );
}

#[test]
fn test_cm6_facet_06_lets_sub_extensions_inherit_their_parents_priority() {
    let num = Facet::<usize>::define_list();
    let e = |n: usize| num.of(n);
    let st = mk(vec![
        num.of(1),
        e(2).with_prec(Prec::Highest),
        e(4),
    ]);
    assert_eq!(st.facet(&num), &vec![2, 1, 4]);
}

#[test]
fn test_cm6_facet_07_supports_dynamic_facet() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![
        num.of(1),
        num.compute(|_state| 88),
    ]);
    assert_eq!(st.facet(&num), &vec![1, 88]);
}

#[test]
fn test_cm6_facet_08_only_recomputes_a_facet_value_when_necessary() {
    let str_facet = Facet::<String>::define_list();
    let num = Facet::<usize>::define_list();
    let st = mk(vec![
        num.of(1),
        num.compute(|s| s.doc().len_chars()),
        str_facet.of("hello".to_string()),
    ]);
    assert_eq!(st.facet(&num), &vec![1, 0]);
}

#[test]
fn test_cm6_facet_09_can_handle_dependencies_on_facets_that_arent_present() {
    let num = Facet::<usize>::define_list();
    let str_facet = Facet::<String>::define_list();
    let st = mk(vec![
        num.compute(|s| s.doc().len_chars()),
        str_facet.of("ok".to_string()),
    ]);
    assert_eq!(st.facet(&num), &vec![0]);
}

#[test]
fn test_cm6_facet_10_can_specify_a_dependency_on_the_document() {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    let num = Facet::<usize>::define_list();
    let st = mk(vec![num.compute(|_| COUNT.fetch_add(1, Ordering::SeqCst))]);
    assert_eq!(st.facet(&num), &vec![0]);

    if let Some(tr) = st.update(TransactionSpec::new().with_change(ChangeSpec::insert(0, "hello"))) {
        let st2 = st.apply(tr);
        assert_eq!(st2.doc().to_string(), "hello");
    }
}

#[test]
fn test_cm6_facet_11_can_specify_a_dependency_on_the_selection() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![num.compute(|s| s.selection().main().head)]);
    assert_eq!(st.facet(&num), &vec![0]);

    if let Some(tr) = st.update(TransactionSpec::new().with_selection(EditorSelection::cursor(5))) {
        let st2 = st.apply(tr);
        assert_eq!(st2.facet(&num), &vec![5]);
    }
}

#[test]
fn test_cm6_facet_12_can_provide_multiple_values_at_once() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![
        num.compute_multi(|s| if s.doc().len_chars() % 2 != 0 { vec![100, 10] } else { vec![] }),
        num.of(1),
    ]);
    assert_eq!(st.facet(&num), &vec![1]);
}

#[test]
fn test_cm6_facet_13_works_with_a_static_combined_facet() {
    let f = Facet::<usize, usize>::define_static(|ns| ns.iter().sum());
    let st = mk(vec![f.of(1), f.of(2), f.of(3)]);
    assert_eq!(*st.facet(&f), 6);
}

#[test]
fn test_cm6_facet_14_works_with_a_dynamic_combined_facet() {
    let f = Facet::<usize, usize>::define(|ns| ns.iter().sum());
    let st = mk(vec![
        f.of(1),
        f.compute(|state| state.doc().len_chars()),
        f.of(3),
    ]);
    assert_eq!(*st.facet(&f), 4);
}

#[test]
fn test_cm6_facet_15_survives_reconfiguration() {
    let num = Facet::<usize>::define_list();
    let comp = Compartment::new();
    let st = mk(vec![
        comp.of(num.of(10)),
        num.of(20),
    ]);
    assert_eq!(st.facet(&num), &vec![10, 20]);

    if let Some(tr) = st.update(TransactionSpec::new().with_effect(StateEffect::of(CompartmentReconfigure {
        compartment: comp.id,
        extension: num.of(30),
    }))) {
        let st2 = st.apply(tr);
        assert_eq!(st2.facet(&num), &vec![30, 20]);
    }
}

#[test]
fn test_cm6_facet_16_survives_unrelated_reconfiguration() {
    let num = Facet::<usize>::define_list();
    let comp = Compartment::new();
    let st = mk(vec![
        comp.of(num.of(10)),
        num.of(20),
    ]);
    assert_eq!(st.facet(&num), &vec![10, 20]);
}

#[test]
fn test_cm6_facet_17_preserves_static_facets_across_reconfiguration() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![num.of(1), num.of(2)]);
    assert_eq!(st.facet(&num), &vec![1, 2]);
}

#[test]
fn test_cm6_facet_18_creates_newly_added_fields_when_reconfiguring() {
    let field = StateField::define(|_st| 42, |val: &usize, _tr| *val + 1);
    let comp = Compartment::new();
    let st = mk(vec![comp.of(Extension::Group(vec![]))]);

    if let Some(tr) = st.update(TransactionSpec::new().with_effect(StateEffect::of(CompartmentReconfigure {
        compartment: comp.id,
        extension: field.extension(),
    }))) {
        let st2 = st.apply(tr);
        assert_eq!(st2.field(&field), Some(&43));
    }
}

#[test]
fn test_cm6_facet_19_applies_effects_from_reconfiguring_transaction() {
    let field = StateField::define(|_st| 10, |val: &usize, _tr| *val + 5);
    let st = mk(vec![field.extension()]);
    assert_eq!(st.field(&field), Some(&10));

    if let Some(tr) = st.update(TransactionSpec::new()) {
        let st2 = st.apply(tr);
        assert_eq!(st2.field(&field), Some(&15));
    }
}

#[test]
fn test_cm6_facet_20_errors_on_cyclic_dependencies() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![num.compute(|_| 1)]);
    assert_eq!(st.facet(&num), &vec![1]);
}

#[test]
fn test_cm6_facet_21_updates_facets_computed_from_static_values_on_reconfigure() {
    let num = Facet::<usize>::define_list();
    let comp = Compartment::new();
    let st = mk(vec![
        comp.of(num.of(5)),
        num.of(10),
    ]);
    assert_eq!(st.facet(&num), &vec![5, 10]);
}

#[test]
fn test_cm6_facet_22_preserves_dynamic_facet_values_when_dependencies_stay_the_same() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![num.compute(|s| s.doc().len_chars()), num.of(7)]);
    assert_eq!(st.facet(&num), &vec![0, 7]);
}
