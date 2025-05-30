use miette::{Diagnostic, NamedSource, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

use crate::{
    ast::ASTNode, error::LangError, lexer::Lexer, parser::Parser, util::compile_ast_to_ir,
};

#[derive(Error, Debug, Diagnostic)]
pub enum FileProcessorError {
    #[error("Workspace not found: {0}")]
    WorkspaceNotFound(String),

    #[error("Processing error: {0}")]
    ProcessingError(#[from] LangError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Visits and processes a single file, running it through the entire pipeline:
/// 1. Lexing
/// 2. Parsing
/// 3. AST generation
/// 4. IR compilation
pub fn visit_file(file_path: &Path) -> Result<Vec<ASTNode>, FileProcessorError> {
    // Read the file content
    let content = std::fs::read_to_string(file_path)?;

    // Create a named source for error reporting
    let source = Arc::new(NamedSource::new(
        file_path.to_string_lossy(),
        content.clone(),
    ));

    // Run the lexer
    let tokens = Lexer::new(&content).tokenize();

    // Run the parser
    let ast = Parser::new(tokens, source.clone())
        .parse()
        .map_err(FileProcessorError::ProcessingError)?;

    // Compile to IR (if needed)
    let _ir = compile_ast_to_ir(&ast);

    Ok(ast)
}

/// Validates and loads the workspace if it exists, otherwise returns an error
pub fn ensure_workspace() -> Result<(), FileProcessorError> {
    let workspace_path = PathBuf::from("./designtime.json");
    if !workspace_path.exists() {
        return Err(FileProcessorError::WorkspaceNotFound(
            "designtime.json not found in the current directory".to_string(),
        ));
    }

    // This will validate the workspace configuration
    let _config = super::validate_and_load_workspace();
    Ok(())
}

/// Processes a file with workspace validation
pub fn process_file(file_path: &str) -> Result<Vec<ASTNode>, FileProcessorError> {
    // First ensure we're in a valid workspace
    ensure_workspace()?;

    // Then process the file
    visit_file(Path::new(file_path))
}
