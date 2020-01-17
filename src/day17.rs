use std::collections::HashSet;

use std::io::Write;

use num_complex::Complex;

use crate::intcode;

//
// type definitions and constants
//

type Coord = Complex<i32>;

static UP: Coord = Coord { re: 0, im: -1 };
static DN: Coord = Coord { re: 0, im: 1 };
static LT: Coord = Coord { re: -1, im: 0 };
static RT: Coord = Coord { re: 1, im: 0 };

static CCW: Coord = UP;
static CW : Coord = DN;

//
// enum Action
//

#[derive(Debug, Clone, Copy)]
enum Action {
    TurnLeft,
    MoveForward(Option<i32>),
    TurnRight,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Action::TurnLeft             => "L".into(),
            Action::MoveForward(None)    => "F".into(),
            Action::MoveForward(Some(x)) => x.to_string(),
            Action::TurnRight            => "R".into(),
        })
    }
}

//
// struct CleaningRobot
//

#[derive(Debug, Default)]
struct CleaningRobot {
    scaffold: HashSet<Coord>,
    pos: Coord,
    dir: Coord,
}

impl CleaningRobot {
    fn new(data: &[u8]) -> Self {
        let mut robot = CleaningRobot::default();
        for (y, row) in data.split(|&b| b == 10).enumerate() {
            for (x, ch) in row.iter().enumerate() {
                match *ch as char {
                    '#' => { robot.scaffold.insert(Coord::new(x as i32, y as i32)); }
                    '.' => {}
                    '^' => { robot.scaffold.insert(Coord::new(x as i32, y as i32)); robot.pos = Coord::new(x as i32, y as i32); robot.dir = UP; }
                    'v' => { robot.scaffold.insert(Coord::new(x as i32, y as i32)); robot.pos = Coord::new(x as i32, y as i32); robot.dir = DN; }
                    '<' => { robot.scaffold.insert(Coord::new(x as i32, y as i32)); robot.pos = Coord::new(x as i32, y as i32); robot.dir = LT; }
                    '>' => { robot.scaffold.insert(Coord::new(x as i32, y as i32)); robot.pos = Coord::new(x as i32, y as i32); robot.dir = RT; }
                     _  => { panic!("unrecognized char: {:?}", *ch as char); }
                }
            }
        }
        robot
    }

    fn alignment_parameter(&self) -> i32 {
        self.scaffold
            .iter()
            .filter(|&a| [UP, DN, LT, RT]
                .iter()
                .all(|b| self.scaffold.contains(&(a + b))))
            .map(|a| a.re * a.im)
            .sum()
    }

    fn walk(&self) -> Walker {
        Walker::new(&self)
    }
}

//
// struct Walker
//

struct Walker<'a> {
    robot: &'a CleaningRobot,
    pos: Coord,
    dir: Coord,
}

impl<'a> Walker<'a> {
    fn new(robot: &'a CleaningRobot) -> Self {
        Self {
            robot,
            pos: robot.pos,
            dir: robot.dir,
        }
    }
}

impl Iterator for Walker<'_> {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pos + self.dir;
        if self.robot.scaffold.contains(&next) {
            self.pos = next;
            return Some(Action::MoveForward(None));
        }

        let left = self.pos + self.dir * CCW;
        if self.robot.scaffold.contains(&left) {
            self.dir *= CCW;
            return Some(Action::TurnLeft);
        }

        let right = self.pos + self.dir * CW;
        if self.robot.scaffold.contains(&right) {
            self.dir *= CW;
            return Some(Action::TurnRight);
        }

        None
    }
}

//
// solution
//

fn display_list<T: std::fmt::Display>(list: &[T]) -> String {
    list.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(",")
}

pub fn day17a(vm: &intcode::VM) -> i32 {
    let output = vm.clone().run(&mut vec![]);
    let bytes = output.iter().map(|&w| w as u8).collect::<Vec<u8>>();
    let robot = CleaningRobot::new(&bytes);
    robot.alignment_parameter()
}

pub fn day17_main(vm: &intcode::VM) -> Result<(), Box<dyn std::error::Error>> {
    let output = vm.clone().run(&mut vec![]);
    let bytes = output.iter().map(|&w| w as u8).collect::<Vec<u8>>();
    let robot = CleaningRobot::new(&bytes);
    let answer = robot.alignment_parameter();
    let route = robot.walk().collect::<Vec<Action>>();

    let mut stdout = std::io::stdout();
    stdout.write_all(&bytes)?;
    writeln!(stdout, "*** ROBOT START: POSITION={} DIRECTION={}", robot.pos, robot.dir)?;
    writeln!(stdout, "*** ALIGNMENT PARAMETER: {}", answer)?;
    writeln!(stdout, "*** RAW ROUTE: {}", display_list(&route))?;
    Ok(())
}

//
// tests
//

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    fn test_17() -> Result<(), Box<dyn std::error::Error>> {
        let vm = util::get_parsed_line("input/day17.txt")?;
        assert_eq!(super::day17a(&vm), 3292);
        Ok(())
    }
}
