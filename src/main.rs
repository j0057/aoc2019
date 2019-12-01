mod util;
mod day01;

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
    let elapsed = start.elapsed()?.as_micros();
    println!("problem: {:>2}{}; time: {:9} μs; answer: {}", day, part, elapsed, answer);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Advent of Code 2019, by j0057");

    puzzle(1, 'A', Box::new(util::get_numbers::<u32>), Box::new(day01::day01a))?;
    puzzle(1, 'B', Box::new(util::get_numbers::<u32>), Box::new(day01::day01b))?;

    Ok(())
}
