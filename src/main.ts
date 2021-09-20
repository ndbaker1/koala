import { Grammar } from "./compiler/grammar"
import { createParser } from "./compiler/Parser"
import { createCPU } from "./CPU/CPU"
import { createMemory } from "./CPU/Memory"

/*

fun main() {
  int number = 22
  string text = 'text in a message\n\n'
  int[4] list = [2, 3, 5, 6]
  string functionReturn = recursion(list[2])
  print(functionReturn)

  print(number)
  number = 33
  print(number)

  print(text)
  
}


fun recursion(val: int) {
  if (val > 0) {
    when (val) {
      0 -> { return "finished" }
      else -> { return recursion(val--) }
    }
  }
}

*/

/*

fun main() {
  a = 4
  print(a)
}

*/

/*

Example Grammar:


PROGRAM:
  | IDENTIFIER = EXPR PROGRAM
  | IDENTIFIER ( ARGS ) { STMTS } PROGRAM
  | λ
STMTS:
  | STMT STMTS
  | λ
STMT:
  | IF ( EXPR ) STMT IFEND
  | WHILE ( EXPR ) STMT
  | PRINT ( EXPR )
  | IDENTIFIER = EXPR
  | RETURN
  | RETURN EXPR
  | IDENTIFIER ++
  | IDENTIFIER --
  | { SMTMS }
EXPRLIST:
  | EXPR EXPRLIST
  | EXPR
EXPR:
  | IDENTIFIER
  | IDENTIFIER ( ARGS )
  | STRING
  | INT
ARGS:
  | EXPR , ARGS
  | EXPR

*/

const [, , ...args] = process.argv
const input = args.find(arg => !/^--.*/.test(arg))!!
const debug = !!args.find(arg => /--debug/.test(arg))

const parser = createParser(
  `
  {{${Object.values(Grammar).reduce((acc, cur) => acc + cur.toString(), '')}}}
  
  Program
    = expr:Expression { return ${Grammar.Program}(expr) }
  Expression
    = head:Term "+" tail:Term { return ${Grammar.Expression}('+', head, tail) }
    / head:Term "-" tail:Term { return ${Grammar.Expression}('-', head, tail) }
  Term
    = "(" expr:Expression ")" { return expr; }
    / Integer
  Integer
    = [0-9]+ { return ${Grammar.Integer}(parseInt(text(), 10)) }
  `
)

const instructions = parser.parse(input)
const memory = createMemory()
const cpu = createCPU({ memory, debug })
memory.load(instructions)
cpu.run()