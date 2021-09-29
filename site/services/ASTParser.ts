export const bootstrapASTParser = (RootNode: string) => (parserCode: string) => `let ${RootNode} = arguments[0]\n${parserCode}`

export const objectToVarDecls = (a: any) => Object.values(a).reduce((acc, cur: any) => acc + `\t\"${cur.name}\": ${cur.function},\n`, 'const CodeGens = {\n') + '}'
