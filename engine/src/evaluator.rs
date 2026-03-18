use crate::ast::{Expression, Layer, NamedDiagram, Node, Project};
use std::collections::HashMap;

/// Evaluates a project's AST before layout.
/// Resolves let bindings and computes math expressions to concrete values.
pub fn evaluate_project(project: &mut Project) {
    for diagram in &mut project.diagrams {
        evaluate_diagram(diagram);
    }
}

fn evaluate_diagram(diagram: &mut NamedDiagram) {
    // Collect all root-level variables (from let statements if we had them in Layer)
    let mut context = EvaluationContext::new();

    // Evaluate the root layer
    evaluate_layer(&mut diagram.root, &mut context);
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

    // Simple expression evaluator
    fn eval(&self, expr: &Expression) -> Option<f64> {
        match expr {
            Expression::Number(n) => Some(*n),
            Expression::Unit(n, _) => Some(*n), // Ignore unit for now
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
                    _ => None,
                }
            }
            _ => None, // Properties/strings not fully supported in this stub
        }
    }
}

fn evaluate_layer(layer: &mut Layer, context: &mut EvaluationContext) {
    // Evaluate children nodes
    for node in &mut layer.nodes {
        evaluate_node(node, context);
    }

    // Evaluate children layers recursively
    for child_layer in &mut layer.layers {
        evaluate_layer(child_layer, context);
    }
}

fn evaluate_node(_node: &mut Node, _context: &mut EvaluationContext) {
    // Here we would iterate over node.properties, evaluate their Expressions
    // and store the result or replace the Expression with a resolved value.
    //  node.width = context.eval(&node.properties["width"]).unwrap_or(120.0);
}
