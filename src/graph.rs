use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Node {
    pub value: usize,
    pub weight: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: usize, weight: i32) -> Self {
        Node {value, weight, next: None}
    }

    pub fn append(&mut self, value: usize, weight: i32) {
        match &mut self.next {
            Some(next_node) => next_node.append(value, weight),
            None => self.next = Some(Box::new(Node::new(value, weight))),
        }
    }

    pub fn print(&self) {
        print!("{} (weight: {})", self.value, self.weight);
        if let Some(next) = &self.next {
            print!(" -> ");
            next.print();
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    num_vertex: usize,
    num_edges: usize,
    pub adj: HashMap<usize, Option<Box<Node>>>,
}

impl Graph {
    pub fn num_vertex(&self) -> usize {
        self.num_vertex
    }

    pub fn new(num_vertex: usize, num_edges: usize) -> Self {
        Graph {num_vertex, num_edges, adj: HashMap::new()}
    }

    pub fn edge(&mut self, origin: usize, destination: usize, destination_weight: i32) {
        // Use `entry` para garantir que a origem tem um valor válido
        let head = self.adj.entry(origin).or_insert(None);
        match head {
            Some(node) => node.append(destination, destination_weight),
            None => *head = Some(Box::new(Node::new(destination, destination_weight))),
        }
    }

    pub fn print(&self) {
        for (v, list) in &self.adj {
            print!("{} -> ", v);
            if let Some(node) = list {
                node.print();
            }
            println!();
        }
    }
}

pub fn read_graph(path: &str) -> Result<Graph, io::Error>{
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
    }

    print!("Grafo lido:\n");
    graph.print();
    Ok(graph)
}