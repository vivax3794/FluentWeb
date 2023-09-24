//! Contains errors used in the crate

use thiserror::Error;

/// Everything that can go wrong
#[derive(Error, Debug)]
pub enum Compiler {
    /// We are not quite sure, should be replaced with better error.
    #[error("Unknwon: {0}")]
    Generic(String),
    /// Error in reading or writing files
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Error in decoding content
    #[error(transparent)]
    Encoding(#[from] std::str::Utf8Error),
    /// Erorr in parsing some syntax
    #[error("Error parsing rust syntax: {0}")]
    Parse(#[from] syn::Error),
    /// Wrong syntax, it is valid rust, but not the format expected
    #[error("{0}")]
    WrongSyntax(String),
}

/// A []
pub type CompilerResult<T> = Result<T, Compiler>;
