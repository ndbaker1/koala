/// Container for Processor Fields
pub struct Processor {
    /// Program Counter
    pub pc: usize,
    /// Instruction Pointer
    pub ip: u32,
    /// Frame Pointer
    pub fp: usize,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0,
            ip: 0,
            fp: 0,
        }
    }
}
