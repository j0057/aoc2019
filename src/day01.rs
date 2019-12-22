#[allow(clippy::trivially_copy_pass_by_ref)]
fn fuel(mass: &u32) -> u32 {
    mass / 3 - 2
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn total(mass: &u32) -> u32 {
    let f = fuel(mass);
    f + if f < 6 { 0 } else { total(&f) }
}

pub fn day01a(mass: &[u32]) -> u32 {
    mass.iter().map(fuel).sum()
}

pub fn day01b(mass: &[u32]) -> u32 {
    mass.iter().map(total).sum()
}

#[cfg(test)]
mod test {
    use crate::util;

    #[test] fn test_01_ex1() { assert_eq!(super::day01a(&[12]), 2); }
    #[test] fn test_01_ex2() { assert_eq!(super::day01a(&[14]), 2); }
    #[test] fn test_01_ex3() { assert_eq!(super::day01a(&[1969]), 654); }
    #[test] fn test_01_ex4() { assert_eq!(super::day01a(&[100756]), 33583); }

    #[test] fn test_01_ex5() { assert_eq!(super::day01b(&[12]), 2); }
    #[test] fn test_01_ex6() { assert_eq!(super::day01b(&[14]), 2); }
    #[test] fn test_01_ex7() { assert_eq!(super::day01b(&[1969]), 966); }
    #[test] fn test_01_ex8() { assert_eq!(super::day01b(&[100756]), 50346); }

    #[test]
    fn test_01() -> Result<(), Box<dyn std::error::Error>> {
        let masses: Vec<u32> = util::get_parsed_lines::<u32>("input/day01.txt")?;
        assert_eq!(super::day01a(&masses), 3374289);
        assert_eq!(super::day01b(&masses), 5058559);
        Ok(())
    }
}
