/// Euler's totient function gives us the number of coprimes on n.
/// We just need to sum them, and using mobius inversion is fast
/// The inversion:
/// https://en.wikipedia.org/wiki/Totient_summatory_function
/// Mobius functions:
/// https://mathworld.wolfram.com/MoebiusFunction.html
///
fn sieve_mobius(k: usize) -> Vec<i32> {
    let mut mobius = vec![1; k];
    let sieve = slow_primes::Primes::sieve(k);

    for p in sieve.primes() {
        for i in (p..k).step_by(p) {
            mobius[i] = 0 - mobius[i];
        }
        // println!("{p}");
    }
    for prime in sieve.primes() {
        let prod = prime * prime;
        for i in (prod..k).step_by(prod) {
            mobius[i] = 0;
        }
    }
    mobius
}

fn sum_mobious_once(n: i64, k: i64, mobius_value: i64) -> i64 {
    mobius_value * (n / k) * (1 + n / k)
}

fn check_part(n: u64) -> i64 {
    let triangle = (n * (n + 1) / 2) as i64;
    triangle
        - sieve_mobius((n + 1) as usize)
            .iter()
            .enumerate()
            .skip(1)
            .map(|(k, mobius_value)| sum_mobious_once(n as i64, k as i64, *mobius_value as i64))
            .sum::<i64>()
            / 2
}

fn main() {
    println!("{}", check_part(100_000_000) * 6);
}

#[test]
fn check_mobius() {
    assert_eq!(
        sieve_mobius(20),
        [1, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1]
    );
}

#[test]
fn check_part_5() {
    assert_eq!(check_part(5) * 6, 30);
}

#[test]
fn check_part_10() {
    assert_eq!(check_part(10) * 6, 138);
}

#[test]
fn check_part_1000() {
    assert_eq!(check_part(1000) * 6, 1177848);
}
