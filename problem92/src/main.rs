use std::time::Instant;

struct Cache {
    nums: Vec<Option<bool>>,
    arrived: usize,
}

impl Cache {
    fn new() -> Cache {
        let mut vec = vec![];
        vec.resize(20_000_000, None);
        vec[1] = Some(false);
        vec[89] = Some(true);
        Cache {
            nums: vec,
            arrived: 0,
        }
    }

    fn set(&mut self, i: usize, arrived_at_89: bool) {
        if i < self.nums.len() {
            self.nums[i] = Some(arrived_at_89);
            if arrived_at_89 && i < 10_000_000 {
                self.arrived += 1
            }
        }
    }

    fn get(&self, i: usize) -> Option<bool> {
        self.nums[i]
    }

    fn count(&self, limit: usize) -> usize {
        self.arrived
    }
}

fn square_digits(mut i: usize) -> usize {
    let mut sum = 0;
    while i > 0 {
        let digit = i % 10;
        i = i / 10;
        sum += digit * digit;
    }
    sum
}

fn is_arriving_at_89(num: usize, cache: &mut Cache) -> bool {
    let mut next = num;
    loop {
        next = square_digits(next);
        match cache.get(next) {
            Some(arrived_at_89) => {
                return arrived_at_89;
            }
            None => (),
        }
    }
}

fn main() {
    let start = Instant::now();

    let mut cache = Cache::new();
    const LIMIT: usize = 10_000_000;
    for i in 1..LIMIT {
        let arriving = is_arriving_at_89(i, &mut cache);
        cache.set(i, arriving);
    }

    println!("{} (in {:?})", cache.count(LIMIT), start.elapsed());
}
