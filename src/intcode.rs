use std::str::FromStr;

#[derive(Debug)]
pub struct VM {
    pub memory: Vec<i128>,
    pub ip: usize,
    pub bp: i128,
}

#[derive(Debug)]
pub enum Status {
    Halted,
    Blocked,
    Suspended
}

impl FromStr for VM {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let memory = text
            .split(',')
            .map(|s| s.trim().parse::<i128>())
            .collect::<Result<Vec<i128>, std::num::ParseIntError>>()?;
        Ok(VM::new(&memory))
    }
}

impl VM {
    pub fn new(program: &[i128]) -> VM {
        let result = VM {
            memory: program.to_vec(),
            ip: 0,
            bp: 0
        };
        result
    }

    pub fn run(&mut self, input: &mut Vec<i128>, output: &mut Vec<i128>) -> () {
        loop {
            if let Status::Halted = self.step(input, output) {
                break;
            }
        }
    }

    pub fn step(&mut self, input: &mut Vec<i128>, output: &mut Vec<i128>) -> Status {
        loop {
            match self.memory[self.ip] % 100 {
                // day 2 : add
                1   => { *self.arg(3) = *self.arg(1) + *self.arg(2);
                         self.ip += 4; },

                // day 2 : mul
                2   => { *self.arg(3) = *self.arg(1) * *self.arg(2);
                         self.ip += 4; },

                // day 5 : in
                3   => { if input.is_empty() {
                            return Status::Blocked;
                         }
                         *self.arg(1) = input.remove(0);
                         self.ip += 2; },

                // day 5 : out
                4   => { output.push(*self.arg(1));
                         self.ip += 2;
                         return Status::Suspended; }

                // day 5 : jnz
                5   => { self.ip = if *self.arg(1) != 0 { *self.arg(2) as usize } else { self.ip + 3 } },

                // day 5 : jz
                6   => { self.ip = if *self.arg(1) == 0 { *self.arg(2) as usize } else { self.ip + 3 } },

                // day 5 : lt
                7   => { *self.arg(3) = if *self.arg(1) < *self.arg(2) { 1 } else { 0 };
                         self.ip += 4; },

                // day 5 : eq
                8   => { *self.arg(3) = if *self.arg(1) == *self.arg(2) { 1 } else { 0 };
                         self.ip += 4 },

                // day 9 : add bp
                9   => { self.bp += *self.arg(1);
                         self.ip += 2; },

                // day 2 : halt
                99  => return Status::Halted,

                // day 2 : wtf
                _   => panic!("unrecognized opcode {}; IP={}", self.memory[self.ip], self.ip)
            };
        }
    }

    fn arg<'a>(&'a mut self, i: usize) -> &'a mut i128 {
        let o = match self.memory[self.ip] / 10_i128.pow((i as u32) + 1) % 10 {
            0 => self.memory[self.ip + i] as usize,
            1 => self.ip + i,
            2 => (self.memory[self.ip + i] + self.bp) as usize,
            _ => panic!("bad opcode {} at IP {}", self.memory[self.ip] as usize, self.ip)
        };
        if o >= self.memory.len() {
            self.memory.resize(o+1, 0);
        }
        &mut self.memory[o]
    }
}
