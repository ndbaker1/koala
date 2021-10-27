
export const KoalaCodeExampe = `
fn main() {
  print(fact(6))
  print(fib(6))
}

// Fibonacci 
fn fib(n) {
  if (n > 0) {
    return (fib((n-1)) + fib((n-2)))
  }
  return 1
}

// Factorial
fn fact(n) {
  if (n != 0) {
    return (fact((n-1)) * n)
  }
  return 1
}

// WIP
// - global variables
// - when (expr) -> { ... } syntax
// - arrays ?


`