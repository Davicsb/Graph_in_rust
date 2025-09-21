//! # Cenário 3: Robô de armazém com obstáculos
//! ## Descrição
//! Um robô de inventário precisa ir do ponto de recarga (S) até a docking station de coleta (G) dentro de um armazém. Há estantes (obstáculos), corredores livres e trechos de piso difícil (ex.: área com pallets baixos) com custo maior. A ideia é propor um mecanismo de busca em grafo que permita encontrar o caminho de menor custo do S ao G, desviando de obstáculos e preferindo corredores “baratos”.
//! ## Output esperado
//! Movimento em 4-direções (N,S,L,O) da célula de começo 'S' para a célula destivo 'G'.
//! ## Algortimo utilizado
//! Dijkstra.
//! ### Motivação
//! Como cada célula do grid é um vértice com até 4 arestas, temos um grafo com um grande volume de vértices e arestas, por isso usamos o algoritmo mais eficiente dos 3 apresentados, tendo a garantia que todas as arestas são positivas.

pub use crate::graph::read_map;
pub use crate::graph::map_to_txt;
pub use crate::graph::read_graph;
pub use crate::dijkstra::dijikstra;
pub use crate::dijkstra::reconstruir_caminho;

/// # Transforma um par de coordenadas da matriz no n° do nó especifico do grafo
/// ```rust
///    let v = x + (y * coluumns);
/// ```
pub fn achar_node(coords: &(usize, usize), coluumns: usize) -> usize {
    let x = coords.0;
    let y = coords.1;
    let v = x + (y * coluumns);

    v
}

/// # Transforma um vetor de caminho em um vetor de string com direções
pub fn caminho_coord(caminho: &Vec<usize>) -> Vec<&str>{
    let mut coords = vec![];

    for par in caminho.windows(2) {
        let i = par[0];
        let j = par[1];
        let res = j as isize - i as isize;

        match res {
            1 => coords.push("E"),  //Leste
            -1 => coords.push("W"), //Oeste
            15 => coords.push("S"), //Sul
            -15 => coords.push("N"),//Norte
            _ => coords.push("?")   // case default
        };
    }

    coords
}

/// # Função de chamada do primeiro cenário
/// ## Mudando o grafo
/// Para mudar o grafo lido basta alterar o caminho presente na seguinte função:
/// ```rust
///    if let Ok((matrix, start, goal)) = read_map("data/grid_example.txt")
/// ```
/// ## Chamada do Dijkstra
/// ```rust
///    let (distancias, anteriores) = dijikstra(&gr, &s_node);
/// ```
/// ## Outputs
/// ```rust
///    let caminho = reconstruir_caminho(s_node, g_node, &anteriores);
///    let coords = caminho_coord(&caminho);
/// 
///    println!("The path from S to G is: {:?}", caminho);
///    println!("That is, the directions will be: {:?}", coords);
///    println!("The cost (sum of all the weights) of the path will be: {}", distancias[g_node]);
/// ```
pub fn third_scenario(){
    let mut s_node = 0;
    let mut g_node = 0;

    if let Ok((matrix, start, goal)) = read_map("data/grid_example.txt") {
        println!("Map read successfully!");
        println!("Start: {:?}, Goal: {:?}", start, goal);

        s_node = achar_node(&start, 15);
        g_node = achar_node(&goal, 15);
        //println!("{} {}", s_node, g_node);

        if let Err(e) = map_to_txt(&matrix) {
            eprintln!("Error writing graph file: {}", e);
        }

    } else {
        eprintln!("Failed to read map from file.");
    }

    let gr = match read_graph("data/graph3.txt") {
        Ok(graph_sucesso) => {
            println!("Graph successfully read from file!\n");
            graph_sucesso // Se der certo, `gr` recebe o valor do grafo
        },
        Err(e) => {
            eprintln!("Fatal error reading graph: {}\n", e);
            return; // Sai do programa se não conseguir ler o arquivo
        }
    };

    let (distancias, anteriores) = dijikstra(&gr, &s_node);
    let caminho = reconstruir_caminho(s_node, g_node, &anteriores);
    let coords = caminho_coord(&caminho);

    println!("The path from S to G is: {:?}", caminho);
    println!("That is, the directions will be: {:?}", coords);
    println!("The cost (sum of all the weights) of the path will be: {}", distancias[g_node]);
}