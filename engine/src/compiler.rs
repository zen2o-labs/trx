use crate::parser::parse;
use crate::ast::Project;
use crate::parser::error::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("Logic error: {0}")]
    ValidationError(String),
}

pub fn compile(input: &str) -> Result<Project, CompileError> {
    let project = parse(input)?;

    if project.diagrams.is_empty() {
        return Err(CompileError::ValidationError("No diagrams found in input. Use '#' to start a diagram.".into()));
    }

    Ok(project)
}
