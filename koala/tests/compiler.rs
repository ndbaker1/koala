use koala::{
    grammar::{
        compiler::CodeGen,
        grammar::{BinExpr, Expr, FunctionDefinition, If, Program, Statement, TopLevel, Variable},
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
    let ast_root = Program(vec![
        TopLevel::Statement(Statement::If(Box::new(If {
            expr: Expr::BoolLit(true),
            stmts: vec![Statement::Print(Expr::StringLit(String::from("Print")))],
        }))),
        TopLevel::Statement(Statement::Print(Expr::IntLit(2))),
        TopLevel::Statement(Statement::Assignment {
            var: Variable(String::from("test")),
            expr: Expr::BinExpr(Box::new(BinExpr {
                binop: koala::grammar::grammar::BinOp::Div,
                op1: Expr::IntLit(2),
                op2: Expr::IntLit(2),
            })),
        }),
        TopLevel::Statement(Statement::ReturnExpr(Expr::Variable(Variable(
            String::from("test"),
        )))),
    ]);

    let json = match serde_json::to_string_pretty(&ast_root) {
        Ok(s) => s,
        Err(_) => return,
    };

    println!("{}", json);
}
