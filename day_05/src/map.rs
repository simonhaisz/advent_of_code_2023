use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Range(pub i64, pub i64);

impl Range {
	pub fn intersect(&self, other: &Range) -> Option<Range> {
		/*
		let before = if self.0 < other.0 {
			let start = self.0;
			let end = self.1.min(other.0);
			Some(Range(start, end))
		} else {
			None
		};
		*/
		if self.0 < other.1 && self.1 > other.0 {
			let start = self.0.max(other.0);
			let end = self.1.min(other.1);
			Some(Range(start, end))
		} else {
			None
		}

		/*
		let after = if self.1 > other.1 {
			let start = self.0.max(other.1);
			let end = self.1;
			Some(Range(start, end))
		} else {
			None
		};
		*/
		// (before, intersection, after)
	}
}

pub struct ValueMapRange {
	source: Range,
	destination_delta: i64,
}

impl ValueMapRange {
	pub fn new_from_str(source_start: i64, destination_start: i64, length: i64) -> Self {
		Self { source: Range(source_start, source_start + length), destination_delta: destination_start - source_start }
	}

	pub fn new(source: Range, destination_delta: i64) -> Self {
		Self { source, destination_delta }
	}
}

#[derive(Debug)]
pub struct ValueMapRangeParseError;

impl FromStr for ValueMapRange {
    type Err = ValueMapRangeParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let values = text
			.split(" ")
			.map(|v| v.parse::<i64>().unwrap())
			.collect::<Vec<_>>();
		assert_eq!(3, values.len());

		let source_start = values[1];
		let destination_start = values[0];
		let length = values[2];

		Ok(ValueMapRange::new_from_str(source_start, destination_start, length))
    }
}

pub struct ValueMap {
	ranges: Vec<ValueMapRange>,
}

impl ValueMap {
	pub fn new(mut ranges: Vec<ValueMapRange>) -> Self {
		ranges.sort_by(|a, b| a.source.0.cmp(&b.source.0));
		Self { ranges }
	}

	pub fn map(&self, source: i64) -> i64 {
		for r in self.ranges.iter() {
			if source >= r.source.0 && source < r.source.1 {
				return source + r.destination_delta;
			}
		}

		source
	}

	pub fn map_range(&self, sources: Vec<Range>) -> Vec<Range> {
		let mut temp_value_maps = vec![];

		for s in sources.iter() {
			for m in self.ranges.iter() {
				let intersection = s.intersect(&m.source);

				if let Some(intersection) = intersection {
					temp_value_maps.push(ValueMapRange::new(intersection.clone(), m.destination_delta));
				}
			}
		}

		temp_value_maps.sort_by(|a, b| a.source.0.cmp(&b.source.0));

		let mut missing_ranges = vec![];

		for s in sources.iter() {
			
		}

		mapped_ranges
	}
}

pub struct ChainedValueMap {
	maps: Vec<ValueMap>,
}

impl ChainedValueMap {
	pub fn new(maps: Vec<ValueMap>) -> Self {
		Self { maps }
	}

	pub fn map(&self, source: i64) -> i64 {
		let mut current_source = source;
		let mut current_destination = 0;
		for m in self.maps.iter() {
			current_destination = m.map(current_source);

			current_source = current_destination;
		}

		current_destination
	}

	pub fn map_range(&self, source: Range) -> Vec<Range> {

		let mut current_source = vec![source];
		let mut current_destinations = vec![];

		for m in self.maps.iter() {
			current_destinations.clear();
			for s in current_source {
				let mut d = m.map_range(s);
				current_destinations.append(&mut d);
			}
			current_source = current_destinations.clone();
		}

		current_destinations
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_seed_to_soil() {
		let map = ValueMap::new(vec![
			ValueMapRange::new_from_str(98, 59, 2),
			ValueMapRange::new_from_str(50, 52, 48),
		]);

		let soil = map.map(79);
		assert_eq!(81, soil);

		let soil = map.map(14);
		assert_eq!(14, soil);

		let soil = map.map(55);
		assert_eq!(57, soil);

		let soil = map.map(13);
		assert_eq!(13, soil);
	}

	#[test]
	fn example_seed_to_location() {
		let seed_soil = ValueMap::new(vec![
			ValueMapRange::new_from_str(98, 59, 2),
			ValueMapRange::new_from_str(50, 52, 48),
		]);
		let soil_fertilizer = ValueMap::new(vec![
			ValueMapRange::new_from_str(15, 0, 37),
			ValueMapRange::new_from_str(52, 37, 2),
			ValueMapRange::new_from_str(0, 39, 15),
		]);
		let fertilizer_water = ValueMap::new(vec![
			ValueMapRange::new_from_str(53, 49, 8),
			ValueMapRange::new_from_str(11, 0, 42),
			ValueMapRange::new_from_str(0, 42, 7),
			ValueMapRange::new_from_str(7, 57, 4),
		]);
		let water_light = ValueMap::new(vec![
			ValueMapRange::new_from_str(18, 88, 7),
			ValueMapRange::new_from_str(25, 18, 70),
		]);
		let light_temperature = ValueMap::new(vec![
			ValueMapRange::new_from_str(77, 45, 23),
			ValueMapRange::new_from_str(45, 81, 19),
			ValueMapRange::new_from_str(64, 68, 13),
		]);
		let temperature_humidity = ValueMap::new(vec![
			ValueMapRange::new_from_str(69, 0, 1),
			ValueMapRange::new_from_str(0, 1, 69),
		]);
		let humidity_location = ValueMap::new(vec![
			ValueMapRange::new_from_str(56, 60, 37),
			ValueMapRange::new_from_str(93, 56, 4),
		]);

		let chained_map = ChainedValueMap::new(vec![
			seed_soil,
			soil_fertilizer,
			fertilizer_water,
			water_light,
			light_temperature,
			temperature_humidity,
			humidity_location,
		]);

		let location = chained_map.map(79);
		assert_eq!(82, location);

		let location = chained_map.map(14);
		assert_eq!(43, location);

		let location = chained_map.map(55);
		assert_eq!(86, location);

		let location = chained_map.map(13);
		assert_eq!(35, location);
	}

	#[test]
	fn intersect_before() {
		let a = Range(0, 5);
		let b = Range(10, 20);

		let intersection = a.intersect(&b);

		assert_eq!(None, intersection);
	}

	#[test]
	fn intersect_after() {
		let a = Range(30, 35);
		let b = Range(10, 20);

		let intersection = a.intersect(&b);

		assert_eq!(None, intersection);
	}

	#[test]
	fn intersect_equal() {
		let a = Range(10, 20);
		let b = Range(10, 20);

		let intersection = a.intersect(&b);

		assert_eq!(Range(10, 20), intersection.unwrap());
	}

	#[test]
	fn intersect_beginning() {
		let a = Range(5, 15);
		let b = Range(10, 20);

		let intersection = a.intersect(&b);

		assert_eq!(Range(10, 15), intersection.unwrap());
	}

	#[test]
	fn map_range() {
		let seeds = Range(79, 79 + 14);

		let seed_to_soil = ValueMap::new(vec![
			ValueMapRange::from_str("50 98 2").unwrap(),
			ValueMapRange::from_str("52 50 48").unwrap(),
		]);
		
		let soils = seed_to_soil.map_range(seeds);

		assert_eq!(1, soils.len());
		assert_eq!(Range(81, 95), soils[0]);

		let soil_to_fertilizer = ValueMap::new(vec![
			ValueMapRange::from_str("0 15 8").unwrap(),
			ValueMapRange::from_str("37 52 2").unwrap(),
			ValueMapRange::from_str("39 0 15").unwrap(),
		]);

		let fertilizers = soil_to_fertilizer.map_range(soils[0].clone());

		assert_eq!(1, fertilizers.len());
		assert_eq!(Range(81, 95), fertilizers[0]);
	}
}