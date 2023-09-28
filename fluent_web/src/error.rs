//! Contains errors used in the crate

use thiserror::Error;

/// Everything that can go wrong
#[derive(Error, Debug)]
pub enum Compiler {
    /// Error in reading or writing files
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Error in decoding content
    #[error(transparent)]
    Encoding(#[from] std::str::Utf8Error),
    /// Another encoding error
    #[error(transparent)]
    Encoding2(#[from] std::string::FromUtf8Error),
    /// Erorr in parsing some syntax
    #[error("Error parsing rust syntax: {0}")]
    Parse(#[from] syn::Error),
    /// Wrong syntax, it is valid rust, but not the format expected
    #[error("Invalid syntax: {0}")]
    WrongSyntax(&'static str),
    /// Something wrong with css unparssing
    #[error("Css unparse error: {0}")]
    CssPrintError(#[from] lightningcss::error::PrinterError),
    /// Something wrong with css parsing
    #[error("Css parse error: {0}")]
    CssPraseError(String),
    /// Component missing src attribute
    #[error("<component> tag needs a src attribute to point to component")]
    MissingSrc,
}

/// A `Result` using the `Compiler` error
pub type CompilerResult<T> = Result<T, Compiler>;
