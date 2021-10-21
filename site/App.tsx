import Editor from '@monaco-editor/react'
// @ts-ignore
import ReactFullpage from '@fullpage/react-fullpage'

import init, { run, sourceCodeGen, parseAst } from 'koala'
import React, { Dispatch, SetStateAction } from 'react'

const repo = "https://github.com/ndbaker1/koala"

function App() {

  React.useEffect(() => { init() }, [])

  const codeRef = React.useRef("")
  const [ast, setAst] = React.useState("")
  const vmCodeRef = React.useRef(new Uint32Array)

  const environment = (): Environment => ({ codeRef, ast: [ast, setAst], vmCodeRef })

  return (
    <div className="max-w-screen-lg h-screen m-auto p-5 flex items-center justify-center">
      <ReactFullpage
        render={() => {
          return (
            <ReactFullpage.Wrapper>
              <div className="section">
                <Intro />
              </div>
              <div className="section">
                <Playground env={environment()} />
              </div>
              <div className="section">
                <SyntaxTree env={environment()} />
              </div>
            </ReactFullpage.Wrapper>
          )
        }}
      />
    </div>
  )
}

export default App

type Environment = {
  codeRef: React.MutableRefObject<string>,
  ast: [string, Dispatch<SetStateAction<string>>],
  vmCodeRef: React.MutableRefObject<Uint32Array>
}


function Intro() {
  return (
    <div className="w-full bg-white rounded-lg shadow-lg p-10">
      <div className="grid gap-10 md:grid-cols-2 sm:grid-cols-1">
        <div>
          <p className="text-5xl">
            Koala.
            <br />
            <p className="text-gray-400"><b>ʕ •ᴥ•ʔ</b></p>
          </p>
        </div>
        <div className="grid grid-rows-2">
          <p className="text-xl text-justify">
            A Simple Programming Language that runs on a Stack-based Virtual Machine all written in Rust 🦀.
          </p>
          <div className="flex items-center justify-center">
            <a className="bg-gray-600 text-gray-100 rounded-lg p-3" href={repo} target="_blank" rel="noopener noreferrer">Github</a>
          </div>
        </div>
      </div>
    </div>
  )
}

function Playground({ env }: { env: Environment }) {
  return (
    <div className="max-w-screen-lg h-screen m-auto p-5 flex items-center justify-center">
      <div className="w-full bg-white rounded-lg p-5">
        <Editor
          height={500}
          value={env.codeRef.current}
          onChange={data => { env.codeRef.current = data || '' }} />
        <div className="grid grid-cols-2">
          <button onClick={() => { env.ast[1](parseAst(env.codeRef.current)) }}>Compile</button>
          <button >Run</button>
        </div>
      </div>
    </div>
  )
}


function SyntaxTree({ env }: { env: Environment }) {
  return (
    <div className="max-w-screen-lg h-screen m-auto p-5 flex items-center justify-center">
      <div className="w-full bg-white rounded-lg p-5">
        <Editor
          height={500}
          value={env.ast[0]}
          options={{ readOnly: true }} />
      </div>
    </div>
  )
}

function Output({ env }: { env: Environment }) {
  return (
    <div className="max-w-screen-lg h-screen m-auto p-5 flex items-center justify-center">
      <div className="w-full bg-gray-500 rounded-lg p-5">
        <Editor
          height={500}
          value={env.ast[0]}
          options={{ readOnly: true }} />
      </div>
    </div>
  )
}