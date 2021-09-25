import Editor from '@monaco-editor/react'
import React, { useRef, useState } from 'react'
import { createParser } from '../src/compiler/Parser'
import { themes } from './services/theme.service'

import { bootstrapASTParser } from '../src/compiler/ASTParser'
import {
  ASTParserExample,
  CodeExample,
  GrammarExample,
} from './examples'
import { createMemory } from '../src/CPU/Memory'
import { createCPU } from '../src/CPU/CPU'

themes[0].setCurrent()

const ASTRootVariableName = 'ASTRoot'
const ASTParser = bootstrapASTParser(ASTRootVariableName)

function App() {
  const grammarCodeRef = useRef(GrammarExample)
  const [ASTParserCode, setASTParserCode] = useState(ASTParserExample)
  const [code, setCode] = useState(CodeExample)
  const [AST, setAST] = useState(undefined)
  const [assembly, setAssembly] = useState('')

  const [grammarParser, setGrammarParser] = useState(createParser(grammarCodeRef.current))

  const [output, setOutput] = useState('')

  return (
    <div style={{
      display: 'grid',
      gridGap: '1rem',
      placeContent: 'center',
    }}>

      <section id="grammar-editor">
        <Editor
          height='800px'
          width='900px'
          theme="vs-dark"
          language="javascript"
          value={grammarCodeRef.current}
          onChange={value => grammarCodeRef.current = (value || '')}
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

      <button onClick={() => {
        try {
          setAST(grammarParser.parse(code))
        } catch (e) {
          console.error(e)
        }
      }}>COMPILE CODE</button>

      <section id="AST-viewer">
        <Editor
          options={{ readOnly: true }}
          height='500px'
          width='900px'
          theme="vs-dark"
          language="json"
          value={JSON.stringify(AST, null, 4)}
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

      <button onClick={() => setAssembly(((Function(ASTParser(ASTParserCode)) as (a: any) => string)(AST)))}>COMPILE AST</button>

      <section id="assembly-viewer">
        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          height='500px'
          width='900px'
          value={assembly}
        />
      </section>

      <button onClick={() => {
        const memory = createMemory()
        memory.load(new Uint32Array(assembly.split('\n').map(Number)))
        const cpu = createCPU({ memory })
        setOutput(cpu.run())
      }}>RUN ASSEMBLY</button>

      <section id="terminal">
        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          height='500px'
          width='900px'
          value={output}
        />
      </section>
    </div>
  )
}

export default App
