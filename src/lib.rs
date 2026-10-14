//! ConectaStore - biblioteca do recomendador da MegaStore.
//! Não utiliza dependências externas; compatível com Rust estável (1.70+).

pub mod catalog;
pub mod graph;
pub mod model;
mod recommendation;

pub use catalog::{MegaStore, StoreError};
pub use model::{Interaction, Recommendation};

/// Dados fictícios pequenos para apresentação, testes e vídeo pitch.
pub fn demo_store() -> MegaStore {
    let mut store = MegaStore::new();
    store.add_category(1, "Eletrônicos").unwrap();
    store.add_category(2, "Livros").unwrap();
    store.add_category(3, "Casa e escritório").unwrap();
    store.add_client(1, "Ana").unwrap();
    store.add_client(2, "Carlos").unwrap();
    for (id, name, category) in [
        (101, "Notebook", 1),
        (102, "Mouse sem fio", 1),
        (103, "Teclado mecânico", 1),
        (104, "Monitor", 1),
        (105, "Livro de Rust", 2),
        (106, "Livro de algoritmos", 2),
        (107, "Luminária", 3),
        (108, "Cadeira ergonômica", 3),
        (109, "Suporte para notebook", 3),
    ] {
        store.add_product(id, name, category).unwrap();
    }
    store.add_similarity(101, 104, 0.9).unwrap();
    store.add_similarity(101, 109, 0.85).unwrap();
    store.add_similarity(102, 103, 0.8).unwrap();
    store.add_similarity(105, 106, 0.95).unwrap();
    store.record_interaction(1, 101, Interaction::Purchase).unwrap();
    store.record_interaction(1, 102, Interaction::Interest).unwrap();
    store.record_interaction(2, 105, Interaction::Purchase).unwrap();
    store
}
