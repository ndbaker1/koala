// import { Grammar } from "./compiler/grammar"
// import { createParser } from "./compiler/Parser"
// import { createCPU } from "./CPU/CPU"
// import { createMemory } from "./CPU/Memory"


// const [, , ...args] = process.argv
// const input = args.find(arg => !/^--.*/.test(arg))!!
// const debug = !!args.find(arg => /--debug/.test(arg))

// const parser = createParser(Grammar)

// const instructions = parser.parse(input)
// const memory = createMemory()
// const cpu = createCPU({ memory, debug })
// memory.load(instructions)
// cpu.run()
