use petgraph::stable_graph::StableGraph;
use kurbo::Point;

#[derive(Default)]
pub struct LayoutContext {
    pub graph: StableGraph<(), ()>,
    pub positions: Vec<Point>,
}