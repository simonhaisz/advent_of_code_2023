use std::str::FromStr;

pub struct ValueMapRange {
	source: (i64, i64),
	destination_delta: i64,
}

impl ValueMapRange {
	pub fn new(source_start: i64, destination_start: i64, length: i64) -> Self {
		Self { source: (source_start, source_start + length), destination_delta: destination_start - source_start }
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

		Ok(ValueMapRange::new(source_start, destination_start, length))
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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_seed_to_soil() {
		let map = ValueMap::new(vec![
			ValueMapRange::new(98, 59, 2),
			ValueMapRange::new(50, 52, 48),
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
			ValueMapRange::new(98, 59, 2),
			ValueMapRange::new(50, 52, 48),
		]);
		let soil_fertilizer = ValueMap::new(vec![
			ValueMapRange::new(15, 0, 37),
			ValueMapRange::new(52, 37, 2),
			ValueMapRange::new(0, 39, 15),
		]);
		let fertilizer_water = ValueMap::new(vec![
			ValueMapRange::new(53, 49, 8),
			ValueMapRange::new(11, 0, 42),
			ValueMapRange::new(0, 42, 7),
			ValueMapRange::new(7, 57, 4),
		]);
		let water_light = ValueMap::new(vec![
			ValueMapRange::new(18, 88, 7),
			ValueMapRange::new(25, 18, 70),
		]);
		let light_temperature = ValueMap::new(vec![
			ValueMapRange::new(77, 45, 23),
			ValueMapRange::new(45, 81, 19),
			ValueMapRange::new(64, 68, 13),
		]);
		let temperature_humidity = ValueMap::new(vec![
			ValueMapRange::new(69, 0, 1),
			ValueMapRange::new(0, 1, 69),
		]);
		let humidity_location = ValueMap::new(vec![
			ValueMapRange::new(56, 60, 37),
			ValueMapRange::new(93, 56, 4),
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
}