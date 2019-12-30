use std::io::Write;
use std::error::Error;

use crate::intcode;
use crate::csiseq;

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
    ball_x: Option<i128>,
    paddle_x: Option<i128>,
}

impl Game {
    fn new(vm: &intcode::VM) -> Game {
        Game {
            vm: vm.clone(),
            out: vec![],
            state: intcode::Status::Suspended,
            ball_x: None,
            paddle_x: None,
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

            let mut input = vec![];
            loop {
                match self.vm.step(&mut input, &mut self.out) {
                    intcode::Status::Suspended  => { if self.out.len() >= 3 { break } },
                    intcode::Status::Halted     => { if self.out.len() >= 3 { break } else { return None } },
                    intcode::Status::Blocked    => { input.push((self.ball_x.unwrap_or(0) - self.paddle_x.unwrap_or(0)).signum()); },
                }
            }
        }

        let x = self.out.remove(0);
        let y = self.out.remove(0);
        let v = self.out.remove(0);

        let result = if x == -1 && y == 0 {
            Output::ScoreUpdate(v)
        }
        else {
            Output::TileUpdate(x, y, v.into())
        };

        match result {
            Output::TileUpdate(x, _, Tile::Paddle) => self.paddle_x = Some(x),
            Output::TileUpdate(x, _, Tile::Ball)   => self.ball_x = Some(x),
            _                                      => (),
        }

        Some(result)
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

pub fn day13_main(vm: &intcode::VM) -> Result<(), Box<dyn Error>> {
    let mut stdout = std::io::stdout();
    let mut game = Game::new(vm);

    game.vm.memory[0] = 2;

    stdout.write_all(csiseq::CLEAR_SCREEN)?;
    stdout.write_all(csiseq::HIDE_CURSOR)?;
    stdout.flush()?;

    for output in game {

        match output {
            Output::TileUpdate(x, y, v) => {
                stdout.write_all(&csiseq::move_cursor((y+3) as i32, (x+4) as i32))?;
                stdout.write_all(v.to_string().as_bytes())?;
                stdout.flush()?;
            },
            Output::ScoreUpdate(s) => {
                stdout.write_all(&csiseq::move_cursor(25, 6))?;
                stdout.write_all(s.to_string().as_bytes())?;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(6));
    }

    stdout.write_all(csiseq::SHOW_CURSOR)?;
    stdout.write_all(&csiseq::move_cursor(25, 1))?;
    stdout.write_all(&[10])?;
    stdout.flush()?;

    Ok(())
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
