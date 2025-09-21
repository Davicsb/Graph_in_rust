//! # Cenário 2: Otimizando caminho com regeneração
//! ## Descrição
//! Considere um carro elétrico com regeneração de bateria eficiente por freio-motor. O carro deve ir da origem ao destino minimizando a energia líquida da bateria (Wh). Trechos subida/arranque consomem energia (peso positivo). Trechos descida/frenagem regenerativa devolvem energia à bateria (peso negativo).
//! ## Output esperado
//! O nó que representa a estação central escolhida;\
//! O caminho mínimo, saindo sempre do vértice 0 até o vértice 6;\
//! Somatório do custo do caminho.
//! ## Algortimo utilizado
//! Bellman Ford.
//! ### Motivação
//! O principal motivo da escolha foram as arestas negativas que o Dijkstra não suporta, e não consumir tanta memória como o Floyd Warshall. O grafo também não apresenta ciclos negativos, possibilitando o uso do algoritmo.

pub use std::io;
pub use std::fs;

pub use crate::graph::read_graph;
pub use crate::bellman_ford::bellman_ford;
pub use crate::dijkstra::reconstruir_caminho;
pub use crate::graph::Graph;

/// # Função de chamada do primeiro cenário
/// ## Mudando o grafo
/// Para mudar o grafo lido basta alterar o caminho presente na seguinte função:
/// ```rust
///    let gr = match read_graph("data/graph2.txt")
/// ```
/// ## Chamada do Bellman Ford
/// ```rust
///    let (distancias, anteriores) = bellman_ford(&gr, &0);
/// ```
/// ## Outputs
/// ```rust
///    let caminho = reconstruir_caminho(0, 6, &anteriores);
///    println!("The path from vertex {} to {} is: {:?}", 0, 6, caminho);
//     println!("The total cost of the trip is: {:?}", distancias[6]);
/// ```
pub fn second_scenario(){
    let gr = match read_graph("data/graph2.txt") {
        Ok(graph_sucesso) => {
            graph_sucesso.print();
            println!("Graph successfully read from file!\n");
            graph_sucesso // Se der certo, `gr` recebe o valor do grafo
        },
        Err(e) => {
            eprintln!("Fatal error reading graph: {}\n", e);
            return; // Sai do programa se não conseguir ler o arquivo
        }
    };

    let (distancias, anteriores) = bellman_ford(&gr, &0);
    let caminho = reconstruir_caminho(0, 6, &anteriores);

    println!("The path from vertex {} to {} is: {:?}", 0, 6, caminho);
    println!("The total cost of the trip is: {:?}", distancias[6]);
}