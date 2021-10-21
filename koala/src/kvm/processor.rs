const NUM_REGISTERS: usize = 12;

/// Container for Processor Fields
pub struct Processor {
    /// Program Counter
    pub pc: usize,
    /// Instruction Pointer
    pub ip: u32,
    /// Stack Pointer
    pub sp: usize,
    /// Return Address
    pub ret: u32,
    /// Register
    pub regs: [i32; NUM_REGISTERS],
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0,
            ip: 0,
            sp: 0,
            ret: 0,
            regs: [0; NUM_REGISTERS],
        }
    }
}
