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
