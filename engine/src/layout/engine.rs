use crate::ast::{NamedDiagram, Layer, Node};
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

// ────────────────────────────────────────────────────────────────────────────
// Milestone 02 — Layered (Sugiyama-style) Layout Engine
// ────────────────────────────────────────────────────────────────────────────

/// A simple layered layout that assigns ranks via topological sort and
/// places nodes in horizontal bands (layers / ranks).
#[derive(Debug, Clone)]
pub struct LayeredLayout {
    pub x_gap: f32,
    pub y_gap: f32,
    pub node_width: f32,
    pub node_height: f32,
}

impl Default for LayeredLayout {
    fn default() -> Self {
        Self {
            x_gap: 60.0,
            y_gap: 80.0,
            node_width: 120.0,
            node_height: 60.0,
        }
    }
}

impl LayoutEngine for LayeredLayout {
    fn layout(&mut self, diagram: &mut NamedDiagram, _bounds: Rect) {
        // Build adjacency from connections
        let node_ids: Vec<String> = collect_all_node_ids(&diagram.root);
        let n = node_ids.len();
        if n == 0 {
            return;
        }

        let id_index: HashMap<&str, usize> = node_ids
            .iter()
            .enumerate()
            .map(|(i, s)| (s.as_str(), i))
            .collect();

        // Compute in-degrees for topological sort
        let mut in_degree = vec![0usize; n];
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for conn in &diagram.connections {
            if let (Some(&from), Some(&to)) = (
                id_index.get(conn.from.as_str()),
                id_index.get(conn.to.as_str()),
            ) {
                adj[from].push(to);
                in_degree[to] += 1;
            }
        }

        // Kahn's algorithm — assign topological ranks
        let mut rank = vec![0usize; n];
        let mut queue: std::collections::VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                rank[v] = rank[v].max(rank[u] + 1);
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        // Group nodes by rank
        let max_rank = *rank.iter().max().unwrap_or(&0);
        let mut rank_groups: Vec<Vec<usize>> = vec![Vec::new(); max_rank + 1];
        for (i, &r) in rank.iter().enumerate() {
            rank_groups[r].push(i);
        }

        // Place nodes
        let positions: HashMap<usize, (f32, f32)> = rank_groups
            .iter()
            .enumerate()
            .flat_map(|(r, group)| {
                group.iter().enumerate().map(move |(col, &node_idx)| {
                    let x = col as f32 * (120.0 + 60.0);
                    let y = r as f32 * (60.0 + 80.0);
                    (node_idx, (x, y))
                })
            })
            .collect();

        // Apply positions to AST nodes
        apply_positions_to_layer(&mut diagram.root, &node_ids, &positions);

        // Milestone 06 — Recursive Offsetting
        apply_recursive_offsets(&mut diagram.root, 0.0, 0.0);
    }

    fn configure(&mut self, params: &HashMap<String, f64>) {
        if let Some(&g) = params.get("x_gap") { self.x_gap = g as f32; }
        if let Some(&g) = params.get("y_gap") { self.y_gap = g as f32; }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Milestone 06 — Recursive Offsetting helpers
// ────────────────────────────────────────────────────────────────────────────

/// Recursively offset child layer node coordinates by their parent's origin.
pub fn apply_recursive_offsets(layer: &mut Layer, parent_x: f32, parent_y: f32) {
    for node in &mut layer.nodes {
        node.x += parent_x;
        node.y += parent_y;
    }
    for child_layer in &mut layer.layers {
        // Determine the child layer's own origin from its first node (if any)
        let cx = child_layer.nodes.first().map(|n| n.x).unwrap_or(0.0);
        let cy = child_layer.nodes.first().map(|n| n.y).unwrap_or(0.0);
        apply_recursive_offsets(child_layer, parent_x + cx, parent_y + cy);
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn collect_all_node_ids(layer: &Layer) -> Vec<String> {
    let mut ids = Vec::new();
    for node in &layer.nodes {
        ids.push(node.id.clone());
    }
    for child in &layer.layers {
        ids.extend(collect_all_node_ids(child));
    }
    ids
}

fn apply_positions_to_layer(
    layer: &mut Layer,
    node_ids: &[String],
    positions: &HashMap<usize, (f32, f32)>,
) {
    for node in &mut layer.nodes {
        if let Some(idx) = node_ids.iter().position(|id| id == &node.id) {
            if let Some(&(x, y)) = positions.get(&idx) {
                node.x = x;
                node.y = y;
            }
        }
    }
    for child in &mut layer.layers {
        apply_positions_to_layer(child, node_ids, positions);
    }
}
