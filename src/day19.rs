use crate::intcode::{VM, Status};

fn get_input() -> Vec<i128> {
    let mut input = vec![];
    for y in 0..50 {
        for x in 0..50 {
            input.push(x);
            input.push(y)
        }
    }
    input
}

fn run_program(vm: &VM) -> Vec<i128> {
    let mut input = get_input();
    let mut output = vec![];
    loop {
        match vm.clone().step(&mut input, &mut output) {
            Status::Blocked => break,
            Status::Suspended => continue,
            Status::Halted => break
        }
    }
    output
}

pub fn day19a(vm: &VM) -> i128 {
    run_program(&vm)
        .iter()
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_19() -> Result<(), Box<dyn std::error::Error>> {
        let vm = crate::util::get_parsed_line::<crate::intcode::VM>("input/day19.txt")?;
        assert_eq!(super::day19a(&vm), 138);
        Ok(())
    }
}
