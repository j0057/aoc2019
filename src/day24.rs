use std::iter::empty;
use std::iter::once;
use std::iter::successors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("character {0} not recognized")]
    Parse(char)
}

#[derive(Debug)]
pub struct Bugs(u32);

static MASK: [u32; 25] = [0x00000022, 0x00000045, 0x0000008A, 0x00000114, 0x00000208,
                          0x00000441, 0x000008A2, 0x00001144, 0x00002288, 0x00004110,
                          0x00008820, 0x00011440, 0x00022880, 0x00045100, 0x00082200,
                          0x00110400, 0x00228800, 0x00451000, 0x008A2000, 0x01044000,
                          0x00208000, 0x00510000, 0x00A20000, 0x01440000, 0x00880000];

static MCUR: [u32; 25] = [0x00000022, 0x00000045, 0x0000008A, 0x00000114, 0x00000208,
                          0x00000441, 0x000008A2, 0x00000144, 0x00002288, 0x00004110,
                          0x00008820, 0x00010440, 0x00000000, 0x00044100, 0x00082200,
                          0x00110400, 0x00228800, 0x00450000, 0x008A2000, 0x01044000,
                          0x00208000, 0x00510000, 0x00A20000, 0x01440000, 0x00880000];

static M_UP: [u32; 25] = [0x00000880, 0x00000080, 0x00000080, 0x00000080, 0x00002080,
                          0x00000800, 0x00000000, 0x00000000, 0x00000000, 0x00002000,
                          0x00000800, 0x00000000, 0x00000000, 0x00000000, 0x00002000,
                          0x00000800, 0x00000000, 0x00000000, 0x00000000, 0x00002000,
                          0x00020800, 0x00020000, 0x00020000, 0x00020000, 0x00022000];

static M_DN: [u32; 25] = [0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                          0x00000000, 0x00000000, 0x0000001F, 0x00000000, 0x00000000,
                          0x00000000, 0x00108421, 0x00000000, 0x01084210, 0x00000000,
                          0x00000000, 0x00000000, 0x01F00000, 0x00000000, 0x00000000,
                          0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000];

impl Bugs {
    fn grow(list: &[u32]) -> Vec<u32> {
        let mut res = list.to_vec();
        if res.first().unwrap() & 0x00022880 != 0 { res.insert(0, 0); }
        if res.last().unwrap()  & 0x01F8C63F != 0 { res.push(0); }
        res
    }

    fn evolve(&self) -> impl Iterator<Item=u32> {
        successors(Some(self.0), |b| {
            Some((0..25)
                 .map(|n| (1 << n, (b & MASK[n]).count_ones()))
                 .filter_map(|(n, p)| (((b & n) == 0 && 1 <= p && p <= 2)
                                    || ((b & n) != 0 && p == 1)).then(|| n))
                 .sum())
        })
    }

    fn evolve_recursively(&self) -> impl Iterator<Item=Vec<u32>> {
        successors(Some(vec![0, self.0, 0]), |b| {
            Some(empty()
                .chain(once(0))
                .chain(Bugs::grow(b).into_iter())
                .chain(once(0))
                .collect::<Vec<_>>()
                .windows(3)
                .map(|w| (0..25)
                     .map(|i| (1 << i, [w[0] & M_UP[i], w[1] & MCUR[i], w[2] & M_DN[i]].iter().fold(0, |a,b| a+b.count_ones())))
                     .filter_map(|(n, p)| (((w[1] & n) == 0 && 1 <= p && p <= 2)
                                        || ((w[1] & n) != 0 && p == 1)).then(|| n))
                     .sum())
                .collect::<Vec<_>>())
        })
    }
}

impl core::str::FromStr for Bugs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .map(move |(x, chr)| match chr {
                    '.' => Ok(0),
                    '#' => Ok(1 << (x + y * 5)),
                     c  => Err(Error::Parse(c)),
                }))
            .try_fold(0, |a, b| Ok(a + b?))?;
        Ok(Bugs(b))
    }
}

impl AsRef<Bugs> for Bugs {
    fn as_ref(&self) -> &Self {
        &self
    }
}

pub fn day24a(bugs: &Bugs) -> u32 {
    let mut seen = std::collections::HashSet::new();
    bugs.evolve()
        .find(|b| ! seen.insert(*b))
        .unwrap()
}

pub fn day24b(bugs: &Bugs) -> u32 {
    bugs.evolve_recursively()
        .map(|v| v.iter().fold(0, |a, b| a + b.count_ones()))
        .nth(200)
        .unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_24_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = "....#\n\
                     #..#.\n\
                     #..##\n\
                     ..#..\n\
                     #....\n".parse::<super::Bugs>()?;

        assert_eq!(input.0, 0x00126530);

        let states = input.evolve().take(5).collect::<Vec<_>>();
        assert_eq!(states, [0b00001_00100_11001_01001_10000,
                            0b00110_11011_10111_01111_01001,
                            0b11101_01000_10000_10000_11111,
                            0b10110_01101_11000_01111_00001,
                            0b00011_00000_10011_10000_01111]);

        let result = super::day24a(&input);
        assert_eq!(result, 0x00208000);
        assert_eq!(result, 2129920);
        Ok(())
    }

    #[test]
    fn test_24_2() -> Result<(), Box<dyn std::error::Error>> {
        let bugs = "....#\n\
                   #..#.\n\
                   #..##\n\
                   ..#..\n\
                   #....\n".parse::<super::Bugs>()?;

        let result = bugs.evolve_recursively()
            .map(|v| v.iter().fold(0, |a, b| a + b.count_ones()))
            .nth(10);
        assert_eq!(result, Some(99));
        Ok(())
    }

    #[test]
    fn test_24a() -> Result<(), Box<dyn std::error::Error>> {
        let bugs = crate::util::get_parsed::<super::Bugs>("input/day24.txt")?;
        let result = super::day24a(&bugs);
        assert_eq!(result, 7543003);
        Ok(())
    }

    #[test]
    fn test_24b() -> Result<(), Box<dyn std::error::Error>> {
        let bugs = crate::util::get_parsed::<super::Bugs>("input/day24.txt")?;
        let result = super::day24b(&bugs);
        assert_eq!(result, 1975);
        Ok(())
    }
}
