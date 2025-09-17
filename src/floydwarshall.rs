use crate::graph::Graph; 

pub fn floyd(gr : &Graph) -> Vec<Vec<i32>>{

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

    cost
}