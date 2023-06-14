use primes::{PrimeSet, Sieve};
use std::collections::{BTreeSet, HashMap};

fn sqrt(n: u32) -> u32 {
    f64::sqrt(n as f64) as u32 + 1
}

struct MinFactors {
    lengths: Vec<u32>,
}

impl MinFactors {
    fn update(&mut self, num: u32, factorisation_lengths: &BTreeSet<u32>) {
        for factorisation_length in factorisation_lengths {
            let factorisation_length = *factorisation_length as usize;

            let current = self.lengths.get(factorisation_length);
            if current.is_none() {
                    continue;
            }
            let current = *current.unwrap();

            if current == 0 || current > num {
                self.lengths[factorisation_length] = num;
            }
        }
    }
}

struct NCalc {
    factorisation_lengths: HashMap<u32, BTreeSet<u32>>,
    min_factors: MinFactors,
    prime_set: Sieve,
}

impl NCalc {
    fn new() -> Self {
        Self {
            factorisation_lengths: HashMap::new(),
            min_factors: MinFactors {
                lengths: Vec::new(),
            },
            prime_set: Sieve::new(),
        }
    }

    fn sum_min_product_sums(&mut self, k: u32) -> u32 {
        self.sieve_factorisation_lengths(k);
        self.update_min_factors(k);

        let mut sum = 0u32;
        let mut seen = BTreeSet::<u32>::new();
        for min_factor_lnegth in &self.min_factors.lengths[2..k as usize+1] {
            if !seen.contains(min_factor_lnegth) {
                sum += min_factor_lnegth;
            }
            seen.insert(*min_factor_lnegth);
        }
        sum
    }

    fn update_min_factors(&mut self, k: u32) {
        self.min_factors.lengths = vec!(0; k as usize + 1);
        for (num, factorisation_lengths) in &self.factorisation_lengths {
            self.min_factors.update(*num, &factorisation_lengths);
        }
    }

    fn sieve_factorisation_lengths(&mut self, k: u32) {
        for num in 4..k*2 {
            let a_set = self.calc_factorisation_lengths(num);
            if let Some(set) = a_set {
                self.factorisation_lengths.insert(num, set);
            }
        }

    }

    fn calc_factorisation_lengths(&mut self, num: u32) -> Option<BTreeSet<u32>> {
        if self.is_prime(num) {
            return None;
        }
        let mut set = BTreeSet::<u32>::new();
        for i in 2..sqrt(num) {
            if num % i == 0 {
                self.add_ns(&mut set, num, num / i);
            }
        }
        Some(set)
    }

    fn is_prime(&mut self, n: u32) -> bool {
        self.prime_set.is_prime(n.into())
    }

    fn add_ns(&mut self, set: &mut BTreeSet<u32>, num: u32, remainder: u32) {
        let n = calc_n(num, remainder);
        set.insert(n);
        if let Some(subset) = self.factorisation_lengths.get(&remainder) {
            for sub_n in subset {
                set.insert(n + sub_n - 1);
            }
        }
    }
}

fn calc_n(num: u32, remainder: u32) -> u32 {
    return count_ones(num, remainder) + 2;
}

fn count_ones(num: u32, remainder: u32) -> u32 {
    let factor = num / remainder;
    let ones = num - remainder - factor;
    ones
}

fn main() {
    use std::time::Instant;

    let now = Instant::now();
    let mut calc = NCalc::new();
    let solution = calc.sum_min_product_sums(12_000);
    let elapsed = now.elapsed();

    println!("{}", solution);
    println!("Elapsed: {:.2?}", elapsed);
}
