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
            = id:$(['a'..='z' | 'A'..='Z']+['a'..='z' | 'A'..='Z' |  '0'..='9' | '_']*) { id.to_string() }

        rule binop() -> BinOp
            = "*" { BinOp::Mul }
            / "/" { BinOp::Div }
            / "+" { BinOp::Plus }
            / "-" { BinOp::Minus }

        rule args() -> Vec<Expr>
        = _ expr:expr() _ "," _ args:args() {
            let mut all = vec![expr];
            all.extend(args);
            return all;
         }
        / _ expr:expr() _ { vec![expr] }
        / _ { vec![] }

        rule function_call() -> FunctionCall
            = id:identifier() "(" args:args() ")" { FunctionCall { id, args } }

        rule arg_defs() -> Vec<String>
            = _ id:identifier() _ "," _ args:arg_defs() {
                let mut arg_defs = vec![id];
                arg_defs.extend(args);
                return arg_defs;
             }
            / _ id:identifier() _ { vec![id] }
            / _ { vec![] }

        rule expr() -> Expr
            = "(" _ op1:expr() _ binop:binop() _ op2:expr() _ ")" { Expr::BinExpr(Box::new(BinExpr { binop, op2, op1 })) }
            / "(" _ expr:expr() _ ")" { expr }
            / "true" { Expr::BoolLit(true) }
            / "false" { Expr::BoolLit(false) }
            / f:function_call() { Expr::FunctionCall(f) }
            / id:identifier() { Expr::Variable(id) }
            / n:number() { Expr::IntLit(n) }

        rule statement() -> Statement
            = "print(" _ e:expr() _ ")" { Statement::Print(e) }
            / "if" _ expr:expr() _ "{" stmts:statements() "}" {
                Statement::If(Box::new(If {
                    expr,
                    stmts,
                }))
            }
            / "return" _ expr:expr() { Statement::ReturnExpr(expr) }
            / "return" { Statement::Return }
            / f:function_call() { Statement::FunctionCall(f) }
            / "let "? _ id:identifier() _ "=" _ expr:expr() { Statement::Assignment { id, expr } }

        rule statements() -> Vec<Statement>
            = _ stmt:statement() _ stmts:statements() {
                let mut statements = vec![stmt];
                statements.extend(stmts);
                return statements;
            }
            / _ stmt:statement() _ { vec![stmt] }
            / _ { vec![] }

        rule function_definition() -> FunctionDefinition
            =  "fn " _ id:identifier() "(" args:arg_defs() ")" _ "{" body:statements() "}" { FunctionDefinition { id, args, body } }

        pub rule program() -> Program
            = _ f:function_definition() _ p:program() {
                let mut program = p;
                program.0.push(f);
                return program;
            }
            / _ f:function_definition() _ { Program(vec![f]) }
    }
}

pub fn parse_code(code: &str) -> Result<Program, ParseError<LineCol>> {
    // Convert the JSON string back to a Point.
    koala_parser::program(code)
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
fn main_test() {
    let code = "fn main() {
        print(2)
    }";
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
fn binary_expr_test() {
    let code = "fn main() {
        print((2+5))
    }";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn function_call_test() {
    let code = "
    fn main() {
      func()
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

#[test]
fn void_return_test() {
    let code = "
    fn main() {
        recurse()
    }
    fn recurse() {
        return
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn return_expr_test() {
    let code = "
    fn main() {
        print(return_value())
    }
    fn return_value() {
        return 5
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn args_test() {
    let code = "
    fn main() {
        test(4, 6)
    }
    fn test(n, m) {
        return (n * m)
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn factorial_test() {
    let code = "
    fn main() {
        factorial(4)
    }
    fn factorial(n) {
        return (factorial((n - 1)) * n)
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn variable_usages_test() {
    let code = "
    fn main() {
        let a = 2
        b = 3
    }
    ";
    match koala_parser::program(code) {
        Ok(program) => program,
        Err(error) => panic!("{}", error),
    };
}
