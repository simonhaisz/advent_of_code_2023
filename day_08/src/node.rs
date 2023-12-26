use std::str::FromStr;

use lazy_regex::regex_captures;

pub const START_NODE_ID: &'static str = "AAA";
pub const END_NODE_ID: &'static str = "ZZZ";

pub struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    pub fn new(id: String, left: String, right: String) -> Self {
        Self { id, left, right }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn left(&self) -> &str {
        &self.left
    }

    pub fn right(&self) -> &str {
        &self.right
    }
}

#[derive(Debug)]
pub struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, id, left, right) = regex_captures!(r"^(?<id>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)$", line).unwrap();

        Ok(Node::new(id.to_string(), left.to_string(), right.to_string()))
    }
}
