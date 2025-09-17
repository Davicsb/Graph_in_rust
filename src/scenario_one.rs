use crate::graph::read_graph;
use crate::floydwarshall::floyd;

pub fn first_scenario(){
    let gr = match read_graph("data/graph1.txt") {
        Ok(graph_sucesso) => {
            println!("Grafo lido com sucesso do arquivo!\n");
            graph_sucesso // Se der certo, `gr` recebe o valor do grafo
        },
        Err(e) => {
            eprintln!("Erro fatal ao ler o grafo: {}\n", e);
            return; // Sai do programa se não conseguir ler o arquivo
        }
    };

    let matriz = floyd(&gr);
    let num_v = gr.num_vertex();
    let inf = i32::MAX/2;
    for i in 0..num_v{
        print!("{} -> ", i + 1);
        for j in 0..num_v{
            if matriz[i][j] != inf{
                print!("{} ", matriz[i][j]);
            }
            else{
                print!("inf ");
            }
        }
        print!("\n");
    }
}