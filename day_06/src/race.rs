use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct RaceResult {
    time: u64,
    distance: u64,
}

impl RaceResult {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    pub fn compute_winners(&self) -> u64 {
        let mut winners = 0;

        for speed in 1..self.time {
            let race = Race::new(self.time, speed);
            let result = race.result();
            if result.distance > self.distance {
                winners += 1;
            }
        }

        winners
    }
}

#[derive(Debug)]
pub struct RaceResultParseError;

impl FromStr for RaceResult {
    type Err = RaceResultParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut time = None;
        let mut distance = None;

        for line in text.lines() {
            let (prefix, value) = if time.is_none() {
                ("Time:", &mut time)
            } else if distance.is_none() {
                ("Distance:", &mut distance)
            } else {
                panic!()
            };

            if let Some(0) = line.find(prefix) {
                let digits = line[prefix.len()..]
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();

                value.replace(digits.parse::<u64>().unwrap());
            } else {
                eprintln!("Failed to find '{}' at beginning of line '{}'", prefix, line);
                return Err(RaceResultParseError);
            }
        }

        let time = time.unwrap();
        let distance = distance.unwrap();

        Ok(RaceResult::new(time, distance))
    }
}

pub struct Race {
    time: u64,
    speed: u64,
}

impl Race {
    pub fn new(time: u64, speed: u64) -> Self {
        Self { time, speed }
    }

    pub fn result(&self) -> RaceResult {
        let distance = (self.time - self.speed) * self.speed;

        RaceResult::new(self.time, distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_record_1() {
        let record = RaceResult::new(7, 9);
        assert_eq!(4, record.compute_winners());
    }

    #[test]
    fn example_record_2() {
        let record = RaceResult::new(15, 40);
        assert_eq!(8, record.compute_winners());
    }

    #[test]
    fn example_record_3() {
        let record = RaceResult::new(30, 200);
        assert_eq!(9, record.compute_winners());
    }

    #[test]
    fn example_part_2() {
        let record = RaceResult::new(71530, 940200);

        assert_eq!(71503, record.compute_winners());
    }
}