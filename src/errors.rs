// Error module for Oak Language
use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: String,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub enum OakError {
    // Lexer errors
    LexerError {
        message: String,
        location: SourceLocation,
    },
    
    // Parser errors
    ParseError {
        message: String,
        location: SourceLocation,
    },
    
    // Type errors
    TypeError {
        message: String,
        expected: String,
        found: String,
        location: SourceLocation,
    },
    
    // Semantic analysis errors
    SemanticError {
        message: String,
        location: SourceLocation,
    },
    
    // Code generation errors
    CodegenError {
        message: String,
        location: SourceLocation,
    },
    
    // Runtime errors
    RuntimeError {
        message: String,
        location: SourceLocation,
    },
    
    // IO errors
    IOError {
        message: String,
    },
}

impl fmt::Display for OakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OakError::LexerError { message, location } => {
                write!(f, "Lexer Error at {}: {}", location, message)
            }
            OakError::ParseError { message, location } => {
                write!(f, "Parse Error at {}: {}", location, message)
            }
            OakError::TypeError {
                message,
                expected,
                found,
                location,
            } => {
                write!(
                    f,
                    "Type Error at {}: {}. Expected {}, found {}",
                    location, message, expected, found
                )
            }
            OakError::SemanticError { message, location } => {
                write!(f, "Semantic Error at {}: {}", location, message)
            }
            OakError::CodegenError { message, location } => {
                write!(f, "Codegen Error at {}: {}", location, message)
            }
            OakError::RuntimeError { message, location } => {
                write!(f, "Runtime Error at {}: {}", location, message)
            }
            OakError::IOError { message } => {
                write!(f, "IO Error: {}", message)
            }
        }
    }
}

impl std::error::Error for OakError {}

pub type Result<T> = std::result::Result<T, OakError>;

#[derive(Debug)]
pub struct ErrorReport {
    pub errors: Vec<OakError>,
}

impl ErrorReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: OakError) {
        self.errors.push(error);
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn print_errors(&self) {
        for error in &self.errors {
            eprintln!("{}", error);
        }
    }
}
