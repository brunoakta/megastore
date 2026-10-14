# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

Projeto acadêmico da disciplina **Data Structure Strategy and Implementation**, desenvolvido em **Rust**, sem dependências externas. Simula a **MegaStore**, uma loja virtual que precisa recomendar itens relacionados ao comportamento dos clientes e às conexões entre produtos.

## 1. Objetivo

O sistema permite cadastrar clientes, categorias e produtos; registrar compras, interesses e avaliações; criar relações de similaridade; e gerar recomendações personalizadas a partir de **um cliente ou um produto**. O grafo utiliza vértices tipados e arestas ponderadas para representar essas relações.

## 2. Pré-requisitos

- [Rust e Cargo](https://www.rust-lang.org/tools/install), versão estável **1.70 ou superior** (edição 2021).
- Terminal (Linux, macOS ou Windows PowerShell).
- Não é necessário banco de dados nem instalação de crates de terceiros.

## 3. Como compilar e executar

Abra um terminal na pasta que contém o `Cargo.toml` e execute:

```bash
cargo build
cargo run -- demo
cargo run -- interactive
cargo test
cargo run --release -- bench
```

**Modos disponíveis**:

| Comando | Função |
| --- | --- |
| `cargo run -- demo` | Exibe catálogo e recomendações com dados fictícios |
| `cargo run -- interactive` | Menu para cadastrar e consultar informações |
| `cargo test` | Roda testes unitários e de integração |
| `cargo run --release -- bench` | Mede construção e 100 consultas para 1.000, 5.000 e 20.000 produtos |
| `cargo run -- help` | Lista os comandos |

Se o terminal estiver fora desta pasta, navegue primeiro para `megastore/`. No PowerShell, use os mesmos comandos.

## 4. Arquitetura do projeto

```text
megastore/
├── Cargo.toml              # Metadados e edição do Rust
├── README.md               # Esta documentação
├── .gitignore
├── docs/
│   └── performance.md      # Procedimento e campo para resultados reais
├── src/
│   ├── main.rs             # CLI: demo, interactive, bench
│   ├── lib.rs              # API pública e dados fictícios
│   ├── model.rs            # Cliente, Produto, Categoria, Vertex, Interaction
│   ├── graph.rs            # Grafo como HashMap<Vertex, Vec<Edge>>
│   ├── catalog.rs          # Cadastros e gerenciamento das relações
│   ├── recommendation.rs   # BFS por camadas e ranking ponderado
│   └── benchmark.rs        # Geração determinística e medição
└── tests/
    ├── integration.rs      # Fluxos funcionais e interface demo
    └── architecture.rs     # IDs tipados e múltiplos caminhos
```

### Estruturas de dados utilizadas

- **Grafo dirigido e ponderado**, implementado por **lista de adjacência**: `HashMap<Vertex, Vec<Edge>>`.
- **`HashMap<u64, Product/Client/Category>`** para consultas cadastrais por ID em tempo médio O(1).
- **`HashSet<u64>`** para registrar itens já associados ao cliente e impedir recomendações de itens conhecidos.
- **`HashMap<Vertex, f64>`** para manter a afinidade acumulada na fronteira de cada camada da busca.
- **`HashMap<u64, f64>`** para somar pontuações sem repetir produtos recomendados.
- **`Vec<Recommendation>`** ordenado por pontuação, com desempate por ID.

### Vértices e relações

| Origem | Destino | Relação | Peso |
| --- | --- | --- | --- |
| Cliente | Produto | Compra | 4,0 |
| Cliente | Produto | Interesse | 2,0 |
| Cliente | Produto | Avaliação de 3, 4 ou 5 | 0,5 / 2,0 / 3,0 |
| Produto | Categoria | Pertence a | 1,0 |
| Categoria | Produto | Contém | 1,0 |
| Produto | Produto | Similaridade | entre 0 e 1 (positiva), informada no cadastro |

Similaridade e associação de categorias aparecem nos **dois sentidos** por meio de **duas arestas dirigidas**. Compra, interesse e avaliação geram apenas arestas do cliente para o produto. Avaliações 1 e 2 registram o histórico, mas **não ativam uma conexão positiva**. Uma nova avaliação substitui a avaliação anterior (inclusive removendo sua aresta positiva, se necessário). A API rejeita pesos inválidos e entidades ausentes.

## 5. Algoritmo de recomendação

1. Iniciar na entidade escolhida, com massa de afinidade igual a `1.0`.
2. Avançar na rede por **camadas de BFS**, agregando as contribuições de **todos os caminhos** que chegam a cada vértice na mesma camada.
3. Em cada expansão, propagar para o vizinho uma fração `massa_atual × 0,85 × peso_da_aresta / soma_dos_pesos_de_saída`.
4. Para recomendações por **cliente**, percorrer até **3 arestas**, considerando produtos encontrados nos níveis **2 e 3**. Para recomendações por **produto**, percorrer até **2 arestas**, considerando níveis **1 e 2**.
5. Descartar produtos com interação prévia do cliente ou o próprio produto de origem. Consolidar cada candidato por ID, somando sua afinidade entre camadas.
6. Ordenar pelo maior score, desempatar pelo menor ID e retornar até o limite solicitado.

O score é **relativo**, não representa uma probabilidade de compra. Os dados são intencionalmente limitados por profundidade para controlar o custo; caminhos podem voltar a vértices anteriores, mas a profundidade é finita. A entrada e os pesos são sempre positivos e finitos.

### Exemplo conceitual

```text
Ana --compra--> Notebook --similar--> Monitor
                         |
                         +--pertence--> Eletrônicos --contém--> Teclado
                         |
                         +--similar--> Suporte para notebook
```

Como Ana também interagiu com o Mouse, **Notebook e Mouse não serão exibidos como novas recomendações**. Os demais produtos são candidatos, conforme pontuação.

## 6. Exemplo de uso

Para reproduzir a demonstração no terminal:

```bash
cargo run -- demo
```

Serão exibidos nove produtos fictícios e recomendações para:

- Ana (cliente `1`), que comprou Notebook (`101`) e demonstrou interesse em Mouse (`102`);
- Notebook (produto `101`), com relacionamentos de categoria e similaridade.

No menu interativo:

```bash
cargo run -- interactive
```

Escolha, por exemplo, a opção `9` e informe o cliente `1`; use a opção `3` para adicionar um produto à categoria existente; registre interações com as opções `5` a `7` e consulte novamente. O menu inicia com a base fictícia, mas **os dados não são persistidos após fechar o programa**.

## 7. Testes unitários e de integração

```bash
cargo test
```

**Abrangência dos testes**: atualização de arestas sem duplicação, natureza direcionada, remoção seletiva, rejeição de categoria inexistente, atualização de avaliações, consulta cadastral, exclusão de produtos já vistos, ausência de candidatos duplicados, ordenação, erro de identificador, pesos e notas inválidos, ausência de histórico, limite zero, IDs iguais com tipos diferentes, determinismo dos resultados e execução do modo `demo` pelo binário.

**Importante**: a existência de testes no repositório não significa que tenham sido executados com sucesso no computador do autor. Rode `cargo test` e registre a saída real antes da entrega.

## 8. Complexidade de tempo e espaço

Denotando `V` o número de vértices, `E` o número de arestas dirigidas, `C` o número de produtos candidatos e `D` a profundidade máxima (2 ou 3):

| Operação | Tempo esperado / limite superior |
| --- | --- |
| Consultar produto por ID | O(1) médio (HashMap) |
| Adicionar vértice | O(1) médio |
| Cadastrar produto e duas arestas de categoria | O(deg(categoria)) para verificar duplicidade na lista de adjacência |
| Criar/atualizar uma aresta | O(deg(origem)) devido à varredura de `Vec<Edge>` |
| Recomendar | O(D × E + C log C), limite conservador |
| Memória do grafo e cadastros | O(V + E) |
| Memória auxiliar de uma consulta | O(V + C) |

Nota: `HashMap` tem acesso médio O(1), não é uma garantia absoluta de pior caso. `D` é fixo neste protótipo. A implementação percorre somente vizinhanças alcançadas a cada camada; na prática, normalmente examina menos arestas do que `E`.

## 9. Desempenho

```bash
cargo run --release -- bench
```

O programa constrói três massas determinísticas (1.000, 5.000 e 20.000 produtos), cria categorias, 50 clientes, duas relações de similaridade dirigidas por par e três interações por cliente. Executa 10 consultas de aquecimento e mede **100 consultas** com `std::time::Instant`.

A saída tem colunas CSV para volumes, número de vértices/arestas, tempo de construção e tempo total/médio das consultas. **Não há resultados de tempo pré-preenchidos**, pois a execução deverá ocorrer na máquina de avaliação. Instruções e espaço para os dados reais: [`docs/performance.md`](docs/performance.md).

## 10. Estratégia de escalabilidade e limitações

A lista de adjacência evita o custo O(V²) de uma matriz para um grafo esparso. O limite de profundidade controla custo e ruído das recomendações; o algoritmo suporta múltiplos caminhos sem criar resultados duplicados. A atualização de listas de vizinhos é linear no grau, logo uma base muito conectada pode exigir `HashMap<Vertex, HashMap<(Vertex, EdgeKind), f64>>`, indexação por partição, cache de scores, paralelismo de consultas e persistência externa.

O protótipo **não é uma plataforma de produção**: todos os dados permanecem em memória; não possui autenticação, API HTTP, banco persistente, ingestão distribuída nem experimento A/B para provar relevância real. A densidade do grafo e a qualidade de sinais também influenciam desempenho e pertinência das recomendações.

## 11. Publicação e vídeo pitch

O projeto deverá ser enviado para um repositório **público no GitHub**. O trabalho também inclui um **relatório técnico em PDF** e um **vídeo de no máximo 4 minutos** com demonstração real, testes e desempenho. Um roteiro de gravação acompanha o pacote de entrega. Atualize os links após publicar:

- GitHub: **[INSERIR LINK PÚBLICO DO REPOSITÓRIO]**
- Vídeo: **[INSERIR LINK PÚBLICO OU NÃO LISTADO]**

### Referências recomendadas

- RUST PROJECT. *The Rust Programming Language*. https://doc.rust-lang.org/book/
- RUST PROJECT. *std::collections::HashMap*. https://doc.rust-lang.org/std/collections/struct.HashMap.html
- RUST PROJECT. *std::collections::HashSet*. https://doc.rust-lang.org/std/collections/struct.HashSet.html
- CORMEN, T. H. et al. *Introduction to Algorithms*. 4. ed. MIT Press, 2022.
