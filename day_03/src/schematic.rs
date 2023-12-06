use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Clone)]
struct Position(u32, u32);

#[derive(Clone)]
struct Number {
	value: u32,
	position: Position,
	size: u32,
}

#[derive(Clone)]
struct Symbol {
	value: char,
	position: Position,
}

struct Gear {
	symbol: Symbol,
	first_part_number: Number,
	second_part_number: Number,
}

impl Gear {
	fn ratio(&self) -> u32 {
		self.first_part_number.value * self.second_part_number.value
	}
}

pub struct Schematic {
	width: u32,
	height: u32,
	numbers: Vec<Number>,
	symbols: Vec<Symbol>,
}

impl Number {
	fn adjacent(&self, symbol: &Symbol) -> bool {
		let column_delta = self.position.0 as i64 - symbol.position.0 as i64;
		if column_delta > 1 {
			return false;
		}
		if column_delta < -(self.size as i64) {
			return false;
		}
		let row_delta = self.position.1 as i64 - symbol.position.1 as i64;
		row_delta.abs() < 2
	}

	fn well_before(&self, symbol: &Symbol) -> bool {
		if self.position.1 < 2 {
			false
		} else {
			self.position.1 < symbol.position.1 - 1
		}
	}

	fn well_after(&self, symbol: &Symbol) -> bool {
		self.position.1 > symbol.position.1 + 1
	}
}

impl Schematic {
	fn new(width: u32, height: u32, numbers: Vec<Number>, symbols: Vec<Symbol>) -> Self {
		Self { width, height, numbers, symbols }
	}

	fn find_part_numbers(&self) -> Vec<u32> {
		let mut part_numbers = vec![];

		for n in self.numbers.iter() {
			for s in self.symbols.iter() {
				if n.adjacent(&s) {
					part_numbers.push(n.value);
					break;
				}
			}
		}

		part_numbers
	}

	pub fn part_number_sum(&self) -> u32 {
		self.find_part_numbers()
			.iter()
			.sum()
	}

	fn find_gears(&self) -> Vec<Gear> {
		let mut gears = vec![];

		for s in self.symbols.iter() {
			if s.value != '*' {
				continue;
			}

			let mut adjacent_numbers = vec![];
			for n in self.numbers.iter() {
				if n.well_before(&s) {
					continue;
				}
				if n.well_after(&s) {
					break;
				}
				if n.adjacent(&s) {
					adjacent_numbers.push(n.clone());
				}
			}

			if adjacent_numbers.len() == 2 {
				gears.push(Gear { symbol: s.clone(), first_part_number: adjacent_numbers[0].clone(), second_part_number: adjacent_numbers[1].clone() })
			}
		}

		gears
	}

	pub fn gear_ratio_sum(&self) -> u32 {
		self.find_gears()
			.iter()
			.map(|g| g.ratio())
			.sum()
	}
}

#[derive(Debug)]
pub struct ParseSchematicError;

impl TryFrom<File> for Schematic {
    type Error = ParseSchematicError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut width = 0;
		let mut height = 0;
		let mut numbers = vec![];
		let mut symbols = vec![];

		let mut current_number = String::new();

		let lines = BufReader::new(file).lines();

		let mut add_number = |number: &str, column: u32, row: u32| {
			let value = number.parse::<u32>().unwrap();
			let size = u32::try_from(number.len()).unwrap();
			let x = column.checked_sub(size).unwrap();
			let y = row;
			let position = Position(x, y);
			
			numbers.push(Number { value, position, size });
		};

		for line in lines {
			let line = line.unwrap();
			if line.is_empty() {
				continue;
			}
			if width == 0 {
				width = u32::try_from(line.len()).unwrap();
			}

			for (column, ch) in line.chars().enumerate() {
				if ch.is_ascii_digit() {
					current_number.push(ch);
				} else {
					if current_number.len() > 0 {
						add_number(&current_number, u32::try_from(column).unwrap(), height);

						current_number.clear();
					}

					assert!(!ch.is_ascii_alphabetic());

					if ch != '.' {
						let value = ch;
						let x = u32::try_from(column).unwrap();
						let y = height;
						let position = Position(x, y);
						symbols.push(Symbol { value, position })
					}
				}
			}

			if current_number.len() > 0 {
				add_number(&current_number, width, height);

				current_number.clear();
			}

			height += 1;
		}

        Ok(Schematic::new(width, height, numbers, symbols))
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part_number_sum() {
		let file = File::open("./example.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let part_number_sum = schematic.part_number_sum();

		assert_eq!(4361, part_number_sum);
	}

	#[test]
	fn online_example_1_part_number_sum() {
		let file = File::open("./online-example-1.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let part_number_sum = schematic.part_number_sum();

		assert_eq!(413, part_number_sum);
	}

	#[test]
	fn online_example_2_part_number_sum() {
		let file = File::open("./online-example-2.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let part_number_sum = schematic.part_number_sum();

		assert_eq!(925, part_number_sum);
	}

	#[test]
	fn online_example_3_part_number_sum() {
		let file = File::open("./online-example-3.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let part_number_sum = schematic.part_number_sum();

		assert_eq!(62, part_number_sum);
	}

	#[test]
	fn online_example_4_part_number_sum() {
		let file = File::open("./online-example-4.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let part_number_sum = schematic.part_number_sum();

		assert_eq!(4, part_number_sum);
	}

	#[test]
	fn online_example_5_part_number_sum() {
		let file = File::open("./online-example-5.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let part_number_sum = schematic.part_number_sum();

		assert_eq!(156, part_number_sum);
	}

	#[test]
	fn example_gear_ratio_sum() {
		let file = File::open("./example.txt").unwrap();
		let schematic = Schematic::try_from(file).unwrap();

		let gear_ratio_sum = schematic.gear_ratio_sum();

		assert_eq!(467835, gear_ratio_sum);
	}
}