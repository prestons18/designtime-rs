use thiserror::Error;

#[derive(Error, Debug)]
pub enum DesignTimeError {
    #[error("Lexer error at {span:?}: {message}")]
    LexerError {
        span: Span,
        message: String,
        suggestion: Option<String>,
    },

    #[error("Parser error at {span:?}: {message}")]
    ParserError {
        span: Span,
        message: String,
        suggestion: Option<String>,
    },

    // other errors without spans can remain
    #[error("Compiler error: {0}")]
    CompilerError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl DesignTimeError {
    /// Creates a new instance of the error with the same variant and data
    pub fn clone_error(&self) -> Self {
        match self {
            Self::LexerError { span, message, suggestion } => Self::LexerError {
                span: *span,
                message: message.clone(),
                suggestion: suggestion.clone(),
            },
            Self::ParserError { span, message, suggestion } => Self::ParserError {
                span: *span,
                message: message.clone(),
                suggestion: suggestion.clone(),
            },
            Self::CompilerError(msg) => Self::CompilerError(msg.clone()),
            Self::RuntimeError(msg) => Self::RuntimeError(msg.clone()),
            Self::IoError(err) => Self::IoError(std::io::Error::new(err.kind(), err.to_string())),
            Self::Unknown(msg) => Self::Unknown(msg.clone()),
        }
    }
}

// Span struct to represent start and end byte offsets or line/col
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}