use crate::intcode;

//
// enum Tile -- represents the state of a tile
//

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl From<i128> for Tile {
    fn from(v: i128) -> Self {
        match v {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unknown tile type {}", v)
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Tile::Empty => " ",
            Tile::Wall => "%",
            Tile::Block => "#",
            Tile::Paddle => "-",
            Tile::Ball => "o",
        })
    }
}

//
// enum Output -- represents an output instruction of the arcade program
//

#[derive(Debug)]
enum Output {
    TileUpdate(i128, i128, Tile),
    ScoreUpdate(i128),
}

//
// struct Game -- represents the state of the game including whatever it is the arcade program does
//

struct Game {
    vm: intcode::VM,
    out: Vec::<i128>,
    state: intcode::Status,
}

impl Game {
    fn new(vm: &intcode::VM) -> Game {
        Game {
            vm: vm.clone(),
            out: vec![],
            state: intcode::Status::Suspended,
        }
    }
}

impl Iterator for Game {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.out.len() < 3 {
            if let intcode::Status::Halted = self.state {
                return None;
            }

            loop {
                match self.vm.step(&mut vec![], &mut self.out) {
                    intcode::Status::Suspended  => { if self.out.len() >= 3 { break } },
                    intcode::Status::Halted     => { if self.out.len() >= 3 { break } else { return None } },
                    intcode::Status::Blocked    => { panic!("Program is asking for input") },
                }
            }
        }

        let x = self.out.remove(0);
        let y = self.out.remove(0);
        let v = self.out.remove(0);

        if x == -1 && y == 0 {
            Some(Output::ScoreUpdate(v))
        }
        else {
            Some(Output::TileUpdate(x, y, v.into()))
        }
    }
}

//
// solutions
//

pub fn day13a(vm: &intcode::VM) -> usize {
    Game::new(vm)
        .filter(|output| match *output {
            Output::TileUpdate(_, _, Tile::Block) => true,
            _                                     => false
        })
        .count()
}

//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::intcode;
    use crate::util;

    #[test]
    fn test_13() -> Result<(), Box<dyn Error>> {
        let vm = util::get_parsed_line::<intcode::VM>("input/day13.txt")?;
        assert_eq!(super::day13a(&vm), 344);
        Ok(())
    }
}
