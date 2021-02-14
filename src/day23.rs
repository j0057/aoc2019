use std::cell::RefCell;
use std::collections::VecDeque;

use crate::intcode;

//
// struct Machine
//

struct Machine {
    vm: intcode::VM,
    i: Vec<i128>,
    o: Vec<i128>,
}

impl Machine {
    fn new(vm: &intcode::VM, addr: usize) -> Self {
        Self {
            vm: vm.clone(),
            i: vec![addr as i128],
            o: vec![]
        }
    }

    fn step(&mut self) -> intcode::Status {
        self.vm.step(&mut self.i, &mut self.o)
    }
}

//
// struct CategorySix
//

struct CategorySix {
    machines: Vec<RefCell<Machine>>,
    runnable: RefCell<VecDeque<usize>>,
}

impl CategorySix {
    fn new(vm: &intcode::VM, count: usize) -> Self {
        Self {
            machines: (0..count)
                .map(|i| RefCell::new(Machine::new(&vm, i)))
                .collect(),
            runnable: RefCell::new((0..count).collect()),
        }
    }

    fn send(&self, n: i128, x: i128, y: i128) {
        self.machines[n as usize].borrow_mut().i.extend(&[x, y]);
        self.runnable.borrow_mut().push_back(n as usize);
    }
}

//
// struct CategorySixIterator
//

struct CategorySixIterator<'a> {
    network: &'a CategorySix,
}

impl<'a> CategorySixIterator<'a> {
    fn new(network: &'a CategorySix) -> Self {
        Self {
            network,
        }
    }
}

impl Iterator for CategorySixIterator<'_> {
    type Item = Option<(i128, i128, i128)>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(i) = self.network.runnable.borrow_mut().pop_front() {
            if self.network.machines[i].borrow().i.is_empty() {
                self.network.machines[i].borrow_mut().i.push(-1);
            }
            let status = self.network.machines[i].borrow_mut().step();
            match status {
                intcode::Status::Halted => {
                    unreachable!();
                },
                intcode::Status::Blocked => {
                    continue;
                },
                intcode::Status::Suspended if self.network.machines[i].borrow().o.len() >= 3 => {
                    let a = &mut self.network.machines[i].borrow_mut().o;
                    return Some(Some((a.remove(0), a.remove(0), a.remove(0))));
                },
                intcode::Status::Suspended => {
                    continue;
                },
            }
        }
        Some(None)
    }
}

//
// solution
//

pub fn day23a(vm: &intcode::VM) -> i128 {
    let network = CategorySix::new(vm, 50);
    for msg in CategorySixIterator::new(&network) {
        match msg {
            Some((i, _, y)) if i == 255 => return y,
            Some((i, x, y)) => network.send(i, x, y),
            None => (0..50).for_each(|i| network.runnable.borrow_mut().push_back(i)),
        }
    }
    unreachable!()
}

//
// tests
//

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    fn test_23a() -> Result<(), Box<dyn std::error::Error>> {
        let vm = util::get_parsed_line("input/day23.txt")?;
        let y = super::day23a(&vm);
        assert_eq!(y, 15416);
        Ok(())
    }
}
