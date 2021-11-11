use serde::{Deserialize, Serialize};

/// Program:
/// | FunctionDefinition Program
/// | ε
///
/// the main() function will be the entrypoint
#[derive(Deserialize, Serialize)]
pub struct Program(pub Vec<FunctionDefinition>);

#[derive(Deserialize, Serialize)]
pub struct FunctionDefinition {
    pub id: String,
    pub args: Vec<String>,
    pub body: Vec<Statement>,
    pub has_return_val: bool,
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
    Print {
        expr: Option<Expr>,
        newline: bool,
    },
    Return,
    ReturnExpr(Expr),
    VarAssignment {
        id: String,
        expr: Expr,
        global: bool,
    },
    ArrayAssignment {
        id: String,
        size: Option<Expr>,
        elements: Option<Vec<Expr>>,
    },
    FunctionCall(FunctionCall),
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
#[derive(Debug, Deserialize, Serialize)]
pub enum Expr {
    BoolLit(bool),
    StringLit(String),
    IntLit(u32),
    ArrayIndex { id: String, expr: Box<Expr> },
    Variable { id: String },
    BinExpr(Box<BinExpr>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BinExpr {
    pub op1: Expr,
    pub op2: Expr,
    pub binop: BinOp,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BinOp {
    Plus,
    Minus,
    Div,
    Mul,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Or,
    And,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionCall {
    pub id: String,
    pub args: Vec<Expr>,
}
