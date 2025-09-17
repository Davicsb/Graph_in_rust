use crate::graph::read_graph;
use crate::floydwarshall::floyd;

fn central_estation(matriz : Vec<Vec<i32>>, num_v : usize) -> usize{
    let inf = i32::MAX/2;
    let mut lowest_sum = i32::MAX;
    let mut c_estation = 0;
    for i in 0..num_v{
        let mut sum = 0;
        for j in 0..num_v{
            if matriz[i][j] == inf{
                sum = i32::MAX;
                break;
            }
            sum = sum + matriz[i][j]; 
        }
        if sum < lowest_sum{
            lowest_sum = sum;
            c_estation = i;
        }
    }

    c_estation+1
}

fn print_matriz(matriz : Vec<Vec<i32>>, num_v : usize){
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

    print!("A estação central é: {}\n", central_estation(matriz.clone(), num_v));
    print!("\n");
    print_matriz(matriz, num_v);
    

}