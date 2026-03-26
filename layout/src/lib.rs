pub mod physics;
pub mod quadtree;

use std::collections::{HashMap, VecDeque};
use trx_engine::ast::{Connection, Layer, Node, Project};
use crate::physics::{calculate_repulsion, Vector2D, ForceParams};

pub fn apply_layout(project: &mut Project) {
    for diagram in &mut project.diagrams {
        layout_layer(&mut diagram.root, &diagram.connections);
    }
}

pub fn layout_layer(layer: &mut Layer, connections: &[Connection]) {
    for child_layer in &mut layer.layers {
        layout_layer(child_layer, connections);
    }

    let n = layer.nodes.len();
    if n == 0 && layer.layers.is_empty() {
        return;
    }

    let layer_height = 150.0;
    let node_separation = 200.0;
    let start_x = 100.0;
    let start_y = 100.0;

    for (i, node) in layer.nodes.iter_mut().enumerate() {
        node.x = start_x + (i as f32) * node_separation;
        node.y = start_y;
        if node.width <= 0.0 { node.width = 120.0; }
        if node.height <= 0.0 { node.height = 60.0; }
    }

    // Basic physics iteration loop (50 iterations)
    let params = ForceParams::default();
    for _ in 0..50 {
        let mut displacements = vec![Vector2D::zero(); n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let p1 = Vector2D { x: layer.nodes[i].x as f64, y: layer.nodes[i].y as f64 };
                    let p2 = Vector2D { x: layer.nodes[j].x as f64, y: layer.nodes[j].y as f64 };
                    let rep = calculate_repulsion(p1, p2, params.charge);
                    displacements[i].x += rep.x;
                    displacements[i].y += rep.y;
                }
            }
        }
        for i in 0..n {
            layer.nodes[i].x += displacements[i].x as f32 * params.damping as f32;
            layer.nodes[i].y += displacements[i].y as f32 * params.damping as f32;
        }
    }

    let mut current_y = start_y + layer_height;
    for child_layer in &mut layer.layers {
        current_y += layer_height + 50.0;
    }
}
