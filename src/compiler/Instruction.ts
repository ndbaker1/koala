export type Instruction = {
  opCode: number
  data: number
}

export const parseInstruction = (instruction: number): Instruction => {
  const instructionBuffer = new Uint8Array(new Uint32Array([instruction]).buffer)
  const [opCode, byte2, byte3, byte4] = instructionBuffer
  // console.log({ opCode, data: (byte2 << 16) + (byte3 << 8) + byte4 })
  return { opCode, data: (byte2 << 16) + (byte3 << 8) + byte4 }
}