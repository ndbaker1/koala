use super::grammar::{
    BinExpr, BinOp, Expr, FunctionCall, FunctionDefinition, If, Program, Statement, When, WhenCase,
    WhenElse,
};
use crate::instructions::{
    AND, BEQZ, CALL, END, EQ, GLOBAL_LOAD, GLOBAL_STORE, GT, GTE, IADD, IDIV, IMUL, ISUB,
    LOCAL_ARR_LOAD, LOCAL_LOAD, LOCAL_STORE, LT, LTE, NEQ, OR, POP, PRINT, PUSH, RET,
};
use core::panic;
use std::collections::HashMap;

pub struct CompilerContext {
    pub fn_table: HashMap<String, usize>,
    pub local_var_scope: Vec<Vec<String>>,
    pub global_vars: Vec<String>,
}

impl CompilerContext {
    pub fn new() -> Self {
        CompilerContext {
            fn_table: HashMap::new(),
            local_var_scope: Vec::new(),
            global_vars: Vec::new(),
        }
    }
    pub fn find_local_var_index(&self, var_name: &str) -> Result<usize, String> {
        match self
            .local_var_scope
            .last()
            .unwrap()
            .iter()
            .position(|var| var == var_name)
        {
            Some(index) => Ok(index),
            None => Err(format!("usage of undefined local variable! '{}'", var_name)),
        }
    }
    pub fn find_global_var_index(&self, var_name: &str) -> Result<usize, String> {
        match self.global_vars.iter().position(|var| var == var_name) {
            Some(index) => Ok(index),
            None => Err(format!(
                "usage of undefined global variable! '{}'",
                var_name
            )),
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

        const BOOTSTRAP_LENGTH: usize = 4;
        // generate code for every definition
        for def in &self.0 {
            code.extend(def.code_gen(context, code.len() + BOOTSTRAP_LENGTH));
        }
        // generate procedure that executes only main
        let main_addr = match context.fn_table.get(ENTRY_POINT) {
            Some(address) => *address as u32,
            None => panic!("could not find main function."),
        };
        let entry_point_code: [u32; BOOTSTRAP_LENGTH] = [CALL, 0, main_addr, END];

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

                let mut code = vec![];

                // Create a new scope for this function Enclosure
                let new_scope = self.args.iter().cloned().collect();
                // push new scope
                context.local_var_scope.push(new_scope);
                // Recursively Generate Code
                for stmt in &self.body {
                    code.extend(stmt.code_gen(context, start_addr + code.len() + 1));
                }
                // Pop scope since we are leaving function
                context.local_var_scope.pop();
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
        for arg in self.args.iter().rev() {
            code.extend(arg.code_gen(context, start_addr + code.len()));
        }
        // Search function table for address
        let fn_addr = match context.fn_table.get(&self.id) {
            Some(addr) => *addr as u32,
            None => panic!("No function found to jump to"),
        };
        // Tell the Call inst how many args are in the frame
        code.extend([CALL, self.args.len() as u32, fn_addr]);

        return code;
    }
}

impl CodeGen for Statement {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        match self {
            Self::Print { expr, newline } => match expr {
                Some(e) => {
                    let mut code = e.code_gen(context, start_addr);
                    code.extend([PRINT, 1]);
                    code
                }
                None => vec![],
            }
            .into_iter()
            // Conditionally add newline print
            .chain(if *newline {
                vec![PUSH, 10, PRINT, 2]
            } else {
                vec![]
            })
            .collect(),
            Self::VarAssignment { id, expr, global } => {
                let mut code = expr.code_gen(context, start_addr);
                let mut offset = 0;
                if *global {
                    offset = match context.find_global_var_index(id) {
                        Ok(var_index) => var_index,
                        Err(_) => {
                            context.global_vars.push(id.clone());
                            context.global_vars.len() - 1
                        }
                    };
                } else {
                    if let Some(scope) = context.local_var_scope.last_mut() {
                        offset = match scope.into_iter().position(|var| var == id) {
                            Some(off) => off,
                            None => {
                                scope.push(id.clone());
                                scope.len() - 1
                            }
                        };
                    }
                }

                code.extend([
                    if *global { GLOBAL_STORE } else { LOCAL_STORE },
                    offset as u32,
                ]);
                return code;
            }
            Self::ArrayAssignment { id, size, elements } => {
                let mut code = Vec::new();

                // read the size to loop over it
                if let Some(Expr::IntLit(array_size)) = size {
                    // check that the size is equal to the element length !
                    if let Some(elements_vec) = elements {
                        if elements_vec.len() as u32 != *array_size {
                            panic!("cannot have specified array size different from array literal.")
                        }
                    }
                    // fetch the starting variable
                    if let Some(scope) = context.local_var_scope.last_mut() {
                        let offset = match scope.into_iter().position(|var| var == id) {
                            Some(offset) => offset,
                            None => {
                                scope.push(id.clone());
                                scope.len() - 1
                            }
                        } as u32;
                        // loop over size
                        for index in 0..*array_size {
                            code.extend(if let Some(elements_vec) = elements {
                                elements_vec[index as usize].code_gen(context, start_addr)
                            } else {
                                vec![PUSH, 0]
                            });
                            code.extend([LOCAL_STORE, offset + index]);
                        }
                    }
                }

                return code;
            }
            Self::FunctionCall(func_call) => func_call
                .code_gen(context, start_addr)
                .into_iter()
                .chain([POP])
                .collect(),
            Self::If(if_data) => if_data.code_gen(context, start_addr),
            Self::Return => vec![PUSH, 0, RET],
            Self::ReturnExpr(expr) => expr
                .code_gen(context, start_addr)
                .into_iter()
                .chain([RET])
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl CodeGen for If {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        // generate code for the comparison Expression
        let mut code = self.expr.code_gen(context, start_addr);
        // const offset
        const BRANCH_CODE_OFFSET: usize = 2;
        // helper
        let calc_offset =
            |base_code: &Vec<_>, stmt_code: &Vec<_>| start_addr + base_code.len() + stmt_code.len();

        let mut code_to_execute = Vec::new();
        for stmt in &self.stmts {
            code_to_execute.extend(stmt.code_gen(
                context,
                BRANCH_CODE_OFFSET + calc_offset(&code, &code_to_execute),
            ));
        }
        // prefix the statements with the branch
        let branch_code: [u32; BRANCH_CODE_OFFSET] =
            [BEQZ, 1 + calc_offset(&code, &code_to_execute) as u32];

        code.extend(branch_code);
        code.extend(code_to_execute);

        return code;
    }
}

impl CodeGen for Expr {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        match self {
            Self::IntLit(int) => vec![PUSH, *int],
            Self::BoolLit(truthy) => vec![PUSH, *truthy as u32],
            Self::StringLit(string) => string // TODO
                .chars()
                .into_iter()
                .map(|a| a.to_digit(10))
                .take_while(|a| a.is_some())
                .map(|a| a.unwrap())
                .collect(),
            Self::ArrayIndex { id, expr } => {
                let mut code = Vec::new();
                code.extend([PUSH, context.find_local_var_index(id).unwrap() as u32]);
                code.extend(expr.code_gen(context, start_addr));
                code.push(LOCAL_ARR_LOAD);
                return code;
            }
            Self::Variable { id } => {
                if let Ok(index) = context.find_local_var_index(id) {
                    return vec![LOCAL_LOAD, index as u32];
                }
                let index = match context.find_global_var_index(id) {
                    Ok(index) => index,
                    Err(_) => {
                        context.global_vars.push(id.clone());
                        context.global_vars.len() - 1
                    }
                };
                return vec![GLOBAL_LOAD, index as u32];
            }
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
            BinOp::Less => code.push(LT),
            BinOp::LessOrEqual => code.push(LTE),
            BinOp::Greater => code.push(GT),
            BinOp::GreaterOrEqual => code.push(GTE),
            BinOp::Equal => code.push(EQ),
            BinOp::NotEqual => code.push(NEQ),
            BinOp::Or => code.push(OR),
            BinOp::And => code.push(AND),
        };

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
