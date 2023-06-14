use std::collections::LinkedList;

struct Collatz {
    cache: Vec::<usize>,
}

impl Collatz {
    fn new(cache_size: usize) -> Self {
        let mut cache = vec!(0; cache_size);
        cache[1] = 1;
        Self {
            cache,
        }
    }

    fn calc_sequnce_size(&mut self, n: usize) -> usize {
        let mut travel = LinkedList::new();
        let mut next = n;

        while *self.cache.get(next).unwrap_or(&0) == 0 {
            travel.push_back(next);
            next = Self::next(next);
        }

        let sequence_len = self.cache[next] + travel.len(); // found cached length
        self.update_travelled(travel, sequence_len);

        sequence_len
    }

    fn update_travelled(&mut self, travelled: LinkedList<usize>, sequence_len: usize) {
        for (i, num) in travelled.iter().enumerate() {
            match self.cache.get_mut(*num) {
                Some(val) => *val = sequence_len - i,
                None => (),
            };
        }
    }

    fn next(n: usize) -> usize {
        if n % 2 == 0 {
            return n / 2;
        } else {
            return n * 3 + 1;
        }
    }
}

fn solve() -> usize {
    let mut collatz = Collatz::new(2_000_000);
    let mut longest_i = 0;
    let mut longest_len = 0;

    for i in 1..1_000_000 {
        let len = collatz.calc_sequnce_size(i);
        if len > longest_len {
            longest_i = i;
            longest_len = len;
        }
     }

     longest_i
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let longest = solve();

    let elapsed = now.elapsed();

    println!("Solution: {}", longest);
    println!("Elapsed: {:.2?}", elapsed);
}
