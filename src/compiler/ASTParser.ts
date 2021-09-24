
export const bootstrapASTParser = (RootNode: string) => (parserCode: string) => `let ${RootNode} = arguments[0]\n${parserCode}`
