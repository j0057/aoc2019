use crate::intcode;

pub fn day02a(program: &[i128]) -> i128 {
    let mut m = program.to_vec();
    m[1] = 12;
    m[2] = 2;
    intcode::run(&mut m, &mut vec![], &mut vec![]);
    m[0]
}

pub fn day02b(program: &[i128]) -> i128 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut m = program.to_vec();
            m[1] = noun;
            m[2] = verb;
            intcode::run(&mut m, &mut vec![], &mut vec![]);
            if m[0] == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    panic!("no solution found")
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::intcode;
    use crate::util;

    #[test]
    fn test_02_ex1() {
        let mut program = vec![1, 0, 0, 0, 99];
        intcode::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_02_ex2() {
        let mut program = vec![2, 3, 0, 3, 99];
        intcode::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_02_ex3() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        intcode::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_02_ex4() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        intcode::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_02() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day02.txt")?;
        assert_eq!(super::day02a(&program), 4930687);
        assert_eq!(super::day02b(&program), 5335);
        Ok(())
    }
}
