use std::str::FromStr;

use crate::{navigation::{Navigation, Direction}, network::Network, node::Node, lcm::lowest_common_multiple};

pub struct Map {
    navigation: Navigation,
    network: Network,
}

impl Map {
    pub fn new(navigation: Navigation, network: Network) -> Self {
        Self { navigation, network }
    }

    pub fn navigate_camel(&self) -> u32 {
        Map::navigate(&self, self.network.get_camel_start_node(), |n| n.is_camel_end())
    }

    pub fn navigate_ghost(&self) -> u64 {

        let steps = self.network.find_all_ghost_start_nodes().iter()
            .map(|start_node| self.navigate(*start_node, |n| n.is_ghost_end()))
            .collect::<Vec<_>>();

        lowest_common_multiple(&steps)
    }

    fn navigate<F>(&self, start_node: &Node, end_check: F) -> u32
        where F: Fn(&Node) -> bool {
            let mut step_count = 0;
            let mut current_node = start_node;
    
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
    
                if end_check(current_node) {
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
    fn camel_example_1() {
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

        assert_eq!(2, map.navigate_camel());
    }

    #[test]
    fn camel_example_2() {
        let map = Map::from_str(r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
        ".trim()).unwrap();

        assert_eq!(6, map.navigate_camel());
    }

    #[test]
    fn ghost_example_1() {
        let map = Map::from_str(r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
        ".trim()).unwrap();

        assert_eq!(6, map.navigate_ghost());
    }
}