use std::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;


//
// enum InputError
//

#[derive(Debug)]
pub enum InputError {
    Parse(String),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::Parse(s) => write!(f, "ParseError: unable to parse string '{:?}'", s),
        }
    }
}

impl Error for InputError {
}

//
// struct Input
//

#[derive(Debug, PartialEq)]
pub struct Input((String, String));

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Input, Self::Err> {
        let parts = &s.split(')').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(InputError::Parse(s.to_owned()))
        }
        let result = Input((parts[1].to_owned(), parts[0].to_owned()));
        Ok(result)
    }
}

//
// solution
//

fn path_len(graph: &HashMap<String, String>, node: &str) -> usize {
    match graph.get(node) {
        Some(parent) => 1 + path_len(graph, parent),
        None         => 0
    }
}

fn path(graph: &HashMap<String, String>, node: &str) -> Vec<String> {
    match graph.get(node) {
        Some(parent) => { let mut result = path(graph, parent);
                          result.push(node.to_owned());
                          result },
        None         => vec![node.to_owned()]
    }
}

pub fn day06a(edges: &[Input]) -> usize {
    let graph = edges.iter().map(|input| input.0.clone()).collect::<HashMap<String, String>>();
    graph.keys().map(|k| path_len(&graph, &k)).sum()
}

pub fn day06b(edges: &[Input]) -> usize {
    let graph = edges.iter().map(|input| input.0.clone()).collect::<HashMap<String, String>>();
    let mut path1 = path(&graph, "YOU");
    let mut path2 = path(&graph, "SAN");
    while path1[0] == path2[0] {
        path1.remove(0);
        path2.remove(0);
    }
    path1.len() + path2.len() - 2
}

//
// tests
//

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_06_ex1() -> Result<(), Box<dyn Error>> {
        let parsed = &["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"]
            .iter()
            .map(|s| s.parse::<super::Input>())
            .collect::<Result<Vec<_>, super::InputError>>()?;
        assert_eq!(super::day06a(&parsed), 42);
        Ok(())
    }

    #[test]
    fn test_06_ex2() -> Result<(), Box<dyn Error>> {
        let parsed = &["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN"]
            .iter()
            .map(|s| s.parse::<super::Input>())
            .collect::<Result<Vec<_>, super::InputError>>()?;
        assert_eq!(super::day06b(&parsed), 4);
        Ok(())
    }

    #[test]
    fn test_06() -> Result<(), Box<dyn Error>> {
        let input = util::get_parsed_lines::<super::Input>("./input/day06.txt")?;
        assert_eq!(super::day06a(&input), 186597);
        assert_eq!(super::day06b(&input), 412);
        Ok(())
    }
}
