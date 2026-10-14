//! BFS em camadas com propagação ponderada de afinidade.
//! Cada nó da camada propaga sua massa proporcionalmente aos pesos das arestas.

use std::collections::HashMap;
use crate::catalog::{MegaStore, StoreError};
use crate::model::{Recommendation, Vertex};

const DECAY: f64 = 0.85;

impl MegaStore {
    pub fn recommend_for_client(&self, client_id: u64, limit: usize) -> Result<Vec<Recommendation>, StoreError> {
        if !self.clients.contains_key(&client_id) { return Err(StoreError::NotFound("Cliente", client_id)); }
        Ok(self.rank(Vertex::Client(client_id), 3, 2, limit))
    }

    pub fn recommend_for_product(&self, product_id: u64, limit: usize) -> Result<Vec<Recommendation>, StoreError> {
        if !self.products.contains_key(&product_id) { return Err(StoreError::NotFound("Produto", product_id)); }
        Ok(self.rank(Vertex::Product(product_id), 2, 1, limit))
    }

    /// Complexidade máxima por consulta: O(D*E + C log C), memória O(V+C).
    /// D é a profundidade (2 ou 3), E são as arestas e C os candidatos distintos.
    /// O percurso usa a mesma ideia de uma BFS, mas agrega contribuições por camada,
    /// mesmo quando vários caminhos chegam ao mesmo produto.
    fn rank(&self, seed: Vertex, max_depth: usize, min_depth: usize, limit: usize) -> Vec<Recommendation> {
        if limit == 0 { return Vec::new(); }
        let mut frontier = HashMap::from([(seed, 1.0_f64)]);
        let mut scores: HashMap<u64, f64> = HashMap::new();

        for depth in 1..=max_depth {
            let mut next: HashMap<Vertex, f64> = HashMap::new();
            // Ordenar a camada evita diferenças de arredondamento por ordem aleatória
            // de iteração do HashMap, tornando a classificação reproduzível.
            let mut layer: Vec<_> = frontier.into_iter().collect();
            layer.sort_by_key(|(vertex, _)| *vertex);
            for (vertex, mass) in layer {
                let edges = self.graph.neighbors(vertex);
                let sum: f64 = edges.iter().map(|edge| edge.weight).sum();
                if sum <= 0.0 { continue; }
                for edge in edges {
                    let propagated = mass * DECAY * edge.weight / sum;
                    *next.entry(edge.to).or_insert(0.0) += propagated;
                }
            }
            if depth >= min_depth {
                for (vertex, mass) in &next {
                    if let Vertex::Product(id) = *vertex {
                        let excluded = match seed {
                            Vertex::Product(origin_id) => id == origin_id,
                            Vertex::Client(client_id) => self.interacted
                                .get(&client_id).is_some_and(|seen| seen.contains(&id)),
                            Vertex::Category(_) => false,
                        };
                        if !excluded && mass.is_finite() && *mass > 0.0 {
                            *scores.entry(id).or_insert(0.0) += mass;
                        }
                    }
                }
            }
            if next.is_empty() { break; }
            frontier = next;
        }

        let mut recommendations: Vec<_> = scores.into_iter().filter_map(|(id, score)| {
            let product = self.products.get(&id)?;
            let category = self.categories.get(&product.category_id)?;
            Some(Recommendation {
                product_id: id,
                name: product.name.clone(),
                category: category.name.clone(),
                score,
            })
        }).collect();
        recommendations.sort_by(|a, b| {
            b.score.total_cmp(&a.score).then(a.product_id.cmp(&b.product_id))
        });
        recommendations.truncate(limit);
        recommendations
    }
}
