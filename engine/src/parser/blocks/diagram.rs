use crate::ast::{Connection, Expression, Layer, NamedDiagram, Node, ShapeKind};
use crate::parser::error::ParseError;
use std::collections::HashMap;
use trx_syntax::{Rule, TrxPair};

pub fn parse_single_diagram(pair: TrxPair) -> Result<NamedDiagram, ParseError> {
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or(ParseError::MissingId)?
        .as_str()
        .trim()
        .to_string();

    let mut diagram = NamedDiagram::new(name);

    for stmt in inner {
        parse_diagram_stmt(stmt, &mut diagram.root, &mut diagram.connections)?;
    }
    Ok(diagram)
}

fn parse_diagram_stmt(
    pair: TrxPair,
    curr_layer: &mut Layer,
    connections: &mut Vec<Connection>,
) -> Result<(), ParseError> {
    match pair.as_rule() {
        Rule::node_decl => {
            curr_layer.nodes.push(parse_node(pair)?);
        }
        Rule::layer_block => {
            curr_layer.layers.push(parse_layer(pair, connections)?);
        }
        Rule::connection => {
            connections.push(parse_connection(pair)?);
        }
        _ => {}
    }
    Ok(())
}

fn parse_layer(pair: TrxPair, connections: &mut Vec<Connection>) -> Result<Layer, ParseError> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    let mut layer = Layer {
        id,
        nodes: Vec::new(),
        layers: Vec::new(),
    };

    for stmt in inner {
        parse_diagram_stmt(stmt, &mut layer, connections)?;
    }

    Ok(layer)
}

fn parse_node(pair: TrxPair) -> Result<Node, ParseError> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();

    let mut attributes = HashMap::new();
    let mut properties = HashMap::new();

    for item in inner {
        match item.as_rule() {
            Rule::attributes => {
                for kv in item.into_inner() {
                    let mut kv_inner = kv.into_inner();
                    let key = kv_inner.next().unwrap().as_str().to_string();
                    let val = kv_inner
                        .next()
                        .map(|v| v.as_str().to_string())
                        .unwrap_or_default();
                    attributes.insert(key, val);
                }
            }
            Rule::property_block => {
                for prop in item.into_inner() {
                    let mut p_inner = prop.into_inner();
                    let key = p_inner.next().unwrap().as_str().to_string();
                    let expr = parse_expression(p_inner.next().unwrap())?;
                    properties.insert(key, expr);
                }
            }
            _ => {}
        }
    }

    Ok(Node {
        id,
        label: None,
        kind: ShapeKind::Box,
        properties,
        attributes,
        x: 0.0,
        y: 0.0,
        width: 120.0,
        height: 60.0,
    })
}

fn parse_connection(pair: TrxPair) -> Result<Connection, ParseError> {
    let mut inner = pair.into_inner();
    let from = inner.next().unwrap().as_str().to_string();

    let arrow_pair = inner.next().unwrap();
    let arrow = arrow_pair.as_str().to_string();
    let mut attributes = HashMap::new();
    if arrow_pair.as_rule() == Rule::attr_arrow {
        for attr in arrow_pair.into_inner() {
            if attr.as_rule() == Rule::attributes {
                for kv in attr.into_inner() {
                    let mut kv_inner = kv.into_inner();
                    let key = kv_inner.next().unwrap().as_str().to_string();
                    let val = kv_inner
                        .next()
                        .map(|v| v.as_str().to_string())
                        .unwrap_or_default();
                    attributes.insert(key, val);
                }
            }
        }
    }

    let to = inner.next().unwrap().as_str().to_string();

    let label = inner
        .next()
        .map(|l| l.as_str().trim_matches('"').to_string());

    Ok(Connection {
        from,
        to,
        arrow,
        label,
        attributes,
    })
}

fn parse_expression(pair: TrxPair) -> Result<Expression, ParseError> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    let mut expr = parse_math_term(first)?;

    while let Some(op_pair) = inner.next() {
        let op = op_pair.as_str().trim().to_string();
        if let Some(right_pair) = inner.next() {
            let right = parse_math_term(right_pair)?;
            expr = Expression::BinaryOp(Box::new(expr), op, Box::new(right));
        }
    }
    Ok(expr)
}

fn parse_math_term(pair: TrxPair) -> Result<Expression, ParseError> {
    match pair.as_rule() {
        Rule::number => {
            let n = pair.as_str().parse::<f64>().unwrap_or(0.0);
            Ok(Expression::Number(n))
        }
        Rule::number_with_unit => {
            let s = pair.as_str();
            let num_str: String = s.chars().take_while(|c| c.is_numeric() || *c == '.').collect();
            let unit: String = s.chars().skip_while(|c| c.is_numeric() || *c == '.').collect();
            let n = num_str.parse::<f64>().unwrap_or(0.0);
            Ok(Expression::Unit(n, unit))
        }
        Rule::identifier => {
            Ok(Expression::VariableRef(pair.as_str().to_string()))
        }
        _ => {
            Ok(Expression::String(pair.as_str().to_string()))
        }
    }
}
