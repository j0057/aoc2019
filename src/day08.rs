use std::num::ParseIntError;
use std::str::FromStr;

//
// struct Input
//

pub struct Input(Vec<u8>);

impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let result = line.bytes()
            .map(|b| b-48)
            .collect::<Vec<_>>();
        assert!(result.iter().all(|v| *v <= 9));
        Ok(Input(result))
    }
}

impl AsRef<Input> for Input {
    fn as_ref(self: &Self) -> &Self {
        self
    }
}

//
// solution
//

/*
 * Clippy tells me that I appear to be counting bytes the naive way; I should use the bytecount
 * crate instead. This naive version runs in ~3800 ns.
 *
 * --> https://rust-lang.github.io/rust-clippy/master/index.html#naive_bytecount
 */
#[allow(clippy::naive_bytecount)]
fn checksum(layers: &[u8], w: usize, h: usize) -> usize {
    let layer: &[u8] = layers
        .chunks(w * h)
        .min_by_key(|ps| ps.iter().filter(|p| **p == 0).count())
        .unwrap();
    layer.iter().filter(|p| **p == 1).count() * layer.iter().filter(|p| **p == 2).count()
}

pub fn day08a(input: &Input) -> usize {
    checksum(&input.0, 25, 6)
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
    fn test_08() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_line::<super::Input>("input/day08.txt")?;
        assert_eq!(super::day08a(&input), 2806);
        Ok(())
    }
}
