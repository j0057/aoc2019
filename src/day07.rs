use permutohedron;

use crate::day02;

pub fn day07a(program: &[i128]) -> i128 {
    permutohedron::Heap::new(&mut [0, 1, 2, 3, 4])
        .map(|phases| phases
             .iter()
             .scan(0, |state, &phase| {
                let mut m = program.to_vec();
                let mut i = vec![phase, *state];
                let mut o = vec![];
                day02::run(&mut m, &mut i, &mut o);
                *state = *o.last().unwrap();
                Some(*state)
            })
            .last()
            .unwrap()
        )
        .max()
        .unwrap()
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
    fn test_07() -> Result<(), Box<dyn Error>> {
        let program = util::get_splitted_commas_numbers::<i128>("input/day07.txt")?;
        assert_eq!(super::day07a(&program), 22012);
        Ok(())
    }

}
