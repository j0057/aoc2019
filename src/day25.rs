use crate::intcode;

pub fn day25_main(vm: &intcode::VM) -> Result<(), Box<dyn std::error::Error>> {
    vm.clone().run_stdio();
    Ok(())
}
