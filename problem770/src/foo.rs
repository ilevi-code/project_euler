use rug::{Assign, Integer, Rational};

struct NativeRational {
    numer: i32,
    denom: i32,
}

impl PartialEq<Rational> for NativeRational {
    fn eq(&self, other: &Rational) -> bool {
        todo!()
    }
}

impl PartialOrd<Rational> for NativeRational {
    fn partial_cmp(&self, other: &Rational) -> Option<std::cmp::Ordering> {
        // (a/b).cmp(c/d) == (a*d).cmp(c*b)
        let mut left = Integer::new();
        let mut right = Integer::new();
        left.assign(other.numer() * self.denom);
        right.assign(other.denom() * self.numer);
        Some(left.cmp(&right))
    }
}

struct BinomialHalfRowSum {
    whole_row_sum: Integer,
    mid_binom: Integer,
    row_num: i32,
}

impl BinomialHalfRowSum {
    fn new() -> Self {
        Self {
            whole_row_sum: Integer::ONE.clone(),
            mid_binom: Integer::ONE.clone(),
            row_num: 1,
        }
    }

    fn next(&mut self) -> Integer {
        self.whole_row_sum <<= 2;
        let mut ret = Integer::new();
        self.mid_binom <<= 1;
        self.mid_binom *= (self.row_num * 2) - 1;
        self.mid_binom /= self.row_num;
        ret.assign(&self.whole_row_sum + &self.mid_binom);
        self.row_num += 1;
        ret
    }
}

pub fn run() {
    let dest_ratio = NativeRational {
        numer: 199,
        denom: 100,
    };
    let mut half_row_sum = BinomialHalfRowSum::new();
    for i in 1.. {
        let mut numer = Integer::ONE.clone();
        numer <<= i * 2 + 1;
        let denom: Integer = half_row_sum.next(); // binomniual_sum
        let rational = unsafe { Rational::from_canonical(numer, denom) };
        println!("{i} = {}", rational.to_f32());
        // if i % 1000 == 0 {
        //     println!("{i} = {}", rational.to_f32());
        // }
        // println!("{rational}");
        if dest_ratio > rational {
            println!("Done! {i}");
            break;
        }
    }
}
