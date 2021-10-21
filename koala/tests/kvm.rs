use std::{cell::RefCell, fs, rc::Rc};

use koala::{
    grammar::{compiler::CodeGen, parser::parse},
    instructions::{CONST, END, PRINT},
    kvm::VirtualMachine,
};

use crate::utils::fixture_path;

mod utils;

#[test]
fn kvm_output_callback_modify_values() {
    let new_val = "test";

    let value = Rc::new(RefCell::new(""));
    let captured_value = value.clone();

    let print_callback = &move |_: &str| *captured_value.borrow_mut() = new_val;

    let code = &[CONST as u32, 0, PRINT as u32, 1, END as u32];

    let mut vm = VirtualMachine::new(print_callback);
    vm.load_code(code);
    vm.run();

    assert_eq!(*value.borrow(), new_val);
}

#[test]
fn kvn_stack_load_print() {
    let test_value = 1;

    let value = Rc::new(RefCell::new(String::new()));
    let captured_value = value.clone();

    let print_callback = &move |val: &str| *captured_value.borrow_mut() += val;

    let code = &[CONST as u32, test_value, PRINT as u32, 1, END as u32];

    let mut vm = VirtualMachine::new(print_callback);
    vm.load_code(code);
    vm.run();

    assert_eq!(*value.borrow(), test_value.to_string());
}

#[test]
fn program2_test() {
    let code = match fs::read_to_string(fixture_path("program2.json")) {
        Ok(file_content) => file_content,
        Err(_) => String::from(""),
    };

    let program = parse(&code);

    let code = &program.code_gen();

    let value = Rc::new(RefCell::new(String::new()));
    let captured_value = value.clone();

    let print_callback = &move |val: &str| *captured_value.borrow_mut() += val;

    let mut vm = VirtualMachine::new(print_callback);
    vm.load_code(code);
    vm.run();

    assert_eq!(*value.borrow(), "8");
}
