pub mod error;
pub mod blocks;

use crate::ast::Project;
use crate::parser::error::ParseError;
use trx_syntax::{TrxParser, Rule};
use pest::Parser;

use blocks::diagram::parse_single_diagram;

pub fn parse(input: &str) -> Result<Project, ParseError> {
    let pairs = TrxParser::parse(Rule::file, input)
        .map_err(|e| ParseError::PestError(e.to_string()))?;

    let mut project = Project::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::file => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::diagram => {
                            project.diagrams.push(parse_single_diagram(inner_pair)?);
                        }
                        _ => {}
                    }
                }
            }
            Rule::diagram => {
                project.diagrams.push(parse_single_diagram(pair)?);
            }
            _ => {}
        }
    }
    Ok(project)
}
