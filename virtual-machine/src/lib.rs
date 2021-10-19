mod vm;

use vm::vm::VirtualMachine;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run(code: &[u32], output_callback: &js_sys::Function) {
    let rust_output_callback =
        &|msg: &str| match output_callback.call1(&JsValue::NULL, &JsValue::from_str(msg)) {
            Ok(_) => { /* 👍 */ }
            Err(_) => { /* 🔥 */ }
        };
    let mut vm = VirtualMachine::new(rust_output_callback);
    vm.load_code(code);
    vm.run()
}
