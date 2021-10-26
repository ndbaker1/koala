
export const KoalaCodeExampe = `
fn main() {
  print(fact(5))
  print(fib(5))
}

fn fib(n) {
  if n {
    if (n-1) {
      return (fib((n-1)) + fib((n-2)))
    }
  }
  return 1
}

// Factorial
fn fact(n) {
  if n {
    return (fact((n-1)) * n)
  }
  return 1
}

// WIP
// - local & global variables
// - logical binary operators
// - when (expr) -> { ... } syntax
// - arrays ?

`