use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    value: usize,
    weight: usize,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: usize, weight: usize) -> Self {
        Node { value, weight, next: None }
    }

    pub fn append(&mut self, value: usize, weight: usize) {
        match &mut self.next {
            Some(next_node) => next_node.append(value, weight),
            None => self.next = Some(Box::new(Node::new(value, weight))),
        }
    }

    pub fn print(&self) {
        print!("{} (weight: {})", self.value, self.weight);
        if let Some(next) = &self.next {
            print!(" -> ");
            next.print();
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    adj: HashMap<usize, Option<Box<Node>>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adj: HashMap::new(),
        }
    }

    pub fn edge(&mut self, origin: usize, destination: usize, destination_weight: usize) {
        // Use `entry` para garantir que a origem tem um valor válido
        let head = self.adj.entry(origin).or_insert(None);
        match head {
            Some(node) => node.append(destination, destination_weight),
            None => *head = Some(Box::new(Node::new(destination, destination_weight))),
        }
    }

    pub fn print(&self) {
        for (v, list) in &self.adj {
            print!("{} -> ", v);
            if let Some(node) = list {
                node.print();
            }
            println!();
        }
    }
}
