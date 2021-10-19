mod vm;

use crate::vm::{
    instructions::{END, PRINT},
    vm::VirtualMachine,
};

fn main() {
    let out_stream = &|msg: &str| println!("{}", msg);
    let mut vm = VirtualMachine::new(out_stream);
    vm.load_code(&[PRINT, END]);
    vm.run();
}
