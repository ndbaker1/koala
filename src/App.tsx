import Editor from '@monaco-editor/react'
import { languages } from 'monaco-editor'
import React, { useEffect, useState } from 'react'
import { complexGrammar, complexGrammarCode, Program } from './compiler/grammar'
import { createParser } from './compiler/Parser'
import { createCPU } from './CPU/CPU'
import { createMemory } from './CPU/Memory'

languages.typescript.typescriptDefaults.setDiagnosticsOptions({
  noSemanticValidation: true,
  noSyntaxValidation: true,
})

function App() {
  const [grammarCode, setGrammarCode] = useState(complexGrammar)
  const [parserCode, setParserCode] = useState(Program.toString())
  const [code, setCode] = useState(complexGrammarCode)
  const [assembly, setAssembly] = useState('')

  const [parser, setParser] = useState(createParser(grammarCode))

  function compile() {
    const AST = parser.parse(code)
    console.log(AST)
  }

  return (
    <div className="App">
      <Editor // "grammar-editor"
        height="500px"
        width='80%'
        theme="vs-dark"
        language="typescript"
        value={grammarCode}
        onChange={value => setGrammarCode(value || '')}
      />
      <Editor // "parser-editor"
        height="500px"
        width='80%'
        theme="vs-dark"
        language="typescript"
        value={parserCode}
        onChange={value => setParserCode(value || '')}
      />
      <Editor // "code-editor"
        height="500px"
        width='80%'
        theme="vs-dark"
        language="r"
        value={code}
        onChange={value => setCode(value || '')}
      />
      <div id="AST-viewer"></div>
      <Editor // "assembly-viewer"
        options={{ readOnly: true }}
        height="500px"
        width='80%'
        theme="vs-dark"
        value={assembly}
      />
      <button onClick={compile}>COMPILE</button>
      <div id="terminal"></div>
    </div>
  )
}

export default App