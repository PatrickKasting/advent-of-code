use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Primes {
    primes: Vec<usize>,
    current: usize,
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            primes: Vec::new(),
            current: 1,
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        while self.primes.iter().any(|prime| self.current % prime == 0) {
            self.current += 1;
        }
        self.primes.push(self.current);
        Some(self.current)
    }
}

pub fn prime_factors(mut number: usize) -> Vec<usize> {
    let prime_factor_upper_bound = (number as f64).sqrt() as usize;
    let mut prime_factors = Vec::new();
    for prime in Primes::new().take_while(|&prime| prime <= prime_factor_upper_bound) {
        while number % prime == 0 {
            prime_factors.push(prime);
            number /= prime;
        }
    }
    prime_factors
}

pub fn prime_factorization(number: usize) -> HashMap<usize, usize> {
    prime_factors(number)
        .group_by(|left, right| left == right)
        .map(|group| (group[0], group.len()))
        .collect()
}

pub fn least_common_multiple(numbers: impl IntoIterator<Item = usize>) -> usize {
    let prime_factorizations = numbers.into_iter().map(prime_factorization).collect_vec();
    let prime_factors: HashSet<usize> = prime_factorizations
        .iter()
        .flat_map(|prime_factorization| prime_factorization.keys().copied())
        .collect();

    let mut least_common_multiple = 1;
    for prime_factor in prime_factors {
        let maximum_occurrence = prime_factorizations
            .iter()
            .map(|prime_factorization| {
                prime_factorization
                    .get(&prime_factor)
                    .copied()
                    .unwrap_or_default()
            })
            .max()
            .expect("least common multiple should be computed from at least one number");
        least_common_multiple *= prime_factor.pow(maximum_occurrence as u32);
    }
    least_common_multiple
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factors_of_12() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
    }

    #[test]
    fn prime_factors_of_30() {
        assert_eq!(prime_factors(30), vec![2, 3, 5]);
    }

    #[test]
    fn prime_factors_of_49() {
        assert_eq!(prime_factors(49), vec![7, 7]);
    }

    #[test]
    fn least_common_multiple_of_24_and_300() {
        assert_eq!(least_common_multiple([24, 300]), 600);
    }

    #[test]
    fn least_common_multiple_of_12_18_and_30() {
        assert_eq!(least_common_multiple([12, 18, 30]), 180);
    }
}
