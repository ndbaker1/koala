
export const KoalaCodeExampe = `
fn main() {
  dfs_demo() // traversal order as 03124
  println(fact(6)) // 720
  println(fib(6)) // 21
}

fn dfs_demo() {
  global graph[25] = [
    0,0,0,2,0,
    0,0,1,0,1,
    1,0,0,3,0,
    4,3,3,0,1,
    0,2,1,0,0
  ]
  global visited[5] = [0,0,0,0,0]
  dfs(0)
  println()
}

// DFS
fn dfs(row) {
  print(row)
  visited[row] = 1
  let goTo = 0
  while goTo < 5 {
    if (visited[goTo] == 0) && (graph[m2d(row, goTo, 5)] != 0) {
      dfs(goTo, row)
    }
    goTo = goTo + 1
  }
}

// grid lookup
fn m2d(row, col, width) {
  return (width * row) + col
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
// - when (expr) -> { ... } syntax


`