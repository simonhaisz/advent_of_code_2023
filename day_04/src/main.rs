use std::fs::File;

use card::{cards_total_points, play_cards_from_file};
use util::Timer;

mod card;

fn main() -> std::io::Result<()> {

    let _timer = Timer::new();

    let file = File::open("./day_04/input.txt")?;

    part_2(file);

    Ok(())
}

#[allow(dead_code)]
fn part_1(file: File) {
    let total_points = cards_total_points(file);

    println!("the cards total points is {total_points}");
}

fn part_2(file: File) {
    let cards_count = play_cards_from_file(file);

    println!("after playing them the total number of cards is {cards_count}");
}