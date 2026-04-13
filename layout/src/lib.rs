pub mod physics;
pub mod quadtree;

use crate::physics::{ForceParams, Vector2D, calculate_repulsion};
use kurbo::Rect;
use trx_engine::ast::{Connection, Layer, NamedDiagram, Project};
use trx_engine::layout::LayoutEngine;

// Constants for default layout values
const DEFAULT_NODE_WIDTH: f32 = 120.0;
const DEFAULT_NODE_HEIGHT: f32 = 60.0;
const DEFAULT_LAYER_HEIGHT: f32 = 150.0;
const DEFAULT_NODE_SEPARATION: f32 = 200.0;
const START_X: f32 = 100.0;
const START_Y: f32 = 100.0;
const PHYSICS_ITERATIONS: usize = 50;

pub fn apply_layout(project: &mut Project) {
    for diagram in &mut project.diagrams {
        layout_recursive(&mut diagram.root, &diagram.connections, START_X, START_Y);
    }
}

/// Recursively lays out a layer and its children using physics-based positioning.
pub fn layout_recursive(
    layer: &mut Layer,
    connections: &[Connection],
    offset_x: f32,
    offset_y: f32,
) {
    // Initial Position Assignment
    for (i, node) in layer.nodes.iter_mut().enumerate() {
        node.x = offset_x + (i as f32) * DEFAULT_NODE_SEPARATION;
        node.y = offset_y;

        if node.width <= 0.0 {
            node.width = DEFAULT_NODE_WIDTH;
        }
        if node.height <= 0.0 {
            node.height = DEFAULT_NODE_HEIGHT;
        }
    }

    //  Physics Simulation for current layer nodes
    apply_physics_to_layer_nodes(layer);

    // Layout child layers
    let mut child_y_offset = offset_y + DEFAULT_LAYER_HEIGHT;
    for child_layer in &mut layer.layers {
        layout_recursive(child_layer, connections, offset_x, child_y_offset);
        child_y_offset += DEFAULT_LAYER_HEIGHT + 50.0;
    }
}

fn apply_physics_to_layer_nodes(layer: &mut Layer) {
    let n = layer.nodes.len();
    if n < 2 {
        return;
    }

    let params = ForceParams::default();
    for _ in 0..PHYSICS_ITERATIONS {
        let mut displacements = vec![Vector2D::zero(); n];

        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = Vector2D {
                    x: layer.nodes[i].x as f64,
                    y: layer.nodes[i].y as f64,
                };
                let p2 = Vector2D {
                    x: layer.nodes[j].x as f64,
                    y: layer.nodes[j].y as f64,
                };
                let repulsion = calculate_repulsion(p1, p2, params.charge);

                displacements[i].x += repulsion.x;
                displacements[i].y += repulsion.y;
                displacements[j].x -= repulsion.x;
                displacements[j].y -= repulsion.y;
            }
        }

        for i in 0..n {
            layer.nodes[i].x += (displacements[i].x * params.damping) as f32;
            layer.nodes[i].y += (displacements[i].y * params.damping) as f32;
        }
    }
}

pub struct ForceLayoutEngine {
    pub params: ForceParams,
    pub iterations: usize,
}

impl Default for ForceLayoutEngine {
    fn default() -> Self {
        Self {
            params: ForceParams::default(),
            iterations: PHYSICS_ITERATIONS,
        }
    }
}

impl LayoutEngine for ForceLayoutEngine {
    fn layout(&mut self, diagram: &mut NamedDiagram, bounds: Rect) {
        layout_recursive(
            &mut diagram.root,
            &diagram.connections,
            bounds.x0 as f32 + START_X,
            bounds.y0 as f32 + START_Y,
        );
    }
}

pub struct LayeredLayoutEngine {
    pub x_gap: f32,
    pub y_gap: f32,
}

impl Default for LayeredLayoutEngine {
    fn default() -> Self {
        Self {
            x_gap: 60.0,
            y_gap: 80.0,
        }
    }
}

impl LayoutEngine for LayeredLayoutEngine {
    fn layout(&mut self, diagram: &mut NamedDiagram, bounds: Rect) {
        layout_recursive(
            &mut diagram.root,
            &diagram.connections,
            bounds.x0 as f32 + START_X,
            bounds.y0 as f32 + START_Y,
        );
    }
}
