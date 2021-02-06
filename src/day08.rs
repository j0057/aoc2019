use std::num::ParseIntError;
use std::str::FromStr;

use bytecount;

//
// enum InputError
//

#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("Cannot parse number {0:?}")]
    Parse(#[from] ParseIntError)
}

//
// struct Input
//

pub struct Input(Vec<u8>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let result = line.bytes()
            .map(|b| b-48)
            .collect::<Vec<_>>();
        assert!(result.iter().all(|v| *v <= 9));
        Ok(Input(result))
    }
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Self {
        &self
    }
}

//
// solution
//

/*
 * Thanks Clippy -- I naively used .iter().filter(...).count() which took ~3800 ns, but with the
 * bytecount crate it runs more than twice as fast, ~1400 ns.
 *
 * --> https://rust-lang.github.io/rust-clippy/master/index.html#naive_bytecount
 */
fn checksum(layers: &[u8], w: usize, h: usize) -> usize {
    let layer: &[u8] = layers
        .chunks(w * h)
        .min_by_key(|ps| bytecount::count(ps, 0))
        .unwrap();
    bytecount::count(layer, 1) * bytecount::count(layer, 2)
}

fn combine(data: &[u8], w: usize, h: usize) -> Vec<u8> {
    let layers = data
        .chunks(w * h)
        .map(|layer| layer.to_vec())
        .collect::<Vec<Vec<u8>>>();
    (0..layers[0].len())
        .map(|i| layers.iter().find(|v| v[i] != 2).unwrap()[i])
        .collect::<Vec<u8>>()
}

pub fn day08a(input: &Input) -> usize {
    checksum(&input.0, 25, 6)
}

pub fn day08_main(input: &Input) {
    combine(&input.0, 25, 6)
        .chunks(25)
        .map(|row| row.iter().map(|p| &[' ', '@'][*p as usize]).collect::<String>())
        .for_each(|row| println!("{}", row))
}

//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_08_ex1() -> Result<(), Box<dyn Error>> {
        let input = "123456789012".parse::<super::Input>()?;
        let result = super::checksum(&input.0, 2, 3);
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_08_ex2() -> Result<(), Box<dyn Error>> {
        let input = "0222112222120000".parse::<super::Input>()?;
        let result = super::combine(&input.0, 2, 2);
        assert_eq!(result, &[0, 1, 1, 0]);
        Ok(())
    }

    #[test]
    fn test_08() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_line::<super::Input>("input/day08.txt")?;
        assert_eq!(super::day08a(&input), 2806);
        Ok(())
    }
}
