use crate::ast::{NamedDiagram, Layer};
use kurbo::{Point, Rect};
use std::collections::HashMap;

pub trait LayoutEngine {
    fn layout(&mut self, diagram: &mut NamedDiagram, bounds: Rect);
    fn configure(&mut self, params: &HashMap<String, f64>);
}

#[derive(Debug, Clone)]
pub struct ForceLayout {
    pub repulsion: f64,
    pub attraction: f64,
    pub damping: f64,
    pub iterations: u32,
    pub center: Point,
}

impl Default for ForceLayout {
    fn default() -> Self {
        Self {
            repulsion: 1000.0,
            attraction: 0.05,
            damping: 0.1,
            iterations: 200,
            center: Point::new(400.0, 300.0),
        }
    }
}

impl LayoutEngine for ForceLayout {
    fn layout(&mut self, diagram: &mut NamedDiagram, _bounds: Rect) {
        self.layout_layer(&mut diagram.root);
    }

    fn configure(&mut self, params: &HashMap<String, f64>) {
        if let Some(&r) = params.get("repulsion") { self.repulsion = r; }
        if let Some(&a) = params.get("attraction") { self.attraction = a; }
        if let Some(&d) = params.get("damping") { self.damping = d; }
        if let Some(&i) = params.get("iterations") { self.iterations = i as u32; }
    }
}

impl ForceLayout {
    fn layout_layer(&self, layer: &mut Layer) {
        for child_layer in &mut layer.layers {
            self.layout_layer(child_layer);
        }

        let n = layer.nodes.len();
        if n == 0 { return; }

        for _ in 0..self.iterations {
            let mut disp_x = vec![0.0; n];
            let mut disp_y = vec![0.0; n];

            for i in 0..n {
                for j in (i + 1)..n {
                    let dx = (layer.nodes[i].x - layer.nodes[j].x) as f64;
                    let dy = (layer.nodes[i].y - layer.nodes[j].y) as f64;
                    let dist_sq = dx * dx + dy * dy + 0.01;
                    let force = self.repulsion / dist_sq;

                    disp_x[i] += dx * force;
                    disp_y[i] += dy * force;
                    disp_x[j] -= dx * force;
                    disp_y[j] -= dy * force;
                }
            }

            for i in 0..n {
                layer.nodes[i].x += (disp_x[i] * self.damping) as f32;
                layer.nodes[i].y += (disp_y[i] * self.damping) as f32;
            }
        }
    }
}
