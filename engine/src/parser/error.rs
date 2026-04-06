use serde::Serialize;
use thiserror::Error;
use trx_syntax::Rule;

#[derive(Serialize, Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub source: String,
}

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("Syntax error at line {}:{} => {}", location.line, location.col, message)]
    ParseFailed { location: Location, message: String },
    #[error("Unexpected token: {0:?}")]
    UnexpectedRule(Rule),

    #[error("Missing identifier")]
    MissingId,

    #[error("Invalid shape type: {0}")]
    InvalidShape(String),
}
