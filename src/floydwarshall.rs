//! # Algoritimo Floyd Warshall
//! ## Conteúdo:
//! Função de backtracking (reconstruir_caminho());\
//! Associação com o livro (floyd_rot_n_cost())

pub use crate::graph::Graph;

/// # Função de backtracking para reconstrução do caminho
pub fn reconstruir_caminho(rot: &Vec<Vec<Option<usize>>>, origem: usize, destino: usize) -> Vec<usize> {
    let mut caminho = vec![origem];
    let mut head = origem;

    while head != destino {
        if let Some(next) = rot[head][destino] {
            head = next;
            caminho.push(head);
        } else {
            return vec![];
        }
    }

    caminho
}

/// # Algoritmo e associação com o livro.
/// ## Argumentos
/// 
/// "gr" - O grafo analisado.
///
/// ## Retorno
/// 
/// Matriz de custo de todos os vértices para todos os vértices;\
/// Matriz de rotemento, de todos os vértices oara todos os vértices, para a reconstruçao do caminho.
/// 
/// ## Associação com livro
/// ```rust
/// let inf = i32::MAX/2;
///    let num_v = gr.num_vertex();
///    let mut rot = vec![vec![None; num_v]; num_v]; // Matriz de roteamento Rij
///    let mut cost = vec![vec![inf; num_v]; num_v]; // matriz de custo D^0
///
///    for i in 0..num_v{
///        rot[i][i] = Some(i); // Caminho dele para ele mesmo é ele
///        cost[i][i] = 0; // Custo do vétice para ele mesmo é 0
///    }
/// 
///    for (origin, adj_list) in &gr.adj // Pega o par de origem e os nós ligados a origem
///    {
///        if let Some(mut node) = adj_list.as_deref() // Pega a lista de nós
///        {
///            loop{
///                rot[*origin][node.value] = Some(node.value); // Rij <- j
///                cost[*origin][node.value] = node.weight; // Dij <- V(G)
///
///                if let Some(next_node) = node.next.as_deref(){
///                    node = next_node; // Chama próximo nó
///                }
///                
///                else{
///                    break; // Não existe mais nós
///                }
///            }
///        }
///    }
///
///    for k in 0..num_v{ // Para k = 0 ... n
///        for i in 0..num_v{ // Para i = 0 ... n
///            for j in 0..num_v{ // Para j = 0 ... n
///                if (cost[i][k] + cost[k][j]) < cost[i][j] // if Dik + Dkj < Dij
///                {
///                    cost[i][j] = cost[i][k] + cost[k][j]; // Dij <- Dik + Dkj
///                    rot[i][j] = rot[i][k]; // Rij <- Rik
///                }
///            }
///        }
///    }
///
///    (cost, rot) // Retorno
/// ```
pub fn floyd_rot_n_cost(gr : &Graph) -> (Vec<Vec<i32>>, Vec<Vec<Option<usize>>>){ //retorna as duas matrizes

    let inf = i32::MAX/2;
    let num_v = gr.num_vertex();
    let mut rot = vec![vec![None; num_v]; num_v]; // Matriz de roteamento Rij
    let mut cost = vec![vec![inf; num_v]; num_v]; // matriz de custo D^0

    for i in 0..num_v{
        rot[i][i] = Some(i); // Caminho dele para ele mesmo é ele
        cost[i][i] = 0; // Custo do vétice para ele mesmo é 0
    }

    for (origin, adj_list) in &gr.adj // Pega o par de origem e os nós ligados a origem
    {
        if let Some(mut node) = adj_list.as_deref() // Pega a lista de nós
        {
            loop{
                rot[*origin][node.value] = Some(node.value); // Rij <- j
                cost[*origin][node.value] = node.weight; // Dij <- V(G)

                if let Some(next_node) = node.next.as_deref(){
                    node = next_node; // Chama próximo nó
                }
                
                else{
                    break; // Não existe mais nós
                }
            }
        }
    }

    for k in 0..num_v{ // Para k = 0 ... n
        for i in 0..num_v{ // Para i = 0 ... n
            for j in 0..num_v{ // Para j = 0 ... n
                if (cost[i][k] + cost[k][j]) < cost[i][j] // if Dik + Dkj < Dij
                {
                    cost[i][j] = cost[i][k] + cost[k][j]; // Dij <- Dik + Dkj
                    rot[i][j] = rot[i][k]; // Rij <- Rik
                }
            }
        }
    }

    (cost, rot) // Retorno
}