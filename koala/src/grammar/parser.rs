use peg::{error::ParseError, str::LineCol};

use super::grammar::{Expr, Program, Statement, TopLevel};

peg::parser! {
  grammar koala_parser() for str {
    rule number() -> u32
        = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule expr() -> Expr
        = n:number() { Expr::IntLit(n) }

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
    //     // / f:function_def() p:program() {  }
    //     / function_def()
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

//   PROGRAM
//   = stmts:STMTS LINEEND prog:PROGRAM
//     { return stmts.concat(prog) }
//   / func_def:FUNC_DEF LINEEND prog:PROGRAM
//     { return [func_def].concat(prog) }
//   / FUNC_DEF
//   / STMTS

// FUNC_DEF
//   = WS "fun" WS id:IDENTIFIER "(" WS params:PARAMETERS WS ")" WS stmts:BLOCK
//     { return { type: '${CodeGens.assignment.name}', id: id.id, params, stmts } }
//   / WS "fun" WS id:IDENTIFIER "(" ")" WS stmts:BLOCK
//     { return { type: '${CodeGens.function_def.name}', id: id.id, params: [], stmts } }
// STMTS
//   = stmt:STMT LINEEND stmts:STMTS
//     { return [stmt].concat(stmts) }
//   / stmt:STMT
//     { return { Statement:  } }
// STMT
//   = WS "if" WS "(" cond:EXPR ")" WS true_branch:BLOCK WS "else" WS false_branch:BLOCK
//     { return { type: '${CodeGens.if_else.name}', cond, true_branch, false_branch } }
//   / WS "if" WS "(" cond:EXPR ")" WS stmts:BLOCK
//     { return { If: { expr: cond, stmts } } }
//   / WS when:WHEN
//     { return when }
//   / WS "while" WS "(" cond:EXPR ")" WS stmts:BLOCK
//     { return { type: '${CodeGens.while.name}', cond, stmts } }
//   / WS "print" "(" expr:EXPR ")"
//     { return { type: '${CodeGens.print.name}', expr } }
//   / WS id:IDENTIFIER WS type:ASSIGN WS expr:EXPR
//     { return { id, type: '${CodeGens.assignment.name}', expr } }
//   / WS "return" WS expr:EXPR
//     { return { type: '${CodeGens.return_expr.name}', expr } }
//   / WS "return"
//     { return { type: '${CodeGens.return.name}' } }
//   / WS id:IDENTIFIER "++"
//     { return { type: '${CodeGens.increment.name}', id } }
//   / WS id:IDENTIFIER "--"
//     { return { type: '${CodeGens.decrement.name}', id } }
//   / WS block:BLOCK
//     { return block }
// BLOCK
//   = "{" LINEEND stmts:STMTS LINEEND WS "}"
//     { return stmts }
// WHEN
//   = "when" WS "(" expr:EXPR ")" WS "{" LINEEND when_cases:WHENCASES LINEEND WS "}"
//     { return { type: '${CodeGens.when.name}', expr, when_cases } }
// WHENCASES
//   = when_case:WHENCASE LINEEND when_cases:WHENCASES
//     { return [when_case].concat(when_cases) }
//   / when_case:WHENCASE
//     { return [when_case] }
// WHENCASE
//   = WS case_expr:EXPR WS "->" WS case_value:STMT
//     { return { type: '${CodeGens.when_case_stmt.name}', case_expr, case_value } }
//   / WS case_expr:EXPR WS "->" WS case_value:EXPR
//     { return { type: '${CodeGens.when_case_expr.name}', case_expr, case_value } }
//   / WS "else" WS "->" WS case_value:STMT
//     { return { type: '${CodeGens.when_else_stmt.name}', case_value } }
//   / WS "else" WS "->" WS case_value:EXPR
//     { return { type: '${CodeGens.when_else_expr.name}', case_value } }
// EXPR
//   = "(" WS expr:EXPR WS ")"
//     { return expr }
//   / BINARY_EXPR
//   / FUNC_CALL
//   / IDENTIFIER
//   / STRINGLIT
//   / INTLIT
//   / BOOLLIT

// BINARY_EXPR
//   = fun_call:FUNC_CALL WS binop:BINARYOP WS expr:EXPR
//     { return { type: '${CodeGens.binary_expr.name}', binop, args: [fun_call, expr] } }
//   / id:IDENTIFIER WS binop:BINARYOP WS expr:EXPR
//     { return { type: '${CodeGens.binary_expr.name}', binop, args: [id, expr] } }
//   / str:STRINGLIT WS binop:BINARYOP WS expr:EXPR
//     { return { type: '${CodeGens.binary_expr.name}', binop, args: [str, expr] } }
//   / int:INTLIT WS binop:BINARYOP WS expr:EXPR
//     { return { type: '${CodeGens.binary_expr.name}', binop, args: [int, expr] } }

// PARAMETERS
//   = id:IDENTIFIER WS "," WS params:PARAMETERS
//     { return [id].concat(params) }
//   / id:IDENTIFIER
//     { return [id] }

// ARGS
//   = expr:EXPR WS "," WS args:ARGS
//     { return [expr].concat(args) }
//   / expr:EXPR
//     { return [expr] }

// BINARYOP = ("*"/"/"/"+"/"-"/"<"/">"/"<="/">="/"=="/"<>"/"||"/"&&")
//   { return text() }
// ASSIGN = "="

// FUNC_CALL
//   = id:IDENTIFIER "(" WS args:ARGS WS ")"
//     { return { type: '${CodeGens.function_call.name}', id: id.id, args } }
//   / id:IDENTIFIER "(" ")"
//     { return { type: '${CodeGens.function_call.name}', id: id.id, args: [] } }

// IDENTIFIER = [a-zA-Z][a-zA-Z0-9]*
//   { return { type: '${CodeGens.variable.name}', id: text() } }
// INTLIT = "-"? [0-9]+
//   { return { type: '${CodeGens.int.name}', value: parseInt(text()) } }
// STRINGLIT = "'" [a-zA-Z0-9 \\n\\r\\t]* "'"
//   { return { type: '${CodeGens.str.name}', value: text() } }
// BOOLLIT = ("true"/"false")
//   { return { type: '${CodeGens.bool.name}', value: Boolean(text()) } }

// LINEEND = WS NL
// NL = [\\r\\n]+
// WS = [ \\t]*`
