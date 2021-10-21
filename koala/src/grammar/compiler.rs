use crate::instructions::{CONST, END, IADD, IDIV, IMUL, ISUB, LOAD, PRINT};

use super::grammar::{
    BinExpr, BinOp, Expr, If, IfElse, Program, Statement, TopLevel, Variable, When, WhenCase,
    WhenElse,
};

pub trait CodeGen {
    fn code_gen(&self) -> Vec<u32>;
}

impl CodeGen for Program {
    fn code_gen(&self) -> Vec<u32> {
        self.0
            .iter()
            .flat_map(|stmt| stmt.code_gen())
            .chain([END])
            .collect()
    }
}

impl CodeGen for TopLevel {
    fn code_gen(&self) -> Vec<u32> {
        match self {
            Self::Statement(stmt) => stmt.code_gen(),
            _ => Vec::new(),
        }
    }
}

impl CodeGen for Statement {
    fn code_gen(&self) -> Vec<u32> {
        match self {
            Self::Print(expr) => expr
                .code_gen()
                .iter()
                .chain([PRINT, 1].iter())
                .cloned()
                .collect(),
            _ => Vec::new(),
        }
    }
}
impl CodeGen for If {
    fn code_gen(&self) -> Vec<u32> {
        vec![]
    }
}
impl CodeGen for IfElse {
    fn code_gen(&self) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for When {
    fn code_gen(&self) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for WhenCase {
    fn code_gen(&self) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for WhenElse {
    fn code_gen(&self) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for Expr {
    fn code_gen(&self) -> Vec<u32> {
        match self {
            Self::IntLit(int) => vec![CONST, *int],
            Self::StringLit(string) => string
                .chars()
                .into_iter()
                .map(|a| a.to_digit(10))
                .take_while(|a| a.is_some())
                .map(|a| a.unwrap())
                .collect(),
            Self::BoolLit(truthy) => vec![CONST, *truthy as u32],
            Self::Variable(_) => vec![LOAD],
            Self::BinExpr(bin_expr) => bin_expr.code_gen(),
        }
    }
}

impl CodeGen for BinExpr {
    fn code_gen(&self) -> Vec<u32> {
        let mut code = Vec::new();

        code.extend(self.op2.code_gen());
        code.extend(self.op1.code_gen());

        match self.binop {
            BinOp::Plus => code.push(IADD),
            BinOp::Minus => code.push(ISUB),
            BinOp::Mul => code.push(IMUL),
            BinOp::Div => code.push(IDIV),
        };

        code
    }
}

impl CodeGen for Variable {
    fn code_gen(&self) -> Vec<u32> {
        vec![]
    }
}
