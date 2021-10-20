import Editor from '@monaco-editor/react'
import React, { useRef, useState } from 'react'
import { createParser } from '../src/compiler/Parser'

import { bootstrapASTParser } from './services/ASTParser'
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
  TERMINAL = 2,
}

import init, { run } from 'virtual-machine'
init().then(_ => run(new Uint32Array([25 << 24, 0]), (str: string) => alert(str)))

function App() {

  const grammarCodeRef = useRef(GrammarExample)
  const ASTParserCodeRef = useRef(ASTParserExample)
  const codeRef = useRef(CodeExample)
  const [AST, setAST] = useState(undefined)
  const [assembly, setAssembly] = useState('')

  const grammarParserRef = useRef(createParser(grammarCodeRef.current))

  const [output, setOutput] = useState('')

  const [view, setView] = useState(View.GRAMMAR)

  function CurrentView() {
    switch (view) {
      case View.GRAMMAR:
        return <Parsers />
      case View.CODE:
        return <CodeEditor />
      case View.TERMINAL:
        return <Run />
    }
  }

  function Parsers() {
    return (
      <section id="grammar-editor" style={{ display: 'flex', flexDirection: 'row', height: '100%' }}>
        <Editor
          width='900px'
          theme="vs-dark"
          language="javascript"
          value={grammarCodeRef.current}
          onChange={value => grammarCodeRef.current = (value || '')}
        />
        <Editor
          width='900px'
          theme="vs-dark"
          language="javascript"
          value={ASTParserCodeRef.current}
          onChange={value => ASTParserCodeRef.current = (value || '')}
        />
        <button onClick={() => setView(View.CODE)}>Write Code</button>
      </section>
    )
  }

  function CodeEditor() {
    return (
      <section id="code-editor" style={{ display: 'flex', flexDirection: 'row', height: '100%' }}>
        <Editor
          width='900px'
          theme="vs-dark"
          language="r"
          value={codeRef.current}
          onChange={value => codeRef.current = (value || '')}
        />

        <button onClick={() => {
          try {
            setAST(grammarParserRef.current.parse(codeRef.current))
          } catch (e) {
            console.error(e)
          }
        }}>Compile</button>

        <button onClick={() => {
          setAssembly(((Function(ASTParser(ASTParserCodeRef.current)) as (a: any) => string)(AST)))
          setView(View.TERMINAL)
        }}>COMPILE AST</button>

        <section id="AST-viewer">
          <Editor
            options={{ readOnly: true }}
            width='900px'
            theme="vs-dark"
            language="json"
            value={JSON.stringify(AST, null, 4)}
          />
        </section>
      </section>
    )
  }

  function Run() {
    return (
      <section id="execute" style={{ display: 'flex', flexDirection: 'row', height: '100%' }}>
        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          width='900px'
          value={assembly}
        />

        <button onClick={() => {
          const memory = createMemory()
          memory.load(new Uint32Array(assembly.split('\n').map(Number)))
          const cpu = createCPU({ memory, io: { output: (a) => setOutput(b => b + a) } })
          cpu.run()
        }}>RUN ASSEMBLY</button>

        <Editor
          options={{ readOnly: true }}
          theme="vs-dark"
          width='900px'
          value={output}
        />
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
    <div style={{ height: '100vh', padding: '1rem' }}>
      <div style={{ display: 'grid', gridGap: '1rem', gridTemplateRows: 'auto auto', height: '100%' }}>
        <div>
          <Tabs />
        </div>
        <div style={{ height: '100%' }}>
          <CurrentView />
        </div>
      </div>
    </div>
  )
}

export default App


