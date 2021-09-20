

type CodeGen = number[]

export const Grammar = {
  Program: function Program(code: CodeGen) {
    return code.concat([
      // print AC
      9 + (1 << 24),
      // end program
      50
    ])
  },
  Expression: function Expression(op: '+' | '-', op1CodeGen: CodeGen, op2CodeGen: CodeGen) {
    let code: CodeGen = []
    // set AC to op2
    code = code.concat(op1CodeGen)
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
  }
}
