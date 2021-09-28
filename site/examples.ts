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

export const ASTTypes = {
  test: function (node: any) { },
  functionDefinition: 'function_def',
  functionCall: 'function_call',
  ifControl: 'if',
  ifElseControl: 'if_else',
  functionReturn: 'return',
  functionReturnExpr: 'return_expr',
  whileControl: 'while',
  expr: 'expr',
  binaryExpr: 'binary_expr',
  variable: 'variable',
  bool: 'bool',
  string: 'str',
  int: 'int',
  print: 'print',
  assignment: 'assignment',
  increment: 'increment',
  decrement: 'decrement',

  when: 'when',
  whenCaseStmt: 'when_case_stmt',
  whenCaseExpr: 'when_case_expr',
  whenElseStmt: 'when_else_stmt',
  whenElseExpr: 'when_else_expr',
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
    { return { type: '${ASTTypes.functionDefinition}', id: id.id, params, stmts } }
  / WS "fun" WS id:IDENTIFIER "(" ")" WS stmts:BLOCK
    { return { type: '${ASTTypes.functionDefinition}', id: id.id, params: [], stmts } }
STMTS
  = stmt:STMT LINEEND stmts:STMTS
    { return [stmt].concat(stmts) }
  / stmt:STMT
    { return [stmt] }
STMT
  = WS "if" WS "(" cond:EXPR ")" WS true_branch:BLOCK WS "else" WS false_branch:BLOCK
    { return { type: '${ASTTypes.ifElseControl}', cond, true_branch, false_branch } }
  / WS "if" "(" cond:EXPR ")" WS stmts:BLOCK
    { return { type: '${ASTTypes.ifControl}', cond, stmts } }
  / WS when:WHEN
    { return when }
  / WS "while" "(" cond:EXPR ")" WS stmts:BLOCK
    { return { type: '${ASTTypes.whileControl}', cond, stmts } }
  / WS "print" "(" expr:EXPR ")"
    { return { type: '${ASTTypes.print}', expr } }
  / WS id:IDENTIFIER WS type:ASSIGN WS expr:EXPR
    { return { id, type: '${ASTTypes.assignment}', expr } }
  / WS "return" WS expr:EXPR
    { return { type: '${ASTTypes.functionReturnExpr}', expr } }
  / WS "return"
    { return { type: '${ASTTypes.functionReturn}' } }
  / WS id:IDENTIFIER "++"
    { return { type: '${ASTTypes.increment}', id } }
  / WS id:IDENTIFIER "--"
    { return { type: '${ASTTypes.decrement}', id } }
  / WS block:BLOCK
    { return block }
BLOCK
  = "{" LINEEND stmts:STMTS LINEEND WS "}"
    { return stmts }
WHEN
  = "when" WS "(" expr:EXPR ")" WS "{" LINEEND when_cases:WHENCASES LINEEND WS "}"
    { return { type: '${ASTTypes.when}', expr, when_cases } }
WHENCASES
  = when_case:WHENCASE LINEEND when_cases:WHENCASES
    { return [when_case].concat(when_cases) }
  / when_case:WHENCASE
    { return [when_case] }
WHENCASE
  = WS case_expr:EXPR WS "->" WS case_value:STMT
    { return { type: '${ASTTypes.whenCaseStmt}', case_expr, case_value } }
  / WS case_expr:EXPR WS "->" WS case_value:EXPR
    { return { type: '${ASTTypes.whenCaseExpr}', case_expr, case_value } }
  / WS "else" WS "->" WS case_value:STMT
    { return { type: '${ASTTypes.whenElseStmt}', case_value } }
  / WS "else" WS "->" WS case_value:EXPR
    { return { type: '${ASTTypes.whenElseExpr}', case_value } }
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
    { return { type: '${ASTTypes.binaryExpr}', binop, args: [fun_call, expr] } }
  / id:IDENTIFIER WS binop:BINARYOP WS expr:EXPR
    { return { type: '${ASTTypes.binaryExpr}', binop, args: [id, expr] } }
  / str:STRINGLIT WS binop:BINARYOP WS expr:EXPR
    { return { type: '${ASTTypes.binaryExpr}', binop, args: [str, expr] } }
  / int:INTLIT WS binop:BINARYOP WS expr:EXPR
    { return { type: '${ASTTypes.binaryExpr}', binop, args: [int, expr] } }

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
    { return { type: '${ASTTypes.functionCall}', id: id.id, args } }
  / id:IDENTIFIER "(" ")"
    { return { type: '${ASTTypes.functionCall}', id: id.id, args: [] } }

IDENTIFIER = [a-zA-Z][a-zA-Z0-9]*
  { return { type: '${ASTTypes.variable}', id: text() } }
INTLIT = "-"? [0-9]+
  { return { type: '${ASTTypes.int}', value: parseInt(text()) } }
STRINGLIT = "'" [a-zA-Z0-9 \\n\\r\\t]* "'"
  { return { type: '${ASTTypes.string}', value: text() } }
BOOLLIT = ("true"/"false")
  { return { type: '${ASTTypes.bool}', value: Boolean(text()) } }

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
${objectToVarDecls(ASTTypes)} 
// real work
for (const node of ASTRoot) codeGen[node.type](node)
return code + 50

`