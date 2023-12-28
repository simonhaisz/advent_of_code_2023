use crate::pipe::Pipe;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}

impl Tile {
    pub fn is_start(&self) -> bool {
        self == &Self::Start
    }

    pub fn is_pipe(&self) -> bool {
        match self {
            Self::Pipe(_) => true,
            _ => false,
        }
    }
}

impl From<char> for Tile {
    fn from(tile: char) -> Self {
        match tile {
            '.' => Self::Ground,
            'S' => Self::Start,
            '|' => Self::Pipe(Pipe::NorthSouth),
            '-' => Self::Pipe(Pipe::EastWest),
            'L' => Self::Pipe(Pipe::NorthEast),
            'J' => Self::Pipe(Pipe::NorthWest),
            '7' => Self::Pipe(Pipe::SouthWest),
            'F' => Self::Pipe(Pipe::SouthEast),
            _ => panic!("Unknown tile character '{tile}'"),
        }
    }
}