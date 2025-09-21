//! # Concentra a lógica da construção do grafo, o método escolhido foi representação com lista de adjacência, para uma melhor eficiência de memória e para criar familiaridade com a estrutra de HashMap do Rust.

/// # Responsável a coleção de HashMap para a representação do grafo.

pub use std::collections::HashMap;
/// # Responável pela possibilidade de leituras de arquivo txt no código em todos cenários.

pub use std::fs::File;

/// # Responsável pela possibilidade de escrita de arquivo txt presente no Cenário 3.

pub use std::io::{BufRead, BufReader, BufWriter, Write};
pub use std::fs;
pub use std::io;
pub use std::error::Error;

/// # Reprenta o nó do grafo.
/// ## Atributos
/// "value" - Valor do vértice (sempre representado por um inteiro positivo);\
/// "weight" - Valor do peso da aresta (podendo ser negativo ou positivo);\
/// "next" - Inclui uma referência para o próximo nó.

#[derive(Debug)]
pub struct Node {
    pub value: usize,
    pub weight: i32,
    pub next: Option<Box<Node>>,
}

impl Node {

    /// # Função para crianção de um novo nó.
    
    pub fn new(value: usize, weight: i32) -> Self {
        Node {value, weight, next: None}
    }

    /// # Função para incluir um nó na lista.

    pub fn append(&mut self, value: usize, weight: i32) {
        match &mut self.next {
            Some(next_node) => next_node.append(value, weight),
            None => self.next = Some(Box::new(Node::new(value, weight))),
        }
    }

    /// # Função para printar o nó;

    pub fn print(&self) {
        print!("{} (weight: {})", self.value, self.weight);
        if let Some(next) = &self.next {
            print!(" -> ");
            next.print();
        }
    }
}

/// # Reprenta o grafo inteiro.
/// ## Atributos
/// "num_vertex" - Valor da quatidade de vértices do grafo (sempre representado por um inteiro positivo);\
/// "num_edges" - Valor da quantidade de arestas do grafo (sempre repesentado por um inteiro positivo);\
/// "adj" - HashMap dos vértices do grafo.

#[derive(Debug)]
pub struct Graph {
    pub num_vertex: usize,
    pub num_edges: usize,
    pub adj: HashMap<usize, Option<Box<Node>>>,
}

impl Graph {

    /// # Função que retorna o número de vérfices do grafo

    pub fn num_vertex(&self) -> usize {
        self.num_vertex
    }

    /// # Função que cria um novo grafo

    pub fn new(num_vertex: usize, num_edges: usize) -> Self {
        Graph {num_vertex, num_edges, adj: HashMap::new()}
    }

    /// # Função para adicionar novas arestas no grafo

    pub fn edge(&mut self, origin: usize, destination: usize, destination_weight: i32) {
        // Use `entry` para garantir que a origem tem um valor válido
        let head = self.adj.entry(origin).or_insert(None); //pega a lista de vizinhos de origin ou None
        match head { //adiciona destination como vizinho final
            Some(node) => node.append(destination, destination_weight),
            None => *head = Some(Box::new(Node::new(destination, destination_weight))),
        }
    }

    /// # Função para printar o grafo

    pub fn print(&self) {
        for (v, list) in &self.adj {
            print!("{} -> ", v);
            if let Some(node) = list {
                node.print();
            }
            println!();
        }
    }

    /// # Retorna os vétices em uma lista
    
    pub fn vertices_list(&self) -> Vec<usize> {
        (0..=self.num_vertex).collect() //coleta e retorna o vetor (intervalo fechado) <- REVER
    }

    /// # Retorna uma lsita de vizinhos de um vertice
    
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

    /// # Retorna o vizinho mais próximo do vértice
    
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

    /// # Retorna o peso de uma aresta
    
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

/// # Função que lê o grafo a partir de um arquivo txt.
/// ## Argumentos
/// 
/// "path" - A string do caminho do txt do grafo na formatação:\
/// <num_vertices> <num_arestas>\
/// <vertice_inicial> <vertice_final> <custo> <- Repetição para cada aresta
///
/// ## Retorno
/// 
/// O grafo completo com um HashMap ou o erro associado a criação.

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

    print!("Graph read:\n");
    graph.print();
    Ok(graph)
}

/// # Função que lê o mapa do "Cenário 3" a partir de um arquivo txt.
/// ## Argumentos
/// 
/// "path" - A string do caminho do txt do mapa.
///
/// ## Retorno
/// 
/// Uma matriz de char igual ao mapa;\
/// Uma tupla representando as coordenadas da matriz (0 based) do vértice de início do algoritmo ('S');\
/// Uma tupla representando as coordenadas da matriz (0 based) do vértice de fim do algoritmo ('G');

pub fn read_map(path: &str) -> Result<(Vec<Vec<char>>, (usize, usize), (usize, usize)), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let first_line = lines.next().ok_or("The file is empty.")??;
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

    let start_coords = start.ok_or("Start character 'S' not found in the map.")?;
    let goal_coords = goal.ok_or("Goal character 'G' not found in the map.")?;
    
    Ok((matrix, start_coords, goal_coords))
}

/// # Cria uma matriz auxiliar para os vértices do mapa do "Cenário 3" a partir do número de linhas e colunas.
/// ## Exemplo 1. rows = 2, cols = 3
/// ### Output
/// 1 2 3\
/// 4 5 6
/// # Exemplo 2. rows = 2, cols = 2
/// ### Output
/// 1 2\
/// 3 4

pub fn create_ordered_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
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

/// # Relaciona o caracter do mapa com o peso correspondido.
/// ## Caracteres e saídas esperadas
/// '.' - Existe aresta de peso 1;\
/// '~' - Existe aresta de peso 3;\
/// 'S' | 'G' - Existe aresta de peso 1;\
/// '#' - Não existe aresta (obstáculo intransponível).

pub fn get_peso(terreno: char) -> Option<i32> {
    match terreno {
        '.' => Some(1),
        '~' => Some(3),
        'S' | 'G' => Some(1),
        '#' => None,
        _ => Some(1),
    }
}

/// # Função que tranforma o mapa do "Cenário 3" para o txt com a formatação convencional vistos nos outros cenários.
/// ## Argumentos
/// 
/// "matrix" - A matriz de char do mapa lido.
///
/// ## Retorno
/// 
/// O arquito txt na formatação padrão

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