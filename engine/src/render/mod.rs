pub mod svg;

use crate::ast::Project;

pub trait Renderer {
    fn render(&self, project: &Project) -> String; 
    fn render_to_bytes(&self, project: &Project) -> Vec<u8>; 
}


pub trait RenderEngine {
    fn render(&self, project: &Project) -> Vec<u8>;
}