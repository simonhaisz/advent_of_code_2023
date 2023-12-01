use calibration::calibration_total;

mod calibration;

fn main() -> std::io::Result<()> {
    let document = std::fs::read_to_string("./day_01/input.txt")?;

    part_1(&document);

    Ok(())
}

fn part_1(document: &str) {
    let total = calibration_total(document);

    println!("The calibration total of the document is {total}");
}