use crate::ast::{Connection, Layer, NamedDiagram, Node, ShapeKind};
use crate::parser::error::ParseError;
use crate::parser::expressions::parse_math_expr_stable;
use std::collections::HashMap;
use trx_syntax::{Rule, TrxPair};

pub fn parse_single_diagram(
    pair: TrxPair,
    classes: &mut HashMap<String, HashMap<String, String>>,
) -> Result<NamedDiagram, ParseError> {
    let mut inner = pair.into_inner();

    let name = inner
        .next()
        .ok_or(ParseError::MissingId)?
        .as_str()
        .trim()
        .to_string();

    let mut diagram = NamedDiagram::new(name);

    for stmt in inner {
        parse_diagram_stmt(stmt, &mut diagram.root, &mut diagram.connections, classes, &mut diagram.scenario)?;
    }
    Ok(diagram)
}

fn parse_diagram_stmt(
    pair: TrxPair,
    curr_layer: &mut Layer,
    connections: &mut Vec<Connection>,
    classes: &mut HashMap<String, HashMap<String, String>>,
    scenario: &mut Option<String>,
) -> Result<(), ParseError> {
    match pair.as_rule() {
        Rule::scenario_decl => {
            let mut inner = pair.into_inner();
            let s_lit = inner.next().unwrap().as_str().trim_matches('"').to_string();
            *scenario = Some(s_lit);
        }
        Rule::node_decl => {
            curr_layer.nodes.push(parse_node(pair)?);
        }
        Rule::layer_block => {
            curr_layer
                .layers
                .push(parse_layer(pair, connections, classes, scenario)?);
        }
        Rule::connection => {
            connections.push(parse_connection(pair)?);
        }
        Rule::style_decl => {
            let mut inner = pair.into_inner();
            let cname = inner.next().unwrap().as_str().to_string(); // identifier

            let cprops_str = inner.next().unwrap().as_str().to_string();
            let mut attrs = HashMap::new();
            for stmt in cprops_str.split(';') {
                let stmt = stmt.trim();
                if stmt.is_empty() {
                    continue;
                }
                if let Some(idx) = stmt.find(':') {
                    let k = stmt[..idx].trim().to_string();
                    let v = stmt[idx + 1..].trim().to_string();
                    attrs.insert(k, v);
                }
            }
            classes.insert(cname, attrs);
        }
        _ => {}
    }
    Ok(())
}

fn parse_layer(
    pair: TrxPair,
    connections: &mut Vec<Connection>,
    classes: &mut HashMap<String, HashMap<String, String>>,
    scenario: &mut Option<String>,
) -> Result<Layer, ParseError> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    let mut layer = Layer {
        id,
        nodes: Vec::new(),
        layers: Vec::new(),
    };

    for stmt in inner {
        parse_diagram_stmt(stmt, &mut layer, connections, classes, scenario)?;
    }

    Ok(layer)
}

fn parse_node(pair: TrxPair) -> Result<Node, ParseError> {
    let mut inner = pair.into_inner();
    let id = inner.next().ok_or(ParseError::MissingId)?.as_str().trim().to_string();

    let mut attributes = HashMap::new();
    let mut properties = HashMap::new();

    let mut class = None;

    for item in inner {
        match item.as_rule() {
            Rule::identifier => {
                class = Some(item.as_str().to_string());
            }
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
                    let expr = parse_math_expr_stable(p_inner.next().unwrap())?;
                    properties.insert(key, expr);
                }
            }
            _ => {}
        }
    }

    Ok(Node {
        id,
        class,
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
