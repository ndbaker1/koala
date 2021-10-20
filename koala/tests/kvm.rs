use std::{cell::Cell, rc::Rc};

use koala::{
    instructions::{END, PRINT},
    kvm::VirtualMachine,
};

#[test]
fn kvm_output_callback_modify_values() {
    let new_val = 5;

    let value = Rc::new(Cell::new(0));
    let captured_value = value.clone();

    let print_callback = &move |_: &str| {
        captured_value.set(new_val);
    };

    let mut vm = VirtualMachine::new(print_callback);
    vm.load_code(&[PRINT as u32, END as u32]);
    vm.run();

    assert_eq!(value.get(), new_val);
}
