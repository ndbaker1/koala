use crate::instructions;
use std::panic;

/// The Koala Language Virtual Machine
pub struct VirtualMachine<'a> {
    /// Program Counter
    pc: usize,
    // Code Memory
    code: &'a [u32],
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
    output_pipe: OutputCallback<'a>,
    /// Callback for debugging output
    debug_pipe: OutputCallback<'a>,
}

/// Callback used to interact with the outside
pub type OutputCallback<'a> = &'a dyn Fn(&str) -> ();

#[derive(Debug)]
pub struct Frame {
    pub fn_addr: usize,
    pub locals: Vec<i32>,
    pub return_addr: usize,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(output_pipe: OutputCallback<'a>, debug_pipe: OutputCallback<'a>) -> Self {
        VirtualMachine {
            pc: 0,
            code: &[],
            call_stack: Vec::new(),
            stack: Vec::new(),
            globals: Vec::new(),
            running: false,
            output_pipe,
            debug_pipe,
        }
    }

    pub fn run(&mut self, code: &'a [u32]) {
        // Take a reference to the Binary Code
        self.code = code;
        // Set Running Flag
        self.running = true;
        // Continue executing until finished
        while self.running {
            self.execute();
        }
    }

    fn fetch(&mut self) -> u32 {
        // Increment PC
        self.pc += 1;
        // Fetch the Data at the PC we stepped over
        self.code[self.pc - 1]
    }

    fn execute(&mut self) {
        // Pull the opcode fetched prior
        let opcode = self.fetch();

        self.debug(&format!(
            "\nPC: {:<3} IR: {:<#6x} SP: {:<3} stack: {:?} frame: {:#?}\n",
            self.pc - 1,
            opcode,
            self.sp(),
            self.stack,
            self.call_stack,
        ));

        match opcode {
            instructions::END => {
                self.running = false;
            }
            instructions::PUSH => {
                // Fetch the value to Load onto the Stack
                let immediate_val = self.fetch() as i32;
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
                let jump_addr = self.fetch() as usize;
                // Move the Instruction Pointer to the Address
                self.pc = jump_addr;
            }
            instructions::BEQZ | instructions::BNEZ => {
                // Fetch the address to jump to when branching
                let branch_addr = self.fetch() as usize;
                // Get the value on the Stack and evaluate condition based on type (opcode)
                let val = self.stack.pop().unwrap() as u32;
                // conditionally branch when corresponding statement evaluates
                if match opcode {
                    instructions::BEQZ => val == 0,
                    instructions::BNEZ => val != 0,
                    _ => panic!("impossible path."),
                } {
                    self.pc = branch_addr;
                }
            }
            instructions::CALL => {
                // Fetch arg count from the stack
                let arg_count = self.fetch();
                // Fetch the address of the call
                let fn_addr = self.fetch() as usize;
                // Copy the args from the stack into Frame Locals
                let locals = (0..arg_count)
                    .into_iter()
                    .map(|_| self.stack.pop().unwrap())
                    .collect();
                // Capture the current PC
                let return_addr = self.pc;
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
                let print_type = self.fetch();
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
            instructions::LOCAL_LOAD => {
                // Fetch the Load offset
                let offset = self.fetch() as usize;

                self.debug(&format!("loading with offset: {}\n", offset));

                // Push a variable in the current Frame onto the Stack
                self.stack
                    .push(self.call_stack.last().unwrap().locals[offset]);
            }
            instructions::LOCAL_STORE => {
                // Fetch the Load offset
                let offset = self.fetch() as usize;

                self.debug(&format!("storing with offset: {}\n", offset));
                // Set a variable in the current Frame fromn the Stack
                self.call_stack.last_mut().unwrap().locals[offset] = self.stack.pop().unwrap();
            }
            instructions::GLOBAL_LOAD => {}
            instructions::GLOBAL_STORE => {}
            _ => { /* no-op */ }
        };
    }

    fn print(&mut self, message: &str) {
        (self.output_pipe)(message);
    }

    fn debug(&mut self, message: &str) {
        (self.debug_pipe)(message);
    }

    fn sp(&self) -> usize {
        self.stack.len()
    }
}
