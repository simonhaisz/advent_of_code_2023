use std::collections::HashMap;

use crate::node::Node;

pub struct Network {
    nodes_map: HashMap<String, Node>,
}

impl Network {
    pub fn new() -> Self {
        Self { nodes_map: HashMap::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes_map.insert(node.id().to_string(), node);
    }
    
    pub fn get_node(&self, id: &str) -> &Node {
        self.nodes_map.get(id).unwrap()
    }
}