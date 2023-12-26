pub type Sequence = Vec<i64>;

pub trait Difference {
    fn difference(&self) -> Sequence;
}

impl Difference for Sequence {
    fn difference(&self) -> Sequence {
        let mut diff = vec![];

        let mut it = self.iter();

        let mut a = it.next().unwrap();
        let mut b = it.next().unwrap();

        loop {
            let d = *b - *a;
            diff.push(d);

            if let Some(next) = it.next() {
                a = b;
                b = next;
            } else {
                break;
            }
        }

        diff
    }
}

pub trait AllZeroes {
    fn all_zeroes(&self) -> bool;
}

impl AllZeroes for Sequence {
    fn all_zeroes(&self) -> bool {
        self.iter()
            .all(|n| *n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_0() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        let difference = sequence.difference();

        assert_eq!(vec![3, 3, 3, 3, 3], difference)
    }

    #[test]
    fn example_1_1() {
        let sequence = vec![3, 3, 3, 3, 3];
        let difference = sequence.difference();

        assert_eq!(vec![0, 0, 0, 0], difference)
    }

    #[test]
    fn example_2_0() {
        let sequence = vec![1, 3, 6, 10, 15, 21];
        let difference = sequence.difference();

        assert_eq!(vec![2, 3, 4, 5, 6], difference)
    }

    #[test]
    fn example_2_1() {
        let sequence = vec![2, 3, 4, 5, 6];
        let difference = sequence.difference();

        assert_eq!(vec![1, 1, 1, 1], difference);
    }

    #[test]
    fn example_2_2() {
        let sequence = vec![1, 1, 1, 1];
        let difference = sequence.difference();

        assert_eq!(vec![0, 0, 0], difference)
    }

    #[test]
    fn example_3_0() {
        let sequence = vec![10, 13, 16, 21, 30, 45];
        let difference = sequence.difference();

        assert_eq!(vec![3, 3, 5, 9, 15], difference);
    }

    #[test]
    fn example_3_1() {
        let sequence = vec![3, 3, 5, 9, 15];
        let difference = sequence.difference();

        assert_eq!(vec![0, 2, 4, 6], difference);
    }

    #[test]
    fn example_3_2() {
        let sequence = vec![0, 2, 4, 6];
        let difference = sequence.difference();

        assert_eq!(vec![2, 2, 2], difference);
    }

    #[test]
    fn example_3_3() {
        let sequence = vec![2, 2, 2];
        let difference = sequence.difference();

        assert_eq!(vec![0, 0], difference);
    }
}