use std::collections::HashMap;

pub struct Memory {
    pub data_stack: Vec<i32>,
    pub call_stack: Vec<i32>,
    global_storage: HashMap<usize, i32>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data_stack: Vec::new(),
            call_stack: Vec::with_capacity(200),
            global_storage: HashMap::new(),
        }
    }

    pub fn read(&self, address: &usize) -> i32 {
        self.global_storage[address]
    }

    pub fn write(&mut self, address: &usize, value: i32) {
        self.global_storage.insert(*address, value);
    }
}
