use crate::render::RenderEngine;
use crate::parser::parse; 
use crate::ast::Project;
use kurbo::Rect;

pub mod engine;
pub mod context;
pub mod quadtree;

pub use engine::LayoutEngine;

pub fn parse_and_render(
    input: &str,
    layout_engine: &mut dyn LayoutEngine,
    render_engine: &mut dyn RenderEngine,
) -> Vec<u8> {
    
    let mut project = parse(input).unwrap_or_default();


    let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);

    for diagram in &mut project.diagrams {
        layout_engine.layout(diagram, bounds);
    }

    render_engine.render(&project)
}
