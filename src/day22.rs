//
// enum ParseError
//

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("cannot parse string {0:?}")]
    UnparseableString(String),

    #[error("cannot parse number")]
    UnparseableNumber(#[from] std::num::ParseIntError),
}

//
// enum Action
//

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    DealNew,
    Cut(i128),
    DealInc(i128),
}

impl std::str::FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            Ok(Action::DealNew)
        }
        else if let Some(n) = s.strip_prefix("deal with increment ") {
            Ok(Action::DealInc(n.parse()?))
        }
        else if let Some(n) = s.strip_prefix("cut ") {
            Ok(Action::Cut(n.parse()?))
        }
        else {
            Err(ParseError::UnparseableString(s.to_owned()))
        }
    }
}

//
// solution
//

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_exp(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        0
    }
    else {
        let mut result = 1;
        base %= modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp >>= 1;
            base = base * base % modulus
        }
        result
    }
}

// https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbwp0r0/
fn slam_shuffle(actions: &[Action], n: i128) -> i128 {
    actions
        .iter()
        .fold(2019, |c, action| match action {
            Action::DealNew     => (-c - 1) % n,
            Action::DealInc(i)  => ( c * i) % n,
            Action::Cut(i)      => ( c - i) % n,
        })
}

// https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbtugcu/
#[allow(clippy::many_single_char_names)] // justification: I don't care
fn slam_shuffle_2(actions: &[Action], m: i128, n: i128, p: i128) -> i128 {
    let (a, b): (i128, i128) = actions
        .iter()
        .fold((1, 0), |(a, b), action| match action {
            Action::DealInc(x)  => ((a*x).rem_euclid(m), (b*x).rem_euclid(m)),
            Action::DealNew     => ((-a).rem_euclid(m), (m-1-b).rem_euclid(m)),
            Action::Cut(x)      => (a, (b-x).rem_euclid(m)),
        });
    let r = (b * mod_exp(1-a, m-2, m)).rem_euclid(m);
    ((p-r) * mod_exp(a, n*(m-2), m) + r).rem_euclid(m)
}

pub fn day22a(actions: &[Action]) -> i128 {
    slam_shuffle(actions, 10007)
}

pub fn day22b(actions: &[Action]) -> i128 {
    slam_shuffle_2(actions, 119315717514047, 101741582076661, 2020)
}

//
// tests
//

#[cfg(test)]
mod test {
    use super::Action;

    #[test]
    fn test_22a() -> Result<(), Box<dyn std::error::Error>> {
        let input = crate::util::get_parsed_lines::<Action>("input/day22.txt")?;
        assert_eq!(super::day22a(&input), 6978);
        Ok(())
    }

    #[test]
    fn test_22b() -> Result<(), Box<dyn std::error::Error>> {
        let input = crate::util::get_parsed_lines::<Action>("input/day22.txt")?;
        assert_eq!(super::day22b(&input), 24460989449140);
        Ok(())
    }
}
