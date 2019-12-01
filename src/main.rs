mod util;
mod day01;

fn puzzle<T, A>(day: u8,
                part: char,
                parse: Box<dyn Fn(&str) -> Result<Vec<T>, Box<dyn std::error::Error>>>,
                solve: Box<dyn Fn(&[T]) -> A>)
               -> Result<(), Box<dyn std::error::Error>>
               where A: std::fmt::Display {
    let input: Vec<T> = parse(&format!("input/day{:02}.txt", day))?;
    let start = std::time::SystemTime::now();
    let answer: A = solve(&input);
    let elapsed = start.elapsed()?.as_micros();
    println!("problem: {:>2}{}; time: {:9} μs; answer: {}", day, part, elapsed, answer);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Advent of Code 2019, by j0057");

    puzzle::<u32, u32>(1, 'A', Box::new(util::get_numbers::<u32>), Box::new(day01::day01a))?;
    puzzle::<u32, u32>(1, 'B', Box::new(util::get_numbers::<u32>), Box::new(day01::day01b))?;

    Ok(())
}
