//! Contains errors used in the crate

use miette::{Diagnostic, SourceOffset, SourceSpan};
use thiserror::Error;

/// Everything that can go wrong
#[derive(Diagnostic, Error, Debug)]
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
    #[error("invalid rust syntax")]
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

    /// Wrong syntax in the data section
    #[error("Invalid syntax in <data> statement")]
    WrongSyntaxInDataSection {
        /// The source code
        #[source_code]
        src: String,

        /// The span, mostly the line, where the error is.
        #[label = "Expected: let mut NAME: TYPE = VALUE;"]
        err_span: SourceSpan,
    },
}

/// A `Result` using the `Compiler` error
pub type CompilerResult<T> = Result<T, Compiler>;

/// Convert a token stream into a error span
pub fn procmacro_tokens_to_mietti_span(
    src: &str,
    tokens: proc_macro2::TokenStream,
    line_offset: usize,
) -> miette::SourceSpan {
    let span = syn::Error::new_spanned(tokens, "").span();
    let start = span.start();
    let end = span.end();

    let start = miette::SourceOffset::from_location(src, start.line - line_offset, start.column);
    let end = miette::SourceOffset::from_location(src, end.line - line_offset, end.column);

    let length = end.offset() - start.offset() + 1;
    SourceSpan::new(start, SourceOffset::from(length))
}
