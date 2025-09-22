//! # Cenário 1: Determinando a estação central
//! ## Descrição
//! Considere um grafo não-direcionado com pesos que representam pontos e conexões de metrô. Precisamos definir qual seria a estação central, ou o vértice central do grafo. O vértice central é o que consegue chegar a qualquer um dos outros vértices com o menor custo. Isso leva em conta tanto o somatório das distâncias do vértice em questão a cada um dos demais vértices.
//! ## Output esperado
//! O nó que representa a estação central escolhida;\
//! Um vetor com as distâncias da estação central até os demais vértices;\
//! O vértice mais distante da estação central, junto com o valor de distância;\
//! Uma matriz em que cada linha representa um vértice candidato à estação central e cada coluna é a distância mínima entre o vértice candidato e o vértice representante da coluna.
//! ## Algortimo utilizado
//! Floyd Warshall.
//! ### Motivação
//! Apesar de não ser o mais eficiênte, o principal motivo foi comodidade, o algoritmo já faz o retorno de todos os outputs esperados no cenário.

pub use crate::graph::read_graph;
pub use crate::floydwarshall::floyd_rot_n_cost;

/// # Define a estação central
/// A estação estral é aquela em que a soma dos pesos dos caminhos a partir dela é a menor de todos os outros vértices.
pub fn def_central_station(matrix : &Vec<Vec<i32>>, num_v : usize) -> usize{
    let inf = i32::MAX/2;
    let mut lowest_sum = i32::MAX;
    let mut c_station = 0;
    for i in 0..num_v{
        let mut sum = 0;
        for j in 0..num_v{
            if matrix[i][j] == inf{
                sum = i32::MAX;
                break;
            }
            sum = sum + matrix[i][j]; 
        }
        if sum < lowest_sum{
            lowest_sum = sum;
            c_station = i;
        }
    }

    c_station+1
}

/// # Retorna o vetor da dos custos da estação central para as outras estações, qual a estação mais distante e qual o custo da estação central para essa estação.
pub fn station_vector(matrix: &[Vec<i32>], station: usize, num_v: usize) -> (Vec<i32>, usize, i32) {
    let mut ev: Vec<i32> = Vec::with_capacity(num_v); 
    
    let mut farthest_station = 0;
    let mut farthest_distance = i32::MIN;
    
    for i in 0..num_v {
        let distancia_atual = matrix[station - 1][i];
        ev.push(distancia_atual); 
        if distancia_atual > farthest_distance {
            farthest_distance = distancia_atual;
            farthest_station = i + 1;
        }
    }

    (ev, farthest_station, farthest_distance)
}

/// # Printa a matriz de distandias de todas as estações para todas estações.
pub fn print_matrix(matrix : &Vec<Vec<i32>>, num_v : usize){
    let inf = i32::MAX/2;
    for i in 0..num_v{
        print!("{} -> ", i + 1);
        for j in 0..num_v{
            if matrix[i][j] != inf{
                print!("{} ", matrix[i][j]);
            }
            else{
                print!("# ");
            }
        }
        print!("\n");
    }
}

/// # Impreime um vetor.
pub fn print_vector(vector : &Vec<i32>){
    print!("[");
    for i in 0..vector.len(){
        print!("{}", vector[i]);
        if i < vector.len()-1{
            print!(", ");
        }
    }
    print!("]\n");
}

/// # Função de chamada do primeiro cenário
/// ## Mudando o grafo
/// Para mudar o grafo lido basta alterar o caminho presente na seguinte função:
/// ```rust
///    let gr = match read_graph("data/graph1.txt")
/// ```
/// ## Chamada do Floyd Wharshall
/// ```rust
///    let (matrix, matrix_rot) = floyd_rot_n_cost(&gr);
/// ```
/// ## Outputs
/// ```rust
///    let central_station = def_central_station(&matrix, num_v);
///    let (central_station_vector, farthest_station, farthest_distance) = station_vector(&matrix, central_station, num_v);
///    print!("The node that represents the choosen central station: {}\n", central_station);
///    print!("\n");
///    print!("A vector with the distances from the central station to the other vertices:");
///    print_vector(&central_station_vector);
///    print!("\n");
///    print!("The vertex furthest from the central station, along with the distance value: {}, d = {}\n", farthest_station, farthest_distance);
///    print!("\n");
///    print!("Matrix in which each row represents a candidate vertex for the central station and each column is the minimum distance between the candidate vertex and the column's representative vertex:\n");
///    print_matrix(&matrix, num_v);
///    print!("\n");
/// ```
pub fn first_scenario(){
    let gr = match read_graph("data/graph1.txt") {
        Ok(graph_sucesso) => {
            graph_sucesso.print();
            println!("Graph successfully read from file!\n");
            graph_sucesso // Se der certo, `gr` recebe o valor do grafo
        },
        Err(e) => {
            eprintln!("Fatal error reading graph: {}\n", e);
            return; // Sai do programa se não conseguir ler o arquivo
        }
    };

    let (matrix, matrix_rot) = floyd_rot_n_cost(&gr);
    let num_v = gr.num_vertex();
    let central_station = def_central_station(&matrix, num_v);
    let (central_station_vector, farthest_station, farthest_distance) = station_vector(&matrix, central_station, num_v);


    print!("The node that represents the choosen central station: {}\n", central_station);
    print!("\n");
    print!("A vector with the distances from the central station to the other vertices:");
    print_vector(&central_station_vector);
    print!("\n");
    print!("The vertex furthest from the central station, along with the distance value: {}, d = {}\n", farthest_station, farthest_distance);
    print!("\n");
    print!("Matrix in which each row represents a candidate vertex for the central station and each column is the minimum distance between the candidate vertex and the column's representative vertex:\n");
    print_matrix(&matrix, num_v);
    print!("\n");

}