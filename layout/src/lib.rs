use std::collections::{HashMap, VecDeque};
use trx_engine::ast::{Connection, Layer, Node, Project};

pub fn apply_layout(project: &mut Project) {
    for diagram in &mut project.diagrams {
        // Layout must be calculated bottom-up (children layers first)
        layout_layer(&mut diagram.root, &diagram.connections);
    }
}

pub fn layout_layer(layer: &mut Layer, connections: &[Connection]) {
    // Layout inner layers first
    for child_layer in &mut layer.layers {
        layout_layer(child_layer, connections);
    }

    let n = layer.nodes.len();
    if n == 0 && layer.layers.is_empty() {
        return;
    }

    // Rank Assignment for nodes in this layer
    // For simplicity, we just lay them out in a grid or simple horizontal sequence
    // as a placeholder for proper force/orthogonal layout.

    let layer_height = 150.0;
    let node_separation = 200.0;
    let start_x = 100.0;
    let start_y = 100.0;

    // Layout nodes
    for (i, node) in layer.nodes.iter_mut().enumerate() {
        node.x = start_x + (i as f32) * node_separation;
        node.y = start_y;
        node.width = 120.0;
        node.height = 60.0;
    }

    // Layout child layers as if they were giant nodes (simple vertical stack below nodes)
    let mut current_y = start_y + layer_height;
    for child_layer in &mut layer.layers {
        // Assume child layer layout is already done.
        // compute its bounding box here and shift it.
        // just give it a static offset placeholder.
        current_y += layer_height + 50.0;
    }
}
