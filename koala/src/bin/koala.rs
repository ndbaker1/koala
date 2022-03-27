use byteorder::{BigEndian, ByteOrder};
use koala::kvm::VirtualMachine;
use std::{env::args, path::Path};

fn main() {
    let files: Vec<String> = args().collect();
    if files.len() > 1 {
        let code = read_file(&files[1]);

        let debug_pipe = |msg: &str| {
            if files.iter().any(|arg| arg == "--debug") {
                print!("{}", msg);
            }
        };

        let mut vm = VirtualMachine::new(&|msg: &str| print!("{}", msg), &debug_pipe);
        vm.run(&code);
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
