import { createTestCPU } from "./architecture/test"
import { createMemory, Memory } from "./Memory"


export type CPU = {
  run: () => void
  memory: Memory
}

/**
 * 
 */
export class CPUConfig {
  memory = createMemory()
  debug?= false
}

/**
 * 
 */
type CPUType = "test"

/**
 * 
 * @param config config object for the CPU
 * @param type which architecture to create
 * @returns a CPU object
 */
export function createCPU(
  config: CPUConfig = new CPUConfig(),
  type: CPUType = "test",
): CPU {
  switch (type) {
    case "test":
      return createTestCPU(config)
  }
}


