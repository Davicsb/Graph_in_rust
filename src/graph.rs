use std::collections::HashMap;
use std::fs;
use std::io;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

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
        let head = self.adj.entry(origin).or_insert(None); //pega a lista de vizinhos de origin ou None
        match head { //adiciona destination como vizinho final
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

    //retorna os vetores em uma lista
    pub fn vertices_list(&self) -> Vec<usize> {
        (0..=self.num_vertex).collect() //coleta e retorna o vetor (intervalo fechado) <- REVER
    }

    //Retorna uma lsita de vizinhos de um vertice
    pub fn vizinhos(&self, vertice: &usize) -> Vec<usize> {
        let mut vizinhos = vec![];

        let mut head = match self.adj.get(vertice) {
            Some(node) => node.as_ref(),
            None => return vizinhos, // retorna vetor vazio se não houver vizinhos
        };

        while let Some(node) = head {
            vizinhos.push(node.value);
            head = node.next.as_ref();
        }

        vizinhos
    }

    //Retorna o vizinho mais próximo
    pub fn vizinho_mais_perto(&self, vertice: &usize) -> Option<usize> {
        let mut distancia_minima = i32::MAX;
        let mut escolhido = None;

        let mut head = self.adj.get(vertice)?.as_ref(); // pega a lista encadeada dos vizinhos do vértice

        while let Some(node) = head {
            if node.weight < distancia_minima {
                distancia_minima = node.weight;
                escolhido = Some(node.value);
            }
            head = node.next.as_ref();
        }

        escolhido
    }

    //Retorna o peso de uma aresta
    pub fn weight(&self, origem: &usize, destino: &usize) -> Option<i32>{

        let mut head = match self.adj.get(origem) {
            Some(node) => node.as_ref(),
            None => return None,
        };

        while let Some(node) = head {
            if node.value == *destino {
                return Some(node.weight);
            }
            head = node.next.as_ref();
        }

        None
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

pub fn read_map(path: &str) -> Result<(Vec<Vec<char>>, (usize, usize), (usize, usize)), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let first_line = lines.next().ok_or("Arquivo está vazio")??;
    let dims: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    let mut goal: Option<(usize, usize)> = None;

    for (y, line_result) in lines.enumerate() {
        let line = line_result?;
        let mut char_vec: Vec<char> = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = Some((x, y));
            }
            if ch == 'G' {
                goal = Some((x, y));
            }
            char_vec.push(ch);
        }
        matrix.push(char_vec);
    }

    let start_coords = start.ok_or("Caractere de início 'S' não foi encontrado no mapa.")?;
    let goal_coords = goal.ok_or("Caractere de fim 'G' não foi encontrado no mapa.")?;
    
    Ok((matrix, start_coords, goal_coords))
}

fn create_ordered_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut matrix = Vec::with_capacity(rows);

    let mut counter = 1;

    for _ in 0..rows {
        let mut current_row = Vec::with_capacity(cols);
        for _ in 0..cols {
            current_row.push(counter);
            counter += 1;
        }
        matrix.push(current_row);
    }

    matrix
}


fn get_peso(terreno: char) -> Option<i32> {
    match terreno {
        '.' => Some(1),
        '~' => Some(3),
        'S' | 'G' => Some(1),
        '#' => None,
        _ => Some(1),
    }
}

pub fn map_to_txt(matrix: &[Vec<char>]) -> io::Result<()> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let num_vertex = rows * cols;
    let ordered_matrix = create_ordered_matrix(rows, cols);

    let mut arestas: Vec<String> = Vec::new();
    for y in 0..rows {
        for x in 0..cols {
            let no_atual_id = ordered_matrix[y][x];

            let vizinhos = [(y.wrapping_sub(1), x), (y + 1, x), (y, x.wrapping_sub(1)), (y, x + 1)];

            for (viz_y, viz_x) in vizinhos {
                if viz_y < rows && viz_x < cols {
                    let terreno_vizinho = matrix[viz_y][viz_x];
                    if let Some(peso) = get_peso(terreno_vizinho) {
                        let no_vizinho_id = ordered_matrix[viz_y][viz_x];
                        let aresta_str = format!("{}   {}   {}", no_atual_id, no_vizinho_id, peso);
                        arestas.push(aresta_str);
                    }
                }
            }
        }
    }

    let path = "data/graph3.txt";
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{} {}", num_vertex, arestas.len())?;

    for aresta in arestas {
        writeln!(writer, "{}", aresta)?;
    }

    Ok(())
}