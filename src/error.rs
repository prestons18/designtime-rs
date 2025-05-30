use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;
use std::sync::Arc;

/// The central error type for all stages of the language toolchain.
#[derive(Error, Debug, Diagnostic)]
pub enum LangError {
    /// Raised when the parser finds an unexpected token.
    #[error("Unexpected token: `{token}`")]
    #[diagnostic(code(lang::parse::unexpected_token), help("Check the syntax or missing brackets."))]
    UnexpectedToken {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("unexpected token here")]
        span: SourceSpan,
        token: String,
    },

    /// Raised when a reference is made to an unknown variable or function.
    #[error("Unknown identifier: `{name}`")]
    #[diagnostic(code(lang::ir::unknown_ident), help("Ensure `{name}` is defined before use."))]
    UnknownIdent {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("identifier not found in scope")]
        span: SourceSpan,
        name: String,
    },

    /// Raised when a type mismatch occurs in an expression or assignment.
    #[error("Type mismatch: expected `{expected}`, found `{found}`")]
    #[diagnostic(code(lang::types::mismatch), help("Ensure the value matches the expected type."))]
    TypeMismatch {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("type mismatch here")]
        span: SourceSpan,
        expected: String,
        found: String,
    },

    /// Raised when an error occurs at runtime in the VM or evaluation phase.
    #[error("Runtime error: {message}")]
    #[diagnostic(code(lang::runtime::error))]
    RuntimeError {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("error occurred here")]
        span: SourceSpan,
        message: String,
        #[help]
        help: Option<String>,
    },

    /// Raised when the compiler encounters an unrecoverable internal issue.
    #[error("Internal compiler error")]
    #[diagnostic(code(lang::internal::bug), help("Please report this as a bug."))]
    InternalError {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("internal error triggered here")]
        span: SourceSpan,
    },
}
