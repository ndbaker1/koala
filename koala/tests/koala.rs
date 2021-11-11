use std::{cell::RefCell, rc::Rc};

use koala::{
    compiler::{CodeGen, CompilerContext},
    kvm::VirtualMachine,
    parser::parse_code,
};

macro_rules! code_tests {
  ($($name:ident: $value:expr,)*) => {$(
      #[test]
      fn $name() {
        let (code, expected) = $value;
        assert_eq!(kvm_run_code(&code), expected);
      }
  )*}
}

fn kvm_run_code(code: &str) -> String {
    let value = Rc::new(RefCell::new(String::new()));
    let captured_value = value.clone();

    let print_callback = &move |msg: &str| *captured_value.borrow_mut() += msg;

    let program = parse_code(&code).unwrap();
    let bin = program.code_gen(&mut CompilerContext::new(), 0);
    let mut kvm = VirtualMachine::new(print_callback, &|msg: &str| println!("{}", msg));
    kvm.run(&bin);

    return value.take();
}

code_tests! {
  empty_main_test: ("fn main() {}", ""),
  print_test: ("
  fn main() {
    print(2)
  }
  ",
  "2"),
  comment_test: ("
  fn main() {
    // print(2)
  }
  ",
  ""),
  variable_test: ("
  fn main() {
    let x = 2
    print(x)
  }
  ",
  "2"),
  nested_ifs_test: ("
  fn main() {
    if 1 { if 0 {} }
    if 0 { if 1 {} }
    if 1 { if 1 { print(5) } }
    if 0 { if 0 {} }
  }
  ",
  "5"),
  fib_test: ("
  fn main() {
    print(fib(4))
  }
  
  fn fib(n) {
    if n {
      if n-1 {
        return fib(n-1) + fib(n-2)
      }
    }
    return 1
  }
  ",
  "5"),
  multple_call_test_fp: ("
  fn main() {
    t(5)
  }
  
  fn t(a) {
    a(a-1)
    a(a-2)
  }

  fn a(a){
    print(a)
  }
  ",
  "43"),
  return_value_test: ("
  fn main() {
    print(f())
  }

  fn f() {
    return 3
  }
  ",
  "3"),
  comparisons_test: ("
  fn main() {
    if 1 < 2 { print(1) }
    if 2 > 1 { print(2) }
    if 1 == 1 { print(3) }
    if 1 != 2 { print(4) }
    if 1 <= 1 { print(5) }
    if 1 >= 1 { print(6) }
  }
  ",
  "123456"),
  logical_test: ("
  fn main() {
    if 1 || 2 { print(1) }
    if 2 && 1 { print(2) }
    if 1 || 0 { print(3) }
    if 1 && 0 { print(4) }
    if 0 && 0 { print(5) }
    if 0 || 0 { print(6) }
  }
  ",
  "123"),
  nested_math1_test: ("
  fn main() {
    print((5 + 3) + 2)
  }
  ",
  "10"),
  nested_math2_test: ("
  fn main() {
    print(2 + (5 + 3))
  }
  ",
  "10"),
  nested_math3_test: ("
  fn main() {
    print((2 + 5 + 3) - ((2 * 3) - (10/2)))
  }
  ",
  "9"),
  array_test: ("
  fn main() {
    let a[3] = [1,12,123]
    let b = a[2]
    print(a[2] + a[2])
  }
  ",
  "246"),
  global_var_test: ("
  fn main() {
    global theglobal = 2
    foo()
  }

  fn foo() {
    print(theglobal)
  }
  ",
  "2"),
  global_array_test: ("
  fn main() {
    global theglobal[2] = [6,7]
    foo()
  }

  fn foo() {
    print(theglobal[0])
  }
  ",
  "6"),
  while_control_test: ("
  fn main() {
    let a = 2
    while a < 5 {
      print(a)
      a = a + 1
    }
  }
  ",
  "234"),

}
