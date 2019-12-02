use std::io::BufRead;
use std::error::Error;
use std::str::FromStr;
use std::num::ParseIntError;

pub fn get_lines(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let result = reader.lines().collect::<Result<Vec<String>, std::io::Error>>()?;
    Ok(result)
}

pub fn get_numbers<T: FromStr<Err=ParseIntError>>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let result = get_lines(filename)?
        .iter()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, ParseIntError>>()?;
    Ok(result)
}
