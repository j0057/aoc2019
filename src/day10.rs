use std::str::FromStr;
use std::collections::HashSet;
use std::f64::consts::PI;

use num_complex::Complex;

fn mdeg(rad: f64) -> i64 { (rad / 2.0 / PI * 360_000.0) as i64 }

//
// enum InputError
//

#[derive(Debug, thiserror::Error)]
pub enum InputError {
    #[error("error parsing character {0:?}")]
    Parse(Option<char>)
}

//
// struct Input
//

#[derive(Debug)]
pub struct Input(Vec<Complex<f64>>);

impl FromStr for Input {
    type Err = InputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let c: Vec<Complex<f64>> = text
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| (y as f64, x as f64, ch)))
            .filter_map(|(y, x, ch)| match ch {
                '.' => None,
                '#' => Some(Ok(Complex::new(x, y))),
                chr => Some(Err(InputError::Parse(Some(chr))))
            })
            .collect::<Result<Vec<Complex<f64>>, InputError>>()?;
        Ok(Input(c))
    }

}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        &self
    }
}

//
// solution
//

fn asteroids_visible(asteroids: &[Complex<f64>], pos: &Complex<f64>) -> usize {
    asteroids
        .iter()
        .filter(|b| pos != *b)
        .map(|b| mdeg((b-pos).to_polar().1))
        .collect::<HashSet<_>>()
        .len()
}

fn best_asteroid(asteroids: &[Complex<f64>]) -> (Complex<f64>, usize) {
    asteroids
        .iter()
        .map(|a| (*a, asteroids_visible(asteroids, a)))
        .max_by_key(|&(_, num_visible)| num_visible)
        .unwrap()
}

pub fn day10a(input: &Input) -> usize {
    best_asteroid(&input.0).1
}


//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_10_ex1() -> Result<(), Box<dyn Error>> {
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##\n".parse::<super::Input>()?;
        assert_eq!(super::day10a(&input), 8);
        Ok(())
    }

    #[test]
    fn test_10_ex2() -> Result<(), Box<dyn Error>> {
        let input = "......#.#.\n\
                     #..#.#....\n\
                     ..#######.\n\
                     .#.#.###..\n\
                     .#..#.....\n\
                     ..#....#.#\n\
                     #..#....#.\n\
                     .##.#..###\n\
                     ##...#..#.\n\
                     .#....####\n".parse::<super::Input>()?;
        assert_eq!(super::day10a(&input), 33);
        Ok(())
    }

    #[test]
    fn test_10_ex3() -> Result<(), Box<dyn Error>> {
        let input = "#.#...#.#.\n\
                     .###....#.\n\
                     .#....#...\n\
                     ##.#.#.#.#\n\
                     ....#.#.#.\n\
                     .##..###.#\n\
                     ..#...##..\n\
                     ..##....##\n\
                     ......#...\n\
                     .####.###.\n".parse::<super::Input>()?;
        assert_eq!(super::day10a(&input), 35);
        Ok(())
    }

    #[test]
    fn test_10_ex4() -> Result<(), Box<dyn Error>> {
        let input = ".#..#..###\n\
                     ####.###.#\n\
                     ....###.#.\n\
                     ..###.##.#\n\
                     ##.##.#.#.\n\
                     ....###..#\n\
                     ..#.#..#.#\n\
                     #..#.#.###\n\
                     .##...##.#\n\
                     .....#.#..\n".parse::<super::Input>()?;
        assert_eq!(super::day10a(&input), 41);
        Ok(())
    }

    #[test]
    fn test_10_ex5() -> Result<(), Box<dyn Error>> {
        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##\n".parse::<super::Input>()?;
        assert_eq!(super::day10a(&input), 210);
        Ok(())
    }

    #[test]
    fn test_10() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed::<super::Input>("input/day10.txt")?;
        assert_eq!(super::day10a(&input), 274);
        Ok(())
    }
}
