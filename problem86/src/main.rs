use std::time::Instant;
use std::cmp;

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

fn count_primitve_cuboids(limit: i32, a: i32, b: i32) -> i32 {
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
}

impl PythagoreanTripletGenerator {
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
    let mut total = 0;
    let mut j = 0;
    // from certain j, end_i will be smaller then init_i
    let max_j = sqrt(limit * 2) as i32;
    while j < max_j {
        j += 1;
        let init_i = cmp::max(sqrt(j * j - limit * 2) as i32, 1); // skip i's where `a` will be too big
        let end_i = limit / j; // with i's bigger then this, `b` will be too big
        for i in init_i..end_i {
            let (a, b) = match PythagoreanTripletGenerator::generate_primitive(j, i) {
                Some(val) => val,
                None => continue,
            };
            if a > limit {
                continue;
            }
            total += count_primitve_cuboids(limit, a, b);
        }
    }
    total
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
