use crate::ast::{Expression, Layer, NamedDiagram, Node, Project};
use std::collections::HashMap;

/// Evaluates a project's AST before layout.
/// Resolves let bindings and computes math expressions to concrete values.
pub fn evaluate_project(project: &mut Project) {
    let mut context = EvaluationContext::new();

    for (name, expr) in &project.variables {
        if let Some(val) = context.eval(expr) {
            context.variables.insert(name.clone(), val);
        }
    }

    for diagram in &mut project.diagrams {
        evaluate_diagram(diagram, &mut context);
    }
}

fn evaluate_diagram(diagram: &mut NamedDiagram, context: &mut EvaluationContext) {
    evaluate_layer(&mut diagram.root, context);
}

struct EvaluationContext {
    variables: HashMap<String, f64>,
}

impl EvaluationContext {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn eval(&self, expr: &Expression) -> Option<f64> {
        match expr {
            Expression::Number(n) => Some(*n),
            Expression::Boolean(b) => {
                if *b {
                    Some(1.0)
                } else {
                    Some(0.0)
                }
            }
            Expression::Unit(n, _) => Some(*n),
            Expression::VariableRef(v) => self.variables.get(v).copied(),
            Expression::BinaryOp(left, op, right) => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                match op.as_str() {
                    "+" => Some(l + r),
                    "-" => Some(l - r),
                    "*" => Some(l * r),
                    "/" => {
                        if r != 0.0 {
                            Some(l / r)
                        } else {
                            None
                        }
                    }
                    "==" => {
                        if l == r {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    "!=" => {
                        if l != r {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    "<" => {
                        if l < r {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    ">" => {
                        if l > r {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    "<=" => {
                        if l <= r {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    ">=" => {
                        if l >= r {
                            Some(1.0)
                        } else {
                            Some(0.0)
                        }
                    }
                    _ => None,
                }
            }
            Expression::UnaryOp(op, inner) => {
                let val = self.eval(inner)?;
                match op.as_str() {
                    "-" => Some(-val),
                    _ => None,
                }
            }
            Expression::FunctionCall { name, args } => {
                let mut evaluated_args = Vec::new();
                for a in args {
                    evaluated_args.push(self.eval(a)?);
                }
                if name == "Math.sin" && evaluated_args.len() == 1 {
                    Some(evaluated_args[0].sin())
                } else if name == "Math.cos" && evaluated_args.len() == 1 {
                    Some(evaluated_args[0].cos())
                } else if name == "Math.round" && evaluated_args.len() == 1 {
                    Some(evaluated_args[0].round())
                } else if name == "Math.abs" && evaluated_args.len() == 1 {
                    Some(evaluated_args[0].abs())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn evaluate_layer(layer: &mut Layer, context: &mut EvaluationContext) {
    for node in &mut layer.nodes {
        evaluate_node(node, context);
    }

    for child_layer in &mut layer.layers {
        evaluate_layer(child_layer, context);
    }
}

fn evaluate_node(node: &mut Node, context: &mut EvaluationContext) {
    if let Some(expr) = node.properties.get("width") {
        if let Some(val) = context.eval(expr) {
            node.width = val as f32;
        }
    }
    if let Some(expr) = node.properties.get("height") {
        if let Some(val) = context.eval(expr) {
            node.height = val as f32;
        }
    }
}
