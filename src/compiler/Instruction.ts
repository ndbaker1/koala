export type Instruction = {
  opCode: number
  data: number
}

export const parseInstruction = (instruction: number, debug = false): Instruction => {
  const instructionBuffer = new Uint8Array(new Uint32Array([instruction]).buffer)
  const [opCode, byte2, byte3, byte4] = instructionBuffer
  const parsedInstruction: Instruction = { opCode, data: (byte2 << 16) + (byte3 << 8) + byte4 }

  if (debug) {
    console.error(parsedInstruction)
  }

  return parsedInstruction
}

// Instruction Definitions
export enum Arch {
  LOAD_VALUE = 1,
  LOAD_ADDR = 2,
  LOAD_IND_ADDR = 3,
  LOAD_IDX_X_ADDR = 4,
  LOAD_IDX_Y_ADDR = 5,
  LOAD_SP_X = 6,
  STORE_ADDR = 7,
  RAND = 8,
  PUT_PORT = 9,
  ADDX = 10,
  ADDY = 11,
  SUBX = 12,
  SUBY = 13,
  COPY_TO_X = 14,
  COPY_FROM_X = 15,
  COPY_TO_Y = 16,
  COPY_FROM_Y = 17,
  COPY_TO_SP = 18,
  COPY_FROM_SP = 19,
  JUMP_ADDR = 20,
  JUMP_IF_EQUAL_ADDR = 21,
  JUMP_IF_NOT_EQUAL_ADDR = 22,
  CALL_ADDR = 23,
  RET = 24,
  INCX = 25,
  DECX = 26,
  PUSH = 27,
  POP = 28,
  IRET = 30,
  END = 50,
}

