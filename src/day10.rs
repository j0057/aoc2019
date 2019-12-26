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
// struct AsteroidShooter
//

#[derive(Debug)]
struct AsteroidShooter {
    asteroids: Vec<(i64, i64, (i64, i64))>,
}

impl AsteroidShooter {
    fn new(asteroids: &[Complex<f64>]) -> AsteroidShooter {
        let (base, _count) = best_asteroid(asteroids);

        let mut result = AsteroidShooter {
            asteroids: asteroids
                .iter()
                .filter(|&&x| x != base)
                .map(|x| (x, (x-base).to_polar()))
                .map(|(x, (dist, angle))| (mdeg(angle + PI), dist as i64, (x.im as i64, x.re as i64)))
                .collect::<Vec<_>>()
        };

        result.asteroids.sort();

        while result.asteroids[0].0 < 90_000 {
            result.asteroids.rotate_left(1);
        }

        result
    }
}

impl Iterator for AsteroidShooter {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.asteroids.is_empty() {
            return None;
        }

        let (angle, _, coord) = self.asteroids.remove(0);

        if self.asteroids.iter().any(|(a, _, _)| *a != angle) {
            while let Some((a, _, _)) = self.asteroids.get(0) {
                if angle != *a {
                    break;
                }
                self.asteroids.rotate_left(1);
            }
        }

        Some(coord)
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

fn asteroid_shootout(asteroids: &[Complex<f64>], n: usize) -> i64 {
    if let Some((y, x)) = AsteroidShooter::new(asteroids).nth(n-1) {
        x * 100 + y
    }
    else {
        panic!("Target #{} not found", n)
    }
}

pub fn day10a(input: &Input) -> usize {
    best_asteroid(&input.0).1
}

pub fn day10b(input: &Input) -> i64 {
    asteroid_shootout(&input.0, 200)
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
    fn test_10_ex6() -> Result<(), Box<dyn Error>> {
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
        assert_eq!(super::asteroid_shootout(&input.0, 1), 1112);
        assert_eq!(super::asteroid_shootout(&input.0, 2), 1201);
        assert_eq!(super::asteroid_shootout(&input.0, 3), 1202);
        assert_eq!(super::asteroid_shootout(&input.0, 10), 1208);
        assert_eq!(super::asteroid_shootout(&input.0, 20), 1600);
        assert_eq!(super::asteroid_shootout(&input.0, 50), 1609);
        assert_eq!(super::asteroid_shootout(&input.0, 199), 906);
        assert_eq!(super::asteroid_shootout(&input.0, 200), 802);
        assert_eq!(super::asteroid_shootout(&input.0, 201), 1009);
        assert_eq!(super::asteroid_shootout(&input.0, 299), 1101);
        Ok(())
    }

    #[test]
    fn test_10() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed::<super::Input>("input/day10.txt")?;
        assert_eq!(super::day10a(&input), 274);
        assert_eq!(super::day10b(&input), 305);
        Ok(())
    }
}
