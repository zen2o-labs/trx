pub mod element;
pub mod style;
pub mod style_buffer;

pub use crate::ast::element::{Id, ShapeKind};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Project {
    pub diagrams: Vec<NamedDiagram>,
    pub packets: Vec<PacketDeclaration>,
    pub states: Vec<StateDeclaration>,
    pub xys: Vec<XyDeclaration>,
    pub sqltables: Vec<SqlTableDeclaration>,
    pub variables: HashMap<String, Expression>,
    pub classes: HashMap<String, HashMap<String, String>>,
}

/// Milestone 04 — Schema Visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlField {
    pub name: String,
    pub field_type: String,
    pub is_pk: bool,
    pub is_fk: bool,
    pub fk_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlTableDeclaration {
    pub name: String,
    pub fields: Vec<SqlField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketField {
    pub range: String,
    pub name: String,
    pub field_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketDeclaration {
    pub name: String,
    pub size: String,
    pub fields: Vec<PacketField>,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDeclaration {
    pub name: String,
    pub transitions: Vec<StateTransition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XyDeclaration {
    pub name: String,
    pub x_axis: String,
    pub y_axis: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedDiagram {
    pub name: String,
    pub root: Layer,
    pub connections: Vec<Connection>,
    /// Milestone 08 — Scenario State Management
    pub scenario: Option<String>,
}

impl NamedDiagram {
    pub fn new(name: String) -> Self {
        Self {
            name,
            root: Layer {
                id: "root".to_string(),
                nodes: Vec::new(),
                layers: Vec::new(),
            },
            connections: Vec::new(),
            scenario: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: String,
    pub nodes: Vec<Node>,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub class: Option<String>,
    pub label: Option<String>,
    pub kind: ShapeKind,
    pub properties: HashMap<String, Expression>,
    pub attributes: HashMap<String, String>,

    // Layout compute properties
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: String,
    pub to: String,
    pub arrow: String,
    pub label: Option<String>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Unit(f64, String),
    VariableRef(String),
    PropertyRef(String, String), // NodeId, PropertyName
    BinaryOp(Box<Expression>, String, Box<Expression>),
    UnaryOp(String, Box<Expression>),
    FunctionCall { name: String, args: Vec<Expression> },
}
