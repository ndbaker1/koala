export type Instruction = {
  opCode: number
  data: number
}

export const parseInstruction = (instruction: number): Instruction => {
  const instructionBuffer = new Uint8Array(new Uint32Array([instruction]).buffer)
  const [opCode, data1, data2, data3] = instructionBuffer
  return { opCode, data: data1 + data2 + data3 }
}