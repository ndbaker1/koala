export type Memory = {
  read: (address: number) => number
  write: (address: number, data: number) => void
  LENGTH: number
}

export function createMemory(): Memory {

  const LENGTH = 2000
  const MEMORY = new Int32Array(LENGTH)

  MEMORY.set(programA)

  // memory read wrapper
  const read = (address: number) => MEMORY[address]
  // memory write wrapper
  const write = (address: number, data: number) => { MEMORY[address] = data }

  return { read, write, LENGTH }
}


const programC = [
  (1 << 24) + 0,    // Load 0
  (14 << 24),   // CopyToX
  (4 << 24) + (32 << 16),  // LoadIdxX 32 (load from A-Z table) 
  (21 << 24) + (12 << 16),  // JumpIfEqual 12 
  (9 << 24) + (2 << 16),  // Put 2 (output as char)
  25,   // IncX
  20,  // Jump 3
  3,
  1,   // Load 0
  0,
  16,   // CopyToY
  5,  // LoadIdxY 59 (load from 1-10 table)
  59,
  21,  // JumpIfEqual 27
  27,
  9,  // Put 1 (output as int)
  1,
  1,   // Load 1  (because no IncY instruction)
  1,
  11,   // AddY
  16,  // CopyToY
  20, // Jump 15
  15,
  1,  // Print newline
  10,
  9,
  2,
  50,   // End
  65,  // Data A-Z
  66,
  67,
  68,
  69,
  70,
  71,
  72,
  73,
  74,
  75,
  76,
  77,
  78,
  79,
  80,
  81,
  82,
  83,
  84,
  85,
  86,
  87,
  88,
  89,
  90,
  0,
  1,   // Data 1-10
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  0,
]

const programA = [
  1,    // Load 0
  0,
  14,   // CopyToX
  4,  // LoadIdxX 32 (load from A-Z table) 
  32,
  21,  // JumpIfEqual 12 
  12,
  9,  // Put 2 (output as char)
  2,
  25,   // IncX
  20,  // Jump 3
  3,
  1,   // Load 0
  0,
  16,   // CopyToY
  5,  // LoadIdxY 59 (load from 1-10 table)
  59,
  21,  // JumpIfEqual 27
  27,
  9,  // Put 1 (output as int)
  1,
  1,   // Load 1  (because no IncY instruction)
  1,
  11,   // AddY
  16,  // CopyToY
  20, // Jump 15
  15,
  1,  // Print newline
  10,
  9,
  2,
  50,   // End
  65,  // Data A-Z
  66,
  67,
  68,
  69,
  70,
  71,
  72,
  73,
  74,
  75,
  76,
  77,
  78,
  79,
  80,
  81,
  82,
  83,
  84,
  85,
  86,
  87,
  88,
  89,
  90,
  0,
  1,   // Data 1-10
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
  10,
  0,
]

const programB = [
  23,   // line one
  15,
  23,  // line two 
  30,
  23,  // line three 
  51,
  23,  // line four 
  86,
  23,  // line five 
  103,
  23, // line six ,
  142,
  23, // line seven ,
  163,
  50,
  // line one,
  1,
  4,
  27,
  23,
  206,
  28,
  1,
  6,
  27,
  23,
  178,
  28,
  23,
  220,
  24,

  // line two
  1,
  32,
  9,
  2,
  1,
  47,
  9,
  2,
  1,
  9,
  27,
  23,
  206,
  28,
  1,
  92,
  9,
  2,
  23,
  220,
  24,

  // line three
  1,
  47,
  9,
  2,
  1,
  32,
  9,
  2,
  9,
  2,
  9,
  2,
  23,
  225,
  1,
  32,
  9,
  2,
  9,
  2,
  23,
  225,
  1,
  32,
  9,
  2,
  9,
  2,
  1,
  92,
  9,
  2,
  23,
  220,
  24,

  // line four 
  1,
  124,
  9,
  2,
  1,
  11,
  27,
  23,
  206,
  28,
  1,
  124,
  9,
  2,
  23,
  220,
  24,
  // line five
  1,
  92,
  9,
  2,
  1,
  32,
  9,
  2,
  9,
  2,
  9,
  2,
  1,
  92,
  9,
  2,
  1,
  4,
  27,
  23,
  192,
  28,
  1,
  47,
  9,
  2,
  1,
  32,
  9,
  2,
  9,
  2,
  1,
  47,
  9,
  2,
  23,
  220,
  24,

  // line six 
  1,
  32,
  9,
  2,
  1,
  92,
  9,
  2,
  1,
  9,
  27,
  23,
  206,
  28,
  1,
  47,
  9,
  2,
  23,
  220,
  24,


  // line seven 
  1,
  4,
  27,
  23,
  206,
  28,
  1,
  6,
  27,
  23,
  178,
  28,
  23,
  220,
  24,

  // print dash 
  1,
  1,
  14,
  6,
  14,
  1,
  45,
  9,
  2,
  26,
  15,
  22,
  183,
  24,

  // print underscore 
  1,
  1,
  14,
  6,
  14,
  1,
  95,
  9,
  2,
  26,
  15,
  22,
  197,
  24,

  // print space 
  1,
  1,
  14,
  6,
  14,
  1,
  32,
  9,
  2,
  26,
  15,
  22,
  211,
  24,

  // print newline 
  1,
  10,
  9,
  2,
  24,

  // print -* 
  1,
  45,
  9,
  2,
  1,
  42,
  9,
  2,
  24,
]