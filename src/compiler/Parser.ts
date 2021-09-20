import { generate } from 'peggy'


export type Parser = {
  parse: (code: string) => any
}

export function createParser(grammar: string): Parser {

  const parser = generate(grammar)

  const parse = (code: string) => parser.parse(code)

  return { parse }
}
