use crate::intcode;

fn survey_hull(vm: &intcode::VM, program: &[&str]) -> Result<i128, String> {
    let mut input = program
        .join("\n")
        .bytes()
        .chain(std::iter::once(10))
        .map(|b| b as i128)
        .collect();

    let output = &vm.clone().run(&mut input);

    match output.last() {
        Some(10) => Err(output.iter().map(|b| *b as u8 as char).collect()),
        Some(x)  => Ok(*x),
        None     => Err("program produced no output".to_owned()),
    }
}

pub fn day21a(vm: &intcode::VM) -> i128 {
    // J = !(A & B & C) & D
    let prog = &["NOT J T", "AND A T", "AND B T", "AND C T", "NOT T J", "AND D J", "WALK"];

    match survey_hull(&vm.clone(), prog) {
        Ok(result) => result,
        Err(diag)  => { println!("{}", diag);
                        panic!("program errored"); },
    }
}

pub fn day21b(vm: &intcode::VM) -> i128 {
    // https://www.reddit.com/r/adventofcode/comments/edll5a/2019_day_21_solutions/fbjujku/
    // (I want to program Rust, not SpringScript)
    let prog = &["NOT H J", "OR C J", "AND B J", "AND A J", "NOT J J", "AND D J", "RUN"];

    match survey_hull(&vm.clone(), prog) {
        Ok(result) => result,
        Err(diag)  => { println!("{}", diag);
                        panic!("program errored"); },
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_21_1() -> Result<(), Box<dyn std::error::Error>> {
        let vm = crate::util::get_parsed("./input/day21.txt")?;
        let diag = super::survey_hull(&vm, &["NOT D J", "WALK"]).unwrap_err();
        println!("{}", diag);
        Ok(())
    }

    #[test]
    fn test_21a() -> Result<(), Box<dyn std::error::Error>> {
        let vm = crate::util::get_parsed("./input/day21.txt")?;
        let part1 = super::day21a(&vm);
        assert_eq!(part1, 19354464);
        Ok(())
    }

    #[test]
    fn test_21b() -> Result<(), Box<dyn std::error::Error>> {
        let vm = crate::util::get_parsed("./input/day21.txt")?;
        let part2 = super::day21b(&vm);
        assert_eq!(part2, 1143198454);
        Ok(())
    }
}
