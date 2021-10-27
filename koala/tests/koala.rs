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
    if 1 { if 1 {} }
    if 0 { if 0 {} }
  }
  ",
  ""),
  fib_test: ("
  fn main() {
    print(fib(4))
  }
  
  fn fib(n) {
    if n {
      if (n-1) {
        return (fib((n-1)) + fib((n-2)))
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
    a((a-1))
    a((a-2))
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
}
