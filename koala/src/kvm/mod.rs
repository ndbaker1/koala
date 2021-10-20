mod memory;
mod processor;

use memory::Memory;
use processor::Processor;
use std::env;

use crate::instructions::{self, read_opcode};

/// The Koala Language Virtual Machine
pub struct VirtualMachine<'a> {
    processor: Processor,
    memory: Memory,
    running: bool,
    outpipe: OutputCallback<'a>,
}

pub type OutputCallback<'a> = &'a dyn Fn(&str) -> ();

impl VirtualMachine<'_> {
    pub fn new(outpipe: OutputCallback) -> VirtualMachine {
        VirtualMachine {
            processor: Processor::new(),
            memory: Memory::new(),
            running: false,
            outpipe,
        }
    }

    pub fn load_code(&mut self, code: &[u32]) {
        let mut address = 0;
        for instruction in code {
            self.memory.write(address, *instruction as i32);
            address += 1;
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.fetch();
            self.execute();
        }
    }

    pub fn fetch(&mut self) {
        self.processor.ir = self.memory.read(self.processor.pc) as u32;
    }

    pub fn execute(&mut self) {
        debug(&format!("executing instruvtion: {}", self.processor.ir));

        match read_opcode(self.processor.ir) {
            instructions::END => {
                self.running = false;
            }
            instructions::RAND => {}
            instructions::ADD => {}
            instructions::SUB => {}
            instructions::JUMP => {}
            instructions::BEQ => {}
            instructions::BNE => {}
            instructions::CALL => {}
            instructions::RET => {}
            instructions::PUSH => {
                self.processor.sp += 1;
            }
            instructions::POP => {
                self.memory.read(self.processor.sp);
                self.processor.sp -= 1;
            }
            instructions::PRINT => {
                let a = "sfd";
                self.print(a);
            }
            _ => { /* no-op */ }
        };

        // move the PC along
        self.processor.pc += 1;
    }

    fn print(&mut self, message: &str) {
        (self.outpipe)(message);
    }
}

fn debug(msg: &str) {
    if let Ok(_) = env::var("DEBUG") {
        println!("{}", msg);
    }
}
