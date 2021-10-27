use koala::compiler::{CodeGen, CompilerContext};
use koala::grammar::Program;
use koala::kvm::VirtualMachine;
use koala::parser::parse_code;
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
pub fn run(
    machine_code: &[u32],
    output_callback: &js_sys::Function,
    debug_callback: &js_sys::Function,
) {
    let rust_output_callback = &|msg: &str| {
        output_callback
            .call1(&JsValue::NULL, &JsValue::from_str(msg))
            .unwrap();
    };
    let rust_debug_callback = &|msg: &str| {
        debug_callback
            .call1(&JsValue::NULL, &JsValue::from_str(msg))
            .unwrap();
    };
    let mut vm = VirtualMachine::new(rust_output_callback, rust_debug_callback);
    vm.run(machine_code);
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn parseAst(source_code: &str) -> String {
    let program_repr = match parse_code(source_code) {
        Ok(program) => program,
        Err(e) => panic!("{}", e),
    };

    match serde_json::to_string_pretty(&program_repr) {
        Ok(ast_string) => ast_string,
        Err(e) => panic!("{}", e),
    }
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn astCodeGen(ast_string: &str) -> Vec<u32> {
    let program: Program = match serde_json::from_str(ast_string) {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    program.code_gen(&mut CompilerContext::new(), 0)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn sourceCodeGen(source_code: &str) -> Vec<u32> {
    let ast_string = parseAst(source_code);
    astCodeGen(&ast_string)
}
