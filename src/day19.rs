use crate::intcode::VM;

pub fn day19a(vm: &VM) -> i128 {
    let mut input = (0..50).flat_map(|y| (0..50).map(move |x| vec![x, y])).flatten().collect::<Vec<_>>();
    let mut output = vec![];
    while ! input.is_empty() {
        vm.clone().run_with(&mut input, &mut output);
    }
    output.iter().sum()
}

pub fn day19b(vm: &VM) -> i128 {
    let mut x = 0;
    let mut y = 100;
    loop {
        while vm.clone().run(&mut vec![x, y]) != [1] {
            x += 1;
        }
        if vm.clone().run(&mut vec![x + 99, y - 99]) == [1] {
            break
        }
        y += 1;
    }
    x * 10_000 + (y - 99)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_19() -> Result<(), Box<dyn std::error::Error>> {
        let vm = crate::util::get_parsed_line::<crate::intcode::VM>("input/day19.txt")?;
        assert_eq!(super::day19a(&vm), 138);
        assert_eq!(super::day19b(&vm), 13530764);
        Ok(())
    }
}
