use std::io::BufRead;

pub fn get_lines(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let result = reader.lines().collect::<Result<Vec<String>, std::io::Error>>()?;
    Ok(result)
}

pub fn get_numbers<T>(filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where T: std::str::FromStr<Err=std::num::ParseIntError> {
    let result = get_lines(filename)?
        .iter()
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, std::num::ParseIntError>>()?;
    Ok(result)
}
