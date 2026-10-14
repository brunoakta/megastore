//! Interface de demonstração, modo interativo e medição de desempenho.

mod benchmark;

use std::io::{self, Write};
use megastore::{demo_store, Interaction, MegaStore, Recommendation};

fn main() {
    let argument = std::env::args().nth(1).unwrap_or_else(|| "demo".to_owned());
    match argument.as_str() {
        "demo" => run_demo(),
        "interactive" => interactive(),
        "bench" => benchmark::run(),
        "help" | "--help" | "-h" => help(),
        _ => { eprintln!("Modo desconhecido: {argument}"); help(); std::process::exit(2); }
    }
}

fn help() {
    println!("ConectaStore - modos de execução:\n  cargo run -- demo\n  cargo run -- interactive\n  cargo run --release -- bench\n  cargo test");
}

fn print_recommendations(items: &[Recommendation]) {
    if items.is_empty() { println!("  Nenhuma recomendação disponível."); return; }
    for (index, item) in items.iter().enumerate() {
        println!("  {}. ID {:>3} | {:<24} | {:<17} | pontuação {:.5}",
            index + 1, item.product_id, item.name, item.category, item.score);
    }
}

fn run_demo() {
    let store = demo_store();
    let (vertices, edges) = store.counts();
    println!("=== ConectaStore: demonstração ===");
    println!("Grafo: {vertices} vértices, {edges} arestas dirigidas.");
    println!("\nCatálogo:");
    for product in store.products_sorted() { println!("  {} - {}", product.id, product.name); }
    println!("\nAna (ID 1) comprou Notebook e demonstrou interesse em Mouse sem fio.");
    println!("Recomendações personalizadas:");
    print_recommendations(&store.recommend_for_client(1, 5).unwrap());
    println!("\nRecomendações para o produto Notebook (ID 101):");
    print_recommendations(&store.recommend_for_product(101, 5).unwrap());
    println!("\nProdutos já associados ao cliente não são recomendados novamente.");
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut line = String::new();
    let bytes = io::stdin().read_line(&mut line)?;
    if bytes == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "fim da entrada"));
    }
    Ok(line.trim().to_string())
}

fn read_id(label: &str) -> Option<u64> {
    let text = read_line(label).ok()?;
    match text.parse::<u64>() {
        Ok(id) => Some(id),
        Err(_) => { println!("ID inválido."); None }
    }
}

fn interactive() {
    let mut store = demo_store();
    println!("=== ConectaStore: modo interativo (dados fictícios pré-carregados) ===");
    loop {
        println!("\n1 Listar produtos | 2 Adicionar categoria | 3 Adicionar produto");
        println!("4 Adicionar cliente | 5 Registrar compra | 6 Registrar interesse");
        println!("7 Registrar avaliação | 8 Relacionar produtos | 9 Recomendar para cliente");
        println!("10 Recomendar por produto | 11 Estatísticas | 0 Sair");
        let option = match read_line("Escolha: ") { Ok(x) => x, Err(_) => break };
        match option.as_str() {
            "0" => break,
            "1" => for p in store.products_sorted() { println!("{} - {} (cat. {})", p.id, p.name, p.category_id); },
            "2" => if let Some(id) = read_id("ID da categoria: ") {
                if let Ok(name) = read_line("Nome: ") { println!("{:?}", store.add_category(id, name)); }
            },
            "3" => if let Some(id) = read_id("ID do produto: ") {
                if let Ok(name) = read_line("Nome: ") {
                    if let Some(cat) = read_id("ID da categoria existente: ") {
                        println!("{:?}", store.add_product(id, name, cat));
                    }
                }
            },
            "4" => if let Some(id) = read_id("ID do cliente: ") {
                if let Ok(name) = read_line("Nome: ") { println!("{:?}", store.add_client(id, name)); }
            },
            "5" | "6" | "7" => if let Some(client) = read_id("ID do cliente: ") {
                if let Some(product) = read_id("ID do produto: ") {
                    let interaction = match option.as_str() {
                        "5" => Some(Interaction::Purchase),
                        "6" => Some(Interaction::Interest),
                        _ => read_id("Avaliação de 1 a 5: ").and_then(|r| u8::try_from(r).ok()).map(Interaction::Rating),
                    };
                    if let Some(interaction) = interaction {
                        println!("{:?}", store.record_interaction(client, product, interaction));
                    } else { println!("Avaliação inválida."); }
                }
            },
            "8" => if let Some(a) = read_id("Produto A: ") {
                if let Some(b) = read_id("Produto B: ") {
                    match read_line("Similaridade (ex.: 0.8): ").ok().and_then(|x| x.parse().ok()) {
                        Some(w) => println!("{:?}", store.add_similarity(a, b, w)),
                        None => println!("Peso inválido."),
                    }
                }
            },
            "9" => if let Some(id) = read_id("ID do cliente: ") {
                match store.recommend_for_client(id, 10) {
                    Ok(items) => print_recommendations(&items), Err(e) => println!("Erro: {e}"),
                }
            },
            "10" => if let Some(id) = read_id("ID do produto: ") {
                match store.recommend_for_product(id, 10) {
                    Ok(items) => print_recommendations(&items), Err(e) => println!("Erro: {e}"),
                }
            },
            "11" => { let (v, e) = store.counts(); println!("Vértices: {v}; arestas: {e}"); },
            _ => println!("Opção desconhecida."),
        }
    }
}
