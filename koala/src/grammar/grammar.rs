use serde::{Deserialize, Serialize};

/// Program:
/// | FunctionDefinition Program
/// | ε
///
/// the main() function will be the entry
#[derive(Deserialize, Serialize)]
pub struct Program(pub Vec<FunctionDefinition>);

#[derive(Deserialize, Serialize)]
pub struct FunctionDefinition {
    pub id: String,
    pub args: Vec<String>,
    pub body: Vec<Statement>,
}

/// Statement:
/// | If
/// | IfElse
/// | When
/// | Print
/// | Return
/// | ReturnExpr
/// | Assignment
#[derive(Deserialize, Serialize)]
pub enum Statement {
    If(Box<If>),
    IfElse(Box<IfElse>),
    When(Box<When>),
    Print(Expr),
    Return,
    ReturnExpr(Expr),
    Assignment { var: Variable, expr: Expr },
}

/// If:
/// | IF Expr Statement
#[derive(Deserialize, Serialize)]
pub struct If {
    pub expr: Expr,
    pub stmts: Vec<Statement>,
}

/// IfElse:
/// | IF Expr Statement ELSE Statement
#[derive(Deserialize, Serialize)]
pub struct IfElse {
    pub expr: Expr,
    pub stmts: Vec<Statement>,
    pub else_stmts: Vec<Statement>,
}

/// When:
/// | WHEN
#[derive(Deserialize, Serialize)]
pub struct When {
    pub expr: Expr,
    pub cond_cases: Vec<WhenCase>,
    pub else_case: WhenElse,
}

#[derive(Deserialize, Serialize)]
pub enum WhenCase {
    Expr(Expr),
    Statment(Statement),
}

#[derive(Deserialize, Serialize)]
pub enum WhenElse {
    Expr(Expr),
    Statment(Statement),
}

/// Expr:
/// | bool
/// | string
/// | int
/// | Variable
#[derive(Deserialize, Serialize)]
pub enum Expr {
    BoolLit(bool),
    StringLit(String),
    IntLit(u32),
    Variable(Variable),
    BinExpr(Box<BinExpr>),
    FunctionCall { id: String, args: Vec<Expr> },
}

#[derive(Deserialize, Serialize)]
pub struct BinExpr {
    pub op1: Expr,
    pub op2: Expr,
    pub binop: BinOp,
}

#[derive(Deserialize, Serialize)]
pub enum BinOp {
    Plus,
    Minus,
    Div,
    Mul,
}
/// Variable:
/// | identifier
#[derive(Deserialize, Serialize)]
pub struct Variable(pub String);
