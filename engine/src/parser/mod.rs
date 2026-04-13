pub mod blocks;
pub mod error;
pub mod expressions;

use crate::ast::Project;
use crate::parser::error::{Location, ParseError};
use pest::Parser;
use trx_syntax::{Rule, TrxParser};

use blocks::diagram::parse_single_diagram;
use blocks::packet::parse_packet_decl;
use blocks::sqltable::parse_sqltable_decl;
use blocks::state::parse_state_decl;
use blocks::xy::parse_xy_decl;
use expressions::parse_math_expr_stable;

pub fn parse(input: &str) -> Result<Project, ParseError> {
    let pairs = TrxParser::parse(Rule::file, input).map_err(|e| {
        let (line, col) = match e.line_col {
            pest::error::LineColLocation::Pos((l, c)) => (l, c),
            pest::error::LineColLocation::Span((l, c), _) => (l, c),
        };
        ParseError::ParseFailed {
            location: Location {
                line,
                col,
                source: e.line().to_string(),
            },
            message: format!("{}", e.variant),
        }
    })?;

    let mut project = Project::default();

    for pair in pairs {
        if pair.as_rule() == Rule::file {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::diagram => {
                        project
                            .diagrams
                            .push(parse_single_diagram(inner_pair, &mut project.classes)?);
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
                    Rule::sqltable_decl => {
                        project.sqltables.push(parse_sqltable_decl(inner_pair)?);
                    }
                    Rule::variable_decl => {
                        let mut inner = inner_pair.into_inner();
                        if let Some(id_pair) = inner.next() {
                            let id = id_pair.as_str().trim().to_string();
                            if let Some(expr_pair) = inner.next() {
                                let parsed = parse_math_expr_stable(expr_pair)?;
                                project.variables.insert(id, parsed);
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
