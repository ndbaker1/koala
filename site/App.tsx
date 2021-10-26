import React from 'react'

import Editor from '@monaco-editor/react'
import ReactJson from 'react-json-view'

import { VscDebugStepBack, VscRepo, VscTerminal } from 'react-icons/vsc'

import { Box, Center, Container, Grid, HStack, Text } from '@chakra-ui/layout'
import { Button } from '@chakra-ui/button'

import init, { run, sourceCodeGen, parseAst } from 'koala'
import { KoalaCodeExampe } from './examples'
import { Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/tabs'
import { Textarea } from '@chakra-ui/textarea'

const repo = "https://github.com/ndbaker1/koala"

const windowHeight = 28
const minScreenHeight = 40

function toRem(value: number) { return value + "rem" }

enum KoalaState {
  open = "ʕ •ᴥ•ʔ",
  closed = "ʕ -ᴥ-ʔ",
}

export default function App() {

  React.useEffect(() => {
    // initialize Koala WASM
    init()
    // Setup out blinking Koala Timer
    startBlinker()
  }, [])

  const codeRef = React.useRef(KoalaCodeExampe)
  const [ast, setAst] = React.useState("")
  const vmCodeRef = React.useRef(new Uint32Array)
  const [output, setOutput] = React.useState("")

  const outputCallback = (str: string) => setOutput(cur => cur + str)

  const [mainKoala, setMainKoala] = React.useState(KoalaState.open)

  const startBlinker = () => {
    const min = 2
    const max = 7
    let blinker: NodeJS.Timeout
    (function blink() {
      setMainKoala(state => {
        if (state == KoalaState.open) {
          clearInterval(blinker)
          blinker = setTimeout(blink, 200)
          return KoalaState.closed
        } else {
          clearInterval(blinker)
          blinker = setTimeout(blink, (Math.random() * (max - min) + min) * 1000)
          return KoalaState.open
        }
      })
    })()
  }

  return (
    <div className="dosis-font">
      {/* Intro Component */}
      <Center
        paddingX="2rem"
        height={`max(100vh,${toRem(minScreenHeight)})`}
      >
        <Container
          borderRadius="xl"
          bg="white"
          maxWidth="container.lg"
          padding="2rem"
          shadow="2xl"
        >
          <Grid
            gap="5"
            templateColumns={{ md: "1fr 3fr" }}
          >
            <Center
              fontSize="5xl"
              fontWeight="bold"
            >
              <Grid gridRow="2">
                <Text>Koala.</Text>
                <Text color="gray.500">{mainKoala}</Text>
              </Grid>
            </Center>
            <Grid
              gap="5"
              templateRows="auto 1fr"
            >
              <Box paddingX="2">
                <Center>
                  <Text
                    fontSize="md"
                    textAlign="left"
                  >
                    🐨 Koala is a simple programming language that runs on a stack-based virtual machine.
                    <br />
                    It was created as a proof-of-concept while studying compiler design and hardware virtualization.
                    <br />
                    <br />
                    Because the entire library is written using Rust 🦀,
                    the compiler and runtime can be ported here using WebAssembly!
                  </Text>
                </Center>
              </Box>
              <Center>
                <HStack>
                  <Button
                    as="a"
                    href={repo}
                    target="_blank"
                    leftIcon={<VscRepo />}>
                    More on Github
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
      <Center
        id="editor"
        padding="2rem"
        height={`max(100vh,${toRem(minScreenHeight)})`}
      >
        <Container
          borderRadius="xl"
          bg="white"
          maxW="container.lg"
          padding="1rem"
          shadow="2xl"
        >

          <Tabs>
            <TabList>
              <Tab>Write Code</Tab>
              <Tab>Language Docs</Tab>
            </TabList>
            <TabPanels>
              <TabPanel>
                <Box
                  paddingBottom="1rem"
                >
                  <Editor
                    height={toRem(windowHeight)}
                    language="python"
                    value={codeRef.current}
                    onChange={data => { codeRef.current = data || '' }}
                    options={{ fontFamily: '"Consolas"' }}
                  />
                </Box>
                <Grid>
                  <Button onClick={() => {
                    let stage
                    try {
                      stage = 'parsing'
                      setAst(parseAst(codeRef.current))
                      stage = 'code generation'
                      vmCodeRef.current = sourceCodeGen(codeRef.current)
                      setOutput('')
                      stage = 'execution'
                      run(vmCodeRef.current, outputCallback)
                      smoothScrollTo('#output')
                    } catch (e) {
                      alert('encountered error in ' + stage + ' stage:\n' + e)
                    }
                  }}>Compile & Run</Button>
                </Grid>
              </TabPanel>
              <TabPanel>
                <Box
                  paddingBottom="1rem"
                  height={toRem(windowHeight)}
                >

                  Docs
                </Box>
              </TabPanel>
            </TabPanels>
          </Tabs>
        </Container>
      </Center >

      {/* Output Panel */}
      <Center
        id="output"
        padding="2rem"
        height={`max(100vh,${toRem(minScreenHeight)})`}
      >
        <Container
          borderRadius="md"
          bg="white"
          maxWidth="container.lg"
          padding="1rem"
          shadow="2xl"
        >
          <Tabs>
            <TabList>
              <Tab>Info</Tab>
              <Tab>Syntax Tree</Tab>
              <Tab>Instructions (Hex)</Tab>
            </TabList>
            <TabPanels>
              <TabPanel>
                <Box
                  textAlign="left"
                  overflow="auto"
                  overscrollBehavior="contain"
                  height={toRem(windowHeight)}
                >
                  <Box
                    fontSize="md"
                    maxWidth="xl"
                  >
                    {!!output
                      ? <>
                        One of the local Koalas has generously compiled and run our code!<br />
                        <br />
                        <Grid gap="2" gridTemplateColumns="min-content auto">
                          <Box textAlign="left" fontSize="xl" width="min-content">
                            <Text whiteSpace="nowrap">&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp;💬</Text>
                            <Text color="gray.500" fontSize="2xl" whiteSpace="nowrap">{KoalaState.open}</Text>
                          </Box>
                          <Textarea value={output} />
                        </Grid>
                        <br />
                        This is the output of our program,
                        but you can also view the <b>Syntax Tree</b> generated by the Language parser,
                        and the corresponding 32bit <b>Instructions</b> (in hex) that are run by the Koala Virtual Machine.
                      </>
                      : <>
                        Compile a Program!
                        <Text color="gray.500" fontSize="2xl" whiteSpace="nowrap">{KoalaState.open}</Text>
                      </>
                    }
                  </Box>
                </Box>
              </TabPanel>
              <TabPanel>
                <Box
                  textAlign="left"
                  overflow="auto"
                  overscrollBehavior="contain"
                  height={toRem(windowHeight)}
                >
                  {/* backup base json stringify */}
                  {/* <pre style={{ fontFamily: 'monospace' }}>{JSON.stringify(JSON.parse(ast || '{}'), null, 2)}</pre> */}
                  <ReactJson
                    src={JSON.parse(ast || '{}')}
                    indentWidth={2}
                    enableClipboard={false}
                    displayDataTypes={false}
                    quotesOnKeys={false}
                  />
                </Box>
              </TabPanel>
              <TabPanel>
                <Textarea
                  overflow="auto"
                  overscrollBehavior="contain"
                  height={toRem(windowHeight)}
                  fontFamily="monospace"
                  value={vmCodeRef.current
                    .reduce<string[]>((cur, val) => cur.concat(['0x' + val.toString(16).padStart(8, '0').toUpperCase()]), [])
                    .map((inst, index) => index.toString().padStart(4, ' ') + '│' + inst)
                    .join("\n")
                  }
                />
              </TabPanel>
            </TabPanels>
          </Tabs>
          <Grid>
            <Button
              leftIcon={<VscDebugStepBack />}
              onClick={() => {
                smoothScrollTo('#editor')
              }}>
              Go Back and Edit
            </Button>
          </Grid>
        </Container>
      </Center>
    </div >
  )
}

function smoothScrollTo(id: string) {
  document.querySelector(id)?.scrollIntoView({ behavior: 'smooth' })
}
