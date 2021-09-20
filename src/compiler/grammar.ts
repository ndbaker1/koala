import { Grammer } from "./Parser"

export const testGrammar: Grammer = [
  {
    name: 'Expression',
    rules: [
      {
        rule: ['Term', '+', 'Term'],
        params: [0, 3],
        function: (op1: Term, op2: Term): Expression => ({ value: op1.value + op2.value })
      }
    ]
  },
  {
    name: 'Term',
    rules: [
      {
        rule: ['(', 'Expression', ')'],
        params: [1],
        function: (expr: Expression): Term => ({ value: expr.value })
      },
      {
        rule: ['Integer'],
        params: [0],
        function: (int: Integer): Term => ({ value: int })
      },
    ]
  },
  {
    name: 'Integer',
    rules: [
      {
        rule: [/[0-9]+/],
        params: [0],
        function: (numString: string): Integer => +numString
      }
    ]
  },
  {
    name: '+',
    rules: [
      {
        rule: [/+/],
        function: () => '+'
      }
    ]
  },
  {
    name: ')',
    rules: [
      {
        rule: [/\)/],
        function: () => ')'
      }
    ]
  },
  {
    name: '(',
    rules: [
      {
        rule: [/\(/],
        function: () => '('
      }
    ]
  },
]

// supporting objects
interface ProductionClass<T> { value: T }
type Term = ProductionClass<number>
type Expression = ProductionClass<number>
type Integer = number


const arithmeticGrammar = `
Expression
  = head:Term _ "+" _ tail:Term { return tail + head; }
Term
  = "(" _ expr:Expression _ ")" { return expr; }
  / Integer
Integer "integer"
  = _ [0-9]+ { return parseInt(text(), 10); }
_ "whitespace"
  = [ \t\n\r]*
`
