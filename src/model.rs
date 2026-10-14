//! Entidades de domínio e identificadores utilizados no grafo.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub category_id: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Client {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Category {
    pub id: u64,
    pub name: String,
}

/// As variantes evitam colisões entre IDs iguais de tipos diferentes.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Vertex {
    Client(u64),
    Product(u64),
    Category(u64),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum EdgeKind {
    Purchase,
    Interest,
    Rating,
    BelongsTo,
    Contains,
    Similarity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interaction {
    Purchase,
    Interest,
    Rating(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Recommendation {
    pub product_id: u64,
    pub name: String,
    pub category: String,
    /// Pontuação de propagação; serve para ordenação relativa, não é probabilidade.
    pub score: f64,
}
