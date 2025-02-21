use itertools::Itertools;
use std::time::Instant;

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Sub,
    RSub,
    Mul,
    Div,
    RDiv,
}

const OPS: [Operation; 6] = [
    Operation::Add,
    Operation::Mul,
    Operation::Sub,
    Operation::RSub,
    Operation::Div,
    Operation::RDiv,
];

fn calc_target(nums: &[i32], ops: &[Operation]) -> Option<i32> {
    let mut target = nums[0] as f64;
    for i in 0..ops.len() {
        let value = nums[i + 1];
        match ops[i] {
            Operation::Add => target += value as f64,
            Operation::Sub => target -= value as f64,
            Operation::Mul => target *= value as f64,
            Operation::Div => target /= value as f64,
            Operation::RSub => target = value as f64 - target,
            Operation::RDiv => target = value as f64 / target,
        }
    }
    if target == target.trunc() {
        Some(target as i32)
    } else {
        None
    }
}

#[test]
fn check_cacl() {
    assert_eq!(
        calc_target(
            &[1, 3, 4, 2],
            &[Operation::Add, Operation::Mul, Operation::Div]
        ),
        Some(8)
    );
    assert_eq!(
        calc_target(
            &[1, 2, 3, 4],
            &[Operation::Div, Operation::Add, Operation::Mul]
        ),
        Some(14)
    );
}

fn count_consecutive(nums: [i32; 4]) -> i32 {
    let mut v = Vec::<i32>::new();
    for nums in nums.into_iter().permutations(4) {
        for ops in itertools::iproduct!(OPS, OPS, OPS) {
            let ops = [ops.0, ops.1, ops.2];
            if let Some(value) = calc_target(&nums, &ops) {
                if value > 0 {
                    v.push(value);
                }
            }
        }
    }
    v.sort();
    let mut max = 0;
    for value in v {
        if value == max + 1 {
            max = value;
        } else if value > max + 1 {
            break;
        }
    }
    max
}

fn main() {
    let start = Instant::now();
    let mut max = 0;
    let mut best = [0i32; 4];
    for i in 1..10 {
        for j in 1..i {
            for k in 1..j {
                for l in 1..k {
                    let values = [l, k, j, i];
                    let consecutive = count_consecutive(values);
                    if consecutive > max {
                        max = consecutive;
                        best = values;
                    }
                }
            }
        }
    }
    println!("{:?} (in {:?})", best, start.elapsed());
}
