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

pub fn day22a(actions: &[Action]) -> i128 {
    slam_shuffle(actions, 10007)
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
}
