//! Grafo dirigido e ponderado implementado por listas de adjacência.

use std::collections::HashMap;
use crate::model::{EdgeKind, Vertex};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub to: Vertex,
    pub kind: EdgeKind,
    pub weight: f64,
}

#[derive(Debug, Default)]
pub struct Graph {
    adjacency: HashMap<Vertex, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self { Self::default() }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.adjacency.entry(vertex).or_default();
    }

    /// Insere/atualiza uma única aresta por (origem, destino, tipo).
    /// Peso deve ser positivo e finito; entidades são validadas em MegaStore.
    pub fn upsert_edge(&mut self, from: Vertex, to: Vertex, kind: EdgeKind, weight: f64) {
        assert!(weight.is_finite() && weight > 0.0, "peso inválido");
        let edges = self.adjacency.entry(from).or_default();
        if let Some(edge) = edges.iter_mut().find(|e| e.to == to && e.kind == kind) {
            edge.weight = weight;
        } else {
            edges.push(Edge { to, kind, weight });
        }
        self.adjacency.entry(to).or_default();
    }

    pub fn remove_edge(&mut self, from: Vertex, to: Vertex, kind: EdgeKind) {
        if let Some(edges) = self.adjacency.get_mut(&from) {
            edges.retain(|edge| !(edge.to == to && edge.kind == kind));
        }
    }

    pub fn neighbors(&self, vertex: Vertex) -> &[Edge] {
        self.adjacency.get(&vertex).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn vertex_count(&self) -> usize { self.adjacency.len() }

    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(Vec::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_does_not_duplicate_same_edge_type() {
        let mut graph = Graph::new();
        graph.upsert_edge(Vertex::Product(1), Vertex::Product(2), EdgeKind::Similarity, 0.4);
        graph.upsert_edge(Vertex::Product(1), Vertex::Product(2), EdgeKind::Similarity, 0.9);
        assert_eq!(graph.edge_count(), 1);
        assert!((graph.neighbors(Vertex::Product(1))[0].weight - 0.9).abs() < 1e-12);
    }

    #[test]
    fn edges_are_directed() {
        let mut graph = Graph::new();
        graph.upsert_edge(Vertex::Client(1), Vertex::Product(1), EdgeKind::Purchase, 4.0);
        assert!(graph.neighbors(Vertex::Product(1)).is_empty());
    }

    #[test]
    fn removal_preserves_other_relationships() {
        let mut graph = Graph::new();
        let a = Vertex::Client(1);
        let b = Vertex::Product(2);
        graph.upsert_edge(a, b, EdgeKind::Rating, 3.0);
        graph.upsert_edge(a, b, EdgeKind::Purchase, 4.0);
        graph.remove_edge(a, b, EdgeKind::Rating);
        assert_eq!(graph.neighbors(a).len(), 1);
        assert_eq!(graph.neighbors(a)[0].kind, EdgeKind::Purchase);
    }
}
