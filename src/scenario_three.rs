use crate::graph::read_map;
use crate::graph::map_to_txt;
use crate::graph::read_graph;
use crate::dijkstra::dijikstra;
use crate::dijkstra::reconstruir_caminho;

//Transforma um par de coordenadas no n° do node especifico do grafo
pub fn achar_node(coords: &(usize, usize), coluumns: usize) -> usize {
    let x = coords.0;
    let y = coords.1;
    let v = x + (y * coluumns);

    v
}

//Transforma um vetor de caminho em um vetor de string com direções
pub fn caminho_coord(caminho: &Vec<usize>) -> Vec<&str>{
    let mut coords = vec![];

    for par in caminho.windows(2) {
        let i = par[0];
        let j = par[1];
        let res = j as isize - i as isize;

        match res {
            1 => coords.push("L"),  //Leste
            -1 => coords.push("O"), //Oeste
            15 => coords.push("S"), //Sul
            -15 => coords.push("N"),//Norte
            _ => coords.push("?")   // case default
        };
    }

    coords
}

pub fn third_scenario(){
    let mut s_node = 0;
    let mut g_node = 0;

    if let Ok((matrix, start, goal)) = read_map("data/grid_example.txt") {
        println!("Mapa lido com sucesso!");
        println!("Início: {:?}, Fim: {:?}", start, goal);

        s_node = achar_node(&start, 15);
        g_node = achar_node(&goal, 15);
        //println!("{} {}", s_node, g_node);

        if let Err(e) = map_to_txt(&matrix) {
            eprintln!("Erro ao escrever o arquivo de grafo: {}", e);
        }

    } else {
        eprintln!("Falha ao ler o mapa do arquivo.");
    }

    let gr = match read_graph("data/graph3.txt") {
        Ok(graph_sucesso) => {
            println!("Grafo lido com sucesso do arquivo!\n");
            graph_sucesso // Se der certo, `gr` recebe o valor do grafo
        },
        Err(e) => {
            eprintln!("Erro fatal ao ler o grafo: {}\n", e);
            return; // Sai do programa se não conseguir ler o arquivo
        }
    };

    let (distancias, anteriores) = dijikstra(&gr, &s_node);
    let caminho = reconstruir_caminho(s_node, g_node, &anteriores);
    let coords = caminho_coord(&caminho);

    println!("O caminho de S até G é: {:?}", caminho);
    println!("Ou seja, as direções serão: {:?}", coords);
    println!("O custo do caminho será: {}", distancias[g_node]);
}