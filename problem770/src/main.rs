use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Actions {
    gives: usize,
    takes: usize,
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

    fn rounds(rounds: usize) -> Actions {
        Actions {
            gives: rounds,
            takes: rounds,
        }
    }
}

fn delphi(cache: &mut HashMap<Actions, f64>, actions: Actions) -> f64 {
    if actions.gives == 0 {
        // offering zeros, not letting the other player get anything
        return 1.0;
    } else if actions.takes == 0 {
        // offering always the whole amoung, doubling it each time
        return (actions.gives as f64).exp2();
    } else if let Some(factor) = cache.get(&actions) {
        return *factor;
    }

    // calculate gain-factor based on possible action of the other player
    let take_factor = delphi(cache, actions.take());
    let give_factor = delphi(cache, actions.give());
    // o is offer, G in gain-factor when opponent gives, T is gain-factor when opponent takes
    // G(1+o) = T(1-o)
    // G+Go = T-To
    // (T+G)o = T-G
    // o = (T-G)/(T+G)
    let offer = (take_factor - give_factor) / (take_factor + give_factor);
    let gain = (1.0 + offer) * give_factor;
    cache.insert(actions, gain);
    gain
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Actions;

    fn delphi(actions: Actions) -> f64 {
        let mut cache = HashMap::new();
        super::delphi(&mut cache, actions)
    }

    #[test]
    fn only_gives() {
        assert_eq!(delphi(Actions { gives: 2, takes: 0 }), 4.0);
    }

    #[test]
    fn only_takes() {
        assert_eq!(delphi(Actions { gives: 0, takes: 3 }), 1.0);
    }

    #[test]
    fn single_round() {
        assert_eq!(delphi(Actions { gives: 1, takes: 1 }), 4.0 / 3.0);
    }

    #[test]
    fn single_take_two_gives() {
        assert_eq!(delphi(Actions { gives: 2, takes: 1 }), 2.0);
    }

    #[test]
    fn two_take_single_gives() {
        assert_eq!(delphi(Actions { gives: 1, takes: 2 }), 8.0 / 7.0);
    }

    #[test]
    fn two_rounds() {
        use assert_float_eq::{afe_is_f64_near, afe_near_error_msg, assert_f64_near};
        assert_f64_near!(delphi(Actions::rounds(2)), 16.0 / 11.0);
    }
}

fn main() {
    let mut cache = HashMap::new();
    for rounds in 1.. {
        let gain = delphi(&mut cache, Actions::rounds(rounds));
        println!("{rounds} = {gain}");
        if gain >= 1.9999 {
            println!("{rounds}");
            break;
        }
    }
}
