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
    type Item = (i128, i128, Tile);

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

        Some((self.out.remove(0), self.out.remove(0), self.out.remove(0).into()))
    }
}

//
// solutions
//

pub fn day13a(vm: &intcode::VM) -> usize {
    Game::new(vm)
        .filter(|(_, _, tile)| *tile == Tile::Block)
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
