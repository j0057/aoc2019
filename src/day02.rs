pub fn run(m: &mut [i128]) -> () {
    let mut ip = 0;
    loop {
        match m[ip] {
            // day 2 : add
            1   => { m[m[ip+3] as usize] = m[m[ip+1] as usize] + m[m[ip+2] as usize];
                     ip += 4; },

            // day 2 : mul
            2   => { m[m[ip+3] as usize] = m[m[ip+1] as usize] * m[m[ip+2] as usize];
                     ip += 4; },

            // day 2 : halt
            99  => break,

            // day 2 : wtf
            _   => panic!(format!("unrecognized opcode {}; IP={}", m[ip], ip))
        }
    }
}

pub fn run_inspect(program: &[i128], x: usize) -> i128 {
    let mut memory = program.to_vec();
    run(&mut memory);
    memory[x]
}

pub fn day02a(program: &[i128]) -> i128 {
    let mut m = program.to_vec();
    m[1] = 12;
    m[2] = 2;
    run_inspect(&m, 0)
}

pub fn day02b(program: &[i128]) -> i128 {
    let mut m = program.to_vec();
    for noun in 0..100 {
        for verb in 0..100 {
            m[1] = noun;
            m[2] = verb;
            if run_inspect(&m, 0) == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    panic!("no solution found")
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_02_ex1() {
        assert_eq!(super::run_inspect(&[1, 0, 0, 0, 99], 0), 2);
    }

    #[test]
    fn test_02_ex2() {
        assert_eq!(super::run_inspect(&[2, 3, 0, 3, 99], 3), 6);
    }

    #[test]
    fn test_02_ex3() {
        assert_eq!(super::run_inspect(&[2, 4, 4, 5, 99, 0], 5), 9801);
    }

    #[test]
    fn test_02_ex4() {
        assert_eq!(super::run_inspect(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 0), 30);
    }

    #[test]
    fn test_02() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day02.txt")?;
        assert_eq!(super::day02a(&program), 4930687);
        assert_eq!(super::day02b(&program), 5335);
        Ok(())
    }
}
