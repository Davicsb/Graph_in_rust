//! # Algoritimo Dijkstra
//! ## Conteúdo:
//! Função de vértice mais próximo (vertice_mais_proximo());\
//! Função do vetor de interseção (intersecao());\
//! Função de backtracking (reconstruir_caminho());\
//! Associação com o livro (dijikstra())

pub use crate::graph::Graph; 
pub use std::cmp;

/// # Retorna a cópia do vértice mais próximo.
pub fn vertice_mais_proximo(distancias: &Vec<i32>, A: &Vec<usize>) -> Option<usize>{
    let mut distancia_minima = i32::MAX;
    let mut escolhido = None;

    for v in A{
        if distancias[*v] < distancia_minima{
            distancia_minima = distancias[*v];
            escolhido = Some(v);
        }
    }

    escolhido.copied()
}

/// # Retorna um vetor com a interseção dos vértices em A e os vizinhos de um certo vértice.
pub fn intersecao(A: &Vec<usize>, grafo: &Graph, vertice: &usize) -> Vec<usize> { // O(n) tem como deixar O(1) transformando A em hashset
    let mut intersec = grafo.vizinhos(vertice);
    intersec.retain(|x| A.contains(x));

    intersec
}

/// # Retorna um vetor com o caminho pra o destino <- acho melhor botar isso num arquivo rs de utilidade
pub fn reconstruir_caminho(origem: usize, destino: usize, anterior: &Vec<Option<usize>>) -> Vec<usize>{
    let mut caminho = vec![];
    let mut atual = Some(destino);

    while let Some(v) = atual {
        caminho.push(v);
        if v == origem {
            break;
        }
        atual = anterior[v];
    }

    caminho.reverse();
    caminho
}

/// # Algoritmo e associação com o livro.
/// ## Argumentos
/// 
/// "grafo" - O grafo analisado;\
/// "origem" - Vértice origem do caminho.
///
/// ## Retorno
/// 
/// Vetor de custos para os demais vértices;\
/// Vetor de anteriores, para a reconstruçao do caminho.
/// 
/// ## Associação com livro
/// ```rust
/// let num_v = grafo.num_vertex() + 1;
///
///   // Linha 1: Cria um vetor de distâncias, inicializando todos com INFINITO. Em seguida a distancia com a origem recebe zero
///    let mut d = vec![i32::MAX/2; num_v]; //ta dividido por 2 pra evitar overflow
///    d[*origem] = 0;
///
///    //Linha 2: Cria o vetor A (aberto) com os vértices e F (fechado) vazio. em seguida cria o vetor anterior para o caminho minimo
///    let mut A = grafo.vertices_list();
///    let mut F = vec![];
///    let mut anterior = vec![None; num_v];
///
///    //Linha 3: Inicio do loop, roda enquanto A não for vazio
///    while !(A.is_empty()){
///        //Linha 4: Pega o vértice com a menor distânica (soma dos pesos) da origem
///        let r = match vertice_mais_proximo(&d, &A) {
///            Some(v) => v,
///            None => break, //Não há vizinhos -> rever o que fazer
///        };
///
///        //Linha 5: Adiciona r em F e remove r de A
///        F.push(r);
///        A.retain(|&x| x != r); // mantem apenas itens que não são iguais a r
///
///        //Linha 6: fazer a interseção dos vizinhos de r e os vertices que estão em A
///        let S = intersecao(&A, &grafo, &r);
///
///        //Linha 7 inicio do loop
///        for i in S {
///            //Linha 8 p recebe o valor menor entre a distancia(1,i) anterior com [d(1,r) + v(r,i)]
///            if let Some(peso_ri) = grafo.weight(&r, &i) {
///                let soma_nova = d[r] + peso_ri;
///                let p = cmp::min(d[i], soma_nova);
///
///                //Linha 9
///                if p < d[i] {
///                    //Linha 10 O valor anterior recebe p e r é inserido no vetor de caminho anterior na posição i
///                    d[i] = p;
///                    anterior[i] = Some(r);
///                }
///            };    
///        }
///    }
///
///    (d, anterior) // Retorno
/// ```
pub fn dijikstra(grafo: &Graph, origem: &usize) -> (Vec<i32>, Vec<Option<usize>>) { //retorna as distancias e anteriores pros caminhos
    let num_v = grafo.num_vertex() + 1;

    // Linha 1: Cria um vetor de distâncias, inicializando todos com INFINITO. Em seguida a distancia com a origem recebe zero
    let mut d = vec![i32::MAX/2; num_v]; //ta dividido por 2 pra evitar overflow
    d[*origem] = 0;

    //Linha 2: Cria o vetor A (aberto) com os vértices e F (fechado) vazio. em seguida cria o vetor anterior para o caminho minimo
    let mut A = grafo.vertices_list();
    let mut F = vec![];
    let mut anterior = vec![None; num_v];

    //Linha 3: Inicio do loop, roda enquanto A não for vazio
    while !(A.is_empty()){
        //Linha 4: Pega o vértice com a menor distânica (soma dos pesos) da origem
        let r = match vertice_mais_proximo(&d, &A) {
            Some(v) => v,
            None => break, //Não há vizinhos -> rever o que fazer
        };

        //Linha 5: Adiciona r em F e remove r de A
        F.push(r);
        A.retain(|&x| x != r); // mantem apenas itens que não são iguais a r

        //Linha 6: fazer a interseção dos vizinhos de r e os vertices que estão em A
        let S = intersecao(&A, &grafo, &r);

        //Linha 7 inicio do loop
        for i in S {
            //Linha 8 p recebe o valor menor entre a distancia(1,i) anterior com [d(1,r) + v(r,i)]
            if let Some(peso_ri) = grafo.weight(&r, &i) {
                let soma_nova = d[r] + peso_ri;
                let p = cmp::min(d[i], soma_nova);

                //Linha 9
                if p < d[i] {
                    //Linha 10 O valor anterior recebe p e r é inserido no vetor de caminho anterior na posição i
                    d[i] = p;
                    anterior[i] = Some(r);
                }
            };    
        }
    }

    (d, anterior) // Retorno
}