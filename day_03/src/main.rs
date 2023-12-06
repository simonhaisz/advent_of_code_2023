use std::fs::File;

use schematic::Schematic;
use util::Timer;

mod schematic;

fn main() -> std::io::Result<()> {

    let _timer = Timer::new();

    let file = File::open("./day_03/input.txt").unwrap();
    let schematic = Schematic::try_from(file).unwrap();

    part_2(schematic);

    Ok(())
}

#[allow(dead_code)]
fn part_1(schematic: Schematic) {
    let part_number_sum = schematic.part_number_sum();

    println!("the schematic's part number sum is {part_number_sum}");
}

fn part_2(schematic: Schematic) {
    let gear_ratio_sum = schematic.gear_ratio_sum();

    println!("the schematic's gear ratio sum is {gear_ratio_sum}");
}