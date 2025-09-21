mod graph;
mod dijkstra;
mod bellman_ford;
mod floydwarshall;
mod scenario_one;
mod scenario_two;
mod scenario_three;

use std::io;
use crate::scenario_one::first_scenario;
use crate::scenario_two::second_scenario;
use crate::scenario_three::third_scenario;

fn main() {
    println!("Escolha um cenário:\n1 - Determinando a estação central\n2 - Otimizando caminho com regeneração\n3 - Robô de armazém com obstáculos");
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Falha ao ler a linha.");
    let num : i32 = input_line.trim().parse().expect("O input não é um inteiro");
    //tudo isso pra ler um int...

    match num {
        1 => first_scenario(),
        2 => second_scenario(),
        3 => third_scenario(),
        _ => println!("Cenário inválido.")
    };
}