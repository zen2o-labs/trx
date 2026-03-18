use crate::ast::{PacketDeclaration, PacketField};
use crate::parser::error::ParseError;
use trx_syntax::{Rule, TrxPair};

pub fn parse_packet_decl(pair: TrxPair) -> Result<PacketDeclaration, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or(ParseError::MissingId)?.as_str().to_string();
    
    let mut attrs = String::new();
    let next_pair = inner.next().ok_or(ParseError::MissingId)?;
    let mut stmts_pair = next_pair.clone();
    
    if next_pair.as_rule() == Rule::packet_attrs {
        attrs = next_pair.as_str().trim().to_string();
        stmts_pair = inner.next().ok_or(ParseError::MissingId)?;
    }

    let mut fields = Vec::new();
    let mut constraint = None;

    if stmts_pair.as_rule() == Rule::packet_stmts {
        for stmt in stmts_pair.into_inner() {
            match stmt.as_rule() {
                Rule::packet_field => {
                    let mut f_inner = stmt.into_inner();
                    let range = f_inner.next().unwrap().as_str().to_string();
                    let f_name = f_inner.next().unwrap().as_str().to_string();
                    let f_type = f_inner.next().map(|a| a.as_str().trim().to_string()).unwrap_or_default();
                    fields.push(PacketField { range, name: f_name, field_type: f_type });
                }
                Rule::packet_constraint => {
                    constraint = Some(stmt.as_str().trim().strip_prefix("constraint:").unwrap_or("").trim().to_string());
                }
                _ => {}
            }
        }
    }

    Ok(PacketDeclaration { name, size: attrs, fields, constraint })
}
