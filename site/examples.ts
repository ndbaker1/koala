import { objectToVarDecls } from "./services/ASTParser"


export const CodeExample = `fun main() {
  number = 22
  text = 'text in a message\\n\\n'
  functionReturn = recursion(number)
  print(functionReturn)

  print(number)
  number = 33
  print(number)

  print(text)
}

when (true) {
  true -> 2
  false -> 3
}

c = 'string'
print(c)

fun foo1() {
  a = 5
  a = a - 2
  if (a < 2) {
    print('less than 2')
  } else {
    print('2 or greater')
  }

  when (a) {
    1 -> -1
    2 -> 2
    3 -> -3
    else -> 99
  }
}

x = 'x'

fun foo() {
  x = 23
  print(x)
}

print(2)

fun recursion(val) {
  if (val > 0) {
    when (val) {
      0 -> 'function'
      2 -> return 5
    }
  } else {
    c = 'string'
  }
}`

enum ASTType {
  'function_def' = 'function_def',
  'function_call' = 'function_call',
  'if' = 'if',
  'if_else' = 'if_else',
  'return' = 'return',
  'return_expr' = 'return_expr',
  'while' = 'while',
  'binary_expr' = 'binary_expr',
  'variable' = 'variable',
  'bool' = 'bool',
  'str' = 'str',
  'int' = 'int',
  'print' = 'print',
  'assignment' = 'assignment',
  'increment' = 'increment',
  'decrement' = 'decrement',
  'when' = 'when',
  'when_case_stmt' = 'when_case_stmt',
  'when_case_expr' = 'when_case_expr',
  'when_else_stmt' = 'when_else_stmt',
  'when_else_expr' = 'when_else_expr',
}

// dummy var
let code = ''
export const CodeGens: Record<ASTType, (node: any) => void> = {
  'function_def': function (node) {

  },
  'function_call': function (node) {

  },
  'if': function (node) {
    code += 'if\n'
    CodeGens[node.cond.type as ASTType](node.cond)
  },
  'if_else': function (node) {

  },
  'return': function (node) {

  },
  'return_expr': function (node) {

  },
  'while': function (node) {

  },
  'binary_expr': function (node) {
    code += 'binary expr\n'
    CodeGens[node.args[0].type as ASTType](node.args[0])
    CodeGens[node.args[1].type as ASTType](node.args[1])
    code += `perform ${node.binop}\n`
  },
  'variable': function (node) {
    code += 'variable: ' + node.id + '\n'
  },
  'bool': function (node) {

  },
  'str': function (node) {

  },
  'int': function (node) {
    code += 'int: ' + node.value + '\n'
  },
  'print': function (node) {

  },
  'assignment': function (node) {

  },
  'increment': function (node) {

  },
  'decrement': function (node) {

  },
  'when': function (node) {

  },
  'when_case_stmt': function (node) {

  },
  'when_case_expr': function (node) {

  },
  'when_else_stmt': function (node) {

  },
  'when_else_expr': function (node) {

  },
}

export const GrammarExample = `PROGRAM
  = stmts:STMTS LINEEND prog:PROGRAM
    { return stmts.concat(prog) }
  / func_def:FUNC_DEF LINEEND prog:PROGRAM
    { return [func_def].concat(prog) }
  / FUNC_DEF
  / STMTS

FUNC_DEF
  = WS "fun" WS id:IDENTIFIER "(" WS params:PARAMETERS WS ")" WS stmts:BLOCK
    { return { type: '${CodeGens.assignment.name}', id: id.id, params, stmts } }
  / WS "fun" WS id:IDENTIFIER "(" ")" WS stmts:BLOCK
    { return { type: '${CodeGens.function_def.name}', id: id.id, params: [], stmts } }
STMTS
  = stmt:STMT LINEEND stmts:STMTS
    { return [stmt].concat(stmts) }
  / stmt:STMT
    { return [stmt] }
STMT
  = WS "if" WS "(" cond:EXPR ")" WS true_branch:BLOCK WS "else" WS false_branch:BLOCK
    { return { type: '${CodeGens.if_else.name}', cond, true_branch, false_branch } }
  / WS "if" WS "(" cond:EXPR ")" WS stmts:BLOCK
    { return { type: '${CodeGens.if.name}', cond, stmts } }
  / WS when:WHEN
    { return when }
  / WS "while" WS "(" cond:EXPR ")" WS stmts:BLOCK
    { return { type: '${CodeGens.while.name}', cond, stmts } }
  / WS "print" "(" expr:EXPR ")"
    { return { type: '${CodeGens.print.name}', expr } }
  / WS id:IDENTIFIER WS type:ASSIGN WS expr:EXPR
    { return { id, type: '${CodeGens.assignment.name}', expr } }
  / WS "return" WS expr:EXPR
    { return { type: '${CodeGens.return_expr.name}', expr } }
  / WS "return"
    { return { type: '${CodeGens.return.name}' } }
  / WS id:IDENTIFIER "++"
    { return { type: '${CodeGens.increment.name}', id } }
  / WS id:IDENTIFIER "--"
    { return { type: '${CodeGens.decrement.name}', id } }
  / WS block:BLOCK
    { return block }
BLOCK
  = "{" LINEEND stmts:STMTS LINEEND WS "}"
    { return stmts }
WHEN
  = "when" WS "(" expr:EXPR ")" WS "{" LINEEND when_cases:WHENCASES LINEEND WS "}"
    { return { type: '${CodeGens.when.name}', expr, when_cases } }
WHENCASES
  = when_case:WHENCASE LINEEND when_cases:WHENCASES
    { return [when_case].concat(when_cases) }
  / when_case:WHENCASE
    { return [when_case] }
WHENCASE
  = WS case_expr:EXPR WS "->" WS case_value:STMT
    { return { type: '${CodeGens.when_case_stmt.name}', case_expr, case_value } }
  / WS case_expr:EXPR WS "->" WS case_value:EXPR
    { return { type: '${CodeGens.when_case_expr.name}', case_expr, case_value } }
  / WS "else" WS "->" WS case_value:STMT
    { return { type: '${CodeGens.when_else_stmt.name}', case_value } }
  / WS "else" WS "->" WS case_value:EXPR
    { return { type: '${CodeGens.when_else_expr.name}', case_value } }
EXPR
  = "(" WS expr:EXPR WS ")"
    { return expr }
  / BINARY_EXPR
  / FUNC_CALL
  / IDENTIFIER
  / STRINGLIT
  / INTLIT
  / BOOLLIT

BINARY_EXPR
  = fun_call:FUNC_CALL WS binop:BINARYOP WS expr:EXPR
    { return { type: '${CodeGens.binary_expr.name}', binop, args: [fun_call, expr] } }
  / id:IDENTIFIER WS binop:BINARYOP WS expr:EXPR
    { return { type: '${CodeGens.binary_expr.name}', binop, args: [id, expr] } }
  / str:STRINGLIT WS binop:BINARYOP WS expr:EXPR
    { return { type: '${CodeGens.binary_expr.name}', binop, args: [str, expr] } }
  / int:INTLIT WS binop:BINARYOP WS expr:EXPR
    { return { type: '${CodeGens.binary_expr.name}', binop, args: [int, expr] } }

PARAMETERS
  = id:IDENTIFIER WS "," WS params:PARAMETERS
    { return [id].concat(params) }
  / id:IDENTIFIER
    { return [id] }

ARGS
  = expr:EXPR WS "," WS args:ARGS
    { return [expr].concat(args) }
  / expr:EXPR
    { return [expr] }

BINARYOP = ("*"/"/"/"+"/"-"/"<"/">"/"<="/">="/"=="/"<>"/"||"/"&&")
  { return text() }
ASSIGN = "="

FUNC_CALL
  = id:IDENTIFIER "(" WS args:ARGS WS ")"
    { return { type: '${CodeGens.function_call.name}', id: id.id, args } }
  / id:IDENTIFIER "(" ")"
    { return { type: '${CodeGens.function_call.name}', id: id.id, args: [] } }

IDENTIFIER = [a-zA-Z][a-zA-Z0-9]*
  { return { type: '${CodeGens.variable.name}', id: text() } }
INTLIT = "-"? [0-9]+
  { return { type: '${CodeGens.int.name}', value: parseInt(text()) } }
STRINGLIT = "'" [a-zA-Z0-9 \\n\\r\\t]* "'"
  { return { type: '${CodeGens.str.name}', value: text() } }
BOOLLIT = ("true"/"false")
  { return { type: '${CodeGens.bool.name}', value: Boolean(text()) } }

LINEEND = WS NL
NL = [\\r\\n]+
WS = [ \\t]*`


export const ASTParserExample = `// Here is where you will write an implementation
// of the AST in Assembly, Machine Code, or etc

// Access the root of the AST using:
ASTRoot
// Return a String of the result
let code  = ''
// keep track of the variable addresses on the stack
const variableAddresses = {}
// Starting stack pointer offset
let stackOffset = 0
// the example grammar handles types like this
${objectToVarDecls(CodeGens)} 
// real work
for (const node of ASTRoot) CodeGens[node.type](node)
return code + 50

`