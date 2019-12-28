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

pub fn day12a(bodies: &[Body]) -> i64 {
    energy(bodies, 1000)
}

//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    fn parse<T: std::str::FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
        input
            .split('\n')
            .map(|line| line.parse::<T>())
            .collect::<Result<Vec<T>, T::Err>>()
    }

    #[test]
    fn test_12_ex1() -> Result<(), Box<dyn Error>> {
        let input = parse::<super::Body>("<x=-1, y=0, z=2>\n\
                                          <x=2, y=-10, z=-7>\n\
                                          <x=4, y=-8, z=8>\n\
                                          <x=3, y=5, z=-1>")?;
        assert_eq!(super::energy(&input, 10), 179);
        Ok(())
    }

    #[test]
    fn test_12_ex2() -> Result<(), Box<dyn Error>> {
        let input = parse::<super::Body>("<x=-8, y=-10, z=0>\n\
                                          <x=5, y=5, z=10>\n\
                                          <x=2, y=-7, z=3>\n\
                                          <x=9, y=-8, z=-3>")?;
        assert_eq!(super::energy(&input, 100), 1940);
        Ok(())
    }

    #[test]
    fn test_12() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_lines::<super::Body>("input/day12.txt")?;
        assert_eq!(super::day12a(&input), 12773);
        Ok(())
    }
}
