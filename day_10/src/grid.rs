use std::str::FromStr;

use crate::{tile::Tile, pipe::Pipe};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position(u32, u32);
pub struct Dimensions(u32, u32);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub const ALL_DIRECTIONS: &'static [Direction] = &[
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl Direction {
    pub fn follow_pipe(&self, pipe: Pipe) -> Option<Direction> {
        match self {
            Self::North => {
                match pipe {
                    Pipe::NorthSouth => Some(Self::North),
                    Pipe::SouthEast => Some(Self::East),
                    Pipe::SouthWest => Some(Self::West),
                    _ => None,
                }
            },
            Self::South => {
                match pipe {
                    Pipe::NorthSouth => Some(Self::South),
                    Pipe::NorthEast => Some(Self::East),
                    Pipe::NorthWest => Some(Self::West),
                    _ => None,
                }
            },
            Self::East => {
                match pipe {
                    Pipe::EastWest => Some(Self::East),
                    Pipe::NorthWest => Some(Self::North),
                    Pipe::SouthWest => Some(Self::South),
                    _ => None,
                }
            },
            Self::West => {
                match pipe {
                    Pipe::EastWest => Some(Self::West),
                    Pipe::SouthEast => Some(Self::South),
                    Pipe::NorthEast => Some(Self::North),
                    _ => None,
                }
            },
        }
    }
}

pub struct Grid {
    dimensions: Dimensions,
    tiles: Vec<Tile>,
}

impl Grid {
    fn convert_index(&self, index: usize) -> Position {
        let index = u32::try_from(index).unwrap();
        let x = index % self.dimensions.0;
        let y = index / self.dimensions.1;

        Position(x, y)
    }

    fn convert_position(&self, position: &Position) -> usize {
        usize::try_from(position.0 + position.1 * self.dimensions.0).unwrap()
    }

    fn next_position(&self, current: &Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::North => {
                if current.1 > 0 {
                    Some(Position(current.0, current.1 - 1))
                } else {
                    None
                }
            },
            Direction::South => {
                if current.1 < (self.dimensions.1 - 1) {
                    Some(Position(current.0, current.1 + 1))
                } else {
                    None
                }
            },
            Direction::East => {
                if current.0 < (self.dimensions.0 - 1) {
                    Some(Position(current.0 + 1, current.1))
                } else {
                    None
                }
            },
            Direction::West => {
                if current.0 > 0 {
                    Some(Position(current.0 - 1 , current.1))
                } else {
                    None
                }
            }
        }
    }

    fn get_tile(&self, position: &Position) -> Tile {
        let index = self.convert_position(position);

        self.tiles[index].clone()
    }

}

impl Grid {
    pub fn find_start(&self) -> (Position, Tile) {
        for (index, tile) in self.tiles.iter().enumerate() {
            if tile.is_start() {
                return (self.convert_index(index), tile.clone());
            }
        }

        panic!("Could not find start tile in grid")
    }

    pub fn next_tile(&self, current: &Position, direction: Direction) -> Option<(Position, Tile)> {
        if let Some(next_position) = self.next_position(current, direction) {
            let next_tile = self.get_tile(&next_position);
            Some((next_position, next_tile))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut height = 0;

        let mut tiles = vec![];

        for line in text.lines() {
            if line.is_empty() {
                continue;
            }

            let line_length = u32::try_from(line.len()).unwrap();
            if let Some(width) = width {
                assert_eq!(width, line_length);
            } else {
                width.replace(line_length);
            }

            for c in line.chars() {
                tiles.push(c.into());
            }

            height += 1;
        }

        Ok(Grid { dimensions: Dimensions(width.unwrap(), height), tiles })
    }
}