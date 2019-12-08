use std::error::Error;
use std::collections::HashSet;
use std::str::FromStr;
use std::num;

use num_complex::Complex;

/*
 * InputError - Error implementation for Input
 */

#[derive(Debug)]
pub enum InputError {
    UnknownDirection(Option<char>),
    BadValue(num::ParseIntError)
}

impl std::fmt::Display for InputError {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InputError::UnknownDirection(_) => write!(f, "UnknownDirection"),
            InputError::BadValue(e) => write!(f, "BadValue({})", e)
        }
    }
}

impl Error for InputError {
}

impl From<num::ParseIntError> for InputError {
    fn from(e: num::ParseIntError) -> Self {
        InputError::BadValue(e)
    }
}

/*
 * Input - represents a wire path
 */

#[derive(Debug, PartialEq)]
pub struct Input(Vec<Complex<i64>>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let t = text.trim()
                    .split(',')
                    .map(|s| match s.chars().nth(0) {
                        Some('U') => Result::Ok(Complex::<i64>::new(0, -s[1..].parse::<i64>()?)),
                        Some('D') => Result::Ok(Complex::<i64>::new(0,  s[1..].parse::<i64>()?)),
                        Some('L') => Result::Ok(Complex::<i64>::new(-s[1..].parse::<i64>()?, 0)),
                        Some('R') => Result::Ok(Complex::<i64>::new( s[1..].parse::<i64>()?, 0)),
                        x         => Result::Err(InputError::UnknownDirection(x)),
                    })
                    .collect::<Result<Vec<Complex<i64>>, Self::Err>>()?;
        Ok(Input(t))
    }
}

/*
 * Actual solution
 */

fn trace_path(turns: &[Complex<i64>]) -> impl Iterator<Item=Complex<i64>> + '_ {
    turns
        .iter()
        .flat_map(|turn| std::iter::repeat(turn.unscale(turn.l1_norm())).take(turn.l1_norm() as usize))
        .scan(Complex::<i64>::new(0, 0), |acc, step| { *acc += step ; Some(*acc) })
}

pub fn day03a(wires: &[Input]) -> i64 {
    let paths = wires
        .iter()
        .map(|input| trace_path(&input.0).collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    paths[1..]
        .iter()
        .fold(paths[0].clone(), |item, other| item.intersection(other).cloned().collect())
        .iter()
        .map(|pos| pos.l1_norm())
        .min()
        .expect("no wire crossings found")
}

/*
 * Unit tests
 */

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::collections::HashSet;

    use num_complex::Complex;

    use crate::util;

    #[test]
    fn test_03_ex0() -> Result<(), Box<dyn Error>> {
        let expect = [(0, 8), (-5, 0), (0, -5), (3, 0)]
            .iter()
            .map(|&(y, x)| Complex::<i64>::new(x, y))
            .collect::<Vec<_>>();
        let parsed = "R8,U5,L5,D3".parse::<super::Input>()?;
        assert_eq!(parsed, super::Input(expect));
        Ok(())
    }

    #[test]
    fn test_03_ex0_err_1() {
        let result = "R8,U5,L5,X3".parse::<super::Input>();
        assert!(result.is_err(), "expected error result");
    }

    #[test]
    fn test_03_ex0_err_2() {
        let result = "R8,U5,L5,DX".parse::<super::Input>();
        assert!(result.is_err(), "expected error result");
    }

    #[test]
    fn test_03_ex1() -> Result<(), Box<dyn Error>> {
        let path = super::trace_path(&"R8,U5,L5,D3".parse::<super::Input>()?.0).collect::<HashSet<_>>();
        let expected = [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
                        (-1, 8), (-2, 8), (-3, 8), (-4, 8), (-5, 8),
                        (-5, 7), (-5, 6), (-5, 5), (-5, 4), (-5, 3),
                        (-4, 3), (-3, 3), (-2, 3)]
            .iter()
            .map(|&(y, x)| Complex::<i64>::new(x, y))
            .collect::<HashSet<_>>();
        assert_eq!(path, expected);
        Ok(())
    }

    #[test]
    fn test_03_ex2() -> Result<(), Box<dyn Error>> {
        let input = ["R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"]
            .iter()
            .map(|s| s.parse::<super::Input>())
            .collect::<Result<Vec<_>, super::InputError>>()?;
        assert_eq!(super::day03a(&input), 159);
        Ok(())
    }

    #[test]
    fn test_03_ex3() -> Result<(), Box<dyn Error>> {
        let input = ["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"]
            .iter()
            .map(|s| s.parse::<super::Input>())
            .collect::<Result<Vec<_>, super::InputError>>()?;
        assert_eq!(super::day03a(&input), 135);
        Ok(())
    }

    #[test]
    fn test_03() -> Result<(), Box<dyn Error>> {
        let wires = util::get_parsed_lines::<super::Input>("input/day03.txt")?;
        assert_eq!(super::day03a(&wires), 1225);
        Ok(())
    }
}
