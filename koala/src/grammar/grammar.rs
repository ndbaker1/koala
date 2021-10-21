use serde::{Deserialize, Serialize};

/// Program:
/// | TopLevel Program
/// | ε
#[derive(Deserialize, Serialize)]
pub struct Program(pub Vec<TopLevel>);

/// TopLevel:
/// | FunctionDefinition
/// | Statement
#[derive(Deserialize, Serialize)]
pub enum TopLevel {
    FunctionDefinition(FunctionDefinition),
    Statement(Statement),
}

#[derive(Deserialize, Serialize)]
pub struct FunctionDefinition();

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
    Assignment(Variable, Expr),
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
}

/// Variable:
/// | identifier
#[derive(Deserialize, Serialize)]
pub struct Variable(pub String);
