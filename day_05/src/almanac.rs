use std::{str::FromStr, char};

use crate::map::{ValueMap, ChainedValueMap, ValueMapRange};

pub enum SeedParsingMode {
	Values,
	RangePair
}

impl SeedParsingMode {
	pub fn from_char(c: char) -> SeedParsingMode {
		match c {
			'V' => SeedParsingMode::Values,
			'P' => SeedParsingMode::RangePair,
			_ => panic!("Unknown seed parsing mode short form '{c}'")
		}
	}

	pub const fn values_short_form() -> char {
		'V'
	}

	pub const fn range_pair_short_form() -> char {
		'P'
	}
}

pub struct Almanac<const M: char> {
	seeds: Vec<i64>,
	chained_map: ChainedValueMap,
}

impl <const M: char> Almanac<M> {
	pub fn new(seeds: Vec<i64>, chained_map: ChainedValueMap) -> Self {
		Self { seeds, chained_map }
	}

	pub fn lowest_location(&self) -> i64 {
		let mut min_location = i64::MAX;

		for s in self.seeds.iter() {
			let location = self.chained_map.map(*s);

			min_location = min_location.min(location);
		}

		min_location
	}
}

#[derive(Debug)]
pub struct ParseAlmanacError;

const SEEDS_HEADER: &str = "seeds: ";
const MAP_HEADER_SUFFIX: &str = "map:";

impl <const M: char> FromStr for Almanac<M> {
    type Err = ParseAlmanacError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
		let mut seeds: Vec<i64> = vec![];
		let mut maps: Vec<ValueMap> = vec![];
		let mut current_map_range: Option<Vec<ValueMapRange>> = None;

		let parse_values = |text: &str| {
			text
				.split(" ")
				.map(|v| v.parse::<i64>().unwrap())
				.collect::<Vec<_>>()
		};

        let mut lines = text.split("\r\n");
		loop {
			if let Some(line) = lines.next() {
				if line.is_empty() {
					continue;
				}
				if line.starts_with(SEEDS_HEADER) {
					let seed_values = &line[SEEDS_HEADER.len()..];
					match SeedParsingMode::from_char(M) {
						SeedParsingMode::Values => {
							let mut seed_values = parse_values(seed_values);

							seeds.append(&mut seed_values);
						},
						SeedParsingMode::RangePair => {
							let seed_ranges = parse_values(seed_values);
							let mut seed_range_iter = seed_ranges.iter();
							loop {
								if let Some(start) = seed_range_iter.next() {
									let length = seed_range_iter.next().unwrap();

									for i in 0..*length {
										seeds.push(start + i);
									}
								} else {
									break;
								}
							}
						},
					}
					
				} else if line.ends_with(MAP_HEADER_SUFFIX) {
					if let Some(range) = current_map_range.take() {
						maps.push(ValueMap::new(range));
					}
					current_map_range = Some(vec![]);
				} else {
					let range = ValueMapRange::from_str(line).unwrap();
					current_map_range.as_mut().unwrap().push(range);
				}
			} else {
				break;
			}
		}
		if let Some(range) = current_map_range.take() {
			maps.push(ValueMap::new(range));
		}

		Ok(Almanac::new(seeds, ChainedValueMap::new(maps)))
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_part_1() {
		let almanac = Almanac::<{SeedParsingMode::values_short_form()}>::from_str(&
r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#.replace("\n", "\r\n")
		).unwrap();

		let lowest_location = almanac.lowest_location();

		assert_eq!(35, lowest_location);
	}

	
	#[test]
	fn example_part_2() {
		let almanac = Almanac::<{SeedParsingMode::range_pair_short_form()}>::from_str(&
r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#.replace("\n", "\r\n")
		).unwrap();

		let lowest_location = almanac.lowest_location();

		assert_eq!(46, lowest_location);
	}
}