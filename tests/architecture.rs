use megastore::{Interaction, MegaStore};

#[test]
fn typed_vertices_can_share_numeric_id() {
    let mut s = MegaStore::new();
    s.add_category(1, "Categoria").unwrap();
    s.add_client(1, "Cliente").unwrap();
    s.add_product(1, "Produto", 1).unwrap();
    assert_eq!(s.counts().0, 3);
}

#[test]
fn multi_path_scores_are_finite_and_deterministic() {
    let mut s = MegaStore::new();
    s.add_category(1, "Grupo").unwrap();
    s.add_client(1, "Cliente").unwrap();
    for id in 1..=5 { s.add_product(id, format!("Produto {id}"), 1).unwrap(); }
    s.record_interaction(1, 1, Interaction::Purchase).unwrap();
    s.record_interaction(1, 2, Interaction::Interest).unwrap();
    s.add_similarity(1, 3, 0.9).unwrap();
    s.add_similarity(2, 3, 0.8).unwrap();
    let first = s.recommend_for_client(1, 10).unwrap();
    let second = s.recommend_for_client(1, 10).unwrap();
    assert_eq!(first, second);
    assert!(first.iter().all(|r| r.score.is_finite() && r.score > 0.0));
    assert!(first.iter().any(|r| r.product_id == 3));
}
