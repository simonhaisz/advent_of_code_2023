use std::fs::File;

use card::cards_total_points;

mod card;

fn main() -> std::io::Result<()> {

    let file = File::open("./day_04/input.txt")?;

    part_1(file);

    Ok(())
}

fn part_1(file: File) {
    let total_points = cards_total_points(file);

    println!("the cards total points is {total_points}");
}