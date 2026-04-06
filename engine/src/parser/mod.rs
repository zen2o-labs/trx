pub mod blocks;
pub mod error;

use crate::ast::{Expression, Project};
use crate::parser::error::{Location, ParseError};
use pest::Parser;
use trx_syntax::{Rule, TrxParser};

use blocks::diagram::parse_single_diagram;
use blocks::packet::parse_packet_decl;
use blocks::sqltable::parse_sqltable_decl;
use blocks::state::parse_state_decl;
use blocks::xy::parse_xy_decl;

fn parse_math_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    if pair.as_rule() == Rule::math_expression {
        let mut inner = pair.into_inner();
        let mut expr = parse_math_term(inner.next().unwrap());
        while let (Some(op), Some(term)) = (inner.next(), inner.next()) {
            expr = Expression::BinaryOp(
                Box::new(expr),
                op.as_str().trim().to_string(),
                Box::new(parse_math_term(term)),
            );
        }
        expr
    } else {
        parse_math_term(pair)
    }
}

fn parse_math_term(pair: pest::iterators::Pair<Rule>) -> Expression {
    let mut check_pair = pair;
    if check_pair.as_rule() == Rule::math_term {
        check_pair = check_pair.into_inner().next().unwrap();
    }
    match check_pair.as_rule() {
        Rule::function_call => {
            let f_inner = check_pair.into_inner();
            let mut name_parts = Vec::new();
            let mut args = Vec::new();
            for p in f_inner {
                if p.as_rule() == Rule::identifier {
                    name_parts.push(p.as_str());
                } else if p.as_rule() == Rule::math_expression
                    || p.as_rule() == Rule::math_term
                    || p.as_rule() == Rule::number
                    || p.as_rule() == Rule::property_ref
                    || p.as_rule() == Rule::function_call
                    || p.as_rule() == Rule::number_with_unit
                {
                    args.push(parse_math_expr(p));
                }
            }
            Expression::FunctionCall {
                name: name_parts.join("."),
                args,
            }
        }
        Rule::identifier | Rule::property_ref => {
            Expression::VariableRef(check_pair.as_str().trim().to_string())
        }
        Rule::number => Expression::Number(check_pair.as_str().parse().unwrap_or(0.0)),
        Rule::number_with_unit => Expression::Unit(
            check_pair
                .as_str()
                .trim_end_matches(|c: char| c.is_alphabetic())
                .parse()
                .unwrap_or(0.0),
            "px".to_string(),
        ),
        _ => Expression::String(check_pair.as_str().to_string()),
    }
}

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
                                println!("Rule: {:?}", expr_pair.as_rule());
                                let parsed = parse_math_expr(expr_pair.clone());
                                println!("Parsed {}: {:?}", id, parsed);
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
