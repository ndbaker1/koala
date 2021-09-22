

type CodeGen = number[]

export function Program(ASTRoot: any) {
  ''
  return 'string representing assembly'
}


/**
 * This is the Grammar for a Language which does not contain Semicolons (;),
 * and utilizes Line endings in order to structure the AST.
 */
export const complexGrammar = `
 PROGRAM
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
 WS = [ \\t]*
 `


export const complexGrammarCode = `

fun main() {
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
}


`

export const CodeGens = {
  Program: function Program(code: CodeGen, type: "str" | 'int') {
    let newCode: CodeGen = []
    switch (type) {
      case "int":
        // continue previous code
        newCode = newCode.concat(code)
        // push print number
        newCode.push(9 + (1 << 24))
        break
      case "str":
        // continue previous code
        newCode = newCode.concat(code)
        // copy string address at PC to Y
        newCode.push(15)
        // read PC for jumping
        newCode.push(29)
        // add l
        newCode.push(15)
        // add value to PC for jumping loop


        break
    }

    return newCode.concat([50 /* end program */])
  },
  Expression: function Expression(op: '+' | '-', op1CodeGen: CodeGen, op2CodeGen: CodeGen) {
    // set AC to op2
    let code: CodeGen = op1CodeGen
    // move AC to SP
    code.push(27)
    // set AC to op1
    code = code.concat(op2CodeGen)
    // move AC to X
    code.push(14)
    // move SP to AC
    code.push(28)
    // add/delete X to AC
    switch (op) {
      case '+':
        code.push(10)
        break
      case '-':
        code.push(12)
        break
    }
    // AC contains new sum
    return code
  },
  Integer: function Integer(int: number) {
    // load number into AC
    return [1 + (int << 24)]
  },
  STRING: function STRING(str: string) {
    let code: CodeGen = []
    // get SP from AC
    code.push(19)
    // move AC to X
    code.push(14)
    // load 1 into AC
    code.push(1 + 0)
    // add X to AC
    code.push(10)
    // save back to X
    code.push(14)
    // push all characters onto stack
    for (const char of str) {
      // set AC top char in string
      code.push(1 + (char.charCodeAt(0) << 24))
      // push string onto stack
      code.push(27)
    }
    // load 0 into AC
    code.push(1 + 0)
    // push 0 onto stack for end of string
    code.push(27)
    // copy X back to AC
    code.push(15)
    // start address of string is at AC
    return code
  },
}

/**
 * 
 * @param map 
 * @returns 
 */
const convertFunctionMap = (map: Record<string, Function>) => Object.values(map).reduce((acc, cur) => acc + cur.toString(), '')


export const Grammar = `
{{
  ${convertFunctionMap(CodeGens)}
}}

Program
  = "print(" expr:Expression ")" { return ${CodeGens.Program}(expr, 'int') }
  / "print(" str:String ")" { return ${CodeGens.Program}(str, 'str') }
Expression
  = head:Term "+" tail:Term { return ${CodeGens.Expression}('+', head, tail) }
  / head:Term "-" tail:Term { return ${CodeGens.Expression}('-', head, tail) }
Term
  = "(" expr:Expression ")" { return expr; }
  / Integer
Integer
  = [0-9]+ { return ${CodeGens.Integer}(parseInt(text(), 10)) }
String
  = "\'" [a-zA-Z0-9 ]* "\'" { return ${CodeGens.STRING}(text()) }
NEWLINE
  = [\\r\\n]*
`


