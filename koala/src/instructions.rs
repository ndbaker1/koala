pub const END: u32 = 0x0; // end
pub const IADD: u32 = 0x1; // add ints
pub const ISUB: u32 = 0x2; // sub ints
pub const IMUL: u32 = 0x3; // add ints
pub const IDIV: u32 = 0x4; // sub ints
pub const FADD: u32 = 0x5; // sub ints
pub const FSUB: u32 = 0x6; // sub ints
pub const FMUL: u32 = 0x7; // add ints
pub const FDIV: u32 = 0x8; // sub ints
pub const CONST: u32 = 0x9; // load
pub const LOAD: u32 = 0xA; // load
pub const STORE: u32 = 0xB; // store
pub const RAND: u32 = 0xC; // get random number
pub const JUMP: u32 = 0xD; // jump
pub const BEQ: u32 = 0xE; // branch equal
pub const BNE: u32 = 0xF; // branch not equal
pub const CALL: u32 = 0x10; // jump and link
pub const RET: u32 = 0x18; // jump return
pub const PRINT: u32 = 0x19; // print
pub const PUSH: u32 = 0x1B; // push stack
pub const POP: u32 = 0x1C; // pop stack