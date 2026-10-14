//! Catálogo, cadastro de entidades e gestão das conexões.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use crate::graph::Graph;
use crate::model::{Category, Client, EdgeKind, Interaction, Product, Vertex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    Duplicate(&'static str, u64),
    NotFound(&'static str, u64),
    InvalidRating(u8),
    InvalidWeight,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::Duplicate(kind, id) => write!(f, "{kind} de ID {id} já cadastrado"),
            StoreError::NotFound(kind, id) => write!(f, "{kind} de ID {id} não encontrado"),
            StoreError::InvalidRating(rating) => write!(f, "nota {rating} inválida: use 1 a 5"),
            StoreError::InvalidWeight => write!(f, "similaridade deve estar em (0, 1] e ser finita"),
        }
    }
}
impl Error for StoreError {}

#[derive(Debug, Default)]
pub struct MegaStore {
    pub(crate) graph: Graph,
    pub(crate) products: HashMap<u64, Product>,
    pub(crate) clients: HashMap<u64, Client>,
    pub(crate) categories: HashMap<u64, Category>,
    pub(crate) interacted: HashMap<u64, HashSet<u64>>,
}

impl MegaStore {
    pub fn new() -> Self { Self::default() }

    pub fn add_category(&mut self, id: u64, name: impl Into<String>) -> Result<(), StoreError> {
        if self.categories.contains_key(&id) { return Err(StoreError::Duplicate("Categoria", id)); }
        self.categories.insert(id, Category { id, name: name.into() });
        self.graph.add_vertex(Vertex::Category(id));
        Ok(())
    }

    pub fn add_client(&mut self, id: u64, name: impl Into<String>) -> Result<(), StoreError> {
        if self.clients.contains_key(&id) { return Err(StoreError::Duplicate("Cliente", id)); }
        self.clients.insert(id, Client { id, name: name.into() });
        self.graph.add_vertex(Vertex::Client(id));
        Ok(())
    }

    pub fn add_product(&mut self, id: u64, name: impl Into<String>, category_id: u64) -> Result<(), StoreError> {
        if self.products.contains_key(&id) { return Err(StoreError::Duplicate("Produto", id)); }
        if !self.categories.contains_key(&category_id) { return Err(StoreError::NotFound("Categoria", category_id)); }
        self.products.insert(id, Product { id, name: name.into(), category_id });
        let product = Vertex::Product(id);
        let category = Vertex::Category(category_id);
        self.graph.add_vertex(product);
        self.graph.upsert_edge(product, category, EdgeKind::BelongsTo, 1.0);
        self.graph.upsert_edge(category, product, EdgeKind::Contains, 1.0);
        Ok(())
    }

    pub fn product(&self, id: u64) -> Option<&Product> { self.products.get(&id) }
    pub fn products_sorted(&self) -> Vec<&Product> {
        let mut result: Vec<_> = self.products.values().collect();
        result.sort_by_key(|product| product.id);
        result
    }

    pub fn add_similarity(&mut self, a: u64, b: u64, weight: f64) -> Result<(), StoreError> {
        if !self.products.contains_key(&a) { return Err(StoreError::NotFound("Produto", a)); }
        if !self.products.contains_key(&b) { return Err(StoreError::NotFound("Produto", b)); }
        if a == b || !weight.is_finite() || !(0.0 < weight && weight <= 1.0) {
            return Err(StoreError::InvalidWeight);
        }
        self.graph.upsert_edge(Vertex::Product(a), Vertex::Product(b), EdgeKind::Similarity, weight);
        self.graph.upsert_edge(Vertex::Product(b), Vertex::Product(a), EdgeKind::Similarity, weight);
        Ok(())
    }

    pub fn record_interaction(&mut self, client_id: u64, product_id: u64, interaction: Interaction) -> Result<(), StoreError> {
        if !self.clients.contains_key(&client_id) { return Err(StoreError::NotFound("Cliente", client_id)); }
        if !self.products.contains_key(&product_id) { return Err(StoreError::NotFound("Produto", product_id)); }
        let (kind, weight) = match interaction {
            Interaction::Purchase => (EdgeKind::Purchase, Some(4.0)),
            Interaction::Interest => (EdgeKind::Interest, Some(2.0)),
            Interaction::Rating(rating) if !(1..=5).contains(&rating) => {
                return Err(StoreError::InvalidRating(rating));
            }
            Interaction::Rating(1 | 2) => (EdgeKind::Rating, None),
            Interaction::Rating(3) => (EdgeKind::Rating, Some(0.5)),
            Interaction::Rating(4) => (EdgeKind::Rating, Some(2.0)),
            Interaction::Rating(5) => (EdgeKind::Rating, Some(3.0)),
            Interaction::Rating(_) => unreachable!(),
        };
        self.interacted.entry(client_id).or_default().insert(product_id);
        let from = Vertex::Client(client_id);
        let to = Vertex::Product(product_id);
        match weight {
            Some(value) => self.graph.upsert_edge(from, to, kind, value),
            None => self.graph.remove_edge(from, to, kind),
        }
        Ok(())
    }

    pub fn counts(&self) -> (usize, usize) {
        (self.graph.vertex_count(), self.graph.edge_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_category_does_not_insert_product() {
        let mut store = MegaStore::new();
        assert_eq!(store.add_product(1, "Mouse", 999), Err(StoreError::NotFound("Categoria", 999)));
        assert!(store.product(1).is_none());
    }

    #[test]
    fn rating_update_replaces_only_rating_edge() {
        let mut store = MegaStore::new();
        store.add_category(1, "A").unwrap();
        store.add_client(1, "C").unwrap();
        store.add_product(1, "P", 1).unwrap();
        store.record_interaction(1, 1, Interaction::Purchase).unwrap();
        store.record_interaction(1, 1, Interaction::Rating(5)).unwrap();
        store.record_interaction(1, 1, Interaction::Rating(1)).unwrap();
        let edges = store.graph.neighbors(Vertex::Client(1));
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].kind, EdgeKind::Purchase);
    }
}
