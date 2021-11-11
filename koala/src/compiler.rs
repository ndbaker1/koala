use super::grammar::{
    BinExpr, BinOp, Expr, FunctionCall, FunctionDefinition, If, Program, Statement,
};
use crate::instructions::{
    AND, BEQZ, CALL, END, EQ, GLOBAL_ARR_LOAD, GLOBAL_ARR_STORE, GLOBAL_LOAD, GLOBAL_STORE, GT,
    GTE, IADD, IDIV, IMUL, ISUB, JUMP, LOCAL_ARR_LOAD, LOCAL_ARR_STORE, LOCAL_LOAD, LOCAL_STORE,
    LT, LTE, NEQ, OR, POP, PRINT, PUSH, RET,
};
use core::panic;
use std::collections::HashMap;

pub struct CompilerContext {
    /// Table of Function names and their address
    pub fn_table: HashMap<String, usize>,
    /// A Frame-based set of indexes for local variables
    pub local_var_scope: Vec<(HashMap<String, usize>, usize)>,
    /// A Table with global variable addresses paired with the current memory offset
    pub global_vars: (HashMap<String, usize>, usize),
}

/// Denote between Global and Local Variables during Code Gen
pub enum ScopeType {
    Global,
    Local,
}

impl CompilerContext {
    pub fn new() -> Self {
        CompilerContext {
            fn_table: HashMap::new(),
            local_var_scope: Vec::new(),
            global_vars: (HashMap::new(), 0),
        }
    }

    /// Search both variables for an ID. (first local then global)
    pub fn find_var_index(&self, var_name: &str) -> Result<(ScopeType, usize), String> {
        if let Ok(index) = self.find_local_var_index(var_name) {
            return Ok((ScopeType::Local, index));
        }
        if let Ok(index) = self.find_global_var_index(var_name) {
            return Ok((ScopeType::Global, index));
        }

        return Err(format!("could not find variable with id: {}", var_name));
    }

    /// search the current Frame's local variable set for an ID
    pub fn find_local_var_index(&self, var_name: &str) -> Result<usize, String> {
        match self.local_var_scope.last().unwrap().0.get(var_name) {
            Some(index) => Ok(*index),
            None => Err(format!("usage of undefined local variable! '{}'", var_name)),
        }
    }

    /// Search the global varable table for an ID
    pub fn find_global_var_index(&self, var_name: &str) -> Result<usize, String> {
        match self.global_vars.0.get(var_name) {
            Some(index) => Ok(*index),
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

/// Function ID to bootstrap the executable
const ENTRY_POINT: &str = "main";

impl CodeGen for Program {
    fn code_gen(&self, context: &mut CompilerContext, _: usize) -> Vec<u32> {
        context.global_vars = self.create_global_var_table();

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
                let mut scope_size = 0;
                // Create a new scope for this function Enclosure
                let new_scope = self
                    .args
                    .iter()
                    .cloned()
                    .map(|id| {
                        scope_size += 1;
                        (id, scope_size - 1)
                    })
                    .collect::<HashMap<String, usize>>();
                // push new scope
                context.local_var_scope.push((new_scope, scope_size));
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
            // Append the codegen for non-empty print statements
            .into_iter()
            // Conditionally append the newline printing steps
            .chain(match *newline {
                true => vec![PUSH, 10, PRINT, 2],
                false => vec![],
            })
            .collect(),
            Self::VarAssignment { id, expr, global } => {
                // generate value to be stored
                let mut code = expr.code_gen(context, start_addr);

                // find the offset of the variable in either the global or local set
                let offset = match context.find_var_index(id) {
                    Ok(pair) => pair.1,
                    Err(_) => {
                        if *global {
                            // global variables are accounted for in the AST prescan, so we shouldnt see this fail.
                            panic!("failed to find global variable by id: {}", id);
                        } else {
                            // add the local variable to the frame if we are seeing it for the first time
                            let scope = context.local_var_scope.last_mut().unwrap();
                            scope.0.insert(id.clone(), scope.1);
                            scope.1 += 1;
                            scope.1 - 1
                        }
                    }
                };
                // append the appropriate Store procedure for global or local variables, with the given offset
                code.extend([
                    match global {
                        true => GLOBAL_STORE,
                        false => LOCAL_STORE,
                    },
                    offset as u32,
                ]);

                return code;
            }
            Self::ArrayIndexAssignment { id, index, expr } => {
                // generate value to be stored
                let mut code = expr.code_gen(context, start_addr);
                // generate the value of the array subscript index
                code.extend(index.code_gen(context, start_addr));
                // fetch the array index
                // since we cannot arbitrarily define new array values, this failure should not be accepted.
                let (scope_type, offset) = match context.find_var_index(id) {
                    Ok(pair) => pair,
                    Err(_) => panic!("array index failure"),
                };

                // push the offset onto the stack in order to read it in the ARRAY_STORE procedures
                code.extend([PUSH, offset as u32]);
                code.push(match scope_type {
                    ScopeType::Global => GLOBAL_ARR_STORE,
                    ScopeType::Local => LOCAL_ARR_STORE,
                });

                return code;
            }
            Self::ArrayInstantiation {
                id,
                size,
                elements,
                global,
            } => {
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
                    let offset = match context.find_var_index(id) {
                        Ok(pair) => pair.1,
                        Err(_) => {
                            if *global {
                                // cannot happen since we do an AST prescan for globals
                                panic!("could not find global index for array id: {}", id);
                            } else {
                                // add the local variable to the frame if we are seeing it for the first time
                                let scope = context.local_var_scope.last_mut().unwrap();
                                scope.0.insert(id.clone(), scope.1);
                                // this involves moving the memory boundary along by the size of the array
                                scope.1 += *array_size as usize;
                                scope.1 - *array_size as usize
                            }
                        }
                    };

                    // loop over size
                    for index in 0..*array_size {
                        // if the values were specified, then go ahead and load them,
                        // otherwise default them to 0
                        code.extend(if let Some(elements_vec) = elements {
                            elements_vec[index as usize].code_gen(context, start_addr)
                        } else {
                            vec![PUSH, 0]
                        });
                        // load the LOCAL or GLOBAL store procedure with its correcponding aray index
                        code.extend([
                            match *global {
                                true => GLOBAL_STORE,
                                false => LOCAL_STORE,
                            },
                            index + offset as u32,
                        ]);
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
            Self::While { cond, stmts } => {
                // generate code for the comparison Expression
                let mut code = cond.code_gen(context, start_addr);
                // const offsets for jumps around the code,
                // since they arent accounted for in the statement block.
                const BRANCH_CODE_OFFSET: usize = 2;
                const JUMP_CODE_OFFSET: usize = 2;
                // helper
                let calc_offset = |base_code: &Vec<_>, stmt_code: &Vec<_>| {
                    start_addr + base_code.len() + stmt_code.len()
                };
                // generate code for the statements and update the start address
                let mut code_to_execute = Vec::new();
                for stmt in stmts {
                    code_to_execute.extend(stmt.code_gen(
                        context,
                        BRANCH_CODE_OFFSET + calc_offset(&code, &code_to_execute),
                    ));
                }
                // prefix the statements with the branch
                let branch_code: [u32; BRANCH_CODE_OFFSET] = [
                    BEQZ,
                    1 + JUMP_CODE_OFFSET as u32 + calc_offset(&code, &code_to_execute) as u32,
                ];
                // jump back to the start of the while for the last step
                let jump_code: [u32; JUMP_CODE_OFFSET] = [JUMP, (start_addr - 1) as u32];

                // assemble the full code block
                code.extend(branch_code);
                code.extend(code_to_execute);
                code.extend(jump_code);

                return code;
            }
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
        // generate code for the statements and update the start address
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
                // find scope type and index of array pointer by id
                let (scope_type, index) = match context.find_var_index(id) {
                    Ok(pair) => pair,
                    Err(_) => panic!("cound not find array index to load for id: {}.", id),
                };
                // push index onto stack and then load subscript index
                let mut code = vec![PUSH, index as u32];
                code.extend(expr.code_gen(context, start_addr));
                // call to array load procedure
                code.push(match scope_type {
                    ScopeType::Local => LOCAL_ARR_LOAD,
                    ScopeType::Global => GLOBAL_ARR_LOAD,
                });

                return code;
            }
            Self::Variable { id } => {
                // fetch scope type and index of variable by id
                let (scope_type, index) = match context.find_var_index(id) {
                    Ok(pair) => pair,
                    Err(_) => panic!("could not find variable '{}'", id),
                };
                // return instructions to load the given index onto the stack
                vec![
                    match scope_type {
                        ScopeType::Global => GLOBAL_LOAD,
                        ScopeType::Local => LOCAL_LOAD,
                    },
                    index as u32,
                ]
            }
            Self::FunctionCall(func_call) => func_call.code_gen(context, start_addr),
            Self::BinExpr(bin_expr) => bin_expr.code_gen(context, start_addr),
        }
    }
}

impl CodeGen for BinExpr {
    fn code_gen(&self, context: &mut CompilerContext, start_addr: usize) -> Vec<u32> {
        let mut code = Vec::new();
        // generate both operands
        code.extend(self.op2.code_gen(context, start_addr + 1));
        code.extend(self.op1.code_gen(context, start_addr + code.len() + 1));
        // push operator code
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

impl Program {
    fn create_global_var_table(&self) -> (HashMap<String, usize>, usize) {
        let mut table: HashMap<String, usize> = HashMap::new();
        let mut index: usize = 0;
        for def in &self.0 {
            for stmt in &def.body {
                match stmt {
                    Statement::ArrayInstantiation {
                        id, size, global, ..
                    } => {
                        if *global {
                            if let Some(Expr::IntLit(incr)) = size {
                                table.insert(id.to_string(), index as usize);
                                index += *incr as usize;
                            }
                        }
                    }
                    Statement::VarAssignment { id, global, .. } => {
                        if *global {
                            table.insert(id.to_string(), index);
                            index += 1;
                        }
                    }
                    _ => { /* no-op */ }
                }
            }
        }

        return (table, index);
    }
}
