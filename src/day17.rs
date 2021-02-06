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
// ActionParseError
//

#[derive(Debug, thiserror::Error)]
enum ActionParseError {
    #[error("Unexpected character {0:?}")]
    UnknownCharacter(String),
    #[error("Bad number {0:?}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

//
// enum Action
//

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Action {
    TurnLeft,
    MoveForward(Option<i32>),
    TurnRight,
    CallSub(char),
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Action::TurnLeft             => "L".into(),
            Action::MoveForward(None)    => "F".into(),
            Action::MoveForward(Some(x)) => x.to_string(),
            Action::TurnRight            => "R".into(),
            Action::CallSub(n)           => n.to_string(),
        })
    }
}

impl std::str::FromStr for Action {
    type Err = ActionParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "L"         => Ok(Action::TurnLeft),
            "R"         => Ok(Action::TurnRight),
            "A"|"B"|"C" => Ok(Action::CallSub(text.chars().next().unwrap())),
            _ if text.chars().all(|ch| ch.is_digit(10))
                        => Ok(Action::MoveForward(Some(text.parse::<i32>()?))),
            _           => Err(ActionParseError::UnknownCharacter(text.into()))
        }
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

fn compressed(steps: &[Action]) -> Vec<Action> {
    let mut result = Vec::<Action>::new();
    for step in steps {
        match step {
            Action::TurnLeft             => { result.push(*step) }
            Action::TurnRight            => { result.push(*step) }
            Action::MoveForward(Some(_)) => { panic!("unexpected {:?}", step) }
            Action::MoveForward(None)    => {
                match result.last_mut() {
                    None       => { result.push(Action::MoveForward(Some(1))) }
                    Some(last) => { match *last {
                        Action::MoveForward(Some(i)) => { *last = Action::MoveForward(Some(i + 1)) }
                        Action::MoveForward(None)    => { panic!("unexpected {:?}", *last) }
                        Action::CallSub(_)           => { panic!("unexpected {:?}", *last) }
                        _                            => { result.push(Action::MoveForward(Some(1))) }
                    } }
                }
            }
            Action::CallSub(_)           => { panic!("unexpected {:?}", step) }
        }
    }
    result
}

fn subroutines(program: &[Action], subs: [Vec<Action>; 4], first_empty: usize) -> Option<[Vec<Action>; 4]> {
    // return result when entire program is processed
    if program.is_empty() {
        if first_empty != 4 {
            return None;
        }
        if subs.iter().any(|s| display_list(s).len() > 20) {
            return None;
        }
        return Some(subs);
    }

    // try to apply earlier subroutine and recursively go for end of program
    for (i, v) in subs[1..].iter().enumerate() {
        if (!v.is_empty()) && program.starts_with(&v) {
            let call = Action::CallSub(((i as u8) + 65) as char);
            let main = subs[0].iter().chain(&[call]).copied().collect::<Vec<Action>>();
            return subroutines(&program[v.len()..], [main, subs[1].clone(), subs[2].clone(), subs[3].clone()], first_empty);
        }
    }

    // make a new subroutine and try to recursively reach the end
    if first_empty < 4 {
        for i in 1..program.len() {
            let call = Action::CallSub(((first_empty as u8) + 64) as char);
            let main = subs[0].iter().chain(&[call]).copied().collect::<Vec<Action>>();
            let new_subs = [main,
                            if first_empty == 1 { program[0..i].to_vec() } else { subs[1].clone() },
                            if first_empty == 2 { program[0..i].to_vec() } else { subs[2].clone() },
                            if first_empty == 3 { program[0..i].to_vec() } else { subs[3].clone() }];
            match subroutines(&program[i..], new_subs, first_empty+1) {
                None         => continue,
                Some(result) => return Some(result),
            };
        }
    }

    // no dice
    None
}

fn run_robot(mut vm: intcode::VM, subs: &[Vec<Action>; 4]) -> i128 {
    let mut subs = subs
        .iter()
        .flat_map(|sub| display_list(&sub)
                            .bytes()
                            .map(|b| b as i128)
                            .chain(vec![10])
                            .collect::<Vec<_>>())
        .chain(vec![110, 10])
        .collect::<Vec<_>>();
    vm.memory[0] = 2;
    let output = vm.run(&mut subs);
    *output.last().unwrap()
}

fn display_list<T: std::fmt::Display>(list: &[T]) -> String {
    list.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(",")
}

pub fn day17a(vm: &intcode::VM) -> i32 {
    let output = vm.clone().run(&mut vec![]);
    let bytes = output.iter().map(|&w| w as u8).collect::<Vec<u8>>();
    let robot = CleaningRobot::new(&bytes);
    robot.alignment_parameter()
}


pub fn day17b(vm: &intcode::VM) -> i128 {
    let output = vm.clone().run(&mut vec![]);
    let bytes = output.iter().map(|&w| w as u8).collect::<Vec<u8>>();
    let robot = CleaningRobot::new(&bytes);
    let route = robot.walk().collect::<Vec<Action>>();
    let prog = compressed(&route);
    let subs = subroutines(&prog, [vec![], vec![], vec![], vec![]], 1).unwrap();
    run_robot(vm.clone(), &subs)
}

pub fn day17_main(vm: &intcode::VM) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();

    let output = vm.clone().run(&mut vec![]);
    let bytes = output.iter().map(|&w| w as u8).collect::<Vec<u8>>();
    stdout.write_all(&bytes)?;

    let robot = CleaningRobot::new(&bytes);
    writeln!(stdout, "*** ROBOT START: POSITION={} DIRECTION={}", robot.pos, robot.dir)?;

    let answer = robot.alignment_parameter();
    writeln!(stdout, "*** ALIGNMENT PARAMETER: {}", answer)?;

    let route = robot.walk().collect::<Vec<Action>>();
    let prog = compressed(&route);
    writeln!(stdout, "*** PROGRAM: {}", display_list(&prog))?;

    let subs = subroutines(&prog, [vec![], vec![], vec![], vec![]], 1).unwrap();
    writeln!(stdout, "*** MAIN: {}", display_list(&subs[0]))?;
    writeln!(stdout, "*** A: {}", display_list(&subs[1]))?;
    writeln!(stdout, "*** B: {}", display_list(&subs[2]))?;
    writeln!(stdout, "*** C: {}", display_list(&subs[3]))?;

    let dust_collected = run_robot(vm.clone(), &subs);
    writeln!(stdout, "*** DUST COLLECTED: {}", dust_collected)?;

    Ok(())
}

//
// tests
//

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_17_1() -> Result<(), Box<dyn std::error::Error>> {
        // R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2
        // A      |B          |C      |B          |A      |C      | <-- "one example"
        // A  |A  |B      |A  |C      |B      |A  |A  |A  |C      | <-- this also works!
        // A                  |B      |C                  |B      | <-- this also works... oh well...
        let program = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
            .split(',')
            .map(|t| t.parse::<Action>())
            .collect::<Result<Vec<Action>, ActionParseError>>()?;
        println!("{}", display_list(&program));
        match subroutines(&program, [vec![], vec![], vec![], vec![]], 1) {
            None         => { panic!("no solution found") },
            Some(result) => { assert_eq!(result[0]
                                            .iter()
                                            .flat_map(|c| match c {
                                                Action::CallSub(s) => result[((*s as u8) - 64) as usize].iter().cloned(),
                                                _                  => panic!("unexpected {:?}", c),
                                            })
                                            .collect::<Vec<_>>(), program) },
        };
        Ok(())
    }

    #[test]
    fn test_17() -> Result<(), Box<dyn std::error::Error>> {
        let vm = util::get_parsed_line("input/day17.txt")?;
        assert_eq!(day17a(&vm), 3292);
        assert_eq!(day17b(&vm), 651043);
        Ok(())
    }
}
