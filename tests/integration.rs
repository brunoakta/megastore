use megastore::{demo_store, Interaction, MegaStore, StoreError};
use std::collections::HashSet;
use std::process::Command;

#[test]
fn retrieves_registered_products() {
    let store = demo_store();
    assert_eq!(store.product(101).unwrap().name, "Notebook");
    assert!(store.product(123_456).is_none());
}

#[test]
fn recommendation_excludes_interacted_products_and_is_unique() {
    let store = demo_store();
    let recs = store.recommend_for_client(1, 20).unwrap();
    assert!(!recs.is_empty());
    assert!(!recs.iter().any(|r| r.product_id == 101 || r.product_id == 102));
    let ids: HashSet<_> = recs.iter().map(|r| r.product_id).collect();
    assert_eq!(ids.len(), recs.len());
}

#[test]
fn product_recommendations_exclude_origin_and_are_ranked() {
    let store = demo_store();
    let recs = store.recommend_for_product(101, 30).unwrap();
    assert!(recs.iter().any(|r| r.product_id == 104));
    assert!(recs.iter().any(|r| r.product_id == 109));
    assert!(!recs.iter().any(|r| r.product_id == 101));
    assert!(recs.windows(2).all(|pair| pair[0].score >= pair[1].score));
}

#[test]
fn missing_entity_returns_error() {
    let store = demo_store();
    assert_eq!(store.recommend_for_client(999, 10), Err(StoreError::NotFound("Cliente", 999)));
    assert_eq!(store.recommend_for_product(999, 10), Err(StoreError::NotFound("Produto", 999)));
}

#[test]
fn invalid_similarity_and_rating_are_rejected() {
    let mut store = demo_store();
    assert_eq!(store.add_similarity(101, 101, 0.5), Err(StoreError::InvalidWeight));
    assert_eq!(store.add_similarity(101, 102, f64::NAN), Err(StoreError::InvalidWeight));
    assert_eq!(store.record_interaction(1, 101, Interaction::Rating(0)), Err(StoreError::InvalidRating(0)));
}

#[test]
fn low_rating_does_not_create_positive_recommendation_paths() {
    let mut store = MegaStore::new();
    store.add_category(1, "Testes").unwrap();
    store.add_client(1, "Cliente").unwrap();
    store.add_product(1, "Produto A", 1).unwrap();
    store.add_product(2, "Produto B", 1).unwrap();
    store.record_interaction(1, 1, Interaction::Rating(1)).unwrap();
    assert!(store.recommend_for_client(1, 10).unwrap().is_empty());
    store.record_interaction(1, 1, Interaction::Rating(5)).unwrap();
    assert_eq!(store.recommend_for_client(1, 10).unwrap()[0].product_id, 2);
}

#[test]
fn limit_zero_and_empty_links_are_handled() {
    let mut store = MegaStore::new();
    store.add_client(1, "Sem histórico").unwrap();
    assert!(store.recommend_for_client(1, 10).unwrap().is_empty());
    assert!(store.recommend_for_client(1, 0).unwrap().is_empty());
}

#[test]
fn updating_similarity_does_not_duplicate_edges() {
    let mut store = demo_store();
    let (_, before) = store.counts();
    store.add_similarity(101, 104, 0.4).unwrap();
    assert_eq!(store.counts().1, before);
}

#[test]
fn demo_cli_executes_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_megastore"))
        .arg("demo").output().expect("executar binário");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Recomendações personalizadas"));
}
