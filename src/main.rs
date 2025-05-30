use std::path::Path;
use designtime_rs::{
    workspace::{self, FileProcessorError},
    util::compile_ast_to_ir
};
use miette::Result;

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

    Ok(())
}
