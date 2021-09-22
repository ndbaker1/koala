import Editor from '@monaco-editor/react'
import React, { useState } from 'react'
import { createParser } from '../src/compiler/Parser'
import { themes } from './services/theme.service'

import {
  ASTParserExample,
  CodeExample,
  GrammarExample,
} from './examples'

themes[0].setCurrent()

const ASTRootVariableName = 'ASTRoot'
const bootstrapASTParser = (parserCode: string) => `let ${ASTRootVariableName} = arguments[0]\n${parserCode}`

function App() {
  const [grammarCode, setGrammarCode] = useState(GrammarExample)
  const [ASTParserCode, setASTParserCode] = useState(ASTParserExample)
  const [code, setCode] = useState(CodeExample)
  const [AST, setAST] = useState(undefined)
  const [assembly, setAssembly] = useState('')

  const [grammarParser, setGrammarParser] = useState(createParser(grammarCode))

  const [ASTParser, setASTParser] = useState(bootstrapASTParser(ASTParserCode))

  return (
    <div style={{
      display: 'grid',
      gridGap: '1rem',
      placeContent: 'center',
    }}>

      <section id="grammar-editor">
        <Editor
          height='500px'
          width='900px'
          theme="vs-dark"
          language="javascript"
          value={grammarCode}
          onChange={value => setGrammarCode(value || '')}
        />
      </section>

      <section id="code-editor">
        <Editor
          height='500px'
          width='900px'
          theme="vs-dark"
          language="r"
          value={code}
          onChange={value => setCode(value || '')}
        />
      </section>

      <button onClick={() => setAST(grammarParser.parse(code))}>COMPILE</button>

      <section id="AST-viewer">
        <Editor
          options={{ readOnly: true }}
          height='500px'
          width='900px'
          theme="vs-dark"
          language="json"
          value={JSON.stringify(AST, null, 2)}
        />
      </section>

      <section id="parser-editor">
        <Editor
          height='500px'
          width='900px'
          theme="vs-dark"
          language="javascript"
          value={ASTParserCode}
          onChange={value => setASTParserCode(value || '')}
        />
      </section>

      <section id="assembly-viewer">
        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          height='500px'
          width='900px'
          value={assembly}
        />
      </section>

      <div id="terminal"></div>
    </div>
  )
}

export default App
