use std::time::Instant;
use std::cmp;
use std::ops::Range;

fn sqrt(n: i32) -> f64 {
    f64::sqrt(n as f64)
}

fn gcd(mut p: i32, mut q: i32) -> i32 {
    while q != 0 {
        let new_q = p % q;
        p = q;
        q = new_q;
    }
    p
}

fn is_coprime(a: i32, b: i32) -> bool {
    return gcd(a, b) == 1
}

fn is_odd(a: i32) -> bool {
    a % 2 == 1
}

fn count_cuboids(limit: i32, a: i32, b: i32) -> i32 {
    let mut sum = 0;
    let max_k = (limit / a) + 1;
    for k in 1..max_k {
        let smaller = a * k;
        let bigger = b * k;
        if bigger <= limit {
            sum += smaller / 2;
        }
        let val = smaller - (bigger - 1) / 2;
        if val < 0 {
            continue;
        }
        sum += val;
    }
    sum
}

/// Generates the smaller elements in a pythagorean triplet.
/// Since `c` will be the biggest, this means it will be the distance.
/// Therefore we don't care about it
struct PythagoreanTripletGenerator {
    limit: i32,
}

impl PythagoreanTripletGenerator {
    fn new(limit: i32) -> Self {
        Self {
            limit,
        }
    }

    fn get_m_range(&self) -> Range<i32> {
        // For m bigger then this values, computed end_n will be smaller then init_n.
        // This will save us unneeded iterations
        let max_m = sqrt(self.limit * 2) as i32;
        1..max_m
    }

    fn get_n_range(&self, m: i32) -> Range<i32> {
        // skip `n`s where `a` will be too big
        let start_n = cmp::max(sqrt(m * m - self.limit * 2) as i32, 1);
        // with `n`s bigger then this, `b` will be too big
        let end_n = self.limit / m;
        start_n..end_n
    }

    fn will_produce_primitive_triplet(m: i32, n: i32) -> bool {
        is_coprime(m,n) && (is_odd(m) ^ is_odd(n))
    }

    fn generate_couple(m: i32, n: i32) -> (i32, i32) {
        let a = m * m - n * n;
        let b = 2 * m * n;
        (a, b)
    }

    fn generate_primitive(m: i32, n: i32) -> Option<(i32, i32)> {
        if !Self::will_produce_primitive_triplet(m, n) {
            return None;
        }
        let (a, b) = Self::generate_couple(m, n);
        let max = cmp::max(a, b);
        let min = cmp::min(a, b);
        Some((min, max))
    }
}

// generate Pythagorean triplets, and compute how many cuboids can be generated using this triplets
fn solve(limit: i32) -> i32 {
    let generator = PythagoreanTripletGenerator::new(limit);
    let mut total = 0;
    for m in generator.get_m_range() {
        for n in generator.get_n_range(m) {
            let (a, b) = match PythagoreanTripletGenerator::generate_primitive(m, n) {
                Some(val) => val,
                None => continue,
            };
            total += count_cuboids(limit, a, b);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_100() {
        assert_eq!(crate::solve(100), 2060);
    }

    #[test]
    fn check_99() {
        assert_eq!(crate::solve(99), 1975);
    }
}

fn main() {
    let now = Instant::now();

    let mut i = 0;
    let solution = loop {
        if solve(i) > 1_000_000 {
            break i;
        }
        i += 1;
    };

    let elapsed = now.elapsed();
    println!("{:?}", solution);
    println!("Elapsed: {:.2?}", elapsed);
}
