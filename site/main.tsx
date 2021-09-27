import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './App'
import { themes } from './services/theme.service'

themes[0].setCurrent()

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
)
