mod graph;

use graph::*;

fn main() {
    let mut graph = Graph::new();
    graph.edge(0, 1, 4);
    graph.edge(0, 2, 5);
    graph.edge(1, 0, 3);
    graph.edge(2, 1, 3);
    graph.print();
}