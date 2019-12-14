#[derive(Debug)]
pub enum Status {
    Running,
    Halted,
    Blocked,
    Suspended
}

pub fn run(m: &mut [i128], input: &mut Vec<i128>, output: &mut Vec<i128>) -> () {
    let mut ip = 0;
    let mut bp = 0;
    loop {
        if let Status::Halted = step(&mut ip, &mut bp, m, input, output) {
            break;
        }
    }
}

pub fn step(ip: &mut usize, bp: &mut usize, m: &mut [i128], input: &mut Vec<i128>, output: &mut Vec<i128>) -> Status
{
    loop {
        let get = |i| { match m[*ip] / 10_i128.pow((i as u32)+1) % 10 {
                            0 => m[m[*ip+i] as usize],
                            1 =>   m[*ip+i],
                            2 => m[(m[*ip+i] + *bp as i128) as usize],
                            _ => panic!("bad opcode {} at IP {}", m[*ip] as usize, *ip)
                      } };
        match m[*ip] % 100 {
            // day 2 : add
            1   => { m[m[*ip+3] as usize] = get(1) + get(2);
                     *ip += 4; },

            // day 2 : mul
            2   => { m[m[*ip+3] as usize] = get(1) * get(2);
                     *ip += 4; },

            // day 5 : in
            3   => { if input.is_empty() {
                        return Status::Blocked;
                     }
                     m[m[*ip+1] as usize] = input.remove(0);
                     *ip += 2; },

            // day 5 : out
            4   => { output.push(get(1));
                     *ip += 2;
                     return Status::Suspended; }

            // day 5 : jnz
            5   => { *ip = if get(1) != 0 { get(2) as usize } else { *ip + 3 } },

            // day 5 : jz
            6   => { *ip = if get(1) == 0 { get(2) as usize } else { *ip + 3 } },

            // day 5 : lt
            7   => { m[m[*ip+3] as usize] = if get(1) < get(2) { 1 } else { 0 };
                     *ip += 4; },

            // day 5 : eq
            8   => { m[m[*ip+3] as usize] = if get(1) == get(2) { 1 } else { 0 };
                     *ip += 4 },

            // day 9 : add bp
            9   => { *bp += get(1) as usize;
                     *ip += 2; },

            // day 2 : halt
            99  => return Status::Halted,

            // day 2 : wtf
            _   => panic!("unrecognized opcode {}; IP={}", m[*ip], *ip)
        }
    }
}
