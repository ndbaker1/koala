const mem_length: usize = 2048;
const STACK_SIZE: usize = 2048;

pub struct Memory {
    mem: [i32; mem_length],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; mem_length],
        }
    }

    pub fn read(&self, address: usize) -> i32 {
        self.mem[address]
    }

    pub fn write(&mut self, address: usize, value: i32) {
        self.mem[address] = value;
    }
}
