//! # Algoritimo Bellman Ford
//! ## Conteúdo:
//! Associação com o livro (bellman_ford())

pub use crate::graph::Graph;

/// # Algoritmo e associação com o livro.
/// ## Argumentos
/// 
/// "grafo" - O grafo analisado;
/// "origem" - Vértice origem do caminho.
///
/// ## Retorno
/// 
/// Vetor de custos para os demais vértices;\
/// Vetor de anteriores, para a reconstruçao do caminho.
/// 
/// ## Associação com livro
/// ```rust
/// let num_v = grafo.num_vertex();
///
///    // Linha 1: Cria um vetor de distâncias, inicializando todos com INFINITO. Em seguida a distancia com a origem recebe zero
///    //é criado tambem o vetor anterior
///    let mut d = vec![i32::MAX/2; num_v + 1];
///    d[*origem] = 0;
///    let mut anterior = vec![None; num_v + 1];
///
///    //Linha 2: Inicio do loop, enquanto existir uma aresta (j,i) no grafo tal que d[i] > d[j] + vij (peso) fazer
///    let mut mudou = true;
///    let mut iteracoes = 0;
///    while mudou && (iteracoes < num_v) {
///        mudou = false;
///
///        for(j, lista_vizinhos) in &grafo.adj {
///            let mut head = lista_vizinhos.as_ref();
///
///            while let Some(node) = head {
///                let i = node.value;
///                let v = node.weight;
///
///                if d[i] > d[*j] + v{
///                    //Checagem pra ver se o grafo tem ciclo negativo
///                    if iteracoes == num_v - 1{ //Na iteração V (lembre que iteracoes começou com 0), se houver outra mudança é porque o grafo tem ciclo negativo
///                        panic!("There is a negative cycle in the graph.");
///                    }
///
///                    //Linha 3: d[i] recebe d[j] + vij e anterior[i] recebe j
///                    d[i] = d[*j] + v;
///                    anterior[i] = Some(*j);
///                    mudou = true;
///                }
///
///                head = node.next.as_ref();
///            }
///        }
///        iteracoes += 1;
///    }
///
///    (d, anterior) // Retorno
/// ```
pub fn bellman_ford(grafo: &Graph, origem: &usize) -> (Vec<i32>, Vec<Option<usize>>) { //retorna as distancias e anteriores pros caminhos
    let num_v = grafo.num_vertex();

    // Linha 1: Cria um vetor de distâncias, inicializando todos com INFINITO. Em seguida a distancia com a origem recebe zero
    //é criado tambem o vetor anterior
    let mut d = vec![i32::MAX/2; num_v + 1];
    d[*origem] = 0;
    let mut anterior = vec![None; num_v + 1];

    //Linha 2: Inicio do loop, enquanto existir uma aresta (j,i) no grafo tal que d[i] > d[j] + vij (peso) fazer
    let mut mudou = true;
    let mut iteracoes = 0;
    while mudou && (iteracoes < num_v) {
        mudou = false;

        for(j, lista_vizinhos) in &grafo.adj {
            let mut head = lista_vizinhos.as_ref();

            while let Some(node) = head {
                let i = node.value;
                let v = node.weight;

                if d[i] > d[*j] + v{
                    //Checagem pra ver se o grafo tem ciclo negativo
                    if iteracoes == num_v - 1{ //Na iteração V (lembre que iteracoes começou com 0), se houver outra mudança é porque o grafo tem ciclo negativo
                        panic!("There is a negative cycle in the graph.");
                    }

                    //Linha 3: d[i] recebe d[j] + vij e anterior[i] recebe j
                    d[i] = d[*j] + v;
                    anterior[i] = Some(*j);
                    mudou = true;
                }

                head = node.next.as_ref();
            }
        }
        iteracoes += 1;
    }

    (d, anterior) // Retorno
}
