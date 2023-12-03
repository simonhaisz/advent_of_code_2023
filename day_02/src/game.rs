use std::str::FromStr;

use lazy_regex::{regex_captures, regex};
use pest::Parser;

use crate::bag_of_cubes::{BagOfCubesParser, Rule};

enum ParseMode {
	Custom,
	Pest
}

const PARSE_MODE: ParseMode = ParseMode::Custom;

#[derive(PartialEq, Eq, Debug)]
pub struct CubeSet {
	red: u32,
	green: u32,
	blue: u32,
}

impl CubeSet {
	pub fn new(red: u32, green: u32, blue: u32) -> Self {
		Self { red, green, blue }
	}

	pub fn subset(&self, other: &CubeSet) -> bool {
		self.red >= other.red && self.green >= other.green && self.blue >= other.blue
	}

	pub fn power(&self) -> u32 {
		self.red * self.green * self.blue
	}
}

pub struct Game {
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

	pub fn validate_bag(&self, bag: &CubeSet) -> bool {
		for set in self.sets.iter() {
			if !bag.subset(set) {
				return false;
			}
		}
		true
	}

	pub fn minimum_bag(&self) -> CubeSet {
		let mut red = 0;
		let mut green = 0;
		let mut blue = 0;

		for s in self.sets.iter() {
			red = red.max(s.red);
			green = green.max(s.green);
			blue = blue.max(s.blue);
		}

		CubeSet::new(red, green, blue)
	}
}

pub fn valid_games_sum(games: &[Game], bag: &CubeSet) -> u32 {
	games
		.iter()
		.filter(|g| g.validate_bag(bag))
		.map(|g| g.id)
		.sum()
}

pub fn minimum_bag_power_sum(games: &[Game]) -> u32 {
	games
		.iter()
		.map(|g| g.minimum_bag())
		.map(|b| b.power())
		.sum()
}

#[derive(Debug)]
pub struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
		
		let game = match PARSE_MODE {
			ParseMode::Custom => {
				let mut id: Option<u32> = None;
				let mut cube_sets = vec![];
				let result = BagOfCubesParser::parse(Rule::game, line).unwrap();
				for game in result.into_iter() {
					match game.as_rule() {
						Rule::game => {
							for game_property in game.into_inner() {
								match game_property.as_rule() {
									Rule::game_id => {
										id = Some(game_property.as_str().parse::<u32>().unwrap());
									}
									Rule::cube_set => {
										let mut red = 0;
										let mut green = 0;
										let mut blue = 0;
										for cube in game_property.into_inner() {
											match cube.as_rule() {
												Rule::cube => {
													let mut count: Option<u32> = None;
													let mut color: Option<&str> = None;
													for cube_prop in cube.into_inner() {
														match cube_prop.as_rule() {
															Rule::cube_count => {
																count = Some(cube_prop.as_str().parse::<u32>().unwrap());
															},
															Rule::color => {
																color = Some(cube_prop.as_str());
															},
															_ => unimplemented!()
														}
													}
													match color.unwrap() {
														"red" => red = count.unwrap(),
														"green" => green = count.unwrap(),
														"blue" => blue = count.unwrap(),
														_ => unimplemented!()
													}
												},
												_ => unimplemented!()
											}
										}
										cube_sets.push(CubeSet::new(red, green, blue));
									},
									_ => unimplemented!()
								}
							}
						},
						_ => unimplemented!()
					}
				}
		
				let mut game = Game::new(id.unwrap());
		
				for cubes in cube_sets {
					game.add_set(cubes);
				}

				game
			},
			ParseMode::Pest => {
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

				game
			}
		};

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

	#[test]
	fn validate_game_1() {
		let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
		let bag = CubeSet::new(12, 13, 14);
		assert!(game.validate_bag(&bag));
	}

	#[test]
	fn validate_game_2() {
		let game = Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
		let bag = CubeSet::new(12, 13, 14);
		assert!(game.validate_bag(&bag));
	}

	#[test]
	fn validate_game_3() {
		let game = Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap();
		let bag = CubeSet::new(12, 13, 14);
		assert!(!game.validate_bag(&bag));
	}

	#[test]
	fn validate_game_4() {
		let game = Game::from_str("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap();
		let bag = CubeSet::new(12, 13, 14);
		assert!(!game.validate_bag(&bag));
	}

	#[test]
	fn validate_game_5() {
		let game = Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
		let bag = CubeSet::new(12, 13, 14);
		assert!(game.validate_bag(&bag));
	}

	#[test]
	fn valid_games() {
		let games = vec![
			Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
			Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap(),
			Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap(),
			Game::from_str("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap(),
			Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
		];
		let bag = CubeSet::new(12, 13, 14);
		let sum = valid_games_sum(&games, &bag);
		assert_eq!(8, sum);
	}

	#[test]
	fn minimum_bag_game_1() {
		let game = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
		let minimum_bag = game.minimum_bag();
		assert_eq!(CubeSet::new(4, 2, 6), minimum_bag);
	}

	#[test]
	fn minimum_bag_game_2() {
		let game = Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap();
		let minimum_bag = game.minimum_bag();
		assert_eq!(CubeSet::new(1, 3, 4), minimum_bag);
	}

	#[test]
	fn minimum_bag_game_3() {
		let game = Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap();
		let minimum_bag = game.minimum_bag();
		assert_eq!(CubeSet::new(20, 13, 6), minimum_bag);
	}

	#[test]
	fn minimum_bag_game_4() {
		let game = Game::from_str("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap();
		let minimum_bag = game.minimum_bag();
		assert_eq!(CubeSet::new(14, 3, 15), minimum_bag);
	}

	#[test]
	fn minimum_bag_game_5() {
		let game = Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
		let minimum_bag = game.minimum_bag();
		assert_eq!(CubeSet::new(6, 3, 2), minimum_bag);
	}

	#[test]
	fn bag_power_game_1() {
		let bag = CubeSet::new(4, 2, 6);
		let power = bag.power();
		assert_eq!(48, power);
	}

	#[test]
	fn bag_power_game_2() {
		let bag = CubeSet::new(1, 3, 4);
		let power = bag.power();
		assert_eq!(12, power);
	}

	#[test]
	fn bag_power_game_3() {
		let bag = CubeSet::new(20, 13, 6);
		let power = bag.power();
		assert_eq!(1560, power);
	}

	#[test]
	fn bag_power_game_4() {
		let bag = CubeSet::new(14, 3, 15);
		let power = bag.power();
		assert_eq!(630, power);
	}

	#[test]
	fn bag_power_game_5() {
		let bag = CubeSet::new(6, 3, 2);
		let power = bag.power();
		assert_eq!(36, power);
	}

	#[test]
	fn minimum_bag_power() {
		let games = vec![
			Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
			Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").unwrap(),
			Game::from_str("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").unwrap(),
			Game::from_str("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").unwrap(),
			Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap(),
		];
		let sum = minimum_bag_power_sum(&games);
		assert_eq!(2286, sum);
	}
}