const NUM_REGISTERS: usize = 12;

pub struct Processor {
    pub pc: usize,
    pub ir: u32,
    pub sp: usize,
    pub ret: u32,
    pub regs: [i32; NUM_REGISTERS],
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0,
            ir: 0,
            sp: 0,
            ret: 0,
            regs: [0; NUM_REGISTERS],
        }
    }
}
