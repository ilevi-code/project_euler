use std::time::Instant;

enum Operation {
    Add,
    Sub,
}

struct Partitioner {
    modulus: i32,
    cache: Vec<i32>,
}

impl Partitioner {
    fn new(modulus: i32) -> Self {
        Self {
            modulus,
            cache: vec!(1),
        }
    }

    fn handle_op(a: i32, b: i32, op: &Operation) -> i32 {
        match op {
            Operation::Add => a + b,
            Operation::Sub => a - b,
        }
    }
}

impl Iterator for Partitioner {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut op = Operation::Add;
        let mut partitions = 0;
        let mut i = 1;
        let mut j = 1;
        let mut n = self.cache.len();

        loop {
            if n < i {
                break;
            }
            n -= i;
            let a1 = self.cache[n];
            partitions = Self::handle_op(partitions, a1, &op);

            if n < j {
                break;
            }
            n -= j;
            let a2 = self.cache[n];
            partitions = Self::handle_op(partitions, a2, &op);

            i += 2;
            j += 1;
            op = match op {
                Operation::Add => Operation::Sub,
                Operation::Sub => Operation::Add,
            }
        }

        self.cache.push(partitions % self.modulus);
        Some(partitions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_ten() {
        let partitioner = crate::Partitioner::new(100);
        let partitions: Vec<_> = partitioner.take(10).collect();
        assert_eq!(partitions,[1, 2, 3, 5, 7, 11, 15, 22, 30, 42]);
    }
}

fn solve() -> usize {
    let modulus = 1_000_000;
    let partitioner = Partitioner::new(modulus);
    let mut iter = partitioner.enumerate();

    loop {
        let (i, p) = iter.next().unwrap();
        if p % modulus == 0 {
            break i + 1;
        }
    }
}

fn main() {
    let now = Instant::now();
    let solution = solve();
    let elapsed = now.elapsed();

    println!("{:?}", solution);
    println!("Elapsed: {:.2?}", elapsed);
}
