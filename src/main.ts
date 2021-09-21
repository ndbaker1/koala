import { Grammar } from "./compiler/grammar"
import { createParser } from "./compiler/Parser"
import { createCPU } from "./CPU/CPU"
import { createMemory } from "./CPU/Memory"


const [, , ...args] = process.argv
const input = args.find(arg => !/^--.*/.test(arg))!!
const debug = !!args.find(arg => /--debug/.test(arg))

// const parser = createParser(Grammar)

// const instructions = parser.parse(input)
// const memory = createMemory()
// const cpu = createCPU({ memory, debug })
// memory.load(instructions)
// cpu.run()




const complexGrammar = `

PROGRAM
  = FUNCDEF PROGRAM
  / stmts:STMTS prog:PROGRAM        { return stmts.concat(prog) }
  / func_def:FUNCDEF prog:PROGRAM   { return [func_def].concat(prog) }
  / FUNCDEF
  / STMTS

FUNCDEF
  = "fun" WHITESPACE func_def:IDENTIFIER "(" ")" WHITESPACE "{" LINEEND stmts:STMTS "}"     { return { func_def, stmts } }
STMTS
  = stmt:STMT stmts:STMTS   { return [stmt].concat(stmts) }
  / stmt:STMT               { return [stmt] }
STMT
  = WHITESPACE "if" "(" cond:EXPR ")" true_branch:STMT "else" false_branch:STMT         { return { action: 'if', true_branch, false_branch } }
  / WHITESPACE "if" "(" EXPR ")" STMT
  / WHITESPACE "while" "(" EXPR ")" STMT
  / WHITESPACE "print" "(" expr:EXPR ")" LINEEND                                        { return { action: 'print', expr } }
  / WHITESPACE id:IDENTIFIER WHITESPACE action:ASSIGN WHITESPACE expr:EXPR LINEEND      { return { id, action, expr } }
  / WHITESPACE "return" LINEEND
  / WHITESPACE "return" WHITESPACE EXPR LINEEND
  / WHITESPACE IDENTIFIER "++" LINEEND
  / WHITESPACE IDENTIFIER "--" LINEEND
  / WHITESPACE "{" STMTS "}"
EXPRLIST
  = EXPR EXPRLIST
  / EXPR 
EXPR
  = id:IDENTIFIER WHITESPACE binop:BINARYOP WHITESPACE expr:EXPR                { return { id, binop, expr } }
  / id:IDENTIFIER WHITESPACE "(" ARGS ")" binop:BINARYOP WHITESPACE expr:EXPR   { return { id, binop, expr } }
  / str:STRINGLIT WHITESPACE binop:BINARYOP WHITESPACE expr:EXPR                { return { str, binop, expr } }
  / int:INTLIT WHITESPACE binop:BINARYOP WHITESPACE expr:EXPR                   { return { int, binop, expr } }
  / IDENTIFIER
  / IDENTIFIER "(" ARGS ")"
  / STRINGLIT                                                                   { return text() }
  / INTLIT                                                                      { return parseInt(text()) }

ARGS
  = EXPR "," ARGS
  / EXPR

BINARYOP
  = ("*"/"/"/"+"/"-"/"<"/">"/"<="/">="/"=="/"<>"/"||"/"&&") { return text() }

ASSIGN = "="                        { return text() }
IDENTIFIER = [a-zA-Z][a-zA-Z0-9]*   { return text() }

INTLIT = [0-9]+ 
STRINGLIT = "\'" [a-zA-Z0-9 ]* "\'" 

LINEEND = WHITESPACE NEWLINE
NEWLINE = [\\r\\n]+
WHITESPACE = [ ]*
`

const code = `

c = 'string'
print(c)

fun main() {   
  a = 5
  a = a + 5
  print(a)
}

x = 55

`

const complexParser = createParser(complexGrammar, { debug })
console.log(JSON.stringify(complexParser.parse(code), null, 2))

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

