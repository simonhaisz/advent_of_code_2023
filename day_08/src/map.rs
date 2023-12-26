use std::str::FromStr;

use crate::{navigation::{Navigation, Direction}, network::Network, node::{START_NODE_ID, END_NODE_ID, Node}};

pub struct Map {
    navigation: Navigation,
    network: Network,
}

impl Map {
    pub fn new(navigation: Navigation, network: Network) -> Self {
        Self { navigation, network }
    }

    pub fn navigate(&self) -> u32 {
        let mut step_count = 0;
        let mut current_node = self.network.get_node(START_NODE_ID);

        let mut instruction_iter = self.navigation.iter();

        loop {
            let instruction = instruction_iter.next().unwrap();

            let next_id = match instruction {
                Direction::Left => {
                    current_node.left()
                },
                Direction::Right => {
                    current_node.right()
                }
            };

            let next_node = self.network.get_node(next_id);
            current_node = next_node;
            step_count += 1;

            if current_node.id() == END_NODE_ID {
                break;
            }
        }
        step_count
    }
}

#[derive(Debug)]
pub struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut navigation = None;
        let mut network = Network::new();

        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            if navigation.is_none() {
                navigation.replace(Navigation::from_str(line).unwrap());
            } else {
                network.add_node(Node::from_str(line).unwrap());
            }
        }

        Ok(Map::new(navigation.unwrap(), network))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let map = Map::from_str(r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
        ".trim()).unwrap();

        assert_eq!(2, map.navigate());
    }

    #[test]
    fn example_2() {
        let map = Map::from_str(r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
        ".trim()).unwrap();

        assert_eq!(6, map.navigate());
    }
}