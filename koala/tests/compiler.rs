use koala::{
    grammar::{
        compiler::CodeGen,
        grammar::{Expr, FunctionDefinition, If, Program, Statement, TopLevel},
    },
    instructions::{CONST, END, PRINT},
};

#[test]
fn load_constant() {
    let print_val = 5;
    let ast_root = Program(vec![TopLevel::Statement(Statement::Print(Expr::IntLit(
        print_val,
    )))]);

    let json = match serde_json::to_string_pretty(&ast_root) {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("{}", json);

    assert_eq!(ast_root.code_gen(), [CONST, print_val, PRINT, 1, END]);
}

#[test]
fn program1_test() {
    let ast_root = Program(vec![
        TopLevel::Statement(Statement::If(Box::new(If {
            expr: Expr::BoolLit(true),
            stmts: vec![Statement::Print(Expr::StringLit(String::from("Print")))],
        }))),
        TopLevel::Statement(Statement::Print(Expr::IntLit(2))),
    ]);

    let json = match serde_json::to_string_pretty(&ast_root) {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("{}", json);
}

#[test]
fn program2_test() {
    let ast_root = Program(vec![TopLevel::FunctionDefinition(FunctionDefinition())]);

    let json = match serde_json::to_string_pretty(&ast_root) {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("{}", json);
}
