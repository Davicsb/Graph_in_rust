//! # Descrição
//! Nesta atividade, vamos explorar o conceito de caminho mínimo utilizando algoritmos como Djkistra, Bellman-Ford, Floyd. A atividade pode ser feita individual, dupla ou trio. Vamos explorar três cenários de aplicação, duas em que um grafo já oferecido como entrada e outra em que você precisa montar um grafo a partir de um mapa/grid.

pub mod graph;
pub mod dijkstra;
pub mod bellman_ford;
pub mod floydwarshall;
pub mod scenario_one;
pub mod scenario_two;
pub mod scenario_three;

use std::io;

/// # Esse módulo traz consigo o primeiro cenário usando o algoritmo Floyd Warshall;
/// A função principal tem como objetivo printar na tela os outputs requeritos no projeto "Cenário 1";
/// O presente código usa como exemplo de grafo arquivo fornecido na documentação do projeto "graph1.txt" presente na pasta data. Para mudar o grafo é preciso trocar os parâmetros da função no próprio arquivo e adicioná-lo na pasta data.

pub use crate::scenario_one::first_scenario;

/// # Esse módulo traz consigo o primeiro cenário usando o algoritmo XXX;
/// A função principal tem como objetivo printar na tela os outputs requeritos no projeto "Cenário 2";
/// O presente código usa como exemplo de grafo o arquivo fornecido na documentação do projeto "graph2.txt" presente na pasta data. Para mudar o grafo é preciso trocar os parâmetros da função no próprio arquivo e adicioná-lo na pasta data.

pub use crate::scenario_two::second_scenario;

/// # Esse módulo traz consigo o primeiro cenário usando o algoritmo Dijsttra;
/// A função principal tem como objetivo printar na tela os outputs requeritos no projeto "Cenário 2";
/// O presente código usa como exemplo de grafo o arquivo fornecido na documentação do projeto "grid_example.txt" presente na pasta data. Para mudar o grafo é preciso trocar os parâmetros da função no próprio arquivo e adicioná-lo na pasta data.
/// Para o tratamento dos dados, a função cria um novo arquivo txt "graph3.txt" que é a representação do grafo como vimos nos outros dois cenários, para não mudarmos como muda a leitura do arquivo.

pub use crate::scenario_three::third_scenario;

/// # A função main orquestra qual cenário será visto, para rodar o Cenário 1 digite no CMD "1" e assim por diante.

pub fn main() {
    loop {println!("Choose a scenario:\n0 - Exit\n1 - Determining the central station\n2 - Optimizing path with regeneration\n3 - Warehouse robot with obstacles");
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line.");
    let num : i32 = input_line.trim().parse().expect("The input is not an integer.");
    //tudo isso pra ler um int...

    match num {
        0 => break,
        1 => first_scenario(),
        2 => second_scenario(),
        3 => third_scenario(),
        _ => println!("Cenário inválido.")
    };}
}