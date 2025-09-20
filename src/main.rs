mod scenario_one;
mod graph;
mod dijkstra;
mod bellman_ford;
mod floydwarshall;
mod scenario_three;

use crate::scenario_one::first_scenario;
use crate::scenario_three::third_scenario;

fn main() {
    first_scenario();
    third_scenario();
}