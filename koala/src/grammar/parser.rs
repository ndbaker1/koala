use super::grammar::{
    BinExpr, BinOp, Expr, FunctionCall, FunctionDefinition, If, Program, Statement,
};
use peg::{error::ParseError, str::LineCol};
use std::vec;

peg::parser! {
    grammar koala_parser() for str {
        rule _() = [' ' | '\n']*

        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        rule string() -> String
            = "\"" id:$(['a'..='z' | 'A'..='Z']*) "\"" { id.to_string() }

        rule identifier() -> String
            = id:$(['a'..='z' | 'A'..='Z']+['a'..='z' | 'A'..='Z' |  '0'..='9']*) { id.to_string() }

        rule expr() -> Expr
            = bin:binexpr() { Expr::BinExpr(Box::new(bin)) }
            / n:number() { Expr::IntLit(n) }
            / "true" { Expr::BoolLit(true) }
            / "false" { Expr::BoolLit(false) }
            / s:string() { Expr::StringLit(s) }

        rule binexpr() -> BinExpr
            = n:number() _ op:binop() _ e:expr() { BinExpr { binop: op, op2: Expr::IntLit(n), op1: e } }

        rule binop() -> BinOp
            = "*" { BinOp::Mul }
            / "/" { BinOp::Div }
            / "+" { BinOp::Plus }
            / "-" { BinOp::Minus }

        rule function_call() -> FunctionCall
            = id:identifier() "()" { FunctionCall { id, args: vec![] } }

        rule statement() -> Statement
            = "print(" _ e:expr() _ ")" { Statement::Print(e) }
            / "if" _ expr:expr() _ "{" stmts:statements() "}" {
                Statement::If(Box::new(If {
                    expr,
                    stmts,
                }))
            }
            / f:function_call() { Statement::FunctionCall(f) }

        rule statements() -> Vec<Statement>
            = _ stmt:statement() _ stmts:statements() {
                let mut statements = vec![stmt];
                statements.extend(stmts);
                statements
            }
            / _ stmt:statement() _ { vec![stmt] }
            / _ { vec![] }

        rule function_definition() -> FunctionDefinition
            =  "fn " _ id:identifier() "()" _ "{" body:statements() "}" {
                FunctionDefinition {
                    id,
                    args: vec![],
                    body,
                }
            }

        pub rule program() -> Program
            = _ f:function_definition() _ p:program() {
                let mut program = p;
                program.0.push(f);
                program
            }
            / _ f:function_definition() _ { Program(vec![f]) }
    }
}

pub fn parse_code(code: &str) -> Result<Program, ParseError<LineCol>> {
    // Convert the JSON string back to a Point.
    koala_parser::program(code)
}

#[test]
fn main_test() {
    let code = "fn main() {
        print(2+3)
    }";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn empty_main_test() {
    let code = "fn main() {}";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn spaced_main_test() {
    let code = "


    fn main() {
      print(2)
    
    
    
      print(3+2)
    
    
    }


    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn multiple_fn_test() {
    let code = "
    fn main() {
      print(2)
      print(3+2)
    }
    fn second() {
      print(5)
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn if_test() {
    let code = "
    fn main() {
      if 3 {
          print(3)
      }
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn recursion_test() {
    let code = "
    fn main() {
        recurse()
    }
    fn recurse() {
        recurse() 
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}
