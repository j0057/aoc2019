#[derive(Debug)]
pub enum Status {
    Running,
    Halted,
    Blocked,
    Suspended
}

pub fn run(m: &mut [i128], input: &mut Vec<i128>, output: &mut Vec<i128>) -> () {
    let mut ip = 0;
    loop {
        if let Status::Halted = step(&mut ip, m, input, output) {
            break;
        }
    }
}

pub fn step(ip: &mut usize, m: &mut [i128], input: &mut Vec<i128>, output: &mut Vec<i128>) -> Status
{
    loop {
        let p = |i| { match m[*ip] / 10_i128.pow((i as u32)+1) % 10 {
                        0 => m[m[*ip+i] as usize],
                        1 =>   m[*ip+i],
                        _ => panic!("bad opcode {} at IP {}", m[*ip], *ip)
                    } };
        match m[*ip] % 100 {
            // day 2 : add
            1   => { m[m[*ip+3] as usize] = p(1) + p(2);
                     *ip += 4; },

            // day 2 : mul
            2   => { m[m[*ip+3] as usize] = p(1) * p(2);
                     *ip += 4; },

            // day 5 : in
            3   => { if input.is_empty() {
                        return Status::Blocked;
            }
            m[m[*ip+1] as usize] = input.remove(0);
                     *ip += 2; },

            // day 5 : out
            4   => { output.push(p(1));
                     *ip += 2;
                     return Status::Suspended;
                   }

            // day 5 : jnz
            5   => { *ip = if p(1) != 0 { p(2) as usize } else { *ip + 3 } },

            // day 5 : jz
            6   => { *ip = if p(1) == 0 { p(2) as usize } else { *ip + 3 } },

            // day 5 : lt
            7   => { m[m[*ip+3] as usize] = if p(1) < p(2) { 1 } else { 0 };
                     *ip += 4; },

            // day 5 : eq
            8   => { m[m[*ip+3] as usize] = if p(1) == p(2) { 1 } else { 0 };
                     *ip += 4 },

            // day 2 : halt
            99  => return Status::Halted,

            // day 2 : wtf
            _   => panic!("unrecognized opcode {}; IP={}", m[*ip], *ip)
        }
    }
}

pub fn day02a(program: &[i128]) -> i128 {
    let mut m = program.to_vec();
    m[1] = 12;
    m[2] = 2;
    run(&mut m, &mut vec![], &mut vec![]);
    m[0]
}

pub fn day02b(program: &[i128]) -> i128 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut m = program.to_vec();
            m[1] = noun;
            m[2] = verb;
            run(&mut m, &mut vec![], &mut vec![]);
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

    use crate::util;

    #[test]
    fn test_02_ex1() {
        let mut program = vec![1, 0, 0, 0, 99];
        super::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_02_ex2() {
        let mut program = vec![2, 3, 0, 3, 99];
        super::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_02_ex3() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        super::run(&mut program, &mut vec![], &mut vec![]);
        assert_eq!(program, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_02_ex4() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        super::run(&mut program, &mut vec![], &mut vec![]);
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
