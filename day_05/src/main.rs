mod map;
mod almanac;

use almanac::{Almanac, SeedParsingMode};
use util::Timer;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    let input = std::fs::read_to_string("./day_05/input.txt")?;

    part_2(&input);

    Ok(())
}

#[allow(dead_code)]
fn part_1(input: &str) {
    let almanac = Almanac::<{SeedParsingMode::values_short_form()}>::from_str(input).unwrap();

    let lowest_location = almanac.lowest_location();

    println!("The lowests location for any of the seed numbers is {lowest_location}");
}

fn part_2(input: &str)  {
    let almanac = Almanac::<{SeedParsingMode::range_pair_short_form()}>::from_str(input).unwrap();

    let lowest_location = almanac.lowest_location();

    println!("The lowests location for any of the seed numbers is {lowest_location}");
}