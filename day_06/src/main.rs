mod race;
mod race_set;

use std::str::FromStr;

use race_set::RaceResultSet;
use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let text = std::fs::read_to_string("./day_06/input.txt")?;

    let record_set = RaceResultSet::from_str(&text).unwrap();
    
    part_1(&record_set);
    
    Ok(())
}

fn part_1(record_set: &RaceResultSet) {
    println!("Different winning count product is {}", record_set.winner_count_multiple());
}