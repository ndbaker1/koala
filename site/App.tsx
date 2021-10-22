import React from 'react'

import Editor from '@monaco-editor/react'
import ReactJson from 'react-json-view'
import { VscRepo, VscTerminal } from 'react-icons/vsc'

import { Box, Center, Container, Divider, Grid, HStack, Stack, Text } from '@chakra-ui/layout'
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
      {/* Intro Component */}
      <Center paddingX="2rem" height="100vh">
        <Container borderRadius="xl" bg="white" maxW="container.lg" padding="2rem" shadow="2xl">
          <Grid gap="10" templateColumns={{ md: "repeat(2, 1fr)" }}>
            <Container fontSize="5xl" fontWeight="bold">
              Koala.
              <Text color="gray.500">
                ʕ •ᴥ•ʔ
              </Text>
            </Container>
            <Grid gap="10" templateRows="repeat(2, 1fr)">
              <Box paddingX="5">
                <Center>
                  <Text
                    fontSize="lg"
                    textAlign="justify"
                    maxWidth="sm">
                    A Simple Programming Language that runs on a Stack-based Virtual Machine all written in Rust 🦀.
                  </Text>
                </Center>
              </Box>
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
        </Container>
      </Center>


      <Center id="editor" padding="2rem" height="100vh">
        <Container borderRadius="xl" bg="white" maxW="container.lg" padding="1rem" shadow="2xl">
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
                setOutput('')
                run(vmCodeRef.current, outputCallback)
                document.querySelector('#output')?.scrollIntoView({ behavior: 'smooth' })
              } catch (e) {
                alert('encountered a not yet handled grammar error!')
              }
            }}>Run</Button>
          </Grid>
        </Container>
      </Center >

      {/* Output Panel */}
      <Center id="output" padding="2rem" height="100vh">
        <Container borderRadius="md" bg="white" maxWidth="container.lg" padding="1rem" shadow="2xl" >
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
        </Container>
      </Center>
    </div>
  )
}