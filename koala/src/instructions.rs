pub const END: u8 = 0x0; // end
pub const ADD: u8 = 0x1; // add
pub const SUB: u8 = 0x2; // sub
pub const MUL: u8 = 0x7; // store
pub const RAND: u8 = 0x8; // get random number
pub const JUMP: u8 = 0x14; // jump
pub const BEQ: u8 = 0x15; // branch equal
pub const BNE: u8 = 0x16; // branch not equal
pub const CALL: u8 = 0x17; // jump and link
pub const RET: u8 = 0x18; // jump return
pub const PRINT: u8 = 0x19; // print
pub const PUSH: u8 = 0x1B; // push stack
pub const POP: u8 = 0x1C; // pop stack

pub struct Inst {
    opcode: u8,
    data: i32,
}

impl Inst {
    pub fn from(instruction: u32) -> Inst {
        Inst {
            opcode: read_opcode(instruction),
            data: (instruction >> 8) as i32,
        }
    }

    pub fn new(opcode: u8, data: i32) -> Inst {
        Inst { opcode, data }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.data << 8) as u32) + (self.opcode as u32)
    }
}

pub fn read_opcode(instruction: u32) -> u8 {
    instruction as u8
}
