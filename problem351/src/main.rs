use itertools::Itertools;
use rayon::prelude::*;
use std::hint::black_box;

fn list_divisors(mut k: u32) -> Vec<u32> {
    let mut divisors = Vec::new();
    divisors.reserve(8);
    let mut i = 2;
    while k != 1 {
        if k % i == 0 {
            divisors.push(i);
            while k % i == 0 {
                k /= i;
            }
        }
        i += 1;
    }
    divisors
}

fn count_divisible_by_divisors(k: u32, limit: u32) -> u32 {
    let mut not_divisible = 0;
    for divisors in list_divisors(k).into_iter().powerset() {
        let product = divisors.iter().product::<u32>();
        if divisors.len() == 0 {
            continue;
        } else if divisors.len() & 1 == 0 {
            not_divisible -= (limit / product) as i32;
            // println!("- {}", limit / product);
        } else {
            not_divisible += (limit / product) as i32;
            // println!("+ {}", limit / product);
        }
    }
    not_divisible as u32
}

fn check_part(size: u32) -> u64 {
    let mut hidden = (size - 1) as u64;
    hidden += (2..size)
        .into_par_iter()
        .map(|i| {
            if i % 10_000 == 0 {
                println!("{i}");
            }
            count_divisible_by_divisors(i, size - i) as u64
        })
        .sum::<u64>();
    hidden
}

fn main() {
    println!("{}", check_part(100_000));
}

#[test]
fn not_divisible_by_1() {
    assert_eq!(count_divisible_by_divisors(1, 5), 0);
    assert_eq!(count_divisible_by_divisors(1, 70), 0);
}

#[test]
fn not_divisible_by_2() {
    assert_eq!(count_divisible_by_divisors(2, 1), 0);
    assert_eq!(count_divisible_by_divisors(2, 2), 1);
    assert_eq!(count_divisible_by_divisors(2, 3), 1);
    assert_eq!(count_divisible_by_divisors(2, 4), 2);
    assert_eq!(count_divisible_by_divisors(2, 70), 35);
}

#[test]
fn not_divisible_by_3() {
    assert_eq!(count_divisible_by_divisors(3, 1), 0);
    assert_eq!(count_divisible_by_divisors(3, 2), 0);
    assert_eq!(count_divisible_by_divisors(3, 3), 1);
    assert_eq!(count_divisible_by_divisors(3, 4), 1);
    assert_eq!(count_divisible_by_divisors(3, 70), 23);
}

#[test]
fn not_divisible_by_6() {
    assert_eq!(count_divisible_by_divisors(6, 10), 7);
    assert_eq!(count_divisible_by_divisors(6, 11), 7);
    assert_eq!(count_divisible_by_divisors(6, 12), 8);
    assert_eq!(count_divisible_by_divisors(6, 13), 8);
    assert_eq!(count_divisible_by_divisors(6, 14), 9);
    assert_eq!(count_divisible_by_divisors(6, 15), 10);
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
