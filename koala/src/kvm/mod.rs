mod memory;
mod processor;

use memory::Memory;
use processor::Processor;
use std::{
    env::{self},
    panic,
};

use crate::instructions::{self};

const DEBUG: bool = false;

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
        for (index, instruction) in code.iter().enumerate() {
            self.memory.write(&index, *instruction as i32);
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
        if DEBUG {
            self.print(&format!(
                "\nPC: {:<3} IP: {:<#6x} FP: {:<3} SP: {:<3} stack: {:?}\n",
                self.processor.pc - 1,
                self.processor.ip,
                self.processor.fp,
                self.sp(),
                self.memory.data_stack
            ));
        }

        match self.processor.ip {
            instructions::END => {
                self.running = false;
            }
            instructions::CONST => {
                // Fetch the value to Load onto the Stack
                self.fetch();
                // Push the immediate Value
                self.memory.data_stack.push(self.processor.ip as i32);
            }
            instructions::IADD | instructions::IMUL | instructions::ISUB | instructions::IDIV => {
                let first = match self.memory.data_stack.pop() {
                    Some(val) => val,
                    None => return,
                };

                let second = match self.memory.data_stack.pop() {
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

                self.memory.data_stack.push(result);
            }
            instructions::FADD | instructions::FMUL | instructions::FSUB | instructions::FDIV => {
                let first: f32 = match self.memory.data_stack.pop() {
                    Some(val) => val as f32,
                    None => return,
                };

                let second: f32 = match self.memory.data_stack.pop() {
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

                self.memory.data_stack.push(result as i32);
            }
            instructions::JUMP => {
                // Fetch the address to Jump to
                self.fetch();
                // Move the Instruction Pointer to the Address
                self.processor.pc = self.processor.ip as usize;
            }
            instructions::BEQZ | instructions::BNEZ => match self.memory.data_stack.len() >= 1 {
                true => {
                    // cache instruction pointer for after the address fetch
                    let opcode = self.processor.ip;
                    // Fetch the address to jump to when value not equal
                    self.fetch();
                    // Compare the top of the stack to 0
                    let val = self.memory.data_stack.pop().unwrap() as u32;
                    let cond = match opcode {
                        instructions::BEQZ => val == 0,
                        instructions::BNEZ => val != 0,
                        _ => panic!("impossible path."),
                    };

                    if cond {
                        self.processor.pc = self.processor.ip as usize;
                    }
                }
                false => panic!("not enough arguments on stack to do BEQZ."),
            },
            instructions::CALL => {
                // Fetch the address of the call, then
                self.fetch();
                // Push the return address onto the Call Stack
                self.memory.call_stack.push(self.processor.pc as i32);
                // Push the Frame Pointer address onto the Call Stack
                self.memory.call_stack.push(self.processor.fp as i32);
                // Read the number of args
                let arg_count = self.memory.data_stack.pop().unwrap() as usize;
                // update the Frame pointer
                self.processor.fp = self.sp() - arg_count;
                // Move the Instruction Pointer to the address of the Function
                self.processor.pc = self.processor.ip as usize;
            }
            instructions::RET => {
                // Move the Frame Pointer back to the previous call's state
                self.processor.fp = self.memory.call_stack.pop().unwrap() as usize;
                // Move the Program Counter back to the previous address
                // by popping from the Call Stack
                self.processor.pc = self.memory.call_stack.pop().unwrap() as usize;
            }
            instructions::PUSH => {
                self.fetch();
                self.memory.data_stack.push(self.processor.ip as i32);
            }
            instructions::POP => {
                self.memory.data_stack.pop();
            }
            instructions::DUP => {
                let val = self.memory.data_stack.pop().unwrap();
                self.memory.data_stack.push(val);
                self.memory.data_stack.push(val);
            }
            instructions::PRINT => {
                self.fetch();
                let val = self.memory.data_stack.pop().unwrap();
                let msg = match self.processor.ip {
                    1 => val.to_string(),
                    _ => char::from_u32(val as u32)
                        .unwrap_or_else(|| panic!("bad character parsing in print!"))
                        .to_string(),
                };
                self.print(&msg);
            }
            instructions::LOAD => {
                self.fetch();

                if DEBUG {
                    self.print(&format!(
                        "loading with FP: {} and offset: {}\n",
                        self.processor.fp, self.processor.ip
                    ));
                }

                self.memory
                    .data_stack
                    .push(self.memory.data_stack[self.processor.fp + self.processor.ip as usize]);
            }
            instructions::STORE => {
                self.fetch();

                if DEBUG {
                    self.print(&format!(
                        "storing with FP: {} and offset: {}\n",
                        self.processor.fp, self.processor.ip
                    ));
                }

                self.memory.data_stack[self.processor.fp + self.processor.ip as usize] =
                    match self.memory.data_stack.pop() {
                        Some(val) => val,
                        None => panic!("no value on stack"),
                    };
            }
            _ => { /* no-op */ }
        };
    }

    fn print(&mut self, message: &str) {
        (self.outpipe)(message);
    }

    fn sp(&self) -> usize {
        self.memory.data_stack.len()
    }
}

fn is_debug() -> bool {
    match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false,
    }
}
