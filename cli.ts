import { createParser } from './src/compiler/Parser'
import { createCPU } from './src/CPU/CPU'
import { createMemory } from './src/CPU/Memory'
import { ASTParserExample, CodeExample, GrammarExample } from './site/examples'
import { bootstrapASTParser } from './src/compiler/ASTParser'

const grammarParser = createParser(GrammarExample)
const ASTParserCode = bootstrapASTParser('ASTRoot')(ASTParserExample)
const ASTParser = Function(ASTParserCode) as (_: any) => string

const code = ASTParser(grammarParser.parse(CodeExample))
const instructions = code.split('\n').map(Number)

const memory = createMemory()
memory.load(new Uint32Array(instructions))

const CPU = createCPU({ memory })
CPU.run()
