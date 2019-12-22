mod csiseq;
mod intcode;
mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;
mod day09;
mod day11;

extern crate itertools;
extern crate num_complex;
extern crate permutohedron;

use std::fmt::Display;
use std::error::Error;

fn format_thousands(n: u128) -> String {
    if n > 0 { format!("{} {:3}", format_thousands(n / 1000), n % 1000) } else { "".to_owned() }
}

fn puzzle<T: AsRef<U>, U: ?Sized, A: Display>(
        day: u8,
        part: char,
        parse: Box<dyn Fn(&str) -> Result<T, Box<dyn Error>>>,
        solve: Box<dyn Fn(&U) -> A>)
        -> Result<(), Box<dyn Error>> {
    let input = parse(&format!("input/day{:02}.txt", day))?;
    let start = std::time::SystemTime::now();
    let answer = solve(input.as_ref());
    let ns = start.elapsed()?.as_nanos();
    println!("{:>2}{} {:>15} {:>15}", day, part, format_thousands(ns), answer);
    Ok(())
}

fn puzzles() -> Result<(), Box<dyn Error>> {
    println!("--- --------------- ---------------");
    println!(" #    s  ms  μs  ns          answer           Advent of Code 2019, by j0057 🎄");
    println!("--- --------------- ---------------");

    puzzle(1, 'a', Box::new(util::get_parsed_lines::<u32>), Box::new(day01::day01a))?;
    puzzle(1, 'b', Box::new(util::get_parsed_lines::<u32>), Box::new(day01::day01b))?;

    puzzle(2, 'a', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day02::day02a))?;
    puzzle(2, 'b', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day02::day02b))?;

    puzzle(3, 'a', Box::new(util::get_parsed_lines::<day03::Input>), Box::new(day03::day03a))?;
    puzzle(3, 'b', Box::new(util::get_parsed_lines::<day03::Input>), Box::new(day03::day03b))?;

    puzzle(4, 'a', Box::new(util::get_parsed_lines::<day04::Input>), Box::new(day04::day04a))?;
    puzzle(4, 'b', Box::new(util::get_parsed_lines::<day04::Input>), Box::new(day04::day04b))?;

    puzzle(5, 'a', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day05::day05a))?;
    puzzle(5, 'b', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day05::day05b))?;

    puzzle(7, 'a', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day07::day07a))?;
    puzzle(7, 'b', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day07::day07b))?;

    puzzle(9, 'a', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day09::day09a))?;
    puzzle(9, 'b', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day09::day09b))?;

    puzzle(11, 'a', Box::new(util::get_parsed_line::<intcode::VM>), Box::new(day11::day11a))?;

    println!("--- --------------- ---------------");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg = std::env::args().skip(1).nth(0);
    match arg.as_ref().map(String::as_str) {
        Some("11b") => { let input = util::get_parsed_line::<intcode::VM>("input/day11.txt")?;
                         day11::day11_main(&input)?; },
        Some(_)     => (),
        None        => puzzles()?
    };
    Ok(())
}
