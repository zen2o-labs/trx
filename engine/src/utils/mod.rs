pub mod text_metrics;

pub trait Renderer {
    fn render(&self, project: &crate::ast::Project) -> String;
}
