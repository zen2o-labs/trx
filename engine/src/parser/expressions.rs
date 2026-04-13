use crate::ast::Expression;
use crate::parser::error::ParseError;
use trx_syntax::Rule;

/// math expression into an AST Node with proper operator precedence.
///
pub fn parse_math_expr_stable(pair: pest::iterators::Pair<Rule>) -> Result<Expression, ParseError> {
    let mut inner = pair.into_inner();

    // Extract first term and all subsequent (operator, term) pairs
    let mut expressions = vec![parse_math_term(inner.next().unwrap())?];
    let mut operators = vec![];

    while let (Some(op_pair), Some(term_pair)) = (inner.next(), inner.next()) {
        operators.push(op_pair.as_str().trim().to_string());
        expressions.push(parse_math_term(term_pair)?);
    }

    Ok(build_precedence_tree(expressions, operators))
}

/// Returns the mathematical precedence level of an operator.
fn get_operator_precedence(op: &str) -> i32 {
    match op {
        "==" | "!=" | "<" | ">" | "<=" | ">=" => 1,
        "+" | "-" => 2,
        "*" | "/" => 3,
        _ => 0,
    }
}

/// Builds a binary expression tree from a list of terms and operators based on precedence.
fn build_precedence_tree(mut terms: Vec<Expression>, mut ops: Vec<String>) -> Expression {
    if ops.is_empty() {
        return terms.remove(0);
    }

    // Find the operator with the lowest precedence to serve as the root of this sub-tree.
    // We scan right-to-left for left-associative operators.
    let mut min_precedence = i32::MAX;
    let mut split_index = 0;

    for (index, operator) in ops.iter().enumerate() {
        let precedence = get_operator_precedence(operator);
        if precedence <= min_precedence {
            min_precedence = precedence;
            split_index = index;
        }
    }

    // Split at the pivot operator
    let left_ops = ops.drain(0..split_index).collect::<Vec<_>>();
    let pivot_op = ops.remove(0);
    let right_ops = ops;

    let left_terms = terms.drain(0..split_index + 1).collect::<Vec<_>>();
    let right_terms = terms;

    Expression::BinaryOp(
        Box::new(build_precedence_tree(left_terms, left_ops)),
        pivot_op,
        Box::new(build_precedence_tree(right_terms, right_ops)),
    )
}

/// Parses a single math term (number, variable, function call, etc).
pub fn parse_math_term(pair: pest::iterators::Pair<Rule>) -> Result<Expression, ParseError> {
    let mut current_pair = pair;
    if current_pair.as_rule() == Rule::math_term {
        current_pair = current_pair.into_inner().next().unwrap();
    }

    let expression = match current_pair.as_rule() {
        Rule::function_call => {
            let inner_elements = current_pair.into_inner();
            let mut function_name_parts = Vec::new();
            let mut function_arguments = Vec::new();

            for element in inner_elements {
                match element.as_rule() {
                    Rule::identifier => function_name_parts.push(element.as_str().to_string()),
                    Rule::math_expression => {
                        function_arguments.push(parse_math_expr_stable(element)?)
                    }
                    _ => {}
                }
            }
            Expression::FunctionCall {
                name: function_name_parts.join("."),
                args: function_arguments,
            }
        }
        Rule::identifier => Expression::VariableRef(current_pair.as_str().trim().to_string()),
        Rule::property_ref => {
            let identifier_string = current_pair.as_str().trim().to_string();
            if let Some(dot_index) = identifier_string.find('.') {
                let node_id = identifier_string[..dot_index].to_string();
                let property_name = identifier_string[dot_index + 1..].to_string();
                Expression::PropertyRef(node_id, property_name)
            } else {
                Expression::VariableRef(identifier_string)
            }
        }
        Rule::number => Expression::Number(current_pair.as_str().parse().unwrap_or(0.0)),
        Rule::number_with_unit => {
            let unit_string = current_pair.as_str();
            let numeric_part: String = unit_string
                .chars()
                .take_while(|c| c.is_numeric() || *c == '.')
                .collect();
            let unit_part: String = unit_string
                .chars()
                .skip_while(|c| c.is_numeric() || *c == '.')
                .collect();
            Expression::Unit(numeric_part.parse().unwrap_or(0.0), unit_part)
        }
        _ => Expression::String(current_pair.as_str().to_string()),
    };

    Ok(expression)
}
