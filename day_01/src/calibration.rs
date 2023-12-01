pub fn calibration_total(document: &str) -> i32 {
	let mut total = 0;
	for line in document.lines() {
		if line.is_empty() {
			continue;
		}
		let value = calibration_value(line);
		total += value;
	}

	total
}

fn calibration_value(line: &str) -> i32 {
	let first_digit = find_first_digit(line);
	let last_digit = find_last_digit(line);

	let mut value = String::new();
	value.push(first_digit);
	value.push(last_digit);

	value.parse().unwrap()
}

fn find_first_digit(line: &str) -> char {
	for c in line.chars() {
		if c.is_numeric() {
			return c;
		}
	}
	panic!("no digit was found on line {line}");
}

fn find_last_digit(line: &str) -> char {
	for c in line.chars().rev() {
		if c.is_numeric() {
			return c;
		}
	}
	panic!("no digit was found on line {line}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn line_1() {
		let value = calibration_value("1abc2");
		assert_eq!(12, value);
	}

	#[test]
	fn line_2() {
		let value = calibration_value("pqr3stu8vwx");
		assert_eq!(38, value);
	}

	#[test]
	fn line_3() {
		let value = calibration_value("a1b2c3d4e5f");
		assert_eq!(15, value);
	}

	#[test]
	fn line_4() {
		let value = calibration_value("treb7uchet");
		assert_eq!(77, value);
	}

	#[test]
	fn document() {
		let total = calibration_total(
			r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
			);
		
		assert_eq!(142, total);
	}
}