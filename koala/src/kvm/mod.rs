mod memory;
mod processor;

use memory::Memory;
use processor::Processor;
use std::env;

use crate::instructions::{self};

/// The Koala Language Virtual Machine
pub struct VirtualMachine<'a> {
    processor: Processor,
    memory: Memory,
    /// Running flag
    running: bool,
    /// Callback for Interaction with the outside world
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
            self.memory.write(&address, *instruction as i32);
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
        self.processor.ip = self.memory.read(&self.processor.pc) as u32;
        self.processor.pc += 1;
    }

    pub fn execute(&mut self) {
        debug(&format!("executing instruvtion: {}", self.processor.ip));

        match self.processor.ip {
            instructions::END => {
                self.running = false;
            }
            instructions::CONST => {
                self.fetch();
                self.memory.stack.push(self.processor.ip as i32);
            }
            instructions::IADD | instructions::IMUL | instructions::ISUB | instructions::IDIV => {
                let first = match self.memory.stack.pop() {
                    Some(val) => val,
                    None => return,
                };

                let second = match self.memory.stack.pop() {
                    Some(val) => val,
                    None => return,
                };

                let result = match self.processor.ip {
                    instructions::IADD => first + second,
                    instructions::IMUL => first * second,
                    instructions::ISUB => first - second,
                    instructions::IDIV => first / second,
                    _ => return,
                };

                self.memory.stack.push(result);
            }
            instructions::FADD | instructions::FMUL | instructions::FSUB | instructions::FDIV => {
                let first: f32 = match self.memory.stack.pop() {
                    Some(val) => val as f32,
                    None => return,
                };

                let second: f32 = match self.memory.stack.pop() {
                    Some(val) => val as f32,
                    None => return,
                };

                let result = match self.processor.ip {
                    instructions::IADD => first + second,
                    instructions::IMUL => first * second,
                    instructions::ISUB => first - second,
                    instructions::IDIV => first / second,
                    _ => return,
                };

                self.memory.stack.push(result as i32);
            }
            instructions::JUMP => {
                self.fetch();
                self.processor.pc = self.processor.ip as usize;
            }
            instructions::BEQ => {}
            instructions::BNE => {}
            instructions::CALL => {}
            instructions::RET => {}
            instructions::PUSH => {
                self.processor.sp += 1;
            }
            instructions::POP => {
                self.memory.stack.pop();
                self.processor.sp -= 1;
            }
            instructions::PRINT => {
                self.fetch();
                match self.processor.ip {
                    1 => self.print(&self.memory.stack[0].to_string()),
                    2 => match char::from_u32(self.memory.stack[0] as u32) {
                        Some(letter) => self.print(&letter.to_string()),
                        None => {}
                    },
                    _ => {}
                }
            }
            _ => { /* no-op */ }
        };
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
