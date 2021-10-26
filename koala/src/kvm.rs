use crate::instructions::{self};
use std::{
    env::{self},
    panic,
};

const DEBUG: bool = false;

/// The Koala Language Virtual Machine
pub struct VirtualMachine<'a> {
    /// Program Counter
    pc: usize,
    /// Instruction Register
    ir: u32,
    // Code Memory
    code: Vec<u32>,
    /// Frames indicate the start of a function call,
    /// which will automatically take care of the need to track Frame Pointers
    call_stack: Vec<Frame>,
    // Executtion/Data Stack
    stack: Vec<i32>,
    /// Globals
    globals: Vec<i32>,
    /// Running flag
    running: bool,
    /// Callback for Interaction with the outside world
    outpipe: OutputCallback<'a>,
}

/// Callback used to interact with the outside
pub type OutputCallback<'a> = &'a dyn Fn(&str) -> ();

pub struct Frame {
    pub fn_addr: usize,
    pub locals: Vec<i32>,
    pub return_addr: usize,
}

impl VirtualMachine<'_> {
    pub fn new(outpipe: OutputCallback) -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            ir: 0,
            code: Vec::new(),
            call_stack: Vec::new(),
            stack: Vec::new(),
            globals: Vec::new(),
            running: false,
            outpipe,
        }
    }

    pub fn load_code(&mut self, code: &[u32]) {
        for instruction in code {
            self.code.push(*instruction);
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            self.fetch();
            self.execute();
        }
    }

    fn fetch(&mut self) {
        self.ir = self.code[self.pc];
        self.pc += 1;
    }

    fn execute(&mut self) {
        if DEBUG {
            self.print(&format!(
                "\nPC: {:<3} IP: {:<#6x} SP: {:<3} stack: {:?}\n",
                self.pc - 1,
                self.ir,
                self.sp(),
                self.stack
            ));
        }

        let opcode = self.ir;

        match opcode {
            instructions::END => {
                self.running = false;
            }
            instructions::PUSH => {
                // Fetch the value to Load onto the Stack
                self.fetch();
                let immediate_val = self.ir as i32;
                // Push the immediate Value
                self.stack.push(immediate_val);
            }
            instructions::POP => {
                self.stack.pop();
            }
            instructions::IADD | instructions::IMUL | instructions::ISUB | instructions::IDIV => {
                let first = self.stack.pop().unwrap();
                let second = self.stack.pop().unwrap();

                let result = match opcode {
                    instructions::IADD => first + second,
                    instructions::IMUL => first * second,
                    instructions::ISUB => first - second,
                    instructions::IDIV => first / second,
                    _ => return,
                };

                self.stack.push(result);
            }
            instructions::FADD | instructions::FMUL | instructions::FSUB | instructions::FDIV => {
                let first = self.stack.pop().unwrap();
                let second = self.stack.pop().unwrap();

                let result = match opcode {
                    instructions::FADD => first + second,
                    instructions::FMUL => first * second,
                    instructions::FSUB => first - second,
                    instructions::FDIV => first / second,
                    _ => return,
                };

                self.stack.push(result);
            }
            instructions::JUMP => {
                // Fetch the address to Jump to
                self.fetch();
                let jump_addr = self.ir as usize;
                // Move the Instruction Pointer to the Address
                self.pc = jump_addr;
            }
            instructions::BEQZ | instructions::BNEZ => {
                // Fetch the address to jump to when branching
                self.fetch();
                let branch_addr = self.ir as usize;
                // Get the value on the Stack and evaluate condition based on type (opcode)
                let val = self.stack.pop().unwrap() as u32;
                let cond = match opcode {
                    instructions::BEQZ => val == 0,
                    instructions::BNEZ => val != 0,
                    _ => panic!("impossible path."),
                };
                // conditionally branch
                if cond {
                    self.pc = branch_addr;
                }
            }
            instructions::CALL => {
                // Capture the current PC
                let return_addr = self.pc;
                // Fetch arg count from the stack
                self.fetch();
                let arg_count = self.ir;
                // Fetch the address of the call
                self.fetch();
                let fn_addr = self.ir as usize;
                // Copy the args from the stack into Frame Locals
                let locals = (0..arg_count)
                    .into_iter()
                    .map(|_| self.stack.pop().unwrap())
                    .collect();
                // Push a new Stack Frame
                self.call_stack.push(Frame {
                    fn_addr,
                    locals,
                    return_addr,
                });
                // Move the PC to the function address
                self.pc = fn_addr;
            }
            instructions::RET => {
                // Pop the state of the Call Stack when exiting function
                let exiting_frame = self.call_stack.pop().unwrap();
                // Move the Program Counter back to the previous address
                self.pc = exiting_frame.return_addr;
                // Pop any return address
            }
            instructions::PRINT => {
                // Read the print Type (encoded/value)
                self.fetch();
                let print_type = self.ir;
                // Get Value to Print
                let val = self.stack.pop().unwrap();
                // Print based on Type
                let msg = match print_type {
                    1 => val.to_string(),
                    _ => char::from_u32(val as u32)
                        .unwrap_or_else(|| panic!("bad character parsing in print!"))
                        .to_string(),
                };
                // Use outisde callback to pipe output
                self.print(&msg);
            }
            instructions::LOAD => {
                self.fetch();
                let offset = self.ir as usize;

                if DEBUG {
                    self.print(&format!("loading with offset: {}\n", offset));
                }

                self.stack
                    .push(self.call_stack.last().unwrap().locals[offset]);
            }
            instructions::STORE => {
                self.fetch();
                let offset = self.ir as usize;

                if DEBUG {
                    self.print(&format!("storing with offset: {}\n", offset));
                }

                self.call_stack.last_mut().unwrap().locals[offset] = match self.stack.pop() {
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
        self.stack.len()
    }
}
