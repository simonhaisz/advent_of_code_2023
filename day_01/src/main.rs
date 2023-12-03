

mod calibration;

use calibration::calibration_total;

use crate::calibration::ParseMode;

use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let document = std::fs::read_to_string("./day_01/input.txt")?;

    part_2(&document);

    Ok(())
}

#[allow(dead_code)]
fn part_1(document: &str) {
    let total = calibration_total(document, ParseMode::Numeric);

    println!("The calibration total of the document is {total}");
}

fn part_2(document: &str) {
    let total = calibration_total(document, ParseMode::Text);

    println!("The calibration total of the document is {total}");
}