use crate::graph::read_map;
use crate::graph::map_to_txt;

pub fn third_scenario(){
    if let Ok((matrix, start, goal)) = read_map("data/grid_example.txt") {
        println!("Mapa lido com sucesso!");
        println!("Início: {:?}, Fim: {:?}", start, goal);
        if let Err(e) = map_to_txt(&matrix) {
            eprintln!("Erro ao escrever o arquivo de grafo: {}", e);
        }

    } else {
        eprintln!("Falha ao ler o mapa do arquivo.");
    }
}