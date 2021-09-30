export type Memory = {
  read: (address: number) => number
  write: (address: number, data: number) => void
  load: (arr: Uint32Array) => void
  LENGTH: number
}

/**
 * Create a new Area of memory with a uint32 array, which can read/write data and load programs
 * @returns a Memory Type object with read/write and load capabilities
 */
export function createMemory(length = 2000): Memory {

  const LENGTH = length
  const MEMORY = new Uint32Array(LENGTH)

  // memory read wrapper
  const read = (address: number) => MEMORY[address]
  // memory write wrapper
  const write = (address: number, data: number) => { MEMORY[address] = data }

  const load = (arr: Uint32Array) => MEMORY.set(arr)

  return { read, write, load, LENGTH }
}
