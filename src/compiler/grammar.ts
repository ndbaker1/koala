


type CodeGen = number[]


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


