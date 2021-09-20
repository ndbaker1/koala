import { connected } from "process"
import { Token } from "./compiler/lexer"
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

const exampleLexed: Token[] = [
  {
    name: 'FUNCTION',
    value: 'fun'
  },
  {
    name: 'IDENTIFIER',
    value: 'main'
  },
  {
    name: 'LPAREN',
    value: '('
  },
  {
    name: 'RPAREN',
    value: ')'
  },
  {
    name: 'LCURLY',
    value: '{'
  },
  {
    name: 'IDENTIFIER',
    value: 'a'
  },
  {
    name: 'EQ',
    value: '='
  },
  {
    name: 'INT',
    value: 4
  },
  {
    name: 'NEWLINE',
    value: '\n'
  },
  {
    name: 'PRINT',
    value: 'print',
  },
  {
    name: 'LPAREN',
    value: '('
  },
  {
    name: 'IDENTIFIER',
    value: 'a',
  },
  {
    name: 'RPAREN',
    value: ')'
  },
  {
    name: 'RCURLY',
    value: '}'
  },
]

// console.log(exampleLexed.map(a => a.value).reduce((acc, cur) => acc + ' ' + cur, ''))

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
type CodeGen = number[]

const helpers: Function[] = [
  function Program(code: CodeGen) {
    return code.concat([9 + (1 << 24), 50])
  },
  function Expression(op: '+' | '-', op1CodeGen: CodeGen, op2CodeGen: CodeGen) {
    let code = []
    // move AC to SP
    code.push(27)
    // set AC to op1
    code = code.concat(op1CodeGen)
    // move to X
    code.push(14)
    // set AC to op2
    code = code.concat(op2CodeGen)
    // add/delete X to AC
    switch (op) {
      case '+':
        code.push(10)
        break
      case '-':
        code.push(12)
        break
    }
    // move AC to X
    code.push(14)
    // move SP to AC
    code.push(28)
    // add X to AC
    code.push(10)
    // AC contains new sum
    return code
  },
  function Integer(int: number) {
    return [1 + (int << 24)]
  }
]

// const parser = generate("start = ('a' / 'b')+ { return 5 }");
// console.log(parser.parse("abba"))

const parser = createParser(
  `
  {{${helpers.reduce((acc, cur) => acc + cur.toString(), '')}}}
  Program
    = expr:Expression { return Program(expr) }
  Expression
    = head:Term "+" tail:Term { return Expression('+', head, tail) }
  Term
    = "(" expr:Expression ")" { return expr; }
    / Integer
  Integer
    = [0-9]+ { return Integer(parseInt(text(), 10)) }
  `,
)

const memory = createMemory()
const cpu = createCPU(memory)
memory.load(parser.parse(process.argv[2]))
cpu.run()