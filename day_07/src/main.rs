mod card;
mod hand;
mod game;
mod rules;

use game::Game;
use util::Timer;
use std::str::FromStr;

use crate::rules::Edition;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day_07/input.txt")?;

    let game = Game::from_str(&text).unwrap();

    part_2(game);

    Ok(())
}

#[allow(unused)]
fn part_1(mut game: Game) {
    game.play(Edition::Standard);

    let total_winnings = game.total_winnings();

    println!("The total winnings are {}", total_winnings);
}
fn part_2(mut game: Game) {
    game.play(Edition::JacksAreJokers);

    let total_winnings = game.total_winnings();

    println!("The total winnings are {}", total_winnings);

}