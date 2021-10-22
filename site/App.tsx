import React from 'react'

import Editor from '@monaco-editor/react'
import ReactJson from 'react-json-view'
import { VscDebugStepBack, VscRepo, VscTerminal } from 'react-icons/vsc'

import { Box, Center, Container, Grid, HStack, Text } from '@chakra-ui/layout'
import { Button } from '@chakra-ui/button'

import init, { run, sourceCodeGen, parseAst } from 'koala'
import { KoalaCodeExampe } from './examples'
import { Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/tabs'

const repo = "https://github.com/ndbaker1/koala"

const windowHeightRem = "32rem"

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
          <Grid gap="5" templateColumns={{ md: "repeat(2, 1fr)" }}>
            <Center fontSize="5xl" fontWeight="bold">
              <Grid gridRow="2">
                <Text>Koala.</Text>
                <Text color="gray.500">ʕ •ᴥ•ʔ</Text>
              </Grid>
            </Center>
            <Grid gap="5" templateRows="auto 1fr">
              <Box paddingX="5">
                <Center>
                  <Text
                    fontSize="md"
                    textAlign="left"
                    maxWidth="sm">
                    Koala is a simple programming language that runs on a stack-based virtual machine
                    created for educational and demonastrative purposes.
                    <br /><br />
                    This project was inspired by cross-platform languages such as Java and C#,
                    which are facilitated through VMs the Java Virtual Machine (JVM) and .NET Core.
                    <br /><br />
                    The library is written completely in Rust 🦀, so has been compiled to WebAssembly for this demo!
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
                      smoothScrollTo('#editor')
                    }}>
                    Try It Out
                  </Button>
                </HStack>
              </Center>
            </Grid>
          </Grid>
        </Container>
      </Center>

      {/* Code Editor */}
      <Center id="editor" padding="2rem" height="100vh">
        <Container borderRadius="xl" bg="white" maxW="container.lg" padding="1rem" shadow="2xl">
          <Box paddingBottom="1rem">
            <Text fontSize="xl">
              Write Koala
            </Text>
            <Editor
              height={windowHeightRem}
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
                smoothScrollTo('#output')
              } catch (e) {
                alert('encountered a not yet handled grammar error!')
              }
            }}>Compile & Run</Button>
          </Grid>
        </Container>
      </Center >

      {/* Output Panel */}
      <Center id="output" padding="2rem" height="100vh">
        <Container borderRadius="md" bg="white" maxWidth="container.lg" padding="1rem" shadow="2xl" >
          <Grid gridTemplateRows="repeat(2,1fr)" height={windowHeightRem}>
            <Tabs>
              <TabList>
                <Tab>Info</Tab>
                <Tab>Syntax Tree</Tab>
                <Tab>Instructions (Hex)</Tab>
              </TabList>
              <TabPanels>
                <TabPanel>
                  <Box textAlign="left" overflow="auto" overscrollBehavior="contain" height={350}>
                    <Text fontSize="md" maxWidth="xl">
                      One of the local Koalas has generously compiled and run our code!<br />
                      <br />
                      <Box textAlign="left" fontSize="xl">
                        {!!output
                          ? <>&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;💬&nbsp; " {output} "<br /></>
                          : ''
                        }
                        <Text color="gray.500" fontSize="2xl">
                          ʕ •ᴥ•ʔ
                        </Text>
                      </Box>
                      <br />
                      This is the output of our program,
                      but you can also view the <b>Syntax Tree</b> generated by the Language parser,
                      and the corresponding 32bit <b>Instructions</b> (in hex) that are run by the Koala Virtual Machine.
                    </Text>
                  </Box>
                </TabPanel>
                <TabPanel>
                  <Box textAlign="left" overflow="auto" overscrollBehavior="contain" height={350}>
                    <ReactJson src={JSON.parse(ast || '{}')} />
                  </Box>
                </TabPanel>
                <TabPanel>
                  <Box textAlign="left" overflow="auto" overscrollBehavior="contain" height={350} fontFamily="consolas">
                    {vmCodeRef.current
                      .reduce<string[]>((cur, val) => cur.concat([
                        '0x' + val.toString(16).padStart(8, '0').toUpperCase()
                      ]), [])
                      .map<JSX.Element>(hex => <p>{hex}</p>)}
                  </Box>
                </TabPanel>
              </TabPanels>
            </Tabs>
            <hr />
            <Box display="flex" alignItems="end">
              <Box display="flex" flexDirection="column">

                <Button
                  maxWidth="min"
                  leftIcon={<VscDebugStepBack />}
                  onClick={() => {
                    smoothScrollTo('#editor')
                  }}>
                  Go Back and Edit
                </Button>
              </Box>
            </Box>
          </Grid>
        </Container>
      </Center>
    </div >
  )
}

function smoothScrollTo(id: string) {
  document.querySelector(id)?.scrollIntoView({ behavior: 'smooth' })
}