use std::collections::HashMap;

pub struct Memory {
    pub stack: Vec<i32>,
    pub ram: HashMap<usize, i32>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            stack: Vec::new(),
            ram: HashMap::new(),
        }
    }

    pub fn read(&self, address: &usize) -> i32 {
        self.ram[address]
    }

    pub fn write(&mut self, address: &usize, value: i32) {
        self.ram.insert(*address, value);
    }
}
