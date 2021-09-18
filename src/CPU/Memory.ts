export type Memory = {
  read: (address: number) => number
  write: (address: number, data: number) => void
  LENGTH: number
}

export function createMemory(): Memory {

  const LENGTH = 2000
  const MEMORY = new Int32Array(LENGTH)

  MEMORY.set(programC)

  // memory read wrapper
  const read = (address: number) => MEMORY[address]
  // memory write wrapper
  const write = (address: number, data: number) => { MEMORY[address] = data }

  return { read, write, LENGTH }
}


const programC = (() => {
  return [
    1 + 0,    // Load 0
    14,   // CopyToX
    4 + (19 << 24),  // LoadIdxX 32 (load from A-Z table) 
    21 + (8 << 24),  // JumpIfEqual 8
    9 + (2 << 24),  // Put 2 (output as char)
    25,   // IncX
    20 + (2 << 24),  // Jump 2
    1 + 0,   // Load 0
    16,   // CopyToY
    5 + (46 << 24),  // LoadIdxY 59 (load from 1-10 table)
    21 + (16 << 24),  // JumpIfEqual 16
    9 + (1 << 24),  // Put 1 (output as int)
    1 + (1 << 24),   // Load 1  (because no IncY instruction)
    11,   // AddY
    16,  // CopyToY
    20 + (8 << 24), // Jump 8
    1 + (10 << 24),  // Load newline char
    9 + (2 << 24), // Print newline
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
})()

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