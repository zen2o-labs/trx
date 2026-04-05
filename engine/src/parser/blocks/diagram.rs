use crate::ast::{Connection, Expression, Layer, NamedDiagram, Node, ShapeKind};
use crate::parser::error::ParseError;
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
        parse_diagram_stmt(stmt, &mut diagram.root, &mut diagram.connections, classes)?;
    }
    Ok(diagram)
}

fn parse_diagram_stmt(
    pair: TrxPair,
    curr_layer: &mut Layer,
    connections: &mut Vec<Connection>,
    classes: &mut HashMap<String, HashMap<String, String>>,
) -> Result<(), ParseError> {
    match pair.as_rule() {
        Rule::node_decl => {
            curr_layer.nodes.push(parse_node(pair)?);
        }
        Rule::layer_block => {
            curr_layer
                .layers
                .push(parse_layer(pair, connections, classes)?);
        }
        Rule::connection => {
            connections.push(parse_connection(pair)?);
        }
        Rule::style_decl => {
            let mut inner = pair.into_inner();
            let cname = inner.next().unwrap().as_str().to_string(); // identifier

            // Collect the style props (which could be attributes or inline strings for now,
            // but the rule is `style_props = { (!"}" ~ ANY)* }` so let's parse basic CSS-like KVs manually:
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
) -> Result<Layer, ParseError> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();
    let mut layer = Layer {
        id,
        nodes: Vec::new(),
        layers: Vec::new(),
    };

    for stmt in inner {
        parse_diagram_stmt(stmt, &mut layer, connections, classes)?;
    }

    Ok(layer)
}

fn parse_node(pair: TrxPair) -> Result<Node, ParseError> {
    let mut inner = pair.into_inner();
    let id = inner.next().unwrap().as_str().to_string();

    let mut attributes = HashMap::new();
    let mut properties = HashMap::new();

    let mut class = None;

    for item in inner {
        match item.as_rule() {
            Rule::identifier => {
                // If there's an extra identifier before attributes, it's the class ref (since first identifier was extracted before this loop)
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
                    let expr = parse_expression(p_inner.next().unwrap())?;
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
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    let (term_pair, is_negative) = if first.as_str() == "-" {
        (inner.next().unwrap(), true)
    } else {
        (first, false)
    };

    let mut expr = match term_pair.as_rule() {
        Rule::function_call => {
            let mut inner = term_pair.into_inner();
            let mut name_parts = Vec::new();
            while let Some(peek) = inner.peek() {
                if peek.as_rule() == Rule::identifier {
                    name_parts.push(inner.next().unwrap().as_str().to_string());
                } else {
                    break;
                }
            }
            let name = name_parts.join(".");
            let mut args = Vec::new();
            for expr_pair in inner {
                if expr_pair.as_rule() == Rule::math_expression {
                    args.push(parse_expression(expr_pair)?);
                }
            }
            Expression::FunctionCall { name, args }
        }
        Rule::number => {
            let n = term_pair.as_str().parse::<f64>().unwrap_or(0.0);
            Expression::Number(n)
        }
        Rule::number_with_unit => {
            let s = term_pair.as_str();
            let num_str: String = s
                .chars()
                .take_while(|c| c.is_numeric() || *c == '.')
                .collect();
            let unit: String = s
                .chars()
                .skip_while(|c| c.is_numeric() || *c == '.')
                .collect();
            let n = num_str.parse::<f64>().unwrap_or(0.0);
            Expression::Unit(n, unit)
        }
        Rule::identifier => Expression::VariableRef(term_pair.as_str().to_string()),
        _ => Expression::String(term_pair.as_str().to_string()),
    };

    if is_negative {
        expr = Expression::UnaryOp("-".to_string(), Box::new(expr));
    }

    Ok(expr)
}
