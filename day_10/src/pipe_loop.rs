use std::collections::HashSet;

use crate::{tile::Tile, grid::{Position, Grid, Direction, ALL_DIRECTIONS}};

#[derive(Debug, Clone)]
pub struct PipeSection(Position, Tile);

pub struct PipeLoop {
    sections: Vec<PipeSection>,
}

impl PipeLoop {
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    pub fn furthest_from_start(&self) -> u32 {
        u32::try_from(self.len() / 2).unwrap()
    }

    pub fn equivalent(&self, other: &PipeLoop) -> bool {
        if self.sections.len() != other.sections.len() {
            false
        } else {
            self.positions().eq(&other.positions())
        }
    }

    fn positions(&self) -> HashSet<Position> {
        self.sections.iter()
                .map(|s| s.0.clone())
                .collect()
    }
}

impl PipeLoop {
    pub fn find_loop(grid: &Grid, start_direction: Direction) -> Option<PipeLoop> {
        let (start_position, start_tile) = grid.find_start();

        let start_section = PipeSection(start_position.clone(), start_tile);

        let mut sections = vec![
            start_section,
        ];

        let mut current_position = start_position;
        let mut current_direction = start_direction;

        loop {
            let next_tile = grid.next_tile(&current_position, current_direction);
            if next_tile.is_none() {
                return None;
            }
            let (next_position, next_tile) = next_tile.unwrap();
            if next_tile.is_start() {
                break;
            }
            if let Tile::Pipe(pipe) = next_tile {
                current_position = next_position.clone();

                if let Some(next_direction) = current_direction.follow_pipe(pipe) {
                    current_direction = next_direction;
                    sections.push(PipeSection(next_position, next_tile));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(PipeLoop { sections })
    }

    pub fn find_all_loops(grid: &Grid) -> Vec<PipeLoop> {
        // ALL_DIRECTIONS.iter()
        //     .map(|d| Self::find_loop(grid, *d))
        //     .filter(|l| l.is_some())
        //     .map(|l| l.unwrap())
        //     .collect()

        let mut pipe_loops = vec![];

        for direction in ALL_DIRECTIONS {
            if let Some(pipe_loop) = Self::find_loop(grid, *direction) {
                if pipe_loops.iter().position(|l: &PipeLoop| l.equivalent(&pipe_loop)).is_none() {
                    pipe_loops.push(pipe_loop);
                }
            }
        }

        pipe_loops
    }

    pub fn find_first_loop_furthest_from_start(grid: &Grid) -> u32 {
        let pipe_loops = Self::find_all_loops(grid);
        assert_eq!(1, pipe_loops.len());
        let first_loop = &pipe_loops[0];

        first_loop.furthest_from_start()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn example_1() {
        let grid = Grid::from_str(r"
.....
.S-7.
.|.|.
.L-J.
.....
        ".trim()).unwrap();

        let pipe_loops = PipeLoop::find_all_loops(&grid);
        assert_eq!(1, pipe_loops.len());
        let pipe_loop = &pipe_loops[0];
        assert_eq!(8, pipe_loop.len());
        assert_eq!(4, pipe_loop.furthest_from_start());
    }

    #[test]
    fn example_2() {
        let grid = Grid::from_str(r"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
        ".trim()).unwrap();

        let pipe_loops = PipeLoop::find_all_loops(&grid);
        assert_eq!(1, pipe_loops.len());
        let pipe_loop = &pipe_loops[0];
        assert_eq!(16, pipe_loop.len());
        assert_eq!(8, pipe_loop.furthest_from_start());
    }
}