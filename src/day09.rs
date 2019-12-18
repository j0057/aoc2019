use crate::intcode;

pub fn run_with_input(vm: &mut intcode::VM, input: &[i128]) -> i128 {
    let mut output = vec![];
    vm.run(&mut input.to_vec(), &mut output);
    output.last().unwrap().clone()
}

pub fn day09a(vm: &intcode::VM) -> i128 {
    run_with_input(&mut vm.clone(), &[1])
}

pub fn day09b(vm: &intcode::VM) -> i128 {
    run_with_input(&mut vm.clone(), &[2])
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::intcode;
    use crate::util;

    #[test]
    fn test_09_ex1() { // should produce copy of itself as output (quine)
        let program = &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        let mut vm = intcode::VM::new(program);
        let mut output = vec![];
        vm.run(&mut vec![], &mut output);
        assert_eq!(output, program);
    }

    #[test]
    fn test_09_ex2() { // should output 16-digit number
        let mut vm = intcode::VM::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        let mut output = vec![];
        vm.run(&mut vec![], &mut output);
        assert_eq!(output.last().unwrap().to_string().len(), 16);
    }

    #[test]
    fn test_09_ex3() { // should output the large number in the middle
        let mut vm = intcode::VM::new(&[104, 1125899906842624, 99]);
        let mut output = vec![];
        vm.run(&mut vec![], &mut output);
        assert_eq!(output[0], 1125899906842624);
    }

    #[test]
    fn test_09_rel_mul() { // test relative mul
        let mut vm = intcode::VM::new(&[109, 6, 202, 1, 8, 9, 99, 23, 42, 0]);
        vm.run(&mut vec![], &mut vec![]);
        assert_eq!(vm.memory[9], 966);
    }

    #[test]
    fn test_09_rel_in() { // test relative in
        let mut vm = intcode::VM::new(&[109, 4, 203, 1, 99, 0]);
        vm.run(&mut vec![23], &mut vec![]);
        assert_eq!(vm.memory[5], 23);
    }

    #[test]
    fn test_09() -> Result<(), Box<dyn Error>> {
        let program = util::get_parsed_line::<intcode::VM>("input/day09.txt")?;
        assert_eq!(super::day09a(&program), 2932210790);
        assert_eq!(super::day09b(&program), 73144);
        Ok(())
    }
}
