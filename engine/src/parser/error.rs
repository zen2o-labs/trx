use thiserror::Error;
use trx_syntax::Rule;

#[derive(Error, Debug, Clone)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    PestError(String),
    #[error("Unexpected token: {0:?}")]
    UnexpectedRule(Rule),

    #[error("Missing identifier")]
    MissingId,

    #[error("Invalid shape type: {0}")]
    InvalidShape(String),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(err.to_string())
    }
}
