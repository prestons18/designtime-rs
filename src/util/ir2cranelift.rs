// PRIORITY: Add support for every instruction in the IR

use cranelift::prelude::*;
use cranelift_codegen::ir::{types::*, UserFuncName};
use cranelift_codegen::ir::{Function};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use std::collections::HashMap;

use crate::ir::function::{IrFunction, Instruction};

pub struct CraneliftCompiler {
    pub builder_context: FunctionBuilderContext,
}

impl CraneliftCompiler {
    pub fn new() -> Self {
        Self {
            builder_context: FunctionBuilderContext::new(),
        }
    }

    pub fn compile_ir_function(
        &mut self,
        ir: &IrFunction,
        sig: &Signature,
    ) -> Function {
        let name = UserFuncName::user(0, 0);
        let mut func = Function::with_name_signature(name, sig.clone());
        let mut builder = FunctionBuilder::new(&mut func, &mut self.builder_context);

        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        let mut vars = HashMap::new();

        // Declare function params as variables
        for (i, _) in ir.params.iter().enumerate() {
            let val = builder.block_params(block)[i];
            let var = Variable::new(i);
            builder.declare_var(var, I64);
            builder.def_var(var, val);
            vars.insert(i, var);
        }

        // Stack for expression evaluation (simple simulation)
        let mut value_stack: Vec<Value> = Vec::new();

        for instr in &ir.instructions {
            match instr {
                Instruction::LoadConst(val) => {
                    if let Ok(parsed) = val.parse::<i64>() {
                        let cval = builder.ins().iconst(I64, parsed);
                        value_stack.push(cval);
                    } else {
                        panic!("Unsupported const: {}", val);
                    }
                }
                Instruction::LoadParam(idx) => {
                    let var = Variable::new(*idx);
                    let val = builder.use_var(var);
                    value_stack.push(val);
                }
                Instruction::SetState(_) => {
                    todo!("SetState not yet implemented");
                }
                Instruction::GetState(_) => {
                    todo!("GetState not yet implemented");
                }
                Instruction::CallFunction(_name) => {
                    // Call a declared function via `builder.ins().call(...)`
                    // `Module` + declared function reference
                    todo!("CallFunction not yet implemented");
                }
                Instruction::Return => {
                    let ret = value_stack.pop().expect("Nothing to return");
                    builder.ins().return_(&[ret]);
                }
            }
        }

        builder.finalize();
        func
    }
}
