use crate::day02;

pub fn day05a(program: &[i128]) -> i128 {
    let mut memory = program.to_vec();
    let mut input = vec![1];
    let mut output = vec![];
    day02::run(&mut memory, &mut input, &mut output);
    *output.last().expect("program did not output anything")
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_05() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day05.txt")?;
        assert_eq!(super::day05a(&program), 5346030);
        Ok(())
    }
}
