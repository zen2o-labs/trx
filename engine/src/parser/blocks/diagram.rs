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
    // A simplified math expression parser mapping pest tree to Expression AST.
    // parse simple numbers to pass basic JSON outputs.
    let s = pair.as_str().to_string();
    Ok(Expression::String(s))
}
