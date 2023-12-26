#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Navigation {
    instructions: Vec<Direction>,
}

impl Navigation {
    pub fn from(line: &str) -> Self {
        let instructions = line.chars()
            .map(|c| {
                match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unexpected Direction character '{}'", c)
                }
            })
            .collect();

        Navigation { instructions }
    }

    pub fn new() -> Self {
        Self { instructions: vec![] }
    }

    pub fn add(&mut self, direction: Direction) {
        self.instructions.push(direction);
    }

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
        let direction = self.navigation.instructions[self.current_index];

        self.current_index += 1;
        if self.current_index >= self.navigation.len() {
            self.current_index = 0;
        }

        Some(direction)
    }
}