use std::str::FromStr;

use lazy_regex::{regex_captures, regex};

#[derive(PartialEq, Eq, Debug)]
struct CubeSet {
	red: u32,
	green: u32,
	blue: u32,
}

impl CubeSet {
	fn new(red: u32, green: u32, blue: u32) -> Self {
		Self { red, green, blue }
	}
}

struct Game {
	id: u32,
	sets: Vec<CubeSet>,
}

impl Game {
	pub fn new(id: u32) -> Self {
		Self { id, sets: vec![] }
	}

	pub fn add_set(&mut self, set: CubeSet) {
		self.sets.push(set);
	}
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, id) = regex_captures!(r"Game (?<id>\d+):", line).unwrap();
		let id = id.parse::<u32>().unwrap();

		let mut game = Game::new(id);

		let start_index = line.find(":").unwrap();
		let cube_sets = &line[start_index+1..];
		let cube_sets = cube_sets.split(";");

		for set in cube_sets {
			let cube_set_regex = regex!(r"\s(?<count>\d+)\s(?<label>\w+)(?:,|$)");

			let mut red = 0;
			let mut green = 0;
			let mut blue = 0;
			for caps in cube_set_regex.captures_iter(set) {
				let count = caps.name("count").unwrap().as_str().parse::<u32>().unwrap();
				let label = caps.name("label").unwrap().as_str();
				match label {
					"red" => red = count,
					"green" => green = count,
					"blue" => blue = count,
					_ => panic!("unknown label '{label}'"),
				}
			}

			game.add_set(CubeSet::new(red, green, blue));
		}

		Ok(game)
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_game_1() {
		let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
		assert_eq!(1, game.id);
		assert_eq!(3, game.sets.len());
		assert_eq!(CubeSet::new(4, 0, 3), game.sets[0]);
		assert_eq!(CubeSet::new(1, 2, 6), game.sets[1]);
		assert_eq!(CubeSet::new(0, 2, 0), game.sets[2]);
	}

	#[test]
	fn parse_game_2() {
		let game = Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
		assert_eq!(2, game.id);
		assert_eq!(3, game.sets.len());
		assert_eq!(CubeSet::new(0, 2, 1), game.sets[0]);
		assert_eq!(CubeSet::new(1, 3, 4), game.sets[1]);
		assert_eq!(CubeSet::new(0, 1, 1), game.sets[2]);
	}

	#[test]
	fn parse_game_3() {
		let game = Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap();
		assert_eq!(3, game.id);
		assert_eq!(3, game.sets.len());
		assert_eq!(CubeSet::new(20, 8, 6), game.sets[0]);
		assert_eq!(CubeSet::new(4, 13, 5), game.sets[1]);
		assert_eq!(CubeSet::new(1, 5, 0), game.sets[2]);
	}

	#[test]
	fn parse_game_4() {
		let game = Game::from_str("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap();
		assert_eq!(4, game.id);
		assert_eq!(3, game.sets.len());
		assert_eq!(CubeSet::new(3, 1, 6), game.sets[0]);
		assert_eq!(CubeSet::new(6, 3, 0), game.sets[1]);
		assert_eq!(CubeSet::new(14, 3, 15), game.sets[2]);
	}

	#[test]
	fn parse_game_5() {
		let game = Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
		assert_eq!(5, game.id);
		assert_eq!(2, game.sets.len());
		assert_eq!(CubeSet::new(6, 3, 1), game.sets[0]);
		assert_eq!(CubeSet::new(1, 2, 2), game.sets[1]);
	}
}