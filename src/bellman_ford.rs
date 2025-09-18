use crate::graph::Graph;

pub fn bellman_ford(grafo: &Graph, origem: &usize) -> (Vec<i32>, Vec<Option<usize>>) { //retorna as distancias e anteriores pros caminhos
    let num_v = grafo.num_vertex() + 1;

    // Linha 1: Cria um vetor de distâncias, inicializando todos com INFINITO. Em seguida a distancia com a origem recebe zero
    //é criado tambem o vetor anterior
    let mut d = vec![i32::MAX/2; num_v];
    d[*origem] = 0;
    let mut anterior = vec![None; num_v];

    //Linha 2: Inicio do loop, enquanto existir uma aresta (j,i) no grafo tal que d[i] > d[j] + vij (peso) fazer
    let mut mudou = true;
    while mudou {
        mudou = false;

        for(j, lista_vizinhos) in &grafo.adj {
            let mut head = lista_vizinhos.as_ref();

            while let Some(node) = head {
                let i = node.value;
                let v = node.weight;

                if d[i] > d[*j] + v{
                    //Linha 3: d[i] recebe d[j] + vij e anterior[i] recebe j
                    d[i] = d[*j] + v;
                    anterior[i] = Some(*j);
                    mudou = true;
                }

                head = node.next.as_ref();
            }
        }
    }

    (d, anterior)
}