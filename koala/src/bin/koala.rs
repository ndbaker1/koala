use std::env::args;

use koala::{
    instructions::{CONST, END, PRINT},
    kvm::VirtualMachine,
};

fn main() {
    let files: Vec<String> = args().collect();

    if files.len() > 1 {
        let code = read_file(&files[1]);

        let mut vm = VirtualMachine::new(&|msg: &str| print!("{}", msg));
        vm.load_code(&code);
        vm.run();
    }
}

fn read_file(file: &str) -> Vec<u32> {
    vec![CONST as u32, 1, PRINT as u32, 1, END as u32]
}
