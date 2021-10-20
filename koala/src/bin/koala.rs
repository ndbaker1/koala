use koala::{
    instructions::{END, PRINT},
    kvm::VirtualMachine,
};

fn main() {
    let code = parse_file();

    let mut vm = VirtualMachine::new(&|msg: &str| print!("{}", msg));
    vm.load_code(&code);
    vm.run();
}

fn parse_file() -> Vec<u32> {
    vec![PRINT as u32, END as u32]
}
