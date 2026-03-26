use crate::ast::{Connection, NamedDiagram, Node, Project, ShapeKind};
use std::collections::HashMap;

/// allows constructing a diagram entirely in native memory
/// without relying on the Pest text parser.
pub struct ProjectBuilder {
    project: Project,
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self {
            project: Project {
                diagrams: Vec::new(),
                packets: Vec::new(),
                states: Vec::new(),
                xys: Vec::new(),
                variables: HashMap::new(),
            },
        }
    }

    /// Adds a new diagram to the project and returns its index.
    pub fn add_diagram(&mut self, name: &str) -> usize {
        let diagram = NamedDiagram::new(name.to_string());
        self.project.diagrams.push(diagram);
        self.project.diagrams.len() - 1
    }

    /// Adds a node to the specified diagram index.
    /// Returns the node identifier string.
    pub fn add_node(&mut self, diagram_idx: usize, id: &str) {
        if let Some(diagram) = self.project.diagrams.get_mut(diagram_idx) {
            // Check if node exists in root layer
            if !diagram.root.nodes.iter().any(|n| n.id == id) {
                diagram.root.nodes.push(Node {
                    id: id.to_string(),
                    label: Some(id.to_string()),
                    kind: ShapeKind::Box,
                    properties: HashMap::new(),
                    attributes: HashMap::new(),
                    x: 0.0,
                    y: 0.0,
                    width: 120.0,
                    height: 60.0,
                });
            }
        }
    }

    /// Adds a directed edge between two nodes in the specified diagram.
    /// Creates the nodes if they don't exist yet.
    pub fn add_edge(&mut self, diagram_idx: usize, from: &str, to: &str, label: Option<&str>) {
        if let Some(diagram) = self.project.diagrams.get_mut(diagram_idx) {
            // Check/add `from` node
            if !diagram.root.nodes.iter().any(|n| n.id == from) {
                diagram.root.nodes.push(Node {
                    id: from.to_string(),
                    label: Some(from.to_string()),
                    kind: ShapeKind::Box,
                    properties: HashMap::new(),
                    attributes: HashMap::new(),
                    x: 0.0,
                    y: 0.0,
                    width: 120.0,
                    height: 60.0,
                });
            }

            // Check/add `to` node
            if !diagram.root.nodes.iter().any(|n| n.id == to) {
                diagram.root.nodes.push(Node {
                    id: to.to_string(),
                    label: Some(to.to_string()),
                    kind: ShapeKind::Box,
                    properties: HashMap::new(),
                    attributes: HashMap::new(),
                    x: 0.0,
                    y: 0.0,
                    width: 120.0,
                    height: 60.0,
                });
            }

            diagram.connections.push(Connection {
                from: from.to_string(),
                to: to.to_string(),
                arrow: "->".to_string(),
                label: label.map(|s| s.to_string()),
                attributes: HashMap::new(),
            });
        }
    }

    /// Consumes the builder and returns the completed Project.
    pub fn build(self) -> Project {
        self.project
    }
}

impl Default for ProjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}
