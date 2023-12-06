use std::{str::FromStr, fs::File, io::{BufReader, BufRead}};

use lazy_regex::regex_captures;

struct Card {
	id: u32,
	winners: Vec<u32>,
	numbers: Vec<u32>,
}

impl Card {
	fn points(&self) -> u32 {
		let mut win_counter = 0;
		for n in self.numbers.iter() {
			for w in self.winners.iter() {
				if *n == *w {
					win_counter += 1;
					break;
				}
			}
		}

		if win_counter == 0 {
			0
		} else {
			2_u32.pow(win_counter - 1)
		}
	}
}

#[derive(Debug)]
pub struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (_, id, winners, numbers) = regex_captures!(r"Card\s+(?<id>\d+): (?<winners>[\d\s]*) \| (?<numbers>[\d\s]*)", text).unwrap();

		let id = id.parse::<u32>().unwrap();
		let number_splitter = |text: &str| {
			text
				.split(" ")
				.filter(|s| s.len() > 0)
				.map(|s| s.parse::<u32>().unwrap())
				.collect::<Vec<_>>()
		};
		let winners = number_splitter(winners);
		let numbers = number_splitter(numbers);

		Ok(Card { id, winners, numbers })
    }
}

pub fn cards_total_points(file: File) -> u32 {
	let mut total_points = 0;

	let lines = BufReader::new(file).lines();

	for line in lines {
		let line = line.unwrap();
		if line.is_empty() {
			continue;
		}

		let card = Card::from_str(&line).unwrap();

		total_points += card.points();
	}

	total_points
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn card_1_points() {
		let card = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();

		let points = card.points();

		assert_eq!(8, points);
	}

	#[test]
	fn card_2_points() {
		let card = Card::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").unwrap();

		let points = card.points();

		assert_eq!(2, points);
	}

	#[test]
	fn card_3_points() {
		let card = Card::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").unwrap();

		let points = card.points();

		assert_eq!(2, points);
	}

	#[test]
	fn card_4_points() {
		let card = Card::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").unwrap();

		let points = card.points();

		assert_eq!(1, points);
	}

	#[test]
	fn card_5_points() {
		let card = Card::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").unwrap();

		let points = card.points();

		assert_eq!(0, points);
	}

	#[test]
	fn card_6_points() {
		let card = Card::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").unwrap();

		let points = card.points();

		assert_eq!(0, points);
	}

	#[test]
	fn example_cards_points_total() {
		let file = File::open("./example.txt").unwrap();
		let total_points = cards_total_points(file);

		assert_eq!(13, total_points)
	}
}