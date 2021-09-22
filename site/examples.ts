export const ASTParserExample = `// Here is where you will write an implementation
// of the AST in Assembly, Machine Code, or etc

// Access the root of the AST using:
ASTRoot

// Return a String of the result
return 'machine code representation'
`

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

export const GrammarExample = `PROGRAM
  = stmts:STMTS LINEEND prog:PROGRAM
    { return stmts.concat(prog) }
  / func_def:FUNC_DEF LINEEND prog:PROGRAM
    { return [func_def].concat(prog) }
  / FUNC_DEF
  / STMTS

FUNC_DEF
  = WS "fun" WS id:IDENTIFIER "(" WS args:ARGS WS ")" WS stmts:BLOCK
    { return { type: 'function_def', id: id.id, args, stmts } }
  / WS "fun" WS id:IDENTIFIER "(" ")" WS stmts:BLOCK
    { return { type: 'function_def', id: id.id, args: [], stmts } }
STMTS
  = stmt:STMT LINEEND stmts:STMTS
    { return [stmt].concat(stmts) }
  / stmt:STMT
    { return [stmt] }
STMT
  = WS "if" WS "(" cond:EXPR ")" WS true_branch:BLOCK WS "else" WS false_branch:BLOCK
    { return { control: 'if_else', cond, true_branch, false_branch } }
  / WS "if" "(" cond:EXPR ")" WS stmts:BLOCK
    { return { control: 'if_else', cond, stmts } }
  / WS when:WHEN
    { return when }
  / WS "while" "(" cond:EXPR ")" WS stmts:BLOCK
    { return { control: 'while', cond, stmts } }
  / WS "print" "(" expr:EXPR ")"
    { return { action: 'print', expr } }
  / WS id:IDENTIFIER WS action:ASSIGN WS expr:EXPR
    { return { id, action, expr } }
  / WS "return" WS expr:EXPR
    { return { control: 'return_expr', expr } }
  / WS "return"
    { return { control: 'return' } }
  / WS id:IDENTIFIER "++"
    { return { action: 'increment', id } }
  / WS id:IDENTIFIER "--"
    { return { action: 'decrement', id } }
  / WS block:BLOCK
    { return block }
BLOCK
  = "{" LINEEND stmts:STMTS LINEEND WS "}"
    { return stmts }
WHEN
  = "when" WS "(" expr:EXPR ")" WS "{" LINEEND when_cases:WHENCASES LINEEND WS "}"
    { return { control: 'when', expr, when_cases } }
WHENCASES
  = when_case:WHENCASE LINEEND when_cases:WHENCASES
    { return [when_case].concat(when_cases) }
  / when_case:WHENCASE
    { return [when_case] }
WHENCASE
  = WS case_expr:EXPR WS "->" WS case_value:STMT
    { return { case: 'expr', case_expr, case_value } }
  / WS case_expr:EXPR WS "->" WS case_value:EXPR
    { return { case: 'expr', case_expr, case_value } }
  / WS "else" WS "->" WS case_value:STMT
    { return { case: 'else', case_value } }
  / WS "else" WS "->" WS case_value:EXPR
    { return { case: 'else', case_value } }
EXPR
  = fun_call:FUNC_CALL binop:BINARYOP WS expr:EXPR
    { return { fun_call, binop, expr } }
  / id:IDENTIFIER WS binop:BINARYOP WS expr:EXPR
    { return { id, binop, expr } }
  / str:STRINGLIT WS binop:BINARYOP WS expr:EXPR
    { return { str, binop, expr } }
  / int:INTLIT WS binop:BINARYOP WS expr:EXPR
    { return { int, binop, expr } }
  / FUNC_CALL
  / IDENTIFIER
  / STRINGLIT
  / INTLIT
  / BOOLLIT

ARGS
  = EXPR WS "," WS ARGS
  / EXPR

BINARYOP = ("*"/"/"/"+"/"-"/"<"/">"/"<="/">="/"=="/"<>"/"||"/"&&")
  { return text() }
ASSIGN = "="

FUNC_CALL
  = id:IDENTIFIER "(" WS args:ARGS WS ")"
    { return { type: 'function_call', id: id.id, args }  }
  / id:IDENTIFIER "(" ")"
    { return { type: 'function_call', id: id.id, args }  }

IDENTIFIER = [a-zA-Z][a-zA-Z0-9]*
  { return { type: 'var', id: text() } }
INTLIT = "-"? [0-9]+
  { return { type: 'int', value: parseInt(text()) } }
STRINGLIT = "\'" [a-zA-Z0-9 \\n\\r\\t]* "\'"
  { return { type: 'str', value: text() } }
BOOLLIT = ("true"/"false")
  { return { type: 'bool', value: Boolean(text()) } }

LINEEND = WS NL
NL = [\\r\\n]+
WS = [ \\t]*`