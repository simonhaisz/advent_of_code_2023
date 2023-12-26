use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Navigation {
    instructions: Vec<Direction>,
}

impl Navigation {

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn get(&self, index: usize) -> Direction {
        self.instructions[index]
    }

    pub fn iter(&self) -> NavigationIterator {
        NavigationIterator::new(self)
    }
}

#[derive(Debug)]
pub struct ParseNavigationError;

impl FromStr for Navigation {
    type Err = ParseNavigationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let instructions = line.chars()
            .map(|c| {
                match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unexpected Direction character '{}'", c)
                }
            })
            .collect();

        Ok(Navigation { instructions })
    }
}

pub struct NavigationIterator<'a> {
    navigation: &'a Navigation,
    current_index: usize,
}

impl <'a> NavigationIterator<'a> {
    pub fn new(navigation: &'a Navigation) -> Self {
        Self { navigation, current_index: 0 }
    }
}

impl Iterator for NavigationIterator<'_> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.navigation.get(self.current_index);

        self.current_index += 1;
        if self.current_index >= self.navigation.len() {
            self.current_index = 0;
        }

        Some(direction)
    }
}