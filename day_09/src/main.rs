mod sequence;
mod value;

use util::Timer;
use value::Value;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();
    
    let input = std::fs::read_to_string("day_09/input.txt").unwrap();

    let values = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| Value::from_str(l).unwrap())
        .collect::<Vec<_>>();

    part_1(&values);

    Ok(())
}

fn part_1(values: &[Value]) {
    let total_next = Value::total_next(values);

    println!("The total of all of the next values is {total_next}");
}
