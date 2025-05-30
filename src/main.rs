use std::path::Path;
use cranelift_codegen::isa::CallConv;
use designtime_rs::{
    ir::function::{Instruction, IrFunction}, util::{compile_ast_to_ir, CraneliftCompiler}, workspace::{self, FileProcessorError}
};
use miette::Result;
use cranelift::prelude::{types::I64, *};

fn make_sig() -> Signature {
    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(I64));
    sig.returns.push(AbiParam::new(I64));
    sig
}

fn main() -> Result<()> {
    // Process the input file
    let ast = match workspace::process_file("examples/example.dt") {
        Ok(ast) => ast,
        Err(FileProcessorError::WorkspaceNotFound(_)) => {
            // If no workspace found, try to process the file directly
            println!("No workspace found, processing file directly...");
            workspace::file_processor::visit_file(Path::new("examples/example.dt"))?
        }
        Err(e) => return Err(e.into()),
    };

    println!("\n=== AST ===\n{:#?}", ast);

    // Compile AST to IR
    let module = compile_ast_to_ir(&ast);
    println!("\n=== IR Program ===\n{:#?}", module);

    let ir = IrFunction::new(
        "add1",
        vec!["x"],
        vec![
            Instruction::LoadParam(0),
            Instruction::LoadConst("1".into()),
            // Assume Add instruction coming later
            Instruction::Return,
        ],
    );


    let mut compiler = CraneliftCompiler::new();
    let sig = make_sig();
    let clif_func = compiler.compile_ir_function(&ir, &sig);

    println!("{}", clif_func.display());

    Ok(())
}
