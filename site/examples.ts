
export const KoalaCodeExampe = `print(2+5)`

export const CodeExample = `fun main() {
  number = 22
  text = 'text in a message\\n\\n'
  functionReturn = recursion(number)
  print(functionReturn)

  print(number)
  number = 33
  print(number)

  print(text)
}

when (true) {
  true -> 2
  false -> 3
}

c = 'string'
print(c)

fun foo1() {
  a = 5
  a = a - 2
  if (a < 2) {
    print('less than 2')
  } else {
    print('2 or greater')
  }

  when (a) {
    1 -> -1
    2 -> 2
    3 -> -3
    else -> 99
  }
}

x = 'x'

fun foo() {
  x = 23
  print(x)
}

print(2)

fun recursion(val) {
  if (val > 0) {
    when (val) {
      0 -> 'function'
      2 -> return 5
    }
  } else {
    c = 'string'
  }
}`
