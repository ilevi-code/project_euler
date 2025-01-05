use std::collections::VecDeque;

fn check_part(k: u32) -> u64 {
    let mut v: VecDeque<(u32, u32)> = VecDeque::new();
    v.push_back((2, 1));
    v.push_back((3, 1));
    let mut hidden = 0;
    while v.len() != 0 {
        let (m, n) = v.pop_front().unwrap();
        let f = ((k / (n + m)) * 2 - 2) as u64;
        hidden += f;
        let generated = [(2 * m - n, m), (2 * m + n, m), (m + 2 * n, n)];
        for (m, n) in generated {
            if m + n <= k / 2 {
                v.push_back((m, n));
            }
        }
    }
    hidden += (k - 1) as u64;
    hidden += ((k - 2) / 2) as u64;
    hidden
}

fn main() {
    println!("{}", check_part(100_000) * 6);
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
