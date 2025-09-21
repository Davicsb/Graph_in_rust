use std::io;
use std::fs;

use crate::bellman_ford::bellman_ford;
use crate::dijkstra::reconstruir_caminho;
use crate::graph::Graph;

pub fn read_graph_slope(path: &str) -> Result<Graph, io::Error>{ // talvez seja melhor só usar a função read_graph com um parâmetro a mais
    let content = fs::read_to_string(path)?;

    let numbers: Vec<i32> = content
        .split_whitespace()  // Retorna um iterador de fatias de string (&str)
        .filter_map(|palavra| palavra.parse::<i32>().ok()) // Tenta o parse e filtra os erros
        .collect();          // Coleta todos os números válidos em um vetor

    let num_vertices = numbers[0] as usize;
    let num_edges = numbers[1] as usize;
    let mut graph = Graph::new(num_vertices, num_edges);

    let arestas_data = &numbers[2..];

    for aresta_chunk in arestas_data.chunks(3) {
        let origem = (aresta_chunk[0] - 1) as usize;
        let destino = (aresta_chunk[1] - 1) as usize;
        let peso = aresta_chunk[2]; // O peso já é i32
        graph.edge(origem, destino, peso);
        //o grafo é pra ser direcionado e uma rampa, portanto: (i,j) = x ^ (j,i) = -x
        graph.edge(destino, origem, peso * -1);
    }

    print!("Grafo lido:\n");
    graph.print();
    Ok(graph)
}

pub fn second_scenario(){
    let gr = match read_graph_slope("data/graph1.txt") {
        Ok(graph_sucesso) => {
            println!("Grafo lido com sucesso do arquivo!\n");
            graph_sucesso // Se der certo, `gr` recebe o valor do grafo
        },
        Err(e) => {
            eprintln!("Erro fatal ao ler o grafo: {}\n", e);
            return; // Sai do programa se não conseguir ler o arquivo
        }
    };

    let (distancias, anteriores) = bellman_ford(&gr, &0);
    let caminho = reconstruir_caminho(0, 6, &anteriores);

    gr.print();
    println!("O caminho para do vértice {} para o {} é: {:?}", 0, 6, caminho);
    println!("O custo total da viagem é: {:?}", distancias[6]);
}