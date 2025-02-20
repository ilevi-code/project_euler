mod coprime;

use coprime::CoprimeGenerator;

fn main() {
    const LIMIT: u64 = 1_000_000_000;
    let mut perimiters = 0u64;
    let mut coprime_generator = CoprimeGenerator::new();
    while let Some(coprimes) = coprime_generator.pop() {
        let m = coprimes.0 as u64;
        let n = coprimes.1 as u64;
        assert!(m > n);
        let a = m * m - n * n;
        let b = 2 * m * n;
        let c = m * m + n * n;
        for side in [a, b] {
            if side * 2 == c - 1 || side * 2 == c + 1 {
                let perimiter = side * 2 + c * 2;
                if perimiter < LIMIT {
                    perimiters += perimiter;
                    coprime_generator.feed(coprimes);
                }
            }
        }
    }
    println!("{perimiters}");
}
