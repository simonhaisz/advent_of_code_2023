mod pipe;
mod tile;
mod grid;
mod pipe_loop;

use std::str::FromStr;

use grid::Grid;
use pipe_loop::PipeLoop;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let input = std::fs::read_to_string("day_10/input.txt").unwrap();

    let grid = Grid::from_str(&input).unwrap();

    part_1(grid);

    Ok(())
}

fn part_1(grid: Grid) {
    let distance = PipeLoop::find_first_loop_furthest_from_start(&grid);

    println!("The furthest distance down the pipe is {distance}");
}