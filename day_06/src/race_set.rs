use std::str::FromStr;

use crate::race::RaceResult;

pub struct RaceResultSet {
    results: Vec<RaceResult>,
}

impl RaceResultSet {
    pub fn new(results: Vec<RaceResult>) -> Self {
        Self { results }
    }

    pub fn winner_count_multiple(&self) -> u64 {
        self.results
            .iter()
            .map(|r| r.compute_winners())
            .product()
    }
}

#[derive(Debug)]
pub struct RaceResultSetParseError;

impl FromStr for RaceResultSet {
    type Err = RaceResultSetParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut times = None;
        let mut distances = None;

        for line in text.lines() {
            let (prefix, collection) = if times.is_none() {
                ("Time:", &mut times)
            } else if distances.is_none() {
                ("Distance:", &mut distances)
            } else {
                panic!()
            };

            if let Some(0) = line.find(prefix) {
                let values = line[prefix.len()..]
                    .trim()
                    .split(" ")
                    .map(|entry| entry.trim())
                    .filter(|entry| !entry.is_empty())
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                collection.replace(values);
            } else {
                eprintln!("Failed to find '{}' at beginning of line '{}'", prefix, line);
                return Err(RaceResultSetParseError);
            }
        }

        let times = times.unwrap();
        let distances = distances.unwrap();

        let count = times.len();

        assert_eq!(count, distances.len());

        let mut results = vec![];
        for i in 0..count {
            let time = times[i];
            let distance = distances[i];
            results.push(RaceResult::new(time, distance));
        }

        Ok(RaceResultSet::new(results))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_example() {
        let text = r"
Time:      7  15   30
Distance:  9  40  200
        ";

        let result_set = RaceResultSet::from_str(text.trim()).unwrap();

        assert_eq!(3, result_set.results.len());

        assert_eq!(RaceResult::new(7, 9), result_set.results[0]);
        assert_eq!(RaceResult::new(15, 40), result_set.results[1]);
        assert_eq!(RaceResult::new(30, 200), result_set.results[2]);
    }

    #[test]
    fn multiple_example() {
        let text = r"
Time:      7  15   30
Distance:  9  40  200
";
        
        let result_set = RaceResultSet::from_str(text.trim()).unwrap();

        assert_eq!(288, result_set.winner_count_multiple());
    }
}