pub enum ParseMode {
	Numeric,
	Text,
}

pub fn calibration_total(document: &str, mode: ParseMode) -> u32 {
	let mut total = 0;
	for line in document.lines() {
		if line.is_empty() {
			continue;
		}
		let value = calibration_value(line, &mode);
		total += value;
	}

	total
}

fn calibration_value(line: &str, mode: &ParseMode) -> u32 {
	let (first_digit, last_digit) = match mode {
		ParseMode::Numeric => {
			(find_first_digit(line), find_last_digit(line))
		},
		ParseMode::Text => {
			let digits = parse_digits(line);
			assert!(!digits.is_empty());
			(digits.first().unwrap().clone(), digits.last().unwrap().clone())
		}
	};

	let mut value = String::new();
	value.push(first_digit);
	value.push(last_digit);

	value.parse().unwrap()
}

fn find_first_digit(line: &str) -> char {
	for c in line.chars() {
		if c.is_ascii_digit() {
			return c;
		}
	}
	panic!("no digit was found on line {line}");
}

fn find_last_digit(line: &str) -> char {
	for c in line.chars().rev() {
		if c.is_ascii_digit() {
			return c;
		}
	}
	panic!("no digit was found on line {line}");
}

fn parse_digits(line: &str) -> Vec<char> {
	let mut digits = vec![];

	let targets = vec![
		"0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
		"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];

	for target in targets {
		let matches = find_matches(line, target);

		for m in matches {
			let digit = if target.len() == 1 {
				target.chars().nth(0).unwrap()
			} else {
				match target {
					"zero" => '0',
					"one" => '1',
					"two" => '2',
					"three" => '3',
					"four" => '4',
					"five" => '5',
					"six" => '6',
					"seven" => '7',
					"eight" => '8',
					"nine" => '9',
					_ => panic!("unexpected text digit '{target}'")
				}
			};
			digits.push((m, digit));
		}
	}

	digits.sort_by(|(index_a, _), (index_b, _)| index_a.cmp(index_b));

	digits.into_iter().map(|(_, digit)| digit).collect()
}

fn find_matches(line: &str, target: &str) -> Vec<usize> {
	let mut matches = vec![];

	for (index, _) in line.match_indices(target) {
		matches.push(index);
	}
	
	matches
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn digit_line_1() {
		let value = calibration_value("1abc2", &ParseMode::Numeric);
		assert_eq!(12, value);
	}

	#[test]
	fn digit_line_2() {
		let value = calibration_value("pqr3stu8vwx", &ParseMode::Numeric);
		assert_eq!(38, value);
	}

	#[test]
	fn digit_line_3() {
		let value = calibration_value("a1b2c3d4e5f", &ParseMode::Numeric);
		assert_eq!(15, value);
	}

	#[test]
	fn digit_line_4() {
		let value = calibration_value("treb7uchet", &ParseMode::Numeric);
		assert_eq!(77, value);
	}

	#[test]
	fn digit_document() {
		let total = calibration_total(
			r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#,
			ParseMode::Numeric
		);
		
		assert_eq!(142, total);
	}

	#[test]
	fn text_line_1() {
		let value = calibration_value("two1nine", &ParseMode::Text);
		assert_eq!(29, value);
	}

	#[test]
	fn text_line_2() {
		let value = calibration_value("eightwothree", &ParseMode::Text);
		assert_eq!(83, value);
	}

	#[test]
	fn text_line_3() {
		let value = calibration_value("abcone2threexyz", &ParseMode::Text);
		assert_eq!(13, value);
	}

	#[test]
	fn text_line_4() {
		let value = calibration_value("xtwone3four", &ParseMode::Text);
		assert_eq!(24, value);
	}

	#[test]
	fn text_line_5() {
		let value = calibration_value("4nineeightseven2", &ParseMode::Text);
		assert_eq!(42, value);
	}

	#[test]
	fn text_line_6() {
		let value = calibration_value("zoneight234", &ParseMode::Text);
		assert_eq!(14, value);
	}

	#[test]
	fn text_line_7() {
		let value = calibration_value("7pqrstsixteen", &ParseMode::Text);
		assert_eq!(76, value);
	}
}