use clap::Parser;
use std::collections::HashSet;

mod primes;
pub mod utils;

#[derive(Parser)]
#[clap(author = "Garrett Simpson", about = "Goldbach's Conjecture Solver")]
struct CliArgs {
    /// The upper bound to test the conjecture
    #[clap(short = 'n')]
    n: u64,
    /// Stop running after the first even without a solution is found
    #[clap(short = 's', long = "stop")]
    stop: bool,
    /// Use a static list of primes from a file (delimited by newlines)
    #[clap(short = 'p', long = "primes-file")]
    primes_file: Option<String>,
    /// Algorithm to use to generate primes
    #[clap(short = 'a', long = "algorithm", value_enum)]
    algorithm: Option<primes::SieveAlgorithm>,
}

/// Solve the Goldbach's Conjecture for n
fn solve(n: u64, primes: &HashSet<u64>) -> Option<(u64, u64)> {
    for &p in primes {
        let complement = n.saturating_sub(p);
        if primes.contains(&complement) {
            return Some((p, complement));
        }
    }

    None
}

fn main() {
    let args = CliArgs::parse();
    let n = args.n;

    if n < 4 || n % 2 != 0 {
        println!("n must be >= 4 and even");
        return;
    }

    let primes: HashSet<u64>;

    if let Some(primes_file) = args.primes_file {
        primes = match primes::load_primes(&primes_file) {
            Ok(primes) => primes,
            Err(e) => {
                println!("Failed to load primes from {}: {}", primes_file, e);
                return;
            }
        };
    } else {
        primes = primes::sieve_primes(n, args.algorithm.unwrap_or(primes::SieveAlgorithm::Atkin));
    }

    for i in (4..=n).step_by(2) {
        let result = solve(i, &primes);

        match result {
            Some((p1, p2)) => println!("{} = {} + {}", i, p1, p2),
            None => {
                println!("{} has no solution", i);

                if args.stop {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let primes = HashSet::from([2, 3, 5, 7]);
        let result = solve(6, &primes);
        assert_eq!(result, Some((3, 3)), "Failed to find solution");
    }
}
