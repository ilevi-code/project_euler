use primes::{PrimeSet, Sieve};
use std::collections::BTreeSet;

fn list_powers(limit: u64, power: u32) -> Vec<u64> {
    let mut primes = Sieve::new();
    let mut nums = Vec::new();

    for power in primes.iter().map(|x| x.pow(power)).take_while(|x| x < &limit) {
        nums.push(power);
    }

    nums
}

fn count_prime_power_triples(limit: u64) -> usize {
    let squares = list_powers(limit, 2);
    let cubes = list_powers(limit, 3);
    let forths = list_powers(limit, 4);
    let mut seen = BTreeSet::<u64>::new();

    for forth in &forths {
        for cube in &cubes {
            for square in &squares {
                let sum = forth + cube + square;
                if sum > limit {
                    break;
                }
                seen.insert(sum);
            }
        }
    }

    seen.len()
}

fn main() {
    use std::time::Instant;

    let now = Instant::now();
    let solution = count_prime_power_triples(50_000_000);
    let elapsed = now.elapsed();

    println!("{}", solution);
    println!("Elapsed: {:.2?}", elapsed);
}
