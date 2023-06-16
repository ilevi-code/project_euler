use itertools::Itertools;
use std::time::Instant;

struct ArthmeticSequenceGenerator {
    i: i32,
}

impl ArthmeticSequenceGenerator {
    fn new() -> Self {
        Self {
            i: 1,
        }
    }

    fn calc_n(n: i32) -> i32 {
        return (n + 1) * n / 2;
    }
}

impl Iterator for ArthmeticSequenceGenerator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Self::calc_n(self.i);
        self.i += 1;
        Some(res)
    }
}

fn diff(target: i32, v: &Vec<&i32>) -> Option<i32> {
    let [a, b] = v[..] else { return None; };
    match a.checked_mul(*b) {
        Some(product) => Some(target - product),
        None => None
    }
}

fn closer_product<'a>(target: i32, a: Vec<&'a i32>, b: Vec<&'a i32>) -> Vec<&'a i32> {
    let a_diff = match diff(target, &a) {
        Some(diff) => diff,
        None => return b,
    };
    let b_diff = match diff(target, &b) {
        Some(diff) => diff,
        None => return a,
    };
    if a_diff.abs() < b_diff.abs() {
        a
    } else {
        b
    }
}

fn solve(target: i32) -> Option<(usize, usize)> {
    let seq = ArthmeticSequenceGenerator::new();
    let sums: Vec<_> = seq.take_while(|x| x < &target).collect();
    let found = sums.iter().permutations(2).reduce(
        |a, b| closer_product(target, a, b)
    );
    let [sum_m, sum_n] = &found?[..] else { return None };
    let m = sums.iter().position(|&x| x == **sum_m)? + 1;
    let n = sums.iter().position(|&x| x == **sum_n)? + 1;
    Some((m, n))
}

fn main() {
    let now = Instant::now();
    let (m, n) = match solve(2_000_000) {
        Some(val) => val,
        None => { println!("No solution found"); return }
    };
    let elapsed = now.elapsed();

    println!("{m}*{n}={:?}", m * n);
    println!("Elapsed: {:.2?}", elapsed);
}
