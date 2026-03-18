use crate::ast::{StateDeclaration, StateTransition};
use crate::parser::error::ParseError;
use trx_syntax::{Rule, TrxPair};

pub fn parse_state_decl(pair: TrxPair) -> Result<StateDeclaration, ParseError> {
    let mut inner = pair.into_inner();
    let name = inner.next().ok_or(ParseError::MissingId)?.as_str().to_string();
    let stmts_pair = inner.next().ok_or(ParseError::MissingId)?;

    let mut transitions = Vec::new();
    if stmts_pair.as_rule() == Rule::state_stmts {
        for stmt in stmts_pair.into_inner() {
            if stmt.as_rule() == Rule::state_transition {
                let mut t_inner = stmt.into_inner();
                let from = t_inner.next().unwrap().as_str().to_string();
                let to = t_inner.next().unwrap().as_str().to_string();
                let trigger = t_inner.next().map(|t| t.as_str().trim_start_matches(':').trim().to_string());
                transitions.push(StateTransition { from, to, trigger });
            }
        }
    }

    Ok(StateDeclaration { name, transitions })
}
