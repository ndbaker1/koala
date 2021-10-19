const NUM_REGISTERS: usize = 12;

pub struct Processor {
    pub regs: [i32; NUM_REGISTERS],
    pub sp: usize,
    pub ir: u32,
    pub pc: usize,
    pub ret: u32,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            ir: 0,
            sp: 0,
            pc: 0,
            ret: 0,
            regs: [0; NUM_REGISTERS],
        }
    }
}
