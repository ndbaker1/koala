import { CPU, CPUConfig } from "../CPU"


/**
 * 
 * @returns test CPU architecture
 */
export function createTestCPU({ memory, debug }: CPUConfig): CPU {
  /**
   * Stack accumulator with Registers architecture 
   */

  let AC = 0 // Accumulator
  let SP = memory.LENGTH // Stack begins at the end of Memory and grows towards 0

  let PC = 0 // Program Counter
  let IR = 0 // Instruction Register

  let X = 0 // arithmetic register 1
  let Y = 0 // arithmetic register 2

  let exit = false

  let output = ''
  /**
   * Run the program with Fetch and Execute Cycle
   */
  const run = () => {
    exit = false
    // while the fetch does not fail, run the returned operation
    while (!exit && (IR = fetch()) != -1)
      execute()

    return output
  }

  /**
   * Loads from the PC and increments the PC
   * 
   * @return Data at the PC
   */
  const fetch = () => {
    const data = load(PC)
    PC++
    return data
  }

  /**
   * Load from memory address
   * 
   * @param address
   * @return data at memory address
   */
  const load = (address: number) => memory.read(address)

  /**
   * save data to memory address
   * 
   * @param address
   * @param data
   */
  const store = (address: number, data: number) => memory.write(address, data)

  /**
   * execute instruction with fetch cycle
   */
  const execute = () => {

    if (debug) {
      console.log(`AC=${AC}, X=${X}, Y=${Y}, SP=${SP}, PC=${PC}, IR=${IR}`)
    }

    const { opCode, data } = parseInstruction(IR, debug)

    let address = 0

    switch (opCode) {
      case Test.LOAD_VALUE: // load given value into AC
        AC = data
        break
      case Test.LOAD_ADDR: // load value from given address into AC
        address = data
        AC = load(address)
        break
      case Test.LOAD_IND_ADDR: // load value in address given by value at given address into AC
        address = data
        address = load(address)
        AC = load(address)
        break
      case Test.LOAD_IDX_X_ADDR: // load value at address of given address + X into AC
        address = data
        AC = load(address + X)
        break
      case Test.LOAD_IDX_Y_ADDR: // load value at address of given address + Y into AC
        address = data
        AC = load(address + Y)
        break
      case Test.LOAD_SP_X: // load value at address of SP + X into AC
        AC = load(SP + X)
        break
      case Test.STORE_ADDR: // place the value in AC into the address given
        address = data
        store(address, AC)
        break
      case Test.RAND: // get a random number from 0 to 100
        AC = Math.round(Math.random() * 100)
        break
      case Test.PUT_PORT: // print character representation or integer based on port flag
        const port = data
        switch (port) {
          case 1:
            output += AC.toFixed(0)
            break
          case 2:
            output += String.fromCharCode(AC)
            break
        }
        break
      case Test.ADDX: // add X to AC
        AC += X
        break
      case Test.ADDY: // add Y to AC
        AC += Y
        break
      case Test.SUBX: // subtract X from AC
        AC -= X
        break
      case Test.SUBY: // subtract Y from AC
        AC -= Y
        break
      case Test.COPY_TO_X: // copy AC to X
        X = AC
        break
      case Test.COPY_FROM_X: // copy X to AC
        AC = X
        break
      case Test.COPY_TO_Y: // copy AC to Y
        Y = AC
        break
      case Test.COPY_FROM_Y: // copy Y to AC
        AC = Y
        break
      case Test.COPY_TO_SP: // copy AC to SP
        SP = AC
        break
      case Test.COPY_FROM_SP: // copy SP to AC
        AC = SP
        break
      case Test.JUMP_ADDR: // jump to address
        PC = data
        break
      case Test.JUMP_IF_EQUAL_ADDR: // jump to address if AC == 0
        if (AC == 0)
          PC = data
        break
      case Test.JUMP_IF_NOT_EQUAL_ADDR: // jump to address if AC != 0
        if (AC != 0)
          PC = data
        break
      case Test.CALL_ADDR: // push PC to SP and jump to address
        SP--
        store(SP, PC + 1)
        PC = data
        break
      case Test.RET: // return to address on SP (pop)
        PC = load(SP)
        SP++
        break
      case Test.INCX: // increment X
        X++
        break
      case Test.DECX: // decrement X
        X--
        break
      case Test.PUSH: // decrement SP and save AC to that location
        SP--
        store(SP, AC)
        break
      case Test.POP: // read value from stack and increment SP
        AC = load(SP)
        SP++
        break
      case Test.COPY_FROM_PC:
        AC = PC
        break
      case Test.IRET: // return from system call
        // load SP and PC from system stack
        PC = load(memory.LENGTH - 1)
        SP = load(memory.LENGTH)
        break
      case Test.END: // cleanup and exit the program
        exit = true
        break
      default:
        break
    }
  }

  return { run, memory }
}


/**
 * 
 */
export type Instruction = {
  opCode: number
  data: number
}

/**
 * 
 * @param instruction 
 * @param debug 
 * @returns 
 */
function parseInstruction(instruction: number, debug = false): Instruction {
  const instructionBuffer = new Uint8Array(new Uint32Array([instruction]).buffer)
  const [opCode, byte2, byte3, byte4] = instructionBuffer
  const parsedInstruction: Instruction = { opCode, data: (byte2 << 16) + (byte3 << 8) + byte4 }

  if (debug) {
    console.error(parsedInstruction)
  }

  return parsedInstruction
}


/**
 * Instruction Set
 */
export enum Test {
  // load given value into AC
  LOAD_VALUE = 1,
  // load value from given address into AC
  LOAD_ADDR = 2,
  // load value in address given by value at given address into AC
  LOAD_IND_ADDR = 3,
  // load value at address of given address + X into AC
  LOAD_IDX_X_ADDR = 4,
  // load value at address of given address + Y into AC
  LOAD_IDX_Y_ADDR = 5,
  // load value at address of SP + X into AC
  LOAD_SP_X = 6,
  // place the value in AC into the address given
  STORE_ADDR = 7,
  // get a random number from 0 to 100
  RAND = 8,
  // print character representation or integer based on port flag
  PUT_PORT = 9,
  // add X to AC
  ADDX = 10,
  // add Y to AC
  ADDY = 11,
  // subtract X from AC
  SUBX = 12,
  // subtract Y from AC
  SUBY = 13,
  // copy AC to X
  COPY_TO_X = 14,
  // copy X to AC
  COPY_FROM_X = 15,
  // copy AC to Y
  COPY_TO_Y = 16,
  // copy Y to AC
  COPY_FROM_Y = 17,
  // copy AC to SP
  COPY_TO_SP = 18,
  // copy SP to AC
  COPY_FROM_SP = 19,
  // jump to address
  JUMP_ADDR = 20,
  // jump to address if AC == 0
  JUMP_IF_EQUAL_ADDR = 21,
  // jump to address if AC != 0
  JUMP_IF_NOT_EQUAL_ADDR = 22,
  // push PC to SP and jump to address
  CALL_ADDR = 23,
  // return to address on SP (pop)
  RET = 24,
  // increment X
  INCX = 25,
  // decrement X
  DECX = 26,
  // decrement SP and save AC to that location
  PUSH = 27,
  // read value from stack and increment SP
  POP = 28,
  // place the PC into AC
  COPY_FROM_PC = 29,
  // return from system call 
  IRET = 30,
  // cleanup and exit the program
  END = 50,
}


