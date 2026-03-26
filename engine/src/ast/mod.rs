pub mod element;
pub mod style;

pub use crate::ast::element::{ShapeKind, Id};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Project {
    pub diagrams: Vec<NamedDiagram>,
    pub packets: Vec<PacketDeclaration>,
    pub states: Vec<StateDeclaration>,
    pub xys: Vec<XyDeclaration>,
    pub variables: HashMap<String, Expression>,
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
    Unit(f64, String),
    VariableRef(String),
    PropertyRef(String, String), // NodeId, PropertyName
    BinaryOp(Box<Expression>, String, Box<Expression>),
}
