import { Arch, parseInstruction } from "../compiler/Instruction"
import { createMemory, Memory } from "./Memory"


export type CPU = {
  run: () => void
  memory: Memory
}

class CPUConfig {
  memory = createMemory()
  debug = false
}

export function createCPU({ memory, debug }: CPUConfig = new CPUConfig()): CPU {

  /**
   * Stack accumulator with Registers architecture 
   */

  let AC = 0 // Accumulator
  let SP = memory.LENGTH // Stack begins at the end of Memory and grows towards 0

  let PC = 0 // Program Counter
  let IR = 0 // Instruction Register

  let X = 0 // arithmetic register 1
  let Y = 0 // arithmetic register 2
  let V = 0 // Value register for returns


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
      case Arch.LOAD_VALUE: // load given value into AC
        AC = data
        break
      case Arch.LOAD_ADDR: // load value from given address into AC
        address = data
        AC = load(address)
        break
      case Arch.LOAD_IND_ADDR: // load value in address given by value at given address into AC
        address = data
        address = load(address)
        AC = load(address)
        break
      case Arch.LOAD_IDX_X_ADDR: // load value at address of given address + X into AC
        address = data
        AC = load(address + X)
        break
      case Arch.LOAD_IDX_Y_ADDR: // load value at address of given address + Y into AC
        address = data
        AC = load(address + Y)
        break
      case Arch.LOAD_SP_X: // load value at address of SP + X into AC
        AC = load(SP + X)
        break
      case Arch.STORE_ADDR: // place the value in AC into the address given
        address = data
        store(address, AC)
        break
      case Arch.RAND: // get a random number from 0 to 100
        AC = Math.round(Math.random() * 100)
        break
      case Arch.PUT_PORT: // print character representation or integer based on port flag
        const port = data
        switch (port) {
          case 1:
            process.stdout.write(AC.toFixed(0))
            break
          case 2:
            process.stdout.write(String.fromCharCode(AC))
            break
        }
        break
      case Arch.ADDX: // add X to AC
        AC += X
        break
      case Arch.ADDY: // add Y to AC
        AC += Y
        break
      case Arch.SUBX: // subtract X from AC
        AC -= X
        break
      case Arch.SUBY: // subtract Y from AC
        AC -= Y
        break
      case Arch.COPY_TO_X: // copy AC to X
        X = AC
        break
      case Arch.COPY_FROM_X: // copy X to AC
        AC = X
        break
      case Arch.COPY_TO_Y: // copy AC to Y
        Y = AC
        break
      case Arch.COPY_FROM_Y: // copy Y to AC
        AC = Y
        break
      case Arch.COPY_TO_SP: // copy AC to SP
        SP = AC
        break
      case Arch.COPY_FROM_SP: // copy SP to AC
        AC = SP
        break
      case Arch.JUMP_ADDR: // jump to address
        PC = data
        break
      case Arch.JUMP_IF_EQUAL_ADDR: // jump to address if AC == 0
        if (AC == 0)
          PC = data
        break
      case Arch.JUMP_IF_NOT_EQUAL_ADDR: // jump to address if AC != 0
        if (AC != 0)
          PC = data
        break
      case Arch.CALL_ADDR: // push PC to SP and jump to address
        SP--
        store(SP, PC + 1)
        PC = data
        break
      case Arch.RET: // return to address on SP (pop)
        PC = load(SP)
        SP++
        break
      case Arch.INCX: // increment X
        X++
        break
      case Arch.DECX: // decrement X
        X--
        break
      case Arch.PUSH: // decrement SP and save AC to that location
        SP--
        store(SP, AC)
        break
      case Arch.POP: // read value from stack and increment SP
        AC = load(SP)
        SP++
        break
      case Arch.IRET: // return from system call
        // load SP and PC from system stack
        PC = load(memory.LENGTH - 1)
        SP = load(memory.LENGTH)
        break
      case Arch.END: // cleanup and exit the program
        exit = true
        break
      default:
        break
    }
  }

  return { run, memory }
}


