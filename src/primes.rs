use clap::ValueEnum;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, ValueEnum)]
pub enum SieveAlgorithm {
    Eratosthenes,
    Atkin,
}

impl Display for SieveAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SieveAlgorithm::Eratosthenes => write!(f, "Eratosthenes"),
            SieveAlgorithm::Atkin => write!(f, "Atkin"),
        }
    }
}

/// Load primes from a static file delimited by newlines
pub fn load_primes(file_path: &str) -> io::Result<HashSet<u64>> {
    let mut primes = HashSet::new();
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let prime = line?.parse::<u64>();

        if let Ok(prime) = prime {
            if primes.contains(&prime) {
                continue;
            }

            primes.insert(prime);
        } else {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
    }

    Ok(primes)
}

/// Generate a list of primes up to n using the specified algorithm
pub fn sieve_primes(n: u64, algo: SieveAlgorithm) -> HashSet<u64> {
    match algo {
        SieveAlgorithm::Eratosthenes => sieve_eratosthenes(n),
        SieveAlgorithm::Atkin => sieve_atkin(n),
    }
}

/// Generate a list of primes up to n using the Sieve of Atkin
fn sieve_atkin(n: u64) -> HashSet<u64> {
    let mut is_prime = vec![false; (n + 1) as usize];
    let sqrt_n = n.isqrt();
    let mut primes = HashSet::new();

    for x in 1..=sqrt_n {
        for y in 1..=sqrt_n {
            let x_sq = x * x;
            let y_sq = y * y;
            let mut num = 4 * x_sq + y_sq;

            if num <= n && (num % 12 == 1 || num % 12 == 5) {
                is_prime[num as usize] ^= true;
            }

            num = 3 * x_sq + y_sq;
            if num <= n && num % 12 == 7 {
                is_prime[num as usize] ^= true;
            }

            if x > y {
                num = 3 * x_sq - y_sq;
                if num <= n && num % 12 == 11 {
                    is_prime[num as usize] ^= true;
                }
            }
        }
    }

    for i in 5..=n {
        if !is_prime[i as usize] {
            continue;
        }

        for j in ((i * i)..=n).step_by((i * i) as usize) {
            is_prime[j as usize] = false;
        }
    }

    // Sieve of Atkin does not include 2 or 3
    if n >= 2 {
        primes.insert(2);
    }
    if n >= 3 {
        primes.insert(3);
    }

    for i in 5..=n {
        if is_prime[i as usize] {
            primes.insert(i);
        }
    }

    primes
}

/// Generate a list of primes up to n using the Sieve of Eratosthenes
fn sieve_eratosthenes(n: u64) -> HashSet<u64> {
    let mut is_prime = vec![true; (n + 1) as usize];
    let mut primes = HashSet::new();

    for i in 2..=n {
        if !is_prime[i as usize] {
            continue;
        }

        primes.insert(i);
        for j in (i * i..=n).step_by(i as usize) {
            is_prime[j as usize] = false;
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parametrized_test;

    #[test]
    fn test_load_primes() {
        let primes = load_primes("test_data/primes.txt").unwrap();
        assert!(primes.contains(&2), "Failed to find prime 2");
        assert!(primes.contains(&3), "Failed to find prime 3");
        assert!(primes.contains(&5), "Failed to find prime 5");
        assert!(primes.contains(&7), "Failed to find prime 7");
    }

    #[test]
    fn test_load_primes_invalid() {
        let primes = load_primes("test_data/invalid_primes.txt");
        assert!(
            primes.is_err(),
            "Returned primes from invalid file should be an error"
        );
    }

    parametrized_test!(
        test_sieve_primes,
        |algo: SieveAlgorithm| {
            let primes = sieve_primes(10, algo);
            assert!(primes.contains(&2), "{}: Failed to find prime 2", algo);
            assert!(primes.contains(&3), "{}: Failed to find prime 3", algo);
            assert!(primes.contains(&5), "{}: Failed to find prime 5", algo);
            assert!(primes.contains(&7), "{}: Failed to find prime 7", algo);
        },
        SieveAlgorithm::Eratosthenes,
        SieveAlgorithm::Atkin
    );

    parametrized_test!(
        test_sieve_primes_large,
        |algo: SieveAlgorithm| {
            let primes = sieve_primes(5_000, algo);
            assert!(primes.contains(&769), "{}: Failed to find prime 769", algo);
            assert!(
                primes.contains(&4231),
                "{}: Failed to find prime 4231",
                algo
            );
        },
        SieveAlgorithm::Eratosthenes,
        SieveAlgorithm::Atkin
    );
}
