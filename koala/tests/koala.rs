use koala::{
    compiler::{CodeGen, CompilerContext},
    kvm::VirtualMachine,
    parser::parse_code,
};

macro_rules! code_tests {
  ($($name:ident: $value:expr,)*) => {$(
      #[test]
      fn $name() {
        kvm_run_code($value);
      }
  )*}
}

fn kvm_run_code(code: &str) {
    let program = parse_code(&code).unwrap();
    let bin = program.code_gen(&mut CompilerContext::new(), 0);
    let mut kvm = VirtualMachine::new(&|msg: &str| print!("{}", msg));
    kvm.load_code(&bin);
    kvm.run();
}

code_tests! {
  empty_main_test: "fn main() {}",
  print_test: "
  fn main() {
    print(2)
  }
  ",
  comment_test: "
  fn main() {
    // print(2)
  }
  ",
  variable_test: "
  fn main() {
    let x = 2
    print(x)
  }
  ",
  nested_ifs_test: "
  fn main() {
    if 1 { if 0 {} }
    if 0 { if 1 {} }
    if 1 { if 1 {} }
    if 0 { if 0 {} }
  }
  ",
  fib_test: "
  fn main() {
    fib(4)
  }
  
  fn fib(n) {
    print(n)
    if n {
      if (n-1) {
        fib((n-1))
        fib((n-2))
      }
    }
  }
  ",
  multple_call_test_fp: "
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
  return_value_test: "
  fn main() {
    print(f())
  }

  f() {
    return 3
  }
  ",
}
