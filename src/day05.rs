use crate::intcode;

pub fn day05a(program: &[i128]) -> i128 {
    let mut memory = program.to_vec();
    let mut input = vec![1];
    let mut output = vec![];
    intcode::run(&mut memory, &mut input, &mut output);
    *output.last().expect("program did not output anything")
}

pub fn day05b(program: &[i128]) -> i128 {
    let mut memory = program.to_vec();
    let mut input = vec![5];
    let mut output = vec![];
    intcode::run(&mut memory, &mut input, &mut output);
    *output.last().expect("program did not output anything")
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::intcode;
    use crate::util;

    #[test]
    fn test_05_ex1() { // outputs whatever it gets as input, then halts
        let mut program = vec![3, 0, 4, 0, 99];
        let mut input = vec![1234567890];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[1234567890]);
    }

    #[test]
    fn test_05_ex2() { // multiply using position and immediate mode
        let mut program = vec![1002, 4, 3, 4, 33];
        let mut input = vec![];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(program, &[1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_05_ex3() { // using position mode, consider whether input is equal to 8
        let mut program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut input = vec![8];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[1]);
    }

    #[test]
    fn test_05_ex4() { // using position mode, consider whether input is less than 8
        let mut program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut input = vec![7];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[1]);
    }

    #[test]
    fn test_05_ex5() { // using immediate mode, consider whether the input is equal to 8
        let mut program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut input = vec![8];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[1]);
    }

    #[test]
    fn test_05_ex6() { // using immediate mode, consider whether the input is less than 8
        let mut program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut input = vec![7];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[1]);
    }

    #[test]
    fn test_05_ex7() { // using position mode and jump instructions, test whether input was 0
        let mut program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut input = vec![0];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[0]);
    }

    #[test]
    fn test_05_ex8() { // using immediate mode and jump instructions, test whether input was 0
        let mut program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut input = vec![0];
        let mut output = vec![];
        intcode::run(&mut program, &mut input, &mut output);
        assert_eq!(output, &[0]);
    }

    #[test]
    fn test_05() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day05.txt")?;
        assert_eq!(super::day05a(&program), 5346030);
        assert_eq!(super::day05b(&program), 513116);
        Ok(())
    }
}
