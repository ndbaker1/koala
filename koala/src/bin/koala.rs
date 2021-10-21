use std::{env::args, path::Path};

use byteorder::{BigEndian, ByteOrder};
use koala::kvm::VirtualMachine;

fn main() {
    let files: Vec<String> = args().collect();

    if files.len() > 1 {
        let code = read_file(&files[1]);

        let mut vm = VirtualMachine::new(&|msg: &str| print!("{}", msg));
        vm.load_code(&code);
        vm.run();
    }
}

fn read_file(file_path: &str) -> Vec<u32> {
    let file_buffer = match std::fs::read(Path::new(file_path)) {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let mut inst_buffer: Vec<u32> = vec![0; file_buffer.len() / 4];
    BigEndian::read_u32_into(&file_buffer, &mut inst_buffer);

    inst_buffer
}
