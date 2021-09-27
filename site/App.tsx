import Editor from '@monaco-editor/react'
import React, { useRef, useState } from 'react'
import { createParser } from '../src/compiler/Parser'

import { bootstrapASTParser } from '../src/compiler/ASTParser'
import {
  ASTParserExample,
  CodeExample,
  GrammarExample,
} from './examples'
import { createMemory } from '../src/CPU/Memory'
import { createCPU } from '../src/CPU/CPU'

const ASTRootVariableName = 'ASTRoot'
const ASTParser = bootstrapASTParser(ASTRootVariableName)

enum View {
  GRAMMAR = 0,
  CODE = 1,
  AST = 2,
  TERMINAL = 3,
}

function App() {
  const grammarCodeRef = useRef(GrammarExample)
  const [ASTParserCode, setASTParserCode] = useState(ASTParserExample)
  const [code, setCode] = useState(CodeExample)
  const [AST, setAST] = useState(undefined)
  const [assembly, setAssembly] = useState('')

  const [grammarParser, setGrammarParser] = useState(createParser(grammarCodeRef.current))

  const [output, setOutput] = useState('')

  const [view, setView] = useState(View.GRAMMAR)

  function CurrentView() {
    switch (view) {
      case View.GRAMMAR:
        return <GrammarEditor />
      case View.AST:
        return <ASTEditor />
      case View.CODE:
        return <CodeEditor />
      case View.TERMINAL:
        return <Run />
    }
  }

  function GrammarEditor() {
    return (
      <section id="grammar-editor">
        <Editor
          height='60vh'
          width='900px'
          theme="vs-dark"
          language="javascript"
          value={grammarCodeRef.current}
          onChange={value => grammarCodeRef.current = (value || '')}
        />
        <button onClick={() => setView(View.CODE)}>Write Code</button>
      </section >
    )
  }

  function CodeEditor() {
    return (
      <section id="code-editor">
        <Editor
          height='500px'
          width='900px'
          theme="vs-dark"
          language="r"
          value={code}
          onChange={value => setCode(value || '')}
        />
        <button onClick={() => {
          try {
            setAST(grammarParser.parse(code))
            setView(View.AST)
          } catch (e) {
            console.error(e)
          }
        }}>Compile</button>
      </section>
    )
  }

  function ASTEditor() {
    return (
      <section id="AST">
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

      </section>
    )
  }

  function Run() {
    return (
      <section id="terminal">

        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          height='500px'
          width='900px'
          value={assembly}
        />
        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          height='500px'
          width='900px'
          value={output}
        />
        <button onClick={() => {
          const memory = createMemory()
          memory.load(new Uint32Array(assembly.split('\n').map(Number)))
          const cpu = createCPU({ memory })
          setOutput(cpu.run())
        }}>RUN ASSEMBLY</button>
      </section>

    )
  }

  function Tabs() {
    const entries = Object.keys(View)
    return <div style={{ display: 'flex' }}>
      {entries
        .splice(0, entries.length / 2)
        .map(mode => <button onClick={() => setView(+mode)}>{View[+mode]}</button>)}
    </div>
  }

  return (
    <div style={{ display: 'grid', placeContent: 'center' }}>
      <Tabs />
      <CurrentView />
    </div>
  )
}

export default App


