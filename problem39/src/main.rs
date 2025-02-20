use std::collections::VecDeque;
use std::time::Instant;

fn pythagorean_triplet_from_coprimes(m: u32, n: u32) -> (u32, u32, u32) {
    (m * m - n * n, 2 * m * n, m * m + n * n)
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn main() {
    let start = Instant::now();

    const LIMIT: u32 = 1000;
    let mut perimiters = [0; LIMIT as usize];
    let mut coprimes = VecDeque::from([(2, 1), (3, 1)]);
    let mut best = 0;

    while let Some((m, n)) = coprimes.pop_front() {
        let (a, b, c) = pythagorean_triplet_from_coprimes(m, n);
        // If any of the sides exceed the limit, we don't want any more coprimes from this pair
        if a < LIMIT && b < LIMIT {
            coprimes.push_back((2 * m - n, m));
            coprimes.push_back((2 * m + n, m));
            coprimes.push_back((m + 2 * n, n));
        }
        // 1. For generation of pythagorean triplet to work from coprimes - at least one of them
        //    must be even.
        // 2. We don't want to count same triplets twice, so we ignore any triplet that isn't primitive
        if m % 2 == 0 || n % 2 == 0 && gcd(a, b) == 1 {
            let perimiter = a + b + c;
            // Account for all non-primitive
            for k in 1..((LIMIT - 1) / perimiter + 1) {
                let perimiter = (perimiter * k) as usize;
                perimiters[perimiter] += 1;

                // Check if we have a new best
                if perimiters[best] < perimiters[perimiter] {
                    best = perimiter;
                }
            }
        }
    }

    println!("{} in {:?}", best, start.elapsed());
}
