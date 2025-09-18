use crate::graph::read_graph;
use crate::floydwarshall::floyd;

fn def_central_station(matriz : &Vec<Vec<i32>>, num_v : usize) -> usize{
    let inf = i32::MAX/2;
    let mut lowest_sum = i32::MAX;
    let mut c_station = 0;
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
            c_station = i;
        }
    }

    c_station+1
}

fn station_vector(matriz: &[Vec<i32>], station: usize, num_v: usize) -> (Vec<i32>, usize, i32) {
    let mut ev: Vec<i32> = Vec::with_capacity(num_v); 
    
    let mut farthest_station = 0;
    let mut farthest_distance = i32::MIN;
    
    for i in 0..num_v {
        let distancia_atual = matriz[station - 1][i];
        ev.push(distancia_atual); 
        if distancia_atual > farthest_distance {
            farthest_distance = distancia_atual;
            farthest_station = i + 1;
        }
    }

    (ev, farthest_station, farthest_distance)
}

fn print_matriz(matriz : &Vec<Vec<i32>>, num_v : usize){
    let inf = i32::MAX/2;
    for i in 0..num_v{
        print!("{} -> ", i + 1);
        for j in 0..num_v{
            if matriz[i][j] != inf{
                print!("{} ", matriz[i][j]);
            }
            else{
                print!("# ");
            }
        }
        print!("\n");
    }
}

fn print_vector(vector : &Vec<i32>){
    print!("[");
    for i in 0..vector.len(){
        print!("{}", vector[i]);
        if i < vector.len()-1{
            print!(", ");
        }
    }
    print!("]\n");
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
    let central_station = def_central_station(&matriz, num_v);
    let (central_station_vector, farthest_station, farthest_distance) = station_vector(&matriz, central_station, num_v);


    print!("O nó que representa a estação central escolhida: {}\n", central_station);
    print!("\n");
    print!("Um vetor com as distâncias da estação central até os demais vértices:");
    print_vector(&central_station_vector);
    print!("\n");
    print!("O vértice mais distante da estação central, junto com o valor de distância: {}, d = {}\n", farthest_station, farthest_distance);
    print!("\n");
    print!("Matriz em que cada linha representa um vértice candidato à estação central e cada coluna é a distância mínima entre o vértice candidato e o vértice representante da coluna:\n");
    print_matriz(&matriz, num_v);
    print!("\n");
    

}