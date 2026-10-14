# Protocolo de medição de desempenho — ConectaStore

## Como executar

No diretório `megastore/` com Rust/Cargo 1.70+ instalado:

```bash
cargo test
cargo run --release -- bench
```

Anote versão e condições:

- Data e horário: ____________________
- Processador e memória RAM: ____________________
- Sistema operacional: ____________________
- Versão `rustc --version`: ____________________
- Versão `cargo --version`: ____________________
- Comando utilizado: `cargo run --release -- bench`
- Resultado de `cargo test`: ____________________

O programa usa dados sintéticos determinísticos, **sem rede e sem biblioteca externa**. Para cada cenário com `N` produtos, cadastra `max(1,N/25)` categorias e 50 clientes; liga produtos consecutivos por similaridade recíproca e registra três interações por cliente. Realiza 10 consultas de aquecimento, seguidas de 100 consultas temporizadas. Mede a construção e as 100 consultas com `Instant`.

## Tabela a preencher com resultados reais

| Produtos | Vértices | Arestas dirigidas | Construção (ms) | 100 consultas (ms) | Média por consulta (ms) |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1.000 | preencher | preencher | preencher | preencher | preencher |
| 5.000 | preencher | preencher | preencher | preencher | preencher |
| 20.000 | preencher | preencher | preencher | preencher | preencher |

**Não foram inseridos tempos fictícios.** Cole abaixo a saída exata de sua máquina:

```text
[COLE A SAÍDA AQUI]
```

## Como interpretar

Compare a evolução do tempo de construção com o tamanho do catálogo e o tempo médio de consulta. O tempo não cresce necessariamente de modo idêntico à contagem de produtos: depende das conexões alcançáveis em cada camada, do número de candidatos e da topologia. O grau médio e a profundidade fixa impactam mais diretamente a consulta do que o número absoluto de produtos que permanecem desconectados da região visitada.

Para comparar máquinas, repita cada cenário diversas vezes e use mediana e intervalo, registrando a configuração. O cronômetro é apenas uma medição básica solicitada pela disciplina; não fornece testes estatísticos nem perfis de memória.

## Registro de validação

- [ ] `cargo build` concluiu sem erro
- [ ] `cargo test` concluiu sem falhas
- [ ] `cargo run -- demo` mostrou recomendações
- [ ] `cargo run -- interactive` permitiu uma nova interação
- [ ] `cargo run --release -- bench` produziu as três linhas esperadas
- [ ] Tempos medidos incluídos no relatório e no vídeo
