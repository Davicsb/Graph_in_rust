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
    println!("Choose a scenario:\n1 - Determining the central station\n2 - Optimizing path with regeneration\n3 - Warehouse robot with obstacles");
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line.");
    let num : i32 = input_line.trim().parse().expect("The input is not an integer.");
    //tudo isso pra ler um int...

    match num {
        1 => first_scenario(),
        2 => second_scenario(),
        3 => third_scenario(),
        _ => println!("Cenário inválido.")
    };
}