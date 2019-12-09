use std::str::FromStr;

use itertools::Itertools;

use crate::day03;

pub struct Input(u64, u64);

impl FromStr for Input {
    type Err = day03::InputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts = text.split('-').collect::<Vec<_>>();
        let input = Input(parts[0].parse::<u64>()?, parts[1].parse::<u64>()?);
        Ok(input)
    }
}

pub fn day04a(input: &[Input]) -> u64 {
    (input[0].0..=input[0].1)
        .map(|n| n.to_string())
        .filter(|p| p.chars().zip(p.chars().skip(1)).any(|(a, b)| a == b))
        .filter(|p| p.chars().zip(p.chars().skip(1)).all(|(a, b)| a <= b))
        .count() as u64
}

pub fn day04b(input: &[Input]) -> u64 {
    (input[0].0..=input[0].1)
        .map(|n| n.to_string())
        .filter(|p| p.chars().group_by(|&ch| ch).into_iter().any(|(_key, group)| group.count() == 2))
        .filter(|p| p.chars().zip(p.chars().skip(1)).any(|(a, b)| a == b))
        .filter(|p| p.chars().zip(p.chars().skip(1)).all(|(a, b)| a <= b))
        .count() as u64
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_04_ex1() {
        assert_eq!(super::day04a(&[super::Input(111111, 111111)]), 1);
    }

    #[test]
    fn test_04_ex2() {
        assert_eq!(super::day04a(&[super::Input(223450, 223450)]), 0);
        assert_eq!(super::day04a(&[super::Input(223456, 223456)]), 1);
    }

    #[test]
    fn test_04_ex3() {
        assert_eq!(super::day04a(&[super::Input(123789, 123789)]), 0);
    }

    #[test]
    fn test_04_ex4() {
        assert_eq!(super::day04b(&[super::Input(112233, 112233)]), 1);
    }

    #[test]
    fn test_04_ex5() {
        assert_eq!(super::day04b(&[super::Input(123444, 123444)]), 0);
    }

    #[test]
    fn test_04_ex6() {
        assert_eq!(super::day04b(&[super::Input(111122, 111122)]), 1);
    }

    #[test]
    fn test_04() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_lines::<super::Input>("input/day04.txt")?;
        assert_eq!(super::day04a(&input), 1855);
        assert_eq!(super::day04b(&input), 1253);
        Ok(())
    }
}
