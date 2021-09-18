import { parseInstruction } from "../compiler/Instruction"
import { createMemory, Memory } from "./Memory"


export type CPU = {
  run: () => void
  memoryInterface: Memory
}

export function createCPU(providedMemory?: Memory): CPU {

  const memoryInterface = providedMemory || createMemory()

  /**
   * Stack accumulator with Registers architecture 
   */

  let AC = 0 // Accumulator
  let PC = 0 // Program Counter
  let IR = 0 // Instruction Register
  let X = 0 // arithmetic register 1
  let Y = 0 // arithmetic register 2
  let V = 0 // Value register for returns
  let SP = memoryInterface.LENGTH // Stack begins at the end of Memory and grows towards 0


  let exit = false

  /**
   * Run the program with Fetch and Execute Cycle
   */
  const run = () => {
    exit = false
    // while the fetch does not fail, run the returned operation
    while (!exit && (IR = fetch()) != -1)
      execute()
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
  const load = (address: number) => memoryInterface.read(address)

  /**
   * save data to memory address
   * 
   * @param address
   * @param data
   */
  const store = (address: number, data: number) => memoryInterface.write(address, data)

  /**
   * execute instruction with fetch cycle
   */
  const execute = () => {

    // const { opCode, data } = parseInstruction(IR)

    let address = 0

    switch (IR) {
      case LOAD_VALUE: // load given value into AC
        AC = fetch()
        break
      case LOAD_ADDR: // load value from given address into AC
        address = fetch()
        AC = load(address)
        break
      case LOAD_IND_ADDR: // load value in address given by value at given address into AC
        address = fetch()
        address = load(address)
        AC = load(address)
        break
      case LOAD_IDX_X_ADDR: // load value at address of given address + X into AC
        address = fetch()
        AC = load(address + X)
        break
      case LOAD_IDX_Y_ADDR: // load value at address of given address + Y into AC
        address = fetch()
        AC = load(address + Y)
        break
      case LOAD_SP_X: // load value at address of SP + X into AC
        AC = load(SP + X)
        break
      case STORE_ADDR: // place the value in AC into the address given
        address = fetch()
        store(address, AC)
        break
      case RAND: // get a random number from 0 to 100
        AC = Math.round(Math.random() * 100)
        break
      case PUT_PORT: // print character representation or integer based on port flag
        const port = fetch()
        switch (port) {
          case 1:
            process.stdout.write(AC.toFixed(0))
            break
          case 2:
            process.stdout.write(String.fromCharCode(AC))
            break
        }
        break
      case ADDX: // add X to AC
        AC += X
        break
      case ADDY: // add Y to AC
        AC += Y
        break
      case SUBX: // subtract X from AC
        AC -= X
        break
      case SUBY: // subtract Y from AC
        AC -= Y
        break
      case COPY_TO_X: // copy AC to X
        X = AC
        break
      case COPY_FROM_X: // copy X to AC
        AC = X
        break
      case COPY_TO_Y: // copy AC to Y
        Y = AC
        break
      case COPY_FROM_Y: // copy Y to AC
        AC = Y
        break
      case COPY_TO_SP: // copy AC to SP
        SP = AC
        break
      case COPY_FROM_SP: // copy SP to AC
        AC = SP
        break
      case JUMP_ADDR: // jump to address
        PC = fetch()
        break
      case JUMP_IF_EQUAL_ADDR: // jump to address if AC == 0
        if (AC == 0)
          PC = fetch()
        else
          PC++
        break
      case JUMP_IF_NOT_EQUAL_ADDR: // jump to address if AC != 0
        if (AC != 0)
          PC = fetch()
        else
          PC++
        break
      case CALL_ADDR: // push PC to SP and jump to address
        SP--
        store(SP, PC + 1)
        PC = fetch()
        break
      case RET: // return to address on SP (pop)
        PC = load(SP)
        SP++
        break
      case INCX: // increment X
        X++
        break
      case DECX: // decrement X
        X--
        break
      case PUSH: // decrement SP and save data to that location
        SP--
        store(SP, AC)
        break
      case POP: // read value from stack and increment SP
        AC = load(SP)
        SP++
        break
      case IRET: // return from system call
        // load SP and PC from system stack
        PC = load(memoryInterface.LENGTH - 1)
        SP = load(memoryInterface.LENGTH)
        break
      case END: // cleanup and exit the program
        exit = true
        break
      default:
        break
    }
  }

  return { run, memoryInterface }
}


// Instruction Definitions
const LOAD_VALUE = 1
const LOAD_ADDR = 2
const LOAD_IND_ADDR = 3
const LOAD_IDX_X_ADDR = 4
const LOAD_IDX_Y_ADDR = 5
const LOAD_SP_X = 6
const STORE_ADDR = 7
const RAND = 8
const PUT_PORT = 9
const ADDX = 10
const ADDY = 11
const SUBX = 12
const SUBY = 13
const COPY_TO_X = 14
const COPY_FROM_X = 15
const COPY_TO_Y = 16
const COPY_FROM_Y = 17
const COPY_TO_SP = 18
const COPY_FROM_SP = 19
const JUMP_ADDR = 20
const JUMP_IF_EQUAL_ADDR = 21
const JUMP_IF_NOT_EQUAL_ADDR = 22
const CALL_ADDR = 23
const RET = 24
const INCX = 25
const DECX = 26
const PUSH = 27
const POP = 28
const IRET = 30
const END = 50
