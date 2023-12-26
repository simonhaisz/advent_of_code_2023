use std::collections::HashMap;

use prime_factorization::Factorization;

pub fn lowest_common_multiple(numbers: &[u32]) -> u64 {
    let mut total_prime_factors: HashMap<u64, u32> = HashMap::new();

    for n in numbers.iter() {
        let mut number_prime_factors: HashMap<u64, u32> = HashMap::new();

        let factors_repr = Factorization::run(*n as u64);

        for p in factors_repr.factors.iter() {
            let count = number_prime_factors.entry(*p).or_insert(0);
            *count += 1;
        }

        for (prime, count) in number_prime_factors.iter() {
            let total_count = total_prime_factors.entry(*prime).or_insert(0);

            *total_count = (*total_count).max(*count);
        }
    }

    total_prime_factors.iter()
        .map(|(prime, count)| prime.pow(*count))
        .product()
}