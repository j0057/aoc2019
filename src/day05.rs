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

    use crate::day02;
    use crate::util;

    #[test]
    fn test_05_ex1() { // outputs whatever it gets as input, then halts
        let mut program = vec![3, 0, 4, 0, 99];
        let mut input = vec![1234567890];
        let mut output = vec![];
        day02::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[1234567890]);
    }

    #[test]
    fn test_05_ex2() { // multiply using position and immediate mode
        let mut program = vec![1002, 4, 3, 4, 33];
        let mut input = vec![];
        let mut output = vec![];
        day02::run(&mut program, &mut input, &mut output);
        assert_eq!(program, &[1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_05() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day05.txt")?;
        assert_eq!(super::day05a(&program), 5346030);
        Ok(())
    }
}
