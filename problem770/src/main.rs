use rug::{Assign, Float};
use std::collections::BTreeMap;

#[derive(Eq, PartialOrd, Ord, PartialEq, Debug, Clone)]
struct Actions {
    gives: i32,
    takes: i32,
}

impl Actions {
    fn take(&self) -> Actions {
        Actions {
            takes: self.takes - 1,
            ..*self
        }
    }

    fn give(&self) -> Actions {
        Actions {
            gives: self.gives - 1,
            ..*self
        }
    }

    fn rounds(rounds: i32) -> Actions {
        Actions {
            gives: rounds,
            takes: rounds,
        }
    }
}

fn delphi_quick(cache: &mut BTreeMap<Actions, Float>, actions: Actions) -> Option<Float> {
    if actions.gives == 0 {
        // offering zeros, not letting the other player get anything
        Some(Float::with_val(50, 1.0))
    } else if actions.takes == 0 {
        // offering always the whole amoung, doubling it each time
        let mut ret = Float::new(50);
        ret.assign(Float::i_exp(1, actions.gives));
        Some(ret)
    } else if let Some(factor) = cache.get(&actions) {
        Some(factor.clone())
    } else {
        None
    }
}

fn delphi(cache: &mut BTreeMap<Actions, Float>, actions: Actions) -> Float {
    let mut need_to_calc = vec![actions.clone()];

    while let Some(current) = need_to_calc.pop() {
        // calculate gain-factor based on possible action of the other player
        let Some(take_factor) = delphi_quick(cache, current.take()) else {
            need_to_calc.push(current.clone());
            need_to_calc.push(current.take());
            continue;
        };
        let Some(give_factor) = delphi_quick(cache, current.give()) else {
            need_to_calc.push(current.clone());
            need_to_calc.push(current.give());
            continue;
        };

        // o is offer, G in gain-factor when opponent gives, T is gain-factor when opponent takes
        // G(1+o) = T(1-o)
        // G+Go = T-To
        // (T+G)o = T-G
        // o = (T-G)/(T+G)
        let mut sub = Float::new(50);
        let mut add = Float::new(50);
        sub.assign(&take_factor - &give_factor);
        add.assign(take_factor + &give_factor);
        let offer = sub / add;
        let gain: Float = (1.0 + offer) * give_factor;
        cache.insert(current, gain.clone());
    }
    cache.get(&actions).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
    use std::collections::BTreeMap;

    use super::Actions;

    fn delphi(actions: Actions) -> f64 {
        let mut cache = BTreeMap::new();
        super::delphi(&mut cache, actions).to_f64()
    }

    fn delphi_quick(actions: Actions) -> f64 {
        let mut cache = BTreeMap::new();
        super::delphi_quick(&mut cache, actions).unwrap().to_f64()
    }

    #[test]
    fn only_gives() {
        assert_f64_near!(delphi_quick(Actions { gives: 2, takes: 0 }), 4.0);
    }

    #[test]
    fn only_takes() {
        assert_f64_near!(delphi_quick(Actions { gives: 0, takes: 3 }), 1.0);
    }

    #[test]
    fn single_round() {
        assert_f64_near!(delphi(Actions { gives: 1, takes: 1 }), 4.0 / 3.0);
    }

    #[test]
    fn single_take_two_gives() {
        assert_f64_near!(delphi(Actions { gives: 2, takes: 1 }), 2.0);
    }

    #[test]
    fn two_take_single_gives() {
        assert_f64_near!(delphi(Actions { gives: 1, takes: 2 }), 8.0 / 7.0);
    }

    #[test]
    fn two_rounds() {
        assert_f64_near!(delphi(Actions::rounds(2)), 16.0 / 11.0);
    }
}

fn main() {
    let mut cache = BTreeMap::new();
    for rounds in 1.. {
        let gain = delphi(&mut cache, Actions::rounds(rounds));
        println!("{rounds} = {gain}");
        if gain >= 1.9999 {
            println!("{rounds}");
            break;
        }
    }
}
