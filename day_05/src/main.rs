mod map;
mod almanac;

use almanac::Almanac;
use util::Timer;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let input = std::fs::read_to_string("./day_05/input.txt")?;

    let almanac = Almanac::from_str(&input).unwrap();

    part_1(almanac);
    Ok(())
}

fn part_1(almanac: Almanac) {
    let lowest_location = almanac.lowest_location();

    println!("The lowests location for any of the seed numbers is {lowest_location}");
}