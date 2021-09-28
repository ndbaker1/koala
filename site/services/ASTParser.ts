export const bootstrapASTParser = (RootNode: string) => (parserCode: string) => `let ${RootNode} = arguments[0]\n${parserCode}`

export const objectToVarDecls = (a: any) => Object.keys(a).reduce((acc, cur) => acc + `\t\"${cur}\": ${a[cur]},\n`, 'const codeGens = {\n') + '}'
