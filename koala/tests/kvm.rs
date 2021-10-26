use std::{cell::RefCell, rc::Rc};

use koala::{
    instructions::{END, PRINT, PUSH},
    kvm::VirtualMachine,
};

#[test]
fn kvm_output_callback_modify_values() {
    let new_val = "test";

    let value = Rc::new(RefCell::new(""));
    let captured_value = value.clone();

    let print_callback = &move |_: &str| *captured_value.borrow_mut() = new_val;

    let code = &[PUSH as u32, 0, PRINT as u32, 1, END as u32];

    let mut vm = VirtualMachine::new(print_callback, print_callback);
    vm.load_code(code);
    vm.run();

    // assert_eq!(*value.borrow(), new_val);
}

#[test]
fn kvn_stack_load_print() {
    let test_value = 1;

    let value = Rc::new(RefCell::new(String::new()));
    let captured_value = value.clone();

    let print_callback = &move |val: &str| *captured_value.borrow_mut() += val;

    let code = &[PUSH as u32, test_value, PRINT as u32, 1, END as u32];

    let mut vm = VirtualMachine::new(print_callback, print_callback);
    vm.load_code(code);
    vm.run();

    // assert_eq!(*value.borrow(), test_value.to_string());
}
