// End of Instruction and Shutdown VM
pub const END: u32 = 0x0;

/* Math */

// Integer
pub const IADD: u32 = 0x1; // add ints
pub const ISUB: u32 = 0x2; // sub ints
pub const IMUL: u32 = 0x3; // add ints
pub const IDIV: u32 = 0x4; // sub ints

// FLoat
pub const FADD: u32 = 0x5; // sub ints
pub const FSUB: u32 = 0x6; // sub ints
pub const FMUL: u32 = 0x7; // add ints
pub const FDIV: u32 = 0x8; // sub ints

// Variables On Call Stack
pub const LOCAL_LOAD: u32 = 0x10; // load LOCAL
pub const LOCAL_STORE: u32 = 0x11; // store LOCAL
pub const GLOBAL_LOAD: u32 = 0x12; // load GLOBAL
pub const GLOBAL_STORE: u32 = 0x13; // store GLOBAL

// RNG
pub const RAND: u32 = 0x20; // get random number

// Control Flow
pub const JUMP: u32 = 0x30; // jump
pub const BEQZ: u32 = 0x31; // branch equal zero
pub const BNEZ: u32 = 0x32; // branch not equal zero

// Functions
pub const CALL: u32 = 0x40; // jump and link
pub const RET: u32 = 0x41; // jump return

// I/O
pub const PRINT: u32 = 0x50; // print

// Stack Ops
pub const PUSH: u32 = 0x60; // load immediate
pub const POP: u32 = 0x61; // pop stack
