use crate::ast::{Expression, Layer, NamedDiagram, Node, Project};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum EvalError {
    #[error("Error in {} -> Math Error: {message}", trace.join(" -> "))]
    MathError { message: String, trace: Vec<String> },
    #[error("Recursion limit exceeded in {}", trace.join(" -> "))]
    RecursionLimitExceeded { trace: Vec<String> },
}

pub fn evaluate_project(project: &mut Project) -> Result<(), EvalError> {
    let mut context = EvaluationContext::new();

    let mut unresolved: HashMap<String, Expression> = project.variables.clone();
    let mut changed = true;

    while changed && !unresolved.is_empty() {
        changed = false;
        let keys: Vec<String> = unresolved.keys().cloned().collect();
        for name in keys {
            let expr = unresolved.get(&name).unwrap();
            context.trace.push(format!("Variable \"{}\"", name));
            if !matches!(expr, Expression::String(_)) {
                match context.eval(expr) {
                    Ok(val) => {
                        context.variables.insert(name.clone(), val);
                        unresolved.remove(&name);
                        changed = true;
                    }
                    Err(e) => {
                        println!("Failed to resolve {}: {}", name, e);
                    }
                }
            } else {
                unresolved.remove(&name);
            }
            context.trace.pop();
        }
    }

    if !unresolved.is_empty() {
        for (name, expr) in unresolved {
            context.trace.push(format!("Variable \"{}\"", name));
            if !matches!(expr, Expression::String(_)) {
                if let Err(e) = context.eval(&expr) {
                    return Err(e);
                }
            }
            context.trace.pop();
        }
    }

    for diagram in &mut project.diagrams {
        evaluate_diagram(diagram, &mut context)?;
    }
    Ok(())
}

fn evaluate_diagram(
    diagram: &mut NamedDiagram,
    context: &mut EvaluationContext,
) -> Result<(), EvalError> {
    evaluate_layer(&mut diagram.root, context)
}

struct EvaluationContext {
    variables: HashMap<String, f64>,
    trace: Vec<String>,
    depth: usize,
}

impl EvaluationContext {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            trace: Vec::new(),
            depth: 0,
        }
    }

    fn eval(&mut self, expr: &Expression) -> Result<f64, EvalError> {
        if self.depth > 100 {
            return Err(EvalError::RecursionLimitExceeded {
                trace: self.trace.clone(),
            });
        }
        self.depth += 1;
        let res = self._eval(expr);
        self.depth -= 1;
        res
    }

    fn _eval(&mut self, expr: &Expression) -> Result<f64, EvalError> {
        match expr {
            Expression::Number(n) => Ok(*n),
            Expression::Boolean(b) => {
                if *b {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Expression::Unit(n, _) => Ok(*n),
            Expression::VariableRef(v) => {
                self.variables
                    .get(v)
                    .copied()
                    .ok_or_else(|| EvalError::MathError {
                        message: format!("Unknown variable: {}", v),
                        trace: self.trace.clone(),
                    })
            }
            Expression::BinaryOp(left, op, right) => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                match op.as_str() {
                    "+" => Ok(l + r),
                    "-" => Ok(l - r),
                    "*" => Ok(l * r),
                    "/" => {
                        if r != 0.0 {
                            Ok(l / r)
                        } else {
                            Err(EvalError::MathError {
                                message: "Division by zero".to_string(),
                                trace: self.trace.clone(),
                            })
                        }
                    }
                    "==" => {
                        if l == r {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    "!=" => {
                        if l != r {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    "<" => {
                        if l < r {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    ">" => {
                        if l > r {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    "<=" => {
                        if l <= r {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    ">=" => {
                        if l >= r {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    _ => Err(EvalError::MathError {
                        message: format!("Unknown operator: {}", op),
                        trace: self.trace.clone(),
                    }),
                }
            }
            Expression::UnaryOp(op, inner) => {
                let val = self.eval(inner)?;
                match op.as_str() {
                    "-" => Ok(-val),
                    _ => Err(EvalError::MathError {
                        message: format!("Unknown unary operator: {}", op),
                        trace: self.trace.clone(),
                    }),
                }
            }
            Expression::FunctionCall { name, args } => {
                let mut evaluated_args = Vec::new();
                for a in args {
                    evaluated_args.push(self.eval(a)?);
                }
                if name == "Math.sin" && evaluated_args.len() == 1 {
                    Ok(libm::sin(evaluated_args[0]))
                } else if name == "Math.cos" && evaluated_args.len() == 1 {
                    Ok(libm::cos(evaluated_args[0]))
                } else if name == "Math.round" && evaluated_args.len() == 1 {
                    Ok(libm::round(evaluated_args[0]))
                } else if name == "Math.abs" && evaluated_args.len() == 1 {
                    Ok(libm::fabs(evaluated_args[0]))
                } else {
                    Err(EvalError::MathError {
                        message: format!("Unknown function: {}", name),
                        trace: self.trace.clone(),
                    })
                }
            }
            _ => Err(EvalError::MathError {
                message: "Unsupported expression type".to_string(),
                trace: self.trace.clone(),
            }),
        }
    }
}

fn evaluate_layer(layer: &mut Layer, context: &mut EvaluationContext) -> Result<(), EvalError> {
    for node in &mut layer.nodes {
        evaluate_node(node, context)?;
    }

    for child_layer in &mut layer.layers {
        evaluate_layer(child_layer, context)?;
    }

    Ok(())
}

fn evaluate_node(node: &mut Node, context: &mut EvaluationContext) -> Result<(), EvalError> {
    context.trace.push(format!("Node \"{}\"", node.id));

    if let Some(expr) = node.properties.get("width") {
        context.trace.push("Property \"width\"".to_string());
        if !matches!(expr, Expression::String(_)) {
            let val = context.eval(expr)?;
            node.width = val as f32;
        }
        context.trace.pop();
    }
    if let Some(expr) = node.properties.get("height") {
        context.trace.push("Property \"height\"".to_string());
        if !matches!(expr, Expression::String(_)) {
            let val = context.eval(expr)?;
            node.height = val as f32;
        }
        context.trace.pop();
    }

    context.trace.pop();
    Ok(())
}
