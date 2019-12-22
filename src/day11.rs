use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Write;

use num_complex::Complex;

use crate::intcode;
use crate::csiseq;

// -i is up, +i is down, -1 is left, +1 is right
static TURN_CCW: Complex<i32> = Complex { re: 0, im: -1 };
static TURN_CW: Complex<i32> = Complex { re: 0, im: 1 };

struct PaintRobot {
    vm: intcode::VM,
    out: Vec<i128>,
    state: intcode::Status,
    grid: HashMap<Complex<i32>, i128>,
    pos: Complex<i32>,
    dir: Complex<i32>,
}

impl PaintRobot {
    fn new(vm: &intcode::VM) -> Self {
        PaintRobot {
            vm: vm.clone(),
            out: Vec::new(),
            state: intcode::Status::Suspended,
            grid: HashMap::new(),
            pos: Complex::new(0, 0),
            dir: Complex::new(0, -1)
        }
    }
}

impl Iterator for PaintRobot {
    type Item = (Complex<i32>, Complex<i32>, i128);

    // input color for current position
    // outputs two values: 1) color and 2) turn [1 for CW, 0 for CCW]
    // 1. put color to current position
    // 2. apply turn to current direction
    // 3. move one step
    fn next(self: &mut Self) -> Option<Self::Item> {
        // try to get more output if output vector is empty
        if self.out.len() < 2 {

            // VM has halted: end of iteration
            if let intcode::Status::Halted = self.state {
                return None;
            }

            // get more output from VM
            let mut input = Vec::<i128>::new();
            loop {
                self.state = self.vm.step(&mut input, &mut self.out);
                match self.state {
                    intcode::Status::Halted     => break,
                    intcode::Status::Blocked    => input.push(*self.grid.get(&self.pos).unwrap_or(&0)),
                    intcode::Status::Suspended  => if self.out.len() >= 2 { break },
                }
            }
        }

        // stop if still no output available
        if self.out.len() < 2 {
            return None;
        }

        // update grid with first output
        let new_color = self.out.remove(0);
        *self.grid.entry(self.pos).or_insert(0) = new_color;

        // update direction and position with second output
        let turn = self.out.remove(0);
        self.dir *= if turn == 0 { TURN_CCW } else { TURN_CW };
        self.pos += self.dir;

        // return iteration step
        Some((self.pos.clone(), self.dir.clone(), new_color))
    }
}

pub fn day11a(vm: &intcode::VM) -> i128 {
    PaintRobot::new(vm)
        .filter(|(_, _, color)| *color == 1)
        .map(|(pos, dir, _)| pos - dir)
        .collect::<HashSet<_>>()
        .len() as i128
}

pub fn day11_main(vm: &intcode::VM) -> Result<(), Box<dyn Error>> {
    let mut robot = PaintRobot::new(vm);
    robot.grid.insert(Complex::new(0, 0), 1);
    let path = robot.collect::<Vec<_>>();
    let min_x = path.iter().map(|(pos, dir, _)| (pos - dir).re).min().unwrap();
    let min_y = path.iter().map(|(pos, dir, _)| (pos - dir).im).min().unwrap();
    let max_y = path.iter().map(|(pos, dir, _)| (pos - dir).im).max().unwrap();
    let mut stdout = std::io::stdout();
    stdout.write(csiseq::CLEAR_SCREEN)?;
    stdout.flush()?;
    for (pos, dir, new_color) in &path {
        let pos = pos - dir;
        stdout.write(&csiseq::move_cursor(pos.im + min_y.abs() + 1, pos.re + min_x.abs() + 1))?;
        stdout.write(if *new_color == 0 { b" " } else { b"@" })?;
        stdout.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    stdout.write(&csiseq::move_cursor(max_y + min_y.abs() + 2, 1))?;
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::intcode;
    use crate::util;

    #[test]
    fn test_11() -> Result<(), Box<dyn Error>> {
        let vm = util::get_parsed_line::<intcode::VM>("input/day11.txt")?;
        assert_eq!(super::day11a(&vm), 1681);
        Ok(())
    }
}
