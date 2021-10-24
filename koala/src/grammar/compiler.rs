use super::grammar::{
    BinExpr, BinOp, Expr, FunctionCall, FunctionDefinition, If, Program, Statement, When, WhenCase,
    WhenElse,
};
use crate::instructions::{
    BEQZ, CALL, CONST, END, IADD, IDIV, IMUL, ISUB, LOAD, POP, PRINT, RET, STORE,
};
use std::collections::HashMap;

pub struct CompilerContext {
    pub fn_map: HashMap<String, usize>,
    pub var_map: HashMap<String, usize>,
    pub var_scope: Vec<Vec<String>>,
}
impl CompilerContext {
    pub fn new() -> CompilerContext {
        CompilerContext {
            fn_map: HashMap::new(),
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
            match context.fn_map.get(ENTRY_POINT) {
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
        match context.fn_map.get(&self.id) {
            None => {
                /*
                 * We can keep track of the function addresses by labels this way,
                 * and then load their address in the FunctionCall production more easily
                 */
                context.fn_map.insert(self.id.clone(), start_addr);

                let mut code = Vec::new();

                let mut new_scope = Vec::new();
                for (index, arg) in self.args.iter().enumerate() {
                    // expect the values to be on the stack from FunctionCall,
                    // and then move them into call stack
                    code.extend([STORE, index as u32, POP]);
                    // set variable in scope for lookups
                    new_scope.push(arg.clone());
                }
                // push new scope
                context.var_scope.push(new_scope);
                for stmt in &self.body {
                    code.extend(stmt.code_gen(context, start_addr + code.len() + 1));
                }
                // pop new scope
                context.var_scope.pop();
                // return to caller
                code.extend([RET]);

                return code;
            }
            Some(_) => panic!("duplicate function definition for {}().", self.id),
        }
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
                        Some(var_offset) => code.extend([STORE, var_offset as u32, POP]),
                        None => {
                            scope.push(id.clone());
                            code.extend([STORE, scope.len() as u32, POP]);
                        }
                    }
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
                    Some(index) => (index) as u32,
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

impl CodeGen for FunctionCall {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        let mut code = Vec::new();

        for (index, arg) in self.args.iter().enumerate() {
            code.extend(arg.code_gen(context, start_addr + code.len()));
        }
        code.extend([
            CALL,
            match context.fn_map.get(&self.id) {
                Some(addr) => *addr as u32,
                None => panic!("No function found to jump to"),
            },
        ]);

        return code;
    }
}
