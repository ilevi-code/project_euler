use std::collections::VecDeque;

pub struct CoprimeGenerator {
    queue: VecDeque<(u32, u32)>,
}

impl CoprimeGenerator {
    pub fn new() -> CoprimeGenerator {
        CoprimeGenerator {
            queue: VecDeque::from([(2, 1), (3, 1)]),
        }
    }
    pub fn pop(&mut self) -> Option<(u32, u32)> {
        self.queue.pop_front()
    }

    pub fn feed(&mut self, (m, n): (u32, u32)) {
        self.queue.push_back((2 * m - n, m));
        self.queue.push_back((2 * m + n, m));
        self.queue.push_back((m + 2 * n, n));
    }
}

#[test]
fn test_coprimes_below_10() {
    let expected = BTreeSet::<(u32, u32)>::from([
        (2, 1),
        (3, 1),
        (3, 2),
        (5, 2),
        (4, 1),
        (5, 3),
        (7, 3),
        (5, 1),
        (4, 3),
        (8, 3),
        (7, 2),
        (8, 5),
        (9, 2),
        (7, 4),
        (9, 4),
        (6, 1),
        (7, 5),
        (9, 5),
        (7, 1),
        (5, 4),
        (8, 1),
        (9, 7),
        (9, 1),
        (6, 5),
        (7, 6),
        (8, 7),
        (9, 8),
    ]);
    let mut coprimes = BTreeSet::<(u32, u32)>::new();
    let mut coprime_generator = CoprimeGenerator::new();
    while let Some(coprime_duo) = coprime_generator.pop() {
        if coprime_duo.0 < 10 && coprime_duo.1 < 10 {
            coprimes.insert(coprime_duo);
            coprime_generator.feed(coprime_duo);
        }
    }
    assert_eq!(coprimes, expected);
}
