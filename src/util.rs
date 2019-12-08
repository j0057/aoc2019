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

pub fn get_splitted(filename: &str, ch: char) -> Result<Vec<String>, Box<dyn Error>> {
    let text: String = std::fs::read_to_string(filename)?;
    let result = text.split(ch).map(|s| s.into()).collect();
    Ok(result)
}

pub fn get_parsed_lines<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> where T::Err : Error + 'static {
    let result: Vec<T> = get_lines(filename)?
        .iter()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result)
}

pub fn get_splitted_commas_numbers<T: FromStr<Err=ParseIntError>>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let result = get_splitted(filename, ',')?
        .iter()
        .map(|s| s.trim().parse::<T>())
        .collect::<Result<Vec<T>, ParseIntError>>()?;
    Ok(result)
}
