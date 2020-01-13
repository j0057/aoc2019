use std::iter::repeat;

#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Error parsing character {0:?} as radix-10 digit")]
    ParseIntError(char)
}

#[derive(Debug, Clone)]
pub struct FFT {
    state: Vec<i64>,
}

impl FFT {
    fn new(v: &[i64]) -> Self {
        Self {
            state: v.to_vec(),
        }
    }
}

impl std::str::FromStr for FFT {
    type Err = InputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let state = line.chars()
            .map(|ch| ch.to_digit(10).map(|v| v as i64).ok_or(InputError::ParseIntError(ch)))
            .collect::<Result<Vec<i64>, InputError>>()?;
        Ok(FFT::new(&state))
    }
}

impl Iterator for FFT {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.state.clone();
        self.state = (0..self.state.len())
            .map(|i| self.state
                 .iter()
                 .zip([0, 1, 0, -1].iter().cycle().flat_map(|p| repeat(p).take(i+1)).skip(1))
                 .map(|(a, b)| a * b)
                 .sum::<i64>().abs() % 10)
            .collect::<Vec<_>>();
        Some(result)
    }
}

impl AsRef<FFT> for FFT {
    fn as_ref(&self) -> &FFT {
        &self
    }
}

pub fn day16a(input: &FFT) -> i64 {
    input.clone().nth(100).unwrap().iter().take(8).fold(0, |n, &d| 10 * n + d)
}

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    fn test_16_1() -> Result<(), Box<dyn std::error::Error>> {
        let mut input = "12345678".parse::<super::FFT>()?;
        assert_eq!(input.next(), Some(vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(input.next(), Some(vec![4, 8, 2, 2, 6, 1, 5, 8]));
        assert_eq!(input.next(), Some(vec![3, 4, 0, 4, 0, 4, 3, 8]));
        assert_eq!(input.next(), Some(vec![0, 3, 4, 1, 5, 5, 1, 8]));
        assert_eq!(input.next(), Some(vec![0, 1, 0, 2, 9, 4, 9, 8]));
        Ok(())
    }

    #[test]
    fn test_16_2() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(super::day16a(&"80871224585914546619083218645595".parse()?), 24176176);
        assert_eq!(super::day16a(&"19617804207202209144916044189917".parse()?), 73745418);
        assert_eq!(super::day16a(&"69317163492948606335995924319873".parse()?), 52432133);
        Ok(())
    }

    #[test]
    fn test_16() -> Result<(), Box<dyn std::error::Error>> {
        let input = util::get_parsed_line::<super::FFT>("input/day16.txt")?;
        assert_eq!(super::day16a(&input), 94935919);
        Ok(())
    }
}
