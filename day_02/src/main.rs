use std::str::FromStr;

use game::{Game, CubeSet, valid_games_sum};
use util::Timer;

mod game;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let game_lines = std::fs::read_to_string("./day_02/input.txt")?;

    let games = game_lines
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Game::from_str(l).unwrap())
        .collect::<Vec<_>>();

    part_1(&games);
    
    Ok(())
}

fn part_1(games: &[Game]) {
    let bag = CubeSet::new(12, 13, 14);
    let sum = valid_games_sum(games, &bag);

    println!("the sum of valid game ids is {sum}");
}
