use crate::ast::XyDeclaration;
use crate::parser::error::ParseError;
use trx_syntax::{Rule, TrxPair};

pub fn parse_xy_decl(pair: TrxPair) -> Result<XyDeclaration, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or(ParseError::MissingId)?.as_str().to_string();
    let stmts_pair = inner.next().ok_or(ParseError::MissingId)?;

    let mut x_axis = String::new();
    let mut y_axis = String::new();
    let mut data = String::new();

    if stmts_pair.as_rule() == Rule::xy_stmts {
        for stmt in stmts_pair.into_inner() {
            if stmt.as_rule() == Rule::xy_prop {
                let mut p_inner = stmt.into_inner();
                let key = p_inner.next().unwrap().as_str();
                let val_pair = p_inner.next().unwrap();
                let val = if val_pair.as_rule() == Rule::string_literal {
                    val_pair.as_str().trim_matches('"').to_string()
                } else {
                    val_pair.as_str().to_string()
                };

                match key {
                    "x_axis" => x_axis = val,
                    "y_axis" => y_axis = val,
                    "data" => data = val,
                    _ => {}
                }
            }
        }
    }

    Ok(XyDeclaration { name, x_axis, y_axis, data })
}
