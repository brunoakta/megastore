//! Medição reproduzível (não substitui um benchmark estatístico como Criterion).

use std::time::{Duration, Instant};
use megastore::{Interaction, MegaStore};

fn dataset(product_count: usize) -> MegaStore {
    let mut store = MegaStore::new();
    let n_categories = (product_count / 25).max(1);
    for id in 1..=n_categories {
        store.add_category(id as u64, format!("Categoria {id}")).unwrap();
    }
    for id in 1..=product_count {
        store.add_product(id as u64, format!("Produto {id}"), ((id - 1) % n_categories + 1) as u64).unwrap();
    }
    for id in 1..product_count {
        store.add_similarity(id as u64, (id + 1) as u64, 0.75).unwrap();
    }
    for client_id in 1..=50_u64 {
        store.add_client(client_id, format!("Cliente {client_id}")).unwrap();
        for k in 0..3_u64 {
            let product_id = (client_id * 13 + k * 47) % product_count as u64 + 1;
            let interaction = if k == 0 { Interaction::Purchase } else { Interaction::Interest };
            store.record_interaction(client_id, product_id, interaction).unwrap();
        }
    }
    store
}

fn ms(d: Duration) -> f64 { d.as_secs_f64() * 1_000.0 }

pub fn run() {
    println!("BENCHMARK (Rust std::time::Instant, build release recomendado)");
    println!("produtos,categorias,vertices,arestas,construcao_ms,100_consultas_ms,media_consulta_ms");
    for n in [1_000_usize, 5_000, 20_000] {
        let start_build = Instant::now();
        let store = dataset(n);
        let build = start_build.elapsed();
        // Aquecimento: fora da janela de medição.
        for id in 1..=10 { std::hint::black_box(store.recommend_for_client(id, 10).unwrap()); }
        let start_queries = Instant::now();
        for i in 0..100 {
            let client_id = (i % 50 + 1) as u64;
            std::hint::black_box(store.recommend_for_client(client_id, 10).unwrap());
        }
        let queries = start_queries.elapsed();
        let (v, e) = store.counts();
        println!("{n},{},{v},{e},{:.3},{:.3},{:.5}", (n / 25).max(1), ms(build), ms(queries), ms(queries) / 100.0);
    }
    println!("AVISO: resultados variam conforme hardware, otimização e carga do sistema.");
}
