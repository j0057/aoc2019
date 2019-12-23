use std::io::BufRead;
use std::io::Read;
use std::error::Error;
use std::str::FromStr;

pub fn get_text(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = std::fs::File::open(filename)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

pub fn get_lines(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let result = reader.lines().collect::<Result<Vec<String>, std::io::Error>>()?;
    Ok(result)
}

pub fn get_line(filename: &str) -> Result<String, Box<dyn Error>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let result = reader.lines().nth(0).ok_or("no first line found")??;
    Ok(result)
}

pub fn get_parsed<T: FromStr>(filename: &str) -> Result<T, Box<dyn Error>> where T::Err : Error + 'static {
    let result = get_text(filename)?.parse::<T>()?;
    Ok(result)
}

pub fn get_parsed_lines<T: FromStr>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> where T::Err : Error + 'static {
    let result: Vec<T> = get_lines(filename)?
        .iter()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result)
}

pub fn get_parsed_line<T: FromStr>(filename: &str) -> Result<T, Box<dyn Error>> where T::Err : Error + 'static {
    let result = get_line(filename)?.parse()?;
    Ok(result)
}
