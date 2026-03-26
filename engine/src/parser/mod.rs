pub mod error;
pub mod blocks;

use crate::ast::{Expression, Project};
use crate::parser::error::ParseError;
use trx_syntax::{TrxParser, Rule};
use pest::Parser;

use blocks::diagram::parse_single_diagram;
use blocks::packet::parse_packet_decl;
use blocks::state::parse_state_decl;
use blocks::xy::parse_xy_decl;

pub fn parse(input: &str) -> Result<Project, ParseError> {
    let pairs = TrxParser::parse(Rule::file, input)
        .map_err(|e| ParseError::PestError(e.to_string()))?;

    let mut project = Project::default();

    for pair in pairs {
        if pair.as_rule() == Rule::file {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::diagram => {
                        project.diagrams.push(parse_single_diagram(inner_pair)?);
                    }
                    Rule::packet_decl => {
                        project.packets.push(parse_packet_decl(inner_pair)?);
                    }
                    Rule::state_decl => {
                        project.states.push(parse_state_decl(inner_pair)?);
                    }
                    Rule::xy_decl => {
                        project.xys.push(parse_xy_decl(inner_pair)?);
                    }
                    Rule::variable_decl => {
                        let mut inner = inner_pair.into_inner();
                        if let Some(id_pair) = inner.next() {
                            let id = id_pair.as_str().trim().to_string();
                            if let Some(expr_pair) = inner.next() {
                                // store strings, or a proper expr parser.
                                // Parse_expression helper and import it if needed.
                                let expr_str = expr_pair.as_str().to_string();

                                // Basic parsing of "n px" vs "n"
                                let expr = if let Ok(val) = expr_str.parse::<f64>() {
                                    Expression::Number(val)
                                } else if expr_str.ends_with("px") || expr_str.ends_with("bits") {
                                    let num_str = expr_str.trim_end_matches("px").trim_end_matches("bits").trim();
                                    if let Ok(val) = num_str.parse::<f64>() {
                                        Expression::Unit(val, "px".to_string())
                                    } else {
                                        Expression::String(expr_str)
                                    }
                                } else {
                                    Expression::String(expr_str)
                                };

                                project.variables.insert(id, expr);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(project)
}
