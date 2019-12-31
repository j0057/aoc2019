use std::cell::RefCell;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

//
// enum ParseError
//

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Text could not be parsed: {0:?}")]
    Parse(Option<String>),

    #[error("Bad number: {0:?}")]
    BadInt(#[from] std::num::ParseIntError),
}

//
// struct Vector
//

#[derive(Debug, Clone)]
pub struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::str::FromStr for Vector {
    type Err = ParseError;

    fn from_str(repr: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref VECTOR_RE: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        }
        let captures = VECTOR_RE.captures(repr).ok_or_else(|| ParseError::Parse(Some(repr.to_owned())))?;
        let result = Vector {
            x: captures.get(1).ok_or_else(|| ParseError::Parse(Some(repr.to_owned())))?.as_str().parse::<i64>()?,
            y: captures.get(2).ok_or_else(|| ParseError::Parse(Some(repr.to_owned())))?.as_str().parse::<i64>()?,
            z: captures.get(3).ok_or_else(|| ParseError::Parse(Some(repr.to_owned())))?.as_str().parse::<i64>()?
        };
        Ok(result)
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

//
// struct Body
//

#[derive(Debug, Clone)]
pub struct Body {
    pos: Vector,
    vel: Vector
}

impl Body {
    fn update_vel(&mut self, other: &mut Body) {
        if self.pos.x > other.pos.x { self.vel.x -= 1; other.vel.x += 1 }
        if self.pos.x < other.pos.x { self.vel.x += 1; other.vel.x -= 1 }
        if self.pos.y > other.pos.y { self.vel.y -= 1; other.vel.y += 1 }
        if self.pos.y < other.pos.y { self.vel.y += 1; other.vel.y -= 1 }
        if self.pos.z > other.pos.z { self.vel.z -= 1; other.vel.z += 1 }
        if self.pos.z < other.pos.z { self.vel.z += 1; other.vel.z -= 1 }
    }

    fn update_pos(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn xs(&self) -> [i64; 2] { [self.pos.x, self.vel.x] }
    fn ys(&self) -> [i64; 2] { [self.pos.y, self.vel.y] }
    fn zs(&self) -> [i64; 2] { [self.pos.z, self.vel.z] }
}

impl std::str::FromStr for Body {
    type Err = ParseError;

    fn from_str(repr: &str) -> Result<Self, Self::Err> {
        let result = Body {
            pos: repr.parse()?,
            vel: Vector { x: 0, y: 0, z: 0 }
        };
        Ok(result)
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

//
// solution
//

fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
fn lcm(a: i64, b: i64) -> i64 { (a * b).abs() / gcd(a, b) }

fn step(bodies: &[RefCell<Body>]) {
    (0..bodies.len())
        .tuple_combinations()
        .for_each(|(a, b)| bodies[a].borrow_mut().update_vel(&mut bodies[b].borrow_mut()));

    bodies
        .iter()
        .for_each(|b| b.borrow_mut().update_pos());
}

fn as_refcells<T: Clone>(items: &[T]) -> Vec<RefCell<T>> {
    items.iter().map(|item| RefCell::new(item.clone())).collect()
}

fn energy(bodies: &[Body], steps: i64) -> i64 {
    let bodies = as_refcells(bodies);
    for _ in 1..=steps {
        step(&bodies);
    }
    bodies
        .iter()
        .map(|body| body.borrow().pos.norm() * body.borrow().vel.norm())
        .sum()
}

fn find_periods(bodies: &[Body]) -> [i64; 3] {
    let bodies = as_refcells(bodies);

    let start_x = bodies.iter().map(|b| b.borrow().xs()).collect::<Vec<_>>();
    let start_y = bodies.iter().map(|b| b.borrow().ys()).collect::<Vec<_>>();
    let start_z = bodies.iter().map(|b| b.borrow().zs()).collect::<Vec<_>>();

    let mut periods = [0; 3];

    for n in 1.. {
        if periods.iter().all(|&p| p != 0) {
            break;
        }

        step(&bodies);

        if periods[0] == 0 && start_x == bodies.iter().map(|b| b.borrow().xs()).collect::<Vec<_>>() { periods[0] = n; }
        if periods[1] == 0 && start_y == bodies.iter().map(|b| b.borrow().ys()).collect::<Vec<_>>() { periods[1] = n; }
        if periods[2] == 0 && start_z == bodies.iter().map(|b| b.borrow().zs()).collect::<Vec<_>>() { periods[2] = n; }

    }

    periods
}

pub fn day12a(bodies: &[Body]) -> i64 {
    energy(bodies, 1000)
}

pub fn day12b(bodies: &[Body]) -> i64 {
    let periods = find_periods(bodies);
    lcm(periods[0], lcm(periods[1], periods[2]))
}

//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_12_ex1() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("<x=-1, y=0, z=2>\n\
                                      <x=2, y=-10, z=-7>\n\
                                      <x=4, y=-8, z=8>\n\
                                      <x=3, y=5, z=-1>")?;
        assert_eq!(super::energy(&input, 10), 179);
        Ok(())
    }

    #[test]
    fn test_12_ex2() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("<x=-8, y=-10, z=0>\n\
                                       <x=5, y=5, z=10>\n\
                                       <x=2, y=-7, z=3>\n\
                                       <x=9, y=-8, z=-3>")?;
        assert_eq!(super::energy(&input, 100), 1940);
        Ok(())
    }

    #[test]
    fn test_12_ex3() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("<x=-1, y=0, z=2>\n\
                                       <x=2, y=-10, z=-7>\n\
                                       <x=4, y=-8, z=8>\n\
                                       <x=3, y=5, z=-1>")?;
        assert_eq!(super::day12b(&input), 2772);
        Ok(())
    }

    #[test]
    fn test_12_ex4() -> Result<(), Box<dyn Error>> {
        let input = util::parse_lines("<x=-8, y=-10, z=0>\n\
                                       <x=5, y=5, z=10>\n\
                                       <x=2, y=-7, z=3>\n\
                                       <x=9, y=-8, z=-3>")?;
        assert_eq!(super::day12b(&input), 468_6774_924);
        Ok(())
    }

    #[test]
    fn test_12() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_lines::<super::Body>("input/day12.txt")?;
        assert_eq!(super::day12a(&input), 12773);
        assert_eq!(super::day12b(&input), 306_798_770_391_636);
        Ok(())
    }
}
