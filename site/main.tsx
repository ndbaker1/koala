import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './App'
import { themes } from './services/theme.service'
import { ChakraProvider } from '@chakra-ui/react'

themes[0].setCurrent()

ReactDOM.render(
  <React.StrictMode>
    <ChakraProvider>
      <App />
    </ChakraProvider>
  </React.StrictMode>,
  document.getElementById('root')
)
