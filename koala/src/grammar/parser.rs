use peg::{error::ParseError, str::LineCol};

use super::grammar::{BinExpr, BinOp, Expr, Program, Statement, TopLevel};

peg::parser! {
  grammar koala_parser() for str {
    rule _() = [' ' | '\n']*

    rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule string() -> String
        = "\"" id:$(['a'..='z' | 'A'..='Z']*) "\"" { id.to_string() }

    rule expr() -> Expr
        = bin:binexpr() { Expr::BinExpr(Box::new(bin)) }
        / n:number() { Expr::IntLit(n) }
        / "true" { Expr::BoolLit(true) }
        / "false" { Expr::BoolLit(false) }
        / s:string() { Expr::StringLit(s) }

    rule binexpr() -> BinExpr
        = n:number() op:binop() e:expr() { BinExpr { binop: op, op2: Expr::IntLit(n), op1: e } }

    rule binop() -> BinOp
        = "*" { BinOp::Mul }
        / "/" { BinOp::Div }
        / "+" { BinOp::Plus }
        / "-" { BinOp::Minus }
    rule statement() -> Statement
        = "print(" e:expr() ")" { Statement::Print(e) }

    rule statements() -> Vec<Statement>
        = stmt:statement() { vec![stmt] }

    pub rule program() -> Program
        = s:statements() p:program() {
            let mut p = p;
            p.0.extend(s
                .into_iter()
                .map(|stmt| TopLevel::Statement(stmt))
                .collect::<Vec<TopLevel>>());
            p
        }
        / s:statements() {
            Program(
                s
                .into_iter()
                .map(|stmt| TopLevel::Statement(stmt))
                .collect()
            )
        }
  }
}

pub fn parse_code(code: &str) -> Result<Program, ParseError<LineCol>> {
    // Convert the JSON string back to a Point.
    koala_parser::program(code)
}
