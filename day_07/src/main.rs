mod card;
mod hand;
mod game;

use game::Game;
use util::Timer;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day_07/input.txt")?;

    let game = Game::from_str(&text).unwrap();

    part_1(game);

    Ok(())
}

fn part_1(mut game: Game) {
    game.play();

    let total_winnings = game.total_winnings();

    println!("The total winnings are {}", total_winnings);
}