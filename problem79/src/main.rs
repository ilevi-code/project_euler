use std::fs::read_to_string;
use std::collections::{HashMap, BTreeSet};
use itertools::izip;
use std::time::Instant;

#[derive(Debug)]
struct PasswordRule {
    digit: u32,
    deps: BTreeSet<u32>,
}

struct Password {
    guess: String,
    inserted: BTreeSet<u32>,
}

impl Password {
    fn new() -> Self {
        Self {
            guess: String::new(),
            inserted: BTreeSet::new(),
        }
    }

    fn push(&mut self, digit: u32) {
        if !self.inserted.contains(&digit) {
            self.guess.push(char::from_digit(digit, 10).unwrap());
            self.inserted.insert(digit);
        }
    }
}

struct PasswordDeriver {
    rule_sets: Vec<HashMap<u32, PasswordRule>>,
}

impl PasswordDeriver {
    fn new(digits_per_rule: u32) -> Self {
        Self {
            rule_sets: (1..digits_per_rule).map(|_| HashMap::new()).collect(),
        }
    }

    fn read_rules(&mut self, filename: &str) {
        for line in read_to_string(filename).unwrap().lines() {
            self.parse_rule(line.to_string());
        }
    }

    fn parse_rule(&mut self, line: String) {
        let mut next_digits = line.chars();
        next_digits.next();
        for (first, second, rule_set) in izip!(line.chars(), next_digits, &mut self.rule_sets) {
            let first = match char::to_digit(first, 10) {
                Some(val) => val,
                None => continue,
            };
            let second = match char::to_digit(second, 10) {
                Some(val) => val,
                None => continue,
            };
            let rule = rule_set.entry(second).or_insert(PasswordRule{
                digit: second,
                deps: BTreeSet::new(),
            });
            rule.deps.insert(first);
        }
    }

    fn solve(&mut self) -> String {
        let mut password = Password::new();

        for rule_set in &self.rule_sets {
            let sorted = Self::sort_deps(&rule_set);
            for rule in sorted {
                for dep in &rule.deps {
                    password.push(*dep);
                }
                password.push(rule.digit);
            }
        }

        password.guess
    }

    fn sort_deps(rules: &HashMap<u32, PasswordRule>) -> Vec<&PasswordRule> {
        let mut sorted: Vec<_> = rules.iter().map(|(_,v)| v).collect();
        sorted.sort_by(|a , b| a.deps.len().cmp(&b.deps.len()));
        sorted
    }
}

fn main() {
    let mut rules = PasswordDeriver::new(3);
    rules.read_rules("keylog.txt");

    let now = Instant::now();
    let solution = rules.solve();
    let elapsed = now.elapsed();

    println!("{}", solution);
    println!("Elapsed: {:.2?}", elapsed);

}
