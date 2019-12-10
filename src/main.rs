mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;

extern crate itertools;
extern crate num_complex;
extern crate permutohedron;

use std::ops::Deref;
use std::fmt::Display;
use std::error::Error;

fn puzzle<T: Deref<Target=U>, U: ?Sized, A: Display>(
        day: u8,
        part: char,
        parse: Box<dyn Fn(&str) -> Result<T, Box<dyn Error>>>,
        solve: Box<dyn Fn(&U) -> A>)
        -> Result<(), Box<dyn Error>> {
    let input = parse(&format!("input/day{:02}.txt", day))?;
    let start = std::time::SystemTime::now();
    let answer = solve(&*input);
    let elapsed = start.elapsed()?.as_nanos();
    println!("{0:>2}{1} {2:>9} {3:>15}", day, part, elapsed, answer);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- --------- ---------------");
    println!(" #         ns         answer          Advent of Code 2019, by j0057 🎄");
    println!("--- --------- ---------------");

    puzzle(1, 'a', Box::new(util::get_parsed_lines::<u32>), Box::new(day01::day01a))?;
    puzzle(1, 'b', Box::new(util::get_parsed_lines::<u32>), Box::new(day01::day01b))?;

    puzzle(2, 'a', Box::new(util::get_splitted_commas_numbers::<i128>), Box::new(day02::day02a))?;
    puzzle(2, 'b', Box::new(util::get_splitted_commas_numbers::<i128>), Box::new(day02::day02b))?;

    puzzle(3, 'a', Box::new(util::get_parsed_lines::<day03::Input>), Box::new(day03::day03a))?;
    puzzle(3, 'b', Box::new(util::get_parsed_lines::<day03::Input>), Box::new(day03::day03b))?;

    puzzle(4, 'a', Box::new(util::get_parsed_lines::<day04::Input>), Box::new(day04::day04a))?;
    puzzle(4, 'b', Box::new(util::get_parsed_lines::<day04::Input>), Box::new(day04::day04b))?;

    puzzle(5, 'a', Box::new(util::get_splitted_commas_numbers::<i128>), Box::new(day05::day05a))?;
    puzzle(5, 'b', Box::new(util::get_splitted_commas_numbers::<i128>), Box::new(day05::day05b))?;

    puzzle(7, 'a', Box::new(util::get_splitted_commas_numbers::<i128>), Box::new(day07::day07a))?;

    println!("--- --------- ---------------");

    Ok(())
}
