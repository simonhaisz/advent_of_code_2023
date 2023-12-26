mod node;
mod network;
mod navigation;
mod map;

use map::Map;
use util::Timer;
use std::str::FromStr;

use node::{START_NODE_ID, END_NODE_ID};

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let input = std::fs::read_to_string("day_08/input.txt").unwrap();
    let map = Map::from_str(&input).unwrap();

    part_2(map);

    Ok(())
}

#[allow(unused)]
fn part_1(map: Map) {
    let steps = map.navigate_camel();

    println!("It takes {steps} steps to get from {START_NODE_ID} to {END_NODE_ID}");
}

fn part_2(map: Map) {
    let steps = map.navigate_ghost();

    println!("It takes {steps} steps for a ghost travel from all the starts to all of the ends")
}