mod race;
mod race_set;

use std::str::FromStr;

use race_set::RaceResultSet;
use util::Timer;

use crate::race::RaceResult;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let text = std::fs::read_to_string("./day_06/input.txt")?;

    part_2(&text);
    
    Ok(())
}

#[allow(unused)]
fn part_1(text: &str) {
    let record_set: RaceResultSet = RaceResultSet::from_str(text).unwrap();

    println!("Different winning count product is {}", record_set.winner_count_multiple());
}

fn part_2(text: &str) {
    let record = RaceResult::from_str(text).unwrap();

    println!("The different ways of winning the race is {}", record.compute_winners());
}