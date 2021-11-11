use super::grammar::{
    BinExpr, BinOp, Expr, FunctionCall, FunctionDefinition, If, Program, Statement,
};
use peg::{error::ParseError, str::LineCol};
use std::vec;

peg::parser! {
    grammar koala_parser() for str {
        rule _()
            = [' ' | '\n']* "//" [^'\n']* _  // Comments
            / [' ' | '\n']*

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
            / "<=" { BinOp::LessOrEqual }
            / ">=" { BinOp::GreaterOrEqual }
            / "<" { BinOp::Less }
            / ">" { BinOp::Greater }
            / "!=" { BinOp::NotEqual }
            / "==" { BinOp::Equal }
            / "||" { BinOp::Or }
            / "&&" { BinOp::And }

        rule args() -> Vec<Expr>
            = _ expr:compound_expr() _ "," _ args:args() {
                let mut all = vec![expr];
                all.extend(args);
                return all;
            }
            / _ expr:compound_expr() _ { vec![expr] }
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
            = "true" { Expr::BoolLit(true) }
            / "false" { Expr::BoolLit(false) }
            // Function Calls
            / f:function_call() { Expr::FunctionCall(f) }
            // Array Indexing Rule
            / id:identifier() "[" _ expr:compound_expr() _ "]" { Expr::ArrayIndex{ id, expr: Box::new(expr) } }
            // Variable Load Rule
            / id:identifier() { Expr::Variable(id) }
            // Plain Number
            / n:number() { Expr::IntLit(n) }

        rule compound_expr() -> Expr
            = "(" _ op1:compound_expr() _ ")" _ binop:binop() _ op2:compound_expr() {
                Expr::BinExpr(Box::new(BinExpr { binop, op2, op1 }))
            }
            / op1:expr() _ binop:binop() _ op2:compound_expr() {
                Expr::BinExpr(Box::new(BinExpr { binop, op2, op1 }))
            }
            / "(" _ expr:compound_expr() _ ")" { expr }
            / expr()

        rule statement() -> Statement
            = "print(" _ e:compound_expr() _ ")" { Statement::Print(e) }
            / "if" _ expr:compound_expr() _ "{" stmts:statements() "}" {
                Statement::If(Box::new(If {
                    expr,
                    stmts,
                }))
            }
            / "return" _ expr:compound_expr() { Statement::ReturnExpr(expr) }
            / "return" { Statement::Return }
            / f:function_call() { Statement::FunctionCall(f) }
            / "let "? _ id:identifier() _ "=" _ expr:compound_expr() {
                Statement::VarAssignment { id, expr }
            }
            / "let "? _ id:identifier() "[" size:number() "]" _ "=" _ "[" elements:args() "]" {
                Statement::ArrayAssignment { id, size: Some(Expr::IntLit(size)), elements: Some(elements) }
            }
            / "let "? _ id:identifier() "[" size:compound_expr() "]" _ "=" _ "[" elements:args() "]" {
                Statement::ArrayAssignment { id, size: Some(size), elements: Some(elements) }
            }
            / "let "? _ id:identifier() "[" size:compound_expr() "]" {
                Statement::ArrayAssignment { id, size: Some(size), elements: None }
            }

        rule statements() -> Vec<Statement>
            = _ stmt:statement() _ stmts:statements() {
                let mut statements = vec![stmt];
                statements.extend(stmts);
                return statements;
            }
            / _ stmt:statement() _ { vec![stmt] }
            / _ { vec![] }

        rule function_definition() -> FunctionDefinition
            // Possible indicator of return value with '?'
            =  "fn " _ id:identifier() "(" args:arg_defs() ")" _ "?" _ "{" body:statements() "}" { FunctionDefinition { id, args, body, has_return_val: true } }
            /  "fn " _ id:identifier() "(" args:arg_defs() ")" _ "{" body:statements() "}" { FunctionDefinition { id, args, body, has_return_val: false } }

        /// Top Level list of function definitions
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

macro_rules! parser_tests {
    ($($name:ident: $value:expr,)*) => {$(
        #[test]
        fn $name() {
            parse_code($value).unwrap();
        }
    )*}
}

parser_tests! {
    empty_main_test: "fn main() {}",
    main_test: "fn main() {
        print(2)
    }",
    comment_test: "
    fn main() {
        // print(2)
        print(2)
    }",
    spaced_main_test: "

    fn main() {

      print(2)
    
    }

    ",
    multiple_fn_test: "
    fn main() {
      print(2)
    }
    fn second() {
      print(5)
    }
    ",
    binary_expr_test: "fn main() {
        print(2+5)
    }",
    function_call_test: "
    fn main() {
      func()
    }",
    if_test: "
    fn main() {
      if 3 {
          print(3)
      }
    }
    ",
    recursion_test: "
    fn main() {
        recurse()
    }
    fn recurse() {
        recurse() 
    }
    ",
    void_return_test: "
    fn main() {
        recurse()
    }
    fn recurse() {
        return
    }
    ",
    return_expr_test: "
    fn main() {
        print(return_value())
    }
    fn return_value() {
        return 5
    }
    ",
    args_test: "
    fn main() {
        test(4, 6)
    }
    fn test(n, m) {
        return n * m
    }
    ",
    factorial_test: "
    fn main() {
        factorial(4)
    }
    fn factorial(n) {
        return factorial(n - 1) * n
    }
    ",
    variable_usages_test: "
    fn main() {
        let a = 2
        b = 3
        print(a+b)
    }
    ",
    comparisons_parser_test: "
    fn main() {
        if 1 < 2 { }
        if 2 > 1 { }
        if 1 == 1 { }
        if 1 != 2 { }
        if 1 <= 1 { }
        if 1 >= 1 { }
    }
    ",
}
