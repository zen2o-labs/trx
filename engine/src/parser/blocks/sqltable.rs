use crate::ast::{SqlField, SqlTableDeclaration};
use crate::parser::error::ParseError;
use trx_syntax::{Rule, TrxPair};

pub fn parse_sqltable_decl(pair: TrxPair) -> Result<SqlTableDeclaration, ParseError> {
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or(ParseError::MissingId)?
        .as_str()
        .trim()
        .to_string();

    let mut fields = Vec::new();

    for stmt in inner {
        if stmt.as_rule() == Rule::sqltable_field {
            fields.push(parse_sqltable_field(stmt)?);
        }
    }

    Ok(SqlTableDeclaration { name, fields })
}

fn parse_sqltable_field(pair: TrxPair) -> Result<SqlField, ParseError> {
    let mut inner = pair.into_inner();
    
    let mut is_pk = false;
    let mut is_fk = false;
    
    let first = inner.next().unwrap();
    let mut current = first;
    
    if current.as_rule() == Rule::pk_fk {
        let val = current.as_str().trim();
        if val == "PK" {
            is_pk = true;
        } else if val == "FK" {
            is_fk = true;
        }
        current = inner.next().unwrap();
    }
    
    let name = current.as_str().trim().to_string();
    let field_type = inner.next().unwrap().as_str().trim().to_string();
    
    let mut fk_ref = None;
    if let Some(ref_pair) = inner.next() {
        fk_ref = Some(ref_pair.as_str().trim().to_string());
    }

    Ok(SqlField {
        name,
        field_type,
        is_pk,
        is_fk,
        fk_ref,
    })
}
