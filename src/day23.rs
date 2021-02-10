use std::sync::mpsc;
use std::thread;

use crate::intcode;

#[derive(Debug, Clone, Copy)]
enum Packet {
    Msg(i128, i128),
    Stop,
}

fn spawn_vm_thread(mut vm: intcode::VM, addr: i128, rx: mpsc::Receiver<Packet>, tx: Vec<mpsc::Sender<Packet>>, result: mpsc::Sender<i128>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut input = vec![addr];
        let mut output = vec![];
        loop {
            if input.is_empty() {
                match rx.try_recv() {
                    Ok(Packet::Msg(x, y)) => input.extend(&[x, y]),
                    Ok(Packet::Stop) => break,
                    Err(_) => input.push(-1),
                }
            }
            match vm.step(&mut input, &mut output) {
                intcode::Status::Suspended if output.len() >= 3 => {
                    let a = output.remove(0) as usize;
                    let x = output.remove(0);
                    let y = output.remove(0);
                    match tx.get(a) {
                        Some(tx) => tx.send(Packet::Msg(x, y)).unwrap(),
                        None     => result.send(y).unwrap(),
                    }
                },
                intcode::Status::Suspended => continue,
                intcode::Status::Halted => break,
                intcode::Status::Blocked => continue,
            }
        }
    })
}

pub fn day23a(vm: &intcode::VM) -> i128 {
    // channels for communication between VMs: each VM gets all the tx channels and its own rx channel
    let (vm_tx, vm_rx) = std::iter::from_fn(|| Some(mpsc::channel::<Packet>()))
        .take(50)
        .unzip::<_, _, Vec<_>, Vec<_>>();

    // channel for announcing the first result
    let (res_tx, res_rx) = mpsc::channel::<i128>();

    // kick off 50 threads
    let handles = vm_rx
        .into_iter()
        .enumerate()
        .map(|(addr, rx)| spawn_vm_thread(vm.clone(), addr as i128, rx, vm_tx.clone(), res_tx.clone()))
        .collect::<Vec<_>>();

    // wait for result
    let result = res_rx.recv().unwrap();

    // send stop message to all the VMs
    vm_tx.iter().for_each(|tx| tx.send(Packet::Stop).unwrap());

    // wait for VM threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    result
}

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
