use super::grammar::{
    BinExpr, BinOp, Expr, FunctionCall, FunctionDefinition, If, Program, Statement, When, WhenCase,
    WhenElse,
};
use crate::instructions::{
    BEQZ, CALL, CONST, END, FP_MOVE, IADD, IDIV, IMUL, ISUB, LOAD, POP, PRINT, RET, SP_READ, STORE,
};
use std::collections::HashMap;

pub struct CompilerContext {
    pub fn_table: HashMap<String, usize>,
    pub var_map: HashMap<String, usize>,
    pub var_scope: Vec<Vec<String>>,
}
impl CompilerContext {
    pub fn new() -> CompilerContext {
        CompilerContext {
            fn_table: HashMap::new(),
            var_map: HashMap::new(),
            var_scope: Vec::new(),
        }
    }
}
/// Trait for Productions and Terminals which generate code
pub trait CodeGen {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32>;
}

const ENTRY_POINT: &str = "main";

impl CodeGen for Program {
    fn code_gen(&self, context: &mut CompilerContext, _: usize) -> Vec<u32> {
        let mut code = Vec::new();

        const BOOTSTRAP_LENGTH: usize = 3;
        // generate code for every definition
        for def in &self.0 {
            code.extend(def.code_gen(context, code.len() + BOOTSTRAP_LENGTH));
        }
        // generate procedure that executes only main
        let entry_point_code: [u32; BOOTSTRAP_LENGTH] = [
            CALL,
            match context.fn_table.get(ENTRY_POINT) {
                Some(address) => *address as u32,
                None => panic!("could not find main function."),
            },
            END,
        ];
        // prefix the code with main entrypoint
        return entry_point_code
            .iter()
            .cloned()
            .chain(code.into_iter())
            .collect();
    }
}

impl CodeGen for FunctionDefinition {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        // check already existing function name
        match context.fn_table.get(&self.id) {
            None => {
                /*
                 * We can keep track of the function addresses by labels this way,
                 * and then load their address in the FunctionCall production more easily
                 */
                context.fn_table.insert(self.id.clone(), start_addr);

                let mut code = Vec::new();

                // Create a new scope for this function Enclosure
                let mut new_scope = Vec::new();
                for arg in self.args.iter() {
                    new_scope.push(arg.clone());
                }
                // push new scope
                context.var_scope.push(new_scope);
                for stmt in &self.body {
                    code.extend(stmt.code_gen(context, start_addr + code.len() + 1));
                }
                // Pop scope since we are leaving function
                context.var_scope.pop();
                // Pop each Var passed through the stack by FunctionCall
                for _ in &self.args {
                    code.push(POP);
                }

                // return to caller
                code.extend([RET]);

                return code;
            }
            Some(_) => panic!("duplicate function definition for {}().", self.id),
        }
    }
}

impl CodeGen for FunctionCall {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        let mut code = Vec::new();
        // load every expression onto stack
        for (index, arg) in self.args.iter().enumerate() {
            code.extend(arg.code_gen(context, start_addr + code.len()));
        }
        /*
         * Read Stack pointer (which is on top of function args),
         * Subtract the length to get to the start of the args,
         * then update our frame pointer to start here.
         */
        code.extend([
            CONST,
            (self.args.len() - 1 + 2 /* PLus 2 because we created an addition 2 elements on stack to calculate */) as u32,
            SP_READ,
            ISUB,
            FP_MOVE,
        ]);

        // Search function table for address
        let fn_addr = match context.fn_table.get(&self.id) {
            Some(addr) => *addr as u32,
            None => panic!("No function found to jump to"),
        };
        code.extend([CALL, fn_addr]);

        return code;
    }
}

impl CodeGen for Statement {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        match self {
            Self::Print(expr) => expr
                .code_gen(context, start_addr)
                .iter()
                .chain([PRINT, 1].iter())
                .cloned()
                .collect(),
            Self::Assignment { id, expr } => {
                let mut code = expr.code_gen(context, start_addr);

                /*
                 * This functions as a part of Semantic analysis,
                 * Since we can determine if we are using an undefined varaible later on
                 */
                if let Some(scope) = context.var_scope.last_mut() {
                    match scope.into_iter().position(|var| var == id) {
                        Some(offset) => code.extend([STORE, offset as u32]),
                        None => scope.push(id.clone()),
                    };
                }

                return code;
            }
            Self::FunctionCall(func_call) => func_call.code_gen(context, start_addr),
            Self::If(if_data) => if_data.code_gen(context, start_addr),
            Self::Return => vec![RET],
            Self::ReturnExpr(expr) => expr
                .code_gen(context, start_addr)
                .iter()
                .chain([RET].iter())
                .cloned()
                .collect(),
            _ => Vec::new(),
        }
    }
}
impl CodeGen for If {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        let mut code = self.expr.code_gen(context, start_addr);

        // helper
        let calc_offset =
            |base_code: &Vec<_>, stmt_code: &Vec<_>| start_addr + base_code.len() + stmt_code.len();

        let mut code_to_execute = Vec::new();
        for stmt in &self.stmts {
            code_to_execute.extend(stmt.code_gen(context, calc_offset(&code, &code_to_execute)));
        }
        // prefix the statements with the branch
        code.extend([BEQZ, (calc_offset(&code, &code_to_execute)) as u32]);
        code.extend(code_to_execute);

        return code;
    }
}

impl CodeGen for When {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for WhenCase {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for WhenElse {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        vec![]
    }
}

impl CodeGen for Expr {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        match self {
            Self::IntLit(int) => vec![CONST, *int],
            Self::BoolLit(truthy) => vec![CONST, *truthy as u32],
            Self::StringLit(string) => string // TODO
                .chars()
                .into_iter()
                .map(|a| a.to_digit(10))
                .take_while(|a| a.is_some())
                .map(|a| a.unwrap())
                .collect(),
            Self::Variable(name) => vec![
                LOAD,
                match context
                    .var_scope
                    .last()
                    .unwrap()
                    .iter()
                    .position(|var| var == name)
                {
                    Some(index) => index as u32,
                    None => panic!("usage of undefined variable! '{}'", name),
                },
            ],
            Self::FunctionCall(func_call) => func_call.code_gen(context, start_addr),
            Self::BinExpr(bin_expr) => bin_expr.code_gen(context, start_addr),
        }
    }
}

impl CodeGen for BinExpr {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        let mut code = Vec::new();

        code.extend(self.op2.code_gen(context, start_addr + 1));
        code.extend(self.op1.code_gen(context, start_addr + code.len() + 1));

        match self.binop {
            BinOp::Plus => code.push(IADD),
            BinOp::Minus => code.push(ISUB),
            BinOp::Mul => code.push(IMUL),
            BinOp::Div => code.push(IDIV),
        };

        return code;
    }
}
