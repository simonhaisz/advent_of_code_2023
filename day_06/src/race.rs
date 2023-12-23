#[derive(Debug, PartialEq)]
pub struct RaceResult {
    time: u32,
    distance: u32,
}

impl RaceResult {
    pub fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }

    pub fn compute_winners(&self) -> Vec<Race> {
        let mut winners = vec![];

        for speed in 1..self.time {
            let race = Race::new(self.time, speed);
            let result = race.result();
            if result.distance > self.distance {
                winners.push(race);
            }
        }

        winners
    }
}

pub struct Race {
    time: u32,
    speed: u32,
}

impl Race {
    pub fn new(time: u32, speed: u32) -> Self {
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
        assert_eq!(4, record.compute_winners().len());
    }

    #[test]
    fn example_record_2() {
        let record = RaceResult::new(15, 40);
        assert_eq!(8, record.compute_winners().len());
    }

    #[test]
    fn example_record_3() {
        let record = RaceResult::new(30, 200);
        assert_eq!(9, record.compute_winners().len());
    }
}