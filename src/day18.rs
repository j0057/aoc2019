use std::collections::HashMap;
use std::convert::TryFrom;

//
// type Coord
//

type Coord = num_complex::Complex<u32>;

//
// enum ParseErorr
//

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown character: {0:?}")]
    UnknownCharacter(char),
}

//
// enum Tile
//

#[derive(Debug, PartialEq)]
pub enum Tile {
    Entrance,
    Wall,
    Floor,
    Door(u8),
    Key(u8),
}

impl std::convert::TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(chr: char) -> Result<Self, Self::Error> {
        match chr {
            '@'       => Ok(Tile::Entrance),
            '#'       => Ok(Tile::Wall),
            '.'       => Ok(Tile::Floor),
            'A'..='Z' => Ok(Tile::Door(chr as u8 - 48)),
            'a'..='z' => Ok(Tile::Key(chr as u8 - 96)),
             _        => Err(ParseError::UnknownCharacter(chr)),
        }
    }
}

//
// struct Maze
//

#[derive(Debug, Default)]
pub struct Maze {
    grid: HashMap<Coord, Tile>,
    start: Coord,
}

impl Maze {
    fn new(grid: HashMap<Coord, Tile>) -> Self {
        let start = Maze::find_entrance(&grid);
        Maze {
            grid,
            start,
        }
    }

    fn find_entrance(grid: &HashMap<Coord, Tile>) -> Coord {
        *grid.iter().filter(|&(_, t)| *t == Tile::Entrance).nth(0).unwrap().0
    }
}

impl std::str::FromStr for Maze {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid = text
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .map(move |(x, chr)| Ok((Coord::new(x as u32, y as u32), Tile::try_from(chr)?))))
            .collect::<Result<HashMap<Coord, Tile>, ParseError>>()?;
        Ok(Maze::new(grid))
    }
}

//
// tests
//

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    fn test_18_1() -> Result<(), Box<dyn std::error::Error>> {
        let maze = "#########\n\
                    #b.A.@.a#\n\
                    #########".parse::<super::Maze>()?;
        assert_eq!(maze.floor.get(&super::Coord::new(1, 1)), Some(&super::Tile::Key(2)));
        Ok(())
    }

    #[test]
    fn test_18_2() -> Result<(), Box<dyn std::error::Error>> {
        let maze = "########################\n\
                    #f.D.E.e.C.b.A.@.a.B.c.#\n\
                    ######################.#\n\
                    #d.....................#\n\
                    ########################".parse::<super::Maze>()?;
        Ok(())
    }

    #[test]
    fn test_18_3() -> Result<(), Box<dyn std::error::Error>> {
        let maze = "########################\n\
                    #...............b.C.D.f#\n\
                    #.######################\n\
                    #.....@.a.B.c.d.A.e.F.g#\n\
                    ########################".parse::<super::Maze>()?;
        Ok(())
    }

    #[test]
    fn test_18_4() -> Result<(), Box<dyn std::error::Error>> {
        let maze = "#################\n\
                    #i.G..c...e..H.p#\n\
                    ########.########\n\
                    #j.A..b...f..D.o#\n\
                    ########@########\n\
                    #k.E..a...g..B.n#\n\
                    ########.########\n\
                    #l.F..d...h..C.m#\n\
                    #################".parse::<super::Maze>()?;
        Ok(())
    }

    #[test]
    fn test_18_5() -> Result<(), Box<dyn std::error::Error>> {
        let maze = "########################\n\
                    #@..............ac.GI.b#\n\
                    ###d#e#f################\n\
                    ###A#B#C################\n\
                    ###g#h#i################\n\
                    ########################".parse::<super::Maze>()?;
        Ok(())
    }

    #[test]
    fn test_18() -> Result<(), Box<dyn std::error::Error>> {
        let maze = util::get_parsed::<super::Maze>("input/day18.txt")?;
        Ok(())
    }
}
