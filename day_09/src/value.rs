use std::str::FromStr;

use crate::sequence::{Sequence, Difference, AllZeroes};

pub struct Value {
    history: Sequence,
}

impl Value {
    pub fn next(&self) -> i64 {
        let mut current = self.history.clone();

        let mut last_value = vec![*current.last().unwrap()];

        loop {
            let next = current.difference();

            if next.all_zeroes() {
                break;
            }

            last_value.push(*next.last().unwrap());

            current = next;
        }
        last_value.iter().sum()
    }

    pub fn total_next(values: &[Value]) -> i64 {
        values.iter()
            .map(|v| v.next())
            .sum()
    }

    pub fn previous(&self) -> i64 {
        let mut current = self.history.clone();

        let mut first_value = vec![*current.first().unwrap()];

        loop {
            let next = current.difference();

            if next.all_zeroes() {
                break;
            }

            first_value.push(*next.first().unwrap());

            current = next;
        }

        first_value.reverse();
        
        first_value.into_iter()
            .reduce(|total, n| n - total).unwrap()
    }

    pub fn total_previous(values: &[Value]) -> i64 {
        values.iter()
            .map(|v: &Value| v.previous())
            .sum()
    }
}

#[derive(Debug)]
pub struct ParseValueError;

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let history = line.split(" ")
            .map(|t| t.trim())
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Value { history })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_example_1() {
        let value = Value::from_str("0 3 6 9 12 15").unwrap();

        assert_eq!(18, value.next());
    }

    #[test]
    fn next_example_2() {
        let value = Value::from_str("1 3 6 10 15 21").unwrap();

        assert_eq!(28, value.next());
    }

    #[test]
    fn next_example_3() {
        let value = Value::from_str("10 13 16 21 30 45").unwrap();

        assert_eq!(68, value.next());
    }

    #[test]
    fn previous_example_1() {
        let value = Value::from_str("0 3 6 9 12 15").unwrap();

        assert_eq!(-3, value.previous());
    }

    #[test]
    fn previous_example_2() {
        let value = Value::from_str("1 3 6 10 15 21").unwrap();

        assert_eq!(0, value.previous());
    }

    #[test]
    fn previous_example_3() {
        let value = Value::from_str("10 13 16 21 30 45").unwrap();

        assert_eq!(5, value.previous());
    }
}