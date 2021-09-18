import { Token } from "./compiler/lexer"
import { createCPU } from "./CPU/CPU"

const cpu = createCPU()

cpu.run()


/*

fun main() {
  int number = 22
  string text = 'text in a message\n\n'
  int[4] list = [2, 3, 5, 6]
  string functionReturn = recursion(list[2])
  print(functionReturn)

  print(number)
  number = 33
  print(number)

  print(text)
  
}


fun recursion(val: int) {
  if (val > 0) {
    when (val) {
      0 -> { return "finished" }
      else -> { return recursion(val--) }
    }
  }
}

*/

const exampleLexed: Token[] = [
  {
    name: 'EQ',
    value: '='
  }
]