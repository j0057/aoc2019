use crate::intcode;

fn run_noun_verb(program: &[i128], noun: i128, verb: i128) -> i128 {
    let mut memory = program.to_vec();
    memory[1] = noun;
    memory[2] = verb;
    let mut vm = intcode::VM::new(&memory);
    vm.run(&mut vec![], &mut vec![]);
    vm.memory[0]
}

pub fn day02a(program: &[i128]) -> i128 {
    run_noun_verb(program, 12, 2)
}

pub fn day02b(program: &[i128]) -> i128 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_noun_verb(program, noun, verb) == 19690720 {
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
        let mut vm = intcode::VM::new(&[1, 0, 0, 0, 99]);
        vm.run(&mut vec![], &mut vec![]);
        assert_eq!(vm.memory, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_02_ex2() {
        let mut vm = intcode::VM::new(&[2, 3, 0, 3, 99]);
        vm.run(&mut vec![], &mut vec![]);
        assert_eq!(vm.memory, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_02_ex3() {
        let mut vm = intcode::VM::new(&[2, 4, 4, 5, 99, 0]);
        vm.run(&mut vec![], &mut vec![]);
        assert_eq!(vm.memory, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_02_ex4() {
        let mut vm = intcode::VM::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        vm.run(&mut vec![], &mut vec![]);
        assert_eq!(vm.memory, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_02() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day02.txt")?;
        assert_eq!(super::day02a(&program), 4930687);
        assert_eq!(super::day02b(&program), 5335);
        Ok(())
    }
}
