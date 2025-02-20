use rug::Integer;

struct EContinuedFraction {
    index: Option<usize>,
    k: u32,
}

impl EContinuedFraction {
    fn new() -> EContinuedFraction {
        EContinuedFraction { index: None, k: 0 }
    }
}

impl Iterator for EContinuedFraction {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            None => {
                self.index = Some(0);
                Some(2)
            }
            Some(index) => {
                let digit = match index % 3 {
                    0 => 1,
                    1 => {
                        self.k += 1;
                        2 * self.k
                    }
                    2 => 1,
                    _ => panic!("invalid modulus value"),
                };
                self.index = Some((index + 1) % 3);
                Some(digit)
            }
        }
    }
}

#[test]
fn e_continued_fraction_generation() {
    assert_eq!(
        EContinuedFraction::new().take(10).collect::<Vec<u32>>(),
        vec![2, 1, 2, 1, 1, 4, 1, 1, 6, 1]
    );
}

fn continued_fraction_convergant(
    continued_fraction: impl Iterator<Item = u32>,
) -> (Integer, Integer) {
    // we represent our convergant fraction as (Ap + B)/(Cp + D) where `p` is a partial
    // denominator.
    // When iterating over the partial denominators, p is always of the form `i +1/p`, where i is
    // the partial denominator, and p is the next partial denomiator.
    let mut numerator = (Integer::from(1), Integer::from(0));
    let mut denominator = (Integer::from(0), Integer::from(1));
    for partial_denomiator in continued_fraction {
        // numerator = (
        //     numerator.0 * Integer::from(partial_denomiator) + numerator.1,
        //     numerator.0,
        // );
        // denominator = (
        //     denominator.0 * partial_denomiator + denominator.1,
        //     denominator.0,
        // );
        // ===============================
        let (num0, num1) = numerator;
        numerator = (num0.clone() * partial_denomiator + num1, num0);
        let (den0, den1) = denominator;
        denominator = (den0.clone() * partial_denomiator + den1, den0);
    }
    (numerator.0, denominator.0)
}

#[test]
fn test_continued_fraction_convergant() {
    assert_eq!(
        continued_fraction_convergant([2].into_iter()),
        (Integer::from(2), Integer::from(1))
    );
    assert_eq!(
        continued_fraction_convergant([2, 1].into_iter()),
        (Integer::from(3), Integer::from(1))
    );
    assert_eq!(
        continued_fraction_convergant([2, 1, 1].into_iter()),
        (Integer::from(5), Integer::from(2))
    );
    assert_eq!(
        continued_fraction_convergant([2, 1, 1, 1].into_iter()),
        (Integer::from(8), Integer::from(3))
    );
    assert_eq!(
        continued_fraction_convergant([2, 1, 1, 1, 4].into_iter()),
        (Integer::from(37), Integer::from(14))
    );
}

#[test]
fn test_continued_fraction_convergant_of_e() {
    assert_eq!(
        continued_fraction_convergant(EContinuedFraction::new().take(10)),
        (Integer::from(1457), Integer::from(536))
    );
}

fn sum_digits(mut n: Integer) -> u32 {
    let mut sum = 0;
    while n != 0 {
        let (quotient, remainder) = n.div_rem(Integer::from(10));
        sum += remainder.to_u32().unwrap();
        n = quotient;
    }
    sum
}

#[test]
fn test_sum_digits() {
    assert_eq!(sum_digits(Integer::from(1457)), 17);
}

#[test]
fn test_foo() {
    assert_eq!(
        sum_digits(continued_fraction_convergant(EContinuedFraction::new().take(10)).0,),
        17
    );
}

fn main() {
    let (numerator, _) = continued_fraction_convergant(EContinuedFraction::new().take(100));
    let sum = sum_digits(numerator);
    println!("{:?}", sum);
}
