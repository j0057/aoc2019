use std::collections::HashSet;

use std::io::Write;

use num_complex::Complex;

use crate::intcode;

type Coord = Complex<i32>;

static UP: Coord = Coord { re: 0, im: -1 };
static DN: Coord = Coord { re: 0, im: 1 };
static LT: Coord = Coord { re: -1, im: 0 };
static RT: Coord = Coord { re: 1, im: 0 };

fn get_scaffold(data: &[i128]) -> HashSet<Coord> {
    data.split(|&b| b == 10)
        .enumerate()
        .flat_map(|(y, row)| row
              .iter()
              .enumerate()
              .filter(|&(_, ch)| *ch != 46)
              .map(move |(x, _)| Coord::new(x as i32, y as i32)))
        .collect()
}

fn get_alignment_params(scaffold: &HashSet<Coord>) -> i32 {
    scaffold
        .iter()
        .filter(|&a| [UP, DN, LT, RT].iter().all(|b| scaffold.contains(&(a + b))))
        .map(|a| a.re * a.im)
        .sum()
}

pub fn day17a(vm: &intcode::VM) -> i32 {
    let output = vm.clone().run(&mut vec![]);
    let scaffold = get_scaffold(&output);
    get_alignment_params(&scaffold)
}

pub fn day17_main(vm: &intcode::VM) -> Result<(), Box<dyn std::error::Error>> {
    let output = vm.clone().run(&mut vec![]);
    let bytes = output.iter().map(|&w| w as u8).collect::<Vec<u8>>();
    let mut stdout = std::io::stdout();
    stdout.write_all(&bytes)?;
    let answer = get_alignment_params(&get_scaffold(&output));
    writeln!(stdout, "*** ALIGNMENT PARAMETER : {:?} ***", answer)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    fn test_17() -> Result<(), Box<dyn std::error::Error>> {
        let vm = util::get_parsed_line("input/day17.txt")?;
        assert_eq!(super::day17a(&vm), 3292);
        Ok(())
    }
}
