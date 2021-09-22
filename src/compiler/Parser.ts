import { generate } from 'peggy'


export type Parser = {
  parse: (code: string) => any
}

class ParserOptions {
  debug = false
}

/**
 * 
 * @param grammar 
 * @returns 
 */
export function createParser(grammar: string, { debug }: ParserOptions = new ParserOptions()): Parser {

  const parser = generate(grammar, { trace: debug })

  const parse = (code: string) => parser.parse(replaceEscapeCodes(code.trim()))

  return { parse }
}

function replaceEscapeCodes(str: string) {
  return str
    .replaceAll('\\n', '\n')
    .replaceAll('\\r', '\r')
    .replaceAll('\\t', '\t')
}