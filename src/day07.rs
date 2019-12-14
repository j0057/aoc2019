use std::cell::RefCell;

use permutohedron;

use crate::intcode;

pub fn day07a(program: &[i128]) -> i128 {
    permutohedron::Heap::new(&mut [0, 1, 2, 3, 4])
        .map(|phases| phases
             .iter()
             .scan(0, |state, &phase| {
                let mut m = program.to_vec();
                let mut i = vec![phase, *state];
                let mut o = vec![];
                intcode::run(&mut m, &mut i, &mut o);
                *state = *o.last().unwrap();
                Some(*state)
            })
            .last()
            .unwrap()
        )
        .max()
        .unwrap()
}

pub fn day07b(p: &[i128]) -> i128 {
    let mut max = 0;
    for phases in permutohedron::Heap::new(&mut [5, 6, 7, 8, 9]) {
        let st = &mut [intcode::Status::Running, intcode::Status::Running, intcode::Status::Running, intcode::Status::Running, intcode::Status::Running];
        let ip = &mut [0, 0, 0, 0, 0];
        let bp = &mut [0, 0, 0, 0, 0];
        let m = &mut [p.to_vec(), p.to_vec(), p.to_vec(), p.to_vec(), p.to_vec()];
        let b = [RefCell::new(vec![]), RefCell::new(vec![]), RefCell::new(vec![]), RefCell::new(vec![]), RefCell::new(vec![])];

        for i in 0..=4 {
            b[i].borrow_mut().push(phases[i]);
        }
        b[0].borrow_mut().push(0);

        loop {
            if st.iter().all(|s| if let intcode::Status::Halted = s { true } else { false }) {
                let v = b[0].borrow().last().unwrap().clone();
                if v > max {
                    max = v;
                }
                break;
            }
            for i in 0..=4 {
                if let intcode::Status::Halted = st[i] {
                    continue
                }
                st[i] = intcode::step(&mut ip[i], &mut bp[i], &mut m[i], &mut b[i].borrow_mut(), &mut b[(i+1)%5].borrow_mut());
                //println!("Status:{:?}; machine:{:?}, IP:{:?}, buffers:{:?}", st[i], i, ip[i], b);
            }
        }
    }
    max
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::util;

    #[test]
    fn test_07_ex1() {
        assert_eq!(super::day07a(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]), 43210);
    }

    #[test]
    fn test_07_ex2() {
        assert_eq!(super::day07a(&[3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0]), 54321);
    }

    #[test]
    fn test_07_ex3() {
        assert_eq!(super::day07a(&[3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0]), 65210);
    }

    #[test]
    fn test_07_ex4() {
        assert_eq!(super::day07b(&[3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5]), 139629729);
    }

    #[test]
    fn test_07_ex5() {
        assert_eq!(super::day07b(&[3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10]), 18216);
    }

    #[test]
    fn test_07() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day07.txt")?;
        assert_eq!(super::day07a(&program), 22012);
        assert_eq!(super::day07b(&program), 4039164);
        Ok(())
    }
}
