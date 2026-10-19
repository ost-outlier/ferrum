use crate::facet::{Extension, Facet, Prec};
use crate::state::{EditorState, EditorStateConfig};
use std::sync::Arc;

fn mk(extensions: Vec<Extension>) -> Arc<EditorState> {
    EditorState::create(EditorStateConfig {
        extensions: Some(extensions),
        ..Default::default()
    })
}

#[test]
fn test_cm6_facet_allows_querying_of_facets() {
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
fn test_cm6_facet_returns_an_empty_array_for_absent_facet() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![]);
    assert_eq!(st.facet(&num), &Vec::<usize>::new());
}

#[test]
fn test_cm6_facet_sorts_extensions_by_priority() {
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
fn test_cm6_facet_supports_dynamic_facet() {
    let num = Facet::<usize>::define_list();
    let st = mk(vec![
        num.of(1),
        num.compute(|_state| 88),
    ]);
    assert_eq!(st.facet(&num), &vec![1, 88]);
}

#[test]
fn test_cm6_facet_works_with_a_static_combined_facet() {
    let f = Facet::<usize, usize>::define_static(|ns| ns.iter().sum());
    let st = mk(vec![f.of(1), f.of(2), f.of(3)]);
    assert_eq!(*st.facet(&f), 6);
}

#[test]
fn test_cm6_facet_works_with_a_dynamic_combined_facet() {
    let f = Facet::<usize, usize>::define(|ns| ns.iter().sum());
    let st = mk(vec![
        f.of(1),
        f.compute(|state| state.doc().len_chars()),
        f.of(3),
    ]);
    assert_eq!(*st.facet(&f), 4); // doc initially empty, sum = 1 + 0 + 3 = 4
}
