import React from 'react'

import Editor from '@monaco-editor/react'
import ReactJson from 'react-json-view'
import { VscRepo, VscTerminal } from 'react-icons/vsc'

import { Box, Center, Grid, HStack, Text } from '@chakra-ui/layout'
import { Button } from '@chakra-ui/button'

import init, { run, sourceCodeGen, parseAst } from 'koala'
import { KoalaCodeExampe } from './examples'

const repo = "https://github.com/ndbaker1/koala"

export default function App() {

  React.useEffect(() => { init() }, [])

  const codeRef = React.useRef(KoalaCodeExampe)
  const [ast, setAst] = React.useState("")
  const vmCodeRef = React.useRef(new Uint32Array)
  const [output, setOutput] = React.useState("")

  const outputCallback = (str: string) => setOutput(cur => cur + str)

  return (
    <div className="dosis-font">
      <Center padding="2rem" height="100vh">
        <Box borderRadius="xl" bg="white" width="container.lg" padding="2rem" shadow="2xl">
          <Grid gap="10" templateColumns="repeat(2, 1fr)">
            <Box fontSize="5xl" fontWeight="bold">
              Koala.
              <Text color="gray.500">
                ʕ •ᴥ•ʔ
              </Text>
            </Box>
            <Grid gap="10" templateRows="repeat(2, 1fr)">
              <Text
                fontSize="lg"
                textAlign="justify">
                A Simple Programming Language that runs on a Stack-based Virtual Machine all written in Rust 🦀.
              </Text>
              <Center>
                <HStack>
                  <Button as="a"
                    href={repo}
                    target="_blank"
                    leftIcon={<VscRepo />}>
                    Source Code
                  </Button>
                  <Button
                    leftIcon={<VscTerminal />}
                    onClick={() => {
                      document.querySelector('#editor')?.scrollIntoView({ behavior: 'smooth' })
                    }}>
                    Try It Out
                  </Button>
                </HStack>
              </Center>
            </Grid>
          </Grid>
        </Box>
      </Center>


      <Center id="editor" padding="2rem" height="100vh">
        <Box borderRadius="xl" bg="white" width="container.lg" padding="1rem" shadow="2xl">
          <Box paddingBottom="1rem">
            <Text fontSize="xl">
              Write Koala
            </Text>
            <Editor
              height={500}
              language="python"
              value={codeRef.current}
              onChange={data => { codeRef.current = data || '' }}
              options={{ fontFamily: '"Consolas"' }} />
          </Box>
          <Grid >
            <Button onClick={() => {
              try {
                setAst(parseAst(codeRef.current))
                vmCodeRef.current = sourceCodeGen(codeRef.current)
                run(vmCodeRef.current, setOutput)
                document.querySelector('#output')?.scrollIntoView({ behavior: 'smooth' })
              } catch (e) {
                alert('encountered a not yet handled grammar error!')
              }
            }}>Run</Button>
          </Grid>
        </Box>
      </Center >

      {/* Output Panel */}
      <Center id="output" padding="2rem" height="100vh">
        <Box borderRadius="md" bg="white" width="container.lg" padding="1rem" shadow="2xl" >
          <Grid gridTemplateRows="repeat(2,1fr)" height={500}>
            <Box>
              <Text textAlign="left" fontSize="xl">
                Syntax Tree
              </Text>
              <hr />
              <Box textAlign="left" overflow="auto" overscrollBehavior="contain" height={350}>
                <ReactJson src={JSON.parse(ast || '{}')} />
              </Box>
              <hr />
            </Box>
            <Box display="flex" alignItems="end">
              <Text textAlign="left" fontSize="xl">
                {!!output
                  ? <>Koala ran your code:<br />&nbsp; &nbsp; &nbsp; &nbsp; &nbsp;💬&nbsp; {output}<br /></>
                  : ''
                }
                ʕ •ᴥ•ʔ
              </Text>
            </Box>
          </Grid>
        </Box>
      </Center>
    </div>
  )
}